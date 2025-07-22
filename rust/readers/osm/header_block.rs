use alloc::{string::String, vec::Vec};
use pbf::{ProtoRead, Protobuf};
use s2json::BBox;

/// OSM Header Block
#[derive(Debug, PartialEq, Default)]
pub struct OSMHeader {
    /// The bounding box field in the OSM header
    pub bbox: BBox,
    /// The required features field in the OSM header
    pub required_features: Vec<String>,
    /// The optional features field in the OSM header
    pub optional_features: Vec<String>,
    /// The writingprogram field in the OSM header
    pub writingprogram: Option<String>,
    /// The source field in the OSM header
    pub source: Option<String>,
    /// Tags that allow continuing an Osmosis replication
    /// Replication timestamp, expressed in seconds since the epoch,
    /// otherwise the same value as in the "timestamp=..." field
    /// in the state.txt file used by Osmosis.
    pub osmosis_replication_timestamp: i64,
    /// Replication sequence number (sequenceNumber in state.txt).
    pub osmosis_replication_sequence_number: i64,
    /// Replication base URL (from Osmosis' configuration.txt file).
    pub osmosis_replication_base_url: Option<String>,
}
impl From<&HeaderBlock> for OSMHeader {
    fn from(block: &HeaderBlock) -> Self {
        OSMHeader {
            bbox: block.bbox.to_bbox(),
            required_features: block.required_features.clone(),
            optional_features: block.optional_features.clone(),
            writingprogram: block.writingprogram.clone(),
            source: block.source.clone(),
            osmosis_replication_timestamp: block.osmosis_replication_timestamp,
            osmosis_replication_sequence_number: block.osmosis_replication_sequence_number,
            osmosis_replication_base_url: block.osmosis_replication_base_url.clone(),
        }
    }
}

/// The OSM Header Block
/// A block containing OSM header information that helps guide the parser
/// of the OSM data how to interpret the data.
#[derive(Debug, Default, PartialEq)]
pub struct HeaderBlock {
    bbox: HeaderBBox,
    // Additional tags to aid in parsing this dataset
    required_features: Vec<String>,
    optional_features: Vec<String>,
    writingprogram: Option<String>,
    source: Option<String>,
    // Tags that allow continuing an Osmosis replication
    // Replication timestamp, expressed in seconds since the epoch,
    // otherwise the same value as in the "timestamp=..." field
    // in the state.txt file used by Osmosis.
    osmosis_replication_timestamp: i64,
    // Replication sequence number (sequenceNumber in state.txt).
    osmosis_replication_sequence_number: i64,
    // Replication base URL (from Osmosis' configuration.txt file).
    osmosis_replication_base_url: Option<String>,
}
impl HeaderBlock {
    /// Read the header block's contents into an object
    pub fn to_header(&self) -> OSMHeader {
        self.into()
    }
}
/// Read in the contents of the header block
impl ProtoRead for HeaderBlock {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => pb.read_message(&mut self.bbox),
            4 => self.required_features.push(pb.read_string()),
            5 => self.optional_features.push(pb.read_string()),
            16 => self.writingprogram = Some(pb.read_string()),
            17 => self.source = Some(pb.read_string()),
            32 => self.osmosis_replication_timestamp = pb.read_varint(),
            33 => self.osmosis_replication_sequence_number = pb.read_varint(),
            34 => self.osmosis_replication_base_url = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// The bounding box field in the OSM header. BBOX, as used in the OSM
/// header. Units are always in nanodegrees -- they do not obey
/// granularity rules.
#[derive(Debug, Default, PartialEq)]
pub struct HeaderBBox {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}
impl HeaderBBox {
    /// Returns the bounding box as a [left, bottom, right, top] array
    fn to_bbox(&self) -> BBox {
        BBox::new(
            self.left as f64 / 1_000_000_000.0,
            self.bottom as f64 / 1_000_000_000.0,
            self.right as f64 / 1_000_000_000.0,
            self.top as f64 / 1_000_000_000.0,
        )
    }
}
/// Read in the contents of the bounding box
impl ProtoRead for HeaderBBox {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.left = pb.read_s_varint(),
            2 => self.right = pb.read_s_varint(),
            3 => self.top = pb.read_s_varint(),
            4 => self.bottom = pb.read_s_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
