use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Fare Leg Rules
///
/// **Optional**
/// Defines more granular fare rules for individual legs of travel (GTFS-Fares V2).
///
/// Use these rules by filtering on:
/// - `network_id`
/// - `from_area_id`
/// - `to_area_id`
/// - `from_timeframe_group_id`
/// - `to_timeframe_group_id`
///
/// Multiple matching strategies exist depending on the presence or absence of `rule_priority`:
/// - If `rule_priority` does **not** exist, empty fields represent an **inverse** match against
///   all possible values **except** those otherwise specified.
/// - If `rule_priority` **does** exist, empty fields mean the field does not affect matching.
///
/// For matching an “effective fare leg” that spans multiple legs, see the specification for
/// rules on using the first vs. last leg’s departure/arrival areas and timeframes.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareLegRule {
    /// **Optional**
    /// Identifies a group of entries in fare_leg_rules.txt that can be referenced
    /// in `fare_transfer_rules.from_leg_group_id` or `fare_transfer_rules.to_leg_group_id`.
    pub leg_group_id: Option<String>,
    /// **Optional**
    /// Identifies a route network (`routes.network_id` or `networks.network_id`) this rule applies to.
    /// - If `rule_priority` is omitted and no matching `network_id`, empty matches “all but listed”.
    /// - If `rule_priority` exists, empty means network does not affect matching.
    pub network_id: Option<String>,
    /// **Optional**
    /// Identifies a departure area (`areas.area_id`) for this fare leg rule.
    /// - If `rule_priority` is omitted and no matching `from_area_id`, empty matches “all but listed”.
    /// - If `rule_priority` exists, empty means departure area does not affect matching.
    pub from_area_id: Option<String>,
    /// **Optional**
    /// Identifies an arrival area (`areas.area_id`) for this fare leg rule.
    /// - If `rule_priority` is omitted and no matching `to_area_id`, empty matches “all but listed”.
    /// - If `rule_priority` exists, empty means arrival area does not affect matching.
    pub to_area_id: Option<String>,
    /// **Optional**
    /// References a `timeframes.timeframe_group_id` for the start of the fare leg.
    /// An empty value means the start time does not affect matching.
    pub from_timeframe_group_id: Option<String>,
    /// **Optional**
    /// References a `timeframes.timeframe_group_id` for the end of the fare leg.
    /// An empty value means the end time does not affect matching.
    pub to_timeframe_group_id: Option<String>,
    /// **Required**
    /// References a `fare_products.fare_product_id`.
    /// The rider must possess/purchase this fare product for the described leg.
    pub fare_product_id: String,
    /// **Optional**
    /// Defines the order of priority in which matching rules are applied.
    /// - Higher priority overrides lower priority when multiple rules match.
    /// - Empty is treated as zero.
    pub rule_priority: i64,
}
impl GTFSFareLegRule {
    /// Create a new GTFSFareLegRule
    pub fn new(source: &str) -> Vec<GTFSFareLegRule> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSFareLegRule>(source, None, None) {
            res.push(record);
        }
        res
    }
}
