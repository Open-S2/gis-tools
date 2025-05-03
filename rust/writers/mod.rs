/// Write (S2|Geo)JSON to a file
pub mod json;
/// The (S2)PMTiles Writer
pub mod pmtiles;
/// Tile based writers
pub mod tile;
/// To Tiles Writer
pub mod to_tiles;

pub use json::*;
pub use pmtiles::*;
use s2json::VectorFeature;
pub use tile::*;
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
