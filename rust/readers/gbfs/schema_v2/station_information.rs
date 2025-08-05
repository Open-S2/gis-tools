use crate::readers::GBFSRentalUri;
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use s2json::MultiPolygonGeometry;
use serde::{Deserialize, Serialize};

/// # GBFS Station Information Schema V2.3, V2.2, V2.1, OR V2.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson)
pub type GBFSStationInformationV2 = GBFSStationInformationV23;

/// GBFS Station Information Parking Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationParkingTypeV23 {
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

/// GBFS Station Information Station V2.3
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationStationV23 {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethodV21>>,
    /// Is Virtual Station
    pub is_virtual_station: Option<bool>,
    /// Station Area
    pub station_area: Option<MultiPolygonGeometry>,
    /// Parking Type
    pub parking_type: Option<GBFSStationInformationParkingTypeV23>,
    /// Parking Hoop
    pub parking_hoop: Option<bool>,
    /// Contact Phone
    pub contact_phone: Option<String>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Vehicle Capacity
    pub vehicle_capacity: Option<BTreeMap<String, f64>>,
    /// Is Valet Station
    pub is_valet_station: Option<bool>,
    /// Is Charging Station
    pub is_charging_station: Option<bool>,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
    /// Vehicle Type Capacity
    pub vehicle_type_capacity: Option<BTreeMap<String, f64>>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV23 {
    /// Stations
    pub stations: Vec<GBFSStationInformationStationV23>,
}

/// # GBFS Station Information V2.3
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains station information for the system.
    pub data: GBFSStationInformationDataV23,
}

/// GBFS Station Information Station V2.2
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationStationV22 {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethodV21>>,
    /// Is Virtual Station
    pub is_virtual_station: Option<bool>,
    /// Station Area
    pub station_area: Option<MultiPolygonGeometry>,
    /// Parking Type
    pub capacity: Option<u64>,
    /// Vehicle Capacity
    pub vehicle_capacity: Option<BTreeMap<String, f64>>,
    /// Is Valet Station
    pub is_valet_station: Option<bool>,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
    /// Vehicle Type Capacity
    pub vehicle_type_capacity: Option<BTreeMap<String, f64>>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV22 {
    /// Stations
    pub stations: Vec<GBFSStationInformationStationV22>,
}

/// # GBFS Station Information V2.2
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains station information for the system.
    pub data: GBFSStationInformationDataV22,
}

/// GBFS Station Information Rental Method
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationRentalMethodV21 {
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

/// GBFS Station Information Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationStationV21 {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethodV21>>,
    /// Is Virtual Station
    pub is_virtual_station: Option<bool>,
    /// Station Area
    pub station_area: Option<MultiPolygonGeometry>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Vehicle Capacity
    pub vehicle_capacity: Option<BTreeMap<String, f64>>,
    /// Is Valet Station
    pub is_valet_station: Option<bool>,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
    /// Vehicle Type Capacity
    pub vehicle_type_capacity: Option<BTreeMap<String, f64>>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV21 {
    /// Stations
    pub stations: Vec<GBFSStationInformationStationV21>,
}

/// # GBFS Station Information V2.1
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains station information for the system.
    pub data: GBFSStationInformationDataV21,
}

/// GBFS Station Information Rental Method
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationRentalMethodV20 {
    /// Key
    #[default]
    #[serde(rename = "KEY")]
    Key,
    /// Credit Card
    #[serde(rename = "CREDITCARD")]
    CreditCard,
    /// PayPass
    #[serde(rename = "PAYPASS")]
    PayPass,
    /// Apple Pay
    #[serde(rename = "APPLEPAY")]
    ApplePay,
    /// Android Pay
    #[serde(rename = "ANDROIDPAY")]
    AndroidPay,
    /// Google Wallet
    #[serde(rename = "TRANSITCARD")]
    TransitCard,
    /// Cash
    #[serde(rename = "ACCOUNTNUMBER")]
    AccountNumber,
    /// Phone
    #[serde(rename = "PHONE")]
    Phone,
}

/// GBFS Station Information Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationStationV20 {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethodV20>>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV20 {
    /// Stations
    pub stations: Vec<GBFSStationInformationStationV20>,
}

/// # GBFS Station Information V2.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Contains station information for the system.
    pub data: GBFSStationInformationDataV20,
}
