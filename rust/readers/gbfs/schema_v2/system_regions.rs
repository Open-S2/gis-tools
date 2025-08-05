use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Regions V2.3, V2.2, V2.1, OR V2.0
/// Describes regions for a system that is broken up by geographic or political region.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson)
pub type GBFSSystemRegionsV2 = GBFSSystemRegionsV23;

/// # GBFS System Regions Schema V2.3
///
/// Describes regions for a system that is broken up by geographic or political region.
///
/// **Links**:
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV23 {
    /// Last time the data in the feed was updated in POSIX time
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    pub ttl: u64,
    /// GBFS version number
    pub version: String,
    /// Region data
    pub data: GBFSSystemRegionsRegionV20,
}

/// # GBFS System Regions Schema V2.2
///
/// Describes regions for a system that is broken up by geographic or political region.
///
/// **Links**:
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV22 {
    /// Last time the data in the feed was updated in POSIX time
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    pub ttl: u64,
    /// GBFS version number
    pub version: String,
    /// Region data
    pub data: GBFSSystemRegionsRegionV20,
}

/// # GBFS System Regions Schema V2.1
///
/// Describes regions for a system that is broken up by geographic or political region.
///
/// **Links**:
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV21 {
    /// Last time the data in the feed was updated in POSIX time
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    pub ttl: u64,
    /// GBFS version number
    pub version: String,
    /// Region data
    pub data: GBFSSystemRegionsDataV20,
}

/// GBFS Station Information Region
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsRegionV20 {
    /// Identifier of the region
    pub region_id: String,
    /// Public name for the region
    pub name: String,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsDataV20 {
    /// Stations
    pub regions: Vec<GBFSSystemRegionsRegionV20>,
}

/// # GBFS System Regions Schema V2.0
///
/// Describes regions for a system that is broken up by geographic or political region.
///
/// **Links**:
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV20 {
    /// Last time the data in the feed was updated in POSIX time
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    pub ttl: u64,
    /// GBFS version number
    pub version: String,
    /// Stations
    pub data: GBFSSystemRegionsDataV20,
}
