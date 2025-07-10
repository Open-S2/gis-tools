use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Agency Information
///
/// ## Details
/// **Required** - Transit agencies with service represented in this dataset.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSAgency {
    /// **Required**
    /// Identifies a location: stop/platform, station, entrance/exit, generic node or boarding area (see location_type).
    /// ID must be unique across all stops. `stop_id`, locations.geojson id, and location_groups.location_group_id values.
    /// Multiple routes may use the same `stop_id`.
    pub agency_id: String,
    /// **Required**
    /// Full name of the transit agency.
    pub agency_name: String,
    /// **Required**
    /// URL of the transit agency.
    pub agency_url: String,
    /// **Required**
    /// Timezone where the transit agency is located.
    /// If multiple agencies are specified in the dataset, each must have the same `agency_timezone`.
    pub agency_timezone: String,
    /// **Optional**
    /// Primary language used by this transit agency.
    /// Should be provided to help GTFS consumers choose capitalization rules and other language-specific settings for the dataset.
    /// See [ISO 639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) for language codes.
    pub agency_lang: Option<String>,
    /// **Optional**
    /// A voice telephone number for the specified agency.
    /// This field is a string value that presents the telephone number as typical for the agency's service area.
    pub agency_phone: Option<String>,
    /// **Optional**
    /// URL of a web page that allows a rider to purchase tickets or other fare instruments for that
    /// agency online.
    pub agency_fare_url: Option<String>,
    /// **Optional**
    /// Email address actively monitored by the agency’s customer service department. This email
    /// address should be a direct contact point where transit riders can reach a customer service
    /// representative at the agency.
    pub agency_email: Option<String>,
}
impl GTFSAgency {
    /// Create a new GTFSAgency
    pub fn new(source: &str) -> BTreeMap<String, GTFSAgency> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSAgency>(source, None, None) {
            res.insert(record.agency_id.clone(), record);
        }
        res
    }
}
