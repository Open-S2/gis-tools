use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// # Cache System
///
/// ## Description
/// A cache of values with a max size to ensure that too much old data is not stored.
///
/// ## Usage
///
/// ```rust
/// extern crate alloc;
/// use gistools::data_structures::Cache;
/// use alloc::borrow::ToOwned;
/// use alloc::string::{String, ToString};
///
/// fn on_delete(key: &String, value: &String) {
///     println!("Deleted key {} with value {}", key, value);
/// }
///
/// let mut cache = Cache::new(10, Some(on_delete));
/// cache.set("key".to_owned(), "value".to_owned());
/// println!("{:?}", cache.get(&"key".to_string())); // Some("value")
/// cache.delete(&"key".to_string());
/// ```
pub struct Cache<K, V, F>
where
    K: Ord + Clone,
    F: Fn(&K, &V),
{
    map: BTreeMap<K, V>,
    order: Vec<K>,
    max_size: usize,
    on_delete: Option<F>,
}

impl<K, V, F> Cache<K, V, F>
where
    K: Ord + Clone,
    F: Fn(&K, &V),
{
    /// Creates a new cache with a given max size and an optional deletion callback.
    pub fn new(max_size: usize, on_delete: Option<F>) -> Self {
        Self { map: BTreeMap::new(), order: Vec::new(), max_size, on_delete }
    }

    /// Returns the number of elements in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Inserts a key-value pair into the cache.
    pub fn set(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        }
        self.order.insert(0, key.clone());
        self.map.insert(key, value);

        while self.order.len() > self.max_size {
            if let Some(oldest) = self.order.pop() {
                if let Some(val) = self.map.remove(&oldest) {
                    if let Some(ref callback) = self.on_delete {
                        callback(&oldest, &val);
                    }
                }
            }
        }
    }

    /// Retrieves a value from the cache.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.insert(0, key.clone());
        }
        self.map.get(key)
    }

    /// Retrieves a mutable value from the cache.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.insert(0, key.clone());
        }
        self.map.get_mut(key)
    }

    /// Removes a key from the cache.
    pub fn delete(&mut self, key: &K) -> bool {
        if let Some(value) = self.map.remove(key) {
            self.order.retain(|k| k != key);
            if let Some(ref callback) = self.on_delete {
                callback(key, &value);
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;
    use alloc::string::{String, ToString};
    use std::println;

    #[test]
    fn test_cache() {
        fn on_delete(key: &String, value: &String) {
            #![allow(clippy::print_stdout)]
            println!("Deleted key {} with value {}", key, value);
        }

        let mut cache = Cache::new(5, Some(on_delete));
        cache.set("key".to_owned(), "value".to_owned());
        assert_eq!(cache.get(&"key".to_string()), Some(&"value".to_string()));
        // cache.delete(&"key".to_string());

        // get_mut
        let val = cache.get_mut(&"key".to_string()).unwrap();
        *val = "new value".to_owned();
        assert_eq!(cache.get(&"key".to_string()), Some(&"new value".to_string()));
        assert!(cache.delete(&"key".to_string()));

        // delete non-existant
        assert!(!cache.delete(&"key".to_string()));
    }

    #[test]
    fn test_cache_overflow() {
        fn on_delete(key: &i32, value: &i32) {
            #![allow(clippy::print_stdout)]
            println!("Deleted key {} with value {}", key, value);
        }

        let mut cache = Cache::<i32, i32, fn(&i32, &i32)>::new(5, Some(on_delete)); // Cache::new(5, None);
        assert!(cache.is_empty());

        cache.set(1, 2);
        assert_eq!(cache.get(&1), Some(&2));
        assert!(cache.delete(&1));

        cache.set(1, 2);
        cache.set(2, 3);
        cache.set(3, 4);
        cache.set(4, 5);
        cache.set(5, 6);
        cache.set(6, 7);
        cache.set(7, 8);
        cache.set(4, 9);

        assert_eq!(cache.len(), 5);
        assert!(!cache.is_empty());
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&4));
    }
}
