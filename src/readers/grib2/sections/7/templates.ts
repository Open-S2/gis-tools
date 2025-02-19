import { JpxImage } from '../../../image';
import { complexUnpacking, spectralComplexUnpacking, spectralSimpleUnpacking } from '.';

import type { Grib2BitMapSection } from '../6';
import type { Grib2DataRepresentationSection } from '../5';
import type { Grib2Sections } from '..';
import type { Reader } from '../../..';

// TODO: spectrals are not implemented correctly yet.
// TODO: case 41: case 40010: PNG
// TODO: case 42: AEC https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/aecunpack.c

/**
 * Converts data Buffer according to data representation section
 * @param reader - The raw data to convert
 * @param sections - The sections of the GRIB2 message that have been parsed so far
 * @returns Converted data
 */
export function getGrib2Template7(reader: Reader, sections: Grib2Sections): number[] {
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
    case 50:
      return spectralSimpleUnpacking(reader, drs);
    case 51:
      return spectralComplexUnpacking(reader, drs);
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

  const bufferString = data.reduce((acc, value) => acc + value.toString(2).padStart(8, '0'), '');
  const chunks = bufferString
    .match(new RegExp(`.{${numberOfBits}}`, 'g'))
    ?.map((c) => parseInt(c, 2));
  if (chunks === undefined) throw new Error('Failed to parse data');

  const values = chunks.map((chunk) => (referenceValue + chunk * EE) / DD);

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
  if (jpx.tiles[0].height !== 1)
    throw new Error('JPEG Decoder: Only single row (1xN) is supported');

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

  // A bit map applies to this product
  if (bitMapIndicator.code === 0 && bitBuffer !== null) {
    const bitMapData = new Uint8Array(bitBuffer.buffer, bitBuffer.byteOffset, bitBuffer.byteLength);
    for (let i = 0; i < numberOfDataPoints; i++) {
      // Apply bit map to the data.
      // Length of data values is often smaller than the bit map itself. Bitmap is used to
      // indicate which data values are present, 1 bit meaning is present, 0 bit meaning is missing, -1 meaning undefined.
      // [Read more](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ)
      const byte = bitMapData[Math.floor(i / 8)];
      if ((byte & (1 << i % 8)) !== 0) {
        result.push((referenceValue + jpx.tiles[0].items[i] * EE) / DD);
      } else {
        result.push(Number.NEGATIVE_INFINITY);
      }
    }
  } else {
    // Do not use `.map` on Uint8Array, as it clamps the values to 0-255
    for (const byte of jpx.tiles[0].items) result.push((referenceValue + byte * EE) / DD);
  }

  return result;
}
