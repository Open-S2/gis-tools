use super::{
    Raster,
    // constants::{ExtraSamplesValues, PhotometricInterpretations},
};
use alloc::vec;
use libm::{fmax, fmin, pow, round};

// /**
//  * Converts photometric interpretation to samples
//  * @param pi - photometric interpretation
//  * @param bitsPerSample - bits per sample
//  * @param extraSamples - extra samples
//  * @returns - sample output
//  */
// export function build_samples(
//   pi?: number,
//   bitsPerSample: number[] = [],
//   extraSamples: number = 0,
// ): number[] {
//   let samples: number[];
//   if (pi === PhotometricInterpretations::RGB) {
//     samples = [0, 1, 2, 3];
//     // support alpha if it exists
//     if (!(extraSamples === ExtraSamplesValues::Unspecified)) {
//       samples = [];
//       for (let i = 0; i < bitsPerSample.length; i += 1) {
//         samples.push(i);
//       }
//     }
//   } else {
//     switch (pi) {
//       case PhotometricInterpretations::WhiteIsZero:
//       case PhotometricInterpretations::BlackIsZero:
//       case PhotometricInterpretations::Palette:
//         samples = [0];
//         break;
//       case PhotometricInterpretations::CMYK:
//         samples = [0, 1, 2, 3];
//         break;
//       case PhotometricInterpretations::YCbCr:
//       case PhotometricInterpretations::CIELab:
//       case PhotometricInterpretations::ICCLab:
//         samples = [0, 1, 2];
//         break;
//       default:
//         throw new Error('Invalid or unsupported photometric interpretation.');
//     }
//   }

//   return samples;
// }

// /**
//  * Convert color space raster to RGB
//  * TODO: ICCLAB, ITULAB
//  * @param pi - photometric interpretation
//  * @param rasterData - raster data
//  * @param max - maximum value if needed
//  * @param colorMap - color map if needed
//  */
// export function convertColorSpace(
//   pi: number | undefined,
//   rasterData: Raster,
//   max: number,
//   colorMap: number[] = [],
// ): void {
//   if (pi === PhotometricInterpretations::RGB) {
//     return;
//   } else if (pi === PhotometricInterpretations::WhiteIsZero) {
//     fromWhiteIsZero(rasterData, max);
//   } else if (pi === PhotometricInterpretations::BlackIsZero) {
//     fromBlackIsZero(rasterData, max);
//   } else if (pi === PhotometricInterpretations::Palette) {
//     fromPalette(rasterData, colorMap);
//   } else if (pi === PhotometricInterpretations::CMYK) {
//     fromCMYK(rasterData);
//   } else if (pi === PhotometricInterpretations::YCbCr) {
//     fromYCbCr(rasterData);
//   } else if (pi === PhotometricInterpretations::CIELab) {
//     from_cei_lab(rasterData);
//   } else {
//     throw new Error(`Unsupported photometric interpretation ${pi}.`);
//   }
// }

// /**
//  * Converts raster with white is zero and max is one to RGB
//  * @param raster - raster
//  * @param max - maximum value
//  */
// export function fromWhiteIsZero(raster: Raster, max: number): void {
//   const { width, height, data } = raster;
//   const rbgdata = new Uint8Array(width * height * 3);
//   let value: number;
//   for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
//     value = 256 - (data[i] / max) * 256;
//     rbgdata[j] = value;
//     rbgdata[j + 1] = value;
//     rbgdata[j + 2] = value;
//   }
//   raster.data = rbgdata;
// }

// /**
//  * Converts raster with black is zero and max is one to RGB
//  * @param raster - raster
//  * @param max - maximum value
//  */
// export function fromBlackIsZero(raster: Raster, max: number): void {
//   const { width, height, data } = raster;
//   const rbgdata = new Uint8Array(width * height * 3);
//   let value: number;
//   for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
//     value = (data[i] / max) * 256;
//     rbgdata[j] = value;
//     rbgdata[j + 1] = value;
//     rbgdata[j + 2] = value;
//   }
//   raster.data = rbgdata;
// }

// /**
//  * Converts raster with a color map to RGB
//  * @param raster - raster
//  * @param colorMap - color map
//  */
// export function fromPalette(raster: Raster, colorMap: number[]): void {
//   const { width, height, data } = raster;
//   const rbgdata = new Uint8Array(width * height * 3);
//   const greenOffset = colorMap.length / 3;
//   const blueOffset = (colorMap.length / 3) * 2;
//   let mapIndex: number;
//   for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
//     mapIndex = data[i];
//     rbgdata[j] = (colorMap[mapIndex] / 65536) * 256;
//     rbgdata[j + 1] = (colorMap[mapIndex + greenOffset] / 65536) * 256;
//     rbgdata[j + 2] = (colorMap[mapIndex + blueOffset] / 65536) * 256;
//   }
//   raster.data = rbgdata;
// }

// /**
//  * Converts CMYK to RGB
//  * @param raster - CMYK raster
//  */
// export function fromCMYK(raster: Raster): void {
//   const { width, height, data } = raster;
//   const rbgdata = new Uint8Array(width * height * 3);
//   let c: number, m: number, y: number, k: number;
//   for (let i = 0, j = 0; i < data.length; i += 4, j += 3) {
//     c = data[i];
//     m = data[i + 1];
//     y = data[i + 2];
//     k = data[i + 3];

//     rbgdata[j] = 255 * ((255 - c) / 256) * ((255 - k) / 256);
//     rbgdata[j + 1] = 255 * ((255 - m) / 256) * ((255 - k) / 256);
//     rbgdata[j + 2] = 255 * ((255 - y) / 256) * ((255 - k) / 256);
//   }
//   raster.data = rbgdata;
// }

// /**
//  * Converts YCbCr to RGB
//  * @param raster - YCbCr raster
//  */
// export function fromYCbCr(raster: Raster): void {
//   const { width, height, data } = raster;
//   const rbgdata = new Uint8ClampedArray(width * height * 3);
//   let y: number, cb: number, cr: number;
//   for (let i = 0, j = 0; i < data.length; i += 3, j += 3) {
//     y = data[i];
//     cb = data[i + 1];
//     cr = data[i + 2];

//     rbgdata[j] = y + 1.402 * (cr - 0x80);
//     rbgdata[j + 1] = y - 0.34414 * (cb - 0x80) - 0.71414 * (cr - 0x80);
//     rbgdata[j + 2] = y + 1.772 * (cb - 0x80);
//   }
//   raster.data = rbgdata;
// }

const XN: f64 = 0.95047;
const YN: f64 = 1.0;
const ZN: f64 = 1.08883;

/**
 * Converts CIELab to RGB
 * https://github.com/antimatter15/rgb-lab/blob/master/color.js
 * @param raster - CIELab raster
 */
pub fn from_cei_lab(raster: &mut Raster) {
    let Raster { width, height, data, .. } = raster;
    let mut rbgdata = vec![0u8; *width * *height * 3];
    let data = data.buf();

    let mut l: f64;
    let mut a_: f64;
    let mut b_: f64;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        l = data[i] as f64;
        a_ = (data[i + 1] as i8) as f64; // conversion from uint8 to int8
        b_ = (data[i + 2] as i8) as f64; // same
        y = (l + 16.) / 116.;
        x = a_ / 500. + y;
        z = y - b_ / 200.;

        x = XN * if x * x * x > 0.008856 { x * x * x } else { (x - 16. / 116.) / 7.787 };
        y = YN * if y * y * y > 0.008856 { y * y * y } else { (y - 16. / 116.) / 7.787 };
        z = ZN * if z * z * z > 0.008856 { z * z * z } else { (z - 16. / 116.) / 7.787 };

        r = x * 3.2406 + y * -1.5372 + z * -0.4986;
        g = x * -0.9689 + y * 1.8758 + z * 0.0415;
        b = x * 0.0557 + y * -0.204 + z * 1.057;

        r = if r > 0.0031308 { 1.055 * pow(r, (1. / 2.4) - 0.055) } else { 12.92 * r };
        g = if g > 0.0031308 { 1.055 * pow(g, (1. / 2.4) - 0.055) } else { 12.92 * g };
        b = if b > 0.0031308 { 1.055 * pow(b, (1. / 2.4) - 0.055) } else { 12.92 * b };

        rbgdata[j] = round(fmax(0., fmin(1., r)) * 255.) as u8;
        rbgdata[j + 1] = round(fmax(0., fmin(1., g)) * 255.) as u8;
        rbgdata[j + 2] = round(fmax(0., fmin(1., b)) * 255.) as u8;
        i += 3;
        j += 3;
    }
    raster.data = rbgdata.into();
}
