use crate::parsers::{FileReader, FileWriter, MMapReader, StdReader, Writer};
use core::marker::PhantomData;
use s2json::Properties;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    env, format,
    fs::{self},
    path::Path,
    string::String,
    time::{SystemTime, UNIX_EPOCH},
    vec,
    vec::Vec,
};

use super::{U64, external_sort};

/// Options to create a S2BaseStore
#[derive(Debug, Default)]
pub struct FileOptions {
    /// If true, then the data is already sorted and get calls can be immediately returned
    is_sorted: Option<bool>,
    /// The maximum heap size in bytes for each grouping of data.
    max_heap: Option<usize>,
    /// The number of threads to use for sorting
    thread_count: Option<usize>,
    /// If desired, a temporary directory to use
    tmp_dir: Option<String>,
}

/// The state of the store
#[derive(Debug)]
pub enum FileState<R: StdReader> {
    /// The store is read-only
    Read(R),
    /// The store is write-only
    Write(FileWriter),
    /// No data has been written yet
    Empty,
}

/// The length of a key: [id: u64, value-offset: u64, value-length: u32]
pub const KEY_STORE_LENGTH: u64 = 20;

/// An S2 store that uses the FileSystem for both reading and writing
pub type S2FileStore<K, V> = S2BaseStore<FileReader, K, V>;

/// An S2 store that uses the FileSystem for writing but MMaps the read access
pub type S2MMapStore<K, V> = S2BaseStore<MMapReader, K, V>;

/// NOTE: The File KVStore is designed to be used in states:
/// - write-only. The initial state is write-only. Write all you need to before reading
/// - read-only. Once you have written everything, the first read will lock the file to be static
///   and read-only.
#[derive(Debug)]
pub struct S2BaseStore<
    R: StdReader = FileReader,
    K: U64 = u64,
    V: Serialize + DeserializeOwned = Properties,
> {
    tmp_dir: String,
    file_name: String,
    size: u64,
    sorted: bool,
    max_heap: Option<usize>,
    thread_count: Option<usize>,
    value_offset: u64,
    key_file: FileState<R>,
    value_file: FileState<R>,
    _phantom: PhantomData<(K, V)>,
}
impl<R: StdReader, K: U64, V: Serialize + DeserializeOwned> Default for S2BaseStore<R, K, V> {
    fn default() -> Self {
        S2BaseStore {
            tmp_dir: String::new(),
            file_name: String::new(),
            size: 0,
            sorted: false,
            max_heap: None,
            thread_count: None,
            value_offset: 0,
            key_file: FileState::Empty,
            value_file: FileState::Empty,
            _phantom: PhantomData,
        }
    }
}
impl<R: StdReader, K: U64, V: Serialize + DeserializeOwned> S2BaseStore<R, K, V> {
    /// Builds a new File based KV
    pub fn new(file_name: Option<&str>, options: Option<FileOptions>) -> Self {
        let mut file = Self::default();
        let options = options.unwrap_or_default();
        file.tmp_dir = options.tmp_dir.clone().unwrap_or_else(|| build_tmp_dir(None));
        file.file_name = file_name
            .map(|f| f.into())
            .unwrap_or_else(|| build_tmp_file_name(file.tmp_dir.clone()));
        file.sorted = options.is_sorted.unwrap_or(false);
        file.max_heap = options.max_heap;
        file.thread_count = options.thread_count;
        if !file.sorted {
            file.switch_to_write_state();
        } else {
            file.switch_to_read_state();
        }
        // Update the size if the file already existed
        if let Ok(stat) = fs::metadata(format!("{}.keys", file.file_name)) {
            file.size = stat.len() / KEY_STORE_LENGTH;
        }

        file
    }

    /// Returns the number of entries
    pub fn len(&self) -> u64 {
        self.size
    }

    /// Returns true if the store is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Sets the value for the given key
    pub fn set(&mut self, key: K, value: V) {
        let key = key.into();
        self.switch_to_write_state();
        self.sorted = false;
        // grab writers
        let (FileState::Write(key_file), FileState::Write(value_file)) =
            (&mut self.key_file, &mut self.value_file)
        else {
            panic!("Not in write state");
        };
        // prepare values
        let vec_key = u64::to_le_bytes(key).to_vec();
        let value_str = serde_json::to_vec(&value).unwrap();
        let value_offest = u64::to_le_bytes(self.value_offset).to_vec();
        let value_length = u32::to_le_bytes(value_str.len() as u32).to_vec();
        // write
        key_file.append(&vec_key);
        key_file.append(&value_offest);
        key_file.append(&value_length);
        value_file.append(&value_str);
        // update tracker variables
        self.value_offset += value_str.len() as u64;
        self.size += 1;
    }

    /// Checks if the store contains a key
    pub fn has(&mut self, key: K) -> bool {
        // if we have no items, early return
        if self.is_empty() {
            return false;
        }
        let key = key.into();
        // ensure we are in read state
        self.switch_to_read_state();
        // get the lower bound key
        let lower_index = self.lower_bound(key);
        if lower_index >= self.size {
            return false;
        }
        let lower_key = self.get_key(lower_index);

        lower_key == key
    }

    /// Get the values in relation to the given key
    /// Returns None if the key does not exist, but will return the index of the first match with all values
    /// of key if the key exists
    pub fn get(&mut self, key: K, max: Option<usize>) -> Option<(u64, Vec<V>)> {
        // if we have no items, early return
        if self.is_empty() {
            return None;
        }
        // ensure we are in read state
        self.switch_to_read_state();
        // get the lower bound key
        let key = key.into();
        let mut lower_index = self.lower_bound(key);
        if lower_index >= self.size {
            return None;
        }

        // setup the result
        let max = max.unwrap_or(usize::MAX);
        let mut res = vec![];
        // iterate over the values by using the lower bound and moving up until the key changes,
        // we hit the max, or we hit the end of the file
        while res.len() < max && lower_index < self.size {
            let (search_key, value_index, value_length) = self.get_key_value(lower_index);
            if search_key != key {
                break;
            }
            let value = self.get_value(value_index, value_length);
            res.push(serde_json::from_slice(&value).unwrap());
            lower_index += 1;
        }

        if res.is_empty() { None } else { Some((lower_index - res.len() as u64, res)) }
    }

    /// Get the value at the given index. Return (key, value)
    pub fn get_index(&mut self, index: u64) -> Option<(K, V)> {
        if index >= self.size {
            return None;
        }
        self.switch_to_read_state();
        let (search_key, value_index, value_length) = self.get_key_value(index);
        let value = self.get_value(value_index, value_length);
        Some((K::from(search_key), serde_json::from_slice(&value).unwrap()))
    }

    /// Sort the data if not sorted
    pub fn sort(&mut self) {
        if self.sorted || self.is_empty() {
            return;
        }
        let inputs: Vec<&str> = vec![&self.file_name];
        external_sort(
            &inputs,
            &self.file_name,
            self.max_heap,
            self.thread_count,
            Some(&self.tmp_dir),
        );
        self.sorted = true;
    }

    /// Closes the store
    pub fn cleanup(&mut self) {
        fs::remove_file(format!("{}.keys", self.file_name)).unwrap();
        fs::remove_file(format!("{}.values", self.file_name)).unwrap();
        self.sorted = false;
        self.size = 0;
        self.value_offset = 0;
    }

    /// Switches to write state if in read.
    fn switch_to_write_state(&mut self) {
        match &self.key_file {
            FileState::Write(_) => {}
            _ => {
                self.key_file =
                    FileState::Write(FileWriter::new(format!("{}.keys", self.file_name)).unwrap());
            }
        }
        match &self.value_file {
            FileState::Write(_) => {}
            _ => {
                self.value_file = FileState::Write(
                    FileWriter::new(format!("{}.values", self.file_name)).unwrap(),
                );
            }
        }
    }

    /// Switches to read state if in write. Also sort the keys.
    fn switch_to_read_state(&mut self) {
        match &self.key_file {
            FileState::Read(_) => {}
            _ => {
                self.key_file =
                    FileState::Read(R::new(format!("{}.keys", self.file_name)).unwrap());
            }
        }
        match &self.value_file {
            FileState::Read(_) => {}
            _ => {
                self.value_file =
                    FileState::Read(R::new(format!("{}.values", self.file_name)).unwrap());
            }
        }
        self.sort();
    }

    /// get the index of a key that is less than or equal to the key
    fn lower_bound(&mut self, id: u64) -> u64 {
        // lower bound search
        let mut lo: u64 = 0;
        let mut hi: u64 = self.size;
        let mut mid: u64;

        while lo < hi {
            mid = lo + (hi - lo) / 2;
            let lo_hi = self.get_key(mid);
            if lo_hi < id {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }

    /// Grab a key from the store at an index
    fn get_key(&mut self, index: u64) -> u64 {
        if let FileState::Read(file) = &mut self.key_file {
            file.uint64_le(Some(index * KEY_STORE_LENGTH))
        } else {
            panic!("Not in read state");
        }
    }

    /// Grab a key, value offset, and value length from the store at an index
    fn get_key_value(&mut self, index: u64) -> (u64, u64, u32) {
        if let FileState::Read(file) = &mut self.key_file {
            (
                file.uint64_le(Some(index * KEY_STORE_LENGTH)),
                file.uint64_le(Some(index * KEY_STORE_LENGTH + 8)),
                file.uint32_le(Some(index * KEY_STORE_LENGTH + 16)),
            )
        } else {
            panic!("Not in read state");
        }
    }

    fn get_value(&mut self, index: u64, length: u32) -> Vec<u8> {
        if let FileState::Read(file) = &mut self.value_file {
            file.slice(Some(index), Some(index + length as u64))
        } else {
            panic!("Not in read state");
        }
    }

    /// Iterate over the store, one key-value at a time
    pub fn iter(&mut self) -> Iter<R, K, V> {
        Iter { container: self, index: 0 }
    }

    /// Iterate over the store but group all values related to a key
    pub fn iter_multi(&mut self) -> IterMulti<R, K, V> {
        IterMulti { container: self, index: 0 }
    }
}
/// Iterator for S2BaseStore
#[derive(Debug)]
pub struct Iter<'a, R: StdReader, K: U64, V: Serialize + DeserializeOwned> {
    container: &'a mut S2BaseStore<R, K, V>,
    index: u64,
}
impl<R: StdReader, K: U64, V: Serialize + DeserializeOwned> Iterator for Iter<'_, R, K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.container.get_index(self.index);
        self.index += 1;
        result
    }
}
/// Multi-Value Iterator for S2BaseStore
#[derive(Debug)]
pub struct IterMulti<'a, R: StdReader, K: U64, V: Serialize + DeserializeOwned> {
    container: &'a mut S2BaseStore<R, K, V>,
    index: u64,
}
impl<R: StdReader, K: U64, V: Serialize + DeserializeOwned> Iterator for IterMulti<'_, R, K, V> {
    type Item = (K, Vec<V>);
    fn next(&mut self) -> Option<Self::Item> {
        let first = self.container.get_index(self.index);
        self.index += 1;
        if let Some((key, value)) = first {
            let mut result: (K, Vec<V>) = (key, vec![value]);
            loop {
                let next = self.container.get_index(self.index);
                if let Some((next_key, next_value)) = next {
                    if next_key == key {
                        self.index += 1;
                        result.1.push(next_value);
                    } else {
                        return Some(result);
                    }
                }
            }
        } else {
            None
        }
    }
}

fn build_tmp_dir(tmp_dir: Option<String>) -> String {
    tmp_dir.unwrap_or_else(|| {
        let tmp_dir = env::temp_dir().join("s2_data_store");
        fs::create_dir_all(&tmp_dir).unwrap();
        tmp_dir.to_string_lossy().into()
    })
}

/// Builds a temporary file name
fn build_tmp_file_name(tmp_dir: String) -> String {
    let random_name = format!(
        "tmp_{:?}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() // Unique identifier
    );

    let file_name = format!("{}/{}", tmp_dir, random_name);

    // if file_name already exists let's delete it
    if Path::new(&file_name).exists() {
        fs::remove_file(&file_name).unwrap();
    }

    file_name
}
