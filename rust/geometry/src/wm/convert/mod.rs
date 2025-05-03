mod flat;
mod s2;
mod vector;

use crate::ConvertVectorFeatureS2;
use alloc::{vec, vec::Vec};
pub use flat::*;
pub use s2::*;
use s2json::{
    BBox3D, Feature, MValue, Properties, VectorFeature, VectorFeatureType, VectorGeometry,
};
pub use vector::*;

/// Underlying conversion mechanic to move GeoJSON Feature to GeoJSON Vector Feature
pub trait ConvertFeature<
    M: Clone = (),
    P: Clone + Default = Properties,
    D: Clone + Default = MValue,
>
{
    /// Convert a GeoJSON Feature to a GeoJSON Vector Feature
    fn to_vector(&self, build_bbox: Option<bool>) -> VectorFeature<M, P, D>;
}

impl<M: Clone, P: Clone + Default, D: Clone + Default> ConvertFeature<M, P, D>
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
    P: Clone + Default = Properties,
    D: Clone + Default = MValue,
>
{
    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    fn to_unit_scale(&mut self);
    /// Convert a 0->1 coordinate system to lon-lat
    fn to_ll(&mut self);
    /// Convert a GeoJSON Vector Feature to an S2 Feature
    fn to_s2(&self) -> Vec<VectorFeature<M, P, D>>;
    /// Convert a GeoJSON VectorFeature to a "flat" GeoJSON Feature
    fn to_feature(&self, build_bbox: bool) -> Feature<M, P, D>;
}

impl<M: Clone, P: Clone + Default, D: Clone + Default> ConvertVectorFeatureWM<M, P, D>
    for VectorFeature<M, P, D>
{
    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    fn to_unit_scale(&mut self) {
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
    fn to_s2(&self) -> Vec<VectorFeature<M, P, D>> {
        let VectorFeature { _type, id, properties, metadata, geometry, .. } = self;
        let mut res: Vec<VectorFeature<M, P, D>> = vec![];

        if *_type == VectorFeatureType::S2Feature {
            res.push(self.clone());
        } else {
            let vector_geo = convert_geometry_wm_to_s2(geometry);
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

    /// Convert a GeoJSON VectorFeature to a "flat" GeoJSON Feature
    fn to_feature(&self, build_bbox: bool) -> Feature<M, P, D> {
        if self._type == VectorFeatureType::S2Feature {
            let VectorFeature { id, properties, metadata, geometry, .. } = &self.to_wm();
            let geo = convert_vector_to_geometry(geometry, build_bbox);
            Feature::new(*id, properties.clone(), geo, metadata.clone())
        } else {
            let VectorFeature { id, properties, metadata, geometry, .. } = self;
            let geo = convert_vector_to_geometry(geometry, build_bbox);
            Feature::new(*id, properties.clone(), geo, metadata.clone())
        }
    }
}
