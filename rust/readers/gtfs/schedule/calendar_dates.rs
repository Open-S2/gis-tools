use crate::{
    readers::{csv::parse_csv_as_record, parse_gtfs_date},
    util::Date,
};
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Describes whether service is added or removed on a specific date.
/// 1 - Service added for this date.
/// 2 - Service removed for this date.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GTFSExceptionType {
    /// Service added for this date.
    Added = 1,
    /// Service removed for this date.
    Removed = 2,
}
impl From<i8> for GTFSExceptionType {
    fn from(s: i8) -> Self {
        match s {
            2 => GTFSExceptionType::Removed,
            _ => GTFSExceptionType::Added,
        }
    }
}

/// # Calendar Dates
///
/// **Conditionally Required**
/// Explicitly activates or disables service on particular dates.
/// - If used with `calendar.txt`, it modifies the default service patterns.
/// - If `calendar.txt` is omitted, all service dates must be listed here.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSCalendarDate {
    /**
     * **Required**
     * Identifies a set of dates where service exception occurs.
     * References `calendar.service_id` if used with `calendar.txt`;
     * or acts as a standalone ID if `calendar.txt` is omitted.
     */
    pub service_id: String,
    /**
     * **Required**
     * Date of the service exception, parsed as a JavaScript Date.
     * Originally in GTFS as a YYYYMMDD string (no time component).
     */
    pub date: String,
    /**
     * **Required**
     * Indicates whether service is added (1) or removed (2) on this date.
     */
    pub exception_type: i8,
}
impl GTFSCalendarDate {
    /// Create a new GTFSCalendarDate
    pub fn new(source: &str) -> BTreeMap<String, GTFSCalendarDate> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSCalendarDate>(source, None, None) {
            res.insert(record.service_id.clone(), record);
        }
        res
    }
    /// Get the exception type
    pub fn exception_type(&self) -> GTFSExceptionType {
        self.exception_type.into()
    }
    /// Get the date
    pub fn date(&self) -> Date {
        parse_gtfs_date(&self.date).unwrap_or_default()
    }
}
