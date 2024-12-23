import { getTemplate4 } from './templates';
import { lookupTable40 } from './tables';

import type { Reader } from '../../..';
import type { Sections } from '../';

/** The output of `parseSection4` */
export type ProductDefinitionSection = ReturnType<typeof parseSection4>;

export type { ProductDefinition } from './templates';

/**
 *  Product Definition Section
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect4.shtml)
 * @param reader - The section to parse
 * @param sections - The other sections that have been parsed (1-3)
 * @returns - Parsed Product Definition Information
 */
export function parseSection4(reader: Reader, sections: Sections) {
  const productDefinitionTemplate = reader.getUint16(7);

  return {
    /** Number of GRIB section */
    sectionNumber: reader.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Product Definition Section' as const,
    /** Length of GRIB section */
    length: reader.getUint32(0),
    /** Number of coordinate values after template */
    coordinateValues: reader.getUint16(5),
    /** Product definition template number [Table 4.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml) */
    productDefinitionTemplate: {
      code: productDefinitionTemplate,
      definition: lookupTable40[productDefinitionTemplate],
    },
    /** Product definition */
    values: getTemplate4(productDefinitionTemplate, reader, sections),
  };
}
