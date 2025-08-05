use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Auto-Discovery Schema V2.x
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson)
pub type GBFSV2 = GBFSV23;

/// # GBFS V2.3 Feed Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV23Feed {
    /// Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
    ///
    /// **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
    pub name: GBFSV21FeedsName,
    /// URL for the feed.
    /// **Format**: uri
    pub url: String,
}

/// # GBFS V2.3 Data Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSDataV23 {
    /// An array of all of the feeds that are published by the auto-discovery file.
    pub feeds: Vec<GBFSV23Feed>,
}

/// # GBFS Auto-Discovery V2.3
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.3
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: BTreeMap<String, GBFSDataV23>,
}

/// # GBFS Feed Schema V2.2
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV22Feed {
    /// Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
    ///
    /// **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
    pub name: GBFSV21FeedsName,
    /// URL for the feed.
    /// **Format**: uri
    pub url: String,
}

/// # GBFS Data Schema V2.2
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSDataV22 {
    /// An array of all of the feeds that are published by the auto-discovery file.
    pub feeds: Vec<GBFSV22Feed>,
}

/// # GBFS Auto-Discovery V2.2
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.2
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: BTreeMap<String, GBFSDataV22>,
}

/// GBFS Schema V2.1 Feeds Names
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSV21FeedsName {
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
    /// Geofencing Zones
    #[serde(rename = "geofencing_zones")]
    GeofencingZones,
}

/// # GBFS Feed Schema V2.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV21Feed {
    /// Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
    ///
    /// **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
    pub name: GBFSV21FeedsName,
    /// URL for the feed.
    /// **Format**: uri
    pub url: String,
}

/// # GBFS Data Schema V2.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSDataV21 {
    /// An array of all of the feeds that are published by the auto-discovery file.
    pub feeds: Vec<GBFSV21Feed>,
}

/// # GBFS V2.1
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.1
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: BTreeMap<String, GBFSDataV21>,
}

/// GBFS Schema V2.0 Feeds Names
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSV20FeedsName {
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

/// GBFS V2.0 Feed Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV20Feed {
    /// Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
    ///
    /// **Enum**: "gbfs", "gbfs_versions", "system_information", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans"
    pub name: GBFSV20FeedsName,
    /// URL for the feed.
    /// **Format**: uri
    pub url: String,
}

/// GBFS V2.0 Data Schema
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSDataV20 {
    /// An array of all of the feeds that are published by the auto-discovery file.
    pub feeds: Vec<GBFSV20Feed>,
}

/// # GBFS V2.0
/// Auto-discovery file that links to all of the other files published by the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.0
    pub version: String,
    /// Response data in the form of name:value pairs.
    /// An object containing feeds keyed by language code (e.g., "en", "en-US").
    ///
    /// **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
    pub data: BTreeMap<String, GBFSDataV20>,
}
