import { EXTRA_SAMPLES_VALUES, PHOTOMETRIC_INTERPRETATIONS } from './constants';

import type { Raster } from './image';

/**
 * Converts photometric interpretation to samples
 * @param pi - photometric interpretation
 * @param bitsPerSample - bits per sample
 * @param extraSamples - extra samples
 * @returns - sample output
 */
export function buildSamples(
  pi?: number,
  bitsPerSample: number[] = [],
  extraSamples: number = 0,
): number[] {
  let samples: number[];
  if (pi === PHOTOMETRIC_INTERPRETATIONS.RGB) {
    samples = [0, 1, 2, 3];
    // support alpha if it exists
    if (!(extraSamples === EXTRA_SAMPLES_VALUES.Unspecified)) {
      samples = [];
      for (let i = 0; i < bitsPerSample.length; i += 1) {
        samples.push(i);
      }
    }
  } else {
    switch (pi) {
      case PHOTOMETRIC_INTERPRETATIONS.WhiteIsZero:
      case PHOTOMETRIC_INTERPRETATIONS.BlackIsZero:
      case PHOTOMETRIC_INTERPRETATIONS.Palette:
        samples = [0];
        break;
      case PHOTOMETRIC_INTERPRETATIONS.CMYK:
        samples = [0, 1, 2, 3];
        break;
      case PHOTOMETRIC_INTERPRETATIONS.YCbCr:
      case PHOTOMETRIC_INTERPRETATIONS.CIELab:
      case PHOTOMETRIC_INTERPRETATIONS.ICCLab:
        samples = [0, 1, 2];
        break;
      default:
        throw new Error('Invalid or unsupported photometric interpretation.');
    }
  }

  return samples;
}

/**
 * Convert color space raster to RGB
 * TODO: ICCLAB, ITULAB
 * @param pi - photometric interpretation
 * @param rasterData - raster data
 * @param max - maximum value if needed
 * @param colorMap - color map if needed
 */
export function convertColorSpace(
  pi: number | undefined,
  rasterData: Raster,
  max: number,
  colorMap: number[] = [],
): void {
  if (pi === PHOTOMETRIC_INTERPRETATIONS.RGB) {
    return;
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.WhiteIsZero) {
    fromWhiteIsZero(rasterData, max);
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.BlackIsZero) {
    fromBlackIsZero(rasterData, max);
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.Palette) {
    fromPalette(rasterData, colorMap);
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.CMYK) {
    fromCMYK(rasterData);
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.YCbCr) {
    fromYCbCr(rasterData);
  } else if (pi === PHOTOMETRIC_INTERPRETATIONS.CIELab) {
    fromCIELab(rasterData);
  } else {
    throw new Error(`Unsupported photometric interpretation ${pi}.`);
  }
}

/**
 * Converts raster with white is zero and max is one to RGB
 * @param raster - raster
 * @param max - maximum value
 */
export function fromWhiteIsZero(raster: Raster, max: number): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8Array(width * height * 3);
  let value: number;
  for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
    value = 256 - (data[i] / max) * 256;
    rbgdata[j] = value;
    rbgdata[j + 1] = value;
    rbgdata[j + 2] = value;
  }
  raster.data = rbgdata;
}

/**
 * Converts raster with black is zero and max is one to RGB
 * @param raster - raster
 * @param max - maximum value
 */
export function fromBlackIsZero(raster: Raster, max: number): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8Array(width * height * 3);
  let value: number;
  for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
    value = (data[i] / max) * 256;
    rbgdata[j] = value;
    rbgdata[j + 1] = value;
    rbgdata[j + 2] = value;
  }
  raster.data = rbgdata;
}

/**
 * Converts raster with a color map to RGB
 * @param raster - raster
 * @param colorMap - color map
 */
export function fromPalette(raster: Raster, colorMap: number[]): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8Array(width * height * 3);
  const greenOffset = colorMap.length / 3;
  const blueOffset = (colorMap.length / 3) * 2;
  let mapIndex: number;
  for (let i = 0, j = 0; i < data.length; ++i, j += 3) {
    mapIndex = data[i];
    rbgdata[j] = (colorMap[mapIndex] / 65536) * 256;
    rbgdata[j + 1] = (colorMap[mapIndex + greenOffset] / 65536) * 256;
    rbgdata[j + 2] = (colorMap[mapIndex + blueOffset] / 65536) * 256;
  }
  raster.data = rbgdata;
}

/**
 * Converts CMYK to RGB
 * @param raster - CMYK raster
 */
export function fromCMYK(raster: Raster): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8Array(width * height * 3);
  let c: number, m: number, y: number, k: number;
  for (let i = 0, j = 0; i < data.length; i += 4, j += 3) {
    c = data[i];
    m = data[i + 1];
    y = data[i + 2];
    k = data[i + 3];

    rbgdata[j] = 255 * ((255 - c) / 256) * ((255 - k) / 256);
    rbgdata[j + 1] = 255 * ((255 - m) / 256) * ((255 - k) / 256);
    rbgdata[j + 2] = 255 * ((255 - y) / 256) * ((255 - k) / 256);
  }
  raster.data = rbgdata;
}

/**
 * Converts YCbCr to RGB
 * @param raster - YCbCr raster
 */
export function fromYCbCr(raster: Raster): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8ClampedArray(width * height * 3);
  let y: number, cb: number, cr: number;
  for (let i = 0, j = 0; i < data.length; i += 3, j += 3) {
    y = data[i];
    cb = data[i + 1];
    cr = data[i + 2];

    rbgdata[j] = y + 1.402 * (cr - 0x80);
    rbgdata[j + 1] = y - 0.34414 * (cb - 0x80) - 0.71414 * (cr - 0x80);
    rbgdata[j + 2] = y + 1.772 * (cb - 0x80);
  }
  raster.data = rbgdata;
}

const Xn = 0.95047;
const Yn = 1.0;
const Zn = 1.08883;

/**
 * Converts CIELab to RGB
 * https://github.com/antimatter15/rgb-lab/blob/master/color.js
 * @param raster - CIELab raster
 */
export function fromCIELab(raster: Raster): void {
  const { width, height, data } = raster;
  const rbgdata = new Uint8Array(width * height * 3);

  let L: number, a_: number, b_: number;
  let x: number, y: number, z: number;
  let r: number, g: number, b: number;
  for (let i = 0, j = 0; i < data.length; i += 3, j += 3) {
    L = data[i + 0];
    a_ = (data[i + 1] << 24) >> 24; // conversion from uint8 to int8
    b_ = (data[i + 2] << 24) >> 24; // same
    y = (L + 16) / 116;
    x = a_ / 500 + y;
    z = y - b_ / 200;

    x = Xn * (x * x * x > 0.008856 ? x * x * x : (x - 16 / 116) / 7.787);
    y = Yn * (y * y * y > 0.008856 ? y * y * y : (y - 16 / 116) / 7.787);
    z = Zn * (z * z * z > 0.008856 ? z * z * z : (z - 16 / 116) / 7.787);

    r = x * 3.2406 + y * -1.5372 + z * -0.4986;
    g = x * -0.9689 + y * 1.8758 + z * 0.0415;
    b = x * 0.0557 + y * -0.204 + z * 1.057;

    r = r > 0.0031308 ? 1.055 * r ** (1 / 2.4) - 0.055 : 12.92 * r;
    g = g > 0.0031308 ? 1.055 * g ** (1 / 2.4) - 0.055 : 12.92 * g;
    b = b > 0.0031308 ? 1.055 * b ** (1 / 2.4) - 0.055 : 12.92 * b;

    rbgdata[j] = Math.max(0, Math.min(1, r)) * 255;
    rbgdata[j + 1] = Math.max(0, Math.min(1, g)) * 255;
    rbgdata[j + 2] = Math.max(0, Math.min(1, b)) * 255;
  }
  raster.data = rbgdata;
}
