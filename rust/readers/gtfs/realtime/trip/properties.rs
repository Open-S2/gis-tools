use crate::{readers::parse_gtfs_date, util::Date};
use alloc::string::String;
use pbf::{ProtoRead, Protobuf};

/// Defines updated properties of the trip, such as a new shape_id when there is a detour. Or defines the
/// trip_id, start_date, and start_time of a DUPLICATED trip.
/// NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTripProperties {
    /// Defines the identifier of a new trip that is a duplicate of an existing trip defined in (CSV) GTFS trips.txt
    /// but will start at a different service date and/or time (defined using the TripProperties.start_date and
    /// TripProperties.start_time fields). See definition of trips.trip_id in (CSV) GTFS. Its value must be different
    /// than the ones used in the (CSV) GTFS. Required if schedule_relationship=DUPLICATED, otherwise this field must not
    /// be populated and will be ignored by consumers.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub trip_id: Option<String>, // 1 [string]
    /// Service date on which the DUPLICATED trip will be run, in YYYYMMDD format. Required if
    /// schedule_relationship=DUPLICATED, otherwise this field must not be populated and will be ignored by consumers.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub start_date: Option<Date>, // 2 [string]
    /// Defines the departure start time of the trip when itâ€™s duplicated. See definition of stop_times.departure_time
    /// in (CSV) GTFS. Scheduled arrival and departure times for the duplicated trip are calculated based on the offset
    /// between the original trip departure_time and this field. For example, if a GTFS trip has stop A with a
    /// departure_time of 10:00:00 and stop B with departure_time of 10:01:00, and this field is populated with the value
    /// of 10:30:00, stop B on the duplicated trip will have a scheduled departure_time of 10:31:00. Real-time prediction
    /// delay values are applied to this calculated schedule time to determine the predicted time. For example, if a
    /// departure delay of 30 is provided for stop B, then the predicted departure time is 10:31:30. Real-time
    /// prediction time values do not have any offset applied to them and indicate the predicted time as provided.
    /// For example, if a departure time representing 10:31:30 is provided for stop B, then the predicted departure time
    /// is 10:31:30. This field is required if schedule_relationship is DUPLICATED, otherwise this field must not be
    /// populated and will be ignored by consumers.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub start_time: Option<String>, // 3 [string]
    /// Specifies the shape of the vehicle travel path when the trip shape differs from the shape specified in
    /// (CSV) GTFS or to specify it in real-time when it's not provided by (CSV) GTFS, such as a vehicle that takes differing
    /// paths based on rider demand. See definition of trips.shape_id in (CSV) GTFS. If a shape is neither defined in (CSV) GTFS
    /// nor in real-time, the shape is considered unknown. This field can refer to a shape defined in the (CSV) GTFS in shapes.txt
    /// or a Shape in the (protobuf) real-time feed. The order of stops (stop sequences) for this trip must remain the same as
    /// (CSV) GTFS. Stops that are a part of the original trip but will no longer be made, such as when a detour occurs, should
    /// be marked as schedule_relationship=SKIPPED.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub shape_id: Option<String>, // 4 [string]
    /// Specifies the headsign for this trip when it differs from the original.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub trip_headsign: Option<String>, // 5 [string]
    /// Specifies the name for this trip when it differs from the original.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub trip_short_name: Option<String>, // 6 [string]
}
/// Read in the contents of the GTFSRealtimeTripProperties
impl ProtoRead for GTFSRealtimeTripProperties {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.trip_id = Some(pb.read_string()),
            2 => self.start_date = Some(parse_gtfs_date(&pb.read_string()).unwrap_or_default()),
            3 => self.start_time = Some(pb.read_string()),
            // Experimental. May be used in the future
            // 4 => self.shape_id = Some(pb.read_string()),
            // 5 => self.trip_headsign = Some(pb.read_string()),
            // 6 => self.trip_short_name = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
