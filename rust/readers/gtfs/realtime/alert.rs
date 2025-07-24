use crate::{
    readers::{GTFSRealtimeEntitySelector, GTFSRealtimeTranslatedString},
    util::Date,
};
use alloc::vec::Vec;
use pbf::{BitCast, ProtoRead, Protobuf};

/// Severity of this alert.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeSeverityLevel {
    /// Unknown severity level
    #[default]
    UnknownSeverity = 1,
    /// Informational message
    Info = 2,
    /// Warning
    Warning = 3,
    /// Severe problem
    Severe = 4,
}

/// What is the effect of this problem on the affected entity. If effect_detail is included, then
/// Effect must also be included.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeEffect {
    /// No service
    NoService = 1,
    /// Reduced service
    ReducedService = 2,
    /// We don't care about INsignificant delays: they are hard to detect, have
    /// little impact on the user, and would clutter the results as they are too
    /// frequent.
    SignificantDelays = 3,
    /// Detour
    Detour = 4,
    /// Additional service
    AdditionalService = 5,
    /// Modified service
    ModifiedService = 6,
    /// Other effect
    OtherEffect = 7,
    /// Unknown effect
    #[default]
    UnknownEffect = 8,
    /// Stop moved
    StopMoved = 9,
    /// No effect
    NoEffect = 10,
    /// Accessibility issue
    AccessibilityIssue = 11,
}

/// Cause of this alert. If cause_detail is included, then Cause must also be included.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, BitCast)]
pub enum GTFSRealtimeCause {
    /// Unknown cause
    #[default]
    UnknownCause = 1,
    /// Not machine-representable.
    OtherCause = 2,
    /// Technical problem.
    TechnicalProblem = 3,
    /// Public transit agency employees stopped working.
    Strike = 4,
    /// People are blocking the streets.
    Demonstration = 5,
    /// Accident.
    Accident = 6,
    /// Holiday.
    HOLIHolidayDAY = 7,
    /// Weather.
    Weather = 8,
    /// Maintenance.
    Maintenance = 9,
    /// Construction.
    Construction = 10,
    /// Police activity.
    PoliceActivity = 11,
    /// Medical emergency.
    MedicalEmergency = 12,
}

/// An alert, indicating some sort of incident in the public transit network.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeAlert {
    /// Time when the alert should be shown to the user. If missing, the
    /// alert will be shown as long as it appears in the feed.
    /// If multiple ranges are given, the alert will be shown during all of them.
    pub active_periods: Vec<GTFSRealtimeTimeRange>, // 1 [repeated message]
    /// Entities whose users we should notify of this alert.
    pub informed_entities: Vec<GTFSRealtimeEntitySelector>, // 5 [repeated message]
    /// Cause of this alert. If cause_detail is included, then Cause must also be included.
    pub cause: GTFSRealtimeCause, // 6 [enum]
    /// What is the effect of this problem on the affected entity. If effect_detail is included, then
    /// Effect must also be included.
    pub effect: GTFSRealtimeEffect, // 7 [enum]
    /// The URL which provides additional information about the alert.
    pub url: Option<GTFSRealtimeTranslatedString>, // 8 [message]
    /// Alert header. Contains a short summary of the alert text as plain-text.
    pub header_text: Option<GTFSRealtimeTranslatedString>, // 10 [message]
    /// Full description for the alert as plain-text. The information in the
    /// description should add to the information of the header.
    pub description_text: Option<GTFSRealtimeTranslatedString>, // 11 [message]
    /// Text for alert header to be used in text-to-speech implementations. This field is the
    /// text-to-speech version of header_text.
    pub tts_header_text: Option<GTFSRealtimeTranslatedString>, // 12 [message]
    /// Text for full description for the alert to be used in text-to-speech implementations.
    /// This field is the text-to-speech version of description_text.
    pub tts_description_text: Option<GTFSRealtimeTranslatedString>, // 13 [message]
    /// Severity of this alert.
    pub severity_level: GTFSRealtimeSeverityLevel, // 14 [enum]
    /// TranslatedImage to be displayed along the alert text. Used to explain visually the alert effect of a detour, station closure, etc. The image must enhance the understanding of the alert. Any essential information communicated within the image must also be contained in the alert text.
    /// The following types of images are discouraged : image containing mainly text, marketing or branded images that add no additional information.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub image: Option<GTFSRealtimeTranslatedString>, // 15 [message]
    /// Text describing the appearance of the linked image in the `image` field (e.g., in case the image can't be displayed
    /// or the user can't see the image for accessibility reasons). See the HTML spec for alt image text - <https://html.spec.whatwg.org/#alt>.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future
    pub image_alternative_text: Option<GTFSRealtimeTranslatedString>, // 16 [message]
    /// Description of the cause of the alert that allows for agency-specific language, more specific than the Cause. If cause_detail is included, then Cause must also be included.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub cause_detail: Option<GTFSRealtimeTranslatedString>, // 17 [message]
    /// Description of the effect of the alert that allows for agency-specific language, more specific than the Effect. If effect_detail is included, then Effect must also be included.
    /// NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
    pub effect_detail: Option<GTFSRealtimeTranslatedString>, // 18 [message]
}
/// Read in the contents of the GTFSRealtimeAlert
impl ProtoRead for GTFSRealtimeAlert {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut time_range = GTFSRealtimeTimeRange::default();
                pb.read_message(&mut time_range);
                self.active_periods.push(time_range);
            }
            5 => {
                let mut entity_selector = GTFSRealtimeEntitySelector::default();
                pb.read_message(&mut entity_selector);
                self.informed_entities.push(entity_selector);
            }
            6 => self.cause = pb.read_varint(),
            7 => self.effect = pb.read_varint(),
            8 => {
                let mut translated_string = GTFSRealtimeTranslatedString::default();
                pb.read_message(&mut translated_string);
                self.url = Some(translated_string);
            }
            10 => {
                let mut translated_string = GTFSRealtimeTranslatedString::default();
                pb.read_message(&mut translated_string);
                self.header_text = Some(translated_string);
            }
            11 => {
                let mut translated_string = GTFSRealtimeTranslatedString::default();
                pb.read_message(&mut translated_string);
                self.description_text = Some(translated_string);
            }
            12 => {
                let mut translated_string = GTFSRealtimeTranslatedString::default();
                pb.read_message(&mut translated_string);
                self.tts_header_text = Some(translated_string);
            }
            13 => {
                let mut translated_string = GTFSRealtimeTranslatedString::default();
                pb.read_message(&mut translated_string);
                self.tts_description_text = Some(translated_string);
            }
            14 => self.severity_level = pb.read_varint(),
            // NOTE: These are still experimental fields not yet added. Keeping them here for future
            // use.
            // 15 => {
            //     let mut translated_string = GTFSRealtimeTranslatedString::default();
            //     pb.read_message(&mut translated_string);
            //     self.image = Some(translated_string);
            // }
            // 16 => {
            //     let mut translated_string = GTFSRealtimeTranslatedString::default();
            //     pb.read_message(&mut translated_string);
            //     self.image_alternative_text = Some(translated_string);
            // }
            // 17 => {
            //     let mut translated_string = GTFSRealtimeTranslatedString::default();
            //     pb.read_message(&mut translated_string);
            //     self.cause_detail = Some(translated_string);
            // }
            // 18 => {
            //     let mut translated_string = GTFSRealtimeTranslatedString::default();
            //     pb.read_message(&mut translated_string);
            //     self.effect_detail = Some(translated_string);
            // }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// A time interval. The interval is considered active at time 't' if 't' is
/// greater than or equal to the start time and less than the end time.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTimeRange {
    /// Start time, in POSIX time (i.e., number of seconds since January 1st 1970
    /// 00:00:00 UTC).
    /// If missing, the interval starts at minus infinity.
    pub start: Option<Date>, // 1 [uint64]
    /// End time, in POSIX time (i.e., number of seconds since January 1st 1970
    /// 00:00:00 UTC).
    /// If missing, the interval ends at plus infinity.
    pub end: Option<Date>, // 2 [uint64]
}
/// Read in the contents of the GTFSRealtimeTimeRange
impl ProtoRead for GTFSRealtimeTimeRange {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.start = Some(Date::from_time(pb.read_varint::<u64>() as i64 * 1000)),
            2 => self.end = Some(Date::from_time(pb.read_varint::<u64>() as i64 * 1000)),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
