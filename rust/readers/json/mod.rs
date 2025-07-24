/// Large JSON Parsing
pub mod large_json;
/// NewLine Delimited JSON Reader
pub mod line_delimited;

use crate::{geometry::ConvertFeature, parsers::FeatureReader};
use alloc::{vec, vec::Vec};
pub use large_json::*;
pub use line_delimited::*;
use s2json::{
    Feature, FeatureCollection, Features, JSONCollection, MValue, Properties, S2FeatureCollection,
    VectorFeature,
};
use serde::{Deserialize, de::DeserializeOwned};

/// Error type for ToGisJSON
#[derive(Debug, Deserialize)]
pub enum ToGISJSONError {
    /// Invalid JSON
    InvalidJSON,
}

/// # Convert strings to (Geo|S2)JSON
///
/// ## Description
/// Converts a String or &str to a JSONCollection, FeatureCollection or Feature. Supports S2 and WGS84 JSON
///
/// ## Usage
///
/// This trait allows for a lot of flexibility in how you can parse your data.
/// - [`ToGisJSON::to_gis_json`]: Converts an input into a [`JSONCollection`]
/// - [`ToGisJSON::to_feature_collection`]: Converts an input into a [`FeatureCollection`]
/// - [`ToGisJSON::to_s2_feature_collection`]: Converts an input into a [`S2FeatureCollection`]
/// - [`ToGisJSON::to_feature`]: Converts an input into a [`Feature`]
/// - [`ToGisJSON::to_vector_feature`]: Converts an input into a [`VectorFeature`]
/// - [`ToGisJSON::to_features`]: Converts an input into a [`Features`]
pub trait ToGisJSON {
    /// Converts a String to a JSONCollection. Supports S2 and WGS84 JSON
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a Feature
    fn to_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a VectorFeature
    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a WMFeature
    fn to_features<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Features<M, P, D>, ToGISJSONError>;
}

impl ToGisJSON for &str {
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_features<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Features<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }
}

/// # JSON Collection Reader
///
/// ## Description
/// Parse (Geo|S2)JSON.
///
/// Data parsed using the [`ToGisJSON`] trait can be coerced into this struct
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
/// ```rust
/// use gistools::{parsers::FeatureReader, readers::{ToGisJSON, JSONCollectionReader}};
/// use s2json::{MValue, MValueCompatible, Feature};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
/// struct Test {
///     name: String,
/// }
///
/// let json_str = r#"{
///     "type": "Feature",
///     "geometry": {
///         "type": "Point",
///         "coordinates": [100.0, 0.0]
///     },
///     "properties": {
///         "name": "Tokyo"
///     }
/// }"#;
/// let mut json: Feature<(), Test, MValue> = json_str.to_feature().unwrap();
///
/// let collection = JSONCollectionReader::from(&mut json);
/// let features: Vec<_> = collection.iter().collect();
/// assert_eq!(features.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct JSONCollectionReader<
    M: Clone = (),
    P: Clone + Default = Properties,
    D: Clone + Default = MValue,
> {
    /// Collection of features
    pub features: Vec<VectorFeature<M, P, D>>,
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> JSONCollectionReader<M, P, D> {
    /// Mutable iterator
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, VectorFeature<M, P, D>> {
        self.features.iter_mut()
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut JSONCollection<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut JSONCollection<M, P, D>) -> Self {
        let mut features: Vec<VectorFeature<M, P, D>> = vec![];
        match collection {
            JSONCollection::FeatureCollection(collection) => {
                for feature in &mut collection.features {
                    match feature {
                        Features::Feature(feature) => {
                            features.push(feature.to_vector(Some(true)));
                        }
                        Features::VectorFeature(vf) => {
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
impl<M: Clone + Default, P: Clone + Default, D: Clone + Default>
    From<&mut FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut FeatureCollection<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone + Default, P: Clone + Default, D: Clone + Default>
    From<&mut S2FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut S2FeatureCollection<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::S2FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut VectorFeature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut VectorFeature<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::VectorFeature(core::mem::take(collection)))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut Feature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(feature: &mut Feature<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::Feature(core::mem::take(feature)))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> IntoIterator
    for JSONCollectionReader<M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}
/// The JSON Iterator tool
#[derive(Debug)]
pub struct JSONCollectionIterator<
    'a,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> {
    reader: &'a JSONCollectionReader<M, P, D>,
    offset: usize,
    size: usize,
}
impl<
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> Iterator for JSONCollectionIterator<'_, M, P, D>
{
    type Item = VectorFeature<M, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.size {
            return None;
        }
        self.offset += 1;
        self.reader.features.get(self.offset - 1).cloned()
    }
}
/// A feature reader trait with a callback-based approach
impl<
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> FeatureReader<M, P, D> for JSONCollectionReader<M, P, D>
{
    type FeatureIterator<'a>
        = JSONCollectionIterator<'a, M, P, D>
    where
        M: 'a,
        P: 'a,
        D: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        JSONCollectionIterator { reader: self, offset: 0, size: self.features.len() }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
