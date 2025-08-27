use crate::readers::gbfs_bool_or_int;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Station Status Schema V3.1-RC & V3.0
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
pub type GBFSStationStatusV3 = GBFSStationStatusV30;

/// Vehicle Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusStationV30VehicleType {
    /// Identifier of the vehicle type.
    pub vehicle_type_id: String,
    /// Number of vehicles of this type available at the station.
    pub count: u64,
}

/// GBFS Station Status Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusStationV30 {
    /// Identifier of the station.
    pub station_id: String,
    /// Number of vehicles physically available for rental at the station.
    /// **Minimum**: 0
    pub num_vehicles_available: u64,
    /// Details of vehicles available by type at the station.
    pub vehicle_types_available: Option<Vec<GBFSStationStatusStationV30VehicleType>>,
    /// Number of disabled vehicles at the station.
    /// **Minimum**: 0
    pub num_vehicles_disabled: Option<u64>,
    /// Number of functional docks physically at the station.
    /// **Minimum**: 0
    pub num_docks_available: Option<u64>,
    /// Number of disabled but empty docks at the station.
    /// **Minimum**: 0
    pub num_docks_disabled: Option<u64>,
    /// Indicates whether the station is installed on the street.
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_installed: bool,
    /// Indicates whether the station is currently renting vehicles.
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_renting: bool,
    /// Indicates whether the station is accepting vehicle returns.
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_returning: bool,
    /// Last reported status time in RFC3339 format.
    /// **Format**: date-time
    pub last_reported: String,

    /// Details of docks available by vehicle type at the station.
    pub vehicle_docks_available: Option<Vec<GBFSStationStatusStationV30VehicleType>>,
}

/// GBFS Station Status Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusDataV30 {
    /// Data containing an array of station statuses.
    pub stations: Vec<GBFSStationStatusStationV30>,
}

/// # GBFS Station Status Schema V3.0
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    /// **Const**: '3.0'
    pub version: String,
    /// Data object containing station statuses.
    pub data: GBFSStationStatusDataV30,
}
