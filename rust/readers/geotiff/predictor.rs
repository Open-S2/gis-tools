use alloc::vec::Vec;
use std::ops::AddAssign;

/// Decode a block using the specified predictor (u8 version)
pub fn decode_row_acc_u8(row: &mut [u8], stride: usize) {
    let mut length = row.len().saturating_sub(stride);
    let mut offset = 0;
    while length > 0 {
        for _ in 0..stride {
            row[offset + stride] = row[offset + stride].wrapping_add(row[offset]);
            offset += 1;
        }
        length -= stride;
    }
}

/// Decode a block using the specified predictor (u16 version)
pub fn decode_row_acc_u16(row: &mut [u16], stride: usize) {
    let mut length = row.len().saturating_sub(stride);
    let mut offset = 0;
    while length > 0 {
        for _ in 0..stride {
            row[offset + stride] = row[offset + stride].wrapping_add(row[offset]);
            offset += 1;
        }
        length -= stride;
    }
}

/// Decode a block using the specified predictor (u32 version)
pub fn decode_row_acc_u32(row: &mut [u32], stride: usize) {
    let mut length = row.len().saturating_sub(stride);
    let mut offset = 0;
    while length > 0 {
        for _ in 0..stride {
            row[offset + stride] = row[offset + stride].wrapping_add(row[offset]);
            offset += 1;
        }
        length -= stride;
    }
}

/// Decode a floating point block using the specified predictor
///
/// @param row - the row to decode
/// @param stride - the number of bytes per row
/// @param bytes_per_sample - the number of bytes per sample
fn decode_row_floating_point<T>(row: &mut [T], stride: usize, bytes_per_sample: usize)
where
    T: AddAssign + Copy,
{
    let mut index = 0;
    let mut count = row.len();
    let wc = count / bytes_per_sample;

    while count > stride {
        let mut i = stride;
        while i > 0 {
            row[index + stride] += row[index];
            index += 1;
            i -= 1;
        }
        count -= stride;
    }

    let copy: Vec<T> = row.to_vec();
    for i in 0..wc {
        for b in 0..bytes_per_sample {
            row[bytes_per_sample * i + b] = copy[(bytes_per_sample - b - 1) * wc + i];
        }
    }
}

/// Apply the specified predictor to a block
///
/// @param block - the block to modify
/// @param predictor - the predictor
/// @param width - the block width
/// @param height - the block height
/// @param bits_per_sample - the number of bits per sample
/// @param planar_configuration - the planar configuration
/// @returns - the modified block
pub fn apply_predictor(
    mut block: Vec<u8>,
    predictor: i16,
    width: usize,
    height: usize,
    bits_per_sample: Vec<u16>,
    planar_configuration: i16,
) -> Vec<u8> {
    if predictor == 0 || predictor == 1 {
        return block;
    }

    for i in 0..bits_per_sample.len() {
        if bits_per_sample[i] % 8 != 0 {
            panic!("When decoding with predictor, only multiple of 8 bits are supported.");
        }
        if bits_per_sample[i] != bits_per_sample[0] {
            panic!("When decoding with predictor, all samples must have the same size.");
        }
    }

    let bytes_per_sample = (bits_per_sample[0] / 8) as usize;
    let stride = if planar_configuration == 2 { 1 } else { bits_per_sample.len() };

    for i in 0..height {
        // Last strip will be truncated if height % stripHeight != 0
        if i * stride * width * bytes_per_sample >= block.len() {
            break;
        }
        if predictor == 2 {
            // horizontal prediction
            let row = &mut block[i * stride * width * bytes_per_sample
                ..(i + 1) * stride * width * bytes_per_sample];
            match bits_per_sample[0] {
                8 => {
                    decode_row_acc_u8(row, stride);
                }
                16 => {
                    decode_row_acc_u16(as_u16_slice_mut(row), stride);
                }
                32 => {
                    decode_row_acc_u32(as_u32_slice_mut(row), stride);
                }
                _ => panic!("Predictor 2 not allowed with {} bits per sample.", bits_per_sample[0]),
            }
        } else if predictor == 3 {
            // horizontal floating point
            let row = &mut block[i * stride * width * bytes_per_sample
                ..(i + 1) * stride * width * bytes_per_sample];
            decode_row_floating_point(row, stride, bytes_per_sample);
        }
    }

    block
}

fn as_u16_slice_mut(data: &mut [u8]) -> &mut [u16] {
    assert_eq!(data.len() % 2, 0);
    let ptr = data.as_mut_ptr() as *mut u16;
    let len = data.len() / 2;
    assert_eq!(ptr.align_offset(core::mem::align_of::<u16>()), 0);
    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}

fn as_u32_slice_mut(data: &mut [u8]) -> &mut [u32] {
    assert_eq!(data.len() % 4, 0);
    let ptr = data.as_mut_ptr() as *mut u32;
    let len = data.len() / 4;
    assert_eq!(ptr.align_offset(core::mem::align_of::<u32>()), 0);
    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}
