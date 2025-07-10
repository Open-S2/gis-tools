use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Location Group Stops
///
/// **Optional**
/// Assigns stops from `stops.txt` to location groups (`location_groups.txt`).
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSLocationGroupStop {
    /// **Required**
    /// Identifies a location group (`location_groups.location_group_id`).
    pub location_group_id: String,
    /// **Required**
    /// Identifies a stop (`stops.stop_id`) belonging to that location group.
    pub stop_id: String,
}
impl GTFSLocationGroupStop {
    /// Create a new GTFSLocationGroupStop
    pub fn new(source: &str) -> Vec<GTFSLocationGroupStop> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSLocationGroupStop>(source, None, None) {
            res.push(record);
        }
        res
    }
}
