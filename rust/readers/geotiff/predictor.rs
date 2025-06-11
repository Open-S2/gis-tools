use alloc::vec::Vec;
use std::ops::AddAssign;

/**
 * Decode a block using the specified predictor
 * @param row - the row to decode
 * @param stride - the number of bytes per row
 */
fn decode_row_acc<T>(row: &mut [T], stride: usize)
where
    T: AddAssign + Copy,
{
    let mut length = (row.len() - stride) as isize;
    let mut offset = 0;
    loop {
        let mut i = stride;
        while i > 0 {
            row[offset + stride] += row[offset];
            offset += 1;
            i -= 1;
        }

        length -= stride as isize;
        if length <= 0 {
            break;
        }
    }
}

/**
 * Decode a floating point block using the specified predictor
 * @param row - the row to decode
 * @param stride - the number of bytes per row
 * @param bytes_per_sample - the number of bytes per sample
 */
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

/**
 * Apply the specified predictor to a block
 * @param block - the block to modify
 * @param predictor - the predictor
 * @param width - the block width
 * @param height - the block height
 * @param bits_per_sample - the number of bits per sample
 * @param planar_configuration - the planar configuration
 * @returns - the modified block
 */
pub fn apply_predictor<T: AddAssign + Copy>(
    block: &[u8],
    predictor: usize,
    width: usize,
    height: usize,
    bits_per_sample: Vec<usize>,
    planar_configuration: usize,
) -> T {
    unimplemented!();
    //   if (predictor === 0 || predictor === 1) {
    //     return block;
    //   }

    //   for (let i = 0; i < bits_per_sample.length; ++i) {
    //     if (bits_per_sample[i] % 8 !== 0) {
    //       throw new Error('When decoding with predictor, only multiple of 8 bits are supported.');
    //     }
    //     if (bits_per_sample[i] !== bits_per_sample[0]) {
    //       throw new Error('When decoding with predictor, all samples must have the same size.');
    //     }
    //   }

    //   const bytes_per_sample = bits_per_sample[0] / 8;
    //   const stride = planar_configuration === 2 ? 1 : bits_per_sample.length;

    //   for (let i = 0; i < height; ++i) {
    //     // Last strip will be truncated if height % stripHeight != 0
    //     if (i * stride * width * bytes_per_sample >= block.byteLength) {
    //       break;
    //     }
    //     let row;
    //     if (predictor === 2) {
    //       // horizontal prediction
    //       switch (bits_per_sample[0]) {
    //         case 8:
    //           row = new Uint8Array(
    //             block,
    //             i * stride * width * bytes_per_sample,
    //             stride * width * bytes_per_sample,
    //           );
    //           break;
    //         case 16:
    //           row = new Uint16Array(
    //             block,
    //             i * stride * width * bytes_per_sample,
    //             (stride * width * bytes_per_sample) / 2,
    //           );
    //           break;
    //         case 32:
    //           row = new Uint32Array(
    //             block,
    //             i * stride * width * bytes_per_sample,
    //             (stride * width * bytes_per_sample) / 4,
    //           );
    //           break;
    //         default:
    //           throw new Error(`Predictor 2 not allowed with ${bits_per_sample[0]} bits per sample.`);
    //       }
    //       decode_row_acc(row, stride);
    //     } else if (predictor === 3) {
    //       // horizontal floating point
    //       row = new Uint8Array(
    //         block,
    //         i * stride * width * bytes_per_sample,
    //         stride * width * bytes_per_sample,
    //       );
    //       decode_row_floating_point(row, stride, bytes_per_sample);
    //     }
    //   }
    //   return block;
}
