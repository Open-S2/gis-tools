#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{Area, Inside, InsideResult, Intersection, Segment, polygons_intersections},
        proj::Coords,
    };
    use s2json::{
        BBox, Feature, FeatureType, Geometry, LineString3DGeometry, LineStringGeometry, MValue,
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

    #[test]
    fn polygons_intersections_simple_no_overlap() {
        let a = vec![vec![
            VectorPoint::from_xy(-57.29250824839444, 39.309204530727754),
            VectorPoint::from_xy(-58.742162935523396, 35.86408152890863),
            VectorPoint::from_xy(-53.43642678063297, 37.560632581866784),
            VectorPoint::from_xy(-57.29250824839444, 39.309204530727754),
        ]];
        let b = vec![vec![
            VectorPoint::from_xy(-47.66680112586184, 39.08451444040281),
            VectorPoint::from_xy(-51.377917124910766, 37.76719296237772),
            VectorPoint::from_xy(-47.4058632821789, 35.274503072379716),
            VectorPoint::from_xy(-47.66680112586184, 39.08451444040281),
        ]];
        assert_eq!(polygons_intersections(&vec![a, b]), vec![]);
    }

    #[test]
    fn polygons_intersections_simple_simple_overlap() {
        let a = vec![vec![
            VectorPoint::from_xy(-57.29250824839444, 39.309204530727754),
            VectorPoint::from_xy(-58.742162935523396, 35.86408152890863),
            VectorPoint::from_xy(-53.43642678063297, 37.560632581866784),
            VectorPoint::from_xy(-57.29250824839444, 39.309204530727754),
        ]];
        let b = vec![vec![
            VectorPoint::from_xy(-51.29093784368342, 39.08451444040281),
            VectorPoint::from_xy(-55.118026217701825, 37.72134033908044),
            VectorPoint::from_xy(-50.79805525005969, 35.53445202830912),
            VectorPoint::from_xy(-51.29093784368342, 39.08451444040281),
        ]];
        assert_eq!(
            polygons_intersections(&vec![a, b]),
            vec![
                Intersection {
                    segment1: Segment {
                        id: 1,
                        poly_index: 0,
                        ring_index: 0,
                        from: 1,
                        to: 2,
                        bbox: BBox {
                            left: -58.742162935523396,
                            bottom: 35.86408152890863,
                            right: -53.43642678063297,
                            top: 37.560632581866784
                        }
                    },
                    segment2: Segment {
                        id: 4,
                        poly_index: 1,
                        ring_index: 0,
                        from: 1,
                        to: 2,
                        bbox: BBox {
                            left: -55.118026217701825,
                            bottom: 35.53445202830912,
                            right: -50.79805525005969,
                            top: 37.72134033908044
                        }
                    },
                    point: VectorPoint {
                        x: -54.27247565285823,
                        y: 37.293299349853186,
                        z: None,
                        m: None,
                        t: None
                    }
                },
                Intersection {
                    segment1: Segment {
                        id: 2,
                        poly_index: 0,
                        ring_index: 0,
                        from: 2,
                        to: 3,
                        bbox: BBox {
                            left: -57.29250824839444,
                            bottom: 37.560632581866784,
                            right: -53.43642678063297,
                            top: 39.309204530727754
                        }
                    },
                    segment2: Segment {
                        id: 3,
                        poly_index: 1,
                        ring_index: 0,
                        from: 0,
                        to: 1,
                        bbox: BBox {
                            left: -55.118026217701825,
                            bottom: 37.72134033908044,
                            right: -51.29093784368342,
                            top: 39.08451444040281
                        }
                    },
                    point: VectorPoint {
                        x: -54.37470749761522,
                        y: 37.98610371249223,
                        z: None,
                        m: None,
                        t: None
                    }
                }
            ]
        );
    }
}
