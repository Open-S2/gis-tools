use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Fare Rules
///
/// **Optional**
/// Defines how fares in `fare_attributes.txt` apply to an itinerary.
/// For more complex fare structures, multiple combinations of fields
/// (route, origin, destination, zones) can be used.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareRule {
    /// **Required**
    /// Identifies a fare class (`fare_attributes.fare_id`).
    pub fare_id: String,
    /// **Optional**
    /// Route associated with this fare. If multiple routes share the same fare,
    /// add multiple records in `fare_rules.txt`.
    pub route_id: Option<String>,
    /// **Optional**
    /// Origin zone (`stops.zone_id`). If a fare class applies to multiple origin zones,
    /// each zone requires its own record.
    pub origin_id: Option<String>,
    /// **Optional**
    /// Destination zone (`stops.zone_id`). If a fare class applies to multiple destination zones,
    /// each zone requires its own record.
    pub destination_id: Option<String>,
    /// **Optional**
    /// All zones traveled during the trip using this fare class.
    /// If multiple zones must be passed, each is listed separately.
    pub contains_id: Option<String>,
}
impl GTFSFareRule {
    /// Create a new GTFSFareRule
    pub fn new(source: &str) -> Vec<GTFSFareRule> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSFareRule>(source, None, None) {
            res.push(record);
        }
        res
    }
}
