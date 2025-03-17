use crate::geometry::S2CellId;
use alloc::{
    collections::btree_map::{self, BTreeMap},
    vec::Vec,
};
use s2json::Properties;
use serde::{de::DeserializeOwned, Serialize};

/// Represents a key-multiValue store
pub trait MultiMapStore<V: Serialize + DeserializeOwned = Properties>:
    IntoIterator<Item = (S2CellId, Vec<V>)>
{
    /// The length of the store
    fn len(&self) -> usize;
    /// Check if the store is empty
    fn is_empty(&self) -> bool;
    /// Get a value from the store
    fn get(&self, key: &S2CellId) -> Option<&Vec<V>>;
    /// Set a value in the store
    fn set(&mut self, key: &S2CellId, value: V);
    /// Check if a key exists
    fn has(&self, key: &S2CellId) -> bool;
}

/// A local key-multiValue store
#[derive(Debug, Default)]
pub struct MultiMap<V: Serialize + DeserializeOwned = Properties> {
    store: BTreeMap<S2CellId, Vec<V>>,
}
impl<V: Serialize + DeserializeOwned> IntoIterator for MultiMap<V> {
    type Item = (S2CellId, Vec<V>); // Return key-value pairs
    type IntoIter = btree_map::IntoIter<S2CellId, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}
impl<V: Serialize + DeserializeOwned> MultiMapStore<V> for MultiMap<V> {
    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    fn get(&self, key: &S2CellId) -> Option<&Vec<V>> {
        self.store.get(key)
    }

    fn set(&mut self, key: &S2CellId, value: V) {
        self.store.entry(*key).or_default().push(value);
    }

    fn has(&self, key: &S2CellId) -> bool {
        self.store.contains_key(key)
    }
}
