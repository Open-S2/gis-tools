use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Levels
///
/// **Conditionally Required**
/// Describes levels in a station, useful with `pathways.txt`.
/// Required if `pathways` include elevators (`pathway_mode=5`), otherwise optional.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSLevel {
    /// **Required**
    /// Identifies a level in a station (`level_id`).
    pub level_id: String,
    /// **Required**
    /// Numeric index indicating this level's relative position:
    /// - 0 for ground level
    /// - Positive above ground
    /// - Negative below ground
    pub level_index: i32,
    /// **Optional**
    /// Name of the level as displayed to the rider (e.g., "Mezzanine", "Platform").
    pub level_name: Option<String>,
}
impl GTFSLevel {
    /// Create a new GTFSLevel
    pub fn new(source: &str) -> BTreeMap<String, GTFSLevel> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSLevel>(source, None, None) {
            res.insert(record.level_id.clone(), record);
        }
        res
    }
}
