use super::{
    OSMFilterable, OSMReader, OSMTagFilterType,
    info::{DenseInfo, Info, InfoBlock},
    primitive::{OSMMetadata, PrimitiveBlock},
    relation::{IntermediateRelation, MemberType},
    way::{IntermediateWay, WayNodes},
};
use crate::{data_store::kv::KVStore, parsers::Reader};
use alloc::{string::String, vec, vec::Vec};
use pbf::{ProtoRead, Protobuf};
use s2json::{
    BBox3D, MValue, Properties, VectorFeature, VectorFeatureType, VectorGeometry, VectorPoint,
};
use serde::{Deserialize, Serialize};

/// Intermediate node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntermediateNode {
    /// The node id
    pub id: u64,
    /// The nodes longitude
    pub point: VectorPoint<MValue>,
    /// The key-value pairs of the node
    pub properties: Properties,
    /// The node metadata
    pub info: Option<InfoBlock>,
}
impl IntermediateNode {
    /// Convert the node to a vector feature
    pub fn to_vector_feature(
        &self,
        add_bbox: bool,
    ) -> VectorFeature<OSMMetadata, Properties, MValue> {
        let coordinates = self.point.clone();
        let bbox = if add_bbox { Some(BBox3D::from_point(&coordinates)) } else { None };
        VectorFeature {
            id: Some(self.id),
            face: 0.into(),
            _type: VectorFeatureType::VectorFeature,
            properties: self.properties.clone(),
            geometry: VectorGeometry::new_point(coordinates, bbox),
            metadata: Some(OSMMetadata {
                osm_type: MemberType::Node,
                info: self.info.clone(),
                nodes: None,
                relation: None,
            }),
        }
    }
}

/// Node class
/// contains a single node.
#[derive(Debug, Default, PartialEq)]
pub struct Node {
    /// The node id
    pub id: u64,
    /// The node info block if any. May be a dead info block
    info: Option<Info>,
    /// The nodes latitude
    lat: i64,
    /// The nodes longitude
    lon: i64,
    /// The keys of the node
    keys: Vec<u32>,
    /// The values of the node
    vals: Vec<u32>,
}
impl OSMFilterable for Node {
    fn is_filterable<
        T: Reader,
        _N: KVStore<u64, VectorPoint<MValue>>,
        N: KVStore<u64, IntermediateNode>,
        _W: KVStore<u64, WayNodes>,
        W: KVStore<u64, IntermediateWay>,
        R: KVStore<u64, IntermediateRelation>,
    >(
        &self,
        pb: &PrimitiveBlock,
        reader: &mut OSMReader<T, _N, N, _W, W, R>,
    ) -> bool {
        if reader.skip_nodes || (reader.skip_empty_nodes && self.keys.is_empty()) {
            return true;
        }
        if let Some(tag_filter) = &mut reader.tag_filter {
            for i in 0..self.keys.len() {
                let key_str = pb.get_string(self.keys[i] as usize);
                let val_str = pb.get_string(self.vals[i] as usize);
                if tag_filter.match_found(OSMTagFilterType::Node, key_str, val_str) {
                    return false;
                }
            }
            // if we make it here, we didn't find any matching tags
            return true;
        }

        false
    }
}
impl Node {
    /// Get the lon and lat of the node
    pub fn get_lon_lat(&self, pb: &PrimitiveBlock) -> (f64, f64) {
        let lon_offset = pb.lon_offset as f64;
        let lat_offset = pb.lat_offset as f64;
        let granularity = pb.granularity as f64;
        (
            0.000000001 * (lon_offset + granularity * self.lon as f64),
            0.000000001 * (lat_offset + granularity * self.lat as f64),
        )
    }

    /// Get the properties of the node
    pub fn properties(&self, pb: &PrimitiveBlock) -> Properties {
        pb.tags(&self.keys, &self.vals)
    }

    /// Gain access to the nodes geometry
    pub fn to_vector_geometry(&self, pb: &PrimitiveBlock) -> VectorPoint<MValue> {
        let (lon, lat) = self.get_lon_lat(pb);
        let z = get_elevation(&self.properties(pb));
        VectorPoint::new(lon, lat, z, None)
    }

    /// Converts the way to an intermediate vector feature (way's nodes have not been parsed)
    ///
    /// ## Returns
    /// The way as an intermediate vector feature
    pub fn to_intermediate_feature(&self, pb: &PrimitiveBlock) -> IntermediateNode {
        let point = self.to_vector_geometry(pb);
        IntermediateNode {
            id: self.id,
            point,
            properties: self.properties(pb),
            info: self.info.as_ref().map(|info| info.to_block(pb)),
        }
    }
}
/// Read in the contents of the Node
impl ProtoRead for Node {
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
            8 => self.lat = pb.read_s_varint(),
            9 => self.lon = pb.read_s_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Used to densly represent a sequence of nodes that do not have any tags.
/// We represent these nodes columnwise as five columns: ID's, lats, and
/// lons, all delta coded. When metadata is not omitted,
/// We encode keys & vals for all nodes as a single array of integers
/// containing key-stringid and val-stringid, using a stringid of 0 as a
/// delimiter between nodes.
///
/// `( (<keyid> <valid>)* '0' )*`
#[derive(Debug, Default, PartialEq)]
pub struct DenseNodes {
    ids: Vec<i64>, // DELTA coded
    denseinfo: Option<DenseInfo>,
    lats: Vec<i64>, // DELTA coded
    lons: Vec<i64>, // DELTA coded
    // Special packing of keys and vals into one array. May be empty if all nodes in this block are tagless.
    keys_vals: Vec<i64>,
}
impl DenseNodes {
    /// Access the nodes in this block
    pub fn nodes(&self) -> Vec<Node> {
        let mut res: Vec<Node> = vec![];
        let info_map = self.denseinfo.as_ref().map(|info| info.infos()).unwrap_or_default();
        let mut j = 0;
        let mut cur_id = 0;
        let mut cur_lat = 0;
        let mut cur_lon = 0;
        for i in 0..self.ids.len() {
            let cur_info = info_map.get(i);
            cur_id += self.ids[i];
            cur_lat += self.lats[i];
            cur_lon += self.lons[i];
            let mut keys: Vec<u32> = vec![];
            let mut vals: Vec<u32> = vec![];
            if !self.keys_vals.is_empty() {
                while self.keys_vals[j] != 0 {
                    keys.push(self.keys_vals[j] as u32);
                    vals.push(self.keys_vals[j + 1] as u32);
                    j += 2;
                }
                j += 1;
            }

            res.push(Node {
                id: cur_id as u64,
                keys,
                vals,
                info: cur_info.cloned(),
                lat: cur_lat,
                lon: cur_lon,
            });
        }

        res
    }
}
/// Read in the contents of the Dense Nodes
impl ProtoRead for DenseNodes {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.ids = pb.read_s_packed(),
            5 => {
                let mut info = DenseInfo::default();
                pb.read_message(&mut info);
                self.denseinfo = Some(info);
            }
            8 => self.lats = pb.read_s_packed(),
            9 => self.lons = pb.read_s_packed(),
            10 => self.keys_vals = pb.read_packed(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// returns the altitude assuming it is in meters
fn get_elevation(props: &Properties) -> Option<f64> {
    for s in ["altitude", "ele", "elevation", "height", "depth"] {
        if let Some(elevation) = props.get(s) {
            let val = parse_altitude(&elevation.to_prim().unwrap().to_string().unwrap());
            if let Some(val) = val {
                if s == "depth" {
                    return Some(-val);
                }
                return Some(val);
            }
        }
    }
    None
}

/// returns the altitude assuming it is in meters
fn parse_altitude(alt: &str) -> Option<f64> {
    let digits: String = alt.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { None } else { digits.parse().ok() }
}
