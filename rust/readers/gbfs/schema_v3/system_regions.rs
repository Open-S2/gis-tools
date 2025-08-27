use crate::readers::GBFSName;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Regions Schema V3.1-RC & V3.0
/// Describes regions for a system that is broken up by geographic or political region.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
pub type GBFSSystemRegionsV3 = GBFSSystemRegionsV30;

/// GBFS Station Information Region
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsRegionV30 {
    /// Identifier of the region.
    pub region_id: String,
    /// Public name for this region.
    pub name: Vec<GBFSName>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsDataV30 {
    /// Stations
    pub regions: Vec<GBFSSystemRegionsRegionV30>,
}

/// # GBFS System Regions Schema V3.0
/// Describes regions for a system that is broken up by geographic or political region.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemRegionsV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Data describing regions for a system.
    pub data: GBFSSystemRegionsDataV30,
}
