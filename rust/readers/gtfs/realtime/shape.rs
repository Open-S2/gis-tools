use alloc::string::String;
use pbf::{ProtoRead, Protobuf};

/// Describes the physical path that a vehicle takes when it's not part of the (CSV) GTFS,
/// such as for a detour. Shapes belong to Trips, and consist of a sequence of shape points.
/// Tracing the points in order provides the path of the vehicle.  Shapes do not need to intercept
/// the location of Stops exactly, but all Stops on a trip should lie within a small distance of
/// the shape for that trip, i.e. close to straight line segments connecting the shape points
/// NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future
#[derive(Debug, Default, Clone)]
pub struct GTFSRealtimeShape {
    /// Identifier of the shape. Must be different than any shape_id defined in the (CSV) GTFS.
    /// This field is required as per reference.md, but needs to be specified here optional because "Required is Forever"
    /// See https://developers.google.com/protocol-buffers/docs/proto#specifying_field_rules
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub shape_id: Option<String>, // 1 [string]
    /// Encoded polyline representation of the shape. This polyline must contain at least two points.
    /// For more information about encoded polylines, see https://developers.google.com/maps/documentation/utilities/polylinealgorithm
    /// This field is required as per reference.md, but needs to be specified here optional because "Required is Forever"
    /// See https://developers.google.com/protocol-buffers/docs/proto#specifying_field_rules
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub encoded_polyline: Option<String>, // 2 [string]
}
/// Read in the contents of the GTFSRealtimeShape
impl ProtoRead for GTFSRealtimeShape {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.shape_id = Some(pb.read_string()),
            2 => self.encoded_polyline = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
