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

#[cfg(test)]
mod tests {
    use std::env;

    // use crate::readers::{RasterTileReader, TileReader};
    use super::*;
    use alloc::vec;

    #[test]
    fn test_tile_writing() {
        let tmp_d = env::temp_dir().join("test_tile_writing");
        fs::create_dir_all(&tmp_d).unwrap();
        let tmp_dir: String = tmp_d.to_string_lossy().into();

        let mut writer = FileTileWriter::new(tmp_dir, Some("webp".into()));

        writer.write_tile_wm(16, 2, 3, vec![0, 1, 2]);
        writer.write_tile_s2(4.into(), 3, 2, 1, vec![3, 2, 1]);
        writer.write_temporal_tile_wm(Date::new(2024, 8, 2), 3, 0, 0, vec![1, 1, 1]);
        writer.write_temporal_tile_s2(Date::new(2002, 1, 1), 0.into(), 0, 0, 0, vec![2, 2, 2]);

        writer.commit(Metadata::default(), Some(CompressionFormat::Brotli));

        // let tile_reader = RasterTileReader::new(path);

        // assert_eq!(
        //     writer.tiles.get(&TileKey::S2(S2TileKey { face: 4.into(), zoom: 3, x: 2, y: 1 })),
        //     Some(&vec![3, 2, 1])
        // );
    }
}
