use crate::readers::{GBFSName, gbfs_bool_or_int};
use alloc::{string::String, vec::Vec};
use s2json::{FeatureCollection, MValue, ValuePrimitive};
use serde::{Deserialize, Serialize};

/// # GBFS Geofencing Zones Schema V3.1-RC & V3.0
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#geofencing_zonesjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson)
pub type GBFSGeofencingZonesV3 = GBFSGeofencingZonesV30;

/// GBFS V3: Restrictions that apply within the area of the polygon.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, ValuePrimitive)]
pub struct GBFSGeofencingZonesV3PropertiesRule {
    // /// Array of vehicle type IDs for which these restrictions apply.
    // pub vehicle_type_ids: Option<Vec<String>>,
    /// Is the ride allowed to start in this zone?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub ride_start_allowed: bool,
    /// Is the ride allowed to end in this zone?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub ride_end_allowed: bool,
    /// Is the ride allowed to travel through this zone?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub ride_through_allowed: bool,
    /// Maximum speed allowed, in kilometers per hour.
    /// **minimum** 0
    pub maximum_speed_kph: Option<f64>,
    /// Vehicle MUST be parked at stations defined in station_information.json within this zone.
    pub station_parking: Option<bool>,
}

/// Properties of a geofencing zone
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSGeofencingZonesV3Properties {
    /// Public name of the geofencing zone.
    pub name: Vec<GBFSName>,
    /// Start time of the geofencing zone in RFC3339 format.
    /// **format** date-time
    pub start: Option<String>,
    /// End time of the geofencing zone in RFC3339 format.
    /// **format** date-time
    pub end: Option<String>,
    /// Array of rules defining restrictions within the geofence.
    pub rules: Option<Vec<GBFSGeofencingZonesV3PropertiesRule>>,
}

/// Container for GeoJSON FeatureCollection of geofencing zones
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV30Data {
    /// GeoJSON FeatureCollection of geofencing zones
    pub geofencing_zones: FeatureCollection<(), GBFSGeofencingZonesV3Properties, MValue>,
    /// Array of global rules defining restrictions that apply by default.
    pub global_rules: Vec<GBFSGeofencingZonesV3PropertiesRule>,
}

/// # GBFS Geofencing Zones Schema V3.0
/// Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSGeofencingZonesV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: '3.0'
    pub version: String,
    /// Array that contains geofencing information for the system.
    pub data: GBFSGeofencingZonesV30Data,
}
