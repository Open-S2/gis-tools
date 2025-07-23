use crate::parsers::Reader;
use alloc::string::String;

/// # SECTION 8 - END SECTION
///
/// ## Links
/// - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect8.shtml)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grib2EndSection {
    /// "7777" - Coded according to the International Alphabet Number 5
    pub end_encoded: String,
}
impl Grib2EndSection {
    /// Create a new Grib2EndSection
    ///
    /// ## Parameters
    /// - `section`: byte block for section 8
    ///
    /// ## Returns
    /// Parsed end section
    pub fn new<T: Reader>(section: &T) -> Grib2EndSection {
        Grib2EndSection { end_encoded: section.parse_string(Some(0), Some(4)) }
    }
}
