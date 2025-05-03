use alloc::{vec, vec::Vec};
use s2json::{
    BBox3D, BaseGeometry, Geometry, LineStringMValues, MultiLineStringMValues, MultiPolygonMValues,
    PointOrPoint3D, VectorGeometry, VectorPoint,
};

// TODO: Consider taking m-value data rather then clones

/// Convert a GeoJSON Geometry to an Vector Geometry
pub fn convert_geometry_to_vector<M: Clone + Default>(
    geometry: &Geometry<M>,
    build_bbox: bool,
) -> VectorGeometry<M> {
    let mut bbox = if build_bbox { Some(BBox3D::default()) } else { None };
    match geometry {
        Geometry::Point(geo) => {
            let coords = to_vector_point(geo, &mut bbox);
            VectorGeometry::new_point(coords, bbox)
        }
        Geometry::Point3D(geo) => {
            let coords = to_vector_point(geo, &mut bbox);
            VectorGeometry::new_point(coords, bbox)
        }
        Geometry::MultiPoint(geo) => {
            let coords = to_vector_multipoint(geo, &mut bbox);
            VectorGeometry::new_multipoint(coords, bbox)
        }
        Geometry::MultiPoint3D(geo) => {
            let coords = to_vector_multipoint(geo, &mut bbox);
            VectorGeometry::new_multipoint(coords, bbox)
        }
        Geometry::LineString(geo) => {
            let coords = to_vector_multipoint(geo, &mut bbox);
            VectorGeometry::new_linestring(coords, bbox)
        }
        Geometry::LineString3D(geo) => {
            let coords = to_vector_multipoint(geo, &mut bbox);
            VectorGeometry::new_linestring(coords, bbox)
        }
        Geometry::MultiLineString(geo) => {
            let coords = to_vector_multilinestring(geo, &mut bbox);
            VectorGeometry::new_multilinestring(coords, bbox)
        }
        Geometry::MultiLineString3D(geo) => {
            let coords = to_vector_multilinestring(geo, &mut bbox);
            VectorGeometry::new_multilinestring(coords, bbox)
        }
        Geometry::Polygon(geo) => {
            let coords = to_vector_multilinestring(geo, &mut bbox);
            VectorGeometry::new_polygon(coords, bbox)
        }
        Geometry::Polygon3D(geo) => {
            let coords = to_vector_multilinestring(geo, &mut bbox);
            VectorGeometry::new_polygon(coords, bbox)
        }
        Geometry::MultiPolygon(geo) => {
            let coords = to_vector_multipolygon(geo, &mut bbox);
            VectorGeometry::new_multipolygon(coords, bbox)
        }
        Geometry::MultiPolygon3D(geo) => {
            let coords = to_vector_multipolygon(geo, &mut bbox);
            VectorGeometry::new_multipolygon(coords, bbox)
        }
    }
}

fn convert_point<M: Clone + Default>(
    point: PointOrPoint3D,
    m: Option<M>,
    bbox: &mut Option<BBox3D>,
) -> VectorPoint<M> {
    let new_point = VectorPoint::new(point.0, point.1, point.2, m);
    if let Some(b) = bbox {
        b.extend_from_point(&new_point);
    }
    new_point
}

fn to_vector_point<M: Clone + Default, G: Copy, B>(
    geo: &BaseGeometry<M, G, B>,
    bbox: &mut Option<BBox3D>,
) -> VectorPoint<M>
where
    PointOrPoint3D: core::convert::From<G>,
{
    convert_point(geo.coordinates.into(), geo.m_values.clone(), bbox)
}

fn to_vector_multipoint<M: Clone + Default, G: Copy, B>(
    geo: &BaseGeometry<LineStringMValues<M>, Vec<G>, B>,
    bbox: &mut Option<BBox3D>,
) -> Vec<VectorPoint<M>>
where
    PointOrPoint3D: core::convert::From<G>,
{
    let binding = vec![];
    let m_values = geo.m_values.as_ref().unwrap_or(&binding);
    geo.coordinates
        .iter()
        .enumerate()
        .map(|(i, p)| convert_point((*p).into(), m_values.get(i).cloned(), bbox))
        .collect()
}

fn to_vector_multilinestring<M: Clone + Default, G: Copy, B>(
    geo: &BaseGeometry<MultiLineStringMValues<M>, Vec<Vec<G>>, B>,
    bbox: &mut Option<BBox3D>,
) -> Vec<Vec<VectorPoint<M>>>
where
    PointOrPoint3D: core::convert::From<G>,
{
    let binding = vec![];
    let m_values = geo.m_values.as_ref().unwrap_or(&binding);
    geo.coordinates
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, p)| {
                    let m_value = m_values.get(i).and_then(|m| m.get(j)).cloned();
                    convert_point((*p).into(), m_value, bbox)
                })
                .collect()
        })
        .collect()
}

fn to_vector_multipolygon<M: Clone + Default, G: Copy, B>(
    geo: &BaseGeometry<MultiPolygonMValues<M>, Vec<Vec<Vec<G>>>, B>,
    bbox: &mut Option<BBox3D>,
) -> Vec<Vec<Vec<VectorPoint<M>>>>
where
    PointOrPoint3D: core::convert::From<G>,
{
    let binding = vec![];
    let m_values = geo.m_values.as_ref().unwrap_or(&binding);
    geo.coordinates
        .iter()
        .enumerate()
        .map(|(i, polygon)| {
            polygon
                .iter()
                .enumerate()
                .map(|(j, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(k, p)| {
                            let m_value = m_values
                                .get(i)
                                .and_then(|m| m.get(j))
                                .and_then(|m| m.get(k))
                                .cloned();
                            convert_point((*p).into(), m_value, bbox)
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}
