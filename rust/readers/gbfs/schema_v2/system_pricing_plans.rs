use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Pricing Plans V2.3, V2.2, V2.1, OR V2.0
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson)
pub type GBFSSystemPricingPlansV2 = GBFSSystemPricingPlansV23;

/// # GBFS System Pricing Plans Schema V2.3
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV23 {
    /// Last updated timestamp
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Pricing plans
    pub data: GBFSSystemPricingPlansDataV22,
}

/// GBFS System Pricing Rate
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingRateV22 {
    /// Start
    pub start: f64,
    /// Rate
    pub rate: f64,
    /// Interval
    pub interval: f64,
    /// End
    pub end: Option<f64>,
}

/// GBFS System Pricing Plan
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansPlanV22 {
    /// Plan ID
    pub plan_id: String,
    /// Plan URL
    pub url: Option<String>,
    /// Plan name
    pub name: String,
    /// ISO 4217 currency code
    pub currency: String,
    /// Plan price
    pub price: f64,
    /// Plan is taxable
    pub is_taxable: bool,
    /// Plan description
    pub description: String,
    /// Array of segments when the price is a function of distance travelled, displayed in kilometers (added in v2.1-RC2).
    pub per_km_pricing: Option<Vec<GBFSSystemPricingRateV22>>,
    /// Array of segments when the price is a function of time travelled, displayed in minutes (added in v2.1-RC2).
    pub per_min_pricing: Option<Vec<GBFSSystemPricingRateV22>>,
    /// Is there currently an increase in price in response to increased demand in this pricing plan? (added in v2.1-RC2)
    pub surge_pricing: Option<bool>,
}

/// GBFS System Pricing Plans Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansDataV22 {
    /// Plans
    pub plans: Vec<GBFSSystemPricingPlansPlanV22>,
}

/// # GBFS System Pricing Plans Schema V2.2
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV22 {
    /// Last updated timestamp
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Pricing plans
    pub data: GBFSSystemPricingPlansDataV22,
}

/// # GBFS System Pricing Plans Schema V2.1
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV21 {
    /// Last updated timestamp
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// Pricing plans
    pub data: GBFSSystemPricingPlansDataV20,
}

/// GBFS System Pricing Plan
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansPlanV20 {
    /// Plan ID
    pub plan_id: String,
    /// Plan URL
    pub url: Option<String>,
    /// Plan name
    pub name: String,
    /// ISO 4217 currency code
    pub currency: String,
    /// Plan price
    pub price: f64,
    /// Plan is taxable
    pub is_taxable: bool,
    /// Plan description
    pub description: String,
}

/// GBFS System Pricing Plans Data
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansDataV20 {
    /// Plans
    pub plans: Vec<GBFSSystemPricingPlansPlanV20>,
}

/// # GBFS System Pricing Plans Schema V2.0
/// Describes the pricing schemes of the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemPricingPlansV20 {
    /// Last updated
    pub last_updated: u64,
    /// TTL
    pub ttl: u64,
    /// Version
    pub version: String,
    /// System plans
    pub data: GBFSSystemPricingPlansDataV20,
}
