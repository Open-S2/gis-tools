use crate::readers::{GBFSName, GBFSRentalUri};
use alloc::{string::String, vec::Vec};
use s2json::Geometry;
use serde::{Deserialize, Serialize};

/// # GBFS Station Information Schema V3.1-RC & V3.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
pub type GBFSStationInformationV3 = GBFSStationInformationV30;

/// GBFS Station Information Rental Method
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationRentalMethodV30 {
    /// Key
    #[default]
    #[serde(rename = "key")]
    Key,
    /// Credit Card
    #[serde(rename = "creditcard")]
    CreditCard,
    /// PayPass
    #[serde(rename = "paypass")]
    PayPass,
    /// Apple Pay
    #[serde(rename = "applepay")]
    ApplePay,
    /// Android Pay
    #[serde(rename = "androidpay")]
    AndroidPay,
    /// Google Wallet
    #[serde(rename = "transitcard")]
    TransitCard,
    /// Cash
    #[serde(rename = "accountnumber")]
    AccountNumber,
    /// Phone
    #[serde(rename = "phone")]
    Phone,
}

/// GBFS Station Information Parking Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationParkingTypeV30 {
    /// Parking Lot
    #[default]
    #[serde(rename = "parking_lot")]
    ParkingLot,
    /// Street Parking
    #[serde(rename = "street_parking")]
    StreetParking,
    /// Underground Parking
    #[serde(rename = "underground_parking")]
    UndergroundParking,
    /// Sidewalk Parking
    #[serde(rename = "sidewalk_parking")]
    SidewalkParking,
    /// Other
    #[serde(rename = "other")]
    Other,
}

/// GBFS Station Information Name
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationV3VehicleCapacity {
    /// List of vehicle types.
    pub vehicle_type_ids: Vec<String>,
    /// Number of vehicles.
    pub count: u64,
}

/// Information about a single station.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationV3 {
    /// Identifier of the station.
    pub station_id: String,
    /// Public name of the station.
    pub name: Vec<GBFSName>,
    /// The latitude of the station.
    /// **Minimum**: -90
    /// **Maximum**: 90
    pub lat: f64,
    /// The longitude of the station.
    /// **Minimum**: -180
    /// **Maximum**: 180
    pub lon: f64,
    /// Short name or alternative identifier for the station.
    pub short_name: Option<Vec<GBFSName>>,
    /// Address where the station is located.
    pub address: Option<String>,
    /// Cross street or landmark where the station is located.
    pub cross_street: Option<String>,
    /// Identifier of the region where the station is located.
    pub region_id: Option<String>,
    /// Postal code where the station is located.
    pub post_code: Option<String>,
    /// Hours of operation for the station in OSM opening_hours format.
    pub station_opening_hours: Option<String>,
    /// Payment methods accepted at the station.
    /// **Enum**: ['key', 'creditcard', 'paypass', 'applepay', 'androidpay', 'transitcard', 'accountnumber', 'phone']
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethodV30>>,
    /// Is this station a location with or without physical infrastructure? (added in v2.1-RC)
    pub is_virtual_station: Option<bool>,
    /// A multipolygon describing the area of a virtual station. (added in v2.1-RC)
    pub station_area: Option<Geometry>,
    /// Type of parking station. (added in v2.3)
    /// **Enum**: ['parking_lot', 'street_parking', 'underground_parking', 'sidewalk_parking', 'other']
    pub parking_type: Option<GBFSStationInformationParkingTypeV30>,
    /// Are parking hoops present at this station? (added in v2.3)
    pub parking_hoop: Option<bool>,
    /// Contact phone of the station. (added in v2.3)
    pub contact_phone: Option<String>,
    /// Total docking points installed at the station, both available and unavailable.
    /// **Minimum**: 0
    pub capacity: Option<u64>,
    /// Parking capacity for virtual stations per vehicle type.
    pub vehicle_types_capacity: Option<Vec<GBFSStationV3VehicleCapacity>>,
    /// Docking capacity per vehicle type at the station.
    pub vehicle_docks_capacity: Option<Vec<GBFSStationV3VehicleCapacity>>,
    /// Are valet services provided at the station? (added in v2.1-RC)
    pub is_valet_station: Option<bool>,
    /// Does the station support charging of electric vehicles? (added in v2.3-RC)
    pub is_charging_station: Option<bool>,
    /// Rental URIs for Android, iOS, and web.
    pub rental_uris: Option<GBFSRentalUri>,
}

/// Data object containing station information.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV30Data {
    /// List of stations with their attributes.
    pub stations: Vec<GBFSStationV3>,
}

/// # GBFS Station Information Schema V3.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    /// **Const**: '3.0'
    pub version: String,
    /// Data object containing station information.
    pub data: GBFSStationInformationV30Data,
}
