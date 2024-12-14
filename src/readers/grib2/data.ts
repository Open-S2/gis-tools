import type { BitMapSectionValues, DataRepresentationSectionValues } from './sections';

/**
 * Converts data Buffer according to data representation section
 * @param drs Data Representation Section
 * @param bms
 * @param data Data to be converted
 * @returns Converted data
 */
export function convertData(
  drs: DataRepresentationSectionValues,
  bms: BitMapSectionValues,
  data: Buffer,
): number[] {
  const { dataRepresentationTemplate } = drs.contents;

  switch (dataRepresentationTemplate) {
    case 0:
      return simpleUnpacking(drs, data);
    case 40:
      return jpeg2000Unpacking(drs, bms, data);

    default:
      throw new Error(`Template 7.${dataRepresentationTemplate} not defined`);
  }
}

/**
 * Data Template 7.0
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-0.shtml)
 * @param drs
 * @param data
 */
export function simpleUnpacking(drs: DataRepresentationSectionValues, data: Buffer): number[] {
  const { numberOfBits, decimalScaleFactor, referenceValue, binaryScaleFactor } =
    drs.contents.dataRepresentation;
  const DD = Math.pow(10, decimalScaleFactor);
  const EE = Math.pow(2, binaryScaleFactor);

  const bufferString = data.reduce((acc, value) => acc + value.toString(2).padStart(8, '0'), '');
  const chunks = bufferString.match(new RegExp(`.{${numberOfBits}}`, 'g'));

  if (chunks === null) throw new Error('Unable to split data into chunks');

  const values = chunks.map((chunk) => (referenceValue + parseInt(chunk, 2) * EE) / DD);

  return values;
}

/**
 * Data Template 7.0
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-40.shtml)
 * @param drs
 * @param bms
 * @param data
 */
export function jpeg2000Unpacking(
  drs: DataRepresentationSectionValues,
  bms: BitMapSectionValues,
  data: Buffer,
): number[] {
  const jpx = new JpxImage();
  jpx.parse(data);

  if (jpx.componentsCount !== 1)
    throw new Error('JPEG Decoder: Only single component is supported');
  if (jpx.tiles.length !== 1) throw new Error('JPEG Decoder: Only single tile is supported');
  if (jpx.tiles[0].height !== 1)
    throw new Error('JPEG Decoder: Only single row (1xN) is supported');

  const { dataRepresentation } = drs.contents;

  const D = dataRepresentation.decimalScaleFactor;
  const R = dataRepresentation.referenceValue;
  const E = dataRepresentation.binaryScaleFactor;

  const DD = Math.pow(10, D);
  const EE = Math.pow(2, E);

  const result: number[] = [];

  // A bit map applies to this product
  // [See more](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table6-0.shtml)
  if (bms.contents.bitMapIndicator === 0) {
    if (bms.contents.bitMap === null) {
      throw new Error('Bit map is not defined');
    }

    let k = 0;
    for (const byte of bms.contents.bitMap) {
      // Apply bit map to the data.
      // Length of data values is often smaller than the bit map itself. Bitmap is used to
      // indicate which data values are present, 1 bit meaning is present, 0 bit meaning is missing, -1 meaning undefined.
      // [Read more](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ)
      for (let i = 0; i < 8; i++) {
        if ((byte & (1 << i)) !== 0) {
          result.push((R + jpx.tiles[0].items[k++] * EE) / DD);
        } else {
          result.push(-1);
        }
      }
    }
  } else {
    // Do not use `.map` on Uint8Array, as it clamps the values to 0-255
    for (const byte of jpx.tiles[0].items) result.push((R + byte * EE) / DD);
  }

  return result;
}
