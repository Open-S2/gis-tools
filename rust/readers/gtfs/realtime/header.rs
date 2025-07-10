use crate::util::Date;
use alloc::string::String;
use pbf::{BitCast, ProtoRead, Protobuf};

/// Determines whether the current fetch is incremental. Currently,
/// DIFFERENTIAL mode is unsupported and behavior is unspecified for feeds
/// that use this mode.  There are discussions on the GTFS Realtime mailing
/// list around fully specifying the behavior of DIFFERENTIAL mode and the
/// documentation will be updated when those discussions are finalized.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum Incrementality {
    /// Full dataset
    #[default]
    FullDataset = 0,
    /// Differential
    Differential = 1,
}

/// Metadata about a feed, included in feed messages.
#[derive(Debug, Clone)]
pub struct GTFSRealtimeHeader {
    /// Version of the feed specification.
    /// The current version is 2.0.  Valid versions are "2.0", "1.0".
    pub gtfs_realtime_version: String, // 1 [string]
    /// Determines whether the current fetch is incremental. Currently,
    /// DIFFERENTIAL mode is unsupported and behavior is unspecified for feeds
    /// that use this mode.  There are discussions on the GTFS Realtime mailing
    /// list around fully specifying the behavior of DIFFERENTIAL mode and the
    /// documentation will be updated when those discussions are finalized.
    pub incrementality: Incrementality, // 2 [enum]
    /// This timestamp identifies the moment when the content of this feed has been
    /// created (in server time). In POSIX time (i.e., number of seconds since
    /// January 1st 1970 00:00:00 UTC).
    pub timestamp: Option<Date>, // 3 [uint64]
    /// String that matches the feed_info.feed_version from the GTFS feed that the real
    /// time data is based on. Consumers can use this to identify which GTFS feed is
    /// currently active or when a new one is available to download.
    pub feed_version: Option<String>, // 4 [string]
}
impl Default for GTFSRealtimeHeader {
    fn default() -> Self {
        Self {
            gtfs_realtime_version: "2.0".into(),
            incrementality: Incrementality::FullDataset,
            timestamp: None,
            feed_version: None,
        }
    }
}
/// Read in the contents of the GTFSRealtimeHeader
impl ProtoRead for GTFSRealtimeHeader {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.gtfs_realtime_version = pb.read_string(),
            2 => self.incrementality = pb.read_varint(),
            3 => self.timestamp = Some(Date::from_time(pb.read_varint::<u64>() as i64 * 1000)),
            4 => self.feed_version = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
