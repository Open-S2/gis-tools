use alloc::{string::String, vec::Vec};
use pbf::{ProtoRead, Protobuf};

/// An internationalized message containing per-language versions of a snippet of
/// text or a URL.
/// One of the strings from a message will be picked up. The resolution proceeds
/// as follows:
/// 1. If the UI language matches the language code of a translation,
///    the first matching translation is picked.
/// 2. If a default UI language (e.g., English) matches the language code of a
///    translation, the first matching translation is picked.
/// 3. If some translation has an unspecified language code, that translation is
///    picked.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTranslatedString {
    /// At least one translation must be provided.
    pub translations: Vec<GTFSRealtimeTranslation>, // 1 [message]
}
impl GTFSRealtimeTranslatedString {
    /// Converts the GTFSRealtimeTranslatedString to a String
    pub fn to_string(&self, ui_language: Option<&str>) -> String {
        let ui_language = ui_language.unwrap_or("en");
        self.translations
            .iter()
            .find(|t| t.language.as_ref().map(|l| l == ui_language).unwrap_or(false))
            .map(|t| t.text.clone())
            .unwrap_or_else(|| self.translations[0].text.clone())
    }
}
/// Read in the contents of the GTFSRealtimeTranslatedString
impl ProtoRead for GTFSRealtimeTranslatedString {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => {
                let mut translation = GTFSRealtimeTranslation::default();
                pb.read_message(&mut translation);
                self.translations.push(translation);
            }
            _ => panic!("unknown tag {}", tag),
        }
    }
}

/// The translations field of a GTFSRealtimeTranslatedString
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GTFSRealtimeTranslation {
    /// A UTF-8 string containing the message.
    pub text: String, // 1
    /// BCP-47 language code. Can be omitted if the language is unknown or if
    /// no i18n is done at all for the feed. At most one translation is
    /// allowed to have an unspecified language tag.
    pub language: Option<String>, // 2
}
/// Read in the contents of the GTFSRealtimeTranslation
impl ProtoRead for GTFSRealtimeTranslation {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.text = pb.read_string(),
            2 => self.language = Some(pb.read_string()),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
