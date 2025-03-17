use crate::geometry::S2CellId;
use alloc::{vec, vec::Vec};
use s2json::Properties;
use serde::{de::DeserializeOwned, Serialize};

/// Represents a Vector store
pub trait VectorStore<V: Serialize + DeserializeOwned = Properties>:
    IntoIterator<Item = (S2CellId, V)>
{
    /// Create a new Vector store
    fn new(name: Option<&str>) -> Self;
    /// The length of the store
    fn len(&self) -> usize;
    /// Check if the store is empty
    fn is_empty(&self) -> bool;
    /// Push a value into the store
    fn push(&mut self, id: S2CellId, value: V);
    /// has a key in the store
    fn has(&self, key: &S2CellId) -> bool;
    /// get a value from the store
    fn get(&self, index: usize) -> Option<&(S2CellId, V)>;
    /// Sort the store
    fn sort(&mut self);
}

/// A local Vector store
#[derive(Debug)]
pub struct Vector<V: Serialize + DeserializeOwned = Properties> {
    store: Vec<(S2CellId, V)>,
}
impl<V: Serialize + DeserializeOwned> IntoIterator for Vector<V> {
    type Item = (S2CellId, V); // Return key-value pairs
    type IntoIter = vec::IntoIter<(S2CellId, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}
impl<V: Serialize + DeserializeOwned> VectorStore<V> for Vector<V> {
    fn new(_name: Option<&str>) -> Self {
        Self { store: vec![] }
    }
    fn len(&self) -> usize {
        self.store.len()
    }
    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    fn push(&mut self, id: S2CellId, value: V) {
        self.store.push((id, value));
    }
    fn has(&self, key: &S2CellId) -> bool {
        self.store.iter().any(|(id, _)| id == key)
    }
    fn get(&self, index: usize) -> Option<&(S2CellId, V)> {
        self.store.get(index)
    }
    fn sort(&mut self) {
        self.store.sort_by_key(|(id, _)| *id);
    }
}
