use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Location Groups
///
/// **Optional**
/// Defines groups of stops where a rider may request pickup or drop off.
/// `location_group_id` must be unique across:
/// - stops.stop_id
/// - locations.geojson ID
/// - location_groups.location_group_id
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSLocationGroup {
    /// **Required**
    /// Identifies a location group. Must be unique (e.g., "zoneA", "northSideGroup").
    location_group_id: String,
    /// **Optional**
    /// The name of the location group as displayed to the rider.
    location_group_name: Option<String>,
}
impl GTFSLocationGroup {
    /// Create a new GTFSLocationGroup
    pub fn new(source: &str) -> BTreeMap<String, GTFSLocationGroup> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSLocationGroup>(source, None, None) {
            res.insert(record.location_group_id.clone(), record);
        }
        res
    }
}
