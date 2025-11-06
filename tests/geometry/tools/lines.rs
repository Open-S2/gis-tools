#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{
            DistanceMethod, IntersectionOfSegments, IntersectionOfSegmentsRobust, LengthOfLines,
            ToLines, along_line, clean_linestring, clean_linestrings, intersection_of_segments,
            intersection_of_segments_robust, point_on_line, point_to_line_distance,
        },
        proj::Coords,
    };
    use s2json::{
        Feature, FeatureType, Geometry, LineString3DGeometry, LineStringGeometry, MValue,
        MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint3DGeometry,
        MultiPointGeometry, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
        Point3DGeometry, PointGeometry, Polygon3DGeometry, PolygonGeometry, Properties,
        VectorFeature, VectorFeatureType, VectorGeometry, VectorLineStringGeometry,
        VectorMultiLineString, VectorMultiLineStringGeometry, VectorMultiPointGeometry,
        VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_lines_along_line() {
        let line = vec![Coords::new_xy(0.0, 0.0), Coords::new_xy(1.0, 1.0)];
        assert_eq!(
            along_line::<Coords, VectorPoint>(&line, 0.5, Some(1.)),
            VectorPoint::from_xy(-0.5391218665305646, 59.547812487066544)
        );

        let line = vec![Coords::new_xy(0.0, 0.0), Coords::new_xy(1.0, 1.0)];
        assert_eq!(
            along_line::<Coords, VectorPoint>(&line, 0.5, None),
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
        assert_eq!(feature.to_lines(), vec![] as VectorMultiLineString<MValue>);

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
            vec![vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xy(1.0, 1.0), VectorPoint::from_xy(2.0, 2.0)]]
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
        assert_eq!(feature.to_lines(), vec![] as VectorMultiLineString<MValue>);

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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
        assert_eq!(feature.to_lines(), vec![] as VectorMultiLineString<MValue>);

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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
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
            vec![vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]]
        );
    }

    #[test]
    fn intersection_of_segments_for_crossing_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 2.0));
        let b = (&VectorPoint::from_xy(0.0, 2.0), &VectorPoint::from_xy(2.0, 0.0));
        // assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(1.0, 1.0)));
        assert_eq!(
            intersection_of_segments(a, b),
            Some(IntersectionOfSegments { point: VectorPoint::from_xy(1.0, 1.0), u: 0.5, t: 0.5 })
        );
    }

    #[test]
    fn intersection_of_segments_undefined_when_parallel() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(0.0, 1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments::<VectorPoint, VectorPoint>(a, b), None);
    }

    #[test]
    fn intersection_of_segments_undefined_when_lies_outside_bounds() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(2.0, 2.0), &VectorPoint::from_xy(3.0, 3.0));
        assert_eq!(intersection_of_segments::<VectorPoint, VectorPoint>(a, b), None);
    }

    #[test]
    fn interesciton_of_segments_endpoint_when_only_endpoints_touch() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        // assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(1.0, 1.0)));
        assert_eq!(
            intersection_of_segments(a, b),
            Some(IntersectionOfSegments { point: VectorPoint::from_xy(1.0, 1.0), u: 1.0, t: 0.0 })
        );
    }

    #[test]
    fn interesction_of_segments_undefined_when_parallel_overlap() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments::<VectorPoint, VectorPoint>(a, b), None);
    }

    #[test]
    fn intersection_of_segments_correct_intersection_inside_segment_ranges() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(4.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        // assert_eq!(intersection_of_segments(a, b), Some(VectorPoint::from_xy(2.0, 0.0)));
        assert_eq!(
            intersection_of_segments(a, b),
            Some(IntersectionOfSegments { point: VectorPoint::from_xy(2.0, 0.0), u: 0.5, t: 0.5 })
        );
    }

    #[test]
    fn intersection_of_segments_robust_for_crossing_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 2.0));
        let b = (&VectorPoint::from_xy(0.0, 2.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, false),
            Some(IntersectionOfSegmentsRobust::new(
                1.,
                1.,
                0.5,
                0.5,
                VectorPoint::from_xy(1.0, 1.0),
                VectorPoint::from_xy(1.0, -1.0)
            ))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_for_parallel_non_intersecting_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(0.0, 1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments_robust::<VectorPoint, VectorPoint>(a, b, false), None);
    }

    #[test]
    fn intersection_of_segments_robust_undefined_for_collinear_overlapping_segments() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments_robust::<VectorPoint, VectorPoint>(a, b, false), None);
    }

    #[test]
    fn intersection_of_segments_robust_endpoint_intersection_if_segments_touch_and_ring_ids_differ()
    {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, false),
            Some(IntersectionOfSegmentsRobust::new(
                1.,
                1.,
                1.,
                0.,
                VectorPoint::from_xy(1.0, 1.0),
                VectorPoint::from_xy(-0.0, 0.0)
            ))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_if_segments_touch_at_endpoints_and_ring_ids_are_the_same()
     {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 1.0));
        let b = (&VectorPoint::from_xy(1.0, 1.0), &VectorPoint::from_xy(2.0, 0.0));
        assert_eq!(intersection_of_segments_robust::<VectorPoint, VectorPoint>(a, b, true), None);
    }

    #[test]
    fn intersection_of_segments_robust_returns_intersections_inside_segment_ranges() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(4.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(
            intersection_of_segments_robust(a, b, false),
            Some(IntersectionOfSegmentsRobust::new(
                2.,
                0.,
                0.5,
                0.5,
                VectorPoint::from_xy(2.0, 0.0),
                VectorPoint::from_xy(0.0, 1.0)
            ))
        );
    }

    #[test]
    fn intersection_of_segments_robust_undefined_when_intersection_point_is_outside_of_segment_ranges()
     {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(1.0, 0.0));
        let b = (&VectorPoint::from_xy(2.0, -1.0), &VectorPoint::from_xy(2.0, 1.0));
        assert_eq!(intersection_of_segments_robust::<VectorPoint, VectorPoint>(a, b, false), None);
    }

    #[test]
    fn intersection_of_segments_robust_undefined_when_parallel_overlap() {
        let a = (&VectorPoint::from_xy(0.0, 0.0), &VectorPoint::from_xy(2.0, 0.0));
        let b = (&VectorPoint::from_xy(1.0, 0.0), &VectorPoint::from_xy(3.0, 0.0));
        assert_eq!(intersection_of_segments_robust::<VectorPoint, VectorPoint>(a, b, false), None);
    }

    #[test]
    fn point_on_line_returns_true_for_point_exactly_on_the_segment() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(5.0, 5.0);
        assert!(point_on_line(&line, &point, None));
    }

    #[test]
    fn point_on_line_returns_true_for_point_exactly_on_a_line_point_of_the_line() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(10.0, 10.0);
        assert!(point_on_line(&line, &point, None));
    }

    #[test]
    fn point_on_line_returns_false_for_point_not_on_the_line() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(5.0, 6.0);
        assert!(!point_on_line(&line, &point, None));
    }

    #[test]
    fn point_on_line_returns_true_when_within_epsilon_tolerance() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(5.0, 5.00001);
        assert!(point_on_line(&line, &point, Some(0.001)));
    }

    #[test]
    fn point_on_line_returns_false_when_outside_bounding_box_even_if_collinear() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(15.0, 15.0);
        assert!(!point_on_line(&line, &point, None));
    }

    #[test]
    fn point_on_line_handles_degenerate_line_single_coordinate() {
        let line = vec![VectorPoint::from_xy(1.0, 1.0)];
        let point = VectorPoint::from_xy(1.0, 1.0);
        assert!(!point_on_line(&line, &point, None));
    }

    #[test]
    fn point_to_line_distance_returns_0_when_point_exactly_on_line_vertex() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 10.0)];
        let point = VectorPoint::from_xy(0.0, 0.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 0.0);
    }

    #[test]
    fn point_to_line_distance_returns_neg_1_for_empty_line() {
        let line: Vec<VectorPoint> = vec![];
        let point = VectorPoint::from_xy(0.0, 0.0);
        assert_eq!(point_to_line_distance(&line, &point, None), -1.0);
    }

    #[test]
    fn point_to_line_distance_returns_correct_euclidean_distance_for_midpoint_perpendicular() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 0.0)];
        let point = VectorPoint::from_xy(5.0, 5.0);
        assert_eq!(point_to_line_distance(&line, &point, Some(DistanceMethod::Euclidean)), 5.0);
    }

    #[test]
    fn point_to_line_distance_returns_correct_haversine_distance_for_midpoint_perpendicular() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 0.0)];
        let point = VectorPoint::from_xy(5.0, 5.0);
        assert_eq!(point_to_line_distance(&line, &point, Some(DistanceMethod::Haversine)), 5.);
    }

    #[test]
    fn point_to_line_distance_handles_degenerate_line_one_vertex_only() {
        let line = vec![VectorPoint::from_xy(2.0, 3.0)];
        let point = VectorPoint::from_xy(5.0, 3.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 3.);
    }

    #[test]
    fn point_to_line_distance_handles_line_with_three_vertices_point_closest_to_middle_segment() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(10.0, 0.0),
            VectorPoint::from_xy(20.0, 0.0),
        ];
        let point = VectorPoint::from_xy(9.0, 3.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 3.);
    }

    #[test]
    fn point_to_line_distance_returns_0_when_point_lies_exactly_on_segment() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 0.0)];
        let point = VectorPoint::from_xy(3.0, 0.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 0.0);
    }

    #[test]
    fn point_to_line_distance_uses_haversine_method_internally() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(20.0, 20.0)];
        let point = VectorPoint::from_xy(10.0, 10.0);
        assert_eq!(point_to_line_distance(&line, &point, Some(DistanceMethod::Haversine)), 0.0);
    }

    #[test]
    fn point_to_line_distance_handles_closest_vertex_at_start_of_line() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(10.0, 0.0),
            VectorPoint::from_xy(20.0, 0.0),
        ];
        let point = VectorPoint::from_xy(-5.0, 0.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 5.0);
    }

    #[test]
    fn point_to_line_distance_handles_closest_vertex_at_end_of_line() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(10.0, 0.0)];
        let point = VectorPoint::from_xy(15.0, 0.0);
        assert_eq!(point_to_line_distance(&line, &point, None), 5.0);
    }

    #[test]
    fn clean_linestring_remove_collinear_and_duplicate_points() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 2.0),
            VectorPoint::from_xy(3.0, 3.0),
            VectorPoint::from_xy(3.0, 3.0),
            VectorPoint::from_xy(3.0, 3.0),
            VectorPoint::from_xy(3.0, 3.0),
        ];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(result, vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(3.0, 3.0)]);
    }

    #[test]
    fn clean_linestring_remove_collinear_and_duplicate_points_2() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 2.0),
            VectorPoint::from_xy(3.0, 3.0),
        ];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(result, vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(3.0, 3.0)]);
    }

    #[test]
    fn clean_linestring_remove_collinear_and_duplicate_points_3() {
        let line = vec![
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(2., 0.),
            VectorPoint::from_xy(0., 2.),
            VectorPoint::from_xy(1., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(0., 0.),
        ];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(
            result,
            vec![
                VectorPoint::from_xy(0.0, 0.0),
                VectorPoint::from_xy(2.0, 0.0),
                VectorPoint::from_xy(0.0, 2.0),
                VectorPoint::from_xy(2.0, 2.0),
                VectorPoint::from_xy(0.0, 0.0),
            ]
        );
    }

    #[test]
    fn clean_linestring_remove_collinear_points_along_a_straight_line() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 2.0),
            VectorPoint::from_xy(3.0, 3.0),
        ];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(result, vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(3.0, 3.0)]);
    }

    #[test]
    fn clean_linestring_retains_non_collinear_points() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 0.0),
            VectorPoint::from_xy(3.0, 1.0),
        ];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(result, line);
    }

    #[test]
    fn clean_linestring_returns_original_when_too_few_points() {
        let line = vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(2.0, 2.0)];
        let result = clean_linestring(&line, false, None, false).unwrap();
        assert_eq!(result, line);
    }

    #[test]
    fn clean_linestring_returns_original_when_too_few_points_in_poly() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 2.0),
            VectorPoint::from_xy(0.0, 0.0),
        ];
        let result = clean_linestring(&line, true, None, false);
        assert_eq!(result, None);
    }

    #[test]
    fn clean_linestring_respects_tolerance_for_nearly_collinear_points() {
        let line = vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.000000000001), // tiny deviation
            VectorPoint::from_xy(2.0, 2.0),
        ];
        // high tolerance
        let result = clean_linestring(&line, false, Some(1e-15), false).unwrap();
        assert_eq!(result, line);
        // low tolerance
        let result = clean_linestring(&line, false, Some(1e-3), false).unwrap();
        assert_eq!(result, vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(2.0, 2.0)]);
    }

    #[test]
    fn clean_linestrings_basic() {
        let lines = vec![vec![
            VectorPoint::from_xy(0.0, 0.0),
            VectorPoint::from_xy(1.0, 1.0),
            VectorPoint::from_xy(2.0, 2.0),
            VectorPoint::from_xy(3.0, 3.0),
        ]];
        let result = clean_linestrings(&lines, false, None, false).unwrap();
        assert_eq!(
            result,
            vec![vec![VectorPoint::from_xy(0.0, 0.0), VectorPoint::from_xy(3.0, 3.0)]]
        );
    }
}
