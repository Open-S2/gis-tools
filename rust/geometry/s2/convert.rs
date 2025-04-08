use super::S2Point;
use crate::geometry::{Face, LonLat, VectorFeature, VectorGeometry, VectorPoint};
use s2json::{MValue, Properties, VectorFeatureType};

/// Underlying conversion mechanic to move S2 Geometry to GeoJSON Geometry
pub trait ConvertVectorFeatureS2<
    M: Clone = (),
    P: Clone + Default = Properties,
    D: Clone + Default = MValue,
>
{
    /// Convert an S2 Feature to a GeoJSON Vector Feature
    fn to_wm(&self) -> Self;
}

impl<M: Clone, P: Clone + Default, D: Clone + Default> ConvertVectorFeatureS2<M, P, D>
    for VectorFeature<M, P, D>
{
    /// Convert an S2 Feature to a GeoJSON Vector Feature
    fn to_wm(&self) -> Self {
        if self._type == VectorFeatureType::VectorFeature {
            return self.clone();
        }
        let mut geometry = self.geometry.clone();
        convert_geometry(self.face, &mut geometry);
        VectorFeature::<M, P, D>::new_wm(
            self.id,
            self.properties.clone(),
            geometry,
            self.metadata.clone(),
        )
    }
}

/// Underlying conversion mechanic to move S2Geometry to GeoJSON Geometry
fn convert_geometry<M: Clone + Default>(face: Face, geometry: &mut VectorGeometry<M>) {
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
fn convert_geometry_point<M: Clone + Default>(face: Face, point: &mut VectorPoint<M>) {
    let ll: LonLat = (&S2Point::from_face_st(face.into(), point.x, point.y)).into();
    point.x = ll.lon();
    point.y = ll.lat();
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use s2json::{
        BBox3D, VectorLineStringGeometry, VectorMultiLineStringGeometry, VectorMultiPointGeometry,
        VectorMultiPolygonGeometry, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_convert_geometry_point() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multipoint() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_linestring() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multi_linestring() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_polygon() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multi_polygon() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ]],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ]],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn convert_vector_feature() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                ..Default::default()
            }),
            metadata: None,
        };
        let converted = s2_feature.to_wm();
        assert_eq!(converted, s2_feature);
    }
}
