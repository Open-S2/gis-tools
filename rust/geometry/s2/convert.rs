use crate::{Face, LonLat, S2CellId, VectorFeature, VectorGeometry, VectorPoint};

impl<M: Clone> VectorFeature<M> {
    /// Convert an S2 Feature to a GeoJSON Vector Feature
    pub fn to_wm(&self) -> Self {
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
    let LonLat { lon, lat } = (&S2CellId::from_face_st(face.into(), point.x, point.y)).into();
    point.x = lon;
    point.y = lat;
}
