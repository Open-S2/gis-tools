use super::{TemporalTileWriter, TileWriter};
use crate::util::{CompressionFormat, Date};
use alloc::{format, string::String, vec::Vec};
use s2_tilejson::Metadata;
use s2json::Face;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// A Local Memory Tile Write Store
/// Useful for testing
#[derive(Debug)]
pub struct FileTileWriter {
    /// The tiles
    path: PathBuf,
    /// The extension
    extension: String,
}
impl FileTileWriter {
    /// Create a new Tile FileWriter, truncating the file if it exists
    pub fn new<P: AsRef<Path>>(path: P, extension: Option<String>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            extension: extension.unwrap_or("vector.pbf".into()),
        }
    }
}
impl TileWriter for FileTileWriter {
    fn write_tile_wm(&mut self, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let dir_path = self.path.join(format!("{}/{}", zoom, x));
        fs::create_dir_all(&dir_path).expect("Failed to create directories");
        let tile_path = dir_path.join(format!("{}.{}", y, self.extension));
        fs::write(tile_path, data).expect("Failed to write tile data");
    }
    fn write_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let dir_path = self.path.join(format!("{}/{}/{}/{}", u8::from(face), zoom, x, y));
        fs::create_dir_all(&dir_path).expect("Failed to create directories");
        let tile_path = dir_path.join(format!("{}.{}", y, self.extension));
        fs::write(tile_path, data).expect("Failed to write tile data");
    }
    fn commit(&mut self, metadata: Metadata, _tile_compression: Option<CompressionFormat>) {
        fs::create_dir_all(&self.path).expect("Failed to create directories");
        let meta_path = self.path.join("metadata.json");
        fs::write(meta_path, serde_json::to_string(&metadata).unwrap())
            .expect("Failed to write metadata");
    }
}
impl TemporalTileWriter for FileTileWriter {
    fn write_temporal_tile_wm(&mut self, time: Date, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let dir_path = self.path.join(format!("{}/{}/{}/{}", time.to_iso_string(), zoom, x, y));
        fs::create_dir_all(&dir_path).expect("Failed to create directories");
        let tile_path = dir_path.join(format!("{}.{}", y, self.extension));
        fs::write(tile_path, data).expect("Failed to write tile data");
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
        let dir_path = self.path.join(format!(
            "{}/{}/{}/{}/{}",
            time.to_iso_string(),
            u8::from(face),
            zoom,
            x,
            y
        ));
        fs::create_dir_all(&dir_path).expect("Failed to create directories");
        let tile_path = dir_path.join(format!("{}.{}", y, self.extension));
        fs::write(tile_path, data).expect("Failed to write tile data");
    }
}
