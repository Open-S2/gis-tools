use crate::parsers::{BufferReader, Reader};

/// # SECTION 2 - LOCAL USE SECTION
///
/// ## Notes
/// 1. Center=7 (NCEP), subcenter=14(NWS Meteorological Development Laboratory (MDL))
/// used octet 6 to indicate which local use table to use. For MDL, octet 6=1 indicates use:
/// "MDL Template 2.1"
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect2.shtml)
/// @param section - The byte block to pull basic local information
/// @returns - a parsed explaination of local use.
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2LocalUseSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Section 2 Contents
    pub contents: BufferReader,
}
impl Grib2LocalUseSection {
    /// Create a new Grib2LocalUseSection
    pub fn new<T: Reader>(section: &T) -> Grib2LocalUseSection {
        Grib2LocalUseSection {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            contents: BufferReader::new(section.slice(Some(5), None)),
        }
    }
}
