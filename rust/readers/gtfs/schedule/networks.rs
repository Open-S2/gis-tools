use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Networks
///
/// **Conditionally Forbidden**
/// Defines network identifiers. Used to group routes under a named network
/// for fare leg rules. This file is forbidden if `network_id` exists in `routes.txt`,
/// otherwise optional.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSNetwork {
    /// **Required**
    /// Identifies a network (`network_id`). Must be unique in `networks.txt`.
    pub network_id: String,
    /// **Optional**
    /// The name of the network as used by the local agency and its riders.
    pub network_name: Option<String>,
}
impl GTFSNetwork {
    /// Create a new GTFSNetwork
    pub fn new(source: &str) -> BTreeMap<String, GTFSNetwork> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSNetwork>(source, None, None) {
            res.insert(record.network_id.clone(), record);
        }
        res
    }
}
