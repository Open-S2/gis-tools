import { getGrib2Template3 } from './templates';
import { grib2LookupTable31 } from './tables';

import type { Reader } from '../../..';

export * from './tables';

/** The output of `parseGrib2Section3` */
export type GridDefinitionSection = ReturnType<typeof parseGrib2Section3>;

/**
 * # SECTION 3 - GRID DEFINITION SECTION
 *
 * ## Links
 * - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect3.shtml)
 * @param section - byte block for section 3
 * @returns - parsed grid definition
 */
export function parseGrib2Section3(section: Reader) {
  const gridDefinitionTemplate = section.getUint16(12);

  return {
    /** Number of GRIB section */
    sectionNumber: section.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Grid Definition Section' as const,
    /** Length of GRIB section */
    length: section.getUint32(0),
    /** Source of grid definition */
    definitionSource: section.getUint8(5),
    /** Number of data points */
    numberOfPoints: section.getUint32(6),
    /** Number of octets for optional list of numbers defining number of points */
    numberOfOctets: section.getUint8(10),
    /** Interpetation of list of numbers defining number of points [Table 3.11](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-11.shtml) */
    interpretation: section.getUint8(11),
    /** Grid definition template number [Table 3.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml) */
    gridDefinitionTemplate: {
      code: gridDefinitionTemplate,
      description: grib2LookupTable31[gridDefinitionTemplate],
    },
    /** Grid definition values */
    values: getGrib2Template3(gridDefinitionTemplate, section),
  };
}
