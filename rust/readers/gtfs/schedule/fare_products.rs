use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Fare Products
///
/// **Optional**
/// Describes different fare products riders can purchase.
/// Used by GTFS-Fares V2 to model fare product costs, media, and potential discounts for multi-leg journeys.
///
/// Multiple rows with the same `fare_product_id` can exist, each paired with a different `fare_media_id`.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareProduct {
    /// **Required**
    /// Identifies a fare product or set of fare products.
    pub fare_product_id: String,
    /// **Optional**
    /// The name of the fare product as displayed to riders.
    pub fare_product_name: Option<String>,
    /// **Optional**
    /// Identifies a rider category eligible for the fare product.
    ///
    /// If fare_products.rider_category_id is empty, the fare product is eligible for any
    /// rider_category_id.
    ///
    /// When multiple rider categories are eligible for a single fare product specified by a
    /// fare_product_id, there must be only one of these rider categories indicated as the default
    /// rider category (is_default_fare_category = 1).
    pub rider_category_id: Option<String>,
    /// **Optional**
    /// Identifies a fare media (`fare_media.fare_media_id`) that can be employed to use this fare product.
    /// When empty, the fare media is unknown.
    pub fare_media_id: Option<String>,
    /// **Required**
    /// The cost of the fare product. May be:
    /// - Negative: Transfer discount
    /// - Zero: Free fare
    /// - Positive: Standard fare cost
    pub amount: f64,
    /// **Required**
    /// Currency code (e.g., "USD", "EUR") for the cost of this product.
    pub currency: String,
}
impl GTFSFareProduct {
    /// Create a new GTFSFareProduct
    pub fn new(source: &str) -> BTreeMap<String, GTFSFareProduct> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSFareProduct>(source, None, None) {
            res.insert(record.fare_product_id.clone(), record);
        }
        res
    }
}
