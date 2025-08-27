use crate::readers::{GBFSRentalUri, gbfs_bool_or_int};
use alloc::{string::String, vec::Vec};
use s2json::MValue;
use serde::{Deserialize, Serialize};

/// # GBFS Vehicle Status Schema V3.1-RC & V3.0
/// Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_statusjson)
pub type GBFSVehicleStatusV3 = GBFSVehicleStatusV30;

/// Vehicle Equipment Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSFreeBikeStatusVehicleEquipmentV3 {
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

/// GBFS Vehicle V3
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSVehicleV3 {
    /// Rotating (as of v2.0) identifier of a vehicle.
    pub vehicle_id: String,
    /// The latitude of the vehicle.
    /// **Range**: [-90, 90]
    pub lat: Option<f64>,
    /// The longitude of the vehicle.
    /// **Range**: [-180, 180]
    pub lon: Option<f64>,
    /// Is the vehicle currently reserved?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the vehicle currently disabled (broken)?
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Contains rental URIs for Android, iOS, and web.
    pub rental_uris: Option<GBFSRentalUri>,
    /// The vehicle_type_id of this vehicle (added in v2.1-RC).
    pub vehicle_type_id: Option<String>,
    /// The last time this vehicle reported its status to the operator's backend.
    /// **Format**: date-time
    pub last_reported: Option<String>,
    /// The furthest distance in meters the vehicle can travel without recharging or refueling.
    /// **Minimum**: 0
    pub current_range_meters: Option<f64>,
    /// Current percentage of fuel or battery power remaining in the vehicle.
    /// **Range**: [0, 1]
    pub current_fuel_percent: Option<f64>,
    /// Identifier referencing the station_id if the vehicle is currently at a station.
    pub station_id: Option<String>,
    /// The station_id of the station this vehicle must be returned to.
    pub home_station_id: Option<String>,
    /// The plan_id of the pricing plan this vehicle is eligible for.
    pub pricing_plan_id: Option<String>,
    // /// List of vehicle equipment provided by the operator.
    // /// **Enum**: ['child_seat_a', 'child_seat_b', 'child_seat_c', 'winter_tires', 'snow_chains']
    // pub vehicle_equipment: Option<Vec<GBFSFreeBikeStatusVehicleEquipmentV3>>,
    /// The date and time when any rental of the vehicle must be completed.
    /// **Pattern**: `^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$`
    pub available_until: Option<String>,
}

/// GBFS Vehicle Status Data V3.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleStatusDataV30 {
    /// List of vehicles
    pub vehicles: Vec<GBFSVehicleV3>,
}

/// # GBFS Vehicle Status Schema V3.0
/// Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleStatusV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Vehicle data containing information on available vehicles for rent.
    pub data: GBFSVehicleStatusDataV30,
}
