#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::string::String;
    use data_structures::{IndexPoint, LocalPointIndex};
    use geometry::{LonLat, S1ChordAngle, S2Point};
    use parsers::FileReader;
    use readers::json::JSONReader;
    use s2json::{MValueCompatible, Projection};
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[test]
    fn test_point_index() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i32,
        }

        let mut point_index = LocalPointIndex::new(None, None);
        assert!(point_index.is_empty());
        assert_eq!(point_index.len(), 0);

        point_index.insert_lon_lat(LonLat::new(0., 0., Some(Test { a: 0 })));
        point_index.insert_lon_lat(LonLat::new(0., 1., Some(Test { a: 1 })));
        point_index.insert_lon_lat(LonLat::new(-20., -20., Some(Test { a: 2 })));
        point_index.insert_lon_lat(LonLat::new(-22., 22., Some(Test { a: 3 })));
        point_index.insert_face_st(0.into(), 0., 0., Test { a: 5 });
        assert!(!point_index.is_empty());
        assert_eq!(point_index.len(), 5);

        point_index.sort();

        {
            let first = point_index.get_index(0).unwrap();
            assert_eq!(first.0, 290047191513211121.into());
            assert_eq!(first.1.data, Test { a: 2 });
            assert_eq!(
                first.1.s2point,
                S2Point::new(0.8830222215594891, -0.3213938048432697, -0.3420201433256688)
            );
        }

        {
            let first = point_index.get_index_mut(0).unwrap();
            assert_eq!(first.0, 290047191513211121.into());
            assert_eq!(first.1.data, Test { a: 2 });
            assert_eq!(
                first.1.s2point,
                S2Point::new(0.8830222215594891, -0.3213938048432697, -0.3420201433256688)
            );
        }

        let zero: LonLat<()> = LonLat::new(0., 0., None);
        let zer_p: S2Point = S2Point::from(&zero);
        let tw_tw: LonLat<Test> = LonLat::new(2., 2., None);
        let chord_angle = S1ChordAngle::from_s2_points(&zer_p, &S2Point::from(&tw_tw));
        {
            let radius_search = point_index.search_radius(&zero, chord_angle, None);
            assert_eq!(
                radius_search,
                vec![
                    &(
                        1152921504606846977.into(),
                        IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                    ),
                    &(
                        1153451514845492609.into(),
                        IndexPoint {
                            s2point: S2Point::new(0.9998476951563912, 0., 0.017452406437283467),
                            data: Test { a: 1 }
                        }
                    ),
                ]
            );
        }
        {
            let radius_search = point_index.search_radius(&zero, S1ChordAngle::new(-1.), None);
            assert!(radius_search.is_empty());
        }
        {
            let radius_search = point_index.search_radius_mut(&zero, S1ChordAngle::new(-1.), None);
            assert!(radius_search.is_empty());
        }
        {
            let radius_search = point_index.search_radius_mut(&zero, chord_angle, None);
            assert_eq!(
                radius_search,
                vec![
                    &(
                        1152921504606846977.into(),
                        IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                    ),
                    &(
                        1153451514845492609.into(),
                        IndexPoint {
                            s2point: S2Point::new(0.9998476951563912, 0., 0.017452406437283467),
                            data: Test { a: 1 }
                        }
                    ),
                ]
            );
        }
        {
            let radius_search = point_index.search_radius_mut(&zero, chord_angle, Some(1));
            assert_eq!(
                radius_search,
                vec![&(
                    1152921504606846977.into(),
                    IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                )]
            );
        }
        {
            let radius_search = point_index.search_radius(&zero, chord_angle, Some(1));
            assert_eq!(
                radius_search,
                vec![&(
                    1152921504606846977.into(),
                    IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                )]
            );
        }

        {
            let range_search = point_index.search_range(1152921504606846977.into(), None, None);
            assert_eq!(
                range_search,
                vec![&(
                    1152921504606846977.into(),
                    IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                )]
            );
        }
        {
            let range_search = point_index.search_range_mut(1152921504606846977.into(), None, None);
            assert_eq!(
                range_search,
                vec![&(
                    1152921504606846977.into(),
                    IndexPoint { s2point: S2Point::new(1., 0., 0.), data: Test { a: 0 } }
                )]
            );
        }

        {
            let all_data = point_index.iter().cloned().collect::<Vec<_>>();
            assert_eq!(all_data.len(), 5);
        }

        {
            let all_data = point_index.iter_mut().map(|d| d.clone()).collect::<Vec<_>>();
            assert_eq!(all_data.len(), 5);
        }
    }

    #[test]
    fn test_from_reader() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/readers/json/fixtures/multipoint.geojson");

        let reader: JSONReader<FileReader, (), Props, Props> =
            JSONReader::new(FileReader::from(path), None);

        let mut point_index = LocalPointIndex::new(None, Some(Projection::WG));
        point_index.insert_reader(&reader);
        point_index.sort();

        assert_eq!(point_index.len(), 3);

        let zero: LonLat<()> = LonLat::new(144.9584, -37.8173, None);
        // let zer_p = S2Point::from(&zero);
        let chord_angle = S1ChordAngle::new(0.0001);
        {
            let radius_search = point_index.search_radius(&zero, chord_angle, None);
            assert_eq!(
                radius_search,
                vec![&(
                    1706183736615716769.into(),
                    IndexPoint {
                        s2point: S2Point::new(
                            0.7916652029431729,
                            0.5961757122268558,
                            0.1335691828220351
                        ),
                        data: Props { name: "Melbourne".into() }
                    }
                )]
            );
        }
    }

    #[test]
    fn test_from_reader_s2() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/readers/json/fixtures/multipoint.geojson");

        let reader: JSONReader<FileReader, (), Props, Props> =
            JSONReader::new(FileReader::from(path), None);

        let mut point_index = LocalPointIndex::new(None, None);
        point_index.insert_reader(&reader);
        point_index.sort();

        let zero: LonLat<()> = LonLat::new(144.9584, -37.8173, None);
        // let zer_p = S2Point::from(&zero);
        let chord_angle = S1ChordAngle::new(0.0001);
        {
            let radius_search = point_index.search_radius(&zero, chord_angle, None);
            assert_eq!(
                radius_search,
                vec![&(
                    7698443195519755875.into(),
                    IndexPoint {
                        s2point: S2Point::new(
                            -0.6467763171329768,
                            0.45357784406278107,
                            -0.6131456066639167
                        ),
                        data: Props { name: "Melbourne".into() }
                    }
                )]
            );
        }
    }
}
