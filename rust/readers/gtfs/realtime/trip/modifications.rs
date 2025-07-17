#![cfg_attr(feature = "nightly", coverage(off))]
// NOTE: I can't find data that actually uses this and most modules don't support so.
// This is an experimental module, exists incase the GTFS-Realtime spec changes to include it

use crate::util::Date;
use alloc::{string::String, vec::Vec};
use pbf::{ProtoRead, Protobuf};

/// NOTE: This field is still experimental, and subject to change. It may be formally adopted in
/// the future.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTripModifications {
    /// A list of selected trips affected by this TripModifications.
    pub selected_trips: Vec<GTFSRealtimeSelectedTrips>, // 1 [repeated message]
    /// A list of start times in the real-time trip descriptor for the trip_id defined in trip_ids.
    /// Useful to target multiple departures of a trip_id in a frequency-based trip.
    pub start_times: Vec<String>, // 2 [repeated string]
    /// Dates on which the modifications occurs, in the YYYYMMDD format. Producers SHOULD only
    /// transmit detours occurring within the next week.
    /// The dates provided should not be used as user-facing information, if a user-facing start and
    /// end date needs to be provided, they can be provided in the linked service alert with `service_alert_id`
    pub service_dates: Vec<String>, // 3 [repeated string]
    /// A list of modifications to apply to the affected trips.
    pub modifications: Vec<GTFSRealtimeModification>, // 4 [repeated message]
}
/// Read in the contents of the GTFSRealtimeTripModifications
impl ProtoRead for GTFSRealtimeTripModifications {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut selected_trip = GTFSRealtimeSelectedTrips::default();
                pb.read_message(&mut selected_trip);
                self.selected_trips.push(selected_trip);
            }
            2 => self.start_times.push(pb.read_string()),
            3 => self.service_dates.push(pb.read_string()),
            4 => {
                let mut modification = GTFSRealtimeModification::default();
                pb.read_message(&mut modification);
                self.modifications.push(modification);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// A `Modification` message replaces a span of n stop times from each affected trip starting at
/// `start_stop_selector`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeModification {
    /// The stop selector of the first stop_time of the original trip that is to be affected by this modification.
    /// Used in conjuction with `end_stop_selector`.
    /// `start_stop_selector` is required and is used to define the reference stop used with `travel_time_to_stop`.
    pub start_stop_selector: Option<GTFSRealtimeStopSelector>, // 1 [message]
    /// The stop selector of the last stop of the original trip that is to be affected by this modification.
    /// The selection is inclusive, so if only one stop_time is replaced by that modification, `start_stop_selector`
    /// and `end_stop_selector` must be equivalent.
    /// If no stop_time is replaced, `end_stop_selector` must not be provided. It's otherwise required.
    pub end_stop_selector: Option<GTFSRealtimeStopSelector>, // 2 [message]
    /// The number of seconds of delay to add to all departure and arrival times following the end of this modification.
    /// If multiple modifications apply to the same trip, the delays accumulate as the trip advances.
    pub propagated_modification_delay: i32, // 3 [int32]
    /// A list of replacement stops, replacing those of the original trip.
    /// The length of the new stop times may be less, the same, or greater than the number of replaced stop times.
    pub replacement_stops: Vec<GTFSRealtimeReplacementStop>, // 4 [repeated message]
    /// An `id` value from the `FeedEntity` message that contains the `Alert` describing this Modification
    /// for user-facing communication.
    pub service_alert_id: Option<String>, // 5 [string]
    /// This timestamp identifies the moment when the modification has last been changed.
    /// In POSIX time (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
    pub last_modified_time: Option<Date>, // 6 [uint64]
}
/// Read in the contents of the GTFSRealtimeModification
impl ProtoRead for GTFSRealtimeModification {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut stop_selector = GTFSRealtimeStopSelector::default();
                pb.read_message(&mut stop_selector);
                self.start_stop_selector = Some(stop_selector);
            }
            2 => {
                let mut stop_selector = GTFSRealtimeStopSelector::default();
                pb.read_message(&mut stop_selector);
                self.end_stop_selector = Some(stop_selector);
            }
            3 => self.propagated_modification_delay = pb.read_varint(),
            4 => {
                let mut replacement_stop = GTFSRealtimeReplacementStop::default();
                pb.read_message(&mut replacement_stop);
                self.replacement_stops.push(replacement_stop);
            }
            5 => self.service_alert_id = Some(pb.read_string()),
            6 => {
                self.last_modified_time =
                    Some(Date::from_time(pb.read_varint::<u64>() as i64 * 1000));
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
/// Select a stop by stop sequence or by stop_id. At least one of the two values must be provided.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeStopSelector {
    /// Must be the same as in stop_times.txt in the corresponding GTFS feed.
    pub stop_sequence: Option<u32>, // 1 [uint32]
    /// Must be the same as in stops.txt in the corresponding GTFS feed.
    pub stop_id: Option<String>, // 2 [string]
}
/// Read in the contents of the GTFSRealtimeStopSelector
impl ProtoRead for GTFSRealtimeStopSelector {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.stop_sequence = Some(pb.read_varint()),
            2 => self.stop_id = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Selected trips affected by TripModifications.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeSelectedTrips {
    /// A list of trips affected with this replacement that all have the same new `shape_id`.
    pub trip_ids: Vec<String>, // 1 [repeated string]
    /// The ID of the new shape for the modified trips in this SelectedTrips.
    /// May refer to a new shape added using a GTFS-RT Shape message, or to an existing shape defined in
    /// the GTFS-Static feed's shapes.txt.
    pub shape_id: Option<String>, // 2 [string]
}
/// Read in the contents of the GTFSRealtimeSelectedTrips
impl ProtoRead for GTFSRealtimeSelectedTrips {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.trip_ids.push(pb.read_string()),
            2 => self.shape_id = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the
/// future.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeReplacementStop {
    /// The difference in seconds between the arrival time at this stop and the arrival time at the reference
    /// stop. The reference stop is the stop prior to start_stop_selector. If the modification begins
    /// at the first stop of the trip, then the first stop of the trip is the reference stop.
    /// This value MUST be monotonically increasing and may only be a negative number if the first
    /// stop of the original trip is the reference stop.
    pub travel_time_to_stop: Option<i32>, // 1 [int32]
    /// The replacement stop ID which will now be visited by the trip. May refer to a new stop added
    /// using a GTFS-RT Stop message, or to an existing stop defined in the GTFS-Static feed's stops.txt.
    /// The stop MUST have location_type=0 (routable stops).
    pub stop_id: Option<String>, // 2 [string]
}
/// Read in the contents of the GTFSRealtimeReplacementStop
impl ProtoRead for GTFSRealtimeReplacementStop {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.travel_time_to_stop = Some(pb.read_varint()),
            2 => self.stop_id = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
