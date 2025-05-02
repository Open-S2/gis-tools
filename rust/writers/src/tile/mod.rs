/// File Based Tile Writer
#[cfg(feature = "std")]
pub mod file;

use alloc::{collections::BTreeMap, vec::Vec};
#[cfg(feature = "std")]
pub use file::*;
use s2_tilejson::Metadata;
use s2json::Face;
use util::{CompressionFormat, Date};

/// A base interface for all tile stores.
pub trait TileWriter {
    /// Write a Web Mercator tile to the folder location given its (zoom, x, y) coordinates.
    fn write_tile_wm(&mut self, zoom: u8, x: u32, y: u32, data: Vec<u8>);
    /// Write a S2 tile to the folder location given its (face, zoom, x, y) coordinates.
    fn write_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32, data: Vec<u8>);
    /// Write the metadata to the folder location.
    fn commit(&mut self, metadata: Metadata, tile_compression: Option<CompressionFormat>);
}

/// A base interface for all tile stores.
pub trait TemporalTileWriter {
    /// Write a time series tile to the folder location given its (t, z, x, y) coordinates.
    fn write_temporal_tile_wm(&mut self, time: Date, zoom: u8, x: u32, y: u32, data: Vec<u8>);
    /// Write a time series tile to the folder location given its (t, face, zoom, x, y) coordinates.
    fn write_temporal_tile_s2(
        &mut self,
        time: Date,
        face: Face,
        zoom: u8,
        x: u32,
        y: u32,
        data: Vec<u8>,
    );
}

/// Key store for Web Mercator tiles
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Default)]
pub struct WMTileKey {
    /// The zoom level
    pub zoom: u8,
    /// The tile X coordinate
    pub x: u32,
    /// The tile Y coordinate
    pub y: u32,
}

/// Key store for S2 tiles
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Default)]
pub struct S2TileKey {
    /// The Open S2 projection face
    pub face: Face,
    /// The zoom level
    pub zoom: u8,
    /// The tile X coordinate
    pub x: u32,
    /// The tile Y coordinate
    pub y: u32,
}

/// A Temporal key store for Web Mercator
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone)]
pub struct WMTemporalTileKey {
    /// The date of the data
    pub time: Date,
    /// The zoom level
    pub zoom: u8,
    /// The tile X coordinate
    pub x: u32,
    /// The tile Y coordinate
    pub y: u32,
}

/// A Temporal key store for S2
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone)]
pub struct S2TemporalTileKey {
    /// The date of the data
    pub time: Date,
    /// The Open S2 projection face
    pub face: Face,
    /// The zoom level
    pub zoom: u8,
    /// The tile X coordinate
    pub x: u32,
    /// The tile Y coordinate
    pub y: u32,
}

/// A key store for Web Mercator and S2 tiles
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone)]
pub enum TileKey {
    /// A key store for Web Mercator tiles
    WM(WMTileKey),
    /// A key store for S2 tiles
    S2(S2TileKey),
    /// A key store for Web Mercator time series tiles
    WMTime(WMTemporalTileKey),
    /// A key store for S2 time series tiles
    S2Time(S2TemporalTileKey),
}

/// A Local Memory Tile Write Store
/// Useful for testing
#[derive(Default, Debug, Clone)]
pub struct LocalTileWriter {
    /// The metadata
    pub metadata: Option<Metadata>,
    /// The tiles
    pub tiles: BTreeMap<TileKey, Vec<u8>>,
}
impl LocalTileWriter {
    /// Create a new Local Memory Tile Write Store
    pub fn new() -> LocalTileWriter {
        LocalTileWriter { metadata: None, tiles: BTreeMap::new() }
    }

    /// Grab the metadata
    pub fn metadata(&self) -> Option<Metadata> {
        self.metadata.clone()
    }

    /// Grab a WM tile
    pub fn get_tile_wm(&self, zoom: u8, x: u32, y: u32) -> Option<Vec<u8>> {
        let key = WMTileKey { zoom, x, y };
        self.tiles.get(&TileKey::WM(key)).cloned()
    }

    /// Grab an S2 tile
    pub fn get_tile_s2(&self, face: Face, zoom: u8, x: u32, y: u32) -> Option<Vec<u8>> {
        let key = S2TileKey { face, zoom, x, y };
        self.tiles.get(&TileKey::S2(key)).cloned()
    }
}
impl TileWriter for LocalTileWriter {
    fn write_tile_wm(&mut self, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let key = WMTileKey { zoom, x, y };
        self.tiles.insert(TileKey::WM(key), data);
    }
    fn write_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let key = S2TileKey { face, zoom, x, y };
        self.tiles.insert(TileKey::S2(key), data);
    }
    fn commit(&mut self, metadata: Metadata, _tile_compression: Option<CompressionFormat>) {
        self.metadata = Some(metadata);
    }
}
impl TemporalTileWriter for LocalTileWriter {
    fn write_temporal_tile_wm(&mut self, time: Date, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let key = WMTemporalTileKey { time, zoom, x, y };
        self.tiles.insert(TileKey::WMTime(key), data);
    }
    fn write_temporal_tile_s2(
        &mut self,
        time: Date,
        face: Face,
        zoom: u8,
        x: u32,
        y: u32,
        data: Vec<u8>,
    ) {
        let key = S2TemporalTileKey { time, face, zoom, x, y };
        self.tiles.insert(TileKey::S2Time(key), data);
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_tile_writing() {
        let mut writer = LocalTileWriter::new();

        writer.write_tile_wm(16, 2, 3, vec![0, 1, 2]);
        writer.write_tile_s2(4.into(), 3, 2, 1, vec![3, 2, 1]);
        writer.write_temporal_tile_wm(Date::new(2024, 8, 2), 3, 0, 0, vec![1, 1, 1]);
        writer.write_temporal_tile_s2(Date::new(2002, 1, 1), 0.into(), 0, 0, 0, vec![2, 2, 2]);

        writer.commit(Metadata::default(), Some(CompressionFormat::Brotli));

        assert_eq!(
            writer.tiles.get(&TileKey::S2(S2TileKey { face: 4.into(), zoom: 3, x: 2, y: 1 })),
            Some(&vec![3, 2, 1])
        );
    }
}
