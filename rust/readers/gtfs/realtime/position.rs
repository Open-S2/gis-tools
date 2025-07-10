use pbf::{ProtoRead, Protobuf};

/// A Position is a point on the Earth's surface.
#[derive(Debug, Default, Clone)]
pub struct GTFSRealtimePosition {
    /// Degrees North, in the WGS-84 coordinate system.
    pub latitude: f32, // 1 [float]
    /// Degrees East, in the WGS-84 coordinate system.
    pub longitude: f32, // 2 [float]
    ///Bearing, in degrees, clockwise from North, i.e., 0 is North and 90 is East.
    ///This can be the compass bearing, or the direction towards the next stop
    ///or intermediate location.
    ///This should not be direction deduced from the sequence of previous
    ///positions, which can be computed from previous data.
    pub bearing: Option<f32>, // 3 [float]
    /// Odometer value, in meters.
    pub odometer: Option<f64>, // 4 [double]
    /// Momentary speed measured by the vehicle, in meters per second.
    pub speed: Option<f32>, // 5 [float]
}
/// Read in the contents of the GTFSRealtimePosition
impl ProtoRead for GTFSRealtimePosition {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.latitude = pb.read_varint(),
            2 => self.longitude = pb.read_varint(),
            3 => self.bearing = Some(pb.read_varint()),
            4 => self.odometer = Some(pb.read_varint()),
            5 => self.speed = Some(pb.read_varint()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
