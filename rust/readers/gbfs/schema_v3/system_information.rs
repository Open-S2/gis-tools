use crate::readers::GBFSName;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Information Schema V3.1-RC & V3.0
/// Details including system operator, system location, year implemented, URL, contact info, time zone.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
pub type GBFSSystemInformationV3 = GBFSSystemInformationV30;

/// GBFS System Information Rental Apps
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalAppsV30 {
    /// Rental app URL
    pub store_uri: String,
    /// Rental app discovery URL
    pub discovery_uri: String,
}

/// GBFS System Information Rental App Container
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationRentalAppV30 {
    /// Android Rental App
    pub android: Option<GBFSSystemInformationRentalAppsV30>,
    /// iOS Rental App
    pub ios: Option<GBFSSystemInformationRentalAppsV30>,
}

/// GBFS System Information Brand Assets
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationBrandAssetsV30 {
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

/// GBFS System Information Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationDataV30 {
    /// Globally unique identifier for the system.
    pub system_id: String,
    /// List of languages used in translated strings.
    pub languages: Vec<String>,
    /// Name of the system to be displayed to customers.
    pub name: Vec<GBFSName>,
    /// Hours and dates of operation in OSM opening_hours format.
    pub opening_hours: String,
    /// Abbreviation for the system.
    pub short_name: Option<Vec<GBFSName>>,
    /// Name of the system operator.
    pub operator: Option<Vec<GBFSName>>,
    /// URL of the vehicle share system.
    pub url: Option<String>,
    /// URL to purchase a membership.
    pub purchase_url: Option<String>,
    /// Date the system began operations.
    pub start_date: Option<String>,
    /// Date after which the data source will no longer be available.
    pub termination_date: Option<String>,
    /// Customer service phone number in E.164 format.
    pub phone_number: Option<String>,
    /// Email address actively monitored by customer service.
    pub email: Option<String>,
    /// Contact email for feed consumers to report technical issues.
    pub feed_contact_email: String,
    /// URL to the manifest.json file for the publisher.
    pub manifest_url: Option<String>,
    /// Time zone of the system.
    pub timezone: String,
    /// Standard license identifier for the dataset.
    pub license_id: Option<String>,
    /// URL defining the license terms.
    pub license_url: Option<String>,
    /// Name of the organization to which attribution should be provided.
    pub attribution_organization_name: Option<Vec<GBFSName>>,
    /// URL of the organization for attribution.
    pub attribution_url: Option<String>,
    /// Brand assets and related information.
    pub brand_assets: Option<GBFSSystemInformationBrandAssetsV30>,
    /// Terms of service URL.
    pub terms_url: Option<Vec<GBFSName>>,
    /// Date terms of service were last updated.
    pub terms_last_updated: Option<String>,
    /// Privacy policy URL.
    pub privacy_url: Option<Vec<GBFSName>>,
    /// Date the privacy policy was last updated.
    pub privacy_last_updated: Option<String>,
    /// Rental app information for Android and iOS platforms.
    pub rental_apps: Option<GBFSSystemInformationRentalAppV30>,
}

/// # GBFS System Information Schema V3.0
/// Details including system operator, system location, year implemented, URL, contact info, time zone.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemInformationV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// System information data object.
    pub data: GBFSSystemInformationDataV30,
}
