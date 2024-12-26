import { grib2LookupTable00 } from './tables';

import type { Reader } from '../../..';

export * from './tables';

/** The output of `parseGrib2Section0` */
export type Grib2IndicatorSection = ReturnType<typeof parseGrib2Section0>;

/**
 * # SECTION 0 - INDICATOR SECTION
 *
 * ## Description
 * This section serves to identify the start of the record in a human readable form,
 * indicate the total length of the message, and indicate the Edition number of GRIB used
 * to construct or encode the message. For GRIB2, this section is always 16 octets long.
 *
 * ## Links
 * - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect0.shtml)
 * @param section - the 16 byte metadata section
 * @returns - a parsed explination of the file
 */
export function parseGrib2Section0(section: Reader) {
  const discipline = section.getUint8(6);
  return {
    /** Number of GRIB section */
    sectionNumber: 0,
    /** Name of Grib section */
    sectionName: 'Indicator Section' as const,
    /** Length of GRIB section (Always 16 for Section 0)*/
    length: 16,
    /** GRIB string encoded */
    gribEncoded: section.parseString(0, 4),
    /** Discipline [Table 0.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table0-0.shtml) */
    discipline: {
      code: discipline,
      description: grib2LookupTable00[discipline],
    },
    /** Edition number - 2 for GRIB2 */
    gribEdition: section.getUint8(7),
    /** Total length of GRIB message in octets (All sections) */
    gribLength: Number(section.getBigUint64(8)),
  };
}
