use crate::readers::GBFSRentalUri;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Station Information Schema V1.1 OR GBFS Station Information Schema V1.0
/// List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_informationjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_informationjson)
pub type GBFSStationInformationV1 = GBFSStationInformationV11;

/// GBFS Station Information Rental Methods
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSStationInformationRentalMethods {
    /// Key
    #[default]
    KEY,
    /// Credit Card
    CREDITCARD,
    /// PayPass
    PAYPASS,
    /// Apple Pay
    APPLEPAY,
    /// Android Pay
    ANDROIDPAY,
    /// Transit Card
    TRANSITCARD,
    /// Account Number
    ACCOUNTNUMBER,
    /// Phone
    PHONE,
}

/// GBFS Station Information Station
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV11Station {
    /// Station ID
    pub station_id: String,
    /// Station Name
    pub name: String,
    /// Short Station Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region
    pub region_id: Option<String>,
    /// Postal Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethods>>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV11 {
    /// Data containing an array of stations
    pub stations: Vec<GBFSStationInformationV11Station>,
}

/// GBFS Station Information Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing an array of stations.
    pub data: GBFSStationInformationDataV11,
}

/// GBFS Station Information Rental Methods
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV10Station {
    /// Station ID
    pub station_id: String,
    /// Station Name
    pub name: String,
    /// Short Station Name
    pub short_name: Option<String>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region
    pub region_id: Option<String>,
    /// Postal Code
    pub post_code: Option<String>,
    /// Rental Methods
    pub rental_methods: Option<Vec<GBFSStationInformationRentalMethods>>,
    /// Capacity
    pub capacity: Option<u64>,
}

/// GBFS Station Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationDataV10 {
    /// Data containing an array of stations
    pub stations: Vec<GBFSStationInformationV10Station>,
}

/// GBFS Station Information Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSStationInformationV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing an array of stations.
    pub data: GBFSStationInformationDataV10,
}
