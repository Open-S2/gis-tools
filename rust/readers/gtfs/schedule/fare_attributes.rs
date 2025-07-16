use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

// NOTE:
// The files associated with GTFS-Fares V1 are:
// - fare_attributes.txt
// - fare_rules.txt
//
// The files associated with GTFS-Fares V2 are:
// - fare_media.txt
// - fare_products.txt
// - fare_leg_rules.txt
// - fare_transfer_rules.txt

/// Indicates when the fare must be paid:
/// - 0 = On board
/// - 1 = Before boarding
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GTFSPaymentMethod {
    /// 0 = On board
    OnBoard = 0,
    /// 1 = Before boarding
    PreBoard = 1,
}
impl From<i8> for GTFSPaymentMethod {
    fn from(s: i8) -> Self {
        match s {
            1 => GTFSPaymentMethod::PreBoard,
            _ => GTFSPaymentMethod::OnBoard,
        }
    }
}

/// Transfers can be:
/// - 0 = No transfers permitted
/// - 1 = One transfer
/// - 2 = Two transfers
/// - '' (empty string) = Unlimited transfers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GTFSTransfersType {
    /// 0 = No transfers permitted
    NoTransfers = 0,
    /// 1 = One transfer
    OneTransfer = 1,
    /// 2 = Two transfers
    TwoTransfers = 2,
    /// '' (empty string) = Unlimited transfers
    UnlimitedTransfers,
}
impl From<&str> for GTFSTransfersType {
    fn from(s: &str) -> Self {
        match s {
            "1" => GTFSTransfersType::OneTransfer,
            "2" => GTFSTransfersType::TwoTransfers,
            "" => GTFSTransfersType::UnlimitedTransfers,
            _ => GTFSTransfersType::NoTransfers,
        }
    }
}

/// # Fare Attributes (GTFS-Fares V1)
///
/// **Optional** - But required if using GTFS-Fares V1 approach.
/// Defines basic fare information such as price, currency, and transfer limits.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSFareAttribute {
    /// **Required**
    /// Identifies a fare class.
    pub fare_id: String,
    /// **Required**
    /// Fare price in the currency specified by `currencyType`.
    pub price: f64,
    /// **Required**
    /// Currency code (e.g., "USD", "EUR").
    pub currency_type: String,
    /// **Required**
    /// When the fare must be paid.
    /// - 0 = Paid on board
    /// - 1 = Must be paid before boarding
    pub payment_method: i8,
    /// **Required**
    /// Number of transfers permitted on this fare.
    /// - 0 = No transfers
    /// - 1 = One transfer
    /// - 2 = Two transfers
    /// - '' (empty) = Unlimited transfers
    pub transfers: String,
    /// **Conditionally Required**
    /// Agency for the specified fare.
    /// Required if multiple agencies exist in `agency.txt`.
    pub agency_id: Option<String>,
    /// **Optional**
    /// Length of time in seconds before a transfer (or this fare) expires.
    /// When transfers=0, may indicate ticket validity duration or be empty.
    pub transfer_duration: Option<i32>,
}
impl GTFSFareAttribute {
    /// Create a new GTFSFareAttribute
    pub fn new(source: &str) -> BTreeMap<String, GTFSFareAttribute> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSFareAttribute>(source, None, None) {
            res.insert(record.fare_id.clone(), record);
        }
        res
    }
    /// Get the payment type
    pub fn payment_method(&self) -> GTFSPaymentMethod {
        self.payment_method.into()
    }
    /// Get the transfers type
    pub fn transfers(&self) -> GTFSTransfersType {
        self.transfers.as_str().into()
    }
}
