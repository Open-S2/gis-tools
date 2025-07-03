use crate::{
    parsers::{BufferReader, Reader},
    readers::{
        Grib2DataRepresentationSection, Grib2Sections, Grib2Table5_0 as DataType, Grib2Template50,
    },
};
use alloc::vec::Vec;
use libm::pow;

// TODO: case 42: AEC https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/aecunpack.c

/// Decode GRIB2 Template 7.X
/// Converts data Buffer according to data representation section
///
/// @param reader - The raw data to convert
/// @param sections - The sections of the GRIB2 message that have been parsed so far
/// @returns Converted data
pub fn grib2_template_7_decoder(data: &BufferReader, sections: &Grib2Sections) -> Vec<f64> {
    let drs = sections.data_representation.as_ref().unwrap_or_else(|| {
        panic!("Data Representation Section is not defined");
    });
    let data_representation_template = drs.data_representation_template;

    match data_representation_template {
        DataType::GridPointDataSimplePacking => simple_unpacking(data, drs),
        DataType::GridPointDataComplexPacking
        | DataType::GridPointDataComplexPackingAndSpatialDifferencing => {
            unimplemented!("Complex unpacking not implemented yet")
        }
        DataType::GridPointDataJpeg2000CodeStreamFormat
        | DataType::GridPointDataJpeg2000CodeStreamFormatAndSpatialDifferencing => {
            unimplemented!("JPEG2000 not implemented yet")
        }
        DataType::SpectralDataSimplePacking => {
            unimplemented!("Spectral simple packing not implemented yet")
        }
        DataType::SpectralDataComplexPacking => {
            unimplemented!("Spectral complex packing not implemented yet")
        }
        DataType::GridPointDataPortableNetworkGraphicsPng => {
            unimplemented!("PNG not implemented yet")
        }
        _ => panic!("Template 7.{data_representation_template} not defined"),
    }
}

/// # Data Template 7.0 - Grid point data - simple packing
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-0.shtml)
///
/// @param reader - The raw data to convert
/// @param drs - The data representation section
/// @returns - The converted data
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
    let number_of_bits = *number_of_bits as u8;
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

// /**
//  * # Data Template 7.40 - Grid point data - JPEG 2000 code stream format
//  *
//  * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-40.shtml)
//  * @param reader - The raw data to convert
//  * @param drs - The data representation section
//  * @param bms - The bit map section
//  * @returns - The converted data
//  */
// export function jpeg2000Unpacking(
//   reader: Reader,
//   drs: Grib2DataRepresentationSection,
//   bms: Grib2BitMapSection,
// ): number[] {
//   const jpx = new JpxImage(reader);

//   if (jpx.componentsCount !== 1)
//     throw new Error('JPEG Decoder: Only single component is supported');
//   if (jpx.tiles.length !== 1) throw new Error('JPEG Decoder: Only single tile is supported');
//   if (jpx.tiles[0].height !== 1)
//     throw new Error('JPEG Decoder: Only single row (1xN) is supported');

//   const { bitMap: bitBuffer, bitMapIndicator } = bms;
//   const { numberOfDataPoints, dataRepresentation } = drs;
//   const { decimal_scale_factor, reference_value, binary_scale_factor, number_of_bits } =
//     dataRepresentation;

//   const DD = Math.pow(10, decimal_scale_factor);
//   const EE = Math.pow(2, binary_scale_factor);

//   const result: number[] = [];

//   if (number_of_bits === 0) {
//     for (let i = 0; i < numberOfDataPoints; i++) result.push(reference_value);
//     return result;
//   }

//   // A bit map applies to this product
//   if (bitMapIndicator.code === 0 && bitBuffer !== null) {
//     const bitMapData = new Uint8Array(bitBuffer.buffer, bitBuffer.byteOffset, bitBuffer.byteLength);
//     for (let i = 0; i < numberOfDataPoints; i++) {
//       // Apply bit map to the data.
//       // Length of data values is often smaller than the bit map itself. Bitmap is used to
//       // indicate which data values are present, 1 bit meaning is present, 0 bit meaning is missing, -1 meaning undefined.
//       // [Read more](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ)
//       const byte = bitMapData[Math.floor(i / 8)];
//       if ((byte & (1 << i % 8)) !== 0) {
//         result.push((reference_value + jpx.tiles[0].items[i] * EE) / DD);
//       } else {
//         result.push(Number.NEGATIVE_INFINITY);
//       }
//     }
//   } else {
//     // Do not use `.map` on Uint8Array, as it clamps the values to 0-255
//     for (const byte of jpx.tiles[0].items) result.push((reference_value + byte * EE) / DD);
//   }

//   return result;
// }
