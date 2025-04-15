use super::{Key, buffer_to_keys, keys_to_buffer};
use crate::{data_store::file::KEY_STORE_LENGTH, util::Buffer};
use std::{
    cmp::Ordering,
    collections::VecDeque,
    format,
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::FileExt,
    string::String,
    vec,
    vec::Vec,
};

/// A wrapper of a readable stream to handle reading in the sorted data
struct SortedFile {
    keys: VecDeque<Key>,
    offset: u64,
    is_done: bool,
    input: File,
    size: u64,
}
impl SortedFile {
    /// Create a new SortedFile
    pub fn new(input: &str) -> SortedFile {
        let input = OpenOptions::new().read(true).open(input).unwrap();
        let size = input.metadata().unwrap().len();
        SortedFile { input, size, keys: VecDeque::new(), offset: 0, is_done: false }
    }

    /// Check the current key in the buffer
    pub fn current(&self) -> Option<&Key> {
        self.keys.front()
    }

    /// return the next key in the buffer. Assumes the user has called current first to
    /// validate that there is a current key
    pub fn take(&mut self) -> Option<Key> {
        self.keys.pop_front()
    }

    /// Update the current key store if necessary
    pub fn prepare(&mut self, max_heap: usize) {
        if self.is_done {
            return;
        }
        // if there are no keys in the buffer, read in the next chunk
        if self.keys.is_empty() {
            let length = u64::min(KEY_STORE_LENGTH * max_heap as u64, self.size - self.offset);
            self.keys = buffer_to_keys(&mut self.read_buffer(length)).into();
            if self.offset >= self.size {
                self.is_done = true;
            }
        }
    }

    fn read_buffer(&mut self, length: u64) -> Buffer {
        let mut input_bytes = vec![0; length as usize];
        let _ = self.input.read_at(&mut input_bytes, self.offset);
        let input_buffer = Buffer::new(input_bytes);
        self.offset += length;

        input_buffer
    }
}

/// merge a collection of sorted chunks
pub fn merge_sorted_chunks(inputs: &[String], output: &str, max_heap: usize) {
    let mut input_files: Vec<SortedFile> = vec![];
    for input in inputs {
        input_files.push(SortedFile::new(input));
    }
    let mut output = OpenOptions::new().write(true).open(format!("{}.keys", output)).unwrap();

    // loop through all the input files and grab the next key in order
    let mut key_writes: Vec<Key> = vec![];
    loop {
        let next_key = get_next_lowest_key(&mut input_files, max_heap);
        if next_key.is_none() {
            break;
        }
        key_writes.push(next_key.unwrap());
        if key_writes.len() > max_heap {
            let _ = output.write(&keys_to_buffer(key_writes));
            key_writes = vec![];
        }
    }
    if !key_writes.is_empty() {
        let _ = output.write(&keys_to_buffer(key_writes));
    }

    let _ = output.flush();
}

/// given a list of sorted files, return the next lowest key
fn get_next_lowest_key(sorted_files: &mut [SortedFile], max_heap: usize) -> Option<Key> {
    // make sure all files are up to date on their current key
    for file in &mut *sorted_files {
        file.prepare(max_heap);
    }
    // 1) sort the files by their current key
    sorted_files.sort_by(|a, b| {
        let a_key = a.current();
        let b_key = b.current();
        match (a_key, b_key) {
            (None, _) => Ordering::Greater,
            (_, None) => Ordering::Less,
            (Some(a_key), Some(b_key)) => a_key.id.cmp(&b_key.id),
        }
    });
    if sorted_files.is_empty() { None } else { sorted_files[0].take() }
}
