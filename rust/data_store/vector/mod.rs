use super::U64;
/// File based reader that implements the VectorStore trait
// #[cfg(feature = "std")]
// pub mod file;
use alloc::{vec, vec::Vec};
use s2json::Properties;
use serde::{Serialize, de::DeserializeOwned};

/// Represents a Vector store
pub trait VectorStore<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties> {
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
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_vector() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct TestKey {
            a: f64,
        }

        let mut vector = Vector::<u64, TestKey>::new(None);

        assert!(vector.len() == 0);
        assert!(vector.is_empty());
        vector.push(0, TestKey { a: 1.0 });
        assert!(vector.len() == 1);
        vector.push(500000, TestKey { a: 7.0 });
        vector.push(1, TestKey { a: 2.0 });
        vector.push(12345678900001, TestKey { a: 4.0 });
        vector.push(12345678900000, TestKey { a: 5.0 });
        vector.push(500000, TestKey { a: 3.0 });
        vector.sort();
        assert!(vector.len() == 6);
        assert!(vector.has(0));
        assert!(vector.get(0).unwrap().1.a == 1.0);
        assert!(vector.get(1).unwrap().1.a == 2.0);
        assert!(vector.get_mut(12345678900000).unwrap().1.a == 5.0);
        assert!(vector.get_index(0).unwrap().1.a == 1.0);
        assert!(vector.get_index_mut(1).unwrap().1.a == 2.0);

        let values = vector.iter().cloned().collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, TestKey { a: 1.0 }),
                (1, TestKey { a: 2.0 }),
                (500000, TestKey { a: 7.0 }),
                (500000, TestKey { a: 3.0 }),
                (12345678900000, TestKey { a: 5.0 }),
                (12345678900001, TestKey { a: 4.0 })
            ]
        );

        let values = vector.iter_mut().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, TestKey { a: 1.0 }),
                (1, TestKey { a: 2.0 }),
                (500000, TestKey { a: 7.0 }),
                (500000, TestKey { a: 3.0 }),
                (12345678900000, TestKey { a: 5.0 }),
                (12345678900001, TestKey { a: 4.0 })
            ]
        );

        vector.cleanup();
        assert!(vector.len() == 0);
    }
}
