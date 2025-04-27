use s2json::{
    BBox3D, Geometry, GeometryType, LineString3DGeometry, LineStringGeometry,
    MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D, Point3DGeometry, PointGeometry,
    Polygon3DGeometry, PolygonGeometry, VectorGeometry, VectorLineStringGeometry,
    VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygonGeometry,
    VectorPointGeometry, VectorPolygonGeometry,
};

/// Convert a GeoJSON Geometry to an Vector Geometry
pub fn convert_vector_to_geometry<M: Clone + Default>(
    geometry: &VectorGeometry<M>,
    build_bbox: bool,
) -> Geometry<M> {
    let bbox = if build_bbox { Some(BBox3D::default()) } else { None };
    match geometry {
        VectorGeometry::Point(geo) => to_point(geo, bbox),
        VectorGeometry::MultiPoint(geo) => to_points(geo, bbox),
        VectorGeometry::LineString(geo) => to_linestring(geo, bbox),
        VectorGeometry::MultiLineString(geo) => to_linestrings(geo, bbox),
        VectorGeometry::Polygon(geo) => to_polygon(geo, bbox),
        VectorGeometry::MultiPolygon(geo) => to_polygons(geo, bbox),
    }
}

/// Mutate a GeoJSON Vector Point to a GeoJSON Point or Point3D
fn to_point<M: Clone + Default>(
    point: &VectorPointGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &point.coordinates;
    if let Some(b) = bbox.as_mut() {
        b.extend_from_point(coords);
    }
    if point.is_3d {
        Geometry::Point3D(Point3DGeometry::<M> {
            _type: GeometryType::Point3D,
            coordinates: Point3D(coords.x, coords.y, coords.z.unwrap_or_default()),
            m_values: coords.m.clone(),
            bbox,
        })
    } else {
        Geometry::Point(PointGeometry::<M> {
            _type: GeometryType::Point,
            coordinates: Point(coords.x, coords.y),
            m_values: coords.m.clone(),
            bbox: bbox.map(|b| b.into()),
        })
    }
}

/// Mutate a GeoJSON Vector MultiPoint to a GeoJSON MultiPoint/MultiPoint3D
fn to_points<M: Clone + Default>(
    points: &VectorMultiPointGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &points.coordinates;
    if let Some(b) = bbox.as_mut() {
        coords.iter().for_each(|p| b.extend_from_point(p));
    }
    let m_values = coords.iter().map(|p| p.m.clone()).collect();
    if points.is_3d {
        Geometry::MultiPoint3D(MultiPoint3DGeometry::<M> {
            _type: GeometryType::MultiPoint3D,
            coordinates: coords
                .iter()
                .map(|p| Point3D(p.x, p.y, p.z.unwrap_or_default()))
                .collect(),
            m_values,
            bbox,
        })
    } else {
        Geometry::MultiPoint(MultiPointGeometry::<M> {
            _type: GeometryType::MultiPoint,
            coordinates: coords.iter().map(|p| Point(p.x, p.y)).collect(),
            m_values,
            bbox: bbox.map(|b| b.into()),
        })
    }
}

/// Mutate a GeoJSON Vector LineString to a GeoJSON LineString/LineString3D
fn to_linestring<M: Clone + Default>(
    linestring: &VectorLineStringGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &linestring.coordinates;
    if let Some(b) = bbox.as_mut() {
        coords.iter().for_each(|p| b.extend_from_point(p));
    }
    let m_values = coords.iter().map(|p| p.m.clone()).collect();
    if linestring.is_3d {
        Geometry::LineString3D(LineString3DGeometry {
            _type: GeometryType::LineString3D,
            coordinates: coords
                .iter()
                .map(|p| Point3D(p.x, p.y, p.z.unwrap_or_default()))
                .collect(),
            m_values,
            bbox,
        })
    } else {
        Geometry::LineString(LineStringGeometry {
            _type: GeometryType::LineString,
            coordinates: coords.iter().map(|p| Point(p.x, p.y)).collect(),
            m_values,
            bbox: bbox.map(|b| b.into()),
        })
    }
}

/// Mutate a GeoJSON Vector MultiLineString to a GeoJSON MultiLineString/MultiLineString3D
fn to_linestrings<M: Clone + Default>(
    linestrings: &VectorMultiLineStringGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &linestrings.coordinates;
    if let Some(b) = bbox.as_mut() {
        coords.iter().for_each(|l| l.iter().for_each(|p| b.extend_from_point(p)));
    }
    let m_values = coords.iter().map(|l| l.iter().map(|p| p.m.clone()).collect()).collect();
    if linestrings.is_3d {
        Geometry::MultiLineString3D(MultiLineString3DGeometry {
            _type: GeometryType::MultiLineString3D,
            coordinates: coords
                .iter()
                .map(|l| l.iter().map(|p| Point3D(p.x, p.y, p.z.unwrap_or_default())).collect())
                .collect(),
            m_values,
            bbox,
        })
    } else {
        Geometry::MultiLineString(MultiLineStringGeometry {
            _type: GeometryType::MultiLineString,
            coordinates: coords
                .iter()
                .map(|l| l.iter().map(|p| Point(p.x, p.y)).collect())
                .collect(),
            m_values,
            bbox: bbox.map(|b| b.into()),
        })
    }
}

/// Mutate a GeoJSON Vector Polygon to a GeoJSON Polygon/Polygon3D
fn to_polygon<M: Clone + Default>(
    polygon: &VectorPolygonGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &polygon.coordinates;
    if let Some(b) = bbox.as_mut() {
        coords.iter().for_each(|r| r.iter().for_each(|p| b.extend_from_point(p)));
    }
    let m_values = coords.iter().map(|r| r.iter().map(|p| p.m.clone()).collect()).collect();
    if polygon.is_3d {
        Geometry::Polygon3D(Polygon3DGeometry {
            _type: GeometryType::Polygon3D,
            coordinates: coords
                .iter()
                .map(|r| r.iter().map(|p| Point3D(p.x, p.y, p.z.unwrap_or_default())).collect())
                .collect(),
            m_values,
            bbox,
        })
    } else {
        Geometry::Polygon(PolygonGeometry {
            _type: GeometryType::Polygon,
            coordinates: coords
                .iter()
                .map(|r| r.iter().map(|p| Point(p.x, p.y)).collect())
                .collect(),
            m_values,
            bbox: bbox.map(|b| b.into()),
        })
    }
}

/// Mutate a GeoJSON Vector MultiPolygon to a GeoJSON MultiPolygon/MultiPolygon3D
fn to_polygons<M: Clone + Default>(
    polygons: &VectorMultiPolygonGeometry<M>,
    mut bbox: Option<BBox3D>,
) -> Geometry<M> {
    let coords = &polygons.coordinates;
    if let Some(b) = bbox.as_mut() {
        coords
            .iter()
            .for_each(|p| p.iter().for_each(|r| r.iter().for_each(|p| b.extend_from_point(p))));
    }
    let m_values = coords
        .iter()
        .map(|p| p.iter().map(|r| r.iter().map(|p| p.m.clone()).collect()).collect())
        .collect();
    if polygons.is_3d {
        Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
            _type: GeometryType::MultiPolygon3D,
            coordinates: coords
                .iter()
                .map(|p| {
                    p.iter()
                        .map(|r| {
                            r.iter().map(|p| Point3D(p.x, p.y, p.z.unwrap_or_default())).collect()
                        })
                        .collect()
                })
                .collect(),
            m_values,
            bbox,
        })
    } else {
        Geometry::MultiPolygon(MultiPolygonGeometry {
            _type: GeometryType::MultiPolygon,
            coordinates: coords
                .iter()
                .map(|p| p.iter().map(|r| r.iter().map(|p| Point(p.x, p.y)).collect()).collect())
                .collect(),
            m_values,
            bbox: bbox.map(|b| b.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use s2json::{MValue, VectorGeometryType, VectorPoint};

    #[test]
    fn test_to_point() {
        let vector_point = VectorGeometry::Point(VectorPointGeometry {
            _type: VectorGeometryType::Point,
            coordinates: VectorPoint::new(
                1.0,
                2.0,
                None,
                Some(MValue::from([("a".into(), (1_u64).into())])),
            ),
            is_3d: false,
            ..Default::default()
        });
        let points = convert_vector_to_geometry(&vector_point, false);

        assert_eq!(
            points,
            Geometry::Point(PointGeometry {
                _type: GeometryType::Point,
                coordinates: Point(1.0, 2.0),
                m_values: Some(MValue::from([("a".into(), (1_u64).into())])),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_point_3d() {
        let vector_point = VectorGeometry::Point(VectorPointGeometry {
            _type: VectorGeometryType::Point,
            coordinates: VectorPoint::new(
                1.0,
                2.0,
                Some(3.0),
                Some(MValue::from([("a".into(), (1_u64).into())])),
            ),
            is_3d: true,
            ..Default::default()
        });
        let points = convert_vector_to_geometry(&vector_point, true);

        assert_eq!(
            points,
            Geometry::Point3D(Point3DGeometry {
                _type: GeometryType::Point3D,
                coordinates: Point3D(1.0, 2.0, 3.0),
                m_values: Some(MValue::from([("a".into(), (1_u64).into())])),
                bbox: Some(BBox3D::new(1.0, 2.0, 1.0, 2.0, 3.0, 3.0)),
            })
        )
    }

    #[test]
    fn test_to_points() {
        let vector_points = VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorGeometryType::MultiPoint,
            coordinates: vec![
                VectorPoint::new(
                    1.0,
                    2.0,
                    None,
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                VectorPoint::new(
                    -1.0,
                    -2.0,
                    None,
                    Some(MValue::from([("b".into(), (2_u64).into())])),
                ),
            ],
            is_3d: false,
            ..Default::default()
        });
        let points = convert_vector_to_geometry(&vector_points, false);

        assert_eq!(
            points,
            Geometry::MultiPoint(MultiPointGeometry {
                _type: GeometryType::MultiPoint,
                coordinates: vec![Point(1.0, 2.0), Point(-1.0, -2.0)],
                m_values: Some(vec![
                    MValue::from([("a".into(), (1_u64).into())]),
                    MValue::from([("b".into(), (2_u64).into())])
                ]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_points_3d() {
        let vector_points = VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorGeometryType::MultiPoint,
            coordinates: vec![
                VectorPoint::new(
                    1.0,
                    2.0,
                    Some(3.0),
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                VectorPoint::new(
                    -1.0,
                    -2.0,
                    Some(-3.0),
                    Some(MValue::from([("b".into(), (2_u64).into())])),
                ),
            ],
            is_3d: true,
            ..Default::default()
        });
        let points = convert_vector_to_geometry(&vector_points, true);

        assert_eq!(
            points,
            Geometry::MultiPoint3D(MultiPoint3DGeometry {
                _type: GeometryType::MultiPoint3D,
                coordinates: vec![Point3D(1.0, 2.0, 3.0), Point3D(-1.0, -2.0, -3.0)],
                m_values: Some(vec![
                    MValue::from([("a".into(), (1_u64).into())]),
                    MValue::from([("b".into(), (2_u64).into())])
                ]),
                bbox: Some(BBox3D::new(-1.0, -2.0, 1.0, 2.0, -3.0, 3.0)),
            })
        )
    }

    #[test]
    fn test_to_line() {
        let vector_line = VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
            coordinates: vec![
                VectorPoint::new(
                    1.0,
                    2.0,
                    None,
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                VectorPoint::new(
                    -1.0,
                    -2.0,
                    None,
                    Some(MValue::from([("b".into(), (2_u64).into())])),
                ),
            ],
            is_3d: false,
            ..Default::default()
        });
        let line = convert_vector_to_geometry(&vector_line, false);

        assert_eq!(
            line,
            Geometry::LineString(LineStringGeometry {
                _type: GeometryType::LineString,
                coordinates: vec![Point(1.0, 2.0), Point(-1.0, -2.0)],
                m_values: Some(vec![
                    MValue::from([("a".into(), (1_u64).into())]),
                    MValue::from([("b".into(), (2_u64).into())])
                ]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_line_3d() {
        let vector_line = VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
            coordinates: vec![
                VectorPoint::new(
                    1.0,
                    2.0,
                    Some(3.0),
                    Some(MValue::from([("a".into(), (1_u64).into())])),
                ),
                VectorPoint::new(
                    -1.0,
                    -2.0,
                    Some(-3.0),
                    Some(MValue::from([("b".into(), (2_u64).into())])),
                ),
            ],
            is_3d: true,
            ..Default::default()
        });
        let line = convert_vector_to_geometry(&vector_line, true);

        assert_eq!(
            line,
            Geometry::LineString3D(LineString3DGeometry {
                _type: GeometryType::LineString3D,
                coordinates: vec![Point3D(1.0, 2.0, 3.0), Point3D(-1.0, -2.0, -3.0)],
                m_values: Some(vec![
                    MValue::from([("a".into(), (1_u64).into())]),
                    MValue::from([("b".into(), (2_u64).into())])
                ]),
                bbox: Some(BBox3D::new(-1.0, -2.0, 1.0, 2.0, -3.0, 3.0)),
            })
        )
    }

    #[test]
    fn test_to_lines() {
        let vector_lines = VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: VectorGeometryType::MultiLineString,
            coordinates: vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        None,
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        None,
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        None,
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        None,
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ],
            is_3d: false,
            ..Default::default()
        });
        let lines = convert_vector_to_geometry(&vector_lines, false);

        assert_eq!(
            lines,
            Geometry::MultiLineString(MultiLineStringGeometry {
                _type: GeometryType::MultiLineString,
                coordinates: vec![
                    vec![Point(1.0, 2.0), Point(-1.0, -2.0)],
                    vec![Point(3.0, 4.0), Point(-3.0, -4.0)]
                ],
                m_values: Some(vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_lines_3d() {
        let vector_lines = VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: VectorGeometryType::MultiLineString,
            coordinates: vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        Some(3.0),
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        Some(-3.0),
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        Some(5.0),
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        Some(-5.0),
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ],
            is_3d: true,
            ..Default::default()
        });
        let lines = convert_vector_to_geometry(&vector_lines, true);

        assert_eq!(
            lines,
            Geometry::MultiLineString3D(MultiLineString3DGeometry {
                _type: GeometryType::MultiLineString3D,
                coordinates: vec![
                    vec![Point3D(1.0, 2.0, 3.0), Point3D(-1.0, -2.0, -3.0)],
                    vec![Point3D(3.0, 4.0, 5.0), Point3D(-3.0, -4.0, -5.0)]
                ],
                m_values: Some(vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]),
                bbox: Some(BBox3D::new(-3.0, -4.0, 3.0, 4.0, -5.0, 5.0)),
            })
        )
    }

    #[test]
    fn test_to_poly() {
        let vector_lines = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            coordinates: vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        None,
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        None,
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        None,
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        None,
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ],
            is_3d: false,
            ..Default::default()
        });
        let poly = convert_vector_to_geometry(&vector_lines, false);

        assert_eq!(
            poly,
            Geometry::Polygon(PolygonGeometry {
                _type: GeometryType::Polygon,
                coordinates: vec![
                    vec![Point(1.0, 2.0), Point(-1.0, -2.0)],
                    vec![Point(3.0, 4.0), Point(-3.0, -4.0)]
                ],
                m_values: Some(vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_poly_3d() {
        let vector_lines = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            coordinates: vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        Some(3.0),
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        Some(-3.0),
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        Some(5.0),
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        Some(-5.0),
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ],
            is_3d: true,
            ..Default::default()
        });
        let poly = convert_vector_to_geometry(&vector_lines, true);

        assert_eq!(
            poly,
            Geometry::Polygon3D(Polygon3DGeometry {
                _type: GeometryType::Polygon3D,
                coordinates: vec![
                    vec![Point3D(1.0, 2.0, 3.0), Point3D(-1.0, -2.0, -3.0)],
                    vec![Point3D(3.0, 4.0, 5.0), Point3D(-3.0, -4.0, -5.0)]
                ],
                m_values: Some(vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]),
                bbox: Some(BBox3D::new(-3.0, -4.0, 3.0, 4.0, -5.0, 5.0)),
            })
        )
    }

    #[test]
    fn test_to_polys() {
        let vector_polys = VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            coordinates: vec![vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        None,
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        None,
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        None,
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        None,
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ]],
            is_3d: false,
            ..Default::default()
        });
        let multi_poly = convert_vector_to_geometry(&vector_polys, false);

        assert_eq!(
            multi_poly,
            Geometry::MultiPolygon(MultiPolygonGeometry {
                _type: GeometryType::MultiPolygon,
                coordinates: vec![vec![
                    vec![Point(1.0, 2.0), Point(-1.0, -2.0)],
                    vec![Point(3.0, 4.0), Point(-3.0, -4.0)]
                ]],
                m_values: Some(vec![vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]]),
                ..Default::default()
            })
        )
    }

    #[test]
    fn test_to_multipoly_3d() {
        let vector_polys = VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            coordinates: vec![vec![
                vec![
                    VectorPoint::new(
                        1.0,
                        2.0,
                        Some(3.0),
                        Some(MValue::from([("a".into(), (1_u64).into())])),
                    ),
                    VectorPoint::new(
                        -1.0,
                        -2.0,
                        Some(-3.0),
                        Some(MValue::from([("b".into(), (2_u64).into())])),
                    ),
                ],
                vec![
                    VectorPoint::new(
                        3.0,
                        4.0,
                        Some(5.0),
                        Some(MValue::from([("c".into(), (3_u64).into())])),
                    ),
                    VectorPoint::new(
                        -3.0,
                        -4.0,
                        Some(-5.0),
                        Some(MValue::from([("d".into(), (4_u64).into())])),
                    ),
                ],
            ]],
            is_3d: true,
            ..Default::default()
        });
        let multi_poly = convert_vector_to_geometry(&vector_polys, true);

        assert_eq!(
            multi_poly,
            Geometry::MultiPolygon3D(MultiPolygon3DGeometry {
                _type: GeometryType::MultiPolygon3D,
                coordinates: vec![vec![
                    vec![Point3D(1.0, 2.0, 3.0), Point3D(-1.0, -2.0, -3.0)],
                    vec![Point3D(3.0, 4.0, 5.0), Point3D(-3.0, -4.0, -5.0)]
                ]],
                m_values: Some(vec![vec![
                    vec![
                        MValue::from([("a".into(), (1_u64).into())]),
                        MValue::from([("b".into(), (2_u64).into())]),
                    ],
                    vec![
                        MValue::from([("c".into(), (3_u64).into())]),
                        MValue::from([("d".into(), (4_u64).into())]),
                    ]
                ]]),
                bbox: Some(BBox3D::new(-3.0, -4.0, 3.0, 4.0, -5.0, 5.0)),
            })
        )
    }
}
