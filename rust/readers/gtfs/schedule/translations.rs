use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// # Translations
///
/// **Optional**
/// Provides language-specific translations for text fields in various GTFS tables.
/// Each row defines a single translation for a specific field in a specific language,
/// either targeting a specific record (and possibly sub-record) or matching by field value.
///
/// **Primary Key**: (table_name, field_name, language, record_id, record_sub_id, field_value)
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSTranslation {
    /// **Required**
    /// The table containing the field to be translated.
    ///
    /// Allowed values (official spec):
    /// - "agency"
    /// - "stops"
    /// - "routes"
    /// - "trips"
    /// - "stop_times"
    /// - "pathways"
    /// - "levels"
    /// - "feed_info"
    /// - "attributions"
    ///
    /// Other optional files (calendar, shapes, etc.) may appear for unofficial field translations.
    pub table_name: String,
    /// **Required**
    /// Name of the field within the table that is being translated.
    /// Typically text, URL, phone, or email fields.
    pub field_name: String,
    /// **Required**
    /// ISO language code (e.g., "en", "fr", "mul") for this translation.
    pub language: String,
    /// **Required**
    /// The translated value, matching the type of the original field (text, URL, phone, email).
    pub translation: String,
    /// **Conditionally Required**
    /// Identifies the primary key of the record in the table if the table has a unique ID
    /// (e.g., `agency_id`, `stop_id`, `route_id`, `trip_id`, `pathway_id`, `level_id`, `attribution_id`).
    /// Required unless `fieldValue` is used or if `tableName=feed_info`.
    pub record_id: Option<String>,
    /// **Conditionally Required**
    /// Secondary key if the table doesn’t have a single unique ID (e.g., stop_sequence for stop_times).
    /// Required if `recordId` is used and `tableName=stop_times`. Forbidden otherwise.
    pub record_sub_id: Option<String>,
    /// **Conditionally Required**
    /// The exact field value to match for translation if `recordId` and `recordSubId` are not used.
    /// Forbidden if `recordId` is defined or if `tableName=feed_info`.
    pub field_value: Option<String>,
}
impl GTFSTranslation {
    /// Create a new GTFSTranslation
    pub fn new(source: &str) -> Vec<GTFSTranslation> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSTranslation>(source, None, None) {
            res.push(record);
        }
        res
    }
}
