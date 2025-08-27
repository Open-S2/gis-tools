use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Calendar Schema V1.1 OR GBFS System Calendar Schema V1.0
/// Describes the operating calendar for a system.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_calendarjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_calendarjson)
pub type GBFSSystemCalendarV1 = GBFSSystemCalendarV11;

/// GBFS System Calendar Calendar
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarCalendarV1 {
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
pub struct GBFSSystemCalendarDataV1 {
    /// List of all the system's operating calendars.
    pub calendars: Vec<GBFSSystemCalendarCalendarV1>,
}

/// GBFS System Calendar Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemCalendarV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing the system's operations calendar.
    pub data: GBFSSystemCalendarDataV1,
}

/// GBFS System Calendar Schema V1.0 Interface
pub struct GBFSSystemCalendarV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing the system's operations calendar.
    pub data: GBFSSystemCalendarDataV1,
}
