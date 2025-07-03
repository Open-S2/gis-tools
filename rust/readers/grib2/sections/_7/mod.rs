mod tables;
mod templates;

use crate::{
    parsers::{BufferReader, Reader},
    readers::Grib2Sections,
};
use alloc::vec::Vec;
pub use tables::*;
pub use templates::*;

/// Data Section
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect7.shtml)
///
/// @param section - The raw section data to parse
/// @param sections - The other sections that have been parsed (1-6)
/// @returns - Parsed Data Information with a function to decode the data
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2DataSection {
    /// Number of GRIB section
    section_number: u8,
    /// Length of GRIB section
    length: u32,
    /// data that has yet to be decoded
    raw_data: BufferReader,
}
impl Grib2DataSection {
    /// Create a new instance of Grib2DataSection
    pub fn new<T: Reader>(section: &T) -> Self {
        Self {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            raw_data: BufferReader::new(section.slice(Some(5), None)),
        }
    }

    /// Data in a format described by data Template 7.X, where X is the data representation
    /// template number given in octets 10-11 of [Section 5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect5.shtml).
    ///
    /// @returns - the raw parsed data
    pub fn data(&self, sections: &Grib2Sections) -> Vec<f64> {
        grib2_template_7_decoder(&self.raw_data, sections)
    }
}
