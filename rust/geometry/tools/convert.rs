use alloc::vec;
use alloc::vec::Vec;
use s2json::MValueCompatible;

use crate::geometry::{
    ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM, Feature, JSONCollection,
    Projection, VectorFeature, WMFeature,
};

/// Given an input data, convert it to a vector of VectorFeature
pub fn convert<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    projection: Projection,
    data: &JSONCollection<M, P, D>,
    tolerance: Option<f64>,
    maxzoom: Option<u8>,
    build_bbox: Option<bool>,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    let mut res: Vec<VectorFeature<M, P, D>> = vec![];

    match data {
        JSONCollection::FeatureCollection(feature_collection) => {
            for feature in &feature_collection.features {
                match &feature {
                    WMFeature::Feature(feature) => {
                        res.extend(convert_feature(
                            projection, feature, tolerance, maxzoom, build_bbox,
                        ));
                    }
                    WMFeature::VectorFeature(feature) => {
                        res.extend(convert_vector_feature(projection, feature, tolerance, maxzoom))
                    }
                }
            }
        }
        JSONCollection::S2FeatureCollection(feature_collection) => {
            for feature in &feature_collection.features {
                res.extend(convert_vector_feature(projection, feature, tolerance, maxzoom));
            }
        }
        JSONCollection::Feature(feature) => {
            res.extend(convert_feature(projection, feature, tolerance, maxzoom, build_bbox));
        }
        JSONCollection::VectorFeature(feature) => {
            res.extend(convert_vector_feature(projection, feature, tolerance, maxzoom));
        }
    }

    res
}

/// Convert a GeoJSON Feature to the appropriate VectorFeature
fn convert_feature<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    projection: Projection,
    data: &Feature<M, P, D>,
    tolerance: Option<f64>,
    maxzoom: Option<u8>,
    build_bbox: Option<bool>,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    let mut vf: VectorFeature<M, P, D> = Feature::to_vector(data, build_bbox);
    match projection {
        Projection::S2 => vf.to_s2(tolerance, maxzoom),
        Projection::WG => {
            vf.to_unit_scale(tolerance, maxzoom);
            vec![vf]
        }
    }
}

/// Convert a GeoJSON VectorFeature to the appropriate VectorFeature
// fn convert_vector_feature<M: Clone + ConvertVectorFeatureWM<M>>(
fn convert_vector_feature<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    projection: Projection,
    data: &VectorFeature<M, P, D>,
    tolerance: Option<f64>,
    maxzoom: Option<u8>,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D>,
{
    match projection {
        Projection::S2 => data.to_s2(tolerance, maxzoom),
        Projection::WG => {
            let mut vf = data.to_wm();
            vf.to_unit_scale(tolerance, maxzoom);
            vec![vf]
        }
    }
}
