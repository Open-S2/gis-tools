#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{ConvertVectorFeatureWM, FACE_RULE_SET, Rotation, convert};
    use s2json::{
        BBox3D, FeatureCollection, Features, JSONCollection, MValue, Projection,
        S2FeatureCollection, VectorFeature, VectorGeometry, VectorLineStringGeometry, VectorPoint,
        VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_face_rule_set() {
        assert_eq!(FACE_RULE_SET[0][0], (Rotation::_0, 0, 0));
    }

    #[test]
    fn to_s2_point() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_wm_vectorpoint_convert() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = convert(
            Projection::WG,
            &JSONCollection::VectorFeature(feature),
            Some(true),
            Some(true),
        );

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "VectorFeature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_point_fc_convert() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let fc: FeatureCollection = FeatureCollection {
            _type: "FeatureCollection".into(),
            features: vec![Features::VectorFeature(feature.clone())],
            ..Default::default()
        };
        let s2_feature =
            convert(Projection::S2, &JSONCollection::FeatureCollection(fc), Some(true), None);

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_already_s2() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let s2f: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: Some(1337),
            face: 0.into(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint::new(0.5, 0.5, None, m_value.clone()),
                bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let s2fc: S2FeatureCollection = S2FeatureCollection {
            _type: "S2FeatureCollection".into(),
            features: vec![s2f.clone()],
            ..Default::default()
        };
        let s2_feature =
            convert(Projection::S2, &JSONCollection::S2FeatureCollection(s2fc), Some(true), None);

        assert_eq!(s2_feature, vec![s2f]);
    }

    #[test]
    fn to_s2_point_3d() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., Some(1.), m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: true,
                    coordinates: VectorPoint::new(0.5, 0.5, Some(1.), m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 1., 1.)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_multipoint() {
        let coords = vec![
            VectorPoint::new(0., 0., None, Some(MValue::from([("a".into(), (1_u64).into())]))),
            VectorPoint::new(-180., -90., None, Some(MValue::from([("b".into(), (2_u64).into())]))),
            VectorPoint::new(180., 90., None, Some(MValue::from([("c".into(), (3_u64).into())]))),
        ];
        let properties = MValue::from([("d".into(), (4_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multipoint(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 5.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn to_s2_linestring() {
        let coords = vec![
            VectorPoint::new(0., 0., None, Some(MValue::from([("a".into(), (1_u64).into())]))),
            VectorPoint::new(20., 20., None, Some(MValue::from([("b".into(), (2_u64).into())]))),
            VectorPoint::new(30., 30., None, Some(MValue::from([("c".into(), (3_u64).into())]))),
            VectorPoint::new(40., 40., None, Some(MValue::from([("d".into(), (4_u64).into())]))),
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_linestring(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.5,
                                y: 0.5,
                                z: None,
                                m: Some(MValue::from([("a".into(), (1_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.7231719544476624,
                                y: 0.7351848576118168,
                                z: None,
                                m: Some(MValue::from([("b".into(), (2_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.8264458251405347,
                                y: 0.8660254037844386,
                                z: None,
                                m: Some(MValue::from([("c".into(), (3_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.6953495465482081,
                                y: 1.0625,
                                z: None,
                                m: Some(MValue::from([("d".into(), (4_u64).into())])),
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.8264458251405347,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.17012925937810885,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(1.5284052199258356),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.17012925937810885,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_multilinestring() {
        let coords = vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(40., 40., None, None),
            ],
            vec![
                VectorPoint::new(-120., -30., None, None),
                VectorPoint::new(-130., -40., None, None),
                VectorPoint::new(-140., -50., None, None),
                VectorPoint::new(-150., -60., None, None),
            ],
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multilinestring(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 4);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                            VectorPoint {
                                x: 0.7231719544476624,
                                y: 0.7351848576118168,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.8264458251405347,
                                y: 0.8660254037844386,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.6953495465482081,
                                y: 1.0625,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.8264458251405347,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.17012925937810885,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(1.5284052199258356),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.17012925937810885,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 4.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.8660254037844386,
                                y: 0.17355417485946534,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 1.0332000398839454,
                                y: 0.0919618222017129,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 1.0625,
                                y: 0.1016957300340185,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.8660254037844386,
                            bottom: 0.0919618222017129,
                            right: 1.0625,
                            top: 0.17355417485946534,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 5.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.13866981323286479,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.0919618222017129,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.1909745772474294,
                                y: 0.14437700634864636,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.3169872981077806,
                                y: 0.209430584957905,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(0.07953324204553078),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.0919618222017129,
                            right: 0.3169872981077806,
                            top: 0.209430584957905,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_polygon() {
        let coords = vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 0., None, None),
                VectorPoint::new(40., 0., None, None),
                VectorPoint::new(40., 20., None, None),
                VectorPoint::new(40., 40., None, None),
                VectorPoint::new(20., 40., None, None),
                VectorPoint::new(0., 40., None, None),
                VectorPoint::new(0., 20., None, None),
                VectorPoint::new(0., 0., None, None),
            ],
            vec![
                VectorPoint::new(10., 10., None, None),
                VectorPoint::new(20., 10., None, None),
                VectorPoint::new(30., 10., None, None),
                VectorPoint::new(30., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(20., 30., None, None),
                VectorPoint::new(10., 30., None, None),
                VectorPoint::new(10., 20., None, None),
                VectorPoint::new(10., 10., None, None),
            ],
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_polygon(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![
                            vec![
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.7786828928924201,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7356879031193608,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.6583568237637192,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.7231719544476622,
                                    y: 0.9590168832161913,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.9377231592442196,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.7231719544476624,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.6250859462252395,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.6345893512076446,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.7518028126416558,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.8660254037844386,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.8430910345588061,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.8304773451370653,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.7260776792851733,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                }
                            ]
                        ],
                        offset: Some(vec![3.241841444519629, 0.0]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.9377231592442196,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.15284249599867805,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ]],
                        offset: Some(vec![1.7505894300567113]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.19165525141383033,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_multipolygon() {
        let coords = vec![vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 0., None, None),
                VectorPoint::new(40., 0., None, None),
                VectorPoint::new(40., 20., None, None),
                VectorPoint::new(40., 40., None, None),
                VectorPoint::new(20., 40., None, None),
                VectorPoint::new(0., 40., None, None),
                VectorPoint::new(0., 20., None, None),
                VectorPoint::new(0., 0., None, None),
            ],
            vec![
                VectorPoint::new(10., 10., None, None),
                VectorPoint::new(20., 10., None, None),
                VectorPoint::new(30., 10., None, None),
                VectorPoint::new(30., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(20., 30., None, None),
                VectorPoint::new(10., 30., None, None),
                VectorPoint::new(10., 20., None, None),
                VectorPoint::new(10., 10., None, None),
            ],
        ]];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multipolygon(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![
                            vec![
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.7786828928924201,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7356879031193608,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.6583568237637192,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.7231719544476622,
                                    y: 0.9590168832161913,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.9377231592442196,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.7231719544476624,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.6250859462252395,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.6345893512076446,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.7518028126416558,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.8660254037844386,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.8430910345588061,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.8304773451370653,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.7260776792851733,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                }
                            ]
                        ],
                        offset: Some(vec![3.241841444519629, 0.0]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.9377231592442196,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.15284249599867805,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ]],
                        offset: Some(vec![1.7505894300567113]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.19165525141383033,
                            near: f64::MAX,
                            far: f64::MIN
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
