use crate::readers::gbfs_bool_or_int;
use alloc::{string::String, vec::Vec};
use s2json::{FeatureCollection, MValue, ValuePrimitive};
use serde::{Deserialize, Serialize};

/// # GBFS Geofencing Zones Schema V2.3, V2.2, V2.1, OR V2.0
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson)
pub type GBFSGeofencingZonesV2 = GBFSGeofencingZonesV23;

/// GBFS V3: Restrictions that apply within the area of the polygon.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, ValuePrimitive)]
pub struct GBFSGeofencingZonesV2PropertiesRule {
    // /// Array of vehicle type IDs for which these restrictions apply.
    // pub vehicle_type_id: Option<Vec<String>>,
    ///  Is the undocked ride allowed to start and end in this zone?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub ride_allowed: bool,
    /// Is the ride allowed to travel through this zone?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub ride_through_allowed: bool,
    /// Maximum speed allowed, in kilometers per hour.
    /// **Minimum**: 0
    pub maximum_speed_kph: Option<f64>,
    /// Vehicle MUST be parked at stations defined in station_information.json within this geofence zone.
    pub station_parking: Option<bool>,
}

/// Properties of a geofencing zone
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSGeofencingZonesV2Properties {
    /// Public name of the geofencing zone.
    pub name: String,
    /// Start time of the geofencing zone in RFC3339 format.
    /// **format** date-time
    pub start: Option<String>,
    /// End time of the geofencing zone in RFC3339 format.
    /// **format** date-time
    pub end: Option<String>,
    /// Array of rules defining restrictions within the geofence.
    pub rules: Option<Vec<GBFSGeofencingZonesV2PropertiesRule>>,
}

/// Container for GeoJSON FeatureCollection of geofencing zones
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV2Data {
    /// GeoJSON FeatureCollection of geofencing zones
    pub geofencing_zones: FeatureCollection<(), GBFSGeofencingZonesV2Properties, MValue>,
}

/// # GBFS Geofencing Zones V2.3
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains geofencing information for the system.
    pub data: GBFSGeofencingZonesV2Data,
}

/// # GBFS Geofencing Zones V2.2
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains geofencing information for the system.
    pub data: GBFSGeofencingZonesV2Data,
}

/// # GBFS Geofencing Zones V2.1
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains geofencing information for the system.
    pub data: GBFSGeofencingZonesV2Data,
}
