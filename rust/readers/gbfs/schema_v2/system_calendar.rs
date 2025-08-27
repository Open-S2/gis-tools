use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Calendar Schema V2.3, V2.2, V2.1, OR V2.0
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
pub type GBFSSystemCalendarV2 = GBFSSystemCalendarV23;

/// GBFS System Calendar Calendar
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarCalendarV2 {
    /// Start month of the calendar.
    pub start_month: u64,
    /// Start day of the calendar.
    pub start_day: u64,
    /// Start year of the calendar.
    pub start_year: Option<u64>,
    /// End month of the calendar.
    pub end_month: u64,
    /// End day of the calendar.
    pub end_day: u64,
    /// End year of the calendar.
    pub end_year: Option<u64>,
}

/// GBFS System Calendar Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarDataV2 {
    /// List of all the system's operating calendars.
    pub calendars: Vec<GBFSSystemCalendarCalendarV2>,
}

/// # GBFS System Calendar V2.3
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains the operations calendar data.
    pub data: GBFSSystemCalendarDataV2,
}

/// # GBFS System Calendar V2.2
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains the operations calendar data.
    pub data: GBFSSystemCalendarDataV2,
}

/// # GBFS System Calendar V2.1
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains the operations calendar data.
    pub data: GBFSSystemCalendarDataV2,
}

/// # GBFS System Calendar V2.0
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Contains the operations calendar data.
    pub data: GBFSSystemCalendarDataV2,
}
