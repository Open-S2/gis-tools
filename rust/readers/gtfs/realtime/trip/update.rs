use crate::readers::{
    GTFSRealtimeStopTimeUpdate, GTFSRealtimeTripDescriptor, GTFSRealtimeTripProperties,
    GTFSRealtimeVehicleDescriptor,
};
use alloc::vec::Vec;
use pbf::{ProtoRead, Protobuf};

/// Realtime update of the progress of a vehicle along a trip.
/// Depending on the value of ScheduleRelationship, a TripUpdate can specify:
/// - A trip that proceeds along the schedule.
/// - A trip that proceeds along a route but has no fixed schedule.
/// - A trip that have been added or removed with regard to schedule.
///
/// The updates can be for future, predicted arrival/departure events, or for
/// past events that already occurred.
/// Normally, updates should get more precise and more certain (see
/// uncertainty below) as the events gets closer to current time.
/// Even if that is not possible, the information for past events should be
/// precise and certain. In particular, if an update points to time in the past
/// but its update's uncertainty is not 0, the client should conclude that the
/// update is a (wrong) prediction and that the trip has not completed yet.
///
/// Note that the update can describe a trip that is already completed.
/// To this end, it is enough to provide an update for the last stop of the trip.
/// If the time of that is in the past, the client will conclude from that that
/// the whole trip is in the past (it is possible, although inconsequential, to
/// also provide updates for preceding stops).
/// This option is most relevant for a trip that has completed ahead of schedule,
/// but according to the schedule, the trip is still proceeding at the current
/// time. Removing the updates for this trip could make the client assume
/// that the trip is still proceeding.
/// Note that the feed provider is allowed, but not required, to purge past
/// updates - this is one case where this would be practically useful.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTripUpdate {
    /// The Trip that this message applies to. There can be at most one
    /// TripUpdate entity for each actual trip instance.
    /// If there is none, that means there is no prediction information available.
    /// It does *not* mean that the trip is progressing according to schedule.
    pub trip: GTFSRealtimeTripDescriptor, // 1 [message]
    /// Updates to StopTimes for the trip (both future, i.e., predictions, and in
    /// some cases, past ones, i.e., those that already happened).
    /// The updates must be sorted by stop_sequence, and apply for all the
    /// following stops of the trip up to the next specified one.
    ///
    /// Example 1:
    ///
    /// For a trip with 20 stops, a StopTimeUpdate with arrival delay and departure
    /// delay of 0 for stop_sequence of the current stop means that the trip is
    /// exactly on time.
    ///
    /// Example 2:
    ///
    /// For the same trip instance, 3 StopTimeUpdates are provided:
    /// - delay of 5 min for stop_sequence 3
    /// - delay of 1 min for stop_sequence 8
    /// - delay of unspecified duration for stop_sequence 10
    ///
    /// This will be interpreted as:
    /// - stop_sequences 3,4,5,6,7 have delay of 5 min.
    /// - stop_sequences 8,9 have delay of 1 min.
    /// - stop_sequences 10,... have unknown delay.
    pub stop_time_update: Vec<GTFSRealtimeStopTimeUpdate>, // 2 [message]
    /// Additional information on the vehicle that is serving this trip.
    pub vehicle: Option<GTFSRealtimeVehicleDescriptor>, // 3 [message]
    /// The most recent moment at which the vehicle's real-time progress was measured
    /// to estimate StopTimes in the future. When StopTimes in the past are provided,
    /// arrival/departure times may be earlier than this value. In POSIX
    /// time (i.e., the number of seconds since January 1st 1970 00:00:00 UTC).
    pub timestamp: Option<u64>, // 4 [uint64]
    /// The current schedule deviation for the trip.  Delay should only be
    /// specified when the prediction is given relative to some existing schedule
    /// in GTFS.
    ///
    /// Delay (in seconds) can be positive (meaning that the vehicle is late) or
    /// negative (meaning that the vehicle is ahead of schedule). Delay of 0
    /// means that the vehicle is exactly on time.
    ///
    /// Delay information in StopTimeUpdates take precedent of trip-level delay
    /// information, such that trip-level delay is only propagated until the next
    /// stop along the trip with a StopTimeUpdate delay value specified.
    ///
    /// Feed providers are strongly encouraged to provide a TripUpdate.timestamp
    /// value indicating when the delay value was last updated, in order to
    /// evaluate the freshness of the data.
    ///
    /// NOTE: This field is still experimental, and subject to change. It may be
    /// formally adopted in the future.
    pub delay: Option<i32>, // 5 [int32]
    /// Additional information about the trip.
    pub trip_properties: Option<GTFSRealtimeTripProperties>, // 6 [message]
}
/// Read in the contents of the GTFSRealtimeTripUpdate
impl ProtoRead for GTFSRealtimeTripUpdate {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut trip = GTFSRealtimeTripDescriptor::default();
                pb.read_message(&mut trip);
                self.trip = trip;
            }
            2 => {
                let mut stop_time_update = GTFSRealtimeStopTimeUpdate::default();
                pb.read_message(&mut stop_time_update);
                self.stop_time_update.push(stop_time_update);
            }
            3 => {
                let mut vehicle = GTFSRealtimeVehicleDescriptor::default();
                pb.read_message(&mut vehicle);
                self.vehicle = Some(vehicle);
            }
            4 => self.timestamp = Some(pb.read_varint()),
            5 => self.delay = Some(pb.read_s_varint()),
            6 => {
                let mut trip_properties = GTFSRealtimeTripProperties::default();
                pb.read_message(&mut trip_properties);
                self.trip_properties = Some(trip_properties);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}
