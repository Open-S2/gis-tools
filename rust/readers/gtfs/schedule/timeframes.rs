use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Timeframes
///
/// **Optional**
/// Describes fare variations based on time of day, day of week, or specific dates.
/// Timeframes can be associated with fare products in `fare_leg_rules.txt`.
/// There must be no overlapping [start_time, end_time) intervals for the same
/// `timeframe_group_id` and `service_id`.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSTimeframe {
    /// **Required**
    /// Identifies a timeframe (or set of timeframes).
    pub timeframe_group_id: String,
    /// **Conditionally Required**
    /// Beginning of a timeframe in HH:MM:SS format (<= 24:00:00).
    /// The interval **includes** this time.
    /// - If `end_time` is defined, `start_time` is required.
    /// - If `end_time` is absent, `start_time` must be absent.
    /// - If `start_time` is empty in the CSV, it is considered `00:00:00`.
    pub start_time: Option<String>,
    /// **Conditionally Required**
    /// End of a timeframe in HH:MM:SS format (<= 24:00:00).
    /// The interval **excludes** this time.
    /// - If `start_time` is defined, `end_time` is required.
    /// - If `start_time` is absent, `end_time` must be absent.
    /// - If `end_time` is empty in the CSV, it is considered `24:00:00`.
    pub end_time: Option<String>,
    /// **Required**
    /// Identifies a set of dates (`calendar.service_id` or `calendar_dates.service_id`)
    /// when this timeframe is in effect.
    pub service_id: String,
}
impl GTFSTimeframe {
    /// Create a new GTFSTimeframe
    pub fn new(source: &str) -> BTreeMap<String, GTFSTimeframe> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSTimeframe>(source, None, None) {
            res.insert(record.timeframe_group_id.clone(), record);
        }
        res
    }
}
