use alloc::string::String;
use serde::{Deserialize, Serialize};

/// # GBFS System Information Schema V2.3, V2.2, V2.1, OR V2.0
/// Details including system operator, system location, year implemented, URL, contact info, time zone.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
pub type GBFSSystemInformationV2 = GBFSSystemInformationV23;

/// GBFS System Information Rental Apps
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalAppsV20 {
    /// Rental app URL
    pub store_uri: String,
    /// Rental app discovery URL
    pub discovery_uri: String,
}

/// GBFS System Information Rental App Container
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalAppV20 {
    /// Android Rental App
    pub android: Option<GBFSSystemInformationRentalAppsV20>,
    /// iOS Rental App
    pub ios: Option<GBFSSystemInformationRentalAppsV20>,
}

/// GBFS System Information Brand Assets
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationBrandAssetsV20 {
    /// Last modified date of the brand assets
    pub brand_last_modified: String,
    /// URL to the brand terms
    pub brand_terms_url: Option<String>,
    /// URL to the brand image
    pub brand_image_url: String,
    /// URL to the dark mode brand image
    pub brand_image_url_dark: Option<String>,
    /// Color used to represent the brand
    pub color: Option<String>,
}

/// GBFS System Information Data V2.3
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationDataV23 {
    /// System ID
    pub system_id: String,
    /// System language
    pub language: String, // Matches BCP-47 language tags
    /// System name
    pub name: String,
    /// System short name
    pub short_name: Option<String>,
    /// System operator
    pub operator: Option<String>,
    /// System operator URL
    pub url: Option<String>,
    /// System purchase URL
    pub purchase_url: Option<String>,
    /// Start date
    pub start_date: Option<String>, // ISO 8601 format
    /// System phone number
    pub phone_number: Option<String>,
    /// System email
    pub email: Option<String>,
    /// System feed contact email
    pub feed_contact_email: Option<String>,
    /// System time zone
    pub timezone: String,
    /// System license
    pub license_url: Option<String>,
    /// System brand
    pub brand_assets: Option<GBFSSystemInformationBrandAssetsV20>,
    /// Terms URL
    pub terms_url: Option<String>,
    /// Terms last updated
    pub terms_last_updated: Option<String>, // ISO 8601 format
    /// Privacy URL
    pub privacy_url: Option<String>,
    /// Privacy last updated
    pub privacy_last_updated: Option<String>, // ISO 8601 format
    /// Rental apps
    pub rental_apps: Option<GBFSSystemInformationRentalAppV20>,
}

/// # GBFS System Information V2.3
/// Details including system operator, system location, year implemented, URL, contact info, and time zone.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV23 {
    /// Last updated
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Data
    pub data: GBFSSystemInformationDataV23,
}

/// # GBFS System Information Schema V2.2
/// Details including system operator, system location, year implemented, URL, contact info, and time zone.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV22 {
    /// Last updated
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Data
    pub data: GBFSSystemInformationDataV20,
}

/// # GBFS System Information Schema V2.1
/// Details including system operator, system location, year implemented, URL, contact info, and time zone.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV21 {
    /// Last updated
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Data
    pub data: GBFSSystemInformationDataV20,
}

/// GBFS System Information Data V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationDataV20 {
    /// System ID
    pub system_id: String,
    /// System language
    pub language: String, // Matches BCP-47 language tags
    /// System name
    pub name: String,
    /// System short name
    pub short_name: Option<String>,
    /// System operator
    pub operator: Option<String>,
    /// System operator URL
    pub url: Option<String>,
    /// System purchase URL
    pub purchase_url: Option<String>,
    /// Start date
    pub start_date: Option<String>, // ISO 8601 format
    /// System phone number
    pub phone_number: Option<String>,
    /// System email
    pub email: Option<String>,
    /// System feed contact email
    pub feed_contact_email: Option<String>,
    /// System time zone
    pub timezone: String,
    /// System license
    pub license_url: Option<String>,
    /// System brand
    pub rental_apps: Option<GBFSSystemInformationRentalAppV20>,
}

/// # GBFS System Information Schema V2.0
/// Details including system operator, system location, year implemented, URL, contact info, and time zone.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV20 {
    /// Last updated
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Data
    pub data: GBFSSystemInformationDataV20,
}
