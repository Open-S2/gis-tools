import { BufferReader } from '../../..';

import type { Reader } from '../../..';

/** The output of `parseSection2` */
export type LocalUseSection = ReturnType<typeof parseSection2>;

/**
 * # SECTION 2 - LOCAL USE SECTION
 *
 * ## Notes
 * 1. Center=7 (NCEP), subcenter=14(NWS Meteorological Development Laboratory (MDL))
 * used octet 6 to indicate which local use table to use. For MDL, octet 6=1 indicates use:
 * "MDL Template 2.1"
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect2.shtml)
 * @param section - The byte block to pull basic local information
 * @returns - a parsed explaination of local use.
 */
export function parseSection2(section: Reader) {
  return {
    /** Number of GRIB section */
    sectionNumber: section.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Local Use Section' as const,
    /** Length of GRIB section */
    length: section.getUint32(0),
    /** Section 2 Contents */
    contents: { data: new BufferReader(section.slice(5).buffer) },
  };
}
