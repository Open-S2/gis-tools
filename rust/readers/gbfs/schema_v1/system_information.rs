use alloc::string::String;
use serde::{Deserialize, Serialize};

/// # GBFS System Information Schema V1.1 OR GBFS System Information Schema V1.0
/// Details including system operator, system location, year implemented, URL, contact info, and time zone.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_informationjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_informationjson)
pub type GBFSSystemInformationV1 = GBFSSystemInformationV11;

/// GBFS System Information Rental App Container
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalApp {
    /// Store URI
    pub store_uri: String,
    /// Discovery URI
    pub discovery_uri: String,
}

/// GBFS System Information Rental Apps
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalApps {
    /// Android
    pub android: Option<GBFSSystemInformationRentalApp>,
    /// iOS
    pub ios: Option<GBFSSystemInformationRentalApp>,
}

/// GBFS System Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationDataV11 {
    /// System ID
    pub system_id: String,
    /// Language
    pub language: String,
    /// System name
    pub name: String,
    /// Short name
    pub short_name: Option<String>,
    /// Operator
    pub operator: Option<String>,
    /// URL
    pub url: Option<String>,
    /// Purchase URL
    pub purchase_url: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// Phone number
    pub phone_number: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Feed contact email
    pub feed_contact_email: Option<String>,
    /// Timezone
    pub timezone: String,
    /// License URL
    pub license_url: Option<String>,
    /// Rental Apps
    pub rental_apps: Option<GBFSSystemInformationRentalApps>,
}

/// GBFS System Information Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing system information.
    pub data: GBFSSystemInformationDataV11,
}

/// GBFS System Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationDataV10 {
    /// System ID
    pub system_id: String,
    /// Language
    pub language: String,
    /// Name
    pub name: String,
    /// Short name
    pub short_name: Option<String>,
    /// Operator
    pub operator: Option<String>,
    /// URL
    pub url: Option<String>,
    /// Purchase URL
    pub purchase_url: Option<String>,
    /// Start date
    pub start_date: Option<String>,
    /// Phone number
    pub phone_number: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Timezone
    pub timezone: String,
    /// License URL
    pub license_url: Option<String>,
}

/// GBFS System Information Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing system information.
    pub data: GBFSSystemInformationDataV10,
}
