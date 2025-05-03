mod large_json;
mod line_delimited;

#[cfg(test)]
#[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::string::String;
    use parsers::FeatureReader;
    use readers::json::{JSONCollectionReader, ToGisJSON};
    use s2json::{
        BBox3D, BaseGeometry, Feature, FeatureCollection, FeatureCollectionType, FeatureType,
        Features, Geometry, GeometryType, JSONCollection, MValue, MValueCompatible, Point,
        S2FeatureCollection, S2FeatureCollectionType, VectorBaseGeometry, VectorFeature,
        VectorFeatureType, VectorGeometry, VectorGeometryType, VectorPoint,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_json_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [100.0, 0.0]
            },
            "properties": {
                "name": "Tokyo"
            }
        }"#;
        let mut json: Feature<(), Test, MValue> = json_str.to_feature().unwrap();
        assert_eq!(
            json,
            Feature {
                _type: FeatureType::Feature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: Geometry::Point(BaseGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(100.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);

        let json_collection = JSONCollection::Feature(json.clone());
        let json_coll_str = serde_json::to_string(&json_collection).unwrap();
        assert_eq!(
            json_coll_str,
            "{\"type\":\"Feature\",\"properties\":{\"name\":\"\"},\"geometry\":{\"type\":\"Point\"\
             ,\"coordinates\":[0.0,0.0]}}"
        );

        let json_collection: JSONCollection<(), Test, MValue> = json_str.to_gis_json().unwrap();
        assert_eq!(
            json_collection,
            JSONCollection::Feature(Feature {
                _type: FeatureType::Feature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: Geometry::Point(BaseGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(100.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_json_feature_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "FeatureCollection",
            "features": [{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [100.0, 0.0]
            },
            "properties": {
                "name": "Tokyo"
            }
        }]
        }
        "#;
        let mut json: FeatureCollection<(), Test, MValue> =
            json_str.to_feature_collection().unwrap();
        assert_eq!(
            json,
            FeatureCollection {
                features: vec![Features::Feature(Feature {
                    _type: FeatureType::Feature,
                    id: None,
                    properties: Test { name: "Tokyo".into() },
                    geometry: Geometry::Point(BaseGeometry {
                        _type: GeometryType::Point,
                        coordinates: Point(100.0, 0.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                _type: FeatureCollectionType::FeatureCollection,
                attributions: None,
                bbox: None,
            }
        );

        let mut collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.clone().into_iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> =
            collection.iter_mut().map(|f| f.clone()).collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn test_json_vector_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "VectorFeature",
            "face": 0,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }"#;
        let mut json: VectorFeature<(), Test, MValue> = json_str.to_vector_feature().unwrap();
        assert_eq!(
            json,
            VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_vector_feature_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "FeatureCollection",
            "features": [{
            "type": "VectorFeature",
            "face": 0,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }]
        }
        "#;
        let mut json: FeatureCollection<(), Test, MValue> =
            json_str.to_feature_collection().unwrap();
        assert_eq!(
            json,
            FeatureCollection {
                features: vec![Features::VectorFeature(VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    properties: Test { name: "Tokyo".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                _type: FeatureCollectionType::FeatureCollection,
                attributions: None,
                bbox: None,
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_s2_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "S2Feature",
            "face": 1,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }"#;
        let mut json: VectorFeature<(), Test, MValue> = json_str.to_vector_feature().unwrap();
        assert_eq!(
            json,
            VectorFeature {
                _type: VectorFeatureType::S2Feature,
                face: 1.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_vector_s2_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "S2FeatureCollection",
            "faces": [1],
            "features": [{
            "type": "S2Feature",
            "face": 1,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }]
        }
        "#;
        let mut json: S2FeatureCollection<(), Test, MValue> =
            json_str.to_s2_feature_collection().unwrap();
        assert_eq!(
            json,
            S2FeatureCollection {
                features: vec![VectorFeature {
                    _type: VectorFeatureType::S2Feature,
                    face: 1.into(),
                    properties: Test { name: "Tokyo".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                _type: S2FeatureCollectionType::S2FeatureCollection,
                attributions: None,
                bbox: None,
                faces: vec![1.into()],
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }
}
