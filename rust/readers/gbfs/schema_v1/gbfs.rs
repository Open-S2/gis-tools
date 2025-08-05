use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Schema V1.1 OR GBFS Schema V1.0
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfsjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#gbfsjson)
pub type GBFSV1 = GBFSV11;

/// GBFS Schema V1.1 Feeds Names
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSV11FeedsName {
    /// GBFS
    #[default]
    #[serde(rename = "gbfs")]
    Gbfs,
    /// GBFS Versions
    #[serde(rename = "gbfs_versions")]
    GbfsVersions,
    /// System Information
    #[serde(rename = "system_information")]
    SystemInformation,
    /// Station Information
    #[serde(rename = "station_information")]
    StationInformation,
    /// Station Status
    #[serde(rename = "station_status")]
    StationStatus,
    /// Free Bike Status
    #[serde(rename = "free_bike_status")]
    FreeBikeStatus,
    /// System Hours
    #[serde(rename = "system_hours")]
    SystemHours,
    /// System Alerts
    #[serde(rename = "system_alerts")]
    SystemAlerts,
    /// System Calendar
    #[serde(rename = "system_calendar")]
    SystemCalendar,
    /// System Regions
    #[serde(rename = "system_regions")]
    SystemRegions,
    /// System Pricing Plans
    #[serde(rename = "system_pricing_plans")]
    SystemPricingPlans,
}

/// GBFS Schema V1.1 Feeds
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV11Feeds {
    /// Name of the feed
    pub name: GBFSV11FeedsName,
    /// URL of the feed
    pub url: String,
}

/// GBFS Schema V1.1 Feeds Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV11FeedsData {
    /// List of feeds
    pub feeds: Vec<GBFSV11Feeds>,
}

/// GBFS Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: BTreeMap<String, GBFSV11FeedsData>,
}

/// GBFS Schema V1.0 Feeds
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV10Feeds {
    /// Name of the feed
    pub name: String,
    /// URL of the feed
    pub url: String,
}

/// GBFS Schema V1.0 Feeds Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV10FeedsData {
    /// List of feeds
    feeds: Vec<GBFSV10Feeds>,
}

/// GBFS Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Response data in the form of name:value pairs.
    pub data: BTreeMap<String, GBFSV10FeedsData>,
}
