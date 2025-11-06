#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{
            AverageOfPoints, CenterOfPoints, NearestPoint, ToPoints, bearing, clamp_wgs84_point,
            destination,
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
    fn test_points_bearing() {
        let start = Coords::new_xy(0.0, 0.0);
        let end = Coords::new_xy(1.0, 1.0);
        assert_eq!(bearing(&start, &end), 44.99563645534488);
    }

    #[test]
    fn test_points_destination() {
        let start = Coords::new_xy(0.0, 0.0);
        let bearing = 44.99563645534488;
        let distance = 1.0;
        assert_eq!(
            destination::<Coords, VectorPoint>(&start, bearing, distance, Some(1.)),
            VectorPoint::from_xy(47.756618381104495, 36.51656390940706)
        );

        let start = Coords::new_xy(0.0, 0.0);
        let bearing = 44.99563645534488;
        let distance = 1.0;
        assert_eq!(
            destination::<Coords, VectorPoint>(&start, bearing, distance, None),
            VectorPoint::from_xy(6.358670956091678e-6, 6.359639560000226e-6)
        );
    }

    #[test]
    fn test_average_of_points() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.0, 1.0));

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.0, 1.0, 1.0));

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.0, 1.0, 1.0));

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
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.average_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));
    }

    #[test]
    fn test_center_of_points() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.0, 1.0));

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![Point(1.0, 1.0), Point(2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xy(1.5, 1.5));

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.0, 1.0, 1.0));

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![Point3D(1.0, 1.0, 1.0), Point3D(2.0, 2.0, 2.0)]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.0, 1.0, 1.0));

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
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));

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
        assert_eq!(feature.center_of_points(), VectorPoint::from_xyz(1.5, 1.5, 1.5));
    }

    #[test]
    fn test_to_points() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.to_points(), vec![VectorPoint::from_xy(1.0, 1.0)]);

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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
        assert_eq!(feature.to_points(), vec![VectorPoint::from_xyz(1.0, 1.0, 1.0)]);

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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
        assert_eq!(feature.to_points(), vec![VectorPoint::from_xyz(1.0, 1.0, 1.0)]);

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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
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
            feature.to_points(),
            vec![VectorPoint::from_xyz(1.0, 1.0, 1.0), VectorPoint::from_xyz(2.0, 2.0, 2.0)]
        );
    }

    #[test]
    fn test_nearest_point() {
        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            *feature.nearest_point(&VectorPoint::from_xyz(1.0, 2.0, 3.0)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

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
            *feature.nearest_point(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
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
            *feature.nearest_point(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
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
            *feature.nearest_point(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
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
            *feature.nearest_point(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
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
            *feature.nearest_point(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn test_nearest_point_mut() {
        // VectorFeature -> VectorGeometry -> Point
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.0, 2.0, 3.0)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

        // VectorFeature -> VectorGeometry -> MultiPoint
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
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
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

        // VectorFeature -> VectorGeometry -> LineString
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
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
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

        // VectorFeature -> VectorGeometry -> MultiLineString
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
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
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

        // VectorFeature -> VectorGeometry -> Polygon
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
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
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );

        // VectorFeature -> VectorGeometry -> MultiPolygon
        let mut feature: VectorFeature<(), Properties, MValue> = VectorFeature {
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
            *feature.nearest_point_mut(&VectorPoint::from_xyz(1.1, 1.1, 1.1)).unwrap(),
            VectorPoint::from_xyz(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn test_clamp_wgs84_point() {
        let mut point = Point(0.0, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(0.0, 0.0));

        let mut point = Point(179.0, -90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(179.0, -90.0));

        let mut point = Point(179.999999, -90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(179.999999, -90.0));

        let mut point = Point(180.0, -90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-180.0, -90.0));

        let mut point = Point(-180.0, -90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-180.0, -90.0));

        let mut point = Point(-180.0, 90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-180.0, 90.0));

        let mut point = Point(180.0, 90.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-180.0, 90.0));

        // Clamp y's
        let mut point = Point(0.0, -91.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(0.0, -90.0));

        let mut point = Point(0.0, 91.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(0.0, 90.0));

        // wrap x's
        let mut point = Point(181.0, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-179.0, 0.0));

        let mut point = Point(-181.0, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(179.0, 0.0));

        let mut point = Point(520.0, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(160.0, 0.0));

        let mut point = Point(-420.0, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-60.0, 0.0));

        // 196.4
        let mut point = Point(196.4, 0.0);
        clamp_wgs84_point(&mut point);
        assert_eq!(point, Point(-163.60000000000002, 0.0));
    }
}

// test('clampWGS84Point', () => {
//   expect(clampWGS84Point({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
//   expect(clampWGS84Point({ x: 179, y: -90 })).toEqual({ x: 179, y: -90 });
//   expect(clampWGS84Point({ x: 179.999999, y: -90 })).toEqual({ x: 179.999999, y: -90 });
//   expect(clampWGS84Point({ x: -180, y: 90 })).toEqual({ x: -180, y: 90 });
//   // Clamp y's
//   expect(clampWGS84Point({ x: 0, y: -91 })).toEqual({ x: 0, y: -90 });
//   expect(clampWGS84Point({ x: 0, y: 91 })).toEqual({ x: 0, y: 90 });
//   // wrap x's
//   expect(clampWGS84Point({ x: 181, y: 0 })).toEqual({ x: -179, y: 0 });
//   expect(clampWGS84Point({ x: -181, y: 0 })).toEqual({ x: 179, y: 0 });
//   expect(clampWGS84Point({ x: 520, y: 0 })).toEqual({ x: 160, y: 0 });
//   expect(clampWGS84Point({ x: -420, y: 0 })).toEqual({ x: -60, y: 0 });

//   // 196.4
//   expect(clampWGS84Point({ x: 196.4, y: 0 })).toEqual({ x: -163.60000000000002, y: 0 });
// });
