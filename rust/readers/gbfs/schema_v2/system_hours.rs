use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Hours Schema V2.3, V2.2, V2.1, OR V2.0
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
pub type GBFSSystemHoursV2 = GBFSSystemHoursV23;

/// GBFS System Hours User Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemHourTypeV2 {
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
pub enum GBFSSystemHourDayV2 {
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

/// # GBFS System Hours V2.3
/// Describes the system hours of operation.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains system hours data.
    pub data: GBFSSystemHoursDataV20,
}

/// # GBFS System Hours V2.2
/// Describes the system hours of operation.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains system hours data.
    pub data: GBFSSystemHoursDataV20,
}

/// # GBFS System Hours V2.1
/// Describes the system hours of operation.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains system hours data.
    pub data: GBFSSystemHoursDataV20,
}

/// # GBFS System Hours - Rental Hours V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursRentalHourV20 {
    /// Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
    /// **Enum**: ["member", "nonmember"]
    /// **Min Items**: 1
    /// **Max Items**: 2
    pub user_types: Vec<GBFSSystemHourTypeV2>,
    /// Abbreviations of English names of the days of the week.
    /// **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
    /// **Min Items**: 1
    /// **Max Items**: 7
    pub days: Vec<GBFSSystemHourDayV2>,
    /// Start time for the hours of operation of the system.
    /// **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
    pub start_time: String,
    /// End time for the hours of operation of the system.
    /// **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
    pub end_time: String,
}

/// rental hours for the system
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursDataV20 {
    /// rental hours
    pub rental_hours: Vec<GBFSSystemHoursRentalHourV20>,
}

/// # GBFS System Hours V2.0
/// Describes the system hours of operation.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemHoursV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Contains system hours data.
    pub data: GBFSSystemHoursRentalHourV20,
}
