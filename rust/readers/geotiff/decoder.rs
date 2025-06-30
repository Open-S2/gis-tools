use crate::{
    parsers::{image_decoder_buffer, jpeg_decoder},
    util::{decompress_fflate, decompress_lzw},
};
use alloc::{vec, vec::Vec};

/// Internal interface for decoder
pub type Decoder = fn(buffer: &[u8], tables: Option<&[u8]>) -> Vec<u8>;

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
        6 | 50001 => Some(|buffer: &[u8], _tables: Option<&[u8]>| -> Vec<u8> {
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

/**
/// Packbits decoder
/// @param buffer - an array of packed bits in a block
/// @returns the decoded array
 */
pub fn packbits_decoder(buffer: &[u8]) -> Vec<u8> {
    let mut out = vec![];

    let mut i: usize = 0;
    while i < buffer.len() {
        let mut header = buffer[i] as isize;
        if header < 0 {
            let next = buffer[i + 1];
            header = -header;
            let mut j = 0;
            while j <= header {
                out.push(next);
                j += 1;
            }
            i += 1;
        } else {
            let mut j: isize = 0;
            while j <= header {
                out.push(buffer[((i as isize) + j + 1) as usize]);
                j += 1;
            }
            i = ((i as isize) + header + 1) as usize;
        }
        i += 1;
    }

    out.to_vec()
}
