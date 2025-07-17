/// WKT to JSON Conversion
pub mod object;

use alloc::string::String;
pub use object::*;

/// Cleans up a string by trimming whitespace, removing surrounding quotes, and normalizing spaces.
pub fn clean_string(s: &str) -> String {
    s.trim() // Remove whitespace at the start and end
        .trim_matches(|c: char| c == '\'' || c == '\"' || c.is_control() || c == '\u{0000}') // Remove single or double quotes from start and end
        .replace(char::is_whitespace, " ") // Replace multiple spaces with a single space
}
