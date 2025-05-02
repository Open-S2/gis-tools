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

use alloc::{boxed::Box, vec::Vec};
use blob::{Blob, BlobHeader};
use data_store::{KV, KVStore};
use filter::*;
use header_block::{HeaderBlock, OSMHeader};
use node::IntermediateNode;
use parsers::{FeatureReader, Reader};
use pbf::Protobuf;
use primitive::{OSMMetadata, PrimitiveBlock};
use relation::IntermediateRelation;
use s2json::{Properties, VectorFeature, VectorPoint};
use way::{IntermediateWay, WayNodes};

// TODO: Add threads for reading the blocks

/// OSM Reader options
#[derive(Debug)]
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

/// OSM Buffer Reader ensures we are using local buffers to store intermediate Nodes, Ways, and Relations
pub type OSMLocalReader<T> = OSMReader<
    T,
    KV<u64, VectorPoint<()>>,
    KV<u64, IntermediateNode>,
    KV<u64, WayNodes>,
    KV<u64, IntermediateWay>,
    KV<u64, IntermediateRelation>,
>;

/// # OSM Reader
///
/// ## Description
/// Parses OSM PBF files
#[derive(Debug)]
pub struct OSMReader<
    T: Reader,
    _N: KVStore<u64, VectorPoint<()>> = KV<u64, VectorPoint<()>>,
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
    _N: KVStore<u64, VectorPoint<()>>,
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
        let mut pbf = Protobuf::from(blob_header.unwrap());
        let mut header_block = HeaderBlock::default();
        pbf.read_message(&mut header_block);

        header_block.to_header()
    }

    /// Read the next blob
    /// @returns - the next blob if it exists
    fn next(&mut self) -> Option<Vec<u8>> {
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
        // STEP 2: Get blob data
        let compressed_blob_data =
            self.reader.slice(Some(self._offset), Some(self._offset + blob_header.datasize));
        self._offset += blob_header.datasize;
        Some(compressed_blob_data)
    }

    /// Parse all blocks, storing all nodes, ways, and relations into local stores for future consumption
    pub fn parse_blocks(&mut self) {
        if self._parsed {
            return;
        }
        self._offset = 0;
        // skip the header
        self.next();
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

    /// If you are only interested in the nodes, run this function instead
    pub fn parse_node_blocks(
        &mut self,
        cb: &mut dyn FnMut(VectorFeature<OSMMetadata, Properties, ()>),
    ) {
        self._offset = 0;
        // skip the header
        self.next();
        while let Some(b) = self.next() {
            self.parse_node_block(OSMReader::<T, _N, N, _W, W, R>::next_block(b), cb);
        }
    }

    fn parse_node_block(
        &mut self,
        block: PrimitiveBlock,
        cb: &mut dyn FnMut(VectorFeature<OSMMetadata, Properties, ()>),
    ) {
        for group in &block.primitive_groups {
            for node in &group.nodes {
                if !node.is_filterable(&block, self) {
                    cb(node.to_intermediate_feature(&block).to_vector_feature(self.add_bbox));
                }
            }
        }
    }

    /// Returns an iterator over nodes, ways, and relations in sequence.
    pub fn iter(&self) -> OsmReaderIter<'_, T, _N, N, _W, W, R> {
        OsmReaderIter {
            reader: self,
            node_iter: Box::new(self.nodes.iter()),
            way_iter: Box::new(self.ways.iter()),
            relation_iter: Box::new(self.relations.iter()),
        }
    }
}

/// OSM Reader iterator
#[allow(missing_debug_implementations)]
pub struct OsmReaderIter<
    'a,
    T: Reader,
    _N: KVStore<u64, VectorPoint<()>>,
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
    T: Reader,
    _N: KVStore<u64, VectorPoint<()>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> Iterator for OsmReaderIter<'_, T, _N, N, _W, W, R>
{
    type Item = VectorFeature<OSMMetadata, Properties, ()>;

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
    _N: KVStore<u64, VectorPoint<()>>,
    N: KVStore<u64, IntermediateNode>,
    _W: KVStore<u64, WayNodes>,
    W: KVStore<u64, IntermediateWay>,
    R: KVStore<u64, IntermediateRelation>,
> FeatureReader<OSMMetadata, Properties, ()> for OSMReader<T, _N, N, _W, W, R>
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
        self.iter()
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use info::InfoBlock;
    use relation::{IntermediateNodeMember, MemberType};
    use s2json::{
        BBox, BBox3D, Map, VectorBaseGeometry, VectorFeatureType, VectorGeometry,
        VectorGeometryType,
    };

    use super::*;
    use parsers::FileReader;
    use std::{path::PathBuf, vec};

    #[test]
    fn base_case() {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/readers/osm/fixtures/test.pbf");
        let path_str = path.to_str().unwrap();
        let reader = FileReader::from(path_str);

        let mut osm = OSMLocalReader::new(reader, None);
        osm.parse_blocks();

        let header: OSMHeader = osm.get_header();

        assert_eq!(
            header,
            OSMHeader {
                bbox: BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 },
                required_features: vec!["-x�S��/�\rN�H�M�\r3�3S�rI�+N".into()],
                optional_features: vec![],
                writingprogram: None,
                source: None,
                osmosis_replication_timestamp: 0,
                osmosis_replication_sequence_number: 0,
                osmosis_replication_base_url: None
            }
        );

        let features: Vec<_> = osm.iter().collect();

        assert_eq!(features.len(), 8);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(275452090),
                    face: 0.into(),
                    properties: Map::from([
                        ("amenity".into(), "cafe".into()),
                        ("name".into(), "Jam's Sandwich Bar".into()),
                    ]),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10761860000000001,
                            y: 51.5075933,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10761860000000001,
                            bottom: 51.5075933,
                            right: -0.10761860000000001,
                            top: 51.5075933,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -2,
                            time_stamp: Some(1256818475000),
                            changeset: Some(2540257),
                            uid: Some(1697),
                            user_sid: Some("service".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994979),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10833480000000001,
                            y: 51.507406,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10833480000000001,
                            top: 51.507406,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: 2,
                            time_stamp: Some(1250040812000),
                            changeset: Some(1739860),
                            uid: Some(38244),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994980),
                    face: 0.into(),
                    properties: Map::from([("barrier".into(), "gate".into())]),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.1075735,
                            y: 51.507464500000005,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.1075735,
                            bottom: 51.507464500000005,
                            right: -0.1075735,
                            top: 51.507464500000005,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: 1,
                            time_stamp: Some(1234485707000),
                            changeset: Some(-2591627),
                            uid: Some(3516),
                            user_sid: Some("private".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994981),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10750140000000001,
                            y: 51.5074723,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10750140000000001,
                            bottom: 51.5074723,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1224174957000),
                            changeset: Some(-14817),
                            uid: Some(70),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(319408586),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.1080108,
                            y: 51.5074089,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.1080108,
                            bottom: 51.5074089,
                            right: -0.1080108,
                            top: 51.5074089,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1229476722000),
                            changeset: Some(440330),
                            uid: Some(6871),
                            user_sid: Some("name".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(319408587),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10812640000000001,
                            y: 51.5074343,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10812640000000001,
                            bottom: 51.5074343,
                            right: -0.10812640000000001,
                            top: 51.5074343,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1229476722000),
                            changeset: Some(0),
                            uid: Some(6871),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(27776903),
                    face: 0.into(),
                    properties: Map::from([
                        ("access".into(), "private".into()),
                        ("highway".into(), "service".into()),
                        ("name".into(), "üßé€".into())
                    ]),
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.10833480000000001,
                                y: 51.507406,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10812640000000001,
                                y: 51.5074343,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint { x: -0.1080108, y: 51.5074089, z: None, m: None, t: None },
                            VectorPoint {
                                x: -0.1075735,
                                y: 51.507464500000005,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10750140000000001,
                                y: 51.5074723,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Way,
                        info: Some(InfoBlock {
                            version: -2,
                            time_stamp: Some(-621888578000),
                            changeset: Some(684276),
                            uid: Some(35),
                            user_sid: Some("Matt".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(56688),
                    face: 0.into(),
                    properties: Map::from([
                        ("network".into(), "VVW".into()),
                        ("ref".into(), "123".into()),
                        ("route".into(), "bus".into()),
                        ("type".into(), "route".into())
                    ]),
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.10833480000000001,
                                y: 51.507406,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10812640000000001,
                                y: 51.5074343,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint { x: -0.1080108, y: 51.5074089, z: None, m: None, t: None },
                            VectorPoint {
                                x: -0.1075735,
                                y: 51.507464500000005,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10750140000000001,
                                y: 51.5074723,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Relation,
                        info: Some(InfoBlock {
                            version: 14,
                            time_stamp: Some(-647421115000),
                            changeset: Some(-3473819),
                            uid: Some(28095),
                            user_sid: Some("kmvar".into()),
                            visible: true
                        }),
                        nodes: Some(vec![IntermediateNodeMember {
                            role: "".into(),
                            node_id: 319408586
                        }]),
                        relation: None
                    })
                }
            ]
        );

        osm.cleanup();
    }
}
