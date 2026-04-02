mod flat;
mod s2;
mod vector;

#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod test {
    use gistools::geometry::{ConvertVectorFeatureWM, convert};
    use s2json::{
        BBox, BBox3D, Face, Feature, Geometry, GeometryType, JSONCollection, MValue, Point,
        PointGeometry, Projection, VectorFeature, VectorFeatureType, VectorGeometry,
        VectorGeometryType, VectorLineStringGeometry, VectorMultiLineStringGeometry,
        VectorMultiPointGeometry, VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
        VectorPolygonGeometry,
    };

    #[test]
    fn to_unit_scale_to_ll_point() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_point(VectorPoint::from_xyz(0., 0., 0.), None),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: true,
                coordinates: VectorPoint::from_xyz(0.5, 0.5, 0.),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: true,
                coordinates: VectorPoint::from_xyz(0., 0., 0.),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multipoint() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multipoint(
                vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -90., 0.),
                    VectorPoint::from_xyz(180., 90., 0.),
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0.5, 0.5, 0.),
                    VectorPoint::from_xyz(0., 1., 0.),
                    VectorPoint::from_xyz(1., 0., 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                    VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_linestring() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_linestring(
                vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -90., 0.),
                    VectorPoint::from_xyz(180., 90., 0.),
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0.5, 0.5, 0.),
                    VectorPoint::from_xyz(0., 1., 0.),
                    VectorPoint::from_xyz(1., 0., 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                    VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multilinestring() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multilinestring(
                vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_polygon() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_polygon(
                vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multipolygon() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multipolygon(
                vec![vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ]],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale();
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                is_3d: true,
                coordinates: vec![vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ]],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                is_3d: true,
                coordinates: vec![vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ]],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_wm_point_convert() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = Point(1., 1.);
        let feature: Feature = Feature {
            _type: "Feature".into(),
            id: Some(1337),
            geometry: Geometry::Point(PointGeometry {
                _type: "Point".into(),
                coordinates: coords,
                m_values: m_value.clone(),
                bbox: Some(BBox::new(1., 2., 3., 4.)),
            }),
            ..Default::default()
        };
        let wm_feature = convert(
            Projection::WG,
            &JSONCollection::Feature(feature.clone()),
            // Some(3.),
            // Some(12),
            Some(true),
            Some(true),
        );

        assert_eq!(
            wm_feature,
            vec![VectorFeature {
                _type: "Feature".into(),
                id: Some(1337),
                face: Face::WM,
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(
                        0.5027777777777778,
                        0.49722208118489825,
                        None,
                        m_value
                    ),
                    bbox: Some(BBox3D::new(1., 1., 1., 1., f64::MAX, f64::MIN)),
                    vec_bbox: Some(BBox3D::new(
                        0.5027777777777778,
                        0.49722208118489825,
                        0.5027777777777778,
                        0.49722208118489825,
                        f64::MAX,
                        f64::MIN
                    )),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn vector_to_flat() {
        let vector_feature = VectorFeature::<MValue> {
            _type: VectorFeatureType::VectorFeature,
            id: Some(1_337),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                coordinates: VectorPoint::new(
                    1.0,
                    2.0,
                    None,
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                is_3d: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let points = vector_feature.to_feature(true);

        assert_eq!(
            points,
            Feature {
                _type: "Feature".into(),
                id: Some(1_337),
                geometry: Geometry::Point(PointGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(1.0, 2.0),
                    m_values: Some(MValue::from([("a".into(), (1_u64).into())])),
                    bbox: Some(BBox::new(1.0, 2.0, 1.0, 2.0)),
                }),
                ..Default::default()
            }
        );
    }

    #[test]
    fn s2_to_flat() {
        let vector_feature = VectorFeature::<MValue> {
            _type: VectorFeatureType::S2Feature,
            id: Some(1_337),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                coordinates: VectorPoint::new(
                    1.0,
                    2.0,
                    None,
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                is_3d: false,
                ..Default::default()
            }),
            ..Default::default()
        };
        let points = vector_feature.to_feature(true);

        assert_eq!(
            points,
            Feature {
                _type: "Feature".into(),
                id: Some(1_337),
                geometry: Geometry::Point(PointGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(45.0, 74.20683095173604),
                    m_values: Some(MValue::from([("a".into(), (1_u64).into())])),
                    bbox: Some(BBox::new(45.0, 74.20683095173604, 45.0, 74.20683095173604)),
                }),
                ..Default::default()
            }
        );
    }
}
