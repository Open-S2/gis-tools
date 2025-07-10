use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Areas
///
/// **Optional**
/// Defines area identifiers.
/// Each record in `areas.txt` contains a unique `area_id` that can be referenced
/// in `stop_areas.txt`.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSArea {
    /// **Required**
    /// Identifies an area (`area_id`). Must be unique within `areas.txt`.
    pub area_id: String,
    /// **Optional**
    /// Name of the area as displayed to the rider.
    pub area_name: Option<String>,
}
impl GTFSArea {
    /// Create a new GTFSArea
    pub fn new(source: &str) -> BTreeMap<String, GTFSArea> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSArea>(source, None, None) {
            res.insert(record.area_id.clone(), record);
        }
        res
    }
}
