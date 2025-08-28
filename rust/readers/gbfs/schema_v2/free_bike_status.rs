use crate::readers::{GBFSRentalUri, gbfs_bool_or_int};
use alloc::{string::String, vec::Vec};
use s2json::MValue;
use serde::{Deserialize, Serialize};

/// # Free Bike Status Schema V2.3, V2.2, V2.1, OR V2.0
/// Describes the vehicles that are available for rent.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#free_bike_statusjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#free_bike_statusjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#free_bike_statusjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#free_bike_statusjson)
pub type GBFSFreeBikeStatusV2 = GBFSFreeBikeStatusV23;

/// Vehicle Equipment Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSFreeBikeStatusVehicleEquipmentV2 {
    /// Child Seat A
    #[serde(rename = "child_seat_a")]
    #[default]
    ChildSeatA,
    /// Child Seat B
    #[serde(rename = "child_seat_b")]
    ChildSeatB,
    /// Child Seat C
    #[serde(rename = "child_seat_c")]
    ChildSeatC,
    /// Winter Tires
    #[serde(rename = "winter_tires")]
    WinterTires,
    /// Snow Chains
    #[serde(rename = "snow_chains")]
    SnowChains,
}

/// Free Bike Status Bike Schema V2.3 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSFreeBikeStatusBikeV23 {
    /// Rotating (as of v2.0) identifier of a vehicle.
    pub bike_id: String,
    /// The latitude of the vehicle.
    /// **Minimum**: -90
    /// **Maximum**: 90
    pub lat: Option<f64>,
    /// The longitude of the vehicle.
    /// **Minimum**: -180
    /// **Maximum**: 180
    pub lon: Option<f64>,
    /// Is the vehicle currently reserved?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the vehicle currently disabled (broken)?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Contains rental URIs for Android, iOS, and web (added in v1.1).
    pub rental_uris: Option<GBFSRentalUri>,
    /// The vehicle_type_id of this vehicle (added in v2.1-RC).
    pub vehicle_type_id: Option<String>,
    /// The last time this vehicle reported its status to the operator's backend in POSIX time (added in v2.1-RC).
    /// **Minimum**: 1450155600
    pub last_reported: Option<u64>,
    /// The furthest distance in meters that the vehicle can travel without recharging or refueling with the vehicle's current charge or fuel (added in v2.1-RC).
    /// **Minimum**: 0
    pub current_range_meters: Option<f64>,
    /// This value represents the current percentage, expressed from 0 to 1, of fuel or battery power remaining in the vehicle. Added in v2.3-RC.
    /// **Minimum**: 0
    /// **Maximum**: 1
    pub current_fuel_percent: Option<f64>,
    /// Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).
    pub station_id: Option<String>,
    /// The station_id of the station this vehicle must be returned to (added in v2.3-RC).
    pub home_station_id: Option<String>,
    /// The plan_id of the pricing plan this vehicle is eligible for (added in v2.2).
    pub pricing_plan_id: Option<String>,
    // /// List of vehicle equipment provided by the operator in addition to the accessories already provided in the vehicle. Added in v2.3.
    // pub vehicle_equipment: Option<Vec<GBFSFreeBikeStatusVehicleEquipmentV2>>,
    /// The date and time when any rental of the vehicle must be completed. Added in v2.3.
    /// **Pattern**: `^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$`
    pub available_until: Option<String>,
}

/// Contains the list of bikes published by the auto-discovery file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusDataV23 {
    /// An array of all bikes available for rent.
    pub bikes: Vec<GBFSFreeBikeStatusBikeV23>,
}

/// # Free Bike Status V2.3
/// Describes the vehicles that are available for rent (as of v2.1-RC2).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#free_bike_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.3
    pub version: String,
    /// Contains the list of bikes published by the auto-discovery file.
    pub data: GBFSFreeBikeStatusDataV23,
}

/// Free Bike Status Bike Schema V2.2 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusBikeV22 {
    /// Rotating (as of v2.0) identifier of a vehicle.
    pub bike_id: String,
    /// The latitude of the vehicle.
    /// **Minimum**: -90
    /// **Maximum**: 90
    pub lat: Option<f64>,
    /// The longitude of the vehicle.
    /// **Minimum**: -180
    /// **Maximum**: 180
    pub lon: Option<f64>,
    /// Is the vehicle currently reserved?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the vehicle currently disabled (broken)?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Contains rental URIs for Android, iOS, and web (added in v1.1).
    pub rental_uris: Option<GBFSRentalUri>,
    /// The vehicle_type_id of this vehicle (added in v2.1-RC).
    pub vehicle_type_id: Option<String>,
    /// The last time this vehicle reported its status to the operator's backend in POSIX time (added in v2.1-RC).
    /// **Minimum**: 1450155600
    pub last_reported: Option<u64>,
    /// The furthest distance in meters that the vehicle can travel without recharging or refueling with the vehicle's current charge or fuel (added in v2.1-RC).
    /// **Minimum**: 0
    pub current_range_meters: Option<f64>,
    /// Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).
    pub station_id: Option<String>,
    /// The plan_id of the pricing plan this vehicle is eligible for (added in v2.1-RC2).
    pub pricing_plan_id: Option<String>,
}

/// Contains the list of bikes published by the auto-discovery file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusDataV22 {
    /// An array of all bikes available for rent.
    pub bikes: Vec<GBFSFreeBikeStatusBikeV22>,
}

/// # Free Bike Status V2.2
/// Describes the vehicles that are available for rent (as of v2.1-RC2).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#free_bike_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.2
    pub version: String,
    /// Contains the list of bikes published by the auto-discovery file.
    pub data: GBFSFreeBikeStatusDataV22,
}

/// Free Bike Status Bike Schema V2.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusBikeV21 {
    /// Rotating (as of v2.0) identifier of a vehicle.
    pub bike_id: String,
    /// The latitude of the vehicle.
    /// **Minimum**: -90
    /// **Maximum**: 90
    pub lat: Option<f64>,
    /// The longitude of the vehicle.
    /// **Minimum**: -180
    /// **Maximum**: 180
    pub lon: Option<f64>,
    /// Is the vehicle currently reserved?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the vehicle currently disabled (broken)?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Contains rental URIs for Android, iOS, and web (added in v1.1).
    pub rental_uris: Option<GBFSRentalUri>,
    /// The vehicle_type_id of this vehicle (added in v2.1-RC).
    pub vehicle_type_id: Option<String>,
    /// The last time this vehicle reported its status to the operator's backend in POSIX time (added in v2.1-RC).
    /// **Minimum**: 1450155600
    pub last_reported: Option<u64>,
    /// The furthest distance in meters that the vehicle can travel without recharging or refueling (added in v2.1-RC).
    /// **Minimum**: 0
    pub current_range_meters: Option<f64>,
    /// Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).
    pub station_id: Option<String>,
}

/// Contains the list of bikes published by the auto-discovery file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusDataV21 {
    /// An array of all bikes available for rent.
    pub bikes: Vec<GBFSFreeBikeStatusBikeV21>,
}

/// # Free Bike Status V2.1
/// Describes the vehicles that are available for rent (as of v2.1-RC2).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#free_bike_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.1
    pub version: String,
    /// Contains the list of bikes published by the auto-discovery file.
    pub data: GBFSFreeBikeStatusDataV21,
}

/// Free Bike Status Bike Schema V2.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusBikeV20 {
    /// Rotating (as of v2.0) identifier of a vehicle.
    pub bike_id: String,
    /// The latitude of the vehicle.
    /// **Minimum**: -90
    /// **Maximum**: 90
    pub lat: f64,
    /// The longitude of the vehicle.
    /// **Minimum**: -180
    /// **Maximum**: 180
    pub lon: f64,
    /// Is the vehicle currently reserved?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the vehicle currently disabled (broken)?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Contains rental URIs for Android, iOS, and web (added in v1.1).
    pub rental_uris: Option<GBFSRentalUri>,
}

/// Contains the list of bikes published by the auto-discovery file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusDataV20 {
    /// An array of all bikes available for rent.
    pub bikes: Vec<GBFSFreeBikeStatusBikeV20>,
}

/// # Free Bike Status V2.0
/// Describes the vehicles that are not at a station and are available for rent.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#free_bike_statusjson)
pub struct GBFSFreeBikeStatusV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
    /// **Const**: 2.0
    pub version: String,
    /// Contains the list of bikes published by the auto-discovery file.
    pub data: GBFSFreeBikeStatusDataV20,
}
