use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Regions Schema V1.1 OR GBFS System Regions Schema V1.0
/// Describes regions for a system that is broken up by geographic or political region.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_regionsjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_regionsjson)
pub type GBFSSystemRegionsV1 = GBFSSystemRegionsV11;

/// GBFS System Region
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionV1 {
    /// Region ID
    pub region_id: String,
    /// Region name
    pub name: String,
}

/// GBFS System Regions Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsDataV1 {
    /// List of regions
    pub regions: Vec<GBFSSystemRegionV1>,
}

/// GBFS System Regions Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data describing regions for a system.
    pub data: GBFSSystemRegionsDataV1,
}

/// GBFS System Regions Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data describing regions for a system.
    pub data: GBFSSystemRegionsDataV1,
}
