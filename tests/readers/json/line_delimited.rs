#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{
            GISReader, ReaderType,
            json::{NewLineDelimitedJSONReader, SequenceJSONReader},
        },
    };
    use s2json::{
        BBox3D, Face, MValue, MValueCompatible, Properties, VectorBaseGeometry, VectorFeature,
        VectorFeatureType, VectorGeometry, VectorGeometryType, VectorPoint,
    };
    use serde::{Deserialize, Serialize};
    use std::{path::PathBuf, vec, vec::Vec};

    #[test]
    fn test_json_line_delimited() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/points.geojsonld");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let line_del_reader = NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Melbourne".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 144.9584,
                            y: -37.8173,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 144.9584,
                            bottom: -37.8173,
                            right: 144.9584,
                            top: -37.8173,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Canberra".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 149.1009,
                            y: -35.3039,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 149.1009,
                            bottom: -35.3039,
                            right: 149.1009,
                            top: -35.3039,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Sydney".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 151.2144,
                            y: -33.8766,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 151.2144,
                            bottom: -33.8766,
                            right: 151.2144,
                            top: -33.8766,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let line_del_reader = NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Melbourne".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 144.9584,
                            y: -37.8173,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 144.9584,
                            bottom: -37.8173,
                            right: 144.9584,
                            top: -37.8173,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Canberra".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 149.1009,
                            y: -35.3039,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 149.1009,
                            bottom: -35.3039,
                            right: 149.1009,
                            top: -35.3039,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Sydney".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 151.2144,
                            y: -33.8766,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 151.2144,
                            bottom: -33.8766,
                            right: 151.2144,
                            top: -33.8766,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let features: Vec<VectorFeature<(), Test, MValue>> =
            line_del_reader.par_iter(1, 0).collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Melbourne".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 144.9584,
                            y: -37.8173,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 144.9584,
                            bottom: -37.8173,
                            right: 144.9584,
                            top: -37.8173,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Canberra".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 149.1009,
                            y: -35.3039,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 149.1009,
                            bottom: -35.3039,
                            right: 149.1009,
                            top: -35.3039,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { name: "Sydney".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 151.2144,
                            y: -33.8766,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 151.2144,
                            bottom: -33.8766,
                            right: 151.2144,
                            top: -33.8766,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
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
    fn test_json_line_seq() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/features.geojsonseq");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            prop0: String,
        }

        let seq_del_reader = SequenceJSONReader::new(FileReader::from(path.clone()));
        let features: Vec<VectorFeature<(), Test, MValue>> = seq_del_reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 102.0, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.5,
                            right: 102.0,
                            top: 0.5,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 102.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 103.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 104.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 105.0, y: 1.0, z: None, m: None, t: None }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.0,
                            right: 105.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Polygon(VectorBaseGeometry {
                        _type: VectorGeometryType::Polygon,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None }
                        ]],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 100.0,
                            bottom: 0.0,
                            right: 101.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let seq_del_reader = SequenceJSONReader::new(FileReader::from(path));
        let features: Vec<VectorFeature<(), Test, MValue>> = seq_del_reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 102.0, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.5,
                            right: 102.0,
                            top: 0.5,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 102.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 103.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 104.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 105.0, y: 1.0, z: None, m: None, t: None }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.0,
                            right: 105.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Polygon(VectorBaseGeometry {
                        _type: VectorGeometryType::Polygon,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None }
                        ]],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 100.0,
                            bottom: 0.0,
                            right: 101.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let features: Vec<VectorFeature<(), Test, MValue>> =
            seq_del_reader.par_iter(1, 0).collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 102.0, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.5,
                            right: 102.0,
                            top: 0.5,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 102.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 103.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 104.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 105.0, y: 1.0, z: None, m: None, t: None }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.0,
                            right: 105.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: Face::WM,
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Polygon(VectorBaseGeometry {
                        _type: VectorGeometryType::Polygon,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None }
                        ]],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 100.0,
                            bottom: 0.0,
                            right: 101.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
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
    fn test_json_line_delimited_larger() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/larger.jsonld");

        let line_del_reader = NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        let features: Vec<VectorFeature<(), Properties, MValue>> = line_del_reader.collect();

        assert_eq!(features.len(), 1_064);
    }

    #[test]
    fn test_json_line_delimited_larger_parallel() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/larger.jsonld");

        let line_del_reader: NewLineDelimitedJSONReader<FileReader, (), Properties, MValue> =
            NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);

        let features: Vec<VectorFeature<(), Properties, MValue>> = (0..3usize)
            .into_iter()
            .flat_map(|thread_id| {
                let reader = line_del_reader.clone();
                let res: Vec<_> = reader.par_iter(3, thread_id).collect();
                res
            })
            .collect();

        assert_eq!(features.len(), 1_064);
    }

    #[test]
    fn test_jsonld_gis_reader() {
        // file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/points.geojsonld");
        let gis_reader = GISReader::from_path(path.clone(), None, None);
        assert_eq!(gis_reader.get_type(), ReaderType::JSONLD);
        let features: Vec<_> = gis_reader.iter().collect();
        assert_eq!(features.len(), 3);

        // buffer
        let bytes = std::fs::read(path.clone()).unwrap();
        let gis_reader = GISReader::from_buffer(bytes, ReaderType::JSONLD, None);
        let features: Vec<_> = gis_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 3);
    }

    #[test]
    fn test_jsonseq_gis_reader() {
        // file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/features.geojsonseq");
        let gis_reader = GISReader::from_path(path.clone(), None, None);
        assert_eq!(gis_reader.get_type(), ReaderType::JSONSQ);
        let features: Vec<_> = gis_reader.iter().collect();
        assert_eq!(features.len(), 3);

        // buffer
        let bytes = std::fs::read(path.clone()).unwrap();
        let gis_reader = GISReader::from_buffer(bytes, ReaderType::JSONSQ, None);
        let features: Vec<_> = gis_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 3);
    }
}
