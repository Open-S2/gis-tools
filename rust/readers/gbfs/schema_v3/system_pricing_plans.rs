use crate::readers::GBFSName;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Pricing Plans Schema V3.1-RC & V3.0
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
pub type GBFSSystemPricingPlansV3 = GBFSSystemPricingPlansV30;

/// GBFS System Pricing Plan Rates
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlanRatesV30 {
    /// Start distance or time.
    pub start: f64,
    /// Rate of the pricing plan.
    pub rate: f64,
    /// Interval of the pricing plan.
    pub interval: f64,
    /// End distance or time.
    pub end: Option<f64>,
}

/// GBFS System Pricing Plan
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlanV30 {
    /// Identifier of the pricing plan.
    pub plan_id: String,
    /// URL where customers can learn more about this pricing plan.
    pub url: Option<String>,
    /// Name of the pricing plan.
    pub name: Vec<GBFSName>,
    /// Currency in ISO 4217 format.
    pub currency: String,
    /// Base price of the pricing plan.
    pub price: f64,
    /// Indicates if additional tax is applied to the base price.
    pub is_taxable: bool,
    /// Description of the pricing plan.
    pub description: Vec<GBFSName>,
    /// Segments for distance-based pricing.
    pub per_km_pricing: Option<Vec<GBFSSystemPricingPlanRatesV30>>,
    /// Segments for time-based pricing.
    pub per_min_pricing: Option<Vec<GBFSSystemPricingPlanRatesV30>>,
    /// Indicates if surge pricing is active.
    pub surge_pricing: Option<bool>,
}

/// GBFS System Pricing Plans Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansDataV30 {
    /// GBFS System Pricing Plan List
    pub plans: Vec<GBFSSystemPricingPlanV30>,
}

/// # GBFS System Pricing Plans Schema V3.0
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    pub version: String,
    /// Pricing plan data.
    pub data: GBFSSystemPricingPlansDataV30,
}
