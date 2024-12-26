import type { Reader } from '../../..';

/** End Section Return Type */
export type Grib2EndSection = ReturnType<typeof parseGrib2Section8>;

/**
 * # SECTION 8 - END SECTION
 *
 * ## Links
 * - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect8.shtml)
 * @param section - byte block for section 8
 * @returns - parsed end section
 */
export function parseGrib2Section8(section: Reader) {
  return {
    /** Name of Grib section */
    sectionName: 'End Section',
    /** "7777" - Coded according to the International Alphabet Number 5 */
    endEncoded: section.parseString(0, 4),
  };
}
