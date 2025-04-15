use super::{
    OSMFilterable, OSMReader, OSMTagFilterType,
    info::{Info, InfoBlock},
    node::IntermediateNode,
    primitive::{OSMMetadata, PrimitiveBlock},
    relation::{IntermediateRelation, MemberType},
};
use crate::{data_store::kv::KVStore, readers::Reader};
use alloc::{vec, vec::Vec};
use pbf::{ProtoRead, Protobuf};
use s2json::{
    BBox3D, Properties, VectorFeature, VectorFeatureType, VectorGeometry, VectorLineString,
    VectorPoint,
};
use serde::{Deserialize, Serialize};

/// Linebased node reference store
pub type WayNodes = Vec<u64>;

/// An intermediate vector feature where the way nodes haven't been resolved yet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntermediateWay {
    /// The way's ID
    pub id: u64,
    /// The way's properties
    pub properties: Properties,
    /// Optional metadata
    pub info: Option<InfoBlock>,
    /// The way's nodes IDs to be resolved
    pub way_nodes: WayNodes,
    /// Whether the way is an area
    pub is_area: bool,
}
impl IntermediateWay {
    /// Convert the node to a vector feature
    pub fn to_vector_feature<_N: KVStore<u64, VectorPoint<()>>>(
        &self,
        node_geometry: &_N,
        add_bbox: bool,
    ) -> VectorFeature<OSMMetadata, Properties, ()> {
        let IntermediateWay { id, is_area, way_nodes, properties, info } = &self;
        let mut bbox = BBox3D::default();
        // build line
        let mut vector_line: VectorLineString<()> = vec![];
        for way_node in way_nodes {
            let node = node_geometry.get(*way_node);
            if let Some(node) = node {
                if add_bbox {
                    bbox.extend_from_point(node)
                }
                vector_line.push(node.clone());
            }
        }
        // build geometry
        let bbox = if add_bbox { Some(bbox) } else { None };
        let geometry = match is_area {
            true => VectorGeometry::new_polygon(vec![vector_line], bbox),
            false => VectorGeometry::new_linestring(vector_line, bbox),
        };

        VectorFeature {
            id: Some(*id),
            face: 0.into(),
            _type: VectorFeatureType::VectorFeature,
            properties: properties.clone(),
            geometry,
            metadata: Some(OSMMetadata {
                osm_type: MemberType::Way,
                info: info.clone(),
                nodes: None,
                relation: None,
            }),
        }
    }
}

/// Way Class
#[derive(Debug, Default)]
pub struct Way {
    /// The way's ID
    pub id: u64,
    info: Option<Info>,
    // Parallel arrays
    keys: Vec<u32>,
    vals: Vec<u32>,
    // DELTA coded
    refs: Vec<i64>,
    // Optional infield lat-lon
    // NOTE: I'm not going to bother implementing this, I've never seen it used.
    //   lats: Vec<i32>, // optional DELTA coded
    //   lons: Vec<i32>, // optional DELTA coded */
}
impl Way {
    /// Get the properties of the node
    pub fn properties(&self, pb: &PrimitiveBlock) -> Properties {
        pb.tags(&self.keys, &self.vals)
    }

    /// Checks if the way is an area based on it's key-value pairs
    pub fn is_area<
        T: Reader,
        _N: KVStore<u64, VectorPoint<()>>,
        N: KVStore<u64, IntermediateNode>,
        _W: KVStore<u64, WayNodes>,
        W: KVStore<u64, IntermediateWay>,
        R: KVStore<u64, IntermediateRelation>,
    >(
        &self,
        pb: &PrimitiveBlock,
        reader: &mut OSMReader<T, _N, N, _W, W, R>,
    ) -> bool {
        if (reader.upgrade_ways_to_areas
            && self.refs.len() >= 4
            && self.refs[0] == self.refs[self.refs.len() - 1])
            || self.has_key_value(pb, "area", Some("yes"))
        {
            return true;
        }
        false
    }

    /// Checks if the way has a key-value pair (value optional)
    pub fn has_key_value(&self, pb: &PrimitiveBlock, key: &str, val: Option<&str>) -> bool {
        for i in 0..self.keys.len() {
            if pb.get_string(self.keys[i] as usize) == key {
                match val {
                    None => return true,
                    Some(v) => {
                        if pb.get_string(self.vals[i] as usize) == v {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Access the way's node IDs associated with this way
    pub fn node_refs(&self) -> WayNodes {
        let mut res = vec![];
        let mut _ref = 0;
        // for (let i = 0; i < this.#refs.length; i++) {
        for i in 0..self.refs.len() {
            _ref += self.refs[i];
            res.push(_ref as u64);
        }
        res
    }

    /// Converts the way to an intermediate vector feature (way's nodes have not been parsed)
    /// @returns - the way as an intermediate vector feature
    pub fn to_intermediate_feature<
        T: Reader,
        _N: KVStore<u64, VectorPoint<()>>,
        N: KVStore<u64, IntermediateNode>,
        _W: KVStore<u64, WayNodes>,
        W: KVStore<u64, IntermediateWay>,
        R: KVStore<u64, IntermediateRelation>,
    >(
        &self,
        pb: &PrimitiveBlock,
        reader: &mut OSMReader<T, _N, N, _W, W, R>,
    ) -> Option<IntermediateWay> {
        let is_area = self.is_area(pb, reader);
        let way_nodes = self.node_refs();
        if way_nodes.len() < 2 {
            None
        } else {
            Some(IntermediateWay {
                id: self.id,
                is_area,
                properties: self.properties(pb),
                way_nodes,
                info: self.info.as_ref().map(|info| info.to_block(pb)),
            })
        }
    }
}
impl OSMFilterable for Way {
    fn is_filterable<
        T: Reader,
        _N: KVStore<u64, VectorPoint<()>>,
        N: KVStore<u64, IntermediateNode>,
        _W: KVStore<u64, WayNodes>,
        W: KVStore<u64, IntermediateWay>,
        R: KVStore<u64, IntermediateRelation>,
    >(
        &self,
        pb: &PrimitiveBlock,
        reader: &mut OSMReader<T, _N, N, _W, W, R>,
    ) -> bool {
        if reader.skip_ways {
            return true;
        }
        if let Some(tag_filter) = &mut reader.tag_filter {
            for i in 0..self.keys.len() {
                let key_str = pb.get_string(self.keys[i] as usize);
                let val_str = pb.get_string(self.vals[i] as usize);
                if tag_filter.match_found(OSMTagFilterType::Way, key_str, val_str) {
                    return false;
                }
            }
            // if we make it here, we didn't find any matching tags
            return true;
        }
        false
    }
}
/// Read in the contents of the way
impl ProtoRead for Way {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.id = pb.read_varint(),
            2 => self.keys = pb.read_packed(),
            3 => self.vals = pb.read_packed(),
            4 => {
                let mut info = Info::default();
                pb.read_message(&mut info);
                self.info = Some(info);
            }
            8 => self.refs = pb.read_s_packed(),
            // skip, not used.
            9 | 10 => (),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
