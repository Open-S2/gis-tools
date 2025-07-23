mod tables;
mod tables2;
mod tables3;
mod templates;

use crate::{parsers::Reader, readers::Grib2Sections};
pub use tables::*;
pub use tables2::*;
pub use tables3::*;
pub use templates::*;

///  Product Definition Section
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect4.shtml)
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2ProductDefinitionSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Number of coordinate values after template
    pub coordinate_values: u16,
    /// Product definition template number [Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
    pub product_definition_template: Grib2Table4_0,
    /// Product definition
    pub values: Grib2ProductDefinition,
}
impl Grib2ProductDefinitionSection {
    /// Create a new instance of Grib2ProductDefinitionSection
    ///
    /// ## Parameters
    /// - `reader`: The section to parse
    /// - `sections`: The other sections that have been parsed (1-3)
    ///
    /// ## Returns
    /// Parsed Product Definition Information
    pub fn new<T: Reader>(reader: &T, sections: &Grib2Sections) -> Self {
        let product_definition_template = reader.uint16_be(Some(7));

        Self {
            section_number: reader.uint8(Some(4)),
            length: reader.uint32_be(Some(0)),
            coordinate_values: reader.uint16_be(Some(5)),
            product_definition_template: product_definition_template.into(),
            values: Grib2ProductDefinition::new(product_definition_template, reader, sections),
        }
    }
}
