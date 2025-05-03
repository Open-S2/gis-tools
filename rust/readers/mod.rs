/// CSV Reader
pub mod csv;
/// GPX Reader
pub mod gpx;
/// JSON Reader
pub mod json;
/// OSM (Open Street Map) PBF Reader
pub mod osm;
/// (S2)PMTiles Reader
pub mod pmtiles;
/// Shapefile Reader
pub mod shapefile;
/// Tile based readers
pub mod tile;

pub use csv::*;
pub use gpx::*;
pub use image::*;
pub use osm::*;
pub use pmtiles::*;
use serde::{Deserialize, Serialize};
pub use shapefile::*;
pub use tile::*;

/// The type of readers to choose from
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReaderType {
    /// CSV data
    CSV,
    /// GPX data
    GPX,
    /// JSON data
    JSON,
    /// OSM data
    OSM,
    /// (S2)PMTiles data
    PMTiles,
    /// Shapefile
    Shapefile,
    /// Tile data
    Tile,
    /// WKT
    WKT,
    /// Protobuf
    Protobuf,
}
