use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Route Networks
///
/// **Conditionally Forbidden**
/// Assigns routes (`routes.route_id`) to networks (`networks.network_id`).
/// This file is forbidden if `network_id` exists in `routes.txt`. Otherwise, it is optional.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSRouteNetwork {
    /// **Required**
    /// Identifies a network (`networks.network_id`) to which one or multiple routes belong.
    pub network_id: String,
    /// **Required**
    /// Identifies a route (`routes.route_id`). One route can only belong to one network.
    pub route_id: String,
}
impl GTFSRouteNetwork {
    /// Create a new GTFSRouteNetwork
    pub fn new(source: &str) -> Vec<GTFSRouteNetwork> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSRouteNetwork>(source, None, None) {
            res.push(record);
        }
        res
    }
}
