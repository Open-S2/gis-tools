import {
  JpxImage,
  complexUnpacking,
  imageDecoder,
  spectralComplexUnpacking,
  spectralSimpleUnpacking,
} from '../../../index.js';

import type { Grib2BitMapSection } from '../6/index.js';
import type { Grib2DataRepresentationSection } from '../5/index.js';
import type { Grib2Sections } from '../index.js';
import type { Reader } from '../../../index.js';

// TODO: spectrals MAY not be implemented correctly yet.
// TODO: case 41: case 40010: PNG
// TODO: case 42: AEC https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/aecunpack.c

/**
 * Converts data Buffer according to data representation section
 * @param reader - The raw data to convert
 * @param sections - The sections of the GRIB2 message that have been parsed so far
 * @returns Converted data
 */
export async function getGrib2Template7(
  reader: Reader,
  sections: Grib2Sections,
): Promise<number[]> {
  const drs = sections.dataRepresentation;
  if (drs === undefined) throw new Error('Data Representation Section is not defined');
  const { dataRepresentationTemplate } = drs;

  switch (dataRepresentationTemplate.code) {
    case 0:
      return simpleUnpacking(reader, drs);
    case 2:
    case 3:
      return complexUnpacking(reader, sections);
    case 40:
    case 40000: {
      const bms = sections.bitMap;
      if (bms === undefined) throw new Error('Bit Map Section is not defined');
      return jpeg2000Unpacking(reader, drs, bms);
    }
    case 41:
    case 40010:
      return await pngUnpacking(reader, drs);
    case 50: {
      // 1. Unpack the "simple" part (indices 1 to ndpts-1)
      const unpackedData = spectralSimpleUnpacking(reader, drs);
      // 2. The first value (index 0) is often stored as a raw IEEE float in the DRS
      // In DRS 5.0/5.50, the referenceValue is that IEEE float.
      unpackedData.unshift(drs.dataRepresentation.referenceValue);

      return unpackedData;
    }
    case 51:
      return spectralComplexUnpacking(reader, sections);
    default:
      throw new Error(`Template 7.${dataRepresentationTemplate} not defined`);
  }
}

/**
 * # Data Template 7.0 - Grid point data - simple packing
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-0.shtml)
 * @param reader - The raw data to convert
 * @param drs - The data representation section
 * @returns - The converted data
 */
export function simpleUnpacking(reader: Reader, drs: Grib2DataRepresentationSection): number[] {
  const { numberOfBits, decimalScaleFactor, referenceValue, binaryScaleFactor } =
    drs.dataRepresentation;

  const DD = Math.pow(10, decimalScaleFactor);
  const EE = Math.pow(2, binaryScaleFactor);
  const data = new Uint8Array(reader.slice().buffer);

  const totalBits = data.length * 8;
  const count = Math.floor(totalBits / numberOfBits);
  const values: number[] = [];

  let bitPos = 0;

  for (let i = 0; i < count; i++) {
    let acc = 0;

    for (let b = 0; b < numberOfBits; b++) {
      const byteIndex = Math.floor(bitPos / 8);
      const bitOffset = 7 - (bitPos % 8);
      const bit = (data[byteIndex] >> bitOffset) & 1;

      acc = (acc << 1) | bit;
      bitPos++;
    }

    values.push((referenceValue + acc * EE) / DD);
  }

  return values;
}

/**
 * # Data Template 7.40 - Grid point data - JPEG 2000 code stream format
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-40.shtml)
 * @param reader - The raw data to convert
 * @param drs - The data representation section
 * @param bms - The bit map section
 * @returns - The converted data
 */
export function jpeg2000Unpacking(
  reader: Reader,
  drs: Grib2DataRepresentationSection,
  bms: Grib2BitMapSection,
): number[] {
  const jpx = new JpxImage(reader);
  if (jpx.componentsCount !== 1)
    throw new Error('JPEG Decoder: Only single component is supported');
  if (jpx.tiles.length !== 1) throw new Error('JPEG Decoder: Only single tile is supported');
  const pixels = jpx.tiles[0].items;

  const { bitMap: bitBuffer, bitMapIndicator } = bms;
  const { numberOfDataPoints, dataRepresentation } = drs;
  const { decimalScaleFactor, referenceValue, binaryScaleFactor, numberOfBits } =
    dataRepresentation;

  const DD = Math.pow(10, decimalScaleFactor);
  const EE = Math.pow(2, binaryScaleFactor);

  const result: number[] = [];

  if (numberOfBits === 0) {
    for (let i = 0; i < numberOfDataPoints; i++) result.push(referenceValue);
    return result;
  }

  let dataIndex = 0;
  // A bit map applies to this product
  if (bitMapIndicator.code === 0 && bitBuffer !== null) {
    const bitMapData = new Uint8Array(bitBuffer.buffer, bitBuffer.byteOffset, bitBuffer.byteLength);

    for (let i = 0; i < numberOfDataPoints; i++) {
      // GRIB2 Bitmaps are MSB (Most Significant Bit) first
      const byte = bitMapData[i >> 3];
      const bitIsSet = (byte & (0x80 >> (i % 8))) !== 0;

      if (bitIsSet) {
        // Pull from the next available JPEG pixel
        const rawValue = pixels[dataIndex++];
        result.push((referenceValue + rawValue * EE) / DD);
      } else {
        // Set to NaN so the value is ignored upstream
        result.push(NaN);
      }
    }
  } else {
    // No bitmap: 1:1 mapping
    for (const rawValue of pixels) {
      result.push((referenceValue + rawValue * EE) / DD);
    }
  }

  while (result.length < numberOfDataPoints) result.push(NaN);

  return result;
}

export async function pngUnpacking(
  reader: Reader,
  drs: Grib2DataRepresentationSection,
): Promise<number[]> {
  const { numberOfBits, decimalScaleFactor, referenceValue, binaryScaleFactor } =
    drs.dataRepresentation;

  const DD = Math.pow(10, decimalScaleFactor);
  const EE = Math.pow(2, binaryScaleFactor);
  const imageData = new Uint8Array(reader.slice().buffer);
  const decoded = (await imageDecoder(imageData)).data;
  const pixels = new Uint8Array(decoded.buffer);

  const totalBits = pixels.length * 8;
  const count = Math.floor(totalBits / numberOfBits);
  const values: number[] = [];

  let bitPos = 0;

  for (let i = 0; i < count; i++) {
    let acc = 0;

    for (let b = 0; b < numberOfBits; b++) {
      const byteIndex = Math.floor(bitPos / 8);
      const bitOffset = 7 - (bitPos % 8);
      const bit = (pixels[byteIndex] >> bitOffset) & 1;

      acc = (acc << 1) | bit;
      bitPos++;
    }

    values.push((referenceValue + acc * EE) / DD);
  }

  return values;
}
