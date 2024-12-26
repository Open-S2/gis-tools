/**
 * # Table 7.0 - DATA TEMPLATE DEFINITIONS USED IN SECTION 7
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 64
 * - **Applicable Grid Templates**: 1000
 *
 * **Reserved Ranges**:
 * - `5-39`: Reserved
 * - `43-49`: Reserved
 * - `52`: Reserved
 * - `54-49151`: Reserved
 * - `49152-65534`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the data template definitions used in Section 7 of GRIB2 files,
 * specifying various data representation types and their corresponding templates.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table7-0.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 * - Red text depicts changes made since 05/11/2005.
 */
export const grib2LookupTable70: Record<number, string> = {
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-0.shtml) */
  0: 'Grid Point Data - Simple Packing (see Template 7.0)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-1.shtml) */
  1: 'Matrix Value at Grid Point - Simple Packing (see Template 7.1)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-2.shtml) */
  2: 'Grid Point Data - Complex Packing (see Template 7.2)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-3.shtml) */
  3: 'Grid Point Data - Complex Packing and Spatial Differencing (see Template 7.3)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-4.shtml) */
  4: 'Grid Point Data - IEEE Floating Point Data (see Template 7.4)',
  // 5-39: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-40.shtml) */
  40: 'Grid Point Data - JPEG2000 Compression (see Template 7.40)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-41.shtml) */
  41: 'Grid Point Data - Portable Network Graphics (PNG) format (see Template 7.41)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-42.shtml) */
  42: 'Grid Point and Spectral data - CCSDS recommended lossless compression (see Template 7.42)',
  // 43-49: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-50.shtml) */
  50: 'Spectral Data - Simple Packing (see Template 7.50)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-51.shtml) */
  51: 'Spectral Data - Complex Packing (see Template 7.51)',
  // 52: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-53.shtml) */
  53: 'Spectral Data for limited area models - Complex Packing (see Template 7.53)',
  // 54-49151: Reserved
  // 49152-65534: Reserved for Local Use
  255: 'Missing',
};
