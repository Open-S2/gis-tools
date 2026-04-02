/// Tile grid based utilities
mod grid;
/// Raster based tools
pub mod raster;

use crate::{
    geometry::TileID,
    parsers::RGBA,
    tools::{convert_mapbox_elevation_data, convert_terrarium_elevation_data},
};
pub use grid::*;
pub use raster::*;
use s2_tilejson::{Metadata, Scheme};
use s2json::{Face, MValueCompatible, VectorFeature};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Tile Fetching Mechanism
pub trait TileFetcher<
    // Properties
    P: Clone + Default,
    // M-Value
    D: Clone + Default,
    // Tile Reader
    T: TileReader<P, D>,
>
{
    /// Creates a new file reader from a file path
    fn new<R: AsRef<Path>>(path: R, threshold: Option<u8>) -> Self;
    /// Get the Tile Store's Metadata
    fn get_metadata(&self) -> &Metadata;
    /// Check if a WebMercator tile exists
    fn has_tile_wm(&self, zoom: u8, x: u32, y: u32) -> bool;
    /// Check if an S2 Geometry tile exists
    fn has_tile_s2(&self, face: Face, zoom: u8, x: u32, y: u32) -> bool;
    /// Get a WebMercator tile
    fn get_tile_wm(&self, zoom: u8, x: u32, y: u32) -> T;
    /// Get an S2 Geometry tile
    fn get_tile_s2(&self, face: Face, zoom: u8, x: u32, y: u32) -> T;
    /// Get the Web Mercator value given a zoom, longitude, and latitude
    fn get_tile_value_wm(&self, zoom: u8, lon: f64, lat: f64, tile_size: Option<u64>) -> Option<D>;
    /// Get teh S2 value given a zoom, longitude, and latitude
    fn get_tile_value_s2(&self, zoom: u8, lon: f64, lat: f64, tile_size: Option<u64>) -> Option<D>;
    /// Check if it is S2 tile
    fn is_s2(&self) -> bool {
        let Metadata { scheme, .. } = self.get_metadata();
        *scheme == Scheme::Fzxy || *scheme == Scheme::Tfzxy
    }
}

/// Tile Reading Mechanism
pub trait TileReader<P: Clone + Default, D: Clone + Default> {
    /// Create a new Web Mercator Raster Tile Reader
    fn new(
        path: PathBuf,
        metadata: &Metadata,
        face: Face,
        zoom: u8,
        x: u32,
        y: u32,
        is_s2: bool,
    ) -> Self;
    /// Build a vector feature from the tile
    fn build_feature(&self) -> VectorFeature<TileID, P, D>;
}

/// Trait mechanic to parse a raster tile. Could be elevation or RGB(A)
pub trait GetRasterTileValue {
    /// Get the value of a raster tile pixel
    fn get_raster_tile_value(r: u8, g: u8, b: u8, a: Option<u8>) -> Self;
}

/// Elevation point used by terrarium readers
#[derive(Debug, Default, Clone, MValueCompatible, Serialize, Deserialize)]
pub struct TerrariumElevation {
    /// Elevation of a point
    pub elev: f64,
}
impl GetRasterTileValue for TerrariumElevation {
    fn get_raster_tile_value(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        TerrariumElevation { elev: convert_terrarium_elevation_data(r, g, b, a) }
    }
}

/// Elevation point used by terrarium readers
#[derive(Debug, Default, Clone, MValueCompatible, Serialize, Deserialize)]
pub struct MapboxElevation {
    /// Elevation of a point
    pub elev: f64,
}
impl GetRasterTileValue for MapboxElevation {
    fn get_raster_tile_value(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        MapboxElevation { elev: convert_mapbox_elevation_data(r, g, b, a) }
    }
}

impl GetRasterTileValue for RGBA {
    fn get_raster_tile_value(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        RGBA::from_u8s(r, g, b, a.unwrap_or(255))
    }
}
