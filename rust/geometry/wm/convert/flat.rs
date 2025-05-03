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
