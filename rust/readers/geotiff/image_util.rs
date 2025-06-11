use alloc::vec::Vec;
use core::iter::Sum;
use half::f16;
use std::ops::{Add, Sub};

// /// Raster data element
// pub trait RasterElement: Copy + Clone + Default + Add + Sub + 'static {}

// impl RasterElement for u8 {}
// impl RasterElement for u16 {}
// impl RasterElement for u32 {}
// impl RasterElement for u64 {}
// impl RasterElement for i8 {}
// impl RasterElement for i16 {}
// impl RasterElement for i32 {}
// impl RasterElement for i64 {}
// impl RasterElement for f16 {}
// impl RasterElement for f32 {}
// impl RasterElement for f64 {}

// /// Raster data
// pub struct Raster<T: RasterElement> {
//     /// width of the raster
//     pub width: usize,
//     /// height of the raster
//     pub height: usize,
//     /// raster data
//     pub data: Vec<T>,
//     /// true if the data has an alpha channel
//     pub alpha: bool,
//     /// minimum value
//     pub min: f64,
//     /// maximum value
//     pub max: f64,
// }

// /**
//  * Convert the data format and bits per sample to an array type
//  * @param format - the data format
//  * @param bits_per_sample - the bits per sample
//  * @returns the array type constructor
//  */
// function array_type(format: number, bits_per_sample: number): ArrayTypesConstructors {
//   switch (format) {
//     case 1: // unsigned integer data
//       if (bits_per_sample <= 8) {
//         return Uint8Array;
//       } else if (bits_per_sample <= 16) {
//         return Uint16Array;
//       } else if (bits_per_sample <= 32) {
//         return Uint32Array;
//       }
//       break;
//     case 2: // twos complement signed integer data
//       if (bits_per_sample == 8) {
//         return Int8Array;
//       } else if (bits_per_sample == 16) {
//         return Int16Array;
//       } else if (bits_per_sample == 32) {
//         return Int32Array;
//       }
//       break;
//     case 3: // floating point data
//       switch (bits_per_sample) {
//         case 16:
//         case 32:
//           return Float32Array;
//         case 64:
//           return Float64Array;
//         default:
//           break;
//       }
//       break;
//     default:
//       break;
//   }
//   panic!("Unsupported data format/bits_per_sample");
// }

// /**
//  * Convert the data format and bits per sample to an array type
//  * @param raster - the data
//  * @param format - the data format
//  * @param bits_per_sample - the bits per sample
//  * @returns - the array
//  */
// pub fn to_array_type(raster: number[], format: number, bits_per_sample: number): ArrayTypes {
//   const constructor = array_type(format, bits_per_sample);
//   return new constructor(raster);
// }

// /**
//  * Create an array of the right type
//  * @param format - the data format
//  * @param bits_per_sample - the bits per sample
//  * @param size - the size
//  * @returns - the array
//  */
// pub fn arrayForType(format: number, bits_per_sample: number, size: number): ArrayTypes {
//   const constructor = array_type(format, bits_per_sample);
//   return new constructor(size);
// }

/**
 * @param array - An array of numbers
 * @param start - Start index
 * @param end - End index
 * @returns The sum
 */
pub fn sample_sum<T>(array: &[T], start: usize, end: usize) -> T
where
    T: Copy + Sum<T>,
{
    array[start..end].iter().copied().sum()
}

/**
 * Check if the data needs normalization
 * @param format - the data format
 * @param bits_per_sample - the bits per sample
 * @returns - true if the data needs normalization
 */
pub fn needs_normalization(format: usize, bits_per_sample: usize) -> bool {
    if (format == 1 || format == 2) && bits_per_sample <= 32 && bits_per_sample % 8 == 0 {
        false
    } else {
        !(format == 3 && (bits_per_sample == 16 || bits_per_sample == 32 || bits_per_sample == 64))
    }
}

// /**
//  * Normalize the array
//  * @param inBuffer - the input buffer
//  * @param format - the data format
//  * @param planarConfiguration - the planar configuration
//  * @param samplesPerPixel - the number of samples per pixel
//  * @param bits_per_sample - the bits per sample
//  * @param tileWidth - the tile width
//  * @param tileHeight - the tile height
//  * @returns - the normalized array
//  */
// pub fn normalize_array(
//   inBuffer: ArrayBufferLike,
//   format: number,
//   planarConfiguration: number,
//   samplesPerPixel: number,
//   bits_per_sample: number,
//   tileWidth: number,
//   tileHeight: number,
// ): ArrayBufferLike {
//   // const inByteArray = new Uint8Array(inBuffer);
//   const view = new DataView(inBuffer);
//   const outSize =
//     planarConfiguration == 2 ? tileHeight * tileWidth : tileHeight * tileWidth * samplesPerPixel;
//   const samplesToTransfer = planarConfiguration == 2 ? 1 : samplesPerPixel;
//   const outArray = arrayForType(format, bits_per_sample, outSize);
//   // let pixel = 0;

//   const bitMask = parseInt('1'.repeat(bits_per_sample), 2);

//   if (format == 1) {
//     // unsigned integer
//     // translation of https://github.com/OSGeo/gdal/blob/master/gdal/frmts/gtiff/geotiff.cpp#L7337
//     let pixelBitSkip;
//     // let sampleBitOffset = 0;
//     if (planarConfiguration == 1) {
//       pixelBitSkip = samplesPerPixel * bits_per_sample;
//       // sampleBitOffset = (samplesPerPixel - 1) * bits_per_sample;
//     } else {
//       pixelBitSkip = bits_per_sample;
//     }

//     // Bits per line rounds up to next byte boundary.
//     let bitsPerLine = tileWidth * pixelBitSkip;
//     if ((bitsPerLine & 7) !== 0) {
//       bitsPerLine = (bitsPerLine + 7) & ~7;
//     }

//     for (let y = 0; y < tileHeight; ++y) {
//       const lineBitOffset = y * bitsPerLine;
//       for (let x = 0; x < tileWidth; ++x) {
//         const pixelBitOffset = lineBitOffset + x * samplesToTransfer * bits_per_sample;
//         for (let i = 0; i < samplesToTransfer; ++i) {
//           const bitOffset = pixelBitOffset + i * bits_per_sample;
//           const outIndex = (y * tileWidth + x) * samplesToTransfer + i;

//           const byteOffset = Math.floor(bitOffset / 8);
//           const innerBitOffset = bitOffset % 8;
//           if (innerBitOffset + bits_per_sample <= 8) {
//             outArray[outIndex] =
//               (view.getUint8(byteOffset) >> (8 - bits_per_sample - innerBitOffset)) & bitMask;
//           } else if (innerBitOffset + bits_per_sample <= 16) {
//             outArray[outIndex] =
//               (view.getUint16(byteOffset) >> (16 - bits_per_sample - innerBitOffset)) & bitMask;
//           } else if (innerBitOffset + bits_per_sample <= 24) {
//             const raw = (view.getUint16(byteOffset) << 8) | view.getUint8(byteOffset + 2);
//             outArray[outIndex] = (raw >> (24 - bits_per_sample - innerBitOffset)) & bitMask;
//           } else {
//             outArray[outIndex] =
//               (view.getUint32(byteOffset) >> (32 - bits_per_sample - innerBitOffset)) & bitMask;
//           }

//           // let outWord = 0;
//           // for (let bit = 0; bit < bits_per_sample; ++bit) {
//           //   if (inByteArray[bitOffset >> 3]
//           //     & (0x80 >> (bitOffset & 7))) {
//           //     outWord |= (1 << (bits_per_sample - 1 - bit));
//           //   }
//           //   ++bitOffset;
//           // }

//           // outArray[outIndex] = outWord;
//           // outArray[pixel] = outWord;
//           // pixel += 1;
//         }
//         // bitOffset = bitOffset + pixelBitSkip - bits_per_sample;
//       }
//     }
//   } else if (format == 3) {
//     // floating point
//     // Float16 is handled elsewhere
//     // normalize 16/24 bit floats to 32 bit floats in the array
//     // console.time();
//     // if (bits_per_sample == 16) {
//     //   for (let byte = 0, outIndex = 0; byte < inBuffer.byteLength; byte += 2, ++outIndex) {
//     //     outArray[outIndex] = getFloat16(view, byte);
//     //   }
//     // }
//     // console.timeEnd()
//   }

//   return outArray.buffer;
// }
