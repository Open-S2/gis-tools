use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// Indicates the type of service for a trip with frequencies:
/// 0 or empty = Frequency-based trips
/// 1 = Schedule-based trips (with identical headway)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSExactTimes {
    /// 0 - Frequency-based trips
    FrequencyBased = 0,
    /// 1 - Schedule-based trips
    ScheduleBased = 1,
}
impl From<i32> for GTFSExactTimes {
    fn from(value: i32) -> Self {
        match value {
            1 => GTFSExactTimes::ScheduleBased,
            _ => GTFSExactTimes::FrequencyBased,
        }
    }
}

/// # Frequency
///
/// **Optional**
/// Defines headway-based (or compressed schedule-based) service for specific trips.
/// Each record references a single trip and indicates:
/// - A start/end time window
/// - A headway (seconds between departures)
/// - Whether it’s frequency-based (exact_times=0) or schedule-based (exact_times=1).
///
/// **Primary Key**: (`trip_id`, `start_time`)
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFrequency {
    /// **Required**
    /// Identifies the trip (`trips.trip_id`) to which the specified headway of service applies.
    pub trip_id: String,
    /// **Required**
    /// Time at which the first vehicle departs from the trip’s first stop
    /// with the specified headway (HH:MM:SS, can exceed 24:00:00 if overnight).
    pub start_time: String,
    /// **Required**
    /// Time at which service changes or ends (HH:MM:SS, can exceed 24:00:00 if overnight).
    pub end_time: String,
    /// **Required**
    /// Headway in seconds between departures from the same stop for this trip,
    /// during [start_time, end_time).
    pub headway_secs: i64,
    /// **Optional**
    /// Whether this is frequency-based or schedule-based service.
    /// - 0 or empty = Frequency-based
    /// - 1 = Schedule-based
    pub exact_times: Option<i32>, // GTFSExactTimes;
}
impl GTFSFrequency {
    /// Create a new GTFSFrequency
    pub fn new(source: &str) -> Vec<GTFSFrequency> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSFrequency>(source, None, None) {
            res.push(record);
        }
        res
    }
    /// Get the exact times
    pub fn exact_times(&self) -> Option<GTFSExactTimes> {
        self.exact_times.map(GTFSExactTimes::from)
    }
}
