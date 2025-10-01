#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{
            LengthOfLines, ToLines, along_line, intersection_of_segments,
            intersection_of_segments_robust,
        },
        proj::Coords,
    };
    use s2json::{
        Feature, FeatureType, Geometry, LineString3DGeometry, LineStringGeometry, MValue,
        MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint3DGeometry,
        MultiPointGeometry, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
        Point3DGeometry, PointGeometry, Polygon3DGeometry, PolygonGeometry, Properties,
        VectorFeature, VectorFeatureType, VectorGeometry, VectorLineStringGeometry,
        VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygonGeometry,
        VectorPoint, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_lines_along_line() {
        let line = vec![Coords::new_xy(0.0, 0.0), Coords::new_xy(1.0, 1.0)];
        assert_eq!(
            along_line(&line, 0.5, Some(1.)),
            VectorPoint::from_xy(-0.5391218665305646, 59.547812487066544)
        );

        let line = vec![Coords::new_xy(0.0, 0.0), Coords::new_xy(1.0, 1.0)];
        assert_eq!(
            along_line(&line, 0.5, None),
            VectorPoint::from_xy(0.9999935413484524, 1.0004046818854357)
        );
    }

    #[test]
    fn test_line_length() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 0.0);

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.4142135623730951);

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.4142135623730951);

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.4142135623730951);

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.4142135623730951);

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.4142135623730951);

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 0.0);

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 0.0);

        // VectorFeature -> VectorGeometry -> MultiPoint
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // VectorFeature -> VectorGeometry -> LineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // VectorFeature -> VectorGeometry -> MultiLineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // VectorFeature -> VectorGeometry -> Polygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);

        // VectorFeature -> VectorGeometry -> MultiPolygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.line_length(), 1.7320508075688772);
    }

    #[test]
    fn test_to_lines() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.to_lines(), vec![]);

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]
        );

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]
        );

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]
        );

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]
        );

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]
        );

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.to_lines(), vec![]);

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.to_lines(), vec![]);

        // VectorFeature -> VectorGeometry -> MultiPoint
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // VectorFeature -> VectorGeometry -> LineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // VectorFeature -> VectorGeometry -> MultiLineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // VectorFeature -> VectorGeometry -> Polygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );

        // VectorFeature -> VectorGeometry -> MultiPolygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            feature.to_lines(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );
    }

    #[test]
    fn intersection_of_segments_for_crossing_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 2.0));
        let b = (&VectorPoint::from_xy(0.0, 2.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(1.0, 1.0)));
    }

    #[test]
    fn intersection_of_segments_undefined_when_parallel() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(0.0, 1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments(a, b), None);
    }

    #[test]
    fn intersection_of_segments_undefined_when_lies_outside_bounds() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(2.0, 2.0), &VectorPoint::from_xy(3.0, 3.0));
        assert_eq!(intersection_of_segments(a, b), None);
    }

    #[test]
    fn interesciton_of_segments_endpoint_when_only_endpoints_touch() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(1.0, 1.0)));
    }

    #[test]
    fn interesction_of_segments_undefined_when_parallel_overlap() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments(a, b), None);
    }

    #[test]
    fn intersection_of_segments_correct_intersection_inside_segment_ranges() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(4.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(2.0, 0.0)));
    }

    #[test]
    fn intersection_of_segments_robust_for_crossing_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 2.0));
        let b = (&VectorPoint::from_xy(0.0, 2.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, None, None),
            Some(VectorPoint::from_xy(1.0, 1.0))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_for_parallel_non_intersecting_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(0.0, 1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments_robust(a, b, None, None), None);
    }

    #[test]
    fn intersection_of_segments_robust_undefined_for_collinear_overlapping_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments_robust(a, b, None, None), None);
    }

    #[test]
    fn intersection_of_segments_robust_endpoint_intersection_if_segments_touch_and_ring_ids_differ()
    {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, Some(1), Some(2)),
            Some(VectorPoint::from_xy(1.0, 1.0))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_if_segments_touch_at_endpoints_and_ring_ids_are_the_same()
     {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(intersection_of_segments_robust(a, b, Some(1), Some(1)), None);
    }

    #[test]
    fn intersection_of_segments_robust_returns_intersections_inside_segment_ranges() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(4.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, None, None),
            Some(VectorPoint::from_xy(2.0, 0.0))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_when_intersection_point_is_outside_of_segment_ranges()
     {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments_robust(a, b, None, None), None);
    }

    #[test]
    fn intersection_of_segments_robust_undefined_when_parallel_overlap() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments_robust(a, b, None, None), None);
    }
}
