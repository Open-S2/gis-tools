import { BufferReader } from '../../..';
import { grib2LookupTable60 } from './tables';

import type { Reader } from '../../..';

/** The output of `parseGrib2Section6` */
export type Grib2BitMapSection = ReturnType<typeof parseGrib2Section6>;

/**
 * # Bit-Map Section
 *
 * ## Links
 * - [Consult with this page to understand their purpose.](https://confluence.ecmwf.int/display/UDOC/What+is+the+GRIB+bitmap+-+ecCodes+GRIB+FAQ).
 * - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect6.shtml).
 * @param section - The byte block to understan how to parse bit-map data
 * @returns - Parsed bit-map section
 */
export function parseGrib2Section6(section: Reader) {
  const bitMapIndicator = section.getUint8(5);

  if (![0, 255].includes(bitMapIndicator)) {
    throw new Error('BitMap Indicator not supported: ' + String(bitMapIndicator));
  }

  return {
    /** Number of GRIB section */
    sectionNumber: section.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Bit-Map Section' as const,
    /** Length of GRIB section */
    length: section.getUint32(0),
    /** Bit-map indicator (See [Table 6.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table6-0.shtml)) */
    bitMapIndicator: {
      code: bitMapIndicator,
      description: grib2LookupTable60[bitMapIndicator],
    },
    /** Bit-map */
    bitMap: bitMapIndicator === 0 ? new BufferReader(section.slice(6).buffer) : null,
  };
}
