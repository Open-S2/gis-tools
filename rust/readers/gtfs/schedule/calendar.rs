use crate::{
    readers::{csv::parse_csv_as_record, parse_gtfs_date},
    util::Date,
};
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// Enumeration to represent day availability in the calendar.
/// 0 = Not available, 1 = Available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSDayAvailability {
    /// 0 - Service not available on this day.
    NotAvailable = 0,
    /// 1 - Service available on this day.
    Available = 1,
}
impl From<&str> for GTFSDayAvailability {
    fn from(s: &str) -> Self {
        match s.trim() {
            "1" => GTFSDayAvailability::Available,
            _ => GTFSDayAvailability::NotAvailable,
        }
    }
}

/// # Calendar Information
///
/// **Conditionally Required**
/// Defines a set of dates when service is available for one or more routes.
/// Required unless all dates of service are defined in `calendar_dates.txt`.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSCalendar {
    /// **Required**
    /// Identifies a set of dates when service is available.
    pub service_id: String,
    /// **Required**
    /// Service availability on Mondays: 0 or 1.
    pub monday: String,
    /// **Required**
    /// Service availability on Tuesdays: 0 or 1.
    pub tuesday: String,
    /// **Required**
    /// Service availability on Wednesdays: 0 or 1.
    pub wednesday: String,
    /// **Required**
    /// Service availability on Thursdays: 0 or 1.
    pub thursday: String,
    /// **Required**
    /// Service availability on Fridays: 0 or 1.
    pub friday: String,
    /// **Required**
    /// Service availability on Saturdays: 0 or 1.
    pub saturday: String,
    /// **Required**
    /// Service availability on Sundays: 0 or 1.
    pub sunday: String,
    /// **Required**
    /// Start service day (inclusive) for the interval. Format: YYYYMMDD
    pub start_date: String,
    /// **Required**
    /// End service day (inclusive) for the interval. Format: YYYYMMDD
    pub end_date: String,
}
impl GTFSCalendar {
    /// Create a new GTFSCalendar
    pub fn new(source: &str) -> Vec<GTFSCalendar> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSCalendar>(source, None, None) {
            res.push(record);
        }
        res
    }
    /// Get the availability for Monday
    pub fn monday(&self) -> GTFSDayAvailability {
        self.monday.as_str().into()
    }
    /// Get the availability for Tuesday
    pub fn tuesday(&self) -> GTFSDayAvailability {
        self.tuesday.as_str().into()
    }
    /// Get the availability for Wednesday
    pub fn wednesday(&self) -> GTFSDayAvailability {
        self.wednesday.as_str().into()
    }
    /// Get the availability for Thursday
    pub fn thursday(&self) -> GTFSDayAvailability {
        self.thursday.as_str().into()
    }
    /// Get the availability for Friday
    pub fn friday(&self) -> GTFSDayAvailability {
        self.friday.as_str().into()
    }
    /// Get the availability for Saturday
    pub fn saturday(&self) -> GTFSDayAvailability {
        self.saturday.as_str().into()
    }
    /// Get the availability for Sunday
    pub fn sunday(&self) -> GTFSDayAvailability {
        self.sunday.as_str().into()
    }
    /// Get the start date
    pub fn start_date(&self) -> Date {
        parse_gtfs_date(&self.start_date).unwrap_or_default()
    }
    /// Get the end date
    pub fn end_date(&self) -> Date {
        parse_gtfs_date(&self.end_date).unwrap_or_default()
    }
}
