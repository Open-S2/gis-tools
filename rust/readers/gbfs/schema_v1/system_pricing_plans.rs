use crate::readers::gbfs_bool_or_int;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Pricing Plans Schema V1.1 OR GBFS System Pricing Plans Schema V1.0
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_pricing_plansjson)
pub type GBFSSystemPricingPlansV1 = GBFSSystemPricingPlansV11;

/// GBFS System Pricing Plan
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlan {
    /// Plan ID
    pub plan_id: String,
    /// URL
    pub url: Option<String>,
    /// Name
    pub name: String,
    /// Currency
    pub currency: String,
    /// Price
    pub price: f64,
    /// Is taxable
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_taxable: bool,
    /// Description
    pub description: String,
}

/// GBFS System Pricing Plans Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansData {
    /// GBFS System Pricing Plan List
    pub plans: Vec<GBFSSystemPricingPlan>,
}

///GBFS System Pricing Plans Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing pricing plans of the system.
    pub data: GBFSSystemPricingPlansData,
}

/// GBFS System Pricing Plans Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing pricing plans of the system.
    pub data: GBFSSystemPricingPlansData,
}
