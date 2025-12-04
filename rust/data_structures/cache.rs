use alloc::collections::{BTreeMap, VecDeque};

/// Function to be called when a value is deleted from the cache
pub type CacheDeleteFunction<K, V> = fn(&K, &V);

/// # Cache System
///
/// ## Description
/// A cache of values with a max size to ensure that too much old data is not stored.
///
/// Uses the [`CacheDeleteFunction`] type
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
#[derive(Debug, Clone, Default)]
pub struct Cache<K, V>
where
    K: Ord + Clone,
{
    map: BTreeMap<K, V>,
    order: VecDeque<K>,
    max_size: usize,
    on_delete: Option<CacheDeleteFunction<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: Ord + Clone,
{
    /// Creates a new cache with a given max size and an optional deletion callback.
    pub fn new(max_size: usize, on_delete: Option<CacheDeleteFunction<K, V>>) -> Self {
        Self { map: BTreeMap::new(), order: VecDeque::new(), max_size, on_delete }
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
        self.order.push_front(key.clone());
        self.map.insert(key, value);

        while self.order.len() > self.max_size {
            if let Some(oldest) = self.order.pop_back() {
                self.delete(&oldest);
            }
        }
    }

    /// Retrieves a value from the cache.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
        }
        self.map.get(key)
    }

    /// Retrieves a mutable value from the cache.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
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
