#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{CSVReader, CSVReaderOptions, parse_csv_as_record, parse_csv_line},
    };
    use s2json::{
        MValue, MValueCompatible, VectorBaseGeometry, VectorFeature, VectorFeatureType,
        VectorGeometry, VectorGeometryType, VectorPoint,
    };
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[test]
    fn test_parse_csv_line() {
        assert_eq!(parse_csv_line("a,b,c", ','), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_csv_as_record() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }
        let source = "a,b,c\n1,2,3\n4,5,6";
        let res = parse_csv_as_record::<MValue>(source, None, None);
        assert_eq!(
            res,
            vec![
                MValue::from([
                    ("a".into(), "1".into()),
                    ("b".into(), "2".into()),
                    ("c".into(), "3".into()),
                ]),
                MValue::from([
                    ("a".into(), "4".into()),
                    ("b".into(), "5".into()),
                    ("c".into(), "6".into()),
                ]),
            ]
        );

        let res = parse_csv_as_record::<Test>(source, None, None);
        assert_eq!(
            res,
            vec![
                Test { a: "1".into(), b: "2".into(), c: "3".into() },
                Test { a: "4".into(), b: "5".into(), c: "6".into() },
            ]
        );
    }

    #[test]
    fn test_parse_csv_with_options_and_empty_fields() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
        struct TestOpt {
            a: i64,
            b: Option<i64>,
            c: Option<i64>,
        }

        let source = "a,b,c\n1,,3\n4,5,";
        let res = parse_csv_as_record::<TestOpt>(source, None, None);

        assert_eq!(
            res,
            vec![TestOpt { a: 1, b: None, c: Some(3) }, TestOpt { a: 4, b: Some(5), c: None },]
        );
    }

    #[test]
    fn test_csv_reader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/csv/fixtures/basic.csv");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let reader = CSVReader::new(FileReader::from(path), None);

        let features: Vec<VectorFeature<(), Test, ()>> = reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "3".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 2.0, y: 1.0, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "a".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 1.1, y: 3.2, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let features: Vec<VectorFeature<(), Test, ()>> = reader.par_iter(1, 1).collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "3".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 2.0, y: 1.0, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "a".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 1.1, y: 3.2, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn test_csv_reader_3d() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/csv/fixtures/basic3D.csv");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let reader = CSVReader::new(
            FileReader::from(path),
            Some(CSVReaderOptions {
                delimiter: Some(','),
                line_delimiter: Some('\n'),
                lon_key: Some("Longitude".into()),
                lat_key: Some("Latitude".into()),
                height_key: Some("height".into()),
            }),
        );

        let features: Vec<VectorFeature<(), Test, ()>> = reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "3".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 2.0,
                            y: 1.0,
                            z: Some(55.0),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "a".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 1.1,
                            y: 3.2,
                            z: Some(-2.2),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: None,
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }
}
