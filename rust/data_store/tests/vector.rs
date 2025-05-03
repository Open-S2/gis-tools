#[cfg(test)]
// #[coverage(off)]
mod tests {
    use data_store::{Vector, VectorStore};
    use serde::{Deserialize, Serialize};

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
