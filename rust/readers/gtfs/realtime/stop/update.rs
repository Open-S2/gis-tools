use alloc::string::String;
use pbf::{BitCast, ProtoRead, Protobuf};

use crate::readers::{GTFSRealtimeOccupancyStatus, GTFSRealtimeStopTimeEvent};

/// The relation between the StopTimeEvents and the static schedule.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeScheduleRelationshipUpdate {
    /// The vehicle is proceeding in accordance with its static schedule of
    /// stops, although not necessarily according to the times of the schedule.
    /// At least one of arrival and departure must be provided. If the schedule
    /// for this stop contains both arrival and departure times then so must
    /// this update. Frequency-based trips (GTFS frequencies.txt with exact_times = 0)
    /// should not have a SCHEDULED value and should use UNSCHEDULED instead.
    #[default]
    Scheduled = 0,
    /// The stop is skipped, i.e., the vehicle will not stop at this stop.
    /// Arrival and departure are optional.
    Skipped = 1,
    /// No StopTimeEvents are given for this stop.
    /// The main intention for this value is to give time predictions only for
    /// part of a trip, i.e., if the last update for a trip has a NO_DATA
    /// specifier, then StopTimeEvents for the rest of the stops in the trip
    /// are considered to be unspecified as well.
    /// Neither arrival nor departure should be supplied.
    NoData = 2,
    /// The vehicle is operating a trip defined in GTFS frequencies.txt with exact_times = 0.
    /// This value should not be used for trips that are not defined in GTFS frequencies.txt,
    /// or trips in GTFS frequencies.txt with exact_times = 1. Trips containing StopTimeUpdates
    /// with ScheduleRelationship=UNSCHEDULED must also set TripDescriptor.ScheduleRelationship=UNSCHEDULED.
    /// NOTE: This field is still experimental, and subject to change. It may be
    /// formally adopted in the future.
    Unscheduled = 3,
}

/// Realtime update for arrival and/or departure events for a given stop on a
/// trip. Updates can be supplied for both past and future events.
/// The producer is allowed, although not required, to drop past events.
///
/// The update is linked to a specific stop either through stop_sequence or
/// stop_id, so one of the fields below must necessarily be set.
/// See the documentation in TripDescriptor for more information.
#[derive(Debug, Default, Clone)]
pub struct GTFSRealtimeStopTimeUpdate {
    /// Must be the same as in stop_times.txt in the corresponding GTFS feed.
    pub stop_sequence: Option<u32>, // 1 [uint32]
    /// Realtime updates for arrival events.
    pub arrival: Option<GTFSRealtimeStopTimeEvent>, // 2 [message]
    /// Realtime updates for departure events.
    pub departure: Option<GTFSRealtimeStopTimeEvent>, // 3 [message]
    /// Must be the same as in stops.txt in the corresponding GTFS feed.
    pub stop_id: Option<String>, // 4 [string]
    /// The relation between the StopTimeEvents and the static schedule.
    pub schedule_relationship: GTFSRealtimeScheduleRelationshipUpdate, // 5 [enum]
    /// Realtime updates for certain properties defined within GTFS stop_times.txt
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub stop_time_properties: Option<GTFSRealtimeStopTimeProperties>, // 6 [message]
    /// Expected occupancy after departure from the given stop.
    /// Should be provided only for future stops.
    /// In order to provide departure_occupancy_status without either arrival or
    /// departure StopTimeEvents, ScheduleRelationship should be set to NO_DATA.
    pub departure_occupancy_status: Option<GTFSRealtimeOccupancyStatus>, // 7 [enum]
}
/// Read in the contents of the GTFSRealtimeStopTimeUpdate
impl ProtoRead for GTFSRealtimeStopTimeUpdate {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.stop_sequence = Some(pb.read_varint()),
            2 => {
                let mut arrival = GTFSRealtimeStopTimeEvent::default();
                pb.read_message(&mut arrival);
                self.arrival = Some(arrival);
            }
            3 => {
                let mut departure = GTFSRealtimeStopTimeEvent::default();
                pb.read_message(&mut departure);
                self.departure = Some(departure);
            }
            4 => self.stop_id = Some(pb.read_string()),
            5 => self.schedule_relationship = pb.read_varint(),
            6 => {
                let mut stop_time_properties = GTFSRealtimeStopTimeProperties::default();
                pb.read_message(&mut stop_time_properties);
                self.stop_time_properties = Some(stop_time_properties);
            }
            7 => self.departure_occupancy_status = Some(pb.read_varint()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Provides the updated values for the stop time.
/// NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
#[derive(Debug, Default, Clone)]
pub struct GTFSRealtimeStopTimeProperties {
    /// Supports real-time stop assignments. Refers to a stop_id defined in the GTFS stops.txt.
    /// The new assigned_stop_id should not result in a significantly different trip experience for the end user than
    /// the stop_id defined in GTFS stop_times.txt. In other words, the end user should not view this new stop_id as an
    /// "unusual change" if the new stop was presented within an app without any additional context.
    /// For example, this field is intended to be used for platform assignments by using a stop_id that belongs to the
    /// same station as the stop originally defined in GTFS stop_times.txt.
    /// To assign a stop without providing any real-time arrival or departure predictions, populate this field and set
    /// StopTimeUpdate.schedule_relationship = NO_DATA.
    /// If this field is populated, it is preferred to omit `StopTimeUpdate.stop_id` and use only `StopTimeUpdate.stop_sequence`. If
    /// `StopTimeProperties.assigned_stop_id` and `StopTimeUpdate.stop_id` are populated, `StopTimeUpdate.stop_id` must match `assigned_stop_id`.
    /// Platform assignments should be reflected in other GTFS-realtime fields as well
    /// (e.g., `VehiclePosition.stop_id`).
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub assigned_stop_id: Option<String>, // 1 [string]
}
/// Read in the contents of the GTFSRealtimeStopTimeProperties
impl ProtoRead for GTFSRealtimeStopTimeProperties {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.assigned_stop_id = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
