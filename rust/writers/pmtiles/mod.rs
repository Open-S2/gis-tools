use crate::{
    readers::{
        PMDirectory, PMEntry, PMHeader, PMTilePos, PMTileType, S2PMEntries, S2PMHeader,
        S2_PM_HEADER_SIZE_BYTES, S2_PM_ROOT_SIZE,
    },
    util::CompressionFormat,
};
use alloc::{vec, vec::Vec};
use s2_tilejson::Metadata;
use s2json::Face;

use super::Writer;

/// The result of an optimized directory computation
#[derive(Debug, Clone, Default)]
struct OptimizedDirectory {
    /// The root directory bytes
    pub root_bytes: Vec<u8>,
    /// The leaf directories bytes
    pub leaves_bytes: Vec<u8>,
    /// The number of leaf directories
    #[allow(dead_code)]
    pub num_leaves: u64,
}
impl OptimizedDirectory {
    /// Optimize the directory for storage
    pub fn optimize_directories(
        directory: &mut PMDirectory,
        target_root_length: usize,
    ) -> OptimizedDirectory {
        directory.entries.sort_by(|a, b| a.tile_id.cmp(&b.tile_id));
        let test_bytes = directory.serialize();
        if test_bytes.len() < target_root_length {
            OptimizedDirectory { root_bytes: test_bytes, leaves_bytes: Vec::new(), num_leaves: 0 }
        } else {
            let mut leaf_size = 4096;
            loop {
                let build = OptimizedDirectory::build_root_leaves(directory, leaf_size);
                if build.root_bytes.len() < target_root_length {
                    return build;
                }
                leaf_size *= 2;
            }
        }
    }

    /// Build the root and leaf directories
    pub fn build_root_leaves(directory: &PMDirectory, leaf_size: usize) -> OptimizedDirectory {
        let mut root_entries = PMDirectory::default();
        let mut leaves_bytes = Vec::<u8>::new();
        let mut num_leaves = 0;

        let mut i = 0;
        let entries = &directory.entries;
        while i < entries.len() {
            num_leaves += 1;
            let mut end = i + leaf_size;
            if i + leaf_size > entries.len() {
                end = entries.len();
            }
            let new_dir_slice = PMDirectory::new(entries[i..end].to_vec());
            let serialized = new_dir_slice.serialize();
            let entry = PMEntry {
                tile_id: entries[i].tile_id,
                offset: leaves_bytes.len() as u64,
                length: serialized.len() as u32,
                run_length: 0,
            };
            root_entries.entries.push(entry);
            leaves_bytes.extend(serialized);
            i += leaf_size;
        }

        OptimizedDirectory { root_bytes: root_entries.serialize(), leaves_bytes, num_leaves }
    }
}

/// The data writer
pub trait DataWriter: core::fmt::Debug {
    /// Write data at the specified offset
    fn write_data(&mut self, data: &[u8], offset: u64);
    /// Append data to the end of the storage
    fn append_data(&mut self, data: &[u8]);
    /// Assuming local writer, take ownership of the data when finished writing it
    fn take(&self) -> Vec<u8>;
}

/// The File reader is to be used by the local filesystem.
#[derive(Debug)]
pub struct PMTilesWriter<W: Writer> {
    tile_entries: PMDirectory,
    s2tile_entries: S2PMEntries,
    offset: u64,
    addressed_tiles: u64,
    clustered: bool,
    compression: CompressionFormat,
    writer: W,
}
impl<W: Writer> PMTilesWriter<W> {
    /// given a compression scheme and a data writer, create an instance to start storing tiles
    /// and metadata.
    /// Compression will only describle how tiles are stored, nothing more.
    pub fn new(writer: W, compression: CompressionFormat) -> Self {
        let root_data = vec![0u8; S2_PM_ROOT_SIZE];
        let mut writer = PMTilesWriter {
            tile_entries: PMDirectory::default(),
            s2tile_entries: S2PMEntries::default(),
            offset: 0,
            addressed_tiles: 0,
            clustered: false,
            compression,
            writer,
        };
        writer.writer.append(&root_data);
        writer
    }

    /// take ownership of writer data (if local this actually has content)
    pub fn take(&mut self) -> Vec<u8> {
        self.writer.take()
    }

    /// Write a tile to the PMTiles file given its (face, zoom, x, y) coordinates.
    pub fn write_tile_xyz(&mut self, zoom: u8, x: u64, y: u64, data: &[u8]) {
        let tile_id = PMTilePos::new(zoom, x, y).to_id();
        self.write_tile(tile_id, data, None);
    }

    /// Write a tile to the PMTiles file given its (face, zoom, x, y) coordinates.
    pub fn write_tile_s2(&mut self, face: Face, zoom: u8, x: u64, y: u64, data: &[u8]) {
        let tile_id = PMTilePos::new(zoom, x, y).to_id();
        self.write_tile(tile_id, data, Some(face));
    }

    /// Write a tile to the PMTiles file given its tile ID.
    pub fn write_tile(&mut self, tile_id: u64, data: &[u8], face: Option<Face>) {
        let length = data.len();
        let tile_entries = match face {
            None => &mut self.tile_entries,
            Some(f) => self.s2tile_entries.get_mut(f),
        };
        if !tile_entries.is_empty() && tile_id < tile_entries.last().unwrap().tile_id {
            self.clustered = false;
        }

        let offset = self.offset;
        self.writer.append(data);
        tile_entries.insert(PMEntry { tile_id, offset, length: length as u32, run_length: 1 });
        self.offset += length as u64;

        self.addressed_tiles += 1;
    }

    /// Finish writing by building the header with root and leaf directories
    pub fn commit(&mut self, metadata: &Metadata) {
        if !self.tile_entries.is_empty() {
            self.commit_wm(metadata);
        } else {
            self.commit_s2(metadata);
        }

        self.writer.flush();
    }

    /// Finish writing by building the header with root and leaf directories
    pub fn commit_wm(&mut self, metadata: &Metadata) {
        // build metadata
        let meta_buffer = serde_json::to_vec(metadata).unwrap();

        // optimize directories
        let od: OptimizedDirectory = OptimizedDirectory::optimize_directories(
            &mut self.tile_entries,
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes, leaves_bytes, .. } = od;

        // build header data
        let root_directory_offset = S2_PM_HEADER_SIZE_BYTES as u64;
        let root_directory_length = root_bytes.len() as u64;
        let metadata_offset = root_directory_offset + root_directory_length;
        let metadata_length = meta_buffer.len() as u64;
        let leaf_directory_offset = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length = leaves_bytes.len() as u64;
        self.offset += leaves_bytes.len() as u64;

        // write data
        self.writer.append(&leaves_bytes);
        // to make writing fasters
        let min_zoom = PMTilePos::from_id(self.tile_entries.first().unwrap().tile_id).zoom;
        let max_zoom = PMTilePos::from_id(self.tile_entries.last().unwrap().tile_id).zoom;

        // build header
        let header = PMHeader {
            version: 3,
            root_directory_offset,
            root_directory_length,
            metadata_offset,
            metadata_length,
            leaf_directory_offset,
            leaf_directory_length,
            data_offset: S2_PM_ROOT_SIZE as u64,
            data_length: self.offset,
            n_addressed_tiles: self.addressed_tiles,
            n_tile_entries: self.tile_entries.len() as u64,
            n_tile_contents: 0,
            clustered: self.clustered,
            internal_compression: CompressionFormat::None,
            tile_compression: self.compression,
            tile_type: PMTileType::Unknown,
            min_zoom,
            max_zoom,
            ..Default::default()
        };
        let serialized_header = header.to_bytes().take();

        // write header
        self.writer.write(&serialized_header, 0);
        self.writer.write(&root_bytes, root_directory_offset as usize);
        self.writer.write(&meta_buffer, metadata_offset as usize);
    }

    /// Finish writing by building the header with root and leaf directories
    pub fn commit_s2(&mut self, metadata: &Metadata) {
        // build metadata
        let meta_buffer = serde_json::to_vec(metadata).unwrap();

        // optimize directories
        let od = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face0),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes, leaves_bytes, .. } = od;
        let od1 = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face1),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes: root_bytes1, leaves_bytes: leaves_bytes1, .. } = od1;
        let od2 = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face2),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes: root_bytes2, leaves_bytes: leaves_bytes2, .. } = od2;
        let od3 = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face3),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes: root_bytes3, leaves_bytes: leaves_bytes3, .. } = od3;
        let od4 = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face4),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes: root_bytes4, leaves_bytes: leaves_bytes4, .. } = od4;
        let od5 = OptimizedDirectory::optimize_directories(
            self.s2tile_entries.get_mut(Face::Face5),
            S2_PM_ROOT_SIZE - S2_PM_HEADER_SIZE_BYTES - meta_buffer.len(),
        );
        let OptimizedDirectory { root_bytes: root_bytes5, leaves_bytes: leaves_bytes5, .. } = od5;

        // build header data
        // roots
        let root_directory_offset = S2_PM_HEADER_SIZE_BYTES as u64;
        let root_directory_length = root_bytes.len() as u64;
        let root_directory_offset1 = root_directory_offset + root_directory_length;
        let root_directory_length1 = root_bytes1.len() as u64;
        let root_directory_offset2 = root_directory_offset1 + root_directory_length1;
        let root_directory_length2 = root_bytes2.len() as u64;
        let root_directory_offset3 = root_directory_offset2 + root_directory_length2;
        let root_directory_length3 = root_bytes3.len() as u64;
        let root_directory_offset4 = root_directory_offset3 + root_directory_length3;
        let root_directory_length4 = root_bytes4.len() as u64;
        let root_directory_offset5 = root_directory_offset4 + root_directory_length4;
        let root_directory_length5 = root_bytes5.len() as u64;
        // metadata
        let metadata_offset = root_directory_offset5 + root_directory_length5;
        let metadata_length = meta_buffer.len() as u64;
        // leafs
        let leaf_directory_offset = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length = leaves_bytes.len() as u64;
        self.offset += leaf_directory_length;
        self.writer.append(&leaves_bytes);
        let leaf_directory_offset1 = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length1 = leaves_bytes1.len() as u64;
        self.offset += leaf_directory_length1;
        self.writer.append(&leaves_bytes1);
        let leaf_directory_offset2 = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length2 = leaves_bytes2.len() as u64;
        self.offset += leaf_directory_length2;
        self.writer.append(&leaves_bytes2);
        let leaf_directory_offset3 = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length3 = leaves_bytes3.len() as u64;
        self.offset += leaf_directory_length3;
        self.writer.append(&leaves_bytes3);
        let leaf_directory_offset4 = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length4 = leaves_bytes4.len() as u64;
        self.offset += leaf_directory_length4;
        self.writer.append(&leaves_bytes4);
        let leaf_directory_offset5 = self.offset + S2_PM_ROOT_SIZE as u64;
        let leaf_directory_length5 = leaves_bytes5.len() as u64;
        self.offset += leaf_directory_length5;
        self.writer.append(&leaves_bytes5);

        // write data
        self.writer.append(&leaves_bytes);
        // build header
        let header = S2PMHeader {
            is_s2: true,
            version: 3,
            root_directory_offset,
            root_directory_length,
            root_directory_offset1,
            root_directory_length1,
            root_directory_offset2,
            root_directory_length2,
            root_directory_offset3,
            root_directory_length3,
            root_directory_offset4,
            root_directory_length4,
            root_directory_offset5,
            root_directory_length5,
            metadata_offset,
            metadata_length,
            leaf_directory_offset,
            leaf_directory_length,
            leaf_directory_offset1,
            leaf_directory_length1,
            leaf_directory_offset2,
            leaf_directory_length2,
            leaf_directory_offset3,
            leaf_directory_length3,
            leaf_directory_offset4,
            leaf_directory_length4,
            leaf_directory_offset5,
            leaf_directory_length5,
            data_offset: S2_PM_ROOT_SIZE as u64,
            data_length: self.offset,
            n_addressed_tiles: self.addressed_tiles,
            n_tile_entries: self.tile_entries.len() as u64,
            n_tile_contents: 0,
            clustered: self.clustered,
            internal_compression: CompressionFormat::None,
            tile_compression: self.compression,
            tile_type: PMTileType::Unknown,
            ..Default::default()
        };
        let serialized_header = header.to_bytes().take();

        // write header
        self.writer.write(&serialized_header, 0);
        self.writer.write(&root_bytes, root_directory_offset as usize);
        self.writer.write(&root_bytes1, root_directory_offset1 as usize);
        self.writer.write(&root_bytes2, root_directory_offset2 as usize);
        self.writer.write(&root_bytes3, root_directory_offset3 as usize);
        self.writer.write(&root_bytes4, root_directory_offset4 as usize);
        self.writer.write(&root_bytes5, root_directory_offset5 as usize);
        self.writer.write(&meta_buffer, metadata_offset as usize);
    }
}

#[cfg(test)]
mod tests {
    use std::format;

    use crate::{
        readers::{BufferReader, FileReader, PMTilesReader},
        writers::{BufferWriter, FileWriter},
    };

    use super::*;

    use s2_tilejson::Metadata;

    use tempfile::NamedTempFile;

    #[test]
    fn test_file_writer_wm() {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let file_writer = FileWriter::new(&file_path).unwrap();
        let mut pmtiles_writer = PMTilesWriter::new(file_writer, CompressionFormat::None);

        // setup data
        let tmp_str = "hello world";
        // write data in tile
        pmtiles_writer.write_tile_xyz(0, 0, 0, tmp_str.as_bytes());
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let mut reader = PMTilesReader::new(FileReader::from(file_path), None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: false,
                version: 3,
                root_directory_offset: 262,
                root_directory_length: 5,
                metadata_offset: 267,
                metadata_length: 417,
                leaf_directory_offset: 98315,
                leaf_directory_length: 0,
                data_offset: 98304,
                data_length: 11,
                n_addressed_tiles: 1,
                n_tile_entries: 1,
                n_tile_contents: 0,
                tile_type: PMTileType::Unknown,
                ..Default::default()
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(*metadata, Metadata::default());

        let tile = reader.get_tile_zxy(0, 0, 0).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());

        temp_file.close().unwrap();
    }

    #[test]
    fn test_file_writer_s2() {
        let local_writer = BufferWriter::default();
        let mut pmtiles_writer = PMTilesWriter::new(local_writer, CompressionFormat::None);

        // setup data
        let tmp_str = "hello world";
        // write data in tile
        pmtiles_writer.write_tile_s2(Face::Face0, 0, 0, 0, tmp_str.as_bytes());
        pmtiles_writer.write_tile_s2(Face::Face3, 2, 1, 1, tmp_str.as_bytes());
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let pmtiles_data = pmtiles_writer.take();

        let mut reader = PMTilesReader::new(BufferReader::new(pmtiles_data), None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: true,
                version: 1,
                root_directory_offset: 262,
                root_directory_length: 5,
                metadata_offset: 276,
                metadata_length: 417,
                leaf_directory_offset: 98326,
                leaf_directory_length: 0,
                data_offset: 98304,
                data_length: 22,
                n_addressed_tiles: 2,
                n_tile_entries: 0,
                n_tile_contents: 0,
                clustered: false,
                internal_compression: CompressionFormat::None,
                tile_compression: CompressionFormat::None,
                tile_type: PMTileType::Unknown,
                min_zoom: 0,
                max_zoom: 0,
                min_longitude: 0.0,
                min_latitude: 0.0,
                max_longitude: 0.0,
                max_latitude: 0.0,
                center_zoom: 0,
                center_longitude: 0.0,
                center_latitude: 0.0,
                root_directory_offset1: 267,
                root_directory_length1: 1,
                root_directory_offset2: 268,
                root_directory_length2: 1,
                root_directory_offset3: 269,
                root_directory_length3: 5,
                root_directory_offset4: 274,
                root_directory_length4: 1,
                root_directory_offset5: 275,
                root_directory_length5: 1,
                leaf_directory_offset1: 98326,
                leaf_directory_length1: 0,
                leaf_directory_offset2: 98326,
                leaf_directory_length2: 0,
                leaf_directory_offset3: 98326,
                leaf_directory_length3: 0,
                leaf_directory_offset4: 98326,
                leaf_directory_length4: 0,
                leaf_directory_offset5: 98326,
                leaf_directory_length5: 0,
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(*metadata, Metadata::default());

        let tile = reader.get_tile_s2(Face::Face0, 0, 0, 0).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());

        let tile = reader.get_tile_s2(Face::Face3, 2, 1, 1).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());
    }

    #[test]
    fn test_file_writer_wm_large() {
        let local_writer = BufferWriter::default();
        let mut pmtiles_writer = PMTilesWriter::new(local_writer, CompressionFormat::None);

        // write tiles
        for zoom in 0..8 {
            for x in 0..(1 << zoom) {
                for y in 0..(1 << zoom) {
                    let tmp_str = format!("{}-{}-{}", zoom, x, y);
                    pmtiles_writer.write_tile_xyz(zoom, x, y, tmp_str.as_bytes());
                }
            }
        }
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let pmtiles_data = pmtiles_writer.take();

        let mut reader = PMTilesReader::new(BufferReader::new(pmtiles_data), None);

        let zoom = 5;
        let x = 12;
        let y = 30;

        let tile = reader.get_tile_zxy(zoom, x, y).unwrap();
        let tmp_str = format!("{}-{}-{}", zoom, x, y);
        assert_eq!(tile, tmp_str.as_bytes());
    }
}
