/// Import section 0
pub mod _0;
/// Import section 1
pub mod _1;
/// Import section 2
pub mod _2;
/// Import section 3
pub mod _3;
/// Import section 4
pub mod _4;
/// Import section 5
pub mod _5;
/// Import section 6
pub mod _6;
/// Import section 7
pub mod _7;
/// Import section 8
pub mod _8;
/// Import other
pub mod other;

use crate::parsers::{BufferReader, Reader};
pub use _0::*;
pub use _1::*;
pub use _2::*;
pub use _3::*;
pub use _4::*;
pub use _5::*;
pub use _6::*;
pub use _7::*;
pub use _8::*;
pub use other::*;

/// A parsed GRIB file and it's sections
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Grib2Sections {
    /// The Indicator Section
    pub indicator: Option<Grib2IndicatorSection>,
    /// The Identification Section
    pub identification: Option<Grib2IdentificationSection>,
    /// The Local Use Section
    pub local: Option<Grib2LocalUseSection>,
    /// The Grid Definition Section
    pub grid_definition: Option<GridDefinitionSection>,
    /// The Product Definition Section
    pub product_definition: Option<Grib2ProductDefinitionSection>,
    /// The Data Representation Section
    pub data_representation: Option<Grib2DataRepresentationSection>,
    /// The Bit Map Section
    pub bit_map: Option<Grib2BitMapSection>,
    /// The Data Section
    pub data: Option<Grib2DataSection>,
    /// The End Section
    pub end: Option<Grib2EndSection>,
}

/// Split the bytes of the GRIB file into individual GRIB chunks that represent sections
///
/// @param grib_chunk Buffer containing individual GRIB definition
/// @returns Array of Section Buffers where the index of the item corresponds to the section number. If a section is missing, it will be represented as null
pub fn split_section_chunks(grib_chunk: BufferReader) -> Grib2Sections {
    let mut sections = Grib2Sections::default();

    let mut current_section = grib_chunk;
    // Split sections in file
    while current_section.len() != 0 {
        let section_number = get_section_number(&current_section);

        // First section length is always 16 bytes long and is identified by the first 4 bytes being 'GRIB'
        let mut length =
            if section_number == 0 { 16 } else { current_section.uint32_be(Some(0)) } as u64;
        length = length.min(current_section.len());
        let section = BufferReader::new(current_section.slice(Some(0), Some(length)));
        current_section = BufferReader::new(current_section.slice(Some(length), None));

        parse_grib2_section(&section, &mut sections);
    }

    sections
}

/// Parse the given section
///
/// @param reader - The section to parse
/// @param sections - The result to write to
fn parse_grib2_section(reader: &BufferReader, sections: &mut Grib2Sections) {
    let section_number = get_section_number(reader);

    match section_number {
        0 => sections.indicator = Some(Grib2IndicatorSection::new(reader)),
        1 => sections.identification = Some(Grib2IdentificationSection::new(reader)),
        2 => sections.local = Some(Grib2LocalUseSection::new(reader)),
        3 => sections.grid_definition = Some(GridDefinitionSection::new(reader)),
        4 => {
            sections.product_definition = Some(Grib2ProductDefinitionSection::new(reader, sections))
        }
        5 => sections.data_representation = Some(Grib2DataRepresentationSection::new(reader)),
        6 => sections.bit_map = Some(Grib2BitMapSection::new(reader)),
        7 => sections.data = Some(Grib2DataSection::new(reader)),
        8 => sections.end = Some(Grib2EndSection::new(reader)),
        _ => panic!("Unknown section number: {section_number}"),
    }
}

/// Get the section number
/// @param section Buffer containing GRIB Section data
/// @returns Section number of the input GRIB Section data
pub fn get_section_number(section: &BufferReader) -> u8 {
    let first4_byte_string = section.parse_string(Some(0), Some(4));

    match first4_byte_string.as_ref() {
        "GRIB" => 0,
        "7777" => 8,
        _ => section.uint8(Some(4)),
    }
}
