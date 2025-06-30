#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::{
        borrow::ToOwned,
        string::{String, ToString},
    };
    use gistools::data_structures::Cache;
    use std::println;

    #[test]
    fn test_cache() {
        fn on_delete(key: &String, value: &String) {
            #![allow(clippy::print_stdout)]
            println!("Deleted key {key} with value {value}");
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
            println!("Deleted key {key} with value {value}");
        }

        let mut cache = Cache::<i32, i32>::new(5, Some(on_delete)); // Cache::new(5, None);
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
