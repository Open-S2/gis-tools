use crate::readers::csv::parse_csv_as_record;
use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// Pickup method.
/// - 0 or empty = Regularly scheduled pickup
/// - 1 = No pickup available
/// - 2 = Must phone agency to arrange pickup
/// - 3 = Must coordinate with driver to arrange pickup
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSPickupDropOffType {
    /// Regularly scheduled pickup/drop off
    Regular = 0,
    /// No pickup/drop off available
    None = 1,
    /// Must phone agency to arrange pickup/drop off
    PhoneAgency = 2,
    /// Must coordinate with driver to arrange pickup/drop off
    CoordinateDriver = 3,
}
impl From<i8> for GTFSPickupDropOffType {
    fn from(value: i8) -> Self {
        match value {
            0 => GTFSPickupDropOffType::Regular,
            2 => GTFSPickupDropOffType::PhoneAgency,
            3 => GTFSPickupDropOffType::CoordinateDriver,
            _ => GTFSPickupDropOffType::None,
        }
    }
}

/// Continuous pickup behavior from this stop_time to the next.
/// - 0 = Continuous stopping pickup/drop off
/// - 1 or empty = No continuous stopping pickup/drop off
/// - 2 = Must phone agency
/// - 3 = Must coordinate with driver
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSContinuousPickupDropOff {
    /// Continuous stopping pickup
    Continuous = 0,
    /// No continuous stopping pickup
    None = 1,
    /// Must phone agency
    PhoneAgency = 2,
    /// Must coordinate with driver
    CoordinateDriver = 3,
}
impl From<i8> for GTFSContinuousPickupDropOff {
    fn from(value: i8) -> Self {
        match value {
            0 => GTFSContinuousPickupDropOff::Continuous,
            2 => GTFSContinuousPickupDropOff::PhoneAgency,
            3 => GTFSContinuousPickupDropOff::CoordinateDriver,
            _ => GTFSContinuousPickupDropOff::None,
        }
    }
}

/// Indicates if arrival/departure times are exact or approximate.
/// - 0 = Approximate times
/// - 1 = Exact times
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSTimepoint {
    /// Approximate times
    Approximate = 0,
    /// Exact times
    Exact = 1,
}
impl From<i8> for GTFSTimepoint {
    fn from(value: i8) -> Self {
        match value {
            0 => GTFSTimepoint::Approximate,
            _ => GTFSTimepoint::Exact,
        }
    }
}

/// # Stop Time Information
///
/// **Required** - Times that a vehicle arrives at and departs from stops for each trip.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSStopTime {
    /// **Required**
    /// Identifies a trip (`trips.trip_id`).
    pub trip_id: String,
    /// **Conditionally Required**
    /// Arrival time at the stop in HH:MM:SS (local) or possibly > 24:00:00 after midnight.
    /// Required for the first/last stop of the trip or if `timepoint=1`.
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
    pub arrival_time: Option<String>,
    /// **Conditionally Required**
    /// Departure time at the stop in HH:MM:SS (local) or possibly > 24:00:00 after midnight.
    /// Required if `timepoint=1`.
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
    pub departure_time: Option<String>,
    /// **Conditionally Required**
    /// References a stop (`stops.stop_id`). Must be a location_type of 0 or empty.
    /// Required if neither `location_group_id` nor `location_id` is used.
    /// Forbidden if `location_group_id` or `location_id` is defined.
    pub stop_id: Option<String>,
    /// **Conditionally Forbidden**
    /// References a location group (`location_groups.location_group_id`).
    /// Forbidden if `stop_id` or `location_id` is defined.
    pub location_group_id: Option<String>,
    /// **Conditionally Forbidden**
    /// References a GeoJSON location ID (`locations.geojson`).
    /// Forbidden if `stop_id` or `location_group_id` is defined.
    pub location_id: Option<String>,
    /// **Required**
    /// Order of stops (or location groups, or GeoJSON locations) for this trip.
    /// Must increase along the trip, but need not be consecutive.
    pub stop_sequence: usize,
    /// **Optional**
    /// Overrides the trip’s headsign at this specific stop.
    pub stop_headsign: Option<String>,
    /// **Conditionally Required**
    /// Time on-demand service becomes available at this location/stop/location group.
    /// Required if `end_pickup_drop_off_window` is defined, or if `location_group_id` or `location_id` is used.
    /// Forbidden if `arrival_time` or `departure_time` is defined.
    pub start_pickup_drop_off_window: Option<String>,
    /// **Conditionally Required**
    /// Time on-demand service ends at this location/stop/location group.
    /// Required if `start_pickup_drop_off_window` is defined, or if `location_group_id` or `location_id` is used.
    /// Forbidden if `arrival_time` or `departure_time` is defined.
    pub end_pickup_drop_off_window: Option<String>,
    /// **Conditionally Forbidden**
    /// Pickup method:
    /// 0 or empty = Regular, 1 = None, 2 = Phone Agency, 3 = Coordinate with Driver
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined (for 0 or 3).
    pub pickup_type: Option<i8>, // ?: GTFSPickupType;
    /// **Conditionally Forbidden**
    /// Drop-off method:
    /// 0 or empty = Regular, 1 = None, 2 = Phone Agency, 3 = Coordinate with Driver
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined (for 0).
    pub drop_off_type: Option<i8>, // ?: GTFSDropOffType;
    /// **Conditionally Forbidden**
    /// Continuous pickup from this stop_time to the next.
    /// 0 = Continuous, 1 or empty = None, 2 = Phone Agency, 3 = Coordinate with Driver
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
    pub continuous_pickup: Option<i8>, // ?: ContinuousPickup;
    /// **Conditionally Forbidden**
    /// Continuous drop-off from this stop_time to the next.
    /// 0 = Continuous, 1 or empty = None, 2 = Phone Agency, 3 = Coordinate with Driver
    /// Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
    pub continuous_drop_off: Option<i8>, // ?: ContinuousDropOff;
    /// **Optional**
    /// Distance traveled along the associated shape from the first stop to this record’s stop.
    /// Must be in the same units used in shapes.txt.
    pub shape_dist_traveled: Option<usize>,
    /// **Optional**
    /// 0 = Times are approximate, 1 = Times are exact.
    pub timepoint: Option<i8>, // ?: Timepoint;
    /// **Optional**
    /// Boarding booking rule reference (`booking_rules.booking_rule_id`).
    /// Recommended if `pickup_type=2`.
    pub pickup_booking_rule_id: Option<String>,
    /// **Optional**
    /// Alighting booking rule reference (`booking_rules.booking_rule_id`).
    /// Recommended if `drop_off_type=2`.
    pub drop_off_booking_rule_id: Option<String>,
}
impl GTFSStopTime {
    /// Create a new GTFSStopTime
    pub fn new(source: &str) -> Vec<GTFSStopTime> {
        let mut res = Vec::new();
        for record in parse_csv_as_record::<GTFSStopTime>(source, None, None) {
            res.push(record);
        }
        res
    }
    /// Get the pickup_type
    pub fn pickup_type(&self) -> Option<GTFSPickupDropOffType> {
        self.pickup_type.map(GTFSPickupDropOffType::from)
    }
    /// Get the drop_off_type
    pub fn drop_off_type(&self) -> Option<GTFSPickupDropOffType> {
        self.drop_off_type.map(GTFSPickupDropOffType::from)
    }
    /// Get the continuous_pickup
    pub fn continuous_pickup(&self) -> Option<GTFSContinuousPickupDropOff> {
        self.continuous_pickup.map(GTFSContinuousPickupDropOff::from)
    }
    /// Get the continuous_drop_off
    pub fn continuous_drop_off(&self) -> Option<GTFSContinuousPickupDropOff> {
        self.continuous_drop_off.map(GTFSContinuousPickupDropOff::from)
    }
    /// Get the timepoint
    pub fn timepoint(&self) -> Option<GTFSTimepoint> {
        self.timepoint.map(GTFSTimepoint::from)
    }
}
