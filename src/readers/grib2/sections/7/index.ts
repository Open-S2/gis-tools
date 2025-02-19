import { BufferReader } from '../../..';
import { getGrib2Template7 } from './templates';

import type { Grib2Sections } from '..';
import type { Reader } from '../../..';

export * from './complexUnpacking';
export * from './spectral';
export * from './templates';

/** Data Section Return Type */
export type Grib2DataSection = ReturnType<typeof parseGrib2Section7>;

/**
 *  Data Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect7.shtml)
 * @param section - The raw section data to parse
 * @param sections - The other sections that have been parsed (1-6)
 * @returns - Parsed Data Information with a function to decode the data
 */
export function parseGrib2Section7(section: Reader, sections: Grib2Sections) {
  const data = new BufferReader(section.slice(5).buffer);

  return {
    /** Number of GRIB section */
    sectionNumber: section.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Data Section' as const,
    /** Length of GRIB section */
    length: section.getUint32(0),
    /** data that has yet to be decoded */
    rawData: data,
    /**
     * Data in a format described by data Template 7.X, where X is the data representation
     * template number given in octets 10-11 of [Section 5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect5.shtml).
     * @returns - the raw parsed data
     */
    getData(): number[] {
      return getGrib2Template7(data, sections);
    },
  };
}
