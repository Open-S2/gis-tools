use crate::geometry::{Face, LonLat, S2CellId, VectorFeature, VectorGeometry, VectorPoint};

/// Underlying conversion mechanic to move S2 Geometry to GeoJSON Geometry
pub trait ConvertVectorFeatureS2<M: Clone> {
    /// Convert an S2 Feature to a GeoJSON Vector Feature
    fn to_wm(&self) -> Self;
}

impl<M: Clone> ConvertVectorFeatureS2<M> for VectorFeature<M> {
    /// Convert an S2 Feature to a GeoJSON Vector Feature
    fn to_wm(&self) -> Self {
        if self._type == "VectorFeature" {
            return self.clone();
        }
        let mut geometry = self.geometry.clone();
        convert_geometry(self.face, &mut geometry);
        VectorFeature::<M>::new_wm(
            self.id,
            self.properties.clone(),
            geometry,
            self.metadata.clone(),
        )
    }
}

/// Underlying conversion mechanic to move S2Geometry to GeoJSON Geometry
fn convert_geometry(face: Face, geometry: &mut VectorGeometry) {
    match geometry {
        VectorGeometry::Point(point) => convert_geometry_point(face, &mut point.coordinates),
        VectorGeometry::LineString(points) | VectorGeometry::MultiPoint(points) => {
            points.coordinates.iter_mut().for_each(|point| convert_geometry_point(face, point))
        }
        VectorGeometry::Polygon(lines) | VectorGeometry::MultiLineString(lines) => lines
            .coordinates
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|point| convert_geometry_point(face, point))),
        VectorGeometry::MultiPolygon(polygons) => {
            polygons.coordinates.iter_mut().for_each(|polygon| {
                polygon.iter_mut().for_each(|line| {
                    line.iter_mut().for_each(|point| convert_geometry_point(face, point))
                })
            })
        }
    }
}

/// Mutate an S2 Point to a GeoJSON Point
fn convert_geometry_point(face: Face, point: &mut VectorPoint) {
    let ll: LonLat = (&S2CellId::from_face_st(face.into(), point.x, point.y)).into();
    point.x = ll.lon();
    point.y = ll.lat();
}
