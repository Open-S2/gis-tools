use crate::geometry::S2CellId;
use alloc::collections::btree_map::{self, BTreeMap};
use s2json::Properties;
use serde::{de::DeserializeOwned, Serialize};

/// Represents a key-value store
pub trait KVStore<V: Serialize + DeserializeOwned = Properties>:
    IntoIterator<Item = (S2CellId, V)>
{
    /// The length of the store
    fn len(&self) -> usize;
    /// Check if the store is empty
    fn is_empty(&self) -> bool;
    /// Get a value from the store
    fn get(&self, key: &S2CellId) -> Option<&V>;
    /// Set a value in the store
    fn set(&mut self, key: &S2CellId, value: V);
    /// Check if a key exists
    fn has(&self, key: &S2CellId) -> bool;
}

/// A local key-value store
#[derive(Debug, Default)]
pub struct KV<V: Serialize + DeserializeOwned = Properties> {
    store: BTreeMap<S2CellId, V>,
}
impl<V: Serialize + DeserializeOwned> IntoIterator for KV<V> {
    type Item = (S2CellId, V); // Return key-value pairs
    type IntoIter = btree_map::IntoIter<S2CellId, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}
impl<V: Serialize + DeserializeOwned> KVStore<V> for KV<V> {
    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    fn get(&self, key: &S2CellId) -> Option<&V> {
        self.store.get(key)
    }

    fn set(&mut self, key: &S2CellId, value: V) {
        self.store.insert(*key, value);
    }

    fn has(&self, key: &S2CellId) -> bool {
        self.store.contains_key(key)
    }
}
