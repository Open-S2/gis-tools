mod tables;
mod templates;

pub use tables::*;
pub use templates::*;

use crate::parsers::Reader;

// import { getGrib2Template3 } from './templates.js';
// import { grib2LookupTable31 } from './tables.js';

/// # SECTION 3 - GRID DEFINITION SECTION
///
/// ## Links
/// - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect3.shtml)
///
/// @param section - byte block for section 3
/// @returns - parsed grid definition
#[derive(Debug, Clone, PartialEq)]
pub struct GridDefinitionSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Source of grid definition
    pub definition_source: u8,
    /// Number of data points
    pub number_of_points: u32,
    /// Number of octets for optional list of numbers defining number of points
    pub number_of_octets: u8,
    /// Interpetation of list of numbers defining number of points [Table 3.11](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-11.shtml)
    pub interpretation: u8,
    /// Grid definition template number [Table 3.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
    pub grid_definition_template: Grib2Table3_1,
    /// Grid definition values
    pub values: Grib2Template3,
}
impl GridDefinitionSection {
    /// Create a new instance of GridDefinitionSection
    pub fn new<T: Reader>(section: &T) -> Self {
        let grid_definition_template: Grib2Table3_1 = section.uint16_be(Some(12)).into();
        Self {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            definition_source: section.uint8(Some(5)),
            number_of_points: section.uint32_be(Some(6)),
            number_of_octets: section.uint8(Some(10)),
            interpretation: section.uint8(Some(11)),
            grid_definition_template,
            values: Grib2Template3::new(grid_definition_template, section),
        }
    }
}
