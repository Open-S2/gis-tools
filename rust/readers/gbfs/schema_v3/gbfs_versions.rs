use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Versions Schema V3.1-RC & V3.0
/// Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfs_versionsjson)
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson)
pub type GBFSVersionsV3 = GBFSVersionsV30;

/// GBFS Versions Version scheme
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsVersion30 {
    /// The semantic version of the feed in the form X.Y.
    /// **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
    pub version: String,
    /// URL of the corresponding gbfs.json endpoint.
    /// **Format**: uri
    pub url: String,
}

/// Response data in the form of name:value pairs.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsData30 {
    /// Contains one object for each of the available versions of a feed.
    /// The array must be sorted by increasing MAJOR and MINOR version number.
    pub versions: Vec<GBFSVersionsVersion30>,
}

/// # GBFS Versions Schema V3.0
/// Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVersionsV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 3.0
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVersionsData30,
}
