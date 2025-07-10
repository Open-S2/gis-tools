mod alert;
mod entity;
mod header;
mod position;
mod shape;
mod stop;
mod trip;
mod util;
mod vehicle_position;

pub use alert::*;
use alloc::vec::Vec;
pub use entity::*;
pub use header::*;
use pbf::{ProtoRead, Protobuf};
pub use position::*;
pub use shape::*;
pub use stop::*;
pub use trip::*;
pub use util::*;
pub use vehicle_position::*;

/// # GTFS Realtime message.
///
/// ## Description
/// The input is a Uint8Array that has encoded protobuffer messages.
/// See {@link https://open-s2.github.io/pbf/classes/PbfReader.html}.
///
/// The contents of a feed message.
/// A feed is a continuous stream of feed messages. Each message in the stream is
/// obtained as a response to an appropriate HTTP GET request.
/// A realtime feed is always defined with relation to an existing GTFS feed.
/// All the entity ids are resolved with respect to the GTFS feed.
/// Note that "required" and "optional" as stated in this file refer to Protocol
/// Buffer cardinality, not semantic cardinality.  See reference.md at
/// https://github.com/google/transit/tree/master/gtfs-realtime for field
/// semantic cardinality.
///
/// ## Usage
///
/// ```ts
/// import { GTFSRealtimeReader } from 'gis-tools-ts';
///
/// const gtfsRealtimeReader = new GTFSRealtimeReader(data);
/// const { header, entities } = gtfsRealtimeReader;
/// for (const entity of entities) {
///   console.log(entity);
/// }
/// ```
///
/// ## Links
/// - https://mobilitydatabase.org
/// - https://developers.google.com/transit/gtfs/examples/overview
/// - https://mobilitydata.github.io/
/// - https://www.transit.land
#[derive(Debug, Default, Clone)]
pub struct GTFSRealtimeReader {
    /// The header of the message
    pub header: GTFSRealtimeHeader,
    /// The entities in the message
    pub entities: Vec<GTFSRealtimeEntity>,
}
impl GTFSRealtimeReader {
    /// Create a new GTFSRealtimeReader
    pub fn new(data: Vec<u8>, end: Option<usize>) -> Self {
        let mut this = Self::default();
        let mut pbf = Protobuf::from_input(data);
        pbf.read_fields(&mut this, end);
        this
    }
}
/// Read in the contents of the GTFSRealtimeReader
impl ProtoRead for GTFSRealtimeReader {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut header = GTFSRealtimeHeader::default();
                pb.read_message(&mut header);
                self.header = header;
            }
            2 => {
                let mut entity = GTFSRealtimeEntity::default();
                pb.read_message(&mut entity);
                self.entities.push(entity);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}
