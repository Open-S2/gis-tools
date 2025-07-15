#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{SimplifyVectorGeometry, rewind};
    use s2json::{
        MValue, VectorGeometry, VectorLineStringGeometry, VectorMultiLineStringGeometry,
        VectorMultiPolygonGeometry, VectorPoint, VectorPolygonGeometry,
    };

    const SIMPLIFY_MAXZOOM: u8 = 16;

    #[test]
    fn test_rewind() {
        let mut ring = vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(0., 1., None, None),
            VectorPoint::new(1., 1., None, None),
            VectorPoint::new(1., 0., None, None),
        ];

        rewind(&mut ring, false);

        assert_eq!(
            ring,
            vec![
                VectorPoint::new(1., 0., None, None),
                VectorPoint::new(1., 1., None, None),
                VectorPoint::new(0., 1., None, None),
                VectorPoint::new(0., 0., None, None),
            ]
        );
    }

    #[test]
    fn test_line_string() {
        let mut line_string_geo = VectorGeometry::LineString(VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![
                VectorPoint::<MValue>::new(0.25, 0.25, None, None),
                VectorPoint::new(0.75, 0.25, None, None),
                VectorPoint::new(0.75, 0.75, None, None),
                VectorPoint::new(0.25, 0.75, None, None),
            ],
            offset: None,
            bbox: None,
            vec_bbox: None,
            indices: None,
            tessellation: None,
        });
        line_string_geo.build_sq_dists(3. / 4_096., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::LineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                    VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3. / 4_096., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::LineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                    VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_multi_line_string() {
        let mut line_string_geo = VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            coordinates: vec![
                vec![
                    VectorPoint::<MValue>::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3. / 4_096., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiLineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3. / 4_096., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiLineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_polygon() {
        let mut line_string_geo = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![
                vec![
                    VectorPoint::<MValue>::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3. / 4_096., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::Polygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3. / 4_096., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::Polygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_multi_polygon() {
        let mut line_string_geo = VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![
                vec![
                    VectorPoint::<MValue>::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ]],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3. / 4_096., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiPolygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3. / 4_096., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiPolygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }
}
