mod tables;

use crate::parsers::Reader;
use alloc::string::String;
use tables::Grib2Table0_0 as Discipline;
pub use tables::*;

/// # SECTION 0 - INDICATOR SECTION
///
/// ## Description
/// This section serves to identify the start of the record in a human readable form,
/// indicate the total length of the message, and indicate the Edition number of GRIB used
/// to construct or encode the message. For GRIB2, this section is always 16 octets long.
///
/// ## Links
/// - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect0.shtml)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grib2IndicatorSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section (Always 16 for Section 0)
    pub length: u8,
    /// GRIB string encoded
    pub grib_encoded: String,
    /// Discipline [Table 0.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table0-0.shtml)
    pub discipline: Discipline,
    /// Edition number - 2 for GRIB2
    pub grib_edition: u8,
    /// Total length of GRIB message in octets (All sections)
    pub grib_length: u64,
}
impl Grib2IndicatorSection {
    /// Create a new Grib2IndicatorSection
    ///
    /// ## Parameters
    /// - `section`: the 16 byte metadata section
    ///
    /// ## Returns
    /// A parsed explination of the file
    pub fn new<T: Reader>(section: &T) -> Grib2IndicatorSection {
        Grib2IndicatorSection {
            section_number: 0,
            length: 16,
            grib_encoded: section.parse_string(Some(0), Some(4)),
            discipline: section.uint8(Some(6)).into(),
            grib_edition: section.uint8(Some(7)),
            grib_length: section.uint64_be(Some(8)),
        }
    }
}
