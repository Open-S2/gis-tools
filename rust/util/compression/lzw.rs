use alloc::{vec, vec::Vec};
use core::ops::Div;

const LZW_MIN_BITS: usize = 9;
const LZW_CLEAR_CODE: usize = 256; // clear code
const LZW_EOI_CODE: usize = 257; // end of information
const LZW_MAX_BYTELENGTH: usize = 12;

/// Get a byte from an array
///
/// @param array - The array to read the byte from
/// @param position - The position to read the byte from
/// @param length - The length of the byte
/// @returns - The byte
fn lzw_get_byte(array: &[u8], position: usize, length: usize) -> usize {
    let d = position % 8;
    let a = position.div(8);
    let de = 8 - d;
    let ef = position + length - (a + 1) * 8;
    let fg: isize = 8 * (a + 2) as isize - (position + length) as isize;
    let dg = (a + 2) * 8 - position;
    let fg = isize::max(0, fg) as usize;
    if a >= array.len() {
        // panic!("ran off the end of the buffer before finding LZW_EOI_CODE (end on input code)");
        return LZW_EOI_CODE;
    }
    let mut chunk1 = (array[a] as usize) & (2_usize.pow(8 - (d as u32)) - 1);
    chunk1 <<= length - de;
    let mut chunks = chunk1;
    if a + 1 < array.len() {
        let mut chunk2 = (array[a + 1] as usize) >> fg;
        let chunk2_shift = isize::max(0, (length as isize) - (dg as isize)) as usize;
        chunk2 <<= chunk2_shift;
        chunks += chunk2;
    }
    if ef > 8 && a + 2 < array.len() {
        let hi = (a + 3) * 8 - (position + length);
        let chunk3 = (array[a + 2] as usize) >> hi;
        chunks += chunk3;
    }

    chunks
}

/// Append an array in reverse
///
/// @param dest - The array to append to
/// @param source - The array to append
/// @returns - The dest array
fn append_reversed(dest: &mut Vec<u8>, source: &[u8]) {
    for i in (0..source.len()).rev() {
        dest.push(source[i]);
    }
}

struct LZWDecoder {
    /// The dictionary index
    pub dictionary_index: Vec<u16>,
    /// The dictionary chars
    pub dictionary_char: Vec<u8>,
    /// The dictionary length
    pub dictionary_length: usize,
    /// The byte length
    pub byte_length: usize,
    /// The position
    pub position: usize,
}
impl LZWDecoder {
    /// Create a new LZWDecoder
    pub fn new() -> LZWDecoder {
        let mut entry = LZWDecoder {
            dictionary_index: vec![0_u16; 4093],
            dictionary_char: vec![0_u8; 4093],
            dictionary_length: 258,
            byte_length: LZW_MIN_BITS,
            position: 0,
        };

        for i in 0..=257 {
            entry.dictionary_index[i] = 4096;
            entry.dictionary_char[i] = i as u8;
        }

        entry
    }

    /// Initializes the dictionary
    pub fn init_dictionary(&mut self) {
        self.dictionary_length = 258;
        self.byte_length = LZW_MIN_BITS;
    }

    /// Go Next
    pub fn get_next(&mut self, array: &[u8]) -> usize {
        let byte = lzw_get_byte(array, self.position, self.byte_length);
        self.position += self.byte_length;
        byte
    }

    /// Add to the dictionary
    pub fn add_to_dictionary(&mut self, i: usize, c: u8) -> usize {
        self.dictionary_char[self.dictionary_length] = c;
        self.dictionary_index[self.dictionary_length] = i as u16;
        self.dictionary_length += 1;
        self.dictionary_length - 1
    }

    /// Get the dictionary reversed
    pub fn get_dictionary_reversed(&self, n: usize) -> Vec<u8> {
        let mut rev = vec![];
        let mut i = n;
        while i != 4096 {
            rev.push(self.dictionary_char[i]);
            i = self.dictionary_index[i] as usize;
        }
        rev
    }
}

/// Decompress the LZW data
///
/// @param input - The LZW data
/// @returns - The decompressed data
pub fn decompress_lzw(input: &[u8]) -> Vec<u8> {
    let mut entry = LZWDecoder::new();

    let mut result = vec![];
    entry.init_dictionary();
    let mut code = entry.get_next(input);
    let mut old_code: Option<usize> = None;
    while code != LZW_EOI_CODE {
        if code == LZW_CLEAR_CODE {
            entry.init_dictionary();
            code = entry.get_next(input);
            while code == LZW_CLEAR_CODE {
                code = entry.get_next(input);
            }

            if code == LZW_EOI_CODE {
                break;
            } else if code > LZW_CLEAR_CODE {
                panic!("corrupted code at scanline {code}");
            } else {
                let val = entry.get_dictionary_reversed(code);
                append_reversed(&mut result, &val);
                old_code = Some(code);
            }
        } else if code < entry.dictionary_length {
            let val = entry.get_dictionary_reversed(code);
            append_reversed(&mut result, &val);
            entry.add_to_dictionary(old_code.unwrap_or(code), val[val.len() - 1]);
            old_code = Some(code);
        } else {
            let old_val = entry.get_dictionary_reversed(old_code.unwrap_or(code));
            if old_val.is_empty() {
                panic!(
                    "Bogus entry. Not in dictionary, {:?} / {}, position: {}",
                    old_code, entry.dictionary_length, entry.position
                );
            }
            append_reversed(&mut result, &old_val);
            result.push(old_val[old_val.len() - 1]);
            entry.add_to_dictionary(old_code.unwrap_or(code), old_val[old_val.len() - 1]);
            old_code = Some(code);
        }

        let two_pow = 2_usize.pow(entry.byte_length as u32);
        if entry.dictionary_length + 1 >= two_pow {
            if entry.byte_length == LZW_MAX_BYTELENGTH {
                old_code = None;
            } else {
                entry.byte_length += 1;
            }
        }
        code = entry.get_next(input);
    }
    result
}
