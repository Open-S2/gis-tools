use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Stop Areas
///
/// **Optional**
/// Assigns stops to areas. Multiple rows can reference the same `area_id` to
/// indicate that different stops belong to the same area. Conversely, a single
/// `stop_id` can appear in multiple areas if needed.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSStopArea {
    /// **Required**
    /// Identifies an area (`areas.area_id`).
    pub area_id: String,
    /// **Required**
    /// Identifies a stop (`stops.stop_id`). If a station is defined (location_type=1),
    /// it implies all its child platforms (location_type=0) also belong to this area,
    /// unless otherwise assigned.
    pub stop_id: String,
}
impl GTFSStopArea {
    /// Create a new GTFSStopArea
    pub fn new(source: &str) -> Vec<GTFSStopArea> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSStopArea>(source, None, None) {
            res.push(record);
        }
        res
    }
}
