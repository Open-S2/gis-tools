use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// Duration limit type for how transfer durations are measured.
/// Required if `duration_limit` is defined, forbidden otherwise.
///
/// 0 - Between departure of current leg & arrival of next leg
/// 1 - Between departure of current leg & departure of next leg
/// 2 - Between arrival of current leg & departure of next leg
/// 3 - Between arrival of current leg & arrival of next leg
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSDurationLimitType {
    /// Between departure of current leg & arrival of next leg
    DepCurrentArrNext = 0,
    /// Between departure of current leg & departure of next leg
    DepCurrentDepNext = 1,
    /// Between arrival of current leg & departure of next leg
    ArrCurrentDepNext = 2,
    /// Between arrival of current leg & arrival of next leg
    ArrCurrentArrNext = 3,
}
impl From<i8> for GTFSDurationLimitType {
    fn from(s: i8) -> Self {
        match s {
            1 => GTFSDurationLimitType::DepCurrentDepNext,
            2 => GTFSDurationLimitType::ArrCurrentDepNext,
            3 => GTFSDurationLimitType::ArrCurrentArrNext,
            _ => GTFSDurationLimitType::DepCurrentArrNext,
        }
    }
}

/// Fare transfer type describing how costs are processed between consecutive legs:
///
/// 0 = (A) + (AB)
/// 1 = (A) + (AB) + (B)
/// 2 = (AB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSFareTransferType {
    /// A + AB
    FromLegPlusTransfer = 0, // A + AB
    /// A + AB + B
    FromLegTransferToLeg = 1, // A + AB + B
    /// AB
    TransferOnly = 2, // AB
}
impl From<i8> for GTFSFareTransferType {
    fn from(s: i8) -> Self {
        match s {
            1 => GTFSFareTransferType::FromLegTransferToLeg,
            2 => GTFSFareTransferType::TransferOnly,
            _ => GTFSFareTransferType::FromLegPlusTransfer,
        }
    }
}

/**
 * # Fare Transfer Rules
 *
 * **Optional**
 * Defines the cost of transferring between fare legs specified in `fare_leg_rules.txt`.
 * Matching uses:
 * - from_leg_group_id
 * - to_leg_group_id
 * - transfer_count
 * - duration_limit
 * - duration_limit_type
 * - fare_transfer_type
 * - fare_product_id
 *
 * **Primary Key**: (from_leg_group_id, to_leg_group_id, fare_product_id, transfer_count, duration_limit)
 */
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareTransferRule {
    /**
     * **Optional**
     * The pre-transfer fare leg group (`fare_leg_rules.leg_group_id`).
     * - If no exact match is found, empty corresponds to all leg groups not listed under `from_leg_group_id`.
     */
    pub from_leg_group_id: Option<String>,
    /**
     * **Optional**
     * The post-transfer fare leg group (`fare_leg_rules.leg_group_id`).
     * - If no exact match is found, empty corresponds to all leg groups not listed under `to_leg_group_id`.
     */
    pub to_leg_group_id: Option<String>,
    /**
     * **Conditionally Forbidden / Required**
     * Defines how many consecutive transfers this rule may be applied to.
     * - `-1` means no limit.
     * - `1` or more = the transfer count this rule applies to.
     *
     * Forbidden if `from_leg_group_id !== to_leg_group_id`.
     * Required if `from_leg_group_id === to_leg_group_id`.
     */
    pub transfer_count: Option<i32>,
    /**
     * **Optional**
     * Duration limit (in seconds) for the transfer. Empty means no limit.
     */
    pub duration_limit: Option<i32>,
    /**
     * **Conditionally Required**
     * Defines how to measure the `durationLimit`.
     * - Required if `durationLimit` is defined.
     * - Forbidden if `durationLimit` is empty.
     */
    pub duration_limit_type: Option<i8>, // ?: GTFSDurationLimitType;
    /**
     * **Required**
     * Indicates how to combine transfer costs:
     * - 0 = from-leg cost + transfer cost
     * - 1 = from-leg + transfer + to-leg cost
     * - 2 = transfer cost only
     */
    pub fare_transfer_type: i8, // GTFSFareTransferType;
    /**
     * **Optional**
     * Fare product ID for the transfer. If empty, cost is 0 (no transfer cost).
     */
    pub fare_product_id: Option<String>,
}
impl GTFSFareTransferRule {
    /// Create a new GTFSFareTransferRule
    pub fn new(source: &str) -> Vec<GTFSFareTransferRule> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSFareTransferRule>(source, None, None) {
            res.push(record);
        }
        res
    }
    /// Get the duration_limit_type
    pub fn get_duration_limit_type(&self) -> Option<GTFSDurationLimitType> {
        self.duration_limit_type.map(GTFSDurationLimitType::from)
    }
    /// Get the fare_transfer_type
    pub fn get_fare_transfer_type(&self) -> GTFSFareTransferType {
        self.fare_transfer_type.into()
    }
}
