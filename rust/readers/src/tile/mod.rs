/// Raster based tools
pub mod raster;

use alloc::string::String;
use data_structures::HasLayer;
pub use raster::*;
use s2_tilejson::{Metadata, Scheme};
use s2json::{Face, MValueCompatible, VectorFeature};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use util::RGBA;

/// S2 Tile's metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct S2TileMetadata {
    /// S2 Face
    pub face: Face,
    /// S2 Zoom
    pub zoom: u8,
    /// S2 X Tile Coordinate
    pub x: u32,
    /// S2 Y Tile Coordinate
    pub y: u32,
}

/// Tile's metadata
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct WMTileMetadata {
    /// Zoom level
    pub zoom: u8,
    /// X tile coordinate
    pub x: u32,
    /// Y tile coordinate
    pub y: u32,
}

/// Tile's metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TileMetadata {
    /// Web Mercator metadata
    WM(WMTileMetadata),
    /// S2 Mercator metadata
    S2(S2TileMetadata),
}
impl Default for TileMetadata {
    fn default() -> Self {
        Self::WM(WMTileMetadata::default())
    }
}
impl HasLayer for TileMetadata {
    fn get_layer(&self) -> Option<String> {
        None
    }
}

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
    fn build_feature(&self) -> VectorFeature<TileMetadata, (), D>;
}

/// Elevation converter
pub type ElevationConverter = fn(r: u8, g: u8, b: u8, a: Option<u8>) -> f64;

/// Conver a Terrarium tile encoded elevation data into a float precision elevation
pub fn convert_terrarium_elevation_data(r: u8, g: u8, b: u8) -> f64 {
    (r as f64) * 256.0 + (g as f64) + (b as f64) / 256.0 - 32768.0
}

/// Conver a Mapbox tile encoded elevation data into a float precision elevation
pub fn convert_mapbox_elevation_data(r: u8, g: u8, b: u8) -> f64 {
    -10000. + ((r as f64) * 256. * 256. + (g as f64) * 256. + (b as f64)) * 0.1
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
    fn get_raster_tile_value(r: u8, g: u8, b: u8, _a: Option<u8>) -> Self {
        TerrariumElevation { elev: convert_terrarium_elevation_data(r, g, b) }
    }
}

/// Elevation point used by terrarium readers
#[derive(Debug, Default, Clone, MValueCompatible, Serialize, Deserialize)]
pub struct MapboxElevation {
    /// Elevation of a point
    pub elev: f64,
}
impl GetRasterTileValue for MapboxElevation {
    fn get_raster_tile_value(r: u8, g: u8, b: u8, _a: Option<u8>) -> Self {
        MapboxElevation { elev: convert_mapbox_elevation_data(r, g, b) }
    }
}

impl GetRasterTileValue for RGBA {
    fn get_raster_tile_value(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        RGBA::from_u8s(r, g, b, a.unwrap_or(255))
    }
}
