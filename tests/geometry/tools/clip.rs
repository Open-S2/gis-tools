#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{
        clip_line_string, clip_multi_line_string, clip_multi_point, clip_multi_polygon, clip_point,
        clip_polygon,
    };
    use s2json::{
        Axis, BBox3D, MValue, VectorBaseGeometry, VectorGeometry, VectorLineStringGeometry,
        VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygonGeometry,
        VectorPoint, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_clip_point() {
        let geo = VectorPointGeometry {
            _type: "Point".into(),
            is_3d: false,
            coordinates: VectorPoint::from_xy(2.0, 2.0),
            ..Default::default()
        };
        let res = clip_point(&geo, Axis::X, 0., 1.);
        assert_eq!(res, None);

        let geo = VectorPointGeometry {
            _type: "Point".into(),
            is_3d: false,
            coordinates: VectorPoint::from_xy(0.5, 0.5),
            ..Default::default()
        };
        let res = clip_point(&geo, Axis::X, 0., 1.);
        assert_eq!(res, Some(VectorGeometry::new_point(VectorPoint::from_xy(0.5, 0.5), None)));
    }

    #[test]
    fn test_multi_point() {
        let geo = VectorMultiPointGeometry {
            _type: "MultiPoint".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(0.5, 0.5)],
            ..Default::default()
        };
        let res = clip_multi_point(&geo, Axis::X, 0., 1.);
        assert_eq!(
            res,
            Some(VectorGeometry::new_multipoint(vec![VectorPoint::from_xy(0.5, 0.5),], None))
        );
    }

    #[test]
    fn test_clip_line_string() {
        let geo = VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(0.5, 0.5)],
            ..Default::default()
        };
        let res = clip_line_string(&geo, Axis::X, 0., 1.);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                ]],
                offset: Some(vec![1.4142135623730951]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );

        let geo = VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(-2.0, -2.0), VectorPoint::from_xy(-0.5, -0.5)],
            ..Default::default()
        };
        let res = clip_line_string(&geo, Axis::X, 0., 1.);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_line_string_all_outside() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(3.0, 3.0)], /* Line 1 (outside > 1) */
                vec![VectorPoint::from_xy(-1.0, -1.0), VectorPoint::from_xy(-2.0, -2.0)], /* Line 2 (outside < 0) */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(res, None); // Everything clipped away
    }

    #[test]
    fn test_clip_multi_line_string_all_inside() {
        let coords = vec![
            vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], // Line 1
            vec![VectorPoint::from_xy(0.3, 0.7), VectorPoint::from_xy(0.7, 0.3)], // Line 2
        ];
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: coords.clone(), // Clone for comparison
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                        VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint { x: 0.3, y: 0.7, z: None, m: None, t: None },
                        VectorPoint { x: 0.7, y: 0.3, z: None, m: None, t: None }
                    ]
                ],
                offset: Some(vec![0.0, 0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_mixed_inside_outside() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(3.0, 3.0)], /* Line 1 (outside) */
                vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], /* Line 2 (inside) */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);

        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                    VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_crossing_boundaries() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(-1.0, -1.0), VectorPoint::from_xy(0.5, 0.5)], /* Line 1 crosses x=0 */
                vec![VectorPoint::from_xy(0.8, 0.8), VectorPoint::from_xy(2.0, 2.0)], /* Line 2 crosses x=1 */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);

        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.0, y: 0.0, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None },
                        VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: Some(1.0) }
                    ]
                ],
                offset: Some(vec![1.4142135623730951, 0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_empty_input() {
        let geo = VectorMultiLineStringGeometry::<MValue> {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![], // Empty coordinate array
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_line_string_with_empty_line() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![], // Empty line string inside
                vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], // Valid line
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                    VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_all_inside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(0.1, 0.9),
                VectorPoint::from_xy(0.1, 0.1), // Close the polygon
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_partially_outside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(-0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(-0.1, 0.9),
                VectorPoint::from_xy(-0.1, 0.1), // Close the polygon
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.0, y: 0.9, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]],
                offset: Some(vec![0.1]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_all_outside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(1.1, 0.1),
                VectorPoint::from_xy(1.9, 0.1),
                VectorPoint::from_xy(1.9, 0.9),
                VectorPoint::from_xy(1.1, 0.9),
                VectorPoint::from_xy(1.1, 0.1),
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_polygon_empty() {
        let geo = VectorPolygonGeometry::<MValue> {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_polygon_open_ring_gets_closed() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(0.1, 0.9),
            ]], // Not explicitly closed
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_resulting_in_degenerate_polygon() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.1, 0.1),
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.2, 0.4);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]],
                offset: Some(vec![0.1]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_all_inside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1
                    vec![
                        VectorPoint::from_xy(0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(0.1, 0.4),
                        VectorPoint::from_xy(0.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2
                    vec![
                        VectorPoint::from_xy(0.6, 0.6),
                        VectorPoint::from_xy(0.9, 0.6),
                        VectorPoint::from_xy(0.9, 0.9),
                        VectorPoint::from_xy(0.6, 0.9),
                        VectorPoint::from_xy(0.6, 0.6),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![
                    vec![vec![
                        VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.1, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None }
                    ]],
                    vec![vec![
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None }
                    ]]
                ],
                offset: Some(vec![vec![0.0], vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_mixed_inside_outside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1 (partially outside)
                    vec![
                        VectorPoint::from_xy(-0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(-0.1, 0.4),
                        VectorPoint::from_xy(-0.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2 (completely inside)
                    vec![
                        VectorPoint::from_xy(0.6, 0.6),
                        VectorPoint::from_xy(0.9, 0.6),
                        VectorPoint::from_xy(0.9, 0.9),
                        VectorPoint::from_xy(0.6, 0.9),
                        VectorPoint::from_xy(0.6, 0.6),
                    ],
                ],
                vec![
                    // Polygon 3 (completely outside)
                    vec![
                        VectorPoint::from_xy(1.1, 0.1),
                        VectorPoint::from_xy(1.4, 0.1),
                        VectorPoint::from_xy(1.4, 0.4),
                        VectorPoint::from_xy(1.1, 0.4),
                        VectorPoint::from_xy(1.1, 0.1),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![
                    vec![vec![
                        VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.0, y: 0.4, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) }
                    ]],
                    vec![vec![
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None }
                    ]]
                ],
                offset: Some(vec![vec![0.1], vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_all_outside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1 (outside)
                    vec![
                        VectorPoint::from_xy(1.1, 0.1),
                        VectorPoint::from_xy(1.4, 0.1),
                        VectorPoint::from_xy(1.4, 0.4),
                        VectorPoint::from_xy(1.1, 0.4),
                        VectorPoint::from_xy(1.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2 (also outside)
                    vec![
                        VectorPoint::from_xy(-0.9, -0.6),
                        VectorPoint::from_xy(-0.6, -0.6),
                        VectorPoint::from_xy(-0.6, -0.9),
                        VectorPoint::from_xy(-0.9, -0.9),
                        VectorPoint::from_xy(-0.9, -0.6),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_polygon_empty() {
        let geo = VectorMultiPolygonGeometry::<MValue> {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_polygon_with_empty_polygon() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![], // Empty polygon
                vec![
                    // Valid polygon
                    vec![
                        VectorPoint::from_xy(0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(0.1, 0.4),
                        VectorPoint::from_xy(0.1, 0.1),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.4, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]]],
                offset: Some(vec![vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_resulting_in_degenerate_polygon() {
        let geo = VectorMultiPolygonGeometry::<MValue> {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.1, 0.1),
            ]]],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.2, 0.4);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![vec![vec![
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]]],
                offset: Some(vec![vec![0.1]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        ); // The single polygon becomes degenerate and should be filtered out
    }
}
