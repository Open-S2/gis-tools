use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Describes the type of fare media used.
/// 0 - None
/// 1 - Physical paper ticket
/// 2 - Physical transit card
/// 3 - cEMV (contactless)
/// 4 - Mobile app
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GTFSFareMediaType {
    /// 0 - None
    None = 0,
    /// 1 - Physical paper ticket
    PhysicalPaperTicket = 1,
    /// 2 - Physical transit card
    PhysicalTransitCard = 2,
    /// 3 - cEMV (contactless)
    CEMV = 3,
    /// 4 - Mobile app
    MobileApp = 4,
}
impl From<u8> for GTFSFareMediaType {
    fn from(value: u8) -> Self {
        match value {
            1 => GTFSFareMediaType::PhysicalPaperTicket,
            2 => GTFSFareMediaType::PhysicalTransitCard,
            3 => GTFSFareMediaType::CEMV,
            4 => GTFSFareMediaType::MobileApp,
            _ => GTFSFareMediaType::None,
        }
    }
}

/// # Fare Media
///
/// **Optional**
/// Describes physical or virtual holders used for the representation and validation of a fare product.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareMedia {
    /// **Required**
    /// Identifies a fare media (`fare_media_id`).
    pub fare_media_id: String,
    /// **Optional**
    /// Rider-facing name for this fare media.
    pub fare_media_name: Option<String>,
    /// **Required**
    /// Type of fare media. One of:
    /// - 0 = None
    /// - 1 = Physical paper ticket
    /// - 2 = Physical transit card
    /// - 3 = cEMV (contactless)
    /// - 4 = Mobile app
    pub fare_media_type: u8,
}
impl GTFSFareMedia {
    /// Create a new GTFSFareMedia
    pub fn new(source: &str) -> BTreeMap<String, GTFSFareMedia> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSFareMedia>(source, None, None) {
            res.insert(record.fare_media_id.clone(), record);
        }
        res
    }
    /// Get the media type
    pub fn fare_media_type(&self) -> GTFSFareMediaType {
        self.fare_media_type.into()
    }
}
