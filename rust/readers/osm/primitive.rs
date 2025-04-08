use crate::data_structures::HasLayer;

use super::{
    info::InfoBlock,
    node::{DenseNodes, Node},
    relation::{IntermediateNodeMember, MemberType, Relation},
    way::Way,
};
use alloc::{string::String, vec::Vec};
use pbf::{ProtoRead, Protobuf};
use s2json::Properties;

/// A metadata struct for relations
#[derive(Debug, Clone, PartialEq)]
pub struct OSMMetadataRelation {
    /// The role of the relation related to the VectorFeature
    pub role: String,
    /// The properties of the relation
    pub properties: Properties,
}

/// The expected metadata in the VectorFeature for all types (node, way, relation)
#[derive(Debug, Clone, PartialEq)]
pub struct OSMMetadata {
    /// The type of the VectorFeature
    pub osm_type: MemberType,
    /// the info block describing the data storage history
    pub info: Option<InfoBlock>,
    /// the nodes that make up the VectorFeature
    pub nodes: Option<Vec<IntermediateNodeMember>>,
    /// if this feature is part of a relation, this relation will describe the other components
    pub relation: Option<OSMMetadataRelation>,
}
impl HasLayer for OSMMetadata {
    fn get_layer(&self) -> Option<String> {
        None
    }
}

/// NOTE: currently relations are stored, but we don't wait for the Block to store all relations
/// before we start testing primtiveHandle against the data. This is a problem because
/// relations reference eachother at times, and we need to be able to resolve those references
/// before we can run relationHandle against the data. This isn't an important issue since
/// in practice, all relations that reference eachother often produce garbage or unusable data.
/// But it would be *nice* to fix this. Morbidly enough, the "BEST" solution is to treat relations
/// like we do nodes and ways since relations could possibly reference eachother outside their own block.
/// From a practical standpoint, I can't see this being worth the effort or memory/time cost.
#[derive(Debug)]
pub struct PrimitiveBlock {
    stringtable: StringTable,
    /// Primitive groups are smaller collections of the nodes, ways and relations
    pub primitive_groups: Vec<PrimitiveGroup>,
    /// Granularity, units of nanodegrees, used to store coordinates in this block.
    pub granularity: i32,
    /// Offset value between the output coordinates and the granularity grid in units of nanodegrees.
    pub lat_offset: i64,
    /// Offset value between the output coordinates and the granularity grid in units of nanodegrees.
    pub lon_offset: i64,
    /// Granularity of dates, normally represented in units of milliseconds since the 1970 epoch.
    pub date_granularity: i32,
}
impl Default for PrimitiveBlock {
    fn default() -> Self {
        Self {
            stringtable: StringTable::default(),
            primitive_groups: Vec::new(),
            granularity: 100,
            lat_offset: 0,
            lon_offset: 0,
            date_granularity: 1000,
        }
    }
}
impl PrimitiveBlock {
    /// Get a string from the string table
    pub fn get_string(&self, index: usize) -> &str {
        self.stringtable.get(index)
    }

    /// Get a record of strings from the string table
    pub fn tags(&self, keys: &[u32], values: &[u32]) -> Properties {
        let mut res = Properties::default();
        // zip the keys and values and put them in the map
        for (key, value) in keys.iter().zip(values.iter()) {
            let key = self.get_string(*key as usize);
            let value = self.get_string(*value as usize);
            res.insert(key.into(), value.into());
        }
        res
    }
}
/// Read in the contents of the primitive block
impl ProtoRead for PrimitiveBlock {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => pb.read_message(&mut self.stringtable),
            2 => {
                let mut group = PrimitiveGroup::default();
                pb.read_message(&mut group);
                self.primitive_groups.push(group);
            }
            17 => self.granularity = pb.read_varint(),
            18 => self.date_granularity = pb.read_varint(),
            19 => self.lat_offset = pb.read_varint(),
            20 => self.lon_offset = pb.read_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Group of OSMPrimitives. All primitives in a group must be the same type.
#[derive(Debug, Default)]
pub struct PrimitiveGroup {
    /// Nodes (points)
    pub nodes: Vec<Node>,
    /// Ways (lines and polygons)
    pub ways: Vec<Way>,
    /// Relations - collections of nodes, ways and relations
    pub relations: Vec<Relation>,
    // pub changesets: Vec<ChangeSet>,
}
/// Read in the contents of the primitive block
impl ProtoRead for PrimitiveGroup {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut node = Node::default();
                pb.read_message(&mut node);
                self.nodes.push(node);
            }
            2 => {
                let mut dense_nodes = DenseNodes::default();
                pb.read_message(&mut dense_nodes);
                self.nodes.extend(dense_nodes.nodes());
            }
            3 => {
                let mut way = Way::default();
                pb.read_message(&mut way);
                self.ways.push(way);
            }
            4 => {
                let mut relation = Relation::default();
                pb.read_message(&mut relation);
                self.relations.push(relation);
            }
            // 5 => {
            //     let mut changeset = ChangeSet::default();
            //     pb.read_message(&mut changeset);
            //     self.changesets.push(changeset);
            // }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// String table, contains the common strings in each block.
/// Note that we reserve index '0' as a delimiter, so the entry at that
/// index in the table is ALWAYS blank and unused.
/// NOTE: OSM isn't safe and allows " inside of strings, so we have to replace them with '
/// NOTE: OSM isn't safe and allows \ at the end of strings, so we have to remove them so it can be properly parsed.
#[derive(Debug, Default)]
pub struct StringTable {
    strings: Vec<String>,
    empty_string: String,
}
impl StringTable {
    /// Get a string from the string table
    pub fn get(&self, index: usize) -> &str {
        self.strings.get(index).unwrap_or(&self.empty_string)
    }
}
/// Read in the contents of the header block
impl ProtoRead for StringTable {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.strings.push(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

// /// This is kept for backwards compatibility but not used anywhere.
// #[derive(Debug, Default)]
// pub struct ChangeSet {
//     /// The id of the changeset
//     pub id: i64,
// }
// /// Read in the contents of the header block
// impl ProtoRead for ChangeSet {
//     fn read(&mut self, tag: u64, pb: &mut Protobuf) {
//         match tag {
//             1 => self.id = pb.read_varint(),
//             _ => panic!("unknown tag {}", tag),
//         }
//     }
// }
