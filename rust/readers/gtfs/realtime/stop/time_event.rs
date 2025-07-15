use pbf::{ProtoRead, Protobuf};

/// Timing information for a single predicted event (either arrival or
/// departure).
///
/// Timing consists of delay and/or estimated time, and uncertainty.
/// - delay should be used when the prediction is given relative to some
///   existing schedule in GTFS.
/// - time should be given whether there is a predicted schedule or not. If
///   both time and delay are specified, time will take precedence
///   (although normally, time, if given for a scheduled trip, should be
///   equal to scheduled time in GTFS + delay).
///
/// Uncertainty applies equally to both time and delay.
/// The uncertainty roughly specifies the expected error in true delay (but
/// note, we don't yet define its precise statistical meaning). It's possible
/// for the uncertainty to be 0, for example for trains that are driven under
/// computer timing control.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeStopTimeEvent {
    ///  Delay (in seconds) can be positive (meaning that the vehicle is late) or
    ///  negative (meaning that the vehicle is ahead of schedule). Delay of 0
    ///  means that the vehicle is exactly on time.
    pub delay: Option<i32>, // 1 [int32]
    ///  Event as absolute time.
    ///  In Unix time (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
    pub time: Option<i64>, // 2 [int64]
    ///  If uncertainty is omitted, it is interpreted as unknown.
    ///  If the prediction is unknown or too uncertain, the delay (or time) field
    ///  should be empty. In such case, the uncertainty field is ignored.
    ///  To specify a completely certain prediction, set its uncertainty to 0.
    pub uncertainty: Option<i32>, // 3 [int32]
}
/// Read in the contents of the blob header
impl ProtoRead for GTFSRealtimeStopTimeEvent {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.delay = Some(pb.read_s_varint()),
            2 => self.time = Some(pb.read_s_varint()),
            3 => self.uncertainty = Some(pb.read_s_varint()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
