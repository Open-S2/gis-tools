mod tables;

use crate::parsers::{BufferReader, Reader};
use tables::Grib2Table6_0 as BitMapIndicator;
pub use tables::*;

/// # Bit-Map Section
///
/// ## Links
/// - [Consult with this page to understand their purpose.](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ).
/// - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect6.shtml).
/// @param section - The byte block to understan how to parse bit-map data
/// @returns - Parsed bit-map section
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2BitMapSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Bit-map indicator (See [Table 6.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table6-0.shtml))
    pub bit_map_indicator: BitMapIndicator,
    /// Bit-map
    pub bit_map: Option<BufferReader>,
}
impl Grib2BitMapSection {
    /// Create a new Grib2BitMapSection
    pub fn new<T: Reader>(section: &T) -> Grib2BitMapSection {
        let indicator = section.uint8(Some(5));
        Grib2BitMapSection {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            bit_map_indicator: indicator.into(),
            bit_map: if indicator == 0 {
                Some(BufferReader::new(section.slice(Some(6), None)))
            } else {
                None
            },
        }
    }
}
