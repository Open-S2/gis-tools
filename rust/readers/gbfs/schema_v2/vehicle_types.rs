use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Vehicle Types V2.3, V2.2, V2.1, OR V2.0
/// Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
pub type GBFSVehicleTypesV2 = GBFSVehicleTypesV23;

/// Vehicle air quality certificate.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesEcoLabelV23 {
    /// Country code following the ISO 3166-1 alpha-2 notation.
    pub country_code: String,
    /// Name of the eco label.
    pub eco_sticker: String,
}

/// An object where each key defines vehicle assets.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesVehicleAssetsV23 {
    /// A fully qualified URL pointing to the location of a graphic icon file
    /// that MAY be used to represent this vehicle type on maps and in other applications.
    pub icon_url: String,
    /// A fully qualified URL pointing to the location of a graphic icon file to
    /// be used to represent this vehicle type when in dark mode.
    pub icon_url_dark: Option<String>,
    /// Date that indicates the last time any included vehicle icon images were modified or updated.
    pub icon_last_modified: String,
}

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeFormFacatorV23 {
    /// Bicycle
    #[default]
    #[serde(rename = "bicycle")]
    Bicycle,
    /// Cargo Bicycle
    #[serde(rename = "cargo_bicycle")]
    CargoBicycle,
    /// Car
    #[serde(rename = "car")]
    Car,
    /// Moped
    #[serde(rename = "moped")]
    Moped,
    /// Scooter Standing
    #[serde(rename = "scooter_standing")]
    ScooterStanding,
    /// Scooter Seated
    #[serde(rename = "scooter_seated")]
    ScooterSeated,
    /// Other
    #[serde(rename = "other")]
    Other,
    /// Scooter
    #[serde(rename = "scooter")]
    Scooter,
}

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypePropulsionTypeV23 {
    /// Human
    #[default]
    #[serde(rename = "human")]
    Human,
    /// Electric Assist
    #[serde(rename = "electric_assist")]
    ElectricAssist,
    /// Electric
    #[serde(rename = "electric")]
    Electric,
    /// Combustion
    #[serde(rename = "combustion")]
    Combustion,
    /// Combustion Diesel
    #[serde(rename = "combustion_diesel")]
    CombustionDiesel,
    /// Hybrid
    #[serde(rename = "hybrid")]
    Hybrid,
    /// Plug In Hybrid
    #[serde(rename = "plug_in_hybrid")]
    PlugInHybrid,
    /// Hydrogen Fuel Cell
    #[serde(rename = "hydrogen_fuel_cell")]
    HydrogenFuelCell,
}

/// The conditions for returning the vehicle at the end of the trip.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeReturnConstraintV23 {
    /// Free Floating
    #[default]
    #[serde(rename = "free_floating")]
    FreeFloating,
    /// Roundtrip Station
    #[serde(rename = "roundtrip_station")]
    RoundtripStation,
    /// Any Station
    #[serde(rename = "any_station")]
    AnyStation,
    /// Hybrid
    #[serde(rename = "hybrid")]
    Hybrid,
}

/// Description of accessories available in the vehicle.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeVehicleAccessoriesV23 {
    /// Air Conditioning
    #[default]
    #[serde(rename = "air_conditioning")]
    AirConditioning,
    /// Automatic
    #[serde(rename = "automatic")]
    Automatic,
    /// Manual
    #[serde(rename = "manual")]
    Manual,
    /// Convertible
    #[serde(rename = "convertible")]
    Convertible,
    /// Cruise Control
    #[serde(rename = "cruise_control")]
    CruiseControl,
    /// Doors 2
    #[serde(rename = "doors_2")]
    Doors2,
    /// Doors 3
    #[serde(rename = "doors_3")]
    Doors3,
    /// Doors 4
    #[serde(rename = "doors_4")]
    Doors4,
    /// Doors 5
    #[serde(rename = "doors_5")]
    Doors5,
    /// Navigation
    #[serde(rename = "navigation")]
    Navigation,
}

/// Vehicle types available in the system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeV23 {
    /// Unique identifier of a vehicle type.
    pub vehicle_type_id: String,
    /// The vehicle's general form factor.
    pub form_factor: GBFSVehicleTypeFormFacatorV23,
    /// The primary propulsion type of the vehicle.
    pub propulsion_type: GBFSVehicleTypePropulsionTypeV23,
    /// The number of riders (driver included) the vehicle can legally accommodate.
    /// Minimum: 0.
    pub rider_capacity: Option<u64>,
    /// Cargo volume available in the vehicle, expressed in liters.
    /// Minimum: 0.
    pub cargo_volume_capacity: Option<u64>,
    /// The capacity of the vehicle cargo space (excluding passengers), expressed in kilograms.
    /// Minimum: 0.
    pub cargo_load_capacity: Option<f64>,
    /// The furthest distance in meters that the vehicle can travel without recharging or refueling
    /// when it has the maximum amount of energy potential.
    /// Minimum: 0.
    pub max_range_meters: Option<f64>,
    /// The public name of this vehicle type.
    pub name: Option<String>,
    /// Description of accessories available in the vehicle.
    pub vehicle_accessories: Option<Vec<GBFSVehicleTypeVehicleAccessoriesV23>>,
    /// Maximum quantity of CO2, in grams, emitted per kilometer, according to the WLTP.
    /// Minimum: 0.
    #[serde(rename = "g_CO2_km")]
    pub g_co2_km: Option<f64>,
    /// URL to an image that would assist the user in identifying the vehicle.
    /// JPEG or PNG.
    pub vehicle_image: Option<String>,
    /// The name of the vehicle manufacturer.
    pub make: Option<String>,
    /// The name of the vehicle model.
    pub model: Option<String>,
    /// The color of the vehicle.
    pub color: Option<String>,
    /// Number of wheels this vehicle type has.
    /// Minimum: 0.
    pub wheel_count: Option<u64>,
    /// The maximum speed in kilometers per hour this vehicle is permitted to reach in accordance
    /// with local permit and regulations.
    /// Minimum: 0.
    pub max_permitted_speed: Option<f64>,
    /// The rated power of the motor for this vehicle type in watts.
    /// Minimum: 0.
    pub rated_power: Option<f64>,
    /// Maximum time in minutes that a vehicle can be reserved before a rental begins.
    /// Minimum: 0.
    pub default_reserve_time: Option<f64>,
    /// The conditions for returning the vehicle at the end of the trip.
    pub return_constraint: Option<GBFSVehicleTypeReturnConstraintV23>,
    /// A plan_id as defined in system_pricing_plans.json.
    pub default_pricing_plan_id: Option<String>,
    /// Array of all pricing plan IDs as defined in system_pricing_plans.json.
    pub pricing_plan_ids: Option<Vec<String>>,
    /// Vehicle air quality certificate.
    pub eco_label: Option<Vec<GBFSVehicleTypesEcoLabelV23>>,
    /// An object where each key defines vehicle assets.
    pub vehicle_assets: Option<GBFSVehicleTypesVehicleAssetsV23>,
}

/// GBFS Vehicle Types collection
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeDataV23 {
    /// Array of vehicle types available in the system.
    pub vehicle_types: Vec<GBFSVehicleTypeV23>,
}

/// # GBFS Vehicle Types V2.3
/// Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// Minimum: 1450155600.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    /// (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVehicleTypeDataV23,
}

/// # GBFS Vehicle Types V2.2
/// Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// Minimum: 1450155600.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    /// (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVehicleTypeDataV21,
}

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeFormFacatorV21 {
    /// Bicycle
    #[default]
    #[serde(rename = "bicycle")]
    Bicycle,
    /// Car
    #[serde(rename = "car")]
    Car,
    /// Moped
    #[serde(rename = "moped")]
    Moped,
    /// Other
    #[serde(rename = "other")]
    Other,
    /// Scooter
    #[serde(rename = "scooter")]
    Scooter,
}

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypePropulsionTypeV21 {
    /// Human
    #[default]
    #[serde(rename = "human")]
    Human,
    /// Electric Assist
    #[serde(rename = "electric_assist")]
    ElectricAssist,
    /// Electric
    #[serde(rename = "electric")]
    Electric,
    /// Combustion
    #[serde(rename = "combustion")]
    Combustion,
}

/// GBFS Vehicle Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeV21 {
    /// Unique identifier of a vehicle type.
    pub vehicle_type_id: String,
    /// The vehicle's general form factor.
    pub form_factor: GBFSVehicleTypeFormFacatorV21,
    /// The primary propulsion type of the vehicle.
    pub propulsion_type: GBFSVehicleTypePropulsionTypeV21,
    /// The furthest distance in meters that the vehicle can travel without recharging or refueling
    /// when it has the maximum amount of energy potential.
    /// Minimum: 0.
    pub max_range_meters: Option<f64>,
    /// The public name of this vehicle type.
    pub name: Option<String>,
}

/// GBFS Vehicle Types collection
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeDataV21 {
    /// Array of vehicle types available in the system.
    pub vehicle_types: Vec<GBFSVehicleTypeV21>,
}

/// # GBFS Vehicle Types V2.1
/// Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#vehicle_typesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// Minimum: 1450155600.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again
    /// (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Response data in the form of name:value pairs.
    pub data: GBFSVehicleTypeDataV21,
}
