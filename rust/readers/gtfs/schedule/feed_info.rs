use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// # Feed Information
///
/// **Conditionally Required**
/// Contains information about the dataset itself (publisher, version, etc.).
/// - Required if `translations.txt` is used.
/// - Recommended otherwise.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFeedInfo {
    /// **Required**
    /// Full name of the organization that publishes the dataset.
    pub feed_publisher_name: String,
    /// **Required**
    /// URL of the dataset publisher's website.
    pub feed_publisher_url: String,
    /// **Required**
    /// Default language code for the text in this dataset.
    /// For multilingual datasets, use "mul" and translations.txt for further detail.
    pub feed_lang: String,
    /// **Optional**
    /// Language used if the consumer does not know the rider’s language, often "en".
    pub default_lang: Option<String>,
    /// **Recommended**
    /// First date of service the dataset covers, in `YYYYMMDD` format.
    pub feed_start_date: Option<String>,
    /// **Recommended**
    /// Last date of service the dataset covers, in `YYYYMMDD` format.
    /// Must not precede `feed_start_date` if both are given.
    pub feed_end_date: Option<String>,
    /// **Recommended**
    /// Current version identifier for this GTFS dataset.
    pub feed_version: Option<String>,
    /// **Optional**
    /// Email address for technical contact about the dataset.
    pub feed_contact_email: Option<String>,
    /// **Optional**
    /// URL for technical contact or support form regarding the dataset.
    pub feed_contact_url: Option<String>,
}
impl GTFSFeedInfo {
    /// Create a new GTFSFeedInfo
    pub fn new(source: &str) -> BTreeMap<String, GTFSFeedInfo> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSFeedInfo>(source, None, None) {
            res.insert(record.feed_publisher_name.clone(), record);
        }
        res
    }
}
