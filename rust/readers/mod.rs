/// CSV Reader
pub mod csv;
/// GeoTIFF Reader
pub mod geotiff;
/// GPX Reader
pub mod gpx;
/// Grib2 Reader
pub mod grib2;
/// JSON Reader
pub mod json;
/// LAS/LAZ Reader
pub mod las;
/// NAD Grid Reader
pub mod nadgrid;
/// NetCDF Reader
pub mod netcdf;
/// OSM (Open Street Map) PBF Reader
pub mod osm;
/// (S2)PMTiles Reader
pub mod pmtiles;
/// Shapefile Reader
pub mod shapefile;
/// Tile-based Readers
pub mod tile;

pub use csv::*;
pub use geotiff::*;
pub use gpx::*;
pub use grib2::*;
pub use image::*;
pub use las::*;
pub use nadgrid::*;
pub use netcdf::*;
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
