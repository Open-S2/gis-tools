use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Hours Schema V1.1 OR GBFS System Hours Schema V1.0
/// Describes the system hours of operation.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_hoursjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_hoursjson)
pub type GBFSSystemHoursV1 = GBFSSystemHoursV11;

/// GBFS System Hours User Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemHourTypeV1 {
    /// GBFS System Hours User Type
    #[serde(rename = "member")]
    #[default]
    Member,
    /// GBFS System Hours User Type
    #[serde(rename = "nonmember")]
    NonMember,
}

/// GBFS System Hours Day
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemHourDayV1 {
    /// Sunday
    #[serde(rename = "sun")]
    #[default]
    Sun,
    /// Monday
    #[serde(rename = "mon")]
    Mon,
    /// Tuesday
    #[serde(rename = "tue")]
    Tue,
    /// Wednesday
    #[serde(rename = "wed")]
    Wed,
    /// Thursday
    #[serde(rename = "thu")]
    Thu,
    /// Friday
    #[serde(rename = "fri")]
    Fri,
    /// Saturday
    #[serde(rename = "sat")]
    Sat,
}

/// GBFS System Hour
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHourV1 {
    /// GBFS System Hours User Type
    pub user_types: Vec<GBFSSystemHourTypeV1>,
    /// GBFS System Hours Day
    pub days: Vec<GBFSSystemHourDayV1>,
    /// Start time
    pub start_time: String,
    /// End time
    pub end_time: String,
}

/// GBFS System Hours Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursDataV1 {
    /// Rental hours
    pub rental_hours: Vec<GBFSSystemHourV1>,
}

/// GBFS System Hours Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing system hours of operations.
    pub data: GBFSSystemHoursDataV1,
}

/// GBFS System Hours Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing system hours of operations.
    pub data: GBFSSystemHoursDataV1,
}
