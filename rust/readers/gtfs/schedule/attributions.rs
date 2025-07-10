use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Attributions
///
/// **Optional**
/// Defines the attributions applied to the dataset or parts of it.
/// If `agency_id`, `route_id`, or `trip_id` is specified, the attribution
/// applies only to that entity. If none are specified, the attribution
/// applies to the entire dataset.
///
/// **Primary Key**: (attribution_id) - optional
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSAttribution {
    /// **Optional**
    /// Unique ID that identifies this attribution record.
    /// Useful if multiple attributions exist or for referencing translations.
    pub attribution_id: Option<String>,
    /// **Optional**
    /// Agency to which this attribution applies (`agency.agency_id`).
    /// Must be empty if route_id or trip_id are specified.
    pub agency_id: Option<String>,
    /// **Optional**
    /// Route to which this attribution applies (`routes.route_id`).
    /// Must be empty if agency_id or trip_id are specified.
    pub route_id: Option<String>,
    /// **Optional**
    /// Trip to which this attribution applies (`trips.trip_id`).
    /// Must be empty if agency_id or route_id are specified.
    pub trip_id: Option<String>,
    /// **Required**
    /// Organization name to which the dataset is attributed.
    pub organization_name: String,
    /// **Optional**
    /// 0 or empty = Not a producer, 1 = Is a producer
    pub is_producer: Option<String>,
    /// **Optional**
    /// 0 or empty = Not an operator, 1 = Is an operator
    pub is_operator: Option<String>,
    /// **Optional**
    /// 0 or empty = Not an authority, 1 = Is an authority
    pub is_authority: Option<String>,
    /// **Optional**
    /// URL of the organization.
    pub attribution_url: Option<String>,
    /// **Optional**
    /// Email of the organization.
    pub attribution_email: Option<String>,
    /// **Optional**
    /// Phone number of the organization.
    pub attribution_phone: Option<String>,
}
impl GTFSAttribution {
    /// Create a new GTFSAttribution
    pub fn new(source: &str) -> BTreeMap<String, GTFSAttribution> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSAttribution>(source, None, None) {
            res.insert(record.organization_name.clone(), record);
        }
        res
    }
    /// Check if is_producer is true
    pub fn is_producer(&self) -> bool {
        self.is_producer == Some("1".into())
    }
    /// Check if is_operator is true
    pub fn is_operator(&self) -> bool {
        self.is_operator == Some("1".into())
    }
    /// Check if is_authority is true
    pub fn is_authority(&self) -> bool {
        self.is_authority == Some("1".into())
    }
}
