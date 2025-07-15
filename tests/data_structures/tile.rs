#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use core::f64;
    use gistools::{
        data_structures::{Tile, TileStore, TileStoreOptions, TransformVectorGeometry},
        geometry::S2CellId,
    };
    use s2json::{
        BBox3D, Face, JSONCollection, MValue, Map, Projection, VectorFeature, VectorGeometry,
        VectorLineStringGeometry, VectorMultiLineStringGeometry, VectorMultiPointGeometry,
        VectorPoint, VectorPointGeometry,
    };
    use std::collections::{BTreeMap, BTreeSet};

    const SIMPLIFY_MAXZOOM: u8 = 16;

    #[test]
    fn test_transform() {
        let mut p: VectorPoint = VectorPoint { x: 0.5, y: 0.5, z: Some(0.0), m: None, t: None };
        p.transform(10.0, 0.0, 0.0);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 5.0);

        let mut p: VectorPoint = VectorPoint { x: 0., y: 0., z: Some(0.0), m: None, t: None };
        p.transform(1., 0., 0.);
        assert_eq!(p.x, 0.);
        assert_eq!(p.y, 0.);

        let mut p: VectorPoint = VectorPoint { x: 0., y: 0., z: Some(0.0), m: None, t: None };
        p.transform(1., 1., 0.);
        assert_eq!(p.x, -1.);
        assert_eq!(p.y, -0.);
    }

    #[test]
    fn test_tile() {
        let mut tile: Tile = Tile::new(S2CellId::from_face(0));
        assert_eq!(
            tile,
            Tile { id: 1152921504606846976.into(), layers: BTreeMap::new(), transformed: false }
        );
        assert!(tile.is_empty());
        assert_eq!(tile.len(), 0);

        tile.add_feature(
            VectorFeature::new_wm(
                None,
                Map::new(),
                VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    ..Default::default()
                }),
                None,
            ),
            Some("default".into()),
        );

        assert!(!tile.is_empty());
        assert_eq!(tile.len(), 1);

        tile.transform(3., Some(SIMPLIFY_MAXZOOM));
        // call it again (it will fail)
        tile.transform(3., Some(SIMPLIFY_MAXZOOM));

        // grab the feature
        let layer = tile.layers.get("default").unwrap();
        let first_feature = layer.features.first().unwrap();
        assert_eq!(
            first_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_tile_store() {
        let tile_store: TileStore = TileStore::default();
        assert_eq!(
            tile_store,
            TileStore {
                minzoom: 0,
                maxzoom: 14,
                faces: BTreeSet::<Face>::new(),
                index_maxzoom: 4,
                tolerance: 0.000732421875,
                buffer: 0.0625,
                tiles: BTreeMap::new(),
                projection: Projection::S2,
            }
        );
    }

    #[test]
    fn test_tile_store_wg_points() {
        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": { "a": 1 },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "b": 2 },
                    "geometry": {
                        "type": "Point3D",
                        "coordinates": [45, 45, 1]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "c": 3 },
                    "geometry": {
                        "type": "MultiPoint",
                        "coordinates": [
                            [-45, -45],
                            [-45, 45]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "d": 4 },
                    "geometry": {
                        "type": "MultiPoint3D",
                        "coordinates": [
                            [45, -45, 1],
                            [-180, 20, 2]
                        ]
                    }
                }
            ]
        }"#;
        let data: JSONCollection = serde_json::from_str(json_string).unwrap();
        let mut tile_store: TileStore = TileStore::<_, _, _>::new(
            data,
            TileStoreOptions {
                projection: Some(Projection::WG),
                maxzoom: Some(4),
                ..Default::default()
            },
        );

        let face_0_tile = tile_store.get_tile(S2CellId::from_face(0)).unwrap();
        assert_eq!(face_0_tile.len(), 1);
        let default_layer = face_0_tile.layers.get("default").unwrap();
        assert_eq!(default_layer.features.len(), 4);

        assert_eq!(
            default_layer.features,
            vec![
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("a".into(), 1_u64.into())]),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 0.0,
                            bottom: 0.0,
                            right: 0.0,
                            top: 0.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.5,
                            top: 0.5,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("b".into(), 2_u64.into())]),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 0.625,
                            y: 0.35972503691520497,
                            z: Some(1.0),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 45.0,
                            bottom: 45.0,
                            right: 45.0,
                            top: 45.0,
                            near: 1.0,
                            far: 1.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.625,
                            bottom: 0.35972503691520497,
                            right: 0.625,
                            top: 0.35972503691520497,
                            near: 1.0,
                            far: 1.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("c".into(), 3_u64.into())]),
                    geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                        _type: "MultiPoint".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.375,
                                y: 0.640274963084795,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.375,
                                y: 0.35972503691520497,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -45.0,
                            bottom: -45.0,
                            right: -45.0,
                            top: 45.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.375,
                            bottom: 0.35972503691520497,
                            right: 0.375,
                            top: 0.640274963084795,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("d".into(), 4_u64.into())]),
                    geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                        _type: "MultiPoint".into(),
                        is_3d: true,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.625,
                                y: 0.640274963084795,
                                z: Some(1.0),
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.0,
                                y: 0.4432805993614054,
                                z: Some(2.0),
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -180.0,
                            bottom: -45.0,
                            right: 45.0,
                            top: 20.0,
                            near: 1.0,
                            far: 2.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.0,
                            bottom: 0.4432805993614054,
                            right: 0.625,
                            top: 0.640274963084795,
                            near: 1.0,
                            far: 2.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let children = face_0_tile.id.children(None);

        let zero_child = children[0];
        let zero_child_tile = tile_store.get_tile(zero_child).unwrap();
        assert_eq!(zero_child_tile.len(), 1);

        let first_child = children[1];
        let first_child_tile = tile_store.get_tile(first_child).unwrap();
        assert_eq!(first_child_tile.len(), 1);

        let second_child = children[2];
        let second_child_tile = tile_store.get_tile(second_child).unwrap();
        assert_eq!(second_child_tile.len(), 1);

        let third_child = children[3];
        let third_child_tile = tile_store.get_tile(third_child).unwrap();
        assert_eq!(third_child_tile.len(), 1);
    }

    #[test]
    fn test_tile_store_s2_points() {
        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": { "a": 1 },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "b": 2 },
                    "geometry": {
                        "type": "Point3D",
                        "coordinates": [45, 45, 1]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "c": 3 },
                    "geometry": {
                        "type": "MultiPoint",
                        "coordinates": [
                            [-45, -45],
                            [-45, 45]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "d": 4 },
                    "geometry": {
                        "type": "MultiPoint3D",
                        "coordinates": [
                            [45, -45, 1],
                            [-180, 20, 2]
                        ]
                    }
                }
            ]
        }"#;
        let data: JSONCollection = serde_json::from_str(json_string).unwrap();
        let mut tile_store: TileStore = TileStore::<_, _, _>::new(
            data,
            TileStoreOptions { projection: Some(Projection::S2), ..Default::default() },
        );

        let face_0_tile = tile_store.get_tile(S2CellId::from_face(0)).unwrap();
        assert_eq!(face_0_tile.len(), 1);
        let default_layer = face_0_tile.layers.get("default").unwrap();
        assert_eq!(default_layer.features.len(), 1);

        assert_eq!(
            default_layer.features,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: None,
                face: 0.into(),
                properties: MValue::from([("a".into(), 1_u64.into())]),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    offset: None,
                    bbox: Some(BBox3D {
                        left: 0.0,
                        bottom: 0.0,
                        right: 0.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    vec_bbox: Some(BBox3D {
                        left: 0.5,
                        bottom: 0.5,
                        right: 0.5,
                        top: 0.5,
                        near: f64::MAX,
                        far: f64::MIN
                    }),
                    indices: None,
                    tessellation: None
                }),
                metadata: None
            }]
        );
    }

    #[test]
    fn test_tile_store_wg_lines() {
        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {},
                    "geometry": {
                        "type": "LineString",
                        "coordinates": [
                            [-13.292352825505162, 54.34883408204476],
                            [36.83102287804303, 59.56941785818924],
                            [50.34083898563978, 16.040052775278994],
                            [76.38149901912357, 35.155968522292056]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": {},
                    "geometry": {
                        "type": "MultiLineString3D",
                        "coordinates": [
                            [
                                [138.2192704758947, 53.37525605304839, -1.0],
                                [138.02907780308504, 45.48182328687463, 2.0],
                                [166.1775933788045, 52.68902110529311, 4.0],
                                [161.99335457700874, 40.765696887535825, -0.5]
                            ], [
                                [139.16452129458895, -69.38636090051318, 1.0],
                                [143.85299782010844, -63.55049044056966, 2.0],
                                [128.5373078367444, -51.22800042702269, -0.5],
                                [134.78860987076968, -45.63638565920266, 8.0]
                            ]
                        ]
                    }
                }
            ]
        }"#;
        let data: JSONCollection = serde_json::from_str(json_string).unwrap();
        let mut tile_store: TileStore = TileStore::<_, _, _>::new(
            data,
            TileStoreOptions { projection: Some(Projection::WG), ..Default::default() },
        );

        let face_0_tile = tile_store.get_tile(S2CellId::from_face(0)).unwrap();
        assert_eq!(face_0_tile.len(), 1);
        let default_layer = face_0_tile.layers.get("default").unwrap();
        assert_eq!(default_layer.features.len(), 2);

        // [], []], offset: None, bbox: None, vec_bbox: Some(BBox3D { left: 0.8570480773242899, bottom: 0.3240121995384903, right: 0.9616044260522347, top: 0.7712879476591746, near: -1.0, far: 8.0 }), indices: None, tessellation: None }), metadata: None }]

        assert_eq!(
            default_layer.features,
            vec![
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.4630767977069301,
                                y: 0.31942614957229354,
                                z: None,
                                m: None,
                                t: Some(1.),
                            },
                            VectorPoint {
                                x: 0.6023083968834528,
                                y: 0.29277635129241236,
                                z: None,
                                m: None,
                                t: Some(0.01120038734713082),
                            },
                            VectorPoint {
                                x: 0.6398356638489994,
                                y: 0.45485063470883236,
                                z: None,
                                m: None,
                                t: Some(0.00605876326361668)
                            },
                            VectorPoint {
                                x: 0.7121708306086766,
                                y: 0.3955684303719546,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -13.292352825505162,
                            bottom: 16.040052775278994,
                            right: 76.38149901912357,
                            top: 59.56941785818924,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.4630767977069301,
                            bottom: 0.29277635129241236,
                            right: 0.7121708306086766,
                            top: 0.45485063470883236,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                        _type: "MultiLineString".into(),
                        is_3d: true,
                        coordinates: vec![
                            vec![
                                VectorPoint {
                                    x: 0.8839424179885964,
                                    y: 0.3240121995384903,
                                    z: Some(-1.0),
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.8834141050085695,
                                    y: 0.3578242302600759,
                                    z: Some(2.0),
                                    m: None,
                                    t: Some(0.0011428082308213008)
                                },
                                VectorPoint {
                                    x: 0.9616044260522347,
                                    y: 0.32718207741863975,
                                    z: Some(4.0),
                                    m: None,
                                    t: Some(0.0020631440003536124)
                                },
                                VectorPoint {
                                    x: 0.9499815404916909,
                                    y: 0.37578687158091856,
                                    z: Some(-0.5),
                                    m: None,
                                    t: Some(1.0)
                                }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.8865681147071915,
                                    y: 0.7712879476591746,
                                    z: Some(1.0),
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.8995916606114123,
                                    y: 0.730480837159282,
                                    z: Some(2.0),
                                    m: None,
                                    t: Some(0.0005558708889643396)
                                },
                                VectorPoint {
                                    x: 0.8570480773242899,
                                    y: 0.6662313440317926,
                                    z: Some(-0.5),
                                    m: None,
                                    t: Some(0.0003800636747767906)
                                },
                                VectorPoint {
                                    x: 0.8744128051965825,
                                    y: 0.6427889614041957,
                                    z: Some(8.0),
                                    m: None,
                                    t: Some(1.0)
                                }
                            ]
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 128.5373078367444,
                            bottom: -69.38636090051318,
                            right: 166.1775933788045,
                            top: 53.37525605304839,
                            near: -1.0,
                            far: 8.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.8570480773242899,
                            bottom: 0.3240121995384903,
                            right: 0.9616044260522347,
                            top: 0.7712879476591746,
                            near: -1.0,
                            far: 8.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }
}
