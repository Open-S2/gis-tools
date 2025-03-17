/// Large JSON Parsing
pub mod large_json;
/// NewLine Delimited JSON Reader
pub mod line_delimited;

pub use large_json::*;
pub use line_delimited::*;

use super::FeatureIterator;
use crate::geometry::ConvertFeature;
use alloc::{vec, vec::Vec};
use s2json::{
    Feature, FeatureCollection, JSONCollection, MValueCompatible, S2FeatureCollection,
    VectorFeature, WMFeature,
};
use serde::{de::DeserializeOwned, Deserialize};

/// Error type for ToGisJSON
#[derive(Debug, Deserialize)]
pub enum ToGISJSONError {
    /// Invalid JSON
    InvalidJSON,
}

/// Converts a String to a JSONCollection. Supports S2 and WGS84 JSON
pub trait ToGisJSON {
    /// Converts a String to a JSONCollection. Supports S2 and WGS84 JSON
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a Feature
    fn to_feature<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a VectorFeature
    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a WMFeature
    fn to_features<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<WMFeature<M, P, D>, ToGISJSONError>;
}

impl ToGisJSON for &str {
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }

    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }

    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }

    fn to_feature<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }

    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }

    fn to_features<
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    >(
        &self,
    ) -> Result<WMFeature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).unwrap_or(Err(ToGISJSONError::InvalidJSON))
    }
}

/// JSON Collection Reader
#[derive(Debug)]
pub struct JSONCollectionReader<M: Clone, P: MValueCompatible, D: MValueCompatible> {
    features: Vec<VectorFeature<M, P, D>>,
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> From<&mut JSONCollection<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut JSONCollection<M, P, D>) -> Self {
        let mut features: Vec<VectorFeature<M, P, D>> = vec![];
        match collection {
            JSONCollection::FeatureCollection(collection) => {
                for feature in &mut collection.features {
                    match feature {
                        WMFeature::Feature(feature) => {
                            features.push(feature.to_vector(Some(true)));
                        }
                        WMFeature::VectorFeature(vf) => {
                            features.push(core::mem::take(vf));
                        }
                    }
                }
            }
            JSONCollection::S2FeatureCollection(collection) => {
                features.extend(core::mem::take(&mut collection.features));
            }
            JSONCollection::Feature(feature) => {
                features.push(feature.to_vector(Some(true)));
            }
            JSONCollection::VectorFeature(vf) => {
                features.push(core::mem::take(vf));
            }
        };

        JSONCollectionReader { features }
    }
}
impl<M: Clone + Default, P: MValueCompatible, D: MValueCompatible>
    From<&mut FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut FeatureCollection<M, P, D>) -> Self {
        JSONCollectionReader::from(&mut JSONCollection::FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone + Default, P: MValueCompatible, D: MValueCompatible>
    From<&mut S2FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut S2FeatureCollection<M, P, D>) -> Self {
        JSONCollectionReader::from(&mut JSONCollection::S2FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> From<&mut VectorFeature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut VectorFeature<M, P, D>) -> Self {
        JSONCollectionReader::from(&mut JSONCollection::VectorFeature(core::mem::take(collection)))
    }
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> From<&mut Feature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(feature: &mut Feature<M, P, D>) -> Self {
        JSONCollectionReader::from(&mut JSONCollection::Feature(core::mem::take(feature)))
    }
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> IntoIterator
    for JSONCollectionReader<M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}
impl<M: Clone, P: MValueCompatible, D: MValueCompatible> FeatureIterator<M, P, D>
    for JSONCollectionReader<M, P, D>
{
}
