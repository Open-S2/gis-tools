#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::json::JSONReader,
    };
    use s2json::{
        BBox3D, MValue, MValueCompatible, VectorBaseGeometry, VectorFeature, VectorFeatureType,
        VectorGeometry, VectorGeometryType, VectorPoint,
    };
    use serde::{Deserialize, Serialize};
    use std::{path::PathBuf, vec, vec::Vec};

    #[test]
    fn test_json_line() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/points.geojson");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let line_del_reader = JSONReader::new(FileReader::from(path.clone()), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
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
                    face: 0.into(),
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
                    face: 0.into(),
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

        let line_del_reader = JSONReader::new(FileReader::from(path), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
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
                    face: 0.into(),
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
                    face: 0.into(),
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
}
