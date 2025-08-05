use crate::{
    geometry::S2CellId,
    parsers::{Buffer, Writer},
    util::{CompressionFormat, compress_data},
    writers::TileWriter,
};
use alloc::{vec, vec::Vec};
use s2_tilejson::Metadata;
use s2json::Face;

type Node = Directory;

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
const ROOT_DIR_SIZE: usize = DIR_SIZE * 6; // 27_300 * 6 = 163_800
const ROOT_SIZE: usize = METADATA_SIZE + ROOT_DIR_SIZE;
// assuming all tiles exist for every face from 0->30 the max leafs to reach depth of 30 is 5
// root: 6sides * 27_300bytes/dir = (163_800 bytes)
// all leafs at 6: 1024 * 6sides * 27_300bytes/dir (0.167731 GB)
// al leafs at 12: 524_288 * 6sides * 27_300bytes/dir (85.8783744 GB) - obviously most of this is water

/// # S2 Tiles Writer
///
/// ## Description
///
/// An S2 Tile Writer to store tile and metadata in a cloud optimized format. Similar to PMTiles
/// but simplified to have as few features as possible.
///
/// Writes either a Web Mercator tile or an S2 tile to the folder location given its (zoom, x, y) or (face, zoom, x, y) coordinates.
///
/// ## Usage
///
/// This struct uses the [`TileWriter`] trait.
///
/// Takes a [`Writer`] input to write data to.
///
/// The methods you have access to:
/// - [`S2TilesWriter::new`]: Create a new S2TilesWriter
/// - [`S2TilesWriter::write_tile_wm`]: Write a Web Mercator tile to the folder location given its (zoom, x, y) coordinates.
/// - [`S2TilesWriter::write_tile_s2`]: Write a S2 tile to the folder location given its (face, zoom, x, y) coordinates.
/// - [`S2TilesWriter::commit`]: Write the metadata to the folder location.
/// - [`S2TilesWriter::writer`]: Borrow the writer mutably if needed
/// - [`S2TilesWriter::put_tile_fzxy`]: Write a tile to the S2Tiles file given its (face, zoom, x, y) coordinates.
/// - [`S2TilesWriter::put_tile`]: Inserts a tile into the S2Tiles store.
///
/// ```rust
/// use gistools::{
///     parsers::{BufferWriter, Writer},
///     util::CompressionFormat,
///     writers::{S2TilesWriter, TileWriter},
/// };
/// use s2_tilejson::Metadata;
///
/// let local_writer = BufferWriter::default();
/// let mut s2tiles_writer = S2TilesWriter::new(local_writer, 9, CompressionFormat::None);
///
/// let s = String::from("hello world");
/// let buf = s.as_bytes().to_vec();
///
/// // write data in an wm tile
/// s2tiles_writer.write_tile_wm(0, 0, 0, buf.clone());
/// // or write data in an s2 tile
/// s2tiles_writer.write_tile_s2(0.into(), 0, 0, 0, buf.clone());
///
/// // finish
/// s2tiles_writer.commit(Metadata::default(), None);
/// ```
///
/// ## Links
/// - https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md
#[derive(Debug)]
pub struct S2TilesWriter<W: Writer> {
    offset: u64,
    version: u16,
    maxzoom: u8,
    writer: W,
    compression: CompressionFormat,
}
impl<W: Writer> TileWriter for S2TilesWriter<W> {
    fn write_tile_wm(&mut self, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        self.put_tile_fzxy(0.into(), zoom, x, y, data);
    }
    fn write_tile_s2(&mut self, face: Face, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        self.put_tile_fzxy(face, zoom, x, y, data);
    }
    /// Finish writing by building the header with root and leaf directories
    fn commit(&mut self, metadata: Metadata, tile_compression: Option<CompressionFormat>) {
        let compression = tile_compression.unwrap_or(self.compression);
        // set the ID, version, and compression type
        let mut data = Buffer::new(vec![10]);
        // Store format metadata
        data.set_u8_at(0, 83); // S
        data.set_u8_at(1, 50); // 2
        data.set_u16_at(2, self.version);
        data.set_u8_at(4, self.maxzoom);
        data.set_u8_at(5, compression as u8);
        // store the metadata's length then actual data
        let mut meta_buffer = serde_json::to_vec(&metadata).unwrap();
        meta_buffer = compress_data(meta_buffer, compression).unwrap();
        if meta_buffer.len() > METADATA_SIZE - 10 {
            panic!("Metadata too large for S2Tiles");
        }
        data.set_u32_at(6, meta_buffer.len() as u32);
        // store the format metadata and lengthen the writer to fill METADATA_SIZE. Then store the map metadata
        self.writer.write(&data.take(), 0);
        self.writer.write(&meta_buffer, 10);
    }
}
impl<W: Writer> S2TilesWriter<W> {
    /// given a compression scheme, maxzoom, and a data writer, create an instance to
    /// start storing tiles and metadata.
    /// Compression describes how both the tiles and the metadata are compressed
    pub fn new(writer: W, maxzoom: u8, compression: CompressionFormat) -> Self {
        let mut writer =
            S2TilesWriter { offset: ROOT_SIZE as u64, version: 1, maxzoom, compression, writer };
        writer.writer.append(&vec![0u8; ROOT_SIZE]);
        writer
    }

    /// Borrow the writer
    pub fn writer(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Write a tile to the S2Tiles file given its (face, zoom, x, y) coordinates.
    ///
    /// ## Parameters
    /// - `face`: the Open S2 projection face
    /// - `zoom`: the zoom level
    /// - `x`: the tile X coordinate
    /// - `y`: the tile Y coordinate
    /// - `data`: the tile data to store
    pub fn put_tile_fzxy(&mut self, face: Face, zoom: u8, x: u32, y: u32, data: Vec<u8>) {
        let id = S2CellId::from_face_ij(face.into(), x, y, Some(zoom));
        self.put_tile(id, data);
    }

    /// Inserts a tile into the S2Tiles store.
    ///
    /// ## Parameters
    /// - `id`: the tile ID
    /// - `data`: the tile data
    pub fn put_tile(&mut self, id: S2CellId, data: Vec<u8>) {
        // const length = data.byteLength;
        let length = data.len() as u32;
        // first create node, setting offset
        let node = Node { offset: self.offset, length };
        self.writer.append(&data);
        self.offset += length as u64;
        // store node in the correct directory
        self.put_node_in_dir(id, node);
    }

    /// Work our way towards the correct parent directory.
    /// If parent directory does not exists, we create it.
    ///
    /// ## Parameters
    /// - `id`: the s2cellID
    /// - `node`: the node
    fn put_node_in_dir(&mut self, id: S2CellId, node: Node) {
        // use the s2cellID and move the cursor
        let cursor = self.walk(id);
        // finally store
        self.write_node(cursor, node);
    }

    /// given position and level, explain where to adust the cursor to file
    ///
    /// ## Parameters
    /// - `id`: the s2cellID
    ///
    /// ## Returns
    /// The new cursor position
    fn walk(&mut self, id: S2CellId) -> u64 {
        // grab properties
        let (face, level, i, j) = id.to_face_ij();

        let mut cursor = (METADATA_SIZE + DIR_SIZE * face as usize) as u64;
        let mut leaf;
        let mut depth = 0;
        let mut path = get_path(level, i, j);

        while !path.is_empty() {
            // grab movement
            let shift = path.remove(0);
            depth += 1;
            // update cursor position
            cursor += shift * NODE_SIZE as u64;

            if !path.is_empty() {
                // if we hit a leaf, adjust nodePos position and move cursor to new directory
                // if we are at the max zoom, we are already in the correct position (the "leaf" is actually a node instead)
                if self.maxzoom % 5 == 0 && path.len() == 1 && level == self.maxzoom && path[0] == 0
                {
                    return cursor;
                }
                // grab the leaf from the file
                let mut leaf_node =
                    Buffer::new(self.writer.slice(cursor, cursor + NODE_SIZE as u64));
                leaf = read_uint_48le(&mut leaf_node, None);
                // if the leaf doesn't exist we create a new directory to host it
                if leaf == 0 {
                    cursor = self.create_leaf(cursor, depth * 5);
                } else {
                    cursor = leaf;
                } // move to where leaf is pointing
            }
        }

        cursor
    }

    /// Create a new leaf directory
    ///
    /// ## Parameters
    /// - `cursor`: the cursor
    /// - `depth`: the depth
    ///
    /// ## Returns
    /// The offset of the new leaf
    fn create_leaf(&mut self, cursor: u64, depth: u64) -> u64 {
        // build directory size according to maxzoom
        let dir_size = build_dir_size(depth, self.maxzoom as u64);
        // create offset & node
        let offset = self.offset;
        let node = Node { offset, length: dir_size };
        // create a dir of said size and update to new offset
        self.writer.write(&vec![0u8; dir_size as usize], offset);
        self.offset += dir_size as u64;
        // store our newly created directory as a leaf directory in our current directory
        self.write_node(cursor, node);

        // return the offset of the leaf directory
        offset
    }

    /// Writes a node to the file
    ///
    /// ## Parameters
    /// - `cursor`: the cursor
    /// - `node`: the node
    fn write_node(&mut self, cursor: u64, node: Node) {
        let Node { offset, length } = node;
        // write offset and length to buffer
        let mut node_buf = Buffer::new(vec![0; NODE_SIZE]);
        write_uint48_le(&mut node_buf, offset, 0);
        node_buf.set_u32_at(6, length);
        // write buffer to file at directory offset
        self.writer.write(&node_buf.take(), cursor);
    }
}

/// write a 32 bit and a 16 bit
///
/// ## Parameters
/// - `data`: the data to write to
/// - `num`: the number
/// - `offset`: the offset to write at
fn write_uint48_le(data: &mut Buffer, num: u64, offset: usize) {
    let lower = (num & 0xffff) as u16;
    let upper = (num >> 16) as u32;
    data.set_u16_at(offset, lower);
    data.set_u32_at(offset + 2, upper);
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

/// Build a directory size relative to maxzoom
///
/// ## Parameters
/// - `depth`: the depth
/// - `maxzoom`: the maxzoom
///
/// ## Returns
/// The directory size
fn build_dir_size(depth: u64, maxzoom: u64) -> u32 {
    let mut dir_size = 0;
    // grab the remainder
    let mut remainder = u64::min(maxzoom - depth, 5); // must be increments of 5, so if level 4 then inc is 0 but if 5, inc is 5
    // for each remainder (including 0), we add a quadrant
    loop {
        dir_size += (1 << remainder) * (1 << remainder);
        remainder -= 1;
        if remainder == 0 {
            break;
        }
    }

    dir_size * NODE_SIZE as u32
}

/// Get the path to a tile
///
/// ## Parameters
/// - `zoom`: the zoom
/// - `x`: the x
/// - `y`: the y
///
/// ## Returns
/// The path
fn get_path(mut zoom: u8, mut x: u32, mut y: u32) -> Vec<u64> {
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
