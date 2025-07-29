#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{Area, Inside, InsideResult},
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
    fn test_inside() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Inside);
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![
                    Point(1.0, 1.0),
                    Point(2.0, 2.0),
                    Point(1.0, 2.0),
                    Point(1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Inside);

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> MultiPoint
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> LineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> MultiLineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> Polygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);

        // VectorFeature -> VectorGeometry -> MultiPolygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.inside(&Coords::new_xy(0.0, 0.0)), InsideResult::Outside);
        assert_eq!(feature.inside(&Coords::new_xy(1.0, 1.0)), InsideResult::Boundary);
        assert_eq!(feature.inside(&Coords::new_xy(1.5, 1.85)), InsideResult::Inside);
    }

    #[test]
    fn test_area() {
        // Feature -> Geometry -> Point
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point(PointGeometry {
                coordinates: Point(1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // Feature -> Geometry -> MultiPoint
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![
                    Point(1.0, 1.0),
                    Point(2.0, 2.0),
                    Point(1.0, 2.0),
                    Point(1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0001522545850103477);
        assert_eq!(feature.area(None), 6179976018.4314995);

        // Feature -> Geometry -> MultiPoint not closed
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint(MultiPointGeometry {
                coordinates: vec![Point(1.0, 1.0), Point(2.0, 2.0), Point(1.0, 2.0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // Feature -> Geometry -> LineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString(LineStringGeometry {
                coordinates: vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> MultiLineString
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString(MultiLineStringGeometry {
                coordinates: vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> Polygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon(PolygonGeometry {
                coordinates: vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> MultiPolygon
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon(MultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    Point(1.0, 1.0),
                    Point(1.0, 2.0),
                    Point(2.0, 2.0),
                    Point(1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> Point3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Point3D(Point3DGeometry {
                coordinates: Point3D(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // Feature -> Geometry -> MultiPoint3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> MultiPoint3D not closed
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPoint3D(MultiPoint3DGeometry {
                coordinates: vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // Feature -> Geometry -> LineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::LineString3D(LineString3DGeometry {
                coordinates: vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> MultiLineString3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiLineString3D(MultiLineString3DGeometry {
                coordinates: vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> Polygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::Polygon3D(Polygon3DGeometry {
                coordinates: vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // Feature -> Geometry -> MultiPolygon3D
        let feature: Feature<(), Properties, MValue> = Feature {
            _type: FeatureType::Feature,
            geometry: Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                coordinates: vec![vec![vec![
                    Point3D(1.0, 1.0, 1.0),
                    Point3D(1.0, 2.0, 1.0),
                    Point3D(2.0, 2.0, 2.0),
                    Point3D(1.0, 1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // VectorFeature -> VectorGeometry -> Point
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                coordinates: VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // VectorFeature -> VectorGeometry -> MultiPoint
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // VectorFeature -> VectorGeometry -> MultiPoint not closed
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), 0.0);
        assert_eq!(feature.area(None), 0.0);

        // VectorFeature -> VectorGeometry -> LineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                coordinates: vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // VectorFeature -> VectorGeometry -> MultiLineString
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // VectorFeature -> VectorGeometry -> Polygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                coordinates: vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);

        // VectorFeature -> VectorGeometry -> Polygon with hole
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(1.0, 1.0, 1.0),
                        VectorPoint::from_xyz(1.0, 2.0, 1.0),
                        VectorPoint::from_xyz(2.0, 2.0, 2.0),
                        VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    ],
                    vec![
                        VectorPoint::from_xyz(1.5, 1.88, 1.0),
                        VectorPoint::from_xyz(1.5, 1.89, 1.0),
                        VectorPoint::from_xyz(1.49, 1.89, 1.0),
                        VectorPoint::from_xyz(1.5, 1.88, 1.0),
                    ],
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.00015226980763931002);
        assert_eq!(feature.area(None), -6180593901.183065);

        // VectorFeature -> VectorGeometry -> MultiPolygon
        let feature: VectorFeature<(), Properties, MValue> = VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                coordinates: vec![vec![vec![
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                    VectorPoint::from_xyz(1.0, 2.0, 1.0),
                    VectorPoint::from_xyz(2.0, 2.0, 2.0),
                    VectorPoint::from_xyz(1.0, 1.0, 1.0),
                ]]],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(feature.area(Some(1.0)), -0.0001522545850103477);
        assert_eq!(feature.area(None), -6179976018.4314995);
    }
}
