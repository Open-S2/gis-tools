use super::primitive::PrimitiveBlock;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use pbf::{ProtoRead, Protobuf};
use serde::{Deserialize, Serialize};

/// Info Block - decoded into an object
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfoBlock {
    /// The version of the object. Default is -1
    pub version: i32,
    /// (millisec_stamp = time_stamp*dateGranularity.)
    pub time_stamp: Option<i64>,
    /// The changeset id
    pub changeset: Option<i64>,
    /// The uid
    pub uid: Option<i32>,
    /// String IDs for usernames. Pull string from local Primitive Block
    pub user_sid: Option<String>,
    /// The visible flag is used to store history information. It indicates that
    /// the current object version has been created by a delete operation on the
    /// OSM API.
    /// When a writer sets this flag, it MUST add a required_features tag with
    /// value "HistoricalInformation" to the HeaderBlock.
    /// If this flag is not available for some object it MUST be assumed to be
    /// true if the file has the required_features tag "HistoricalInformation"
    /// set.
    pub visible: bool, // default true
}

/// Optional metadata that may be included into each primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    /// The version of the object. Default is -1
    pub version: i32,
    /// (millisec_stamp = time_stamp*dateGranularity.)
    pub time_stamp: Option<i64>,
    /// The changeset id
    pub changeset: Option<i64>,
    /// The uid
    pub uid: Option<i32>,
    /// String IDs for usernames. Pull string from local Primitive Block
    pub user_sid: Option<u32>,
    /// The visible flag is used to store history information. It indicates that
    /// the current object version has been created by a delete operation on the
    /// OSM API.
    /// When a writer sets this flag, it MUST add a required_features tag with
    /// value "HistoricalInformation" to the HeaderBlock.
    /// If this flag is not available for some object it MUST be assumed to be
    /// true if the file has the required_features tag "HistoricalInformation"
    /// set.
    pub visible: bool, // default true
}
impl Default for Info {
    fn default() -> Self {
        Self {
            version: -1,
            time_stamp: None,
            changeset: None,
            uid: None,
            user_sid: None,
            visible: true,
        }
    }
}
impl Info {
    /// Converts the Info object to an InfoBlock with the injected correct time stamp and user name
    pub fn to_block(&self, pb: &PrimitiveBlock) -> InfoBlock {
        InfoBlock {
            version: self.version,
            time_stamp: self.time_stamp.map(|ts| ts * pb.date_granularity as i64),
            changeset: self.changeset,
            uid: self.uid,
            user_sid: self.user_sid.map(|sid| pb.get_string(sid as usize).to_string()),
            visible: self.visible,
        }
    }
}
/// Read in the contents of the Info block
impl ProtoRead for Info {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.version = pb.read_s_varint(),
            2 => self.time_stamp = Some(pb.read_s_varint()),
            3 => self.changeset = Some(pb.read_s_varint()),
            4 => self.uid = Some(pb.read_s_varint()),
            5 => self.user_sid = Some(pb.read_varint()),
            6 => self.visible = pb.read_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// DenseInfo
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DenseInfo {
    version: Vec<i32>,
    time_stamp: Vec<i64>, // DELTA coded (millisec_stamp = time_stamp*dateGranularity.)
    changeset: Vec<i64>,
    uid: Vec<i32>,      // DELTA coded
    user_sid: Vec<u32>, // String IDs for usernames. DELTA coded
    // The visible flag is used to store history information. It indicates that
    // the current object version has been created by a delete operation on the
    // OSM API.
    // When a writer sets this flag, it MUST add a required_features tag with
    // value "HistoricalInformation" to the HeaderBlock.
    // If this flag is not available for some object it MUST be assumed to be
    // true if the file has the required_features tag "HistoricalInformation"
    // set.
    visible: Vec<bool>,
}
/// Read in the contents of the Dense Info block
impl ProtoRead for DenseInfo {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.version = pb.read_s_packed(),
            2 => self.time_stamp = pb.read_s_packed(),
            3 => self.changeset = pb.read_s_packed(),
            4 => self.uid = pb.read_s_packed(),
            5 => self.user_sid = pb.read_packed(),
            6 => self.visible = pb.read_packed(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
impl DenseInfo {
    /// Get the info objects
    pub fn infos(&self) -> Vec<Info> {
        let mut res: Vec<Info> = vec![];
        let mut cur_time_stamp = 0;
        let mut cur_uid = 0;
        for i in 0..self.version.len() {
            cur_time_stamp += self.time_stamp[i];
            cur_uid += self.uid[i];
            res.push(Info {
                version: self.version[i],
                time_stamp: Some(cur_time_stamp),
                changeset: Some(self.changeset[i]),
                uid: Some(cur_uid),
                user_sid: self.user_sid.get(i).copied(),
                visible: self.visible.get(i).copied().unwrap_or(true),
            });
        }
        res
    }
}
