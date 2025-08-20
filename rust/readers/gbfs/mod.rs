mod schema_v1;
mod schema_v2;
mod schema_v3;

use crate::readers::parse_csv_as_btree;
use alloc::{format, string::String, vec, vec::Vec};
pub use schema_v1::*;
pub use schema_v2::*;
pub use schema_v3::*;
use serde::{Deserialize, Deserializer, Serialize};

/// Contains rental URIs for Android, iOS, and web (added in v1.1).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSRentalUri {
    /// URI that can be passed to an Android app with an intent (added in v1.1).
    /// **Format**: URI
    pub android: Option<String>,
    /// URI that can be used on iOS to launch the rental app for this vehicle (added in v1.1).
    /// **Format**: URI
    pub ios: Option<String>,
    /// URL that can be used by a web browser to show more information about renting this vehicle (added in v1.1).
    /// **Format**: URI
    pub web: Option<String>,
}

/// System Definition that is returned from the github CSV file.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystem {
    /// [**Required**] ISO 3166-1 alpha-2 code designating the country where the system is located.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// [**Required**] Name of the mobility system. This MUST match the name field in `system_information.json`
    pub name: String,
    /// [**Required**] Primary city in which the system is located, followed by the 2-letter state code
    /// for US systems. The location name SHOULD be in English if the location has an English name
    /// (e.g.: Brussels).
    pub location: String,
    /// [**Required**] ID for the system. This MUST match the system_id field in `system_information.json`.
    #[serde(rename = "systemId")]
    pub system_id: String,
    /// [**Required**] URL for the system from the url field in `system_information.json`.
    /// If the url field is not included in `system_information.json` this SHOULD be the primary URL
    /// for the system operator.
    pub url: String,
    /// [**Required**] URL for the system's gbfs.json auto-discovery file.
    #[serde(rename = "autoDiscoveryUrl")]
    pub auto_discovery_url: String,
    /// [**Required**] List of GBFS version(s) under which the feed is published. Multiple values are
    /// separated by a semi-colon surrounded with 1 space on each side for readability (" ; ").
    #[serde(rename = "supportedVersions")]
    pub supported_versions: Vec<String>,
    /// [**Conditionally Required**] If authentication is required, this MUST contain a URL to a
    /// human-readable page describing how the authentication should be performed and how credentials
    /// can be created, or directly contain the public key-value pair to append to the feed URLs.
    #[serde(rename = "authInfo")]
    pub auth_info: Option<String>,
}

/// # General Bikeshare Feed Specification (GBFS) Reader
///
/// ## Description
/// Fetches the list of GBFS systems from the github CSV file
///
/// ## Usage
///
/// ```rust
/// use gistools::readers::{GBFSSystem, parse_gtfs_systems};
/// use std::{fs, path::PathBuf};
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/gbfs/fixtures/systems.csv");
/// let file_str = fs::read_to_string(path).unwrap();
///
/// let systems = parse_gtfs_systems(file_str.as_str());
/// assert_eq!(systems.len(), 960);
/// ```
///
/// ## Links
/// - https://github.com/MobilityData/gbfs/blob/master/systems.csv
///
/// ## Parameters
/// - `systems_csv`: The data of the CSV file as a string. The default is the one used by GBFS.
///   This variable exists for testing
///
/// ## Returns
/// An array of systems
pub fn parse_gtfs_systems(systems_csv: &str) -> Vec<GBFSSystem> {
    let mut res = vec![];
    let parsed = parse_csv_as_btree(systems_csv, None, None);

    for system in parsed {
        let name = system.get("Name").cloned().unwrap_or_default();
        let location = system.get("Location").cloned().unwrap_or_default();
        let url = system.get("URL").cloned().unwrap_or_default();
        let country_code = system.get("Country Code").cloned().unwrap_or_default();
        let system_id = system.get("System ID").cloned().unwrap_or_default();
        let auto_discovery_url = system.get("Auto-Discovery URL").cloned().unwrap_or_default();
        let supported_versions = system.get("Supported Versions").cloned().unwrap_or_default();
        let supported_versions: Vec<String> =
            supported_versions.split(" ; ").map(|v| v.trim().into()).collect();
        let auth_info = system.get("Authentication Info").cloned();
        res.push(GBFSSystem {
            name,
            location,
            url,
            country_code,
            system_id,
            auto_discovery_url,
            supported_versions,
            auth_info,
        });
    }

    res
}

/// Converts a boolean or integer 0/1 to a boolean
pub fn gbfs_bool_or_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolOrIntVisitor;

    impl<'de> serde::de::Visitor<'de> for BoolOrIntVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean or an integer 0/1")
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(E::custom("expected 0 or 1 for a boolean")),
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_u64(value as u64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match value {
                "0" => Ok(false),
                "1" => Ok(true),
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(E::custom(format!("invalid boolean string: {value}"))),
            }
        }
    }

    deserializer.deserialize_any(BoolOrIntVisitor)
}
