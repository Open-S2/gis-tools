mod convert_to_vector;
mod convert_wm_to_s2;

use crate::geometry::build_sq_dists;

use alloc::{vec, vec::Vec};

use convert_to_vector::convert_geometry_to_vector;
use convert_wm_to_s2::{convert_geometry_wm_to_s2, ConvertedGeometry};

use s2json::{
    BBox3D, Feature, MValue, MValueCompatible, Properties, VectorFeature, VectorFeatureType,
    VectorGeometry,
};

/// Underlying conversion mechanic to move GeoJSON Feature to GeoJSON Vector Feature
pub trait ConvertFeature<
    M: Clone = (),
    P: MValueCompatible = Properties,
    D: MValueCompatible = MValue,
>
{
    /// Convert a GeoJSON Feature to a GeoJSON Vector Feature
    fn to_vector(&self, build_bbox: Option<bool>) -> VectorFeature<M, P, D>;
}

impl<M: Clone, P: MValueCompatible, D: MValueCompatible> ConvertFeature<M, P, D>
    for Feature<M, P, D>
{
    /// Convert a GeoJSON Feature to a GeoJSON Vector Feature
    fn to_vector(&self, build_bbox: Option<bool>) -> VectorFeature<M, P, D> {
        let build_bbox = build_bbox.unwrap_or(false);
        let Feature { id, properties, metadata, geometry, .. } = self;
        let vector_geo = convert_geometry_to_vector(geometry, build_bbox);
        VectorFeature::new_wm(*id, properties.clone(), vector_geo, metadata.clone())
    }
}

/// Underlying conversion mechanic to move GeoJSON Geometry to S2 Geometry
pub trait ConvertVectorFeatureWM<
    M: Clone = (),
    P: MValueCompatible = Properties,
    D: MValueCompatible = MValue,
>
{
    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    fn to_unit_scale(&mut self, tolerance: Option<f64>, maxzoom: Option<u8>);
    /// Convert a 0->1 coordinate system to lon-lat
    fn to_ll(&mut self);
    /// Convert a GeoJSON Vector Feature to an S2 Feature
    fn to_s2(&self, tolerance: Option<f64>, maxzoom: Option<u8>) -> Vec<VectorFeature<M, P, D>>;
}

impl<M: Clone, P: MValueCompatible, D: MValueCompatible> ConvertVectorFeatureWM<M, P, D>
    for VectorFeature<M, P, D>
{
    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    fn to_unit_scale(&mut self, tolerance: Option<f64>, maxzoom: Option<u8>) {
        let mut bbox = BBox3D::default();
        match &mut self.geometry {
            VectorGeometry::Point(geo) => {
                geo.coordinates.project(Some(&mut bbox));
                geo.vec_bbox = Some(bbox);
            }
            VectorGeometry::LineString(geo) | VectorGeometry::MultiPoint(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.project(Some(&mut bbox)));
                geo.vec_bbox = Some(bbox);
            }
            VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
                geo.coordinates
                    .iter_mut()
                    .for_each(|p| p.iter_mut().for_each(|p| p.project(Some(&mut bbox))));
                geo.vec_bbox = Some(bbox);
            }
            VectorGeometry::MultiPolygon(geo) => {
                geo.coordinates.iter_mut().for_each(|p| {
                    p.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.project(Some(&mut bbox))))
                });
                geo.vec_bbox = Some(bbox);
            }
        }

        if let Some(tolerance) = tolerance {
            build_sq_dists(&mut self.geometry, tolerance, maxzoom);
        }
    }

    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    fn to_ll(&mut self) {
        match &mut self.geometry {
            VectorGeometry::Point(geo) => {
                geo.coordinates.unproject();
            }
            VectorGeometry::LineString(geo) | VectorGeometry::MultiPoint(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.unproject());
            }
            VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.unproject()));
            }
            VectorGeometry::MultiPolygon(geo) => {
                geo.coordinates.iter_mut().for_each(|p| {
                    p.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.unproject()))
                });
            }
        }
    }

    /// Convet a GeoJSON Feature to an S2Feature
    fn to_s2(&self, tolerance: Option<f64>, maxzoom: Option<u8>) -> Vec<VectorFeature<M, P, D>> {
        let VectorFeature { _type, id, properties, metadata, geometry, .. } = self;
        let mut res: Vec<VectorFeature<M, P, D>> = vec![];

        if *_type == VectorFeatureType::S2Feature {
            res.push(self.clone());
        } else {
            let vector_geo = convert_geometry_wm_to_s2(geometry, tolerance, maxzoom);
            for ConvertedGeometry { geometry, face } in vector_geo {
                res.push(VectorFeature::<M, P, D>::new_s2(
                    *id,
                    face,
                    properties.clone(),
                    geometry,
                    metadata.clone(),
                ));
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use s2json::{
        VectorLineStringGeometry, VectorMultiLineStringGeometry, VectorMultiPointGeometry,
        VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn to_unit_scale_to_ll_point() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_point(VectorPoint::from_xyz(0., 0., 0.), None),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: true,
                coordinates: VectorPoint::from_xyz(0.5, 0.5, 0.),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: true,
                coordinates: VectorPoint::from_xyz(0., 0., 0.),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multipoint() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multipoint(
                vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -90., 0.),
                    VectorPoint::from_xyz(180., 90., 0.),
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0.5, 0.5, 0.),
                    VectorPoint::from_xyz(0., 1., 0.),
                    VectorPoint::from_xyz(1., 0., 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                    VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_linestring() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_linestring(
                vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -90., 0.),
                    VectorPoint::from_xyz(180., 90., 0.),
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0.5, 0.5, 0.),
                    VectorPoint::from_xyz(0., 1., 0.),
                    VectorPoint::from_xyz(1., 0., 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                is_3d: true,
                coordinates: vec![
                    VectorPoint::from_xyz(0., 0., 0.),
                    VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                    VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multilinestring() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multilinestring(
                vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_polygon() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_polygon(
                vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "Polygon".into(),
                is_3d: true,
                coordinates: vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_unit_scale_to_ll_multipolygon() {
        let mut s2_feature: VectorFeature = VectorFeature {
            geometry: VectorGeometry::new_multipolygon(
                vec![vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -90., 0.),
                        VectorPoint::from_xyz(180., 90., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ],
                ]],
                None,
            ),
            ..Default::default()
        };
        // TO UNIT SCALE 0->1
        s2_feature.to_unit_scale(None, None);
        // expect vbox and coords to update:
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                is_3d: true,
                coordinates: vec![vec![
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0., 1., 0.),
                        VectorPoint::from_xyz(1., 0., 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0.5, 0.5, 0.),
                        VectorPoint::from_xyz(0.25, 0.640274963084795, 0.),
                        VectorPoint::from_xyz(0.75, 0.35972503691520497, 0.),
                    ]
                ]],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
        // BACK TO LL WGS84
        s2_feature.to_ll();
        assert_eq!(
            s2_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                is_3d: true,
                coordinates: vec![vec![
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-180., -85.0511287798066, 0.),
                        VectorPoint::from_xyz(180., 85.0511287798066, 0.),
                    ],
                    vec![
                        VectorPoint::from_xyz(0., 0., 0.),
                        VectorPoint::from_xyz(-90., -45., 0.),
                        VectorPoint::from_xyz(90., 45., 0.),
                    ]
                ]],
                vec_bbox: Some(BBox3D::new(0., 0., 1., 1., 0., 0.)),
                ..Default::default()
            })
        );
    }
}
