#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use gistools::data_store::{MultiMap, MultiMapStore};
    use serde::{Deserialize, Serialize};

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
