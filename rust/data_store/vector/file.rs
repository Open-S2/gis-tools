use super::VectorStore;
use crate::data_store::file::{FileOptions, S2FileStore};
use serde::{de::DeserializeOwned, Serialize};

/// File based reader that implements the VectorStore trait
pub struct FileVector<V: Serialize + DeserializeOwned + Clone> {
    store: S2FileStore<V>,
    tmp_val: Option<(u64, V)>,
}
impl<V: Serialize + DeserializeOwned + Clone> FileVector<V> {
    /// Builds a new File based KV with defined options
    pub fn new_with_options(file_name: Option<&str>, opts: Option<FileOptions>) -> FileVector<V> {
        FileVector { store: S2FileStore::new(file_name, opts), tmp_val: None }
    }
}
impl<V: Serialize + DeserializeOwned + Clone> VectorStore<V> for FileVector<V> {
    fn new(file_name: Option<&str>) -> FileVector<V> {
        FileVector { store: S2FileStore::new(file_name, None), tmp_val: None }
    }
    fn len(&self) -> u64 {
        self.store.len()
    }
    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    fn push(&mut self, key: u64, value: V) {
        self.store.set(key, value);
    }
    fn has(&mut self, key: u64) -> bool {
        self.store.has(key)
    }
    fn get(&mut self, key: u64) -> Option<&(u64, V)> {
        if let Some(vec) = self.store.get(key, Some(1)) {
            if let Some(value) = vec.into_iter().next() {
                self.tmp_val = Some((key, value)); // Move the value into tmp_val
                return self.tmp_val.as_ref(); // Return a reference to tmp_val
            }
        }
        None
    }
    fn get_index(&mut self, index: u64) -> Option<&(u64, V)> {
        self.tmp_val = self.store.get_index(index);
        self.tmp_val.as_ref()
    }
    fn sort(&mut self) {
        self.store.sort();
    }
    fn iter<'a>(&'a mut self) -> impl Iterator<Item = (u64, V)>
    where
        V: 'a,
    {
        self.store.iter_mut()
    }
    fn cleanup(&mut self) {
        self.store.cleanup();
    }
}
