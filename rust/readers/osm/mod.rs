/// Blob defines a chunk of data to be parsed as either a header or a primitive
pub mod blob;
/// Filtering tool
pub mod filter;
/// OSM Header Block defines the contents of the file
pub mod header_block;
/// Info Block defines optional metadata that may be included into each primitive
pub mod info;
/// Node Block defines the contents of a node
pub mod node;
/// Primitive Block defines the contents of a node, way or relation
pub mod primitive;
/// Relation Block defines the contents of a relation
pub mod relation;
/// Way Block defines the contents of a way
pub mod way;

use crate::{
    data_store::{KV, KVStore, kv::file::FileKV},
    parsers::{FeatureReader, Reader},
};
use alloc::{boxed::Box, vec::Vec};
use blob::{Blob, BlobHeader};
use core::fmt::Debug;
use filter::*;
use header_block::{HeaderBlock, OSMHeader};
use node::IntermediateNode;
use pbf::{Field, Protobuf, Type};
use primitive::{OSMMetadata, PrimitiveBlock};
use relation::IntermediateRelation;
use s2json::{MValue, Properties, VectorFeature, VectorPoint};
use way::{IntermediateWay, WayNodes};

// TODO: Add threads for reading the blocks

/// OSM Reader options
#[derive(Debug, Clone)]
pub struct OSMReaderOptions {
    /// if true, remove nodes that have no tags [Default = true]
    pub remove_empty_nodes: bool,
    /// If provided, filters of the
    pub tag_filter: Option<OSMTagFilter>,
    /// If set to true, nodes will be skipped. [Default = false]
    pub skip_nodes: bool,
    /// If set to true, ways will be skipped. [Default = false]
    pub skip_ways: bool,
    /// If set to true, relations will be skipped. [Default = false]
    pub skip_relations: bool,
    /// If set to true, ways will be converted to areas if they are closed.
    /// NOTE: They are upgraded anyways if the tag "area" is set to "yes".
    /// [Default = false]
    pub upgrade_ways_to_areas: bool,
    /// If set to true, add a bbox property to each feature [Default = true]
    pub add_bbox: bool,
}
impl Default for OSMReaderOptions {
    fn default() -> Self {
        OSMReaderOptions {
            remove_empty_nodes: false,
            tag_filter: None,
            skip_nodes: false,
            skip_ways: false,
            skip_relations: false,
            upgrade_ways_to_areas: false,
            add_bbox: true,
        }
    }
}

/// OSM File Reader ensures we are using local buffers to store intermediate Nodes, Ways, and Relations
///
/// See [`OSMReader`] for full documentation.
pub type OSMFileReader<T> = OSMReader<
    T,
    FileKV<u64, VectorPoint<MValue>>,
    FileKV<u64, IntermediateNode>,
    FileKV<u64, WayNodes>,
    FileKV<u64, IntermediateWay>,
    FileKV<u64, IntermediateRelation>,
>;
/// OSM File Reader Iterator
pub type OSMFileReaderIter<'a, T> = OsmReaderIter<
    'a,
    T,
    FileKV<u64, VectorPoint<MValue>>,
    FileKV<u64, IntermediateNode>,
    FileKV<u64, WayNodes>,
    FileKV<u64, IntermediateWay>,
    FileKV<u64, IntermediateRelation>,
>;

/// OSM Buffer Reader ensures we are using local buffers to store intermediate Nodes, Ways, and Relations
///
/// See [`OSMReader`] for full documentation.
pub type OSMLocalReader<T> = OSMReader<
    T,
    KV<u64, VectorPoint<MValue>>,
    KV<u64, IntermediateNode>,
    KV<u64, WayNodes>,
    KV<u64, IntermediateWay>,
    KV<u64, IntermediateRelation>,
>;
/// OSM Buffer Reader Iterator
pub type OSMLocalReaderIter<'a, T> = OsmReaderIter<
    'a,
    T,
    KV<u64, VectorPoint<MValue>>,
    KV<u64, IntermediateNode>,
    KV<u64, WayNodes>,
    KV<u64, IntermediateWay>,
    KV<u64, IntermediateRelation>,
>;

/// # OSM Reader
///
/// ## Description
/// Parses OSM PBF files
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
///
/// For Simplicity, you can use the [`OSMFileReader`] or [`OSMLocalReader`] wrappers.
///
/// The methods you have access to:
/// - [`OSMReader::new`]: Create a new OSMReader
/// - [`OSMReader::cleanup`]: Cleans up the reader's temp data
/// - [`OSMReader::get_header`]: Get the OSM Header
/// - [`OSMReader::parse_blocks`]: Parse all blocks to prepare for reads
/// - [`OSMReader::next_block`]: Get the next block (if you want to iteratively use blocks yourself)
/// - [`OSMReader::par_parse_node_blocks`]: If you are only interested in nodes, parse all of them quicker using threads
/// - [`OSMReader::parse_node_blocks`]: If you are only interested in nodes, parse all of them quicker
/// - [`OSMReader::iter`]: Create a new OSMReader Iterator
///
/// ### Local Reader
/// All data is stored locally in memory and will be cleaned up on drop
///
/// ```rust
/// use gistools::{parsers::{FileReader, FeatureReader}, readers::OSMLocalReader};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/osm/fixtures/test.pbf");
/// let path_str = path.to_str().unwrap();
/// let reader = FileReader::from(path_str);
///
/// let mut osm = OSMLocalReader::new(reader, None);
/// osm.parse_blocks();
///
/// let features: Vec<_> = osm.iter().collect();
/// assert_eq!(features.len(), 8);
/// ```
#[derive(Debug, Clone)]
pub struct OSMReader<
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>> = KV<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode> = KV<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes> = KV<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay> = KV<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation> = KV<u64, IntermediateRelation>,
> {
    /// The input reader
    reader: T,
    /// if true, skip nodes that have no tags [Default = true]
    skip_empty_nodes: bool,
    /// If provided, filters of the
    tag_filter: Option<OSMTagFilter>,
    /// If set to true, nodes will be skipped
    skip_nodes: bool,
    /// If set to true, ways will be skipped
    skip_ways: bool,
    /// If set to true, relations will be skipped
    skip_relations: bool,
    /// If set to true, ways will be converted to areas if they are closed
    /// NOTE: They are upgraded anyways if the tag "area" is set to "yes"
    /// [Default = false]
    upgrade_ways_to_areas: bool,
    /// If set to true, add a bbox property to each feature
    add_bbox: bool,
    /// The current offset of the reader
    _offset: u64,
    /// track if the data has been parsed or not
    _parsed: bool,
    /// Node Geometry Store
    node_geometry: _N,
    /// Inermediate node store
    nodes: N,
    /// Way Geometry Store
    way_geometry: _W,
    /// Intermediate way store
    ways: W,
    /// Intermediate relation store
    relations: R,
}
impl<
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> OSMReader<T, _N, N, _W, W, R>
{
    /// Creates a new OSM Reader
    pub fn new(reader: T, options: Option<OSMReaderOptions>) -> Self {
        let options = options.unwrap_or_default();
        OSMReader {
            reader,
            skip_empty_nodes: options.remove_empty_nodes,
            tag_filter: options.tag_filter,
            skip_nodes: options.skip_nodes,
            skip_ways: options.skip_ways,
            skip_relations: options.skip_relations,
            upgrade_ways_to_areas: options.upgrade_ways_to_areas,
            add_bbox: options.add_bbox,
            _offset: 0,
            _parsed: false,
            node_geometry: _N::new(None),
            nodes: N::new(None),
            way_geometry: _W::new(None),
            ways: W::new(None),
            relations: R::new(None),
        }
    }

    /// Cleanup the data which will cleanup any temporary files if they exist
    pub fn cleanup(&mut self) {
        self.node_geometry.cleanup();
        self.nodes.cleanup();
        self.way_geometry.cleanup();
        self.ways.cleanup();
        self.relations.cleanup();
    }

    /// returns - The header of the OSM file
    pub fn get_header(&mut self) -> OSMHeader {
        self._offset = 0;
        let blob_header = self.next();
        if blob_header.is_none() {
            panic!("OSM header not found");
        }
        let bytes = blob_header.unwrap();
        let mut pbf = Protobuf::from(bytes.clone());
        let mut header_block = HeaderBlock::default();
        let Field { tag, r#type } = pbf.read_field();
        if tag != 1 || r#type != Type::Bytes {
            return OSMHeader::default();
        }
        pbf.read_message(&mut header_block);

        header_block.to_header()
    }

    fn next_blob(&mut self) -> Option<BlobHeader> {
        // if we've already read all the data, return null
        if self._offset >= self.reader.len() {
            return None;
        }
        // STEP 1: Get blob size
        // read length of current blob
        let length = self.reader.int32_be(Some(self._offset)) as u64;
        self._offset += 4;
        let blob_header_data = self.reader.slice(Some(self._offset), Some(self._offset + length));
        self._offset += length;
        // build a blob header
        let mut pbf: Protobuf = blob_header_data.into();
        let mut blob_header = BlobHeader::default();
        pbf.read_fields(&mut blob_header, None);
        Some(blob_header)
    }

    /// Read the next blob
    ///
    /// ## Returns
    /// The next blob if it exists
    fn next(&mut self) -> Option<Vec<u8>> {
        if let Some(blob_header) = self.next_blob() {
            // STEP 2: Get blob data
            let compressed_blob_data =
                self.reader.slice(Some(self._offset), Some(self._offset + blob_header.datasize));
            self._offset += blob_header.datasize;
            Some(compressed_blob_data)
        } else {
            None
        }
    }

    /// Skip a block of data
    fn skip(&mut self) {
        if let Some(blob_header) = self.next_blob() {
            self._offset += blob_header.datasize;
        }
    }

    /// Parse all blocks, storing all nodes, ways, and relations into local stores for future consumption
    pub fn parse_blocks(&mut self) {
        if self._parsed {
            return;
        }
        self._offset = 0;
        // skip the header
        self.skip();
        while let Some(b) = self.next() {
            self.parse_block(OSMReader::<T, _N, N, _W, W, R>::next_block(b));
        }
        self._parsed = true;
    }

    /// Read the input blob and parse the block of data
    pub fn next_block(data: Vec<u8>) -> PrimitiveBlock {
        // Blob data is PBF encoded and ?compressed, so we need to parse & decompress it first
        let mut pbf: Protobuf = data.into();
        let mut blob = Blob::default();
        pbf.read_fields(&mut blob, None);
        let mut pbf: Protobuf = blob.data.into();
        // Parse the PrimitiveBlock and read its contents.
        // all nodes/ways/relations that can be filtered already are on invocation.
        let mut pb = PrimitiveBlock::default();
        pbf.read_fields(&mut pb, None);
        pb
    }

    fn parse_block(&mut self, block: PrimitiveBlock) {
        let skip_wr = self.skip_ways && self.skip_relations;
        for group in &block.primitive_groups {
            for node in &group.nodes {
                if !node.is_filterable(&block, self) {
                    self.nodes.set(node.id, node.to_intermediate_feature(&block));
                }
                if !skip_wr {
                    self.node_geometry.set(node.id, node.to_vector_geometry(&block));
                }
            }
            if !skip_wr {
                for way in &group.ways {
                    if !way.is_filterable(&block, self) {
                        if let Some(i_way) = way.to_intermediate_feature(&block, self) {
                            self.ways.set(way.id, i_way);
                        }
                    }
                    if !self.skip_ways {
                        self.way_geometry.set(way.id, way.node_refs());
                    }
                }
                for relation in &group.relations {
                    if !relation.is_filterable(&block, self) {
                        if let Some(i_relation) = relation.to_intermediate_feature(&block) {
                            self.relations.set(relation.id, i_relation);
                        }
                    }
                }
            }
        }
    }

    /// Parse only nodes using threads. Assumed this reader has already been cloned and passed
    /// to a thread.
    pub fn par_parse_node_blocks(
        &mut self,
        pool_size: usize,
        thread_id: usize,
        cb: &mut dyn FnMut(VectorFeature<OSMMetadata, Properties, MValue>),
    ) {
        if pool_size == 0 || thread_id > pool_size {
            panic!("pool_size must be > 0 and thread_id must be <= pool_size");
        }
        // ensure an offset reset, skip header, then skip to offset of thread_id
        self._offset = 0;
        self.skip();
        for _ in 0..thread_id {
            self.skip();
        }
        // loop through the whole list of parse_node_blocks, but with a stride of pool_size
        while let Some(b) = self.next() {
            self.parse_node_block(OSMReader::<T, _N, N, _W, W, R>::next_block(b), cb);
            for _ in 0..pool_size {
                self.skip();
            }
        }
    }

    /// If you are only interested in the nodes, run this function instead as it doesn't need
    /// Prep data in memory
    pub fn parse_node_blocks(
        &mut self,
        cb: &mut dyn FnMut(VectorFeature<OSMMetadata, Properties, MValue>),
    ) {
        self._offset = 0;
        // skip the header
        self.skip();
        while let Some(b) = self.next() {
            self.parse_node_block(OSMReader::<T, _N, N, _W, W, R>::next_block(b), cb);
        }
    }

    fn parse_node_block(
        &mut self,
        block: PrimitiveBlock,
        cb: &mut dyn FnMut(VectorFeature<OSMMetadata, Properties, MValue>),
    ) {
        for group in &block.primitive_groups {
            for node in &group.nodes {
                if !node.is_filterable(&block, self) {
                    cb(node.to_intermediate_feature(&block).to_vector_feature(self.add_bbox));
                }
            }
        }
    }
}

/// OSM Reader iterator
pub struct OsmReaderIter<
    'a,
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> {
    reader: &'a OSMReader<T, _N, N, _W, W, R>,
    node_iter: Box<dyn Iterator<Item = (&'a u64, &'a IntermediateNode)> + 'a>,
    way_iter: Box<dyn Iterator<Item = (&'a u64, &'a IntermediateWay)> + 'a>,
    relation_iter: Box<dyn Iterator<Item = (&'a u64, &'a IntermediateRelation)> + 'a>,
}
impl<
    'a,
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> Debug for OsmReaderIter<'a, T, _N, N, _W, W, R>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OsmReaderIter")
    }
}

impl<
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> Iterator for OsmReaderIter<'_, T, _N, N, _W, W, R>
{
    type Item = VectorFeature<OSMMetadata, Properties, MValue>;

    fn next(&mut self) -> Option<Self::Item> {
        let node_geometry = &self.reader.node_geometry;
        let way_geometry = &self.reader.way_geometry;
        let add_bbox = self.reader.add_bbox;
        if let Some((_, node)) = self.node_iter.next() {
            Some(node.to_vector_feature(add_bbox))
        } else if let Some((_, way)) = self.way_iter.next() {
            Some(way.to_vector_feature(node_geometry, add_bbox))
        } else if let Some((_, relation)) = self.relation_iter.next() {
            relation.to_vector_feature(node_geometry, way_geometry, add_bbox)
        } else {
            None
        }
    }
}
impl<
    T: Reader,
    _N: KVStore<u64, VectorPoint<MValue>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> FeatureReader<OSMMetadata, Properties, MValue> for OSMReader<T, _N, N, _W, W, R>
{
    type FeatureIterator<'a>
        = OsmReaderIter<'a, T, _N, N, _W, W, R>
    where
        T: 'a,
        _N: 'a,
        N: 'a,
        _W: 'a,
        W: 'a,
        R: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        OsmReaderIter {
            reader: self,
            node_iter: Box::new(self.nodes.iter()),
            way_iter: Box::new(self.ways.iter()),
            relation_iter: Box::new(self.relations.iter()),
        }
    }

    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
