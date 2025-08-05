use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Versions Schema V2.3, V2.2, V2.1, OR V2.0
/// Lists all feed endpoints published according to versions of the GBFS documentation.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11)
pub type GBFSVersionsV2 = GBFSVersionsV23;

/// GBFS Versions Version scheme
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsVersion {
    /// The semantic version of the feed in the form X.Y.
    /// **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
    pub version: String,
    /// URL of the corresponding gbfs.json endpoint.
    /// **Format**: uri
    pub url: String,
}

/// Response data in the form of name:value pairs.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsData {
    /// Contains one object for each of the available versions of a feed.
    /// The array must be sorted by increasing MAJOR and MINOR version number.
    pub versions: Vec<GBFSVersionsVersion>,
}

/// # GBFS Versions V2.3
/// Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVersionsData,
}

/// # GBFS Versions V2.2
/// Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVersionsData,
}

/// # GBFS Versions V2.1
/// Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVersionsData,
}

/// # GBFS Versions V2.0
/// Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVersionsData,
}
