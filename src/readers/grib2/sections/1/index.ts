import { lookupTable0 } from '../0';
import { lookupTableC } from '../other/tables';
import { lookupTable10, lookupTable11, lookupTable12, lookupTable13, lookupTable14 } from '.';

import type { Reader } from '../../..';

export * from './tables';

/** The output of `parseSection1` */
export type IdentificationSection = ReturnType<typeof parseSection1>;

/**
 * # Identification Section
 *
 * ## Notes
 * - 1. Local tables define those parts of the master table which are reserved for local use except for the case described below.  In any case, the use of local tables in the messages are intended for non-local or international exchange is strongly discouraged.
 * - 2.  If octet 10 is set to 255 then only local tables are in use.  In this case, the local table version number (octet 11) must not be zero nor missing.  Local tables may include entries from the entire range of the tables.
 * - 3.  If octet 11 is zero, octet 10 must contain a valid master table version number and only those parts of the tables not reserved for local use may be used.
 * - 4.  If octets 8-9 is zero, Not a sub-center, the originating/generating center is the center defined by octets 6-7.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect1.shtml)
 * @param section - The byte block to pull ideintification information
 * @returns - The parsed identification section
 */
export function parseSection1(section: Reader) {
  const center = section.getUint16(5);
  const subcenter = section.getUint16(7);
  const gribMasterTablesVersion = section.getUint8(9); // should be 2
  const gribLocalTablesVersion = section.getUint8(10);
  const significanceOfRT = section.getUint8(11);
  const year = section.getUint16(12);
  const month = section.getUint8(14);
  const day = section.getUint8(15);
  const hours = section.getUint8(16);
  const minutes = section.getUint8(17);
  const seconds = section.getUint8(18);
  const productionStatus = section.getUint8(19);
  const typeOfProcessedData = section.getUint8(20);

  const refTime = new Date(year, month - 1, day);
  refTime.setUTCHours(hours);
  refTime.setUTCMinutes(minutes);
  refTime.setUTCSeconds(seconds);

  if (gribMasterTablesVersion !== 2) console.warn('Master tables version is not 2');

  return {
    /** Number of GRIB section */
    sectionNumber: section.getUint8(4),
    /** Name of Grib section */
    sectionName: 'Identification Section' as const,
    /** Length of GRIB section */
    length: section.getUint32(0),
    /** Identification of originating/generating center [Table 0](https://www.nco.ncep.noaa.gov/pmb/docs/on388/table0.html) */
    center: {
      code: center,
      description: lookupTable0[center],
    },
    /** Identification of originating/generating subcenter [Table C](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablec.html) */
    subcenter: {
      code: subcenter,
      description: lookupTableC[subcenter],
    },
    /** GRIB master tables version number [Table 1.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-0.shtml) */
    gribMasterTablesVersion: {
      code: gribMasterTablesVersion,
      description: lookupTable10[gribMasterTablesVersion],
    },
    /** Version number of GRIB local tables used to augment Master Tables [Table 1.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-1.shtml) */
    gribLocalTablesVersion: {
      code: gribLocalTablesVersion,
      description: lookupTable11[gribLocalTablesVersion],
    },
    /** Significance of reference time [Table 1.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-2.shtml) */
    significanceOfRT: {
      code: significanceOfRT,
      description: lookupTable12[significanceOfRT],
    },
    /** Reference Time */
    refTime,
    /** Production Status of Processed data in the GRIB message [Table 1.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-3.shtml) */
    productionStatus: {
      code: productionStatus,
      description: lookupTable13[productionStatus],
    },
    /** Type of processed data in this GRIB message [Table 1.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-4.shtml) */
    typeOfProcessedData: {
      code: typeOfProcessedData,
      description: lookupTable14[typeOfProcessedData],
    },
  };
}
