#[cfg(test)]
// #[coverage(off)]
mod tests {
    use core::f64;
    use gistools::geometry::convert_geometry_to_vector;
    use s2json::{
        BBox3D, Geometry, LineString3DGeometry, LineStringGeometry, MValue,
        MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint3DGeometry,
        MultiPointGeometry, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
        Point3DGeometry, PointGeometry, Polygon3DGeometry, PolygonGeometry, VectorGeometry,
        VectorPoint,
    };

    #[test]
    fn to_vector_point() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let geo: Geometry = Geometry::Point(PointGeometry {
            _type: "Point".into(),
            coordinates: Point(1.0, 2.0),
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_point(
                VectorPoint::new(1., 2., None, m_value),
                Some(BBox3D::new(1., 2., 1., 2., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_point_no_m() {
        let geo: Geometry = Geometry::Point(PointGeometry {
            _type: "Point".into(),
            coordinates: Point(1.0, 2.0),
            m_values: None,
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, false);
        assert_eq!(vector, VectorGeometry::new_point(VectorPoint::new(1., 2., None, None), None));
    }

    #[test]
    fn to_vector_point_3d() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let geo: Geometry = Geometry::Point3D(Point3DGeometry {
            _type: "Point3D".into(),
            coordinates: Point3D(1.0, 2.0, 3.0),
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_point(
                VectorPoint::new(1., 2., Some(3.), m_value),
                Some(BBox3D::new(1., 2., 1., 2., 3., 3.))
            )
        );
    }

    #[test]
    fn to_vector_multipoint() {
        let m_value = Some(vec![
            MValue::from([("a".into(), (1_u64).into())]),
            MValue::from([("b".into(), (2_u64).into())]),
        ]);
        let geo: Geometry = Geometry::MultiPoint(MultiPointGeometry {
            _type: "MultiPoint".into(),
            coordinates: vec![Point(1.0, 2.0), Point(3.0, 4.0)],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multipoint(
                vec![
                    VectorPoint::new(
                        1.,
                        2.,
                        None,
                        Some(MValue::from([("a".into(), (1_u64).into())]))
                    ),
                    VectorPoint::new(
                        3.,
                        4.,
                        None,
                        Some(MValue::from([("b".into(), (2_u64).into())]))
                    ),
                ],
                Some(BBox3D::new(1., 2., 3., 4., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_multipoint_no_m() {
        let geo: Geometry = Geometry::MultiPoint(MultiPointGeometry {
            _type: "MultiPoint".into(),
            coordinates: vec![Point(1.0, 2.0), Point(3.0, 4.0)],
            m_values: None,
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, false);
        assert_eq!(
            vector,
            VectorGeometry::new_multipoint(
                vec![VectorPoint::new(1., 2., None, None,), VectorPoint::new(3., 4., None, None),],
                None
            )
        );
    }

    #[test]
    fn to_vector_multipoint_3d() {
        let m_value = Some(vec![
            MValue::from([("a".into(), (1_u64).into())]),
            MValue::from([("b".into(), (2_u64).into())]),
        ]);
        let geo: Geometry = Geometry::MultiPoint3D(MultiPoint3DGeometry {
            _type: "MultiPoint3D".into(),
            coordinates: vec![Point3D(1.0, 2.0, -1.0), Point3D(3.0, 4.0, 1.0)],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multipoint(
                vec![
                    VectorPoint::new(
                        1.,
                        2.,
                        Some(-1.),
                        Some(MValue::from([("a".into(), (1_u64).into())]))
                    ),
                    VectorPoint::new(
                        3.,
                        4.,
                        Some(1.),
                        Some(MValue::from([("b".into(), (2_u64).into())]))
                    ),
                ],
                Some(BBox3D::new(1., 2., 3., 4., -1., 1.))
            )
        );
    }

    #[test]
    fn to_vector_linestring() {
        let m_value = Some(vec![
            MValue::from([("a".into(), (1_u64).into())]),
            MValue::from([("b".into(), (2_u64).into())]),
        ]);
        let geo: Geometry = Geometry::LineString(LineStringGeometry {
            _type: "LineString".into(),
            coordinates: vec![Point(1.0, 2.0), Point(3.0, 4.0)],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_linestring(
                vec![
                    VectorPoint::new(
                        1.,
                        2.,
                        None,
                        Some(MValue::from([("a".into(), (1_u64).into())]))
                    ),
                    VectorPoint::new(
                        3.,
                        4.,
                        None,
                        Some(MValue::from([("b".into(), (2_u64).into())]))
                    ),
                ],
                Some(BBox3D::new(1., 2., 3., 4., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_linestring_3d() {
        let m_value = Some(vec![
            MValue::from([("a".into(), (1_u64).into())]),
            MValue::from([("b".into(), (2_u64).into())]),
        ]);
        let geo: Geometry = Geometry::LineString3D(LineString3DGeometry {
            _type: "LineString3D".into(),
            coordinates: vec![Point3D(1.0, 2.0, -1.0), Point3D(3.0, 4.0, 1.0)],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_linestring(
                vec![
                    VectorPoint::new(
                        1.,
                        2.,
                        Some(-1.),
                        Some(MValue::from([("a".into(), (1_u64).into())]))
                    ),
                    VectorPoint::new(
                        3.,
                        4.,
                        Some(1.),
                        Some(MValue::from([("b".into(), (2_u64).into())]))
                    ),
                ],
                Some(BBox3D::new(1., 2., 3., 4., -1., 1.))
            )
        );
    }

    #[test]
    fn to_vector_multilinestring() {
        let m_value = Some(vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]);
        let geo: Geometry = Geometry::MultiLineString(MultiLineStringGeometry {
            _type: "MultiLineString".into(),
            coordinates: vec![
                vec![Point(1.0, 2.0), Point(3.0, 4.0)],
                vec![Point(5.0, 6.0), Point(7.0, 8.0)],
            ],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multilinestring(
                vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            None,
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            None,
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            None,
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            None,
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ],
                Some(BBox3D::new(1., 2., 7., 8., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_multilinestring_no_m() {
        let geo: Geometry = Geometry::MultiLineString(MultiLineStringGeometry {
            _type: "MultiLineString".into(),
            coordinates: vec![
                vec![Point(1.0, 2.0), Point(3.0, 4.0)],
                vec![Point(5.0, 6.0), Point(7.0, 8.0)],
            ],
            m_values: None,
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, false);
        assert_eq!(
            vector,
            VectorGeometry::new_multilinestring(
                vec![
                    vec![
                        VectorPoint::new(1., 2., None, None),
                        VectorPoint::new(3., 4., None, None),
                    ],
                    vec![
                        VectorPoint::new(5., 6., None, None),
                        VectorPoint::new(7., 8., None, None),
                    ]
                ],
                None
            )
        );
    }

    #[test]
    fn to_vector_multilinestring_3d() {
        let m_value = Some(vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]);
        let geo: Geometry = Geometry::MultiLineString3D(MultiLineString3DGeometry {
            _type: "MultiLineString3D".into(),
            coordinates: vec![
                vec![Point3D(1.0, 2.0, -1.0), Point3D(3.0, 4.0, 1.0)],
                vec![Point3D(5.0, 6.0, -2.), Point3D(7.0, 8.0, 2.)],
            ],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multilinestring(
                vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            Some(-1.),
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            Some(1.),
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            Some(-2.),
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            Some(2.),
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ],
                Some(BBox3D::new(1., 2., 7., 8., -2., 2.))
            )
        );
    }

    #[test]
    fn to_vector_polygon() {
        let m_value = Some(vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]);
        let geo: Geometry = Geometry::Polygon(PolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![
                vec![Point(1.0, 2.0), Point(3.0, 4.0)],
                vec![Point(5.0, 6.0), Point(7.0, 8.0)],
            ],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_polygon(
                vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            None,
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            None,
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            None,
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            None,
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ],
                Some(BBox3D::new(1., 2., 7., 8., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_polygon_3d() {
        let m_value = Some(vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]);
        let geo: Geometry = Geometry::Polygon3D(Polygon3DGeometry {
            _type: "Polygon3D".into(),
            coordinates: vec![
                vec![Point3D(1.0, 2.0, -1.0), Point3D(3.0, 4.0, 1.0)],
                vec![Point3D(5.0, 6.0, -2.), Point3D(7.0, 8.0, 2.)],
            ],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_polygon(
                vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            Some(-1.),
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            Some(1.),
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            Some(-2.),
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            Some(2.),
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ],
                Some(BBox3D::new(1., 2., 7., 8., -2., 2.))
            )
        );
    }

    #[test]
    fn to_vector_multipolygon() {
        let m_value = Some(vec![vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]]);
        let geo: Geometry = Geometry::MultiPolygon(MultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![
                vec![Point(1.0, 2.0), Point(3.0, 4.0)],
                vec![Point(5.0, 6.0), Point(7.0, 8.0)],
            ]],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multipolygon(
                vec![vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            None,
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            None,
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            None,
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            None,
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ]],
                Some(BBox3D::new(1., 2., 7., 8., f64::MAX, f64::MIN))
            )
        );
    }

    #[test]
    fn to_vector_multipolygon_no_m() {
        let geo: Geometry = Geometry::MultiPolygon(MultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![
                vec![Point(1.0, 2.0), Point(3.0, 4.0)],
                vec![Point(5.0, 6.0), Point(7.0, 8.0)],
            ]],
            m_values: None,
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, false);
        assert_eq!(
            vector,
            VectorGeometry::new_multipolygon(
                vec![vec![
                    vec![
                        VectorPoint::new(1., 2., None, None),
                        VectorPoint::new(3., 4., None, None),
                    ],
                    vec![
                        VectorPoint::new(5., 6., None, None),
                        VectorPoint::new(7., 8., None, None),
                    ]
                ]],
                None
            )
        );
    }

    #[test]
    fn to_vector_multipolygon_3d() {
        let m_value = Some(vec![vec![
            vec![
                MValue::from([("a".into(), (1_u64).into())]),
                MValue::from([("b".into(), (2_u64).into())]),
            ],
            vec![
                MValue::from([("c".into(), (3_u64).into())]),
                MValue::from([("d".into(), (4_u64).into())]),
            ],
        ]]);
        let geo: Geometry = Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
            _type: "MultiPolygon3D".into(),
            coordinates: vec![vec![
                vec![Point3D(1.0, 2.0, -1.0), Point3D(3.0, 4.0, 1.0)],
                vec![Point3D(5.0, 6.0, -2.), Point3D(7.0, 8.0, 2.)],
            ]],
            m_values: m_value.clone(),
            ..Default::default()
        });
        let vector = convert_geometry_to_vector(&geo, true);
        assert_eq!(
            vector,
            VectorGeometry::new_multipolygon(
                vec![vec![
                    vec![
                        VectorPoint::new(
                            1.,
                            2.,
                            Some(-1.),
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        VectorPoint::new(
                            3.,
                            4.,
                            Some(1.),
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                    ],
                    vec![
                        VectorPoint::new(
                            5.,
                            6.,
                            Some(-2.),
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        VectorPoint::new(
                            7.,
                            8.,
                            Some(2.),
                            Some(MValue::from([("d".into(), (4_u64).into())]))
                        ),
                    ]
                ]],
                Some(BBox3D::new(1., 2., 7., 8., -2., 2.))
            )
        );
    }
}
