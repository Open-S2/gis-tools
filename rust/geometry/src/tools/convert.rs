use crate::{ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM};
use alloc::{vec, vec::Vec};
use s2json::{Feature, Features, JSONCollection, Projection, VectorFeature};

/// Given an input data, convert it to a vector of VectorFeature
pub fn convert<M: Clone, P: Clone + Default, D: Clone + Default>(
    projection: Projection,
    data: &JSONCollection<M, P, D>,
    build_bbox: Option<bool>,
    to_unit_scale: Option<bool>,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    let to_unit_scale = to_unit_scale.unwrap_or(false);
    let mut res: Vec<VectorFeature<M, P, D>> = vec![];

    match data {
        JSONCollection::FeatureCollection(feature_collection) => {
            for feature in &feature_collection.features {
                match &feature {
                    Features::Feature(feature) => {
                        res.extend(convert_feature(projection, feature, to_unit_scale, build_bbox));
                    }
                    Features::VectorFeature(feature) => {
                        res.extend(convert_vector_feature(projection, feature, to_unit_scale))
                    }
                }
            }
        }
        JSONCollection::S2FeatureCollection(feature_collection) => {
            for feature in &feature_collection.features {
                res.extend(convert_vector_feature(projection, feature, to_unit_scale));
            }
        }
        JSONCollection::Feature(feature) => {
            res.extend(convert_feature(projection, feature, to_unit_scale, build_bbox));
        }
        JSONCollection::VectorFeature(feature) => {
            res.extend(convert_vector_feature(projection, feature, to_unit_scale));
        }
    }

    res
}

/// Convert a GeoJSON Feature to the appropriate VectorFeature
fn convert_feature<M: Clone, P: Clone + Default, D: Clone + Default>(
    projection: Projection,
    data: &Feature<M, P, D>,
    to_unit_scale: bool,
    build_bbox: Option<bool>,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    let mut vf: VectorFeature<M, P, D> = Feature::to_vector(data, build_bbox);
    match projection {
        Projection::S2 => vf.to_s2(),
        Projection::WG => {
            if to_unit_scale {
                vf.to_unit_scale();
            }
            vec![vf]
        }
    }
}

/// Convert a GeoJSON VectorFeature to the appropriate VectorFeature
fn convert_vector_feature<M: Clone, P: Clone + Default, D: Clone + Default>(
    projection: Projection,
    data: &VectorFeature<M, P, D>,
    to_unit_scale: bool,
) -> Vec<VectorFeature<M, P, D>>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D>,
{
    match projection {
        Projection::S2 => data.to_s2(),
        Projection::WG => {
            let mut vf = data.to_wm();
            if to_unit_scale {
                vf.to_unit_scale();
            }
            vec![vf]
        }
    }
}
