use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Indicates the direction of travel for a trip. This field should not be used in routing; it provides a way to separate trips by direction when publishing time tables. Valid options are:
/// - 0 - Travel in one direction (e.g. outbound travel).
/// - 1 - Travel in the opposite direction (e.g. inbound travel).
///
/// Example: The trip_headsign and direction_id fields may be used together to assign a name to travel in each direction for a set of trips. A trips.txt file could contain these records for use in time tables:
///
/// ```csv
/// trip_id,...,trip_headsign,direction_id
/// 1234,...,Airport,0
/// 1505,...,Downtown,1
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSDirectionId {
    /// Outbound, e.g., "Airport"
    #[default]
    Outbound = 0, // e.g., "Airport"
    /// Inbound, e.g., "Downtown"
    Inbound = 1, // e.g., "Downtown"
}
impl From<i8> for GTFSDirectionId {
    fn from(value: i8) -> Self {
        match value {
            1 => GTFSDirectionId::Inbound,
            _ => GTFSDirectionId::Outbound,
        }
    }
}

/// Indicates wheelchair accessibility. Valid options are:
/// - 0 or empty - No accessibility information for the trip.
/// - 1 - Vehicle being used on this particular trip can accommodate at least one rider in a wheelchair.
/// - 2 - No riders in wheelchairs can be accommodated on this trip.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSWheelchairAccessibility {
    /// Or empty
    #[default]
    NoInfo = 0, // or empty
    /// At least one wheelchair
    Accessible = 1, // at least one wheelchair space
    /// No wheelchair
    NotAccessible = 2, // no wheelchair accommodation
}
impl From<i8> for GTFSWheelchairAccessibility {
    fn from(value: i8) -> Self {
        match value {
            1 => GTFSWheelchairAccessibility::Accessible,
            2 => GTFSWheelchairAccessibility::NotAccessible,
            _ => GTFSWheelchairAccessibility::NoInfo,
        }
    }
}

/// Indicates whether bikes are allowed. Valid options are:
/// - 0 or empty - No bike information for the trip.
/// - 1 - Vehicle being used on this particular trip can accommodate at least one bicycle.
/// - 2 - No bicycles are allowed on this trip.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSBikesAllowed {
    /// Or empty
    #[default]
    NoInfo = 0,
    /// at least one bicycle can be accommodated
    Allowed = 1,
    /// no bicycles allowed
    NotAllowed = 2,
}
impl From<i8> for GTFSBikesAllowed {
    fn from(value: i8) -> Self {
        match value {
            1 => GTFSBikesAllowed::Allowed,
            2 => GTFSBikesAllowed::NotAllowed,
            _ => GTFSBikesAllowed::NoInfo,
        }
    }
}

/// # Trip Information
///
/// ## Details
/// **Required** - Trips for each route. A trip is a sequence of two or more stops that occur during
/// a specific time period.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSTrip {
    /// **Required**
    /// Identifies which route this trip belongs to (`routes.route_id`).
    pub route_id: String,
    /// **Required**
    /// Identifies a set of dates when service is available (`calendar.service_id` or `calendar_dates.service_id`).
    pub service_id: String,
    /// **Required**
    /// Unique identifier for a trip (`trip_id`).
    pub trip_id: String,
    /// **Optional**
    /// Text that appears on signage identifying the trip's destination to riders. This field is
    /// recommended for all services with headsign text displayed on the vehicle which may be used
    /// to distinguish amongst trips in a route.
    ///
    /// If the headsign changes during a trip, values for trip_headsign may be overridden by
    /// defining values in stop_times.stop_headsign for specific stop_times along the trip.
    pub trip_headsign: Option<String>,
    /// **Optional**
    /// Public-facing text used to identify the trip (e.g., train numbers).
    pub trip_short_name: Option<String>,
    /// **Optional**
    /// Updated to use an enum for direction.
    /// 0 = Outbound, 1 = Inbound.
    pub direction_id: Option<i8>,
    /// **Optional**
    /// Identifies the block this trip belongs to. Sequential trips with the same block_id typically
    /// use the same vehicle.
    pub block_id: Option<String>,
    /// **Conditionally Required**
    /// References a geospatial shape describing the vehicle's travel path (`shapes.shape_id`).
    /// Required if the trip uses continuous pickup or drop-off rules; otherwise optional.
    pub shape_id: Option<String>,
    /// **Optional**
    /// Updated to use an enum for wheelchair accessibility.
    /// 0 = NoInfo, 1 = Accessible, 2 = NotAccessible.
    pub wheelchair_accessible: Option<i8>,
    /// **Optional**
    /// Updated to use an enum for bikes allowed.
    /// 0 = NoInfo, 1 = Allowed, 2 = NotAllowed.
    pub bikes_allowed: Option<i8>,
}
impl GTFSTrip {
    /// Create a new GTFSTrip
    pub fn new(source: &str) -> BTreeMap<String, GTFSTrip> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSTrip>(source, None, None) {
            res.insert(record.trip_id.clone(), record);
        }
        res
    }
    /// Get the direction_id
    pub fn direction_id(&self) -> Option<GTFSDirectionId> {
        self.direction_id.map(GTFSDirectionId::from)
    }
    /// Get the wheelchair_accessible
    pub fn wheelchair_accessible(&self) -> Option<GTFSWheelchairAccessibility> {
        self.wheelchair_accessible.map(GTFSWheelchairAccessibility::from)
    }
    /// Get the bikes_allowed
    pub fn bikes_allowed(&self) -> Option<GTFSBikesAllowed> {
        self.bikes_allowed.map(GTFSBikesAllowed::from)
    }
}
