use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Schema V3.1-RC OR GBFS Schema V3.0
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
pub type GBFSV3 = GBFSV30;

/// GBFS Schema V3.0 Feeds Names
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSV30FeedsName {
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
    /// Vehicle Types
    #[serde(rename = "vehicle_types")]
    VehicleTypes,
    /// Station Information
    #[serde(rename = "station_information")]
    StationInformation,
    /// Station Status
    #[serde(rename = "station_status")]
    StationStatus,
    /// Vehicle Status
    #[serde(rename = "vehicle_status")]
    VehicleStatus,
    /// System Alerts
    #[serde(rename = "system_alerts")]
    SystemAlerts,
    /// System Regions
    #[serde(rename = "system_regions")]
    SystemRegions,
    /// System Pricing Plans
    #[serde(rename = "system_pricing_plans")]
    SystemPricingPlans,
    /// Geofencing Zones
    #[serde(rename = "geofencing_zones")]
    GeofencingZones,
}

/// GBFS V3.0 Data Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV30Feed {
    /// Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
    /// **Enum**: ['gbfs', 'gbfs_versions', 'system_information', 'vehicle_types', 'station_information', 'station_status', 'vehicle_status', 'system_alerts', 'system_regions', 'system_pricing_plans', 'geofencing_zones']
    pub name: GBFSV30FeedsName,
    /// URL to the feed file.
    /// **Format**: url
    pub url: String,
}

/// GBFS V3.0 Data Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSDataV30 {
    /// An array of all of the feeds that are published by the auto-discovery file.
    pub feeds: Vec<GBFSV30Feed>,
}

/// # GBFS Schema V3.0
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 3.0
    pub version: String,
    /// Contains the data for feeds published by the auto-discovery file.
    pub data: GBFSDataV30,
}
