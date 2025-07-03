mod tables;
mod templates;

use crate::parsers::Reader;
pub use tables::*;
pub use templates::*;

// import { getGrib2Template5 } from './templates.js';
// import { grib2LookupTable50 } from './tables.js';

// import type { Reader } from '../../../index.js';

// export * from './templates.js';

// /** The output of `parseGrib2Section5` */
// export type Grib2DataRepresentationSection = ReturnType<typeof parseGrib2Section5>;

///  Data Representation Section
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect5.shtml)
///
/// @param section - The raw section data to parse
/// @returns - Parsed Data Representation Information
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2DataRepresentationSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Number of data points where one or more values are specified in Section 7 when a bit map is present, total number of data points when a bit map is absent.
    pub number_of_data_points: u32,
    /// Data representation template number (See [Table 5.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-0.shtml))
    pub data_representation_template: Grib2Table5_0,
    /// Data representation built using a template
    pub data_representation: Grib2Template5,
}
impl Grib2DataRepresentationSection {
    /// Create a new instance of Grib2DataRepresentationSection
    pub fn new<T: Reader>(section: &T) -> Self {
        Grib2DataRepresentationSection {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            number_of_data_points: section.uint32_be(Some(5)),
            data_representation_template: (section.uint16_be(Some(9))).into(),
            data_representation: Grib2Template5::new(section, section.uint16_be(Some(9)) as u8),
        }
    }
}
