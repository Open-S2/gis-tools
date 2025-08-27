use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Station Status Schema V2.3, V2.2, V2.1, OR V2.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
pub type GBFSStationStatusV2 = GBFSStationStatusV23;

/// # GBFS Station Status V2.3
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains station status information.
    pub data: GBFSStationStatusDataV21,
}

/// # GBFS Station Status V2.2
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains station status information.
    pub data: GBFSStationStatusDataV21,
}

/// GBFS Station Status Vehicle Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusVehicleTypeV21 {
    /// ID of the vehicle type
    pub vehicle_type_id: String,
    /// Number of bikes available
    pub count: u64,
}

/// GBFS Station Status Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusStationV21 {
    /// ID of the station
    pub station_id: String,
    /// Number of bikes available
    pub num_bikes_available: u64,
    /// Number of docks available
    pub vehicle_types_available: Option<Vec<GBFSStationStatusVehicleTypeV21>>,
    /// Number of bikes disabled
    pub num_bikes_disabled: Option<u64>,
    /// Number of docks available
    pub num_docks_available: Option<u64>,
    /// Number of docks disabled
    pub num_docks_disabled: Option<u64>,
    /// Whether the station is installed
    pub is_installed: bool,
    /// Whether the station is renting
    pub is_renting: bool,
    /// Whether the station is returning
    pub is_returning: bool,
    /// Last time the data in the feed was updated in POSIX time.
    pub last_reported: u64,
    /// Number of docks available
    pub vehicle_docks_available: Option<Vec<GBFSStationStatusVehicleTypeV21>>,
}

/// GBFS Station Status Data V2.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusDataV21 {
    /// Contains station status information.
    pub stations: Vec<GBFSStationStatusStationV21>,
}

/// # GBFS Station Status V2.1
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains station status information.
    pub data: GBFSStationStatusDataV21,
}

/// GBFS Station Status - Station V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusStationV20 {
    /// ID of the station
    pub station_id: String,
    /// Number of bikes available
    pub num_bikes_available: u64,
    /// Number of bikes disabled
    pub num_bikes_disabled: Option<u64>,
    /// Number of docks available
    pub num_docks_available: Option<u64>,
    /// Number of docks disabled
    pub num_docks_disabled: Option<u64>,
    /// Whether the station is installed
    pub is_installed: bool,
    /// Whether the station is renting
    pub is_renting: bool,
    /// Whether the station is returning
    pub is_returning: bool,
    /// Last time the data in the feed was updated in POSIX time.
    pub last_reported: u64,
}

/// GBFS Station Status Data V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusDataV20 {
    /// Contains station status information.
    pub stations: Vec<GBFSStationStatusStationV20>,
}

/// # GBFS Station Status V2.0
/// Describes the capacity and rental availability of the station.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationStatusV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Contains station status information.
    pub data: GBFSStationStatusDataV20,
}
