use crate::parsers::{Buffer, image_decoder};
#[cfg(feature = "std")]
use crate::{
    parsers::{BufferReader, Reader},
    readers::{
        _7::{complex_unpacking::*, jpeg2000::*},
        Grib2DataRepresentationSection, Grib2Sections, Grib2Table5_0 as DataType, Grib2Template50,
    },
};
use alloc::vec::Vec;
use libm::pow;

// TODO: case 42: AEC https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/aecunpack.c

/// Decode GRIB2 Template 7.X
/// Converts data Buffer according to data representation section
///
/// ## Parameters
/// - `reader`: The raw data to convert
/// - `sections`: The sections of the GRIB2 message that have been parsed so far
///
/// ## Returns
/// Converted data
pub fn grib2_template_7_decoder(data: &BufferReader, sections: &Grib2Sections) -> Vec<f64> {
    let drs = sections.data_representation.as_ref().unwrap_or_else(|| {
        panic!("Data Representation Section is not defined");
    });
    let data_representation_template = drs.data_representation_template;

    match data_representation_template {
        DataType::GridPointDataSimplePacking => simple_unpacking(data, drs),
        DataType::GridPointDataComplexPacking
        | DataType::GridPointDataComplexPackingAndSpatialDifferencing => {
            complex_unpacking(data, sections)
        }
        DataType::GridPointDataJpeg2000CodeStreamFormat
        | DataType::GridPointDataJpeg2000CodeStreamFormatAndSpatialDifferencing => {
            let bms = sections.bit_map.as_ref().unwrap_or_else(|| {
                panic!("Bit Map Section is not defined");
            });
            if cfg!(not(feature = "std")) {
                unimplemented!("JPEG2000 not implemented yet for no_std. Use the filesystem (std).")
            } else {
                jpeg2000_unpacking_fs(data, drs, bms)
            }
        }
        DataType::GridPointDataPortableNetworkGraphicsPng => png_unpacking(data, drs),
        DataType::SpectralDataSimplePacking => {
            unimplemented!("Spectral simple packing not implemented yet")
        }
        DataType::SpectralDataComplexPacking => {
            unimplemented!("Spectral complex packing not implemented yet")
        }
        _ => panic!("Template 7.{data_representation_template} not defined"),
    }
}

/// # Data Template 7.0 - Grid point data - simple packing
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-0.shtml)
///
/// ## Parameters
/// - `reader`: The raw data to convert
/// - `drs`: The data representation section
///
/// ## Returns
///  The converted data
pub fn simple_unpacking(reader: &BufferReader, drs: &Grib2DataRepresentationSection) -> Vec<f64> {
    let Grib2Template50 {
        decimal_scale_factor,
        binary_scale_factor,
        reference_value,
        number_of_bits,
        ..
    } = drs.data_representation.get_simple_packing_template().unwrap_or_else(|| {
        panic!("Simple packing template is not defined");
    });
    let decimal_scale_factor = *decimal_scale_factor as f64;
    let binary_scale_factor = *binary_scale_factor as f64;
    let reference_value = *reference_value as f64;
    let number_of_bits = *number_of_bits;
    let dd = pow(10., decimal_scale_factor);
    let ee = pow(2., binary_scale_factor);

    let data = reader.slice(None, None);
    let total_bits = data.len() * 8;
    let count = total_bits / (number_of_bits as usize);
    let mut values = Vec::with_capacity(count);

    let mut bit_pos = 0;

    for _ in 0..count {
        let mut acc: u32 = 0;

        for _ in 0..number_of_bits {
            let byte_index = bit_pos / 8;
            let bit_offset = 7 - (bit_pos % 8);
            let bit = (data[byte_index] >> bit_offset) & 1;

            acc = (acc << 1) | (bit as u32);
            bit_pos += 1;
        }

        let value = (reference_value + (acc as f64) * ee) / dd;
        values.push(value);
    }

    values
}

fn png_unpacking(reader: &BufferReader, drs: &Grib2DataRepresentationSection) -> Vec<f64> {
    let Grib2Template50 {
        decimal_scale_factor,
        binary_scale_factor,
        reference_value,
        number_of_bits,
        ..
    } = drs.data_representation.get_simple_packing_template().unwrap_or_else(|| {
        panic!("Simple packing template is not defined");
    });
    let decimal_scale_factor = *decimal_scale_factor as f64;
    let binary_scale_factor = *binary_scale_factor as f64;
    let reference_value = *reference_value as f64;
    let number_of_bits = *number_of_bits;
    let dd = pow(10., decimal_scale_factor);
    let ee = pow(2., binary_scale_factor);

    let data = reader.slice(None, None);
    // Parse the image
    let Ok(mut image) = image_decoder(&Buffer::from(data), None) else {
        panic!("Failed to decode image");
    };
    let data = image.data.take();

    let total_bits = data.len() * 8;
    let count = total_bits / (number_of_bits as usize);
    let mut values = Vec::with_capacity(count);

    let mut bit_pos = 0;

    for _ in 0..count {
        let mut acc: u32 = 0;

        for _ in 0..number_of_bits {
            let byte_index = bit_pos / 8;
            let bit_offset = 7 - (bit_pos % 8);
            let bit = (data[byte_index] >> bit_offset) & 1;

            acc = (acc << 1) | (bit as u32);
            bit_pos += 1;
        }

        let value = (reference_value + (acc as f64) * ee) / dd;
        values.push(value);
    }

    values
}
