use crate::{
    parsers::{image_decoder_buffer, jpeg_decoder},
    util::{decompress_fflate, decompress_lzw},
};
use alloc::{vec, vec::Vec};

/// Internal interface for decoder
pub type Decoder = fn(buffer: &[u8], tables: Option<&[u8]>) -> Vec<u8>;

/// Returns the decoder function matching the given compression value
///
/// @param compression - the encoded compression value
/// @returns the decoder function matching the given compression value
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
            image_decoder_buffer(&(buffer.to_vec().into()), None).take()
        }),
        7 => {
            Some(|buffer: &[u8], tables: Option<&[u8]>| -> Vec<u8> { jpeg_decoder(buffer, tables) })
        }
        32773 => {
            Some(|buffer: &[u8], _tables: Option<&[u8]>| -> Vec<u8> { packbits_decoder(buffer) })
        }
        _ => panic!("Unsupported compression: {}", compression),
    }
}

/// Packbits decoder
///
/// @param buffer - an array of packed bits in a block
/// @returns the decoded array
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
            out.extend(std::iter::repeat(value).take(count));
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
