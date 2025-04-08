// /// File based reader that implements the MultiMapStore trait
// #[cfg(feature = "std")]
// pub mod file;
use super::U64;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use s2json::Properties;
use serde::{de::DeserializeOwned, Serialize};

/// Represents a key-multiValue store
pub trait MultiMapStore<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties> {
    /// New key-value store
    fn new(name: Option<&str>) -> Self;
    /// The length of the store
    fn len(&self) -> u64;
    /// Check if the store is empty
    fn is_empty(&self) -> bool;
    /// Get a value from the store
    fn get(&self, key: K) -> Option<&Vec<V>>;
    /// Get a mutable value from the store
    fn get_mut(&mut self, key: K) -> Option<&mut Vec<V>>;
    /// Set a value in the store
    fn set(&mut self, key: K, value: V);
    /// Check if a key exists
    fn has(&self, key: K) -> bool;
    /// Iterate over the store
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a Vec<V>)>
    where
        K: 'a,
        V: 'a;
    /// Iterate mutably over the store
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a K, &'a mut Vec<V>)>
    where
        K: 'a,
        V: 'a;
    /// Cleanup the store
    fn cleanup(&mut self) {}
}

/// A local key-multiValue store
#[derive(Debug, Default)]
pub struct MultiMap<K: U64 = u64, V: Serialize + DeserializeOwned + Clone = Properties> {
    store: BTreeMap<K, Vec<V>>,
}
impl<K: U64, V: Serialize + DeserializeOwned + Clone> MultiMapStore<K, V> for MultiMap<K, V> {
    fn new(_name: Option<&str>) -> Self {
        Self { store: BTreeMap::new() }
    }
    fn len(&self) -> u64 {
        self.store.len() as u64
    }
    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    fn get(&self, key: K) -> Option<&Vec<V>> {
        self.store.get(&key)
    }
    fn get_mut(&mut self, key: K) -> Option<&mut Vec<V>> {
        self.store.get_mut(&key)
    }
    fn set(&mut self, key: K, value: V) {
        self.store.entry(key).or_default().push(value);
    }
    fn has(&self, key: K) -> bool {
        self.store.contains_key(&key)
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a Vec<V>)>
    where
        K: 'a,
        V: 'a,
    {
        self.store.iter()
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a K, &'a mut Vec<V>)>
    where
        K: 'a,
        V: 'a,
    {
        self.store.iter_mut()
    }
    fn cleanup(&mut self) {
        self.store.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};
    use serde::Deserialize;

    #[test]
    fn test_kv() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct TestKey {
            a: f64,
        }

        let mut mm_store = MultiMap::<u64, TestKey>::new(None);

        assert!(mm_store.len() == 0);
        assert!(mm_store.is_empty());
        mm_store.set(0, TestKey { a: 1.0 });
        assert!(mm_store.len() == 1);
        mm_store.set(500000, TestKey { a: 7.0 });
        mm_store.set(1, TestKey { a: 2.0 });
        mm_store.set(12345678900001, TestKey { a: 4.0 });
        mm_store.set(12345678900000, TestKey { a: 5.0 });
        mm_store.set(500000, TestKey { a: 3.0 });
        assert!(mm_store.len() == 5);
        assert!(mm_store.has(0));
        assert!(mm_store.get(0).unwrap()[0].a == 1.0);
        assert!(mm_store.get(1).unwrap()[0].a == 2.0);
        assert!(mm_store.get_mut(12345678900000).unwrap()[0].a == 5.0);

        let values = mm_store.iter().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, vec![TestKey { a: 1.0 }]),
                (1, vec![TestKey { a: 2.0 }]),
                (500000, vec![TestKey { a: 7.0 }, TestKey { a: 3.0 }]),
                (12345678900000, vec![TestKey { a: 5.0 }]),
                (12345678900001, vec![TestKey { a: 4.0 }]),
            ]
        );

        let values =
            mm_store.iter_mut().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, vec![TestKey { a: 1.0 }]),
                (1, vec![TestKey { a: 2.0 }]),
                (500000, vec![TestKey { a: 7.0 }, TestKey { a: 3.0 }]),
                (12345678900000, vec![TestKey { a: 5.0 }]),
                (12345678900001, vec![TestKey { a: 4.0 }]),
            ]
        );

        mm_store.cleanup();
        assert!(mm_store.len() == 0);
    }
}
