use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::readers::GBFSVersion;

/// # GBFS Versions Schema V1.1
/// Lists all feed endpoints published according to versions of the GBFS documentation.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfs_versionsjson-added-in-v11)
pub type GBFSVersionsV1 = GBFSVersionsV11;

/// GBFS Versions Data scheme V1.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsDataV11 {
    /// Data containing available feed versions
    pub versions: Vec<GBFSVersion>,
}

/// GBFS Versions Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing available feed versions.
    pub data: GBFSVersionsDataV11,
}
