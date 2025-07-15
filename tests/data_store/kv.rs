#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use gistools::data_store::{KV, KVStore, kv::file::FileKV};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_kv() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct TestKey {
            a: f64,
        }

        let mut kv_store = KV::<u64, TestKey>::new(None);

        assert!(kv_store.len() == 0);
        assert!(kv_store.is_empty());
        kv_store.set(0, TestKey { a: 1.0 });
        assert!(kv_store.len() == 1);
        kv_store.set(500000, TestKey { a: 7.0 });
        kv_store.set(1, TestKey { a: 2.0 });
        kv_store.set(12345678900001, TestKey { a: 4.0 });
        kv_store.set(12345678900000, TestKey { a: 5.0 });
        kv_store.set(500000, TestKey { a: 3.0 });
        assert!(kv_store.len() == 5);
        assert!(kv_store.has(0));
        assert!(kv_store.get(0).unwrap().a == 1.0);
        assert!(kv_store.get(1).unwrap().a == 2.0);
        assert!(kv_store.get_mut(12345678900000).unwrap().a == 5.0);

        let values = kv_store.iter().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, TestKey { a: 1.0 }),
                (1, TestKey { a: 2.0 }),
                (500000, TestKey { a: 3.0 }),
                (12345678900000, TestKey { a: 5.0 }),
                (12345678900001, TestKey { a: 4.0 })
            ]
        );

        let values =
            kv_store.iter_mut().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
        assert_eq!(
            values,
            vec![
                (0, TestKey { a: 1.0 }),
                (1, TestKey { a: 2.0 }),
                (500000, TestKey { a: 3.0 }),
                (12345678900000, TestKey { a: 5.0 }),
                (12345678900001, TestKey { a: 4.0 })
            ]
        );

        kv_store.cleanup();
        assert!(kv_store.len() == 0);
    }

    #[test]
    fn test_file_kv() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct TestKey {
            a: f64,
        }

        let mut kv_store = FileKV::<u64, TestKey>::new(None);

        assert!(kv_store.len() == 0);
        assert!(kv_store.is_empty());
        kv_store.set(0, TestKey { a: 1.0 });
        assert!(kv_store.len() == 1);
        kv_store.set(500000, TestKey { a: 7.0 });
        kv_store.set(1, TestKey { a: 2.0 });
        kv_store.set(12345678900001, TestKey { a: 4.0 });
        kv_store.set(12345678900000, TestKey { a: 5.0 });
        kv_store.set(500000, TestKey { a: 3.0 });
        assert!(kv_store.len() == 6);
        assert!(kv_store.has(0));
        assert!(kv_store.get(0).unwrap().a == 1.0);
        assert!(kv_store.get(1).unwrap().a == 2.0);
        assert!(kv_store.get_mut(12345678900000).unwrap().a == 5.0);

        let values = kv_store.iter().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
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

        let values =
            kv_store.iter_mut().map(|(id, value)| (*id, value.clone())).collect::<Vec<_>>();
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

        kv_store.cleanup();
        assert!(kv_store.len() == 0);
    }
}
