#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        geometry::{
            Area, Inside, InsideResult, Intersection, Segment, clean_polygon, dekink_polygon,
            dekink_polygons, polygons_intersections, polygons_intersections_ref,
        },
        proj::Coords,
    };
    use s2json::{
        BBox, Feature, FeatureType, Geometry, LineString3DGeometry, LineStringGeometry, MValue,
        MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint3DGeometry,
        MultiPointGeometry, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
        Point3DGeometry, PointGeometry, Polygon3DGeometry, PolygonGeometry, Properties,
        VectorFeature, VectorFeatureType, VectorGeometry, VectorLineStringGeometry,
        VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygonGeometry,
        VectorPoint, VectorPointGeometry, VectorPolygon, VectorPolygonGeometry,
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
        assert_eq!(polygons_intersections(&vec![a, b], false), vec![]);
    }

    #[test]
    fn polygons_intersections_ref_simple_no_overlap() {
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
        assert_eq!(polygons_intersections_ref(&vec![&a, &b], false), vec![]);
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
            polygons_intersections(&vec![a, b], false),
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
                    point: Point(-54.27247565285823, 37.293299349853186),
                    u: 0.8424254716370221,
                    t: 0.19573061281592416
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
                    point: Point(-54.37470749761522, 37.98610371249223),
                    u: 0.24332492060312594,
                    t: 0.8057743518198057
                }
            ]
        );
    }

    #[test]
    fn polygons_intersections_ref_simple_simple_overlap() {
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
            polygons_intersections_ref(&vec![&a, &b], false),
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
                    point: Point(-54.27247565285823, 37.293299349853186),
                    u: 0.8424254716370221,
                    t: 0.19573061281592416
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
                    point: Point(-54.37470749761522, 37.98610371249223),
                    u: 0.24332492060312594,
                    t: 0.8057743518198057
                }
            ]
        );
    }

    #[test]
    fn dekink_polygon_hourglass() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(2., 0.),
            VectorPoint::from_xy(0., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(0., 0.),
        ]];
        assert_eq!(
            dekink_polygon(&polygon),
            vec![
                vec![vec![
                    VectorPoint::from_xy(0., 0.),
                    VectorPoint::from_xy(2., 0.),
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 0.),
                ]],
                vec![vec![
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 2.),
                    VectorPoint::from_xy(2., 2.),
                    VectorPoint::from_xy(1., 1.),
                ]]
            ]
        );
    }

    #[test]
    fn dekink_polygons_hourglass() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(2., 0.),
            VectorPoint::from_xy(0., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(0., 0.),
        ]];
        assert_eq!(
            dekink_polygons(&vec![polygon]),
            vec![
                vec![vec![
                    VectorPoint::from_xy(0., 0.),
                    VectorPoint::from_xy(2., 0.),
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 0.),
                ]],
                vec![vec![
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 2.),
                    VectorPoint::from_xy(2., 2.),
                    VectorPoint::from_xy(1., 1.),
                ]]
            ]
        );
    }

    #[test]
    fn dekink_polygon_turfjs_issue_1094() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
            VectorPoint::from_xy(-91.9139393415105, 42.75096509455043),
            VectorPoint::from_xy(-91.91403053661699, 42.74800278177934),
            VectorPoint::from_xy(-91.91407345196123, 42.74679733206786),
            VectorPoint::from_xy(-91.91410563846941, 42.74537912592471),
            VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
            VectorPoint::from_xy(-91.9165088977471, 42.74624187186039),
            VectorPoint::from_xy(-91.91667519470603, 42.746249750763376),
            VectorPoint::from_xy(-91.91669128796013, 42.74536730740387),
            VectorPoint::from_xy(-91.91752277275486, 42.745359428388724),
            VectorPoint::from_xy(-91.91750131508273, 42.74636399474419),
            VectorPoint::from_xy(-91.91750667950076, 42.74737248420069),
            VectorPoint::from_xy(-91.91857047641793, 42.74735355619808),
            VectorPoint::from_xy(-91.91856615206176, 42.74748869579865),
            VectorPoint::from_xy(-91.91859029194288, 42.746734301373),
            VectorPoint::from_xy(-91.918995305504, 42.7463029333329),
            VectorPoint::from_xy(-91.91905967852034, 42.7461453552181),
            VectorPoint::from_xy(-91.9191481914178, 42.74588535045296),
            VectorPoint::from_xy(-91.91914282699976, 42.74558594967556),
            VectorPoint::from_xy(-91.91900871654907, 42.74558004043513),
            VectorPoint::from_xy(-91.91898457666794, 42.745357458635716),
            VectorPoint::from_xy(-91.92070119043696, 42.74535154937352),
            VectorPoint::from_xy(-91.92096672912932, 42.745483522761845),
            VectorPoint::from_xy(-91.92110352178906, 42.74550912950613),
            VectorPoint::from_xy(-91.92127786537496, 42.745485492511776),
            VectorPoint::from_xy(-91.92160241266565, 42.74565292102763),
            VectorPoint::from_xy(-91.9217257942803, 42.745786863514724),
            VectorPoint::from_xy(-91.92172311207129, 42.74599959510529),
            VectorPoint::from_xy(-91.92183040043186, 42.746527481454386),
            VectorPoint::from_xy(-91.92210935016931, 42.74716369671481),
            VectorPoint::from_xy(-91.92166678568198, 42.747210969414716),
            VectorPoint::from_xy(-91.92159704824762, 42.7472739996252),
            VectorPoint::from_xy(-91.92226760050114, 42.74730157532212),
            VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
        ]];

        assert_eq!(
            dekink_polygon(&polygon),
            vec![
                vec![vec![
                    VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
                    VectorPoint::from_xy(-91.9139393415105, 42.75096509455043),
                    VectorPoint::from_xy(-91.91403053661699, 42.74800278177934),
                    VectorPoint::from_xy(-91.91407345196123, 42.74679733206786),
                    VectorPoint::from_xy(-91.91410563846941, 42.74537912592471),
                    VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
                    VectorPoint::from_xy(-91.9165088977471, 42.74624187186039),
                    VectorPoint::from_xy(-91.91667519470603, 42.746249750763376),
                    VectorPoint::from_xy(-91.91669128796013, 42.74536730740387),
                    VectorPoint::from_xy(-91.91752277275486, 42.745359428388724),
                    VectorPoint::from_xy(-91.91750131508273, 42.74636399474419),
                    VectorPoint::from_xy(-91.91750667950076, 42.74737248420069),
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                    VectorPoint::from_xy(-91.91859029194288, 42.746734301373),
                    VectorPoint::from_xy(-91.918995305504, 42.7463029333329),
                    VectorPoint::from_xy(-91.91905967852034, 42.7461453552181),
                    VectorPoint::from_xy(-91.9191481914178, 42.74588535045296),
                    VectorPoint::from_xy(-91.91914282699976, 42.74558594967556),
                    VectorPoint::from_xy(-91.91900871654907, 42.74558004043513),
                    VectorPoint::from_xy(-91.91898457666794, 42.745357458635716),
                    VectorPoint::from_xy(-91.92070119043696, 42.74535154937352),
                    VectorPoint::from_xy(-91.92096672912932, 42.745483522761845),
                    VectorPoint::from_xy(-91.92110352178906, 42.74550912950613),
                    VectorPoint::from_xy(-91.92127786537496, 42.745485492511776),
                    VectorPoint::from_xy(-91.92160241266565, 42.74565292102763),
                    VectorPoint::from_xy(-91.9217257942803, 42.745786863514724),
                    VectorPoint::from_xy(-91.92172311207129, 42.74599959510529),
                    VectorPoint::from_xy(-91.92183040043186, 42.746527481454386),
                    VectorPoint::from_xy(-91.92210935016931, 42.74716369671481),
                    VectorPoint::from_xy(-91.92166678568198, 42.747210969414716),
                    VectorPoint::from_xy(-91.92159704824762, 42.7472739996252),
                    VectorPoint::from_xy(-91.92226760050114, 42.74730157532212),
                    VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
                ],],
                vec![vec![
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                    VectorPoint::from_xy(-91.91857047641793, 42.74735355619808),
                    VectorPoint::from_xy(-91.91856615206176, 42.74748869579865),
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                ],],
            ]
        );
    }

    #[test]
    fn dekink_polygon_multiple_kinks() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(8.094854051549703, 44.067038922182604),
            VectorPoint::from_xy(27.45169791493106, 34.31013538862004),
            VectorPoint::from_xy(31.238906496896703, 25.572928139998595),
            VectorPoint::from_xy(26.610096007827508, 22.88716015007573),
            VectorPoint::from_xy(25.978894577499233, 18.957601207155236),
            VectorPoint::from_xy(32.08050840400031, 17.157354229920827),
            VectorPoint::from_xy(38.8133236608289, 20.541732106259843),
            VectorPoint::from_xy(40.496527475035236, 28.199781765371043),
            VectorPoint::from_xy(7.463652621221485, 25.00221485407819),
            VectorPoint::from_xy(25.347693147171753, 4.999693002409302),
            VectorPoint::from_xy(-7.4747812298659255, -36.777396059815665),
            VectorPoint::from_xy(27.662098391706394, -40.233822107102995),
            VectorPoint::from_xy(28.92450125236215, -14.406933337995738),
            VectorPoint::from_xy(4.097244992807987, -34.38206769619466),
            VectorPoint::from_xy(62.79897801327945, -31.19907851930298),
            VectorPoint::from_xy(86.57423188895399, 16.55327251195662),
            VectorPoint::from_xy(54.38295894224376, 12.685928855764459),
            VectorPoint::from_xy(73.73980280562509, -3.197906810124664),
            VectorPoint::from_xy(81.52462044633336, 36.369487623534425),
            VectorPoint::from_xy(54.80375989579596, 56.70904723358515),
            VectorPoint::from_xy(8.094854051549703, 44.067038922182604),
        ]];

        assert_eq!(
            dekink_polygon(&polygon),
            vec![
                vec![vec![
                    VectorPoint::from_xy(8.094854051549703, 44.067038922182604),
                    VectorPoint::from_xy(27.45169791493106, 34.31013538862004),
                    VectorPoint::from_xy(30.51892217779216, 27.233954216494492),
                    VectorPoint::from_xy(7.463652621221485, 25.00221485407819),
                    VectorPoint::from_xy(25.347693147171753, 4.999693002409302),
                    VectorPoint::from_xy(-7.4747812298659255, -36.777396059815665),
                    VectorPoint::from_xy(27.662098391706394, -40.233822107102995),
                    VectorPoint::from_xy(28.011510823463176, -33.08536237940715),
                    VectorPoint::from_xy(62.79897801327945, -31.19907851930298),
                    VectorPoint::from_xy(86.57423188895399, 16.55327251195662),
                    VectorPoint::from_xy(77.40918288852106, 15.452216515532072),
                    VectorPoint::from_xy(81.52462044633336, 36.369487623534425),
                    VectorPoint::from_xy(54.80375989579596, 56.70904723358515),
                    VectorPoint::from_xy(8.094854051549703, 44.067038922182604),
                ]],
                vec![vec![
                    VectorPoint::from_xy(30.51892217779216, 27.233954216494492),
                    VectorPoint::from_xy(31.238906496896703, 25.572928139998595),
                    VectorPoint::from_xy(26.610096007827508, 22.88716015007573),
                    VectorPoint::from_xy(25.978894577499233, 18.957601207155236),
                    VectorPoint::from_xy(32.08050840400031, 17.157354229920827),
                    VectorPoint::from_xy(38.8133236608289, 20.541732106259843),
                    VectorPoint::from_xy(40.496527475035236, 28.199781765371043),
                    VectorPoint::from_xy(30.51892217779216, 27.233954216494492),
                ]],
                vec![vec![
                    VectorPoint::from_xy(28.011510823463176, -33.08536237940715),
                    VectorPoint::from_xy(28.92450125236215, -14.406933337995738),
                    VectorPoint::from_xy(4.097244992807987, -34.38206769619466),
                    VectorPoint::from_xy(28.011510823463176, -33.08536237940715),
                ]],
                vec![vec![
                    VectorPoint::from_xy(77.40918288852106, 15.452216515532072),
                    VectorPoint::from_xy(54.38295894224376, 12.685928855764459),
                    VectorPoint::from_xy(73.73980280562509, -3.197906810124664),
                    VectorPoint::from_xy(77.40918288852106, 15.452216515532072),
                ]],
            ]
        );
    }

    #[test]
    fn clean_polygon_all_problems() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
            VectorPoint::from_xy(-91.9139393415105, 42.75096509455043),
            VectorPoint::from_xy(-91.91403053661699, 42.74800278177934),
            VectorPoint::from_xy(-91.91407345196123, 42.74679733206786),
            VectorPoint::from_xy(-91.91410563846941, 42.74537912592471),
            VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
            VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
            VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
            VectorPoint::from_xy(-91.9165088977471, 42.74624187186039),
            VectorPoint::from_xy(-91.91667519470603, 42.746249750763376),
            VectorPoint::from_xy(-91.91669128796013, 42.74536730740387),
            VectorPoint::from_xy(-91.91752277275486, 42.745359428388724),
            VectorPoint::from_xy(-91.91750131508273, 42.74636399474419),
            VectorPoint::from_xy(-91.91750667950076, 42.74737248420069),
            VectorPoint::from_xy(-91.91857047641793, 42.74735355619808),
            VectorPoint::from_xy(-91.91856615206176, 42.74748869579865),
            VectorPoint::from_xy(-91.91859029194288, 42.746734301373),
            VectorPoint::from_xy(-91.918995305504, 42.7463029333329),
            VectorPoint::from_xy(-91.91905967852034, 42.7461453552181),
            VectorPoint::from_xy(-91.9191481914178, 42.74588535045296),
            VectorPoint::from_xy(-91.91914282699976, 42.74558594967556),
            VectorPoint::from_xy(-91.91900871654907, 42.74558004043513),
            VectorPoint::from_xy(-91.91898457666794, 42.745357458635716),
            VectorPoint::from_xy(-91.92070119043696, 42.74535154937352),
            VectorPoint::from_xy(-91.92096672912932, 42.745483522761845),
            VectorPoint::from_xy(-91.92110352178906, 42.74550912950613),
            VectorPoint::from_xy(-91.92127786537496, 42.745485492511776),
            VectorPoint::from_xy(-91.92160241266565, 42.74565292102763),
            VectorPoint::from_xy(-91.9217257942803, 42.745786863514724),
            VectorPoint::from_xy(-91.92172311207129, 42.74599959510529),
            VectorPoint::from_xy(-91.92183040043186, 42.746527481454386),
            VectorPoint::from_xy(-91.92210935016931, 42.74716369671481),
            VectorPoint::from_xy(-91.92166678568198, 42.747210969414716),
            VectorPoint::from_xy(-91.92159704824762, 42.7472739996252),
            VectorPoint::from_xy(-91.92226760050114, 42.74730157532212),
            VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
        ]];

        assert_eq!(
            clean_polygon(&polygon, false, false).unwrap(),
            vec![
                vec![vec![
                    VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
                    VectorPoint::from_xy(-91.92226760050114, 42.74730157532212),
                    VectorPoint::from_xy(-91.92159704824762, 42.7472739996252),
                    VectorPoint::from_xy(-91.92166678568198, 42.747210969414716),
                    VectorPoint::from_xy(-91.92210935016931, 42.74716369671481),
                    VectorPoint::from_xy(-91.92183040043186, 42.746527481454386),
                    VectorPoint::from_xy(-91.92172311207129, 42.74599959510529),
                    VectorPoint::from_xy(-91.9217257942803, 42.745786863514724),
                    VectorPoint::from_xy(-91.92160241266565, 42.74565292102763),
                    VectorPoint::from_xy(-91.92127786537496, 42.745485492511776),
                    VectorPoint::from_xy(-91.92110352178906, 42.74550912950613),
                    VectorPoint::from_xy(-91.92096672912932, 42.745483522761845),
                    VectorPoint::from_xy(-91.92070119043696, 42.74535154937352),
                    VectorPoint::from_xy(-91.91898457666794, 42.745357458635716),
                    VectorPoint::from_xy(-91.91900871654907, 42.74558004043513),
                    VectorPoint::from_xy(-91.91914282699976, 42.74558594967556),
                    VectorPoint::from_xy(-91.9191481914178, 42.74588535045296),
                    VectorPoint::from_xy(-91.91905967852034, 42.7461453552181),
                    VectorPoint::from_xy(-91.918995305504, 42.7463029333329),
                    VectorPoint::from_xy(-91.91859029194288, 42.746734301373),
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                    VectorPoint::from_xy(-91.91750667950076, 42.74737248420069),
                    VectorPoint::from_xy(-91.91750131508273, 42.74636399474419),
                    VectorPoint::from_xy(-91.91752277275486, 42.745359428388724),
                    VectorPoint::from_xy(-91.91669128796013, 42.74536730740387),
                    VectorPoint::from_xy(-91.91667519470603, 42.746249750763376),
                    VectorPoint::from_xy(-91.9165088977471, 42.74624187186039),
                    VectorPoint::from_xy(-91.91653571983723, 42.745359428388724),
                    VectorPoint::from_xy(-91.91410563846941, 42.74537912592471),
                    VectorPoint::from_xy(-91.91407345196123, 42.74679733206786),
                    VectorPoint::from_xy(-91.91403053661699, 42.74800278177934),
                    VectorPoint::from_xy(-91.9139393415105, 42.75096509455043),
                    VectorPoint::from_xy(-91.92218713423073, 42.750854798206724),
                ]],
                vec![vec![
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                    VectorPoint::from_xy(-91.91856615206176, 42.74748869579865),
                    VectorPoint::from_xy(-91.91857047641793, 42.74735355619808),
                    VectorPoint::from_xy(-91.91857047639631, 42.74735355619846),
                ]],
            ]
        );
    }

    #[test]
    fn clean_polygon_all_problems_and_collinearity() {
        let polygon: VectorPolygon = vec![vec![
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(2., 0.),
            VectorPoint::from_xy(0., 2.),
            VectorPoint::from_xy(1., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(2., 2.),
            VectorPoint::from_xy(0., 0.),
            VectorPoint::from_xy(0., 0.),
        ]];
        assert_eq!(
            clean_polygon(&polygon, true, false).unwrap(),
            vec![
                vec![vec![
                    VectorPoint::from_xy(0., 0.),
                    VectorPoint::from_xy(2., 0.),
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 0.),
                ]],
                vec![vec![
                    VectorPoint::from_xy(1., 1.),
                    VectorPoint::from_xy(0., 2.),
                    VectorPoint::from_xy(2., 2.),
                    VectorPoint::from_xy(1., 1.),
                ]],
            ]
        );
    }
}
