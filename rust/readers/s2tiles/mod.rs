use crate::{
    data_structures::Cache,
    parsers::{Buffer, Reader},
    util::{CompressionFormat, decompress_data},
};
use alloc::{collections::BTreeMap, vec, vec::Vec};
use s2_tilejson::Metadata;
use s2json::Face;

/// A directory consists of an offset and a length pointing to a node or a leaf.
/// The maximum value for a 6-byte offset is `281,474,976,710,655`
/// This is large enough to address 281 TB of byte-indexed data.
/// - Offset: 6 bytes
/// - Length: 4 bytes
#[derive(Debug, Clone, Copy, PartialEq)]
struct Directory {
    pub offset: u64,
    pub length: u32,
}

const NODE_SIZE: usize = 10; // [offset, length] => [6 bytes, 4 bytes]
const DIR_SIZE: usize = 1_365 * NODE_SIZE; // (13_650) -> 6 levels, the 6th level has both node and leaf (1+4+16+64+256+1024)*2 => (1365)+1365 => 2_730
const METADATA_SIZE: usize = 131_072; // 131,072 bytes is 128kB. It is assumed the map metadata AND the S2Tile format metadata is less than 128kB
const ROOT_DIR_SIZE: usize = DIR_SIZE * 7; // 27_300 * 6 = 163_800
const ROOT_SIZE: usize = METADATA_SIZE + ROOT_DIR_SIZE;
// assuming all tiles exist for every face from 0->30 the max leafs to reach depth of 30 is 5
// root: 6sides * 27_300 bytes/dir = (163_800 bytes)
// all leafs at 6: 1024 * 6sides * 27_300bytes/dir (0.167731 GB)
// al leafs at 12: 524_288 * 6sides * 27_300bytes/dir (85.8783744 GB) - obviously most of this is water

/// # S2 Tiles Reader
///
/// ## Description
///
/// An S2 Tile Reader to store tile and metadata in a cloud optimized format. Similar to PMTiles
/// but simplified to have as few features as possible.
///
/// Reads either a Web Mercator tile or an S2 tile to the folder location given its (zoom, x, y) or (face, zoom, x, y) coordinates.
///
/// Reads data via the [S2Tiles specification](https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md).
///
/// ## Usage
///
/// S2TilesReader utilizes any struct that implements the [`Reader`] trait.
/// Options are [`crate::parsers::BufferReader`], [`crate::parsers::FileReader`], [`crate::parsers::MMapReader`], and [`crate::parsers::FetchReader`].
///
/// The methods you have access to:
/// - [`S2TilesReader::new`]: Create a new S2TilesReader
/// - [`S2TilesReader::get_metadata`]: Get the metadata of the archive
/// - [`S2TilesReader::has_tile_wm`]: Check if a WM tile exists in the archive
/// - [`S2TilesReader::has_tile_s2`]: Check if an S2 tile exists in the archive
/// - [`S2TilesReader::get_tile_wm`]: Get the bytes of the tile at the given (zoom, x, y) coordinates
/// - [`S2TilesReader::get_tile_s2`]: Get the bytes of the tile at the given (face, zoom, x, y) coordinates
///
/// ```rust
/// use gistools::{parsers::FileReader, readers::S2TilesReader};
/// use std::path::PathBuf;
///
/// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///     .join("tests/writers/fixtures/example.s2tiles");
/// let file_reader = FileReader::new(path).unwrap();
/// let mut reader = S2TilesReader::new(file_reader, None);
///
/// smol::block_on(async {
///
/// // get the metadata
/// let metadata = reader.get_metadata().await;
///
/// // S2 specific functions
/// assert!(reader.has_tile_s2(0.into(), 0, 0, 0).await);
/// let tile = reader.get_tile_s2(0.into(), 0, 0, 0).await;
///
/// // WM functions
/// assert!(reader.has_tile_wm(0, 0, 0).await);
/// let tile = reader.get_tile_wm(0, 0, 0).await;
///
/// });
/// ```
///
/// ## Links
/// - https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md
#[derive(Debug)]
pub struct S2TilesReader<R: Reader> {
    is_setup: bool,
    version: u16,
    maxzoom: u8,
    compression: CompressionFormat,
    metadata: Option<Metadata>,
    root_dir: BTreeMap<u8, Buffer>,
    dir_cache: Cache<u64, Buffer>,
    reader: R,
}
impl<R: Reader> S2TilesReader<R> {
    /// Create a new S2TilesReader
    ///
    /// ## Parameters
    /// - `reader` - the input reader to parse from
    /// - `max_size` - the max size of the cache before dumping old data. Defaults to 20.
    pub fn new(reader: R, max_size: Option<usize>) -> Self {
        Self {
            is_setup: false,
            version: 1,
            maxzoom: 0,
            compression: CompressionFormat::Gzip,
            metadata: None,
            root_dir: BTreeMap::new(),
            dir_cache: Cache::new(max_size.unwrap_or(20), None),
            reader,
        }
    }

    /// Get the metadata of the archive
    ///
    /// ## Returns
    /// The metadata of the archive
    pub async fn get_metadata(&mut self) -> Metadata {
        if let Some(metadata) = &self.metadata {
            return metadata.clone();
        }
        self.setup().await;
        self.metadata.clone().unwrap()
    }

    /// Check if a WM tile exists in the archive
    ///
    /// ## Parameters
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    ///
    /// ## Returns
    /// True if the tile exists in the archive
    pub async fn has_tile_wm(&mut self, zoom: u8, x: u32, y: u32) -> bool {
        self.setup().await;
        self.has_tile_s2(6.into(), zoom, x, y).await
    }

    /// Check if an S2 tile exists in the archive
    ///
    /// ## Parameters
    /// - `face`: the Open S2 projection face
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    ///
    /// ## Returns
    /// True if the tile exists in the archive
    pub async fn has_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32) -> bool {
        self.setup().await;
        // pull in the correct face's directory
        let dir = self.root_dir.get(&(face as u8)).cloned().unwrap();
        // now we walk to the next directory as necessary
        let node = self.walk(dir, zoom, x, y).await; // [offset, length]
        if let Some(node) = node {
            let Directory { offset, length } = node;
            offset != 0 && length != 0
        } else {
            false
        }
    }

    /// Get the bytes of the tile at the given (zoom, x, y) coordinates
    ///
    /// ## Parameters
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    ///
    /// ## Returns
    /// The bytes of the tile at the given (z, x, y) coordinates, or undefined if the tile
    /// does not exist in the archive.
    pub async fn get_tile_wm(&mut self, zoom: u8, x: u32, y: u32) -> Option<Vec<u8>> {
        self.setup().await;
        self.get_tile_s2(6.into(), zoom, x, y).await
    }

    /// Get the bytes of the tile at the given (face, zoom, x, y) coordinates
    ///
    /// ## Parameters
    /// - `face`: the Open S2 projection face
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    ///
    /// ## Returns
    /// The bytes of the tile at the given (face, zoom, x, y) coordinates, or undefined if
    /// the tile does not exist in the archive.
    pub async fn get_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32) -> Option<Vec<u8>> {
        self.setup().await;

        // pull in the correct face's directory
        let dir = self.root_dir.get(&(face as u8)).cloned().unwrap();
        // now we walk to the next directory as necessary
        let node = self.walk(dir, zoom, x, y).await; // [offset, length]
        if let Some(node) = node {
            let Directory { offset, length } = node;

            // we found the vector file, let's send the details off to the tile worker
            let data = self.get_range(offset, length as u64).await;
            Some(decompress_data(&data, self.compression).unwrap())
        } else {
            None
        }
    }

    /// given position and level, find the tile offset and length
    ///
    /// ## Parameters
    /// - `dir`: the directory to walk
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    ///
    /// ## Returns
    /// The offset and length of the tile if it exists
    async fn walk(&mut self, mut dir: Buffer, zoom: u8, x: u32, y: u32) -> Option<Directory> {
        let mut path = get_tile_path(zoom, x, y);
        let mut offset = 0;
        let mut length = 0;

        // walk the tree if past zoom 0
        while !path.is_empty() {
            // grab position
            let node_pos = path.remove(0) as usize * NODE_SIZE;
            // set
            offset = read_uint_48le(&mut dir, Some(node_pos as usize));
            length = dir.get_u32_at(node_pos + 6);
            if length == 0 {
                return None;
            }
            // if we are still walking, grab the new directory
            if !path.is_empty() {
                // corner case: if maxzoom matches the zoom and is divisible by 5, the leaf is actually a node
                if self.maxzoom.is_multiple_of(5)
                    && zoom == self.maxzoom
                    && path.len() == 1
                    && path[0] == 0
                {
                    return Some(Directory { offset, length });
                }
                // otherwise fetch the directory
                let next_dir = self.get_dir(offset, length).await;
                dir = next_dir;
            }
        }

        if length == 0 { None } else { Some(Directory { offset, length }) }
    }

    /// get a directory given an offset and length
    ///
    /// ## Parameters
    /// - `offset`: the offset
    /// - `length`: the length
    ///
    /// ## Returns
    /// The directory
    async fn get_dir(&mut self, offset: u64, length: u32) -> Buffer {
        if let Some(dir) = self.dir_cache.get(&offset) {
            dir.clone()
        } else {
            let data = self.get_range(offset, length as u64).await;
            let dir = Buffer::new(data);
            self.dir_cache.set(offset, dir.clone());
            dir
        }
    }

    /// Setup the reader
    async fn setup(&mut self) {
        if self.is_setup {
            return;
        }
        self.is_setup = true;
        // fetch the metadata
        let data = self.get_range(0, ROOT_SIZE as u64).await;
        // prep a data view, store in header, build metadata
        let dv = Buffer::new(data.clone());
        if dv.get_u16_at(0) != 12_883 {
            // the first two bytes are S and 2, we validate
            panic!("Bad metadata");
        }
        // parse the version, maxzoom, and compression
        self.version = dv.get_u16_at(2);
        self.maxzoom = dv.get_u8_at(4);
        self.compression = CompressionFormat::from(dv.get_u8_at(5));
        // parse the JSON metadata length and offset
        let m_l = dv.get_u32_at(6);
        if m_l == 0 {
            // if the metadata is empty, we failed
            panic!("Failed to extrapolate metadata");
        }
        let meta_data =
            decompress_data(&data[10..(10 + (m_l as usize))], self.compression).unwrap();
        self.metadata = Some(serde_json::from_slice(&meta_data).unwrap());
        // create root directories
        for face in [0, 1, 2, 3, 4, 5, 6] {
            let start = METADATA_SIZE + (face as usize) * DIR_SIZE;
            self.root_dir.insert(face, Buffer::new(data[start..(start + DIR_SIZE)].to_vec()));
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

/// read a 48 bit number
///
/// ## Parameters
/// - `buffer`: the buffer
/// - `offset`: the offset
///
/// ## Returns
/// The number
fn read_uint_48le(buffer: &mut Buffer, offset: Option<usize>) -> u64 {
    let offset = offset.unwrap_or(0);
    buffer.get_u32_at(offset + 2) as u64 * (1 << 16) + buffer.get_u16_at(offset) as u64
}

/// Get the path to a tile
///
/// ## Parameters
/// - `zoom`: the zoom
/// - `x`: the x
/// - `y`: the y
///
/// ## Returns
/// The path as a collection of offsets pointing to the tile Node in the directory
pub fn get_tile_path(mut zoom: u8, mut x: u32, mut y: u32) -> Vec<u64> {
    let mut path = vec![];

    while zoom >= 5 {
        path.push((5, x & 31, y & 31));
        x >>= 5;
        y >>= 5;
        zoom = zoom.saturating_sub(5);
    }
    path.push((zoom, x, y));

    path.into_iter()
        .map(|(zoom, x, y)| {
            let val = (y as u64) * ((1 << zoom) as u64) + (x as u64);
            let sum: u64 = (0..zoom).map(|z| (1 << z) * (1 << z)).sum();
            val + sum
        })
        .collect()
}
