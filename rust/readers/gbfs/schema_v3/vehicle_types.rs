use crate::readers::GBFSName;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Vehicle Types Schema V3.1-RC & V3.0
/// Describes the types of vehicles that the system operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
pub type GBFSVehicleTypesV3 = GBFSVehicleTypesV30;

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypePropulsionTypeV30 {
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

/// The vehicle's general form factor.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeFormFacatorV30 {
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

/// Description of accessories available in the vehicle.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeVehicleAccessoriesV30 {
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

/// The conditions for returning the vehicle at the end of the trip.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSVehicleTypeReturnConstraintV30 {
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

/// GBFS Vehicle Types Eco Label
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeEcoLabelV30 {
    /// The vehicle's country code.
    pub country_code: String,
    /// The vehicle's eco sticker.
    pub eco_sticker: String,
}

/// GBFS Vehicle Types Asset V3.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeAssetV30 {
    /// The URL of the vehicle's icon.
    pub icon_url: String,
    /// The URL of the vehicle's dark icon.
    pub icon_url_dark: Option<String>,
    /// Date that indicates the last time any included vehicle icon images were modified or updated.
    pub icon_last_modified: String,
}

/// # GBFS Vehicle Types Schema V3.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeV30 {
    /// Unique identifier of a vehicle type.
    pub vehicle_type_id: String,
    /// The vehicle's general form factor.
    pub form_factor: GBFSVehicleTypeFormFacatorV30,
    /// The primary propulsion type of the vehicle.
    pub propulsion_type: GBFSVehicleTypePropulsionTypeV30,
    /// The furthest distance the vehicle can travel without recharging or refueling.
    pub max_range_meters: Option<u64>,
    /// Public name of the vehicle type.
    pub name: Option<Vec<GBFSName>>,
    /// Description of accessories available in the vehicle.
    pub vehicle_accessories: Option<Vec<GBFSVehicleTypeVehicleAccessoriesV30>>,
    /// Maximum CO2 emissions per kilometer, in grams.
    #[serde(rename = "g_CO2_km")]
    pub g_co2_km: Option<f64>,
    /// URL to an image of the vehicle.
    pub vehicle_image: Option<String>,
    /// Manufacturer of the vehicle.
    pub make: Option<Vec<GBFSName>>,
    /// Model of the vehicle.
    pub model: Option<Vec<GBFSName>>,
    /// The vehicle's color.
    pub color: Option<String>,
    /// Customer-readable description of the vehicle type.
    pub description: Option<Vec<GBFSName>>,
    /// Number of wheels on the vehicle.
    pub wheel_count: Option<u64>,
    /// The maximum speed permitted for the vehicle.
    pub max_permitted_speed: Option<f64>,
    /// The rated motor power in watts.
    pub rated_power: Option<f64>,
    /// Default reserve time for the vehicle, in minutes.
    pub default_reserve_time: Option<f64>,
    /// Return conditions for the vehicle.
    pub return_constraint: Option<GBFSVehicleTypeReturnConstraintV30>,
    /// Information about the vehicle's assets.
    pub vehicle_assets: Option<GBFSVehicleTypeAssetV30>,
    /// Default pricing plan ID for this vehicle type.
    pub default_pricing_plan_id: Option<String>,
    /// Array of all pricing plan IDs available for this vehicle type.
    pub pricing_plan_ids: Option<Vec<String>>,
    /// Rider capacity of the vehicle.
    pub rider_capacity: Option<u64>,
    /// Cargo volume capacity in liters.
    pub cargo_volume_capacity: Option<u64>,
    /// Cargo load capacity in kilograms.
    pub cargo_load_capacity: Option<u64>,
    /// Eco labels for the vehicle.
    pub eco_labels: Option<Vec<GBFSVehicleTypeEcoLabelV30>>,
}

/// GBFS Vehicle Types collection
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypeDataV30 {
    /// Array of vehicle types available in the system.
    pub vehicle_types: Vec<GBFSVehicleTypeV30>,
}

/// # GBFS Vehicle Types Schema V3.0
/// Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSVehicleTypesV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Vehicle type data.
    pub data: GBFSVehicleTypeDataV30,
}
