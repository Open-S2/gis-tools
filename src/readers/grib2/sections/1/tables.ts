/**
 * # Table 1.0 - GRIB Master Tables
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 10 (index 9)
 *
 * **Reserved Ranges**:
 * - `34-254`: Future Version
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the version numbers used in GRIB2 Master Tables,
 * providing context for interpreting the data's versioning information.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-0.shtml)
 *
 * ## Notes
 * - Revised 12/07/2023
 */
export const lookupTable10: Record<number, string> = {
  0: 'Experimental',
  1: 'Version Implemented on 7 November 2001',
  2: 'Version Implemented on 4 November 2003',
  3: 'Version Implemented on 2 November 2005',
  4: 'Version Implemented on 7 November 2007',
  5: 'Version Implemented on 4 November 2009',
  6: 'Version Implemented on 15 September 2010',
  7: 'Version Implemented on 4 May 2011',
  8: 'Version Implemented on 8 November 2011',
  9: 'Version Implemented on 2 May 2012',
  10: 'Version Implemented on 7 November 2012',
  11: 'Version Implemented on 8 May 2013',
  12: 'Version Implemented on 14 November 2013',
  13: 'Version Implemented on 7 May 2014',
  14: 'Version Implemented on 5 November 2014',
  15: 'Version Implemented on 6 May 2015',
  16: 'Version Implemented on 11 November 2015',
  17: 'Version Implemented on 4 May 2016',
  18: 'Version Implemented on 2 November 2016',
  19: 'Version Implemented on 3 May 2017',
  20: 'Version Implemented on 8 November 2017',
  21: 'Version Implemented on 2 May 2018',
  22: 'Version Implemented on 7 November 2018',
  23: 'Version Implemented on 15 May 2019',
  24: 'Version Implemented on 06 November 2019',
  25: 'Version Implemented on 06 May 2020',
  26: 'Version Implemented on 16 November 2020',
  27: 'Version Implemented on 16 June 2021',
  28: 'Version Implemented on 15 November 2021',
  29: 'Version Implemented on 15 May 2022',
  30: 'Version Implemented on 15 November 2022',
  31: 'Version Implemented on 15 June 2023',
  32: 'Version Implemented on 30 November 2023',
  33: 'Pre-operational to be implemented by next amendment',
  // 34-254: Future Version
  255: 'Missing',
};

/**
 * # Table 1.1 - GRIB Local Tables Version Number
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 11 (index 10)
 *
 * **Used Ranges**:
 * - `1-254`: Number of local table versions used
 *
 * **Special Values**:
 * - `0`: Local tables not used. Only table entries and templates from the current master table are valid.
 * - `255`: Missing
 *
 * ## Description
 * This table defines the version numbers used in GRIB2 Local Tables,
 * providing context for interpreting the data's versioning information.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-1.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 */
export const lookupTable11: Record<number, string> = {
  0: 'Local tables not used. Only table entries and templates from the current master table are valid.',
  // 1-254: Number of local table versions used
  255: 'Missing',
};

/**
 * # Table 1.2 - Significance of Reference Time
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 12 (index 11)
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the significance of the reference time in GRIB2 files,
 * providing context for interpreting the data's temporal meaning.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-2.shtml)
 *
 * ## Notes
 * - Revised 06/16/2022
 */
export const lookupTable12: Record<number, string> = {
  0: 'Analysis',
  1: 'Start of Forecast',
  2: 'Verifying Time of Forecast',
  3: 'Observation Time',
  4: 'Local Time',
  5: 'Simulation start',
  255: 'Missing',
};

/**
 * # Table 1.3 - Production Status of Data
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 20 (index 19)
 *
 * **Reserved Ranges**:
 * - `14-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the production status of data in GRIB2 files,
 * providing context for interpreting the data's operational and research status.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-3.shtml)
 *
 * ## Notes
 * - Revised 07/12/2024
 */
export const lookupTable13: Record<number, string> = {
  0: 'Operational Products',
  1: 'Operational Test Products',
  2: 'Research Products',
  3: 'Re-Analysis Products',
  4: 'THORPEX Interactive Grand Global Ensemble (TIGGE)',
  5: 'THORPEX Interactive Grand Global Ensemble (TIGGE) test',
  6: 'S2S Operational Products',
  7: 'S2S Test Products',
  8: 'Uncertainties in ensembles of regional reanalysis project (UERRA)',
  9: 'Uncertainties in ensembles of regional reanalysis project (UERRA) Test',
  10: 'Copernicus Regional Reanalysis',
  11: 'Copernicus Regional Reanalysis Test',
  12: 'Destination Earth',
  13: 'Destination Earth test',
  // 14-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 1.4 - TYPE OF DATA
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 21 (index 20)
 *
 * **Reserved Ranges**:
 * - `9-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Values**:
 * - `192`: Experimental Products
 * - `255`: Missing
 *
 * ## Description
 * This table defines the types of data in GRIB2 files,
 * providing context for interpreting the data's nature, whether operational, research, or experimental.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-4.shtml)
 *
 * ## Notes
 * - Revised 08/23/2023
 */
export const lookupTable14: Record<number, string> = {
  0: 'Analysis Products',
  1: 'Forecast Products',
  2: 'Analysis and Forecast Products',
  3: 'Control Forecast Products',
  4: 'Perturbed Forecast Products',
  5: 'Control and Perturbed Forecast Products',
  6: 'Processed Satellite Observations',
  7: 'Processed Radar Observations',
  8: 'Event Probability',
  // 9-191: Reserved
  192: 'Experimental Products',
  // 193-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 1.5 - IDENTIFICATION TEMPLATE NUMBER
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 21 (index 20)
 *
 * **Reserved Ranges**:
 * - `3-32767`: Reserved
 * - `32768-65534`: Reserved for Local Use
 *
 * **Special Value**:
 * - `65535`: Missing
 *
 * ## Description
 * This table defines the identification template numbers in GRIB2 files,
 * providing context for interpreting the data's template classifications.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-5.shtml)
 *
 * ## Notes
 * - Created 07/01/2014
 */
export const lookupTable15: Record<number, string> = {
  0: 'Calendar Definition',
  1: 'Paleontological Offset',
  2: 'Calendar Definition and Paleontological Offset',
  // 3-32767: Reserved
  // 32768-65534: Reserved for Local Use
  65535: 'Missing',
};

/**
 * # Table 1.6 - TYPE OF CALENDAR
 *
 * **Details**:
 * - **Section**: 1
 * - **Octet**: 21 (index 20)
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the types of calendars in GRIB2 files,
 * providing context for interpreting the data's calendar classifications.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-6.shtml)
 *
 * ## Notes
 * - (1). Essentially a non-leap year
 * - (2). Extends the Gregorian calendar indefinitely in the past
 */
export const lookupTable16: Record<number, string> = {
  0: 'Gregorian',
  1: '360-day',
  2: '365-day (see Note 1)',
  3: 'Proleptic Gregorian (see Note 2)',
  // 4-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};
