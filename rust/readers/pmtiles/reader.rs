use super::{
    PMDirectory, PMTilePos, S2_PM_HEADER_SIZE_BYTES, S2_PM_ROOT_SIZE, S2PMEntries, S2PMHeader,
    find_tile,
};
use crate::{
    data_structures::Cache,
    parsers::{Buffer, Reader},
    util::decompress_data,
};
use alloc::{string::String, vec::Vec};
use s2_tilejson::{Metadata, UnknownMetadata};
use s2json::Face;

/// # (S2) PMTiles Reader
///
/// ## Description
/// A V3.0 PMTiles reader for reading standard WebMercator Tile data and V1.0 S2 Tile data.
///
/// A Modified implementation of the PMTiles library. It is backwards compatible but
/// offers support for the S2 Projection.
///
/// You can learn more about the [S2PMTiles Specification here](https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md).
///
/// ## Usage
///
/// PMTilesReader utilizes any struct that implements the [`Reader`] trait.
/// Options are [`crate::parsers::BufferReader`], [`crate::parsers::FileReader`], [`crate::parsers::MMapReader`], and [`crate::parsers::FetchReader`].
///
/// The methods you have access to:
/// - [`PMTilesReader::new`]: Create a new PMTilesReader
/// - [`PMTilesReader::get_header`]: Get the PMTiles header
/// - [`PMTilesReader::get_s2_metadata`]: Get the S2 PMTiles metadata
/// - [`PMTilesReader::get_metadata`]: Get the PMTiles metadata
/// - [`PMTilesReader::get_tile_s2`]: Get an S2 Tile
/// - [`PMTilesReader::get_tile_wm`]: Get an WM Tile
/// - [`PMTilesReader::get_tile`]: Get a Tile irregardless of the projection type
///
/// ```rust
/// use gistools::{parsers::FileReader, readers::PMTilesReader};
/// use std::path::PathBuf;
///
/// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///     .join("tests/readers/pmtiles/fixtures/test_fixture_1.pmtiles");
/// let file_reader = FileReader::new(path).unwrap();
/// let mut reader = PMTilesReader::new(file_reader, None);
///
/// smol::block_on(async {
///
/// // pull out the header
/// let header = reader.get_header().await;
///
/// // get the metadata
/// let metadata = reader.get_metadata();
///
/// // S2 specific functions
/// let tile = reader.get_tile_s2(0.into(), 0, 0, 0).await;
///
/// // WM functions
/// let tile = reader.get_tile_wm(0, 0, 0).await.unwrap();
///
/// });
/// ```
///
/// ## Links
/// - <https://github.com/Open-S2/s2-pmtiles>
/// - <https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md>
/// - <https://github.com/protomaps/PMTiles>
/// - <https://github.com/protomaps/PMTiles/blob/main/spec/v3/spec.md>
#[derive(Debug)]
pub struct PMTilesReader<R: Reader> {
    header: Option<S2PMHeader>,
    root_dir: PMDirectory,
    root_dir_s2: S2PMEntries,
    metadata: Metadata,
    dir_cache: Cache<u64, PMDirectory>,
    reader: R,
}
impl<R: Reader> PMTilesReader<R> {
    /// Given an input path, read in the header and root directory
    pub fn new(reader: R, max_size: Option<usize>) -> Self {
        let max_size = max_size.unwrap_or(20);
        Self {
            header: None,
            root_dir: PMDirectory::default(),
            root_dir_s2: S2PMEntries::default(),
            metadata: Metadata::default(),
            dir_cache: Cache::new(max_size, None),
            reader,
        }
    }

    /// fetch the s2 metadata as needed
    pub async fn get_header(&mut self) -> S2PMHeader {
        if self.header.is_some() {
            return self.header.unwrap();
        }

        let data = self.get_range(0, S2_PM_ROOT_SIZE as u64).await;
        let header_data = data[0..S2_PM_HEADER_SIZE_BYTES].to_vec();
        // header
        let mut header = S2PMHeader::from_bytes(&mut header_data.into());

        // json metadata
        let json_offset = header.metadata_offset as usize;
        let json_length = header.metadata_length as usize;
        let json_metadata = decompress_data(
            &data[json_offset..(json_offset + json_length)],
            header.internal_compression,
        )
        .unwrap();
        let meta: UnknownMetadata = serde_json::from_str(&String::from_utf8_lossy(&json_metadata))
            .unwrap_or_else(|e| panic!("ERROR: {}", e));
        self.metadata = meta.to_metadata();

        // root directory data
        let root_dir_offset = header.root_directory_offset as usize;
        let root_dir_length = header.root_directory_length as usize;
        let root_dir_data = decompress_data(
            &data[root_dir_offset..(root_dir_offset + root_dir_length)],
            header.internal_compression,
        )
        .unwrap();
        self.root_dir = PMDirectory::from_buffer(&mut root_dir_data.into());

        if header.is_s2 {
            self.get_s2_metadata(&data, &mut header);
        }

        self.header = Some(header);

        header
    }

    /// If S2, we need to build the other face's root directories
    pub fn get_s2_metadata(&mut self, data: &[u8], header: &mut S2PMHeader) {
        // move the root directory to the s2 root
        self.root_dir_s2.face_0 = self.root_dir.clone();
        // add the 5 other faces
        for face in [Face::Face1, Face::Face2, Face::Face3, Face::Face4, Face::Face5] {
            let root_offset = header.get_root_offset(face) as usize;
            let root_length = header.get_root_length(face) as usize;
            let face_dir_data = decompress_data(
                &data[root_offset..(root_offset + root_length)],
                header.internal_compression,
            )
            .unwrap();
            self.root_dir_s2.set_dir(face, PMDirectory::from_buffer(&mut face_dir_data.into()));
        }
    }

    /// get the metadata
    pub fn get_metadata(&mut self) -> &Metadata {
        &self.metadata
    }

    /// get an S2 tile
    pub async fn get_tile_s2(&mut self, face: Face, zoom: u8, x: u64, y: u64) -> Option<Vec<u8>> {
        self.get_tile(Some(face), zoom, x, y).await
    }

    /// get an WM tile
    pub async fn get_tile_wm(&mut self, zoom: u8, x: u64, y: u64) -> Option<Vec<u8>> {
        self.get_tile(None, zoom, x, y).await
    }

    /// get a tile, wheather WM or S2
    pub async fn get_tile(
        &mut self,
        face: Option<Face>,
        zoom: u8,
        x: u64,
        y: u64,
    ) -> Option<Vec<u8>> {
        let header = self.get_header().await;
        let tile_id = PMTilePos::new(zoom, x, y).to_id();
        // if zoom < header.min_zoom || zoom > header.max_zoom { return None; }

        let mut d_o = header.root_directory_offset;
        let mut d_l = header.root_directory_length;

        for _ in 0..4 {
            let directory = self.get_directory(d_o, d_l, face).await;
            if directory.is_empty() {
                return None;
            }
            let entry = find_tile(&directory.entries, tile_id);
            match entry {
                None => {
                    return None;
                }
                Some(entry) => {
                    if entry.run_length > 0 {
                        let entry_data = self
                            .get_range(header.data_offset + entry.offset, entry.length as u64)
                            .await;
                        return Some(
                            decompress_data(&entry_data, header.internal_compression).unwrap(),
                        );
                    } else {
                        d_o = header.leaf_directory_offset + entry.offset;
                        d_l = entry.length as u64;
                    }
                }
            }
        }

        panic!("Maximum directory depth exceeded");
    }

    /// Get a full directory
    async fn get_directory(&mut self, offset: u64, length: u64, face: Option<Face>) -> PMDirectory {
        let dir = match face {
            None => &self.root_dir,
            Some(f) => self.root_dir_s2.get(f),
        };
        let internal_compression = self.header.unwrap().internal_compression;
        let root_directory_offset = self.header.unwrap().root_directory_offset;
        // if root_directory_offset, return roon
        if offset == root_directory_offset {
            return dir.clone();
        }
        // check cache
        if let Some(cache) = self.dir_cache.get(&offset) {
            cache.clone()
        } else {
            // get from archive
            let resp = self.get_range(offset, length).await;
            let data = decompress_data(&resp, internal_compression).unwrap();
            let mut buffer: Buffer = Buffer::new(data);
            let directory = PMDirectory::from_buffer(&mut buffer);
            if directory.is_empty() {
                panic!("Empty directory is invalid");
            }
            // save in cache
            self.dir_cache.set(offset, directory.clone());

            directory
        }
    }

    /// Get a range of bytes given an offset and length
    async fn get_range(&mut self, offset: u64, length: u64) -> Vec<u8> {
        let len = self.reader.len();
        if len != 0 {
            // This is not a FetchReader
            let end = u64::min(len, offset + length);
            self.reader.slice(Some(offset), Some(end))
        } else {
            self.reader.get_slice(offset, Some(length)).await
        }
    }
}
