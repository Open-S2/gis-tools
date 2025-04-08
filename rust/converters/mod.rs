/// To JSON Converter
pub mod to_json;
/// To Tiles Converter
pub mod to_tiles;

use s2json::VectorFeature;
pub use to_json::*;
pub use to_tiles::*;

/// Before using the data, you can mutate it here. It can also act as a filter if you return None
pub type OnFeature<M, P, D> = fn(feature: VectorFeature<M, P, D>) -> Option<VectorFeature<M, P, D>>;

/// Specify a trait that defines OnFeature but ensures its thread safe
/// A feature reader trait with a callback-based approach
pub trait OnFeatureMethod<M: Clone, P: Clone + Default, D: Clone + Default> {
    /// Reads features and applies the given callback function to each feature.
    fn on_feature<F>(&mut self, callback: F)
    where
        F: FnMut(VectorFeature<M, P, D>) -> Option<VectorFeature<M, P, D>> + Send + Sync;
}
