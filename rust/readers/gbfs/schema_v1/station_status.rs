use crate::readers::gbfs_bool_or_int;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Station Status Schema V1.1 OR GBFS Station Status Schema V1.0
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_statusjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_statusjson)
pub type GBFSStationStatusV1 = GBFSStationStatusV11;

/// GBFS Station Status Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusStationV11 {
    /// The ID of the station
    pub station_id: String,
    /// The number of bikes available
    pub num_bikes_available: u64,
    /// The number of bikes disabled
    pub num_bikes_disabled: Option<u64>,
    /// The number of docks available
    pub num_docks_available: u64,
    /// The number of docks disabled
    pub num_docks_disabled: Option<u64>,
    /// Whether the station is installed
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_installed: bool,
    /// Whether the station is renting
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_renting: bool,
    /// Whether the station is returning
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_returning: bool,
    /// The last time the station was reported
    pub last_reported: u64,
}

/// GBFS Station Status Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusDataV11 {
    /// Data containing an array of station statuses.
    pub stations: Vec<GBFSStationStatusStationV11>,
}

/// GBFS Station Status Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing an array of station statuses.
    pub data: GBFSStationStatusDataV11,
}

/// GBFS Station Status Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing an array of station statuses.
    pub data: GBFSStationStatusDataV11,
}
