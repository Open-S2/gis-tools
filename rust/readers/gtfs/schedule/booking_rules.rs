use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Describes how far in advance rider can book:
/// 0 - Real-time
/// 1 - Same-day (with advance notice)
/// 2 - Prior day(s)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GTFSBookingType {
    /// Real-time
    RealTime = 0,
    /// Same-day
    SameDay = 1,
    /// Prior day(s)
    PriorDays = 2,
}

/// # Booking Rules
///
/// **Optional**
/// Defines rules for booking rider-requested services. Useful when a trip or stop_time requires
/// advanced scheduling (e.g., dial-a-ride, on-demand pickup).
///
/// **Primary Key**: (booking_rule_id)
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSBookingRule {
    /// **Required**
    /// Identifies a booking rule (`booking_rule_id`).
    pub booking_rule_id: String,
    /// **Required**
    /// Indicates how far in advance booking can be made.
    /// 0 = Real-time, 1 = Same-day, 2 = Prior-day(s)
    pub booking_type: i8,
    /// **Conditionally Required**
    /// Minimum number of minutes before travel to make the request.
    /// Required for booking_type=1; forbidden otherwise.
    pub prior_notice_duration_min: Option<i32>,
    /// **Conditionally Forbidden**
    /// Maximum number of minutes before travel to make the same-day request.
    /// - Forbidden for booking_type=0 or booking_type=2
    /// - Optional for booking_type=1
    pub prior_notice_duration_max: Option<i32>,
    /// **Conditionally Required**
    /// Last day before travel to make booking request. E.g., 1 = 1 day in advance.
    /// Required for booking_type=2; forbidden otherwise.
    pub prior_notice_last_day: Option<i32>,
    /// **Conditionally Required**
    /// Last time on the last day before travel to make booking request, e.g. "17:00:00".
    /// Required if prior_notice_last_day is defined; forbidden otherwise.
    pub prior_notice_last_time: Option<String>,
    /// **Conditionally Forbidden**
    /// Earliest day before travel to make booking request.
    /// - Forbidden for booking_type=0.
    /// - Forbidden for booking_type=1 if prior_notice_duration_max is defined.
    /// - Optional otherwise (mainly for booking_type=2).
    pub prior_notice_start_day: Option<i32>,
    /// **Conditionally Required**
    /// Earliest time on the earliest day before travel, e.g. "00:00:00".
    /// Required if prior_notice_start_day is defined; forbidden otherwise.
    pub prior_notice_start_time: Option<String>,
    /// **Conditionally Forbidden**
    /// Service days on which last_day / start_day are counted (`calendar.service_id`).
    /// - Optional if booking_type=2.
    /// - Forbidden otherwise.
    pub prior_notice_service_id: Option<String>,
    /// **Optional**
    /// Generic message to riders for on-demand booking instructions.
    pub message: Option<String>,
    /// **Optional**
    /// Message for on-demand pickup instructions.
    pub pickup_message: Option<String>,
    /// **Optional**
    /// Message for on-demand drop-off instructions.
    pub drop_off_message: Option<String>,
    /// **Optional**
    /// Phone number riders call to make the booking request.
    pub phone_number: Option<String>,
    /// **Optional**
    /// URL providing additional booking info.
    pub info_url: Option<String>,
    /// **Optional**
    /// URL to an online interface or app to make a booking request.
    pub booking_url: Option<String>,
}
impl GTFSBookingRule {
    /// Create a new GTFSBookingRule
    pub fn new(source: &str) -> BTreeMap<String, GTFSBookingRule> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSBookingRule>(source, None, None) {
            res.insert(record.booking_rule_id.clone(), record);
        }
        res
    }
    /// Get the booking type
    pub fn get_booking_type(&self) -> GTFSBookingType {
        match self.booking_type {
            1 => GTFSBookingType::SameDay,
            2 => GTFSBookingType::PriorDays,
            _ => GTFSBookingType::RealTime,
        }
    }
}
