#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        data_structures::{BoxIndex, BoxIndexAccessor},
        proj::Coords,
    };
    use s2json::BBox;

    const DATA: &[u8] = &[
        8, 62, 11, 66, 57, 17, 57, 19, 76, 26, 79, 29, 36, 56, 38, 56, 92, 77, 96, 80, 87, 70, 90,
        74, 43, 41, 47, 43, 0, 58, 2, 62, 76, 86, 80, 89, 27, 13, 27, 15, 71, 63, 75, 67, 25, 2,
        27, 2, 87, 6, 88, 6, 22, 90, 23, 93, 22, 89, 22, 93, 57, 11, 61, 13, 61, 55, 63, 56, 17,
        85, 21, 87, 33, 43, 37, 43, 6, 1, 7, 3, 80, 87, 80, 87, 23, 50, 26, 52, 58, 89, 58, 89, 12,
        30, 15, 34, 32, 58, 36, 61, 41, 84, 44, 87, 44, 18, 44, 19, 13, 63, 15, 67, 52, 70, 54, 74,
        57, 59, 58, 59, 17, 90, 20, 92, 48, 53, 52, 56, 92, 68, 92, 72, 26, 52, 30, 52, 56, 23, 57,
        26, 88, 48, 88, 48, 66, 13, 67, 15, 7, 82, 8, 86, 46, 68, 50, 68, 37, 33, 38, 36, 6, 15, 8,
        18, 85, 36, 89, 38, 82, 45, 84, 48, 12, 2, 16, 3, 26, 15, 26, 16, 55, 23, 59, 26, 76, 37,
        79, 39, 86, 74, 90, 77, 16, 75, 18, 78, 44, 18, 45, 21, 52, 67, 54, 71, 59, 78, 62, 78, 24,
        5, 24, 8, 64, 80, 64, 83, 66, 55, 70, 55, 0, 17, 2, 19, 15, 71, 18, 74, 87, 57, 87, 59, 6,
        34, 7, 37, 34, 30, 37, 32, 51, 19, 53, 19, 72, 51, 73, 55, 29, 45, 30, 45, 94, 94, 96, 95,
        7, 22, 11, 24, 86, 45, 87, 48, 33, 62, 34, 65, 18, 10, 21, 14, 64, 66, 67, 67, 64, 25, 65,
        28, 27, 4, 31, 6, 84, 4, 85, 5, 48, 80, 50, 81, 1, 61, 3, 61, 71, 89, 74, 92, 40, 42, 43,
        43, 27, 64, 28, 66, 46, 26, 50, 26, 53, 83, 57, 87, 14, 75, 15, 79, 31, 45, 34, 45, 89, 84,
        92, 88, 84, 51, 85, 53, 67, 87, 67, 89, 39, 26, 43, 27, 47, 61, 47, 63, 23, 49, 25, 53, 12,
        3, 14, 5, 16, 50, 19, 53, 63, 80, 64, 84, 22, 63, 22, 64, 26, 66, 29, 66, 2, 15, 3, 15, 74,
        77, 77, 79, 64, 11, 68, 11, 38, 4, 39, 8, 83, 73, 87, 77, 85, 52, 89, 56, 74, 60, 76, 63,
        62, 66, 65, 67,
    ];

    /// Test item
    #[derive(Debug, Clone, PartialEq)]
    struct Item {
        pub id: usize,
        pub min_x: f64,
        pub min_y: f64,
        pub max_x: f64,
        pub max_y: f64,
    }
    impl BoxIndexAccessor for Item {
        fn bbox(&self) -> BBox {
            BBox::new(self.min_x, self.min_y, self.max_x, self.max_y)
        }
    }

    fn create_index() -> BoxIndex<Item> {
        let items = DATA
            .chunks(4)
            .enumerate()
            .map(|(id, chunk)| Item {
                id,
                min_x: chunk[0] as f64,
                min_y: chunk[1] as f64,
                max_x: chunk[2] as f64,
                max_y: chunk[3] as f64,
            })
            .collect();

        BoxIndex::new(items, None)
    }

    #[test]
    fn box_index_basic_test() {
        let mut index = BoxIndex::<Item>::new(vec![], None);
        index.index();

        assert_eq!(index.take(), vec![]);
    }

    #[test]
    fn box_index_lots_of_rectangles() {
        let index = create_index();
        assert_eq!(index.bbox, BBox::new(0.0, 1.0, 96.0, 95.0));
    }

    #[test]
    fn box_index_search() {
        let index = create_index();
        let items = index.search(&BBox::new(40.0, 40.0, 60.0, 60.0), None::<fn(&Item) -> bool>);
        assert_eq!(items.len(), 4);
        assert_eq!(
            items,
            vec![
                Item { id: 29, min_x: 57.0, min_y: 59.0, max_x: 58.0, max_y: 59.0 },
                Item { id: 31, min_x: 48.0, min_y: 53.0, max_x: 52.0, max_y: 56.0 },
                Item { id: 6, min_x: 43.0, min_y: 41.0, max_x: 47.0, max_y: 43.0 },
                Item { id: 75, min_x: 40.0, min_y: 42.0, max_x: 43.0, max_y: 43.0 }
            ]
        );
    }

    #[test]
    fn box_index_neighbors() {
        let index = create_index();
        let items =
            index.neighbors(Coords::new_xy(50., 50.), Some(3), None, None::<fn(&Item) -> bool>);
        assert_eq!(items.len(), 3);
        assert_eq!(
            items,
            vec![
                Item { id: 31, min_x: 48.0, min_y: 53.0, max_x: 52.0, max_y: 56.0 },
                Item { id: 6, min_x: 43.0, min_y: 41.0, max_x: 47.0, max_y: 43.0 },
                Item { id: 75, min_x: 40.0, min_y: 42.0, max_x: 43.0, max_y: 43.0 }
            ]
        );
    }

    #[test]
    fn box_index_neighbors_max_distance() {
        let index = create_index();
        let items =
            index.neighbors(Coords::new_xy(50., 50.), None, Some(12.), None::<fn(&Item) -> bool>);
        assert_eq!(items.len(), 5);
        assert_eq!(
            items,
            vec![
                Item { id: 31, min_x: 48.0, min_y: 53.0, max_x: 52.0, max_y: 56.0 },
                Item { id: 6, min_x: 43.0, min_y: 41.0, max_x: 47.0, max_y: 43.0 },
                Item { id: 75, min_x: 40.0, min_y: 42.0, max_x: 43.0, max_y: 43.0 },
                Item { id: 85, min_x: 47.0, min_y: 61.0, max_x: 47.0, max_y: 63.0 },
                Item { id: 29, min_x: 57.0, min_y: 59.0, max_x: 58.0, max_y: 59.0 }
            ]
        );
    }

    #[test]
    fn box_index_neighbors_with_filter() {
        let index = create_index();
        let items = index.neighbors(
            Coords::new_xy(50., 50.),
            Some(6),
            None,
            Some(|item: &Item| item.min_x > 50.),
        );
        assert_eq!(items.len(), 6);
        assert_eq!(
            items,
            vec![
                Item { id: 29, min_x: 57.0, min_y: 59.0, max_x: 58.0, max_y: 59.0 },
                Item { id: 16, min_x: 61.0, min_y: 55.0, max_x: 63.0, max_y: 56.0 },
                Item { id: 54, min_x: 66.0, min_y: 55.0, max_x: 70.0, max_y: 55.0 },
                Item { id: 50, min_x: 52.0, min_y: 67.0, max_x: 54.0, max_y: 71.0 },
                Item { id: 99, min_x: 62.0, min_y: 66.0, max_x: 65.0, max_y: 67.0 },
                Item { id: 28, min_x: 52.0, min_y: 70.0, max_x: 54.0, max_y: 74.0 }
            ]
        );
    }
}
