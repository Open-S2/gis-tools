use crate::{
    parsers::jpeg_decoder,
    util::{decompress_fflate, decompress_lzw},
};
use alloc::{vec, vec::Vec};

/// Internal interface for decoder
pub type Decoder = fn(buffer: &[u8], tables: Option<&[u8]>) -> Vec<u8>;

// /// Decodes a JPEG buffer using the `image` crate, handling GeoTIFF tables.
// ///
// /// This function handles the two cases for JPEG-in-TIFF:
// /// 1. `tables` is `Some`: This is the GeoTIFF case. We must re-assemble a valid
// ///    JPEG stream by prepending the Start of Image (SOI) marker and the
// ///    provided tables to the tile/strip buffer.
// /// 2. `tables` is `None`: The buffer is a standard, self-contained JPEG.
// #[cfg(feature = "std")]
// fn std_jpeg_decoder(buffer: &[u8], tables: Option<&[u8]>) -> Vec<u8> {
//     use image::ImageDecoder;
//     use image::codecs::jpeg::JpegDecoder;
//     use std::io::Cursor;

//     if let Some(tables_data) = tables {
//         // Case 1: GeoTIFF-specific JPEG.

//         // Strip the EOI marker (last 2 bytes) from the tables

//         let tables_part = &tables_data[..tables_data.len() - 2];
//         // Strip the SOI marker (first 2 bytes) from the tile data
//         let buffer_part = &buffer[2..];
//         // Concatenate them to form a single, valid JPEG stream
//         let mut jpeg_stream = Vec::with_capacity(tables_part.len() + buffer_part.len());
//         jpeg_stream.extend_from_slice(tables_part);
//         jpeg_stream.extend_from_slice(buffer_part);

//         let reader = Cursor::new(&jpeg_stream);
//         let decoder =
//             JpegDecoder::new(reader).expect("Failed to create JPEG decoder with stitched tables");
//         let mut pixels = vec![0; decoder.total_bytes() as usize];
//         decoder.read_image(&mut pixels).expect("Failed to decode re-assembled JPEG data");
//         // println!("SUCCESS! {}", pixels.len());
//         pixels
//     } else {
//         // Case 2: Standard, self-contained JPEG.
//         let reader = Cursor::new(buffer);
//         let decoder = JpegDecoder::new(reader).expect("Failed to create standard JPEG decoder");

//         let mut pixels = vec![0; decoder.total_bytes() as usize];
//         decoder.read_image(&mut pixels).expect("Failed to decode standard JPEG data");
//         pixels
//     }
// }

/// Returns the decoder function matching the given compression value
///
/// ## Parameters
/// - `compression`: the encoded compression value
///
/// ## Returns
/// The decoder function matching the given compression value
pub fn get_decoder(compression: Option<u16>) -> Option<Decoder> {
    let compression = compression.unwrap_or(1);
    match compression {
        1 => None,
        5 => Some(|buffer: &[u8], _tables: Option<&[u8]>| -> Vec<u8> { decompress_lzw(buffer) }),
        8 | 32946 => Some(|buffer: &[u8], dict: Option<&[u8]>| -> Vec<u8> {
            decompress_fflate(buffer, dict).unwrap()
        }),
        #[cfg(feature = "std")]
        6 | 256 | 50001 => Some(|buffer: &[u8], _tables: Option<&[u8]>| -> Vec<u8> {
            crate::parsers::image_decoder_buffer(&(buffer.to_vec().into()), None).take()
        }),
        7 => {
            Some(|buffer: &[u8], tables: Option<&[u8]>| -> Vec<u8> { jpeg_decoder(buffer, tables) })
        }
        // 7 => {
        //     #[cfg(feature = "std")]
        //     {
        //         Some(std_jpeg_decoder)
        //     }
        //     #[cfg(not(feature = "std"))]
        //     {
        //         Some(|buffer: &[u8], tables: Option<&[u8]>| -> Vec<u8> {
        //             crate::parsers::jpeg_decoder(buffer, tables)
        //         })
        //     }
        // }
        32773 => {
            Some(|buffer: &[u8], _tables: Option<&[u8]>| -> Vec<u8> { packbits_decoder(buffer) })
        }
        _ => panic!("Unsupported compression: {}", compression),
    }
}

/// Packbits decoder
///
/// ## Parameters
/// - `buffer`: an array of packed bits in a block
///
/// ## Returns
/// the decoded array
pub fn packbits_decoder(buffer: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    let mut i = 0;

    while i < buffer.len() {
        let header = buffer[i] as i8;

        if header < 0 {
            // Negative header: repeat next byte (-header + 1) times
            let count = (-(header as isize) + 1) as usize;
            i += 1;
            let value = buffer[i];
            out.extend(core::iter::repeat_n(value, count));
            i += 1;
        } else {
            // Positive header: copy next (header + 1) bytes
            let count = (header as usize) + 1;
            out.extend_from_slice(&buffer[i + 1..i + 1 + count]);
            i += count + 1;
        }
    }

    out
}
