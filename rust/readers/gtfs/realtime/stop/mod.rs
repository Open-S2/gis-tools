#![cfg_attr(feature = "nightly", coverage(off))]
// NOTE: THis is an experimental module, exists incase the GTFS-Realtime spec changes to include it

mod time_event;
mod update;

use crate::readers::GTFSRealtimeTranslatedString;
use alloc::string::String;
use pbf::{BitCast, ProtoRead, Protobuf};
pub use time_event::*;
pub use update::*;

/// The type of wheelchair boarding accessibility at a stop.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeWheelchairBoarding {
    /// Unknown accessibility.
    #[default]
    Unknown = 0,
    /// Wheelchair boarding is available.
    Available = 1,
    /// Wheelchair boarding is not available.
    NotAvailable = 2,
}

/// Describes a stop which is served by trips. All fields are as described in the GTFS-Static specification.
/// NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeStop {
    /// The stop_id of the stop
    pub stop_id: Option<String>, // 1 [string]
    /// The stop_code of the stop
    pub stop_code: Option<GTFSRealtimeTranslatedString>, // 2 [message]
    /// The stop_name of the stop
    pub stop_name: Option<GTFSRealtimeTranslatedString>, // 3 [message]
    /// The tts_stop_name of the stop
    pub tts_stop_name: Option<GTFSRealtimeTranslatedString>, // 4 [message]
    /// The stop_desc of the stop
    pub stop_desc: Option<GTFSRealtimeTranslatedString>, // 5 [message]
    /// The lat of the stop
    pub stop_lat: Option<f32>, // 6 [float]
    /// The lon of the stop
    pub stop_lon: Option<f32>, // 7 [float]
    /// The zone_id of the stop
    pub zone_id: Option<String>, // 8 [string]
    /// The stop_url of the stop
    pub stop_url: Option<GTFSRealtimeTranslatedString>, // 9 [string]
    /// The parent_station of the stop
    pub parent_station: Option<String>, // 11 [string]
    /// The stop_timezone of the stop
    pub stop_timezone: Option<String>, // 12 [string]
    /// The wheelchair_boarding of the stop
    pub wheelchair_boarding: Option<GTFSRealtimeWheelchairBoarding>, // 13 [enum]
    /// The level_id of the stop
    pub level_id: Option<String>, // 14 [string]
    /// The platform_code of the stop
    pub platform_code: Option<GTFSRealtimeTranslatedString>, // 15 [string]
}
/// Read in the contents of the blob header
impl ProtoRead for GTFSRealtimeStop {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.stop_id = Some(pb.read_string()),
            2 => {
                self.stop_code = {
                    let mut stop_code = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut stop_code);
                    Some(stop_code)
                }
            }
            3 => {
                self.stop_name = {
                    let mut stop_name = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut stop_name);
                    Some(stop_name)
                }
            }
            4 => {
                self.tts_stop_name = {
                    let mut tts_stop_name = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut tts_stop_name);
                    Some(tts_stop_name)
                }
            }
            5 => {
                self.stop_desc = {
                    let mut stop_desc = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut stop_desc);
                    Some(stop_desc)
                }
            }
            6 => self.stop_lat = Some(pb.read_fixed()),
            7 => self.stop_lon = Some(pb.read_fixed()),
            8 => self.zone_id = Some(pb.read_string()),
            9 => {
                self.stop_url = {
                    let mut stop_url = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut stop_url);
                    Some(stop_url)
                }
            }
            11 => self.parent_station = Some(pb.read_string()),
            12 => self.stop_timezone = Some(pb.read_string()),
            13 => self.wheelchair_boarding = Some(pb.read_varint()),
            14 => self.level_id = Some(pb.read_string()),
            15 => {
                self.platform_code = {
                    let mut platform_code = GTFSRealtimeTranslatedString::default();
                    pb.read_message(&mut platform_code);
                    Some(platform_code)
                }
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}
