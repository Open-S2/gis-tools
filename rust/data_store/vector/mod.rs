use super::U64;
/// File based reader that implements the VectorStore trait
// #[cfg(feature = "std")]
// pub mod file;
use alloc::{vec, vec::Vec};
use s2json::Properties;
use serde::{Serialize, de::DeserializeOwned};

/// Represents a Vector store
pub trait VectorStore<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties>:
    Clone
{
    /// Create a new Vector store
    fn new(name: Option<&str>) -> Self;
    /// The length of the store
    fn len(&self) -> u64;
    /// Check if the store is empty
    fn is_empty(&self) -> bool;
    /// Push a value into the store
    fn push(&mut self, id: K, value: V);
    /// has a key in the store
    fn has(&self, key: K) -> bool;
    /// get a value from the store
    fn get(&self, key: K) -> Option<&(K, V)>;
    /// get a mutable value from the store
    fn get_mut(&mut self, key: K) -> Option<&mut (K, V)>;
    /// get a value from the store
    fn get_index(&self, index: u64) -> Option<&(K, V)>;
    /// get a mutable value from the store
    fn get_index_mut(&mut self, index: u64) -> Option<&mut (K, V)>;
    /// Sort the store
    fn sort(&mut self);
    /// Iterate over the store
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a;
    /// Iterate mutably over the store
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;
    /// Cleanup the store
    fn cleanup(&mut self) {}
}

/// A local Vector store
#[derive(Debug, Clone)]
pub struct Vector<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties> {
    store: Vec<(K, V)>,
    sorted: bool,
}
impl<K: U64, V: Serialize + DeserializeOwned + Clone> VectorStore<K, V> for Vector<K, V> {
    fn new(_name: Option<&str>) -> Self {
        Self { store: vec![], sorted: false }
    }
    fn len(&self) -> u64 {
        self.store.len() as u64
    }
    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    fn push(&mut self, id: K, value: V) {
        self.store.push((id, value));
    }
    fn has(&self, key: K) -> bool {
        assert!(self.sorted);
        self.store.binary_search_by_key(&key, |(id, _)| *id).is_ok()
    }
    fn get(&self, key: K) -> Option<&(K, V)> {
        assert!(self.sorted);
        self.store.get(self.store.binary_search_by_key(&key, |(id, _)| *id).ok()?)
    }
    fn get_mut(&mut self, key: K) -> Option<&mut (K, V)> {
        assert!(self.sorted);
        let index = self.store.binary_search_by_key(&key, |(id, _)| *id).ok()?;
        self.store.get_mut(index)
    }
    fn get_index(&self, index: u64) -> Option<&(K, V)> {
        assert!(self.sorted);
        self.store.get(index as usize)
    }
    fn get_index_mut(&mut self, index: u64) -> Option<&mut (K, V)> {
        assert!(self.sorted);
        self.store.get_mut(index as usize)
    }
    fn sort(&mut self) {
        if self.sorted {
            return;
        }
        self.sorted = true;
        self.store.sort_by_key(|(id, _)| *id);
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        assert!(self.sorted);
        self.store.iter()
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        assert!(self.sorted);
        self.store.iter_mut()
    }
    fn cleanup(&mut self) {
        self.store.clear();
    }
}
