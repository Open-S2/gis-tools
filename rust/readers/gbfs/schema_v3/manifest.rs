use crate::readers::GBFSVersion;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Manifest Schema V3.1-RC & V3.0
/// An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
/// this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
pub type GBFSManifestV3 = GBFSManifestV30;

/// GBFS Manifest Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSManifestV30Data {
    /// Array of datasets containing system IDs and versions.
    pub datasets: Vec<GBFSVersion>,
}

/// # GBFS Manifest Schema V3.0
/// An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
/// this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSManifestV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: '3.0'
    pub version: String,
    /// Data object containing the list of datasets.
    pub data: GBFSManifestV30Data,
}
