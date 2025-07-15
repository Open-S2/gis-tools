use crate::{
    readers::{GTFSRealtimePosition, GTFSRealtimeTripDescriptor},
    util::Date,
};
use alloc::{string::String, vec::Vec};
use pbf::{BitCast, ProtoRead, Protobuf};

/// Status of the vehicle relative to the stop
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSVehicleStopStatus {
    /// The vehicle is just about to arrive at the stop (on a stop
    /// display, the vehicle symbol typically flashes).
    IncomingAt = 0,
    /// The vehicle is standing at the stop.
    StoppedAt = 1,
    /// The vehicle has departed and is in transit to the next stop.
    #[default]
    InTransitTo = 2,
}

/// Congestion level that is affecting this vehicle.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeCongestionLevel {
    /// Unknown congestion level
    #[default]
    UnknownCongestionLevel = 0,
    /// Smooth traffic
    RunningSmoothly = 1,
    /// Stop and go traffic
    StopAndGo = 2,
    /// Heavy traffic
    Congestion = 3,
    /// Severe traffic (people leaving their cars)
    SevereCongestion = 4,
}

/// The state of passenger occupancy for the vehicle or carriage.
/// Individual producers may not publish all OccupancyStatus values. Therefore, consumers
/// must not assume that the OccupancyStatus values follow a linear scale.
/// Consumers should represent OccupancyStatus values as the state indicated
/// and intended by the producer. Likewise, producers must use OccupancyStatus values that
/// correspond to actual vehicle occupancy states.
/// For describing passenger occupancy levels on a linear scale, see `occupancy_percentage`.
/// This field is still experimental, and subject to change. It may be formally adopted in the future.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeOccupancyStatus {
    /// The vehicle or carriage is considered empty by most measures, and has few or no
    /// passengers onboard, but is still accepting passengers.
    Empty = 0,
    /// The vehicle or carriage has a large number of seats available.
    /// The amount of free seats out of the total seats available to be
    /// considered large enough to fall into this category is determined at the
    /// discretion of the producer.
    ManySeatsAvailable = 1,
    /// The vehicle or carriage has a relatively small number of seats available.
    /// The amount of free seats out of the total seats available to be
    /// considered small enough to fall into this category is determined at the
    /// discretion of the feed producer.
    FewSeatsAvailable = 2,
    /// The vehicle or carriage can currently accommodate only standing passengers.
    StandingRoomOnly = 3,
    /// The vehicle or carriage can currently accommodate only standing passengers
    /// and has limited space for them.
    CrushedStandingRoomOnly = 4,
    /// The vehicle or carriage is considered full by most measures, but may still be
    /// allowing passengers to board.
    Full = 5,
    /// The vehicle or carriage is not accepting passengers, but usually accepts passengers for
    /// boarding.
    NotAcceptingPassengers = 6,
    /// The vehicle or carriage doesn't have any occupancy data available at that time.
    #[default]
    NoDataAvailable = 7,
    /// The vehicle or carriage is not boardable and never accepts passengers.
    /// Useful for special vehicles or carriages (engine, maintenance carriage, etc.).
    NotBoardable = 8,
}

/// Realtime positioning information for a given vehicle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeVehiclePosition {
    /// The Trip that this vehicle is serving.
    /// Can be empty or partial if the vehicle can not be identified with a given
    /// trip instance.
    pub trip: Option<GTFSRealtimeTripDescriptor>, // 1 [message]
    /// Current position of this vehicle. */
    pub position: GTFSRealtimePosition, // 2 [message]
    /// The stop sequence index of the current stop. The meaning of
    /// current_stop_sequence (i.e., the stop that it refers to) is determined by
    /// current_status.
    /// If current_status is missing IN_TRANSIT_TO is assumed.
    pub current_stop_sequence: Option<u32>, // 3 [uint32]
    /// The exact status of the vehicle with respect to the current stop.
    /// Ignored if current_stop_sequence is missing.
    pub current_status: GTFSVehicleStopStatus, // 4 [enum]
    /// Moment at which the vehicle's position was measured. In POSIX time
    /// (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
    pub timestamp: Option<Date>, // 5 [uint64]
    /// Congestion level that is affecting this vehicle.
    pub congestion_level: GTFSRealtimeCongestionLevel, // 6 [enum]
    /// Identifies the current stop. The value must be the same as in stops.txt in
    /// the corresponding GTFS feed.
    pub stop_id: Option<String>,
    /// 7 Additional information on the vehicle that is serving this trip.
    pub vehicle: Option<GTFSRealtimeVehicleDescriptor>, // 8 [message]
    /// If multi_carriage_status is populated with per-carriage OccupancyStatus,
    /// then this field should describe the entire vehicle with all carriages accepting passengers considered.
    pub occupancy_status: Option<GTFSRealtimeOccupancyStatus>, // 9 [enum]
    /// A percentage value indicating the degree of passenger occupancy in the vehicle.
    /// The values are represented as an integer without decimals. 0 means 0% and 100 means 100%.
    /// The value 100 should represent the total maximum occupancy the vehicle was designed for,
    /// including both seated and standing capacity, and current operating regulations allow.
    /// The value may exceed 100 if there are more passengers than the maximum designed capacity.
    /// The precision of occupancy_percentage should be low enough that individual passengers cannot be tracked boarding or alighting the vehicle.
    /// If multi_carriage_status is populated with per-carriage occupancy_percentage,
    /// then this field should describe the entire vehicle with all carriages accepting passengers considered.
    /// This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub occupancy_percentage: Option<u32>, // 10 [uint32]
    /// Details of the multiple carriages of this given vehicle.
    /// The first occurrence represents the first carriage of the vehicle,
    /// given the current direction of travel.
    /// The number of occurrences of the multi_carriage_details
    /// field represents the number of carriages of the vehicle.
    /// It also includes non boardable carriages,
    /// like engines, maintenance carriages, etcâ€¦ as they provide valuable
    /// information to passengers about where to stand on a platform.
    /// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
    pub multi_carriage_details: Vec<GTFSRealtimeMultiCarriageDetails>, // 11 [message]
}
/// Read in the contents of the GTFSRealtimeVehiclePosition
impl ProtoRead for GTFSRealtimeVehiclePosition {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut trip = GTFSRealtimeTripDescriptor::default();
                pb.read_message(&mut trip);
                self.trip = Some(trip);
            }
            2 => {
                let mut position = GTFSRealtimePosition::default();
                pb.read_message(&mut position);
                self.position = position;
            }
            3 => self.current_stop_sequence = Some(pb.read_varint()),
            4 => self.current_status = pb.read_varint(),
            5 => self.timestamp = Some(Date::from_time(pb.read_varint::<u64>() as i64 * 1000)),
            6 => self.congestion_level = pb.read_varint(),
            7 => self.stop_id = Some(pb.read_string()),
            8 => {
                let mut vehicle = GTFSRealtimeVehicleDescriptor::default();
                pb.read_message(&mut vehicle);
                self.vehicle = Some(vehicle);
            }
            9 => self.occupancy_status = Some(pb.read_varint()),
            10 => self.occupancy_percentage = Some(pb.read_varint()),
            11 => {
                let mut multi_carriage_details = GTFSRealtimeMultiCarriageDetails::default();
                pb.read_message(&mut multi_carriage_details);
                self.multi_carriage_details.push(multi_carriage_details);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Wheelchair accessibility of the trip
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeWheelchairAccessible {
    /// The trip doesn't have information about wheelchair accessibility.
    /// This is the **default** behavior. If the static GTFS contains a
    /// _wheelchair_accessible_ value, it won't be overwritten.
    #[default]
    NoValue = 0,
    /// The trip has no accessibility value present.
    /// This value will overwrite the value from the GTFS.
    Unknown = 1,
    /// The trip is wheelchair accessible.
    /// This value will overwrite the value from the GTFS.
    WheelchairAccessible = 2,
    /// The trip is **not** wheelchair accessible.
    /// This value will overwrite the value from the GTFS.
    WheelchairInaccessible = 3,
}

/// Identification information for the vehicle performing the trip.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeVehicleDescriptor {
    /// Internal system identification of the vehicle. Should be unique per
    /// vehicle, and can be used for tracking the vehicle as it proceeds through
    /// the system.
    pub id: Option<String>, // 1 [string]
    /// User visible label, i.e., something that must be shown to the passenger to
    /// help identify the correct vehicle.
    pub label: Option<String>, // 2 [string]
    /// The license plate of the vehicle.
    pub license_plate: Option<String>, // 3 [string]
    /// Wheelchair accessibility of the trip
    pub wheelchair_accessible: GTFSRealtimeWheelchairAccessible, // 4 [enum]
}
/// Read in the contents of the GTFSRealtimeVehicleDescriptor
impl ProtoRead for GTFSRealtimeVehicleDescriptor {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.id = Some(pb.read_string()),
            2 => self.label = Some(pb.read_string()),
            3 => self.license_plate = Some(pb.read_string()),
            4 => self.wheelchair_accessible = pb.read_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// Carriage specific details, used for vehicles composed of several carriages
/// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeMultiCarriageDetails {
    /// Identification of the carriage. Should be unique per vehicle.
    pub id: Option<String>, // 1 [string]
    /// User visible label that may be shown to the passenger to help identify
    /// the carriage. Example: "7712", "Car ABC-32", etc...
    /// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
    pub label: Option<String>, // 2 [string]
    /// Occupancy status for this given carriage, in this vehicle
    /// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
    pub occupancy_status: GTFSRealtimeOccupancyStatus, // 3 [enum]
    /// Occupancy percentage for this given carriage, in this vehicle.
    /// Follows the same rules as "VehiclePosition.occupancy_percentage"
    /// -1 in case data is not available for this given carriage (as protobuf defaults to 0 otherwise)
    /// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
    pub occupancy_percentage: i32, // 4 [int32]
    /// Identifies the order of this carriage with respect to the other
    /// carriages in the vehicle's list of CarriageDetails.
    /// The first carriage in the direction of travel must have a value of 1.
    /// The second value corresponds to the second carriage in the direction
    /// of travel and must have a value of 2, and so forth.
    /// For example, the first carriage in the direction of travel has a value of 1.
    /// If the second carriage in the direction of travel has a value of 3,
    /// consumers will discard data for all carriages (i.e., the multi_carriage_details field).
    /// Carriages without data must be represented with a valid carriage_sequence number and the fields
    /// without data should be omitted (alternately, those fields could also be included and set to the "no data" values).
    /// This message/field is still experimental, and subject to change. It may be formally adopted in the future.
    pub carriage_sequence: Option<u32>, // 5 [uint32]
}
/// Read in the contents of the GTFSRealtimeMultiCarriageDetails
impl ProtoRead for GTFSRealtimeMultiCarriageDetails {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.id = Some(pb.read_string()),
            2 => self.label = Some(pb.read_string()),
            3 => self.occupancy_status = pb.read_varint(),
            4 => self.occupancy_percentage = pb.read_s_varint(),
            5 => self.carriage_sequence = Some(pb.read_varint()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
