use crate::readers::{
    GTFSRealtimeAlert, GTFSRealtimeShape, GTFSRealtimeStop, GTFSRealtimeTripDescriptor,
    GTFSRealtimeTripModifications, GTFSRealtimeTripUpdate, GTFSRealtimeVehiclePosition,
};
use alloc::string::String;
use pbf::{ProtoRead, Protobuf};

/// The type of a GTFSRealtimeEntity message
#[derive(Debug)]
pub enum GTFSRealtimeEntityMessage {
    /// A Trip Update message
    TripUpdate(GTFSRealtimeTripUpdate),
    /// A Vehicle Position message
    VehiclePosition(GTFSRealtimeVehiclePosition),
    /// An Alert message
    Alert(GTFSRealtimeAlert),
    /// A Shape message
    Shape(GTFSRealtimeShape),
    /// A Stop message
    Stop(GTFSRealtimeStop),
    /// A Trip Modifications message
    TripModifications(GTFSRealtimeTripModifications),
    /// A Deleted message
    Deleted,
}
impl From<&GTFSRealtimeEntity> for GTFSRealtimeEntityMessage {
    fn from(entity: &GTFSRealtimeEntity) -> Self {
        if entity.is_deleted {
            GTFSRealtimeEntityMessage::Deleted
        } else if entity.trip_update.is_some() {
            GTFSRealtimeEntityMessage::TripUpdate(entity.trip_update.as_ref().unwrap().clone())
        } else if entity.vehicle_position.is_some() {
            GTFSRealtimeEntityMessage::VehiclePosition(
                entity.vehicle_position.as_ref().unwrap().clone(),
            )
        } else if entity.alert.is_some() {
            GTFSRealtimeEntityMessage::Alert(entity.alert.as_ref().unwrap().clone())
        } else if entity.shape.is_some() {
            GTFSRealtimeEntityMessage::Shape(entity.shape.as_ref().unwrap().clone())
        } else if entity.stop.is_some() {
            GTFSRealtimeEntityMessage::Stop(entity.stop.as_ref().unwrap().clone())
        } else if entity.trip_modifications.is_some() {
            GTFSRealtimeEntityMessage::TripModifications(
                entity.trip_modifications.as_ref().unwrap().clone(),
            )
        } else {
            GTFSRealtimeEntityMessage::Deleted
        }
    }
}

/// A definition (or update) of an entity in the transit feed.
/// May be a TripUpdate, VehiclePosition, Alert, Shape, Stop, and/or TripModifications.
/// At least one of the above must be present (unless the entity is being deleted).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeEntity {
    /// The ids are used only to provide incrementality support. The id should be
    /// unique within a FeedMessage. Consequent FeedMessages may contain
    /// FeedEntities with the same id. In case of a DIFFERENTIAL update the new
    /// FeedEntity with some id will replace the old FeedEntity with the same id
    /// (or delete it - see is_deleted below).
    /// The actual GTFS entities (e.g. stations, routes, trips) referenced by the
    /// feed must be specified by explicit selectors (see EntitySelector below for
    /// more info).
    pub id: String, // 1 [string]
    /// Whether this entity is to be deleted. Relevant only for incremental fetches
    pub is_deleted: bool, // 2 [bool]
    /// Trip update
    pub trip_update: Option<GTFSRealtimeTripUpdate>, // 3
    /// Realtime positioning information for a given vehicle.
    pub vehicle_position: Option<GTFSRealtimeVehiclePosition>, // 4
    /// An alert
    pub alert: Option<GTFSRealtimeAlert>, // 5
    /// Describes the physical path that a vehicle takes when it's not part of the (CSV) GTFS,
    /// such as for a detour. Shapes belong to Trips, and consist of a sequence of shape points.
    /// Tracing the points in order provides the path of the vehicle.  Shapes do not need to intercept
    /// the location of Stops exactly, but all Stops on a trip should lie within a small distance of
    /// the shape for that trip, i.e. close to straight line segments connecting the shape points
    /// NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
    pub shape: Option<GTFSRealtimeShape>, // 6
    /// Describes a stop
    pub stop: Option<GTFSRealtimeStop>, // 7
    /// Trip modifications
    pub trip_modifications: Option<GTFSRealtimeTripModifications>, // 8
}
/// Read in the contents of the GTFSRealtimeEntity
impl ProtoRead for GTFSRealtimeEntity {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.id = pb.read_string(),
            2 => self.is_deleted = pb.read_varint(),
            3 => {
                let mut trip_update = GTFSRealtimeTripUpdate::default();
                pb.read_message(&mut trip_update);
                self.trip_update = Some(trip_update);
            }
            4 => {
                let mut vehicle_position = GTFSRealtimeVehiclePosition::default();
                pb.read_message(&mut vehicle_position);
                self.vehicle_position = Some(vehicle_position);
            }
            5 => {
                let mut alert = GTFSRealtimeAlert::default();
                pb.read_message(&mut alert);
                self.alert = Some(alert);
            }
            6 => {
                let mut shape = GTFSRealtimeShape::default();
                pb.read_message(&mut shape);
                self.shape = Some(shape);
            }
            7 => {
                let mut stop = GTFSRealtimeStop::default();
                pb.read_message(&mut stop);
                self.stop = Some(stop);
            }
            8 => {
                let mut trip_modifications = GTFSRealtimeTripModifications::default();
                pb.read_message(&mut trip_modifications);
                self.trip_modifications = Some(trip_modifications);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// A selector for an entity in a GTFS feed.
/// The values of the fields should correspond to the appropriate fields in the
/// GTFS feed.
/// At least one specifier must be given. If several are given, then the
/// matching has to apply to all the given specifiers.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeEntitySelector {
    /// Corresponds to agency_id in GTFS
    pub agency_id: Option<String>, // 1 [string]
    /// Corresponds to route_id in GTFS
    pub route_id: Option<String>, // 2 [string]
    /// corresponds to route_type in GTFS.
    /// For example, 0 means "any route type".
    pub route_type: Option<i32>, // 3 [int32]
    /// Corresponds to trip_id in GTFS
    pub trip: Option<GTFSRealtimeTripDescriptor>, // 4 [message]
    /// Corresponds to stop_id in GTFS stops.txt
    pub stop_id: Option<String>, // 5 [string]
    /// Corresponds to trip direction_id in GTFS trips.txt. If provided the
    /// route_id must also be provided.
    pub direction_id: Option<u32>, // 6 [uint32]
}
/// Read in the contents of the GTFSRealtimeEntitySelector
impl ProtoRead for GTFSRealtimeEntitySelector {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.agency_id = Some(pb.read_string()),
            2 => self.route_id = Some(pb.read_string()),
            3 => self.route_type = Some(pb.read_s_varint()),
            4 => {
                let mut trip = GTFSRealtimeTripDescriptor::default();
                pb.read_message(&mut trip);
                self.trip = Some(trip);
            }
            5 => self.stop_id = Some(pb.read_string()),
            6 => self.direction_id = Some(pb.read_varint()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
