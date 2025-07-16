use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// TransferType enumerates how a rider can transfer between routes/trips/stops:
/// - 0 or empty = Recommended transfer
/// - 1 = Timed transfer
/// - 2 = Requires a minimum transfer time
/// - 3 = Transfers not possible
/// - 4 = In-seat transfer (stay onboard, same vehicle)
/// - 5 = In-seat transfers not allowed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSTransferType {
    /// 0 = Recommended transfer
    Recommended = 0,
    /// 1 = Timed transfer
    Timed = 1,
    /// 2 = Requires a minimum transfer time
    MinTimeRequired = 2,
    /// 3 = Transfers not possible
    NotPossible = 3,
    /// 4 = In-seat transfer (stay onboard, same vehicle)
    InSeatTransfer = 4,
    /// 5 = In-seat transfers not allowed
    InSeatNotAllowed = 5,
}
impl From<i8> for GTFSTransferType {
    fn from(value: i8) -> Self {
        match value {
            1 => GTFSTransferType::Timed,
            2 => GTFSTransferType::MinTimeRequired,
            3 => GTFSTransferType::NotPossible,
            4 => GTFSTransferType::InSeatTransfer,
            5 => GTFSTransferType::InSeatNotAllowed,
            _ => GTFSTransferType::Recommended,
        }
    }
}

/// # Transfers
///
/// **Optional**
/// Defines additional rules/overrides for transfers between routes/trips/stops.
/// The level of specificity is determined by which fields are present:
/// - from_trip_id & to_trip_id (most specific)
/// - route vs. trip combos
/// - only from_stop_id & to_stop_id (least specific)
///
/// **Primary Key**: (from_stop_id, to_stop_id, from_trip_id, to_trip_id, from_route_id, to_route_id)
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSTransfer {
    /// **Conditionally Required**
    /// Identifies where a connection begins (`stops.stop_id`, location_type=0 or 1).
    /// Required if transfer_type is 1, 2, or 3. Optional if transfer_type is 4 or 5.
    pub from_stop_id: Option<String>,
    /// **Conditionally Required**
    /// Identifies where a connection ends (`stops.stop_id`, location_type=0 or 1).
    /// Required if transfer_type is 1, 2, or 3. Optional if transfer_type is 4 or 5.
    pub to_stop_id: Option<String>,
    /// **Optional**
    /// Identifies a route on which the arriving trip is running.
    /// If both `fromTripId` and `fromRouteId` are defined, the trip must belong to that route,
    /// but `fromTripId` takes precedence.
    pub from_route_id: Option<String>,
    /// **Optional**
    /// Identifies a route on which the departing trip is running.
    /// If both `toTripId` and `toRouteId` are defined, the trip must belong to that route,
    /// but `toTripId` takes precedence.
    pub to_route_id: Option<String>,
    /// **Conditionally Required**
    /// Identifies the arriving trip (`trips.trip_id`).
    /// Required if transfer_type is 4 or 5; optional otherwise.
    pub from_trip_id: Option<String>,
    /// **Conditionally Required**
    /// Identifies the departing trip (`trips.trip_id`).
    /// Required if transfer_type is 4 or 5; optional otherwise.
    pub to_trip_id: Option<String>,
    /// **Required**
    /// Indicates the type of connection:
    /// - 0 = Recommended
    /// - 1 = Timed
    /// - 2 = Requires min_transfer_time
    /// - 3 = Not possible
    /// - 4 = In-seat transfer
    /// - 5 = In-seat transfer not allowed
    pub transfer_type: i8,
    /// **Optional**
    /// Time in seconds required for a rider to complete the transfer. If `transfer_type=2`,
    /// this is the minimum transfer time.
    pub min_transfer_time: Option<u32>,
}
impl GTFSTransfer {
    /// Create a new GTFSTransfer
    pub fn new(source: &str) -> Vec<GTFSTransfer> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSTransfer>(source, None, None) {
            res.push(record);
        }
        res
    }
    /// Get the transfer_type
    pub fn transfer_type(&self) -> GTFSTransferType {
        GTFSTransferType::from(self.transfer_type)
    }
}
