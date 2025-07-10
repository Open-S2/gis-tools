use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Fare Leg Join Rules
///
/// **Optional**
/// Defines when two consecutive legs with a transfer should be considered as
/// a single “effective fare leg” for the purpose of matching rules in `fare_leg_rules.txt`.
///
/// **Primary Key**: (`from_network_id`, `to_network_id`, `from_stop_id`, `to_stop_id`)
///
/// Matching Logic:
/// - If both `from_network_id` and `to_network_id` match consecutive legs’ networks,
///   and `from_stop_id`/`to_stop_id` match station or stop IDs for the transfer,
///   those two legs merge into one effective leg.
/// - If a field is empty, that field is ignored for matching.
/// - Consecutive transfers that each match a join rule merge the entire sub-journey
///   into a single effective fare leg.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareLegJoinRule {
    /// **Required**
    /// Matches the pre-transfer leg’s route network (`routes.network_id` or `networks.network_id`).
    /// Must be specified alongside `toNetworkId`.
    pub from_network_id: String,
    /// **Required**
    /// Matches the post-transfer leg’s route network (`routes.network_id` or `networks.network_id`).
    /// Must be specified alongside `fromNetworkId`.
    pub to_network_id: String,
    /// **Conditionally Required**
    /// Matches the pre-transfer leg’s ending stop/station (`stops.stop_id`).
    /// Required if `toStopId` is defined; optional otherwise.
    pub from_stop_id: Option<String>,
    /// **Conditionally Required**
    /// Matches the post-transfer leg’s starting stop/station (`stops.stop_id`).
    /// Required if `fromStopId` is defined; optional otherwise.
    pub to_stop_id: Option<String>,
}
impl GTFSFareLegJoinRule {
    /// Create a new GTFSFareLegJoinRule
    pub fn new(source: &str) -> Vec<GTFSFareLegJoinRule> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSFareLegJoinRule>(source, None, None) {
            res.push(record);
        }
        res
    }
}
