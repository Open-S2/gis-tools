use super::{
    OSMFilterable, OSMReader, OSMTagFilterType,
    info::{Info, InfoBlock},
    node::IntermediateNode,
    primitive::{OSMMetadata, PrimitiveBlock},
    way::{IntermediateWay, WayNodes},
};
use crate::{data_store::kv::KVStore, readers::Reader};
use alloc::{string::String, vec, vec::Vec};
use pbf::{BitCast, ProtoRead, Protobuf};
use s2json::{
    BBox3D, Properties, VectorFeature, VectorFeatureType, VectorGeometry, VectorLineString,
    VectorMultiLineString, VectorMultiPolygon, VectorPoint, VectorPolygon,
};
use serde::{Deserialize, Serialize};

/// An intermediate vector feature where the way nodes haven't been resolved yet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntermediateMember {
    /// A node member
    Node(IntermediateNodeMember),
    /// A way member
    Way(IntermediateWayMember),
}
/// An intermediate vector feature where the way nodes haven't been resolved yet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntermediateNodeMember {
    /// The role of the node relative to the relation
    pub role: String,
    /// The node's id
    pub node_id: u64,
}
/// An intermediate vector feature where the way nodes haven't been resolved yet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntermediateWayMember {
    /// the role of the way relative to the relation
    pub role: String,
    /// The way's id
    pub way_id: u64,
}

/// An intermediate vector feature where the ways and nodes haven't been resolved yet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntermediateRelation {
    /// The relation's id
    pub id: u64,
    /// The relation's properties
    pub properties: Properties,
    /// The relation's members
    pub members: Vec<IntermediateMember>,
    /// The relation's info block
    pub info: Option<InfoBlock>,
}
impl IntermediateRelation {
    /// Convert the node to a vector feature
    pub fn to_vector_feature<_N: KVStore<u64, VectorPoint<()>>, _W: KVStore<u64, WayNodes>>(
        &self,
        node_geometry: &_N,
        way_geometry: &_W,
        add_bbox: bool,
    ) -> Option<VectorFeature<OSMMetadata, Properties, ()>> {
        let mut bbox = BBox3D::default();
        let IntermediateRelation { id, members, properties, info } = &self;
        let i_nodes: Vec<IntermediateNodeMember> = members
            .iter()
            .filter_map(|m| {
                if let IntermediateMember::Node(node) = m { Some(node.clone()) } else { None }
            })
            .collect();
        let mut nodes: Vec<NodeMember> = vec![];
        for IntermediateNodeMember { role, node_id, .. } in &i_nodes {
            let n = node_geometry.get(*node_id);
            if let Some(n) = n {
                nodes.push(NodeMember { id: *node_id, role: role.into(), node: n.clone() });
            }
        }
        let i_ways: Vec<IntermediateWayMember> = members
            .iter()
            .filter_map(
                |m| if let IntermediateMember::Way(way) = m { Some(way.clone()) } else { None },
            )
            .collect();
        let mut ways: Vec<WayMember> = vec![];
        for IntermediateWayMember { role, way_id } in &i_ways {
            let w = way_geometry.get(*way_id);
            if let Some(w) = w {
                let mut mapped_w: VectorLineString<()> = vec![];
                for node_id in w {
                    let n = node_geometry.get(*node_id);
                    if let Some(n) = n {
                        if add_bbox {
                            bbox.extend_from_point(n);
                        }
                        mapped_w.push(n.clone());
                    }
                }
                ways.push(WayMember { id: *way_id, role: role.into(), way: mapped_w });
            }
        }
        let geo = build_geometry(&mut ways);
        geo.as_ref()?;

        let mut relation_geo = geo.unwrap();
        let bbox = if add_bbox { Some(bbox) } else { None };
        let geometry: VectorGeometry<()> = match &mut relation_geo {
            RelationGeometry::Lines(lines) => {
                if lines.len() == 1 {
                    VectorGeometry::new_linestring(core::mem::take(&mut lines[0]), bbox)
                } else {
                    VectorGeometry::new_multilinestring(core::mem::take(lines), bbox)
                }
            }
            RelationGeometry::Area(area) => {
                if area.len() == 1 {
                    VectorGeometry::new_polygon(core::mem::take(&mut area[0]), bbox)
                } else {
                    VectorGeometry::new_multipolygon(core::mem::take(area), bbox)
                }
            }
        };
        Some(VectorFeature {
            id: Some(*id),
            face: 0.into(),
            _type: VectorFeatureType::VectorFeature,
            properties: properties.clone(),
            geometry,
            metadata: Some(OSMMetadata {
                osm_type: MemberType::Relation,
                info: info.clone(),
                nodes: Some(i_nodes),
                relation: None,
            }),
        })
    }
}

/// Member Options. Relations is skipped as it is not supported / has no use.
#[derive(Debug)]
pub enum Member {
    /// Node Member
    Node(NodeMember),
    /// Way Member
    Way(WayMember),
}
/// Node Member
#[derive(Debug)]
pub struct NodeMember {
    /// The node's id
    pub id: u64,
    /// The role of the node relative to the relation
    pub role: String,
    /// The node geometry
    pub node: VectorPoint<()>,
}
/// Way Member
#[derive(Debug)]
pub struct WayMember {
    /// The way's id
    pub id: u64,
    /// The role of the way relative to the relation
    pub role: String,
    /// The way geometry
    pub way: VectorLineString<()>,
}

/// Relation coordinates from ways with information about node relations.
#[derive(Debug)]
pub enum RelationGeometry {
    /// Lines
    Lines(VectorMultiLineString<()>),
    /// Area
    Area(VectorMultiPolygon<()>),
}

/// The expected metadata in the VectorFeature for all types (node, way, relation)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MemberType {
    /// Node type
    Node = 0,
    /// Way type (lines and polygons)
    Way = 1,
    /// Relation type (collection of nodes, ways and relations)
    Relation = 2,
}
impl BitCast for MemberType {
    fn to_u64(&self) -> u64 {
        match self {
            MemberType::Node => 0,
            MemberType::Way => 1,
            MemberType::Relation => 2,
        }
    }
    fn from_u64(value: u64) -> Self {
        match value {
            0 => MemberType::Node,
            1 => MemberType::Way,
            2 => MemberType::Relation,
            _ => panic!("unknown value {}", value),
        }
    }
}

/// Relation class contains a collection of nodes, ways and relations as members.
#[derive(Debug, Default)]
pub struct Relation {
    /// The relation's id
    pub id: u64,
    info: Option<Info>,
    // Parallel arrays
    keys: Vec<u32>,
    vals: Vec<u32>,
    roles_sid: Vec<i32>, /* This should have been defined as uint32 for consistency, but it is now too late to change it */
    memids: Vec<i64>,    // DELTA encoded
    types: Vec<MemberType>,
}
impl Relation {
    /// Get the properties of the node
    pub fn properties(&self, pb: &PrimitiveBlock) -> Properties {
        pb.tags(&self.keys, &self.vals)
    }

    /// Each member can be node, way or relation.
    pub fn members(&self, pb: &PrimitiveBlock) -> Vec<IntermediateMember> {
        let mut res = vec![];
        let mut memid = 0;
        for i in 0..self.memids.len() {
            memid += self.memids[i];
            let role = pb.get_string(self.roles_sid[i] as usize);
            let cur_type = self.types[i];
            if cur_type == MemberType::Node {
                res.push(IntermediateMember::Node(IntermediateNodeMember {
                    role: role.into(),
                    node_id: memid as u64,
                }));
            } else if cur_type == MemberType::Way {
                res.push(IntermediateMember::Way(IntermediateWayMember {
                    role: role.into(),
                    way_id: memid as u64,
                }));
            } else {
                // Relation -> no-op
            }
        }
        res
    }

    /// returns the feature in intermediate format to build later
    pub fn to_intermediate_feature(&self, pb: &PrimitiveBlock) -> Option<IntermediateRelation> {
        let members = self.members(pb);
        if members.is_empty() {
            None
        } else {
            Some(IntermediateRelation {
                id: self.id,
                properties: self.properties(pb),
                members,
                info: self.info.as_ref().map(|info| info.to_block(pb)),
            })
        }
    }

    /// Get the node relation pairs
    pub fn get_node_relation_pairs(members: &[IntermediateMember]) -> Vec<IntermediateNodeMember> {
        let mut res = vec![];
        for member in members {
            if let IntermediateMember::Node(member) = member {
                if member.role == "label" || member.role == "admin_centre" {
                    res.push(member.clone());
                }
            }
        }
        res
    }
}
impl OSMFilterable for Relation {
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
        if reader.skip_relations {
            return true;
        }
        if let Some(tag_filter) = &mut reader.tag_filter {
            for i in 0..self.keys.len() {
                let key_str = pb.get_string(self.keys[i] as usize);
                let val_str = pb.get_string(self.vals[i] as usize);
                if tag_filter.match_found(OSMTagFilterType::Relation, key_str, val_str) {
                    return false;
                }
            }
            // if we make it here, we didn't find any matching tags
            return true;
        }
        false
    }
}
/// Read in the contents of the relation
impl ProtoRead for Relation {
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
            8 => self.roles_sid = pb.read_packed(),
            9 => self.memids = pb.read_s_packed(),
            10 => self.types = pb.read_packed(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Given a group of Members whose type is "way", build a multilinestring or multipolygon Feature.
/// If the ways include an 'outer' or 'inner', then we know its an area, otherwise its a line.
fn build_geometry(ways: &mut [WayMember]) -> Option<RelationGeometry> {
    // prep variables
    let mut polygons: VectorMultiPolygon<()> = vec![];
    let mut current_polygon: VectorPolygon<()> = vec![];
    let mut current_ring: VectorLineString<()> = vec![];
    let is_area = ways.iter().any(|m| m.role == "outer" || m.role == "inner");

    // prepare step: members are stored out of order
    sort_members(ways);

    for member in ways {
        // Using "isClockwise", depending on whether the ring is outer or inner,
        // we may need to reverse the order of the points. Every time we find the
        // first and last point are the same, close out the ring, add it to the current
        // polygon, and start a new ring. if the current polygon is NOT empty, we store
        // it in the polygons list and start a new one before adding the completed ring.
        // NOTE: Due to the nature of OSM data, it is possible that resulting ring is reversed.
        // Check against the current ring to see if the way needs to be edited.
        //
        // grab the geometry from the member and store in current ring, checking current rings order
        if current_ring.is_empty() {
            current_ring.extend_from_slice(&member.way);
        } else {
            current_ring.extend_from_slice(&member.way[1..]);
        }

        // if current rings first and last point are the same, close out the ring
        if current_ring.first() == current_ring.last() {
            // add the ring to the current polygon. If member role is outer and
            // current_polygon already has data, we need to store the current poly and
            // start a new polygon.
            // If the member role is inner, we can add the ring to the
            // current polygon.
            if member.role == "outer" && !current_polygon.is_empty() {
                polygons.push(current_polygon);
                current_polygon = vec![];
            }
            current_polygon.push(current_ring);
            current_ring = vec![];
        }
    }

    // Last step is to build:
    // flush ring if it exists
    if !current_ring.is_empty() {
        current_polygon.push(current_ring);
    }
    //   if (!is_area) return { type: 0, coordinates: current_polygon };
    if !is_area {
        return Some(RelationGeometry::Lines(current_polygon));
    }
    // flush the current polygon if it exists
    if !current_polygon.is_empty() {
        polygons.push(current_polygon);
    }
    // grab the polys and return a feature
    Some(RelationGeometry::Area(polygons))
}

/// osm throws relation members out of order, so we need to not only sort them
/// but also check if the first and last points of each way follow the same direction.
/// @param members - the ways to be sorted
fn sort_members(members: &mut [WayMember]) {
    let len = members.len();
    if len < 3 {
        return;
    }
    for i in 0..len - 1 {
        let cur_first_point = &members[i].way[0];
        let cur_last_point = &members[i].way[members[i].way.len() - 1];
        // if current way is already self closing break
        if cur_first_point == cur_last_point {
            break;
        }
        for j in i + 1..len {
            let next_first_point = &members[j].way[0];
            let next_last_point = &members[j].way[members[j].way.len() - 1];
            // if we find a match between any of the points, swap the member positions
            // if cur_first_point == next_first_point or cur_last_point == next_last_point
            // swap the order
            let equal_first = cur_first_point == next_first_point;
            let equal_last = cur_last_point == next_last_point;
            let equal_first_last = cur_first_point == next_last_point;
            let equal_last_first = cur_last_point == next_first_point;
            if equal_first || equal_last || equal_first_last || equal_last_first {
                if equal_first {
                    members[i].way.reverse();
                } else if equal_last {
                    members[j].way.reverse();
                } else if equal_first_last {
                    members[i].way.reverse();
                    members[j].way.reverse();
                }
                // we want to move the found member to be next to the current member
                if i + 1 != j {
                    members.swap(i + 1, j);
                }
                break;
            }
        }
    }
}
