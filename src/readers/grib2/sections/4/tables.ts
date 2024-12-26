/** Categories track a name with it's units and abbreviation */
export interface TableCategory {
  parameter: string;
  units: string;
  abbrev: string;
}

/**
 * # GRIB2 - CODE TABLE 4.0 - PRODUCT DEFINITION TEMPLATE NUMBER
 *
 * **Details**:
 * - **Section**: 4
 * - **Octets**: 8-9
 * - **Revised**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `16-19`: Reserved
 * - `21-29`: Reserved
 * - `36-39`: Reserved
 * - `50`: Reserved
 * - `52`: Reserved
 * - `64-66`: Reserved
 * - `69`: Reserved
 * - `74-75`: Reserved
 * - `128-253`: Reserved
 * - `255-999`: Reserved
 * - `1003-1099`: Reserved
 * - `1102-32767`: Reserved
 * - `32768-65534`: Reserved for Local Use
 *
 * **Special Value**:
 * - `65535`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
 *
 * ## Notes
 * - Red text depicts changes made since 08/23/2023.
 */
export const grib2LookupTable40: Record<number, string> = {
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml) */
  0: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1.shtml) */
  1: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-2.shtml) */
  2: 'Derived forecasts based on all ensemble members at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-3.shtml) */
  3: 'Derived forecasts based on a cluster of ensemble members over a rectangular area at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-4.shtml) */
  4: 'Derived forecasts based on a cluster of ensemble members over a circular area at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-5.shtml) */
  5: 'Probability forecasts at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-6.shtml) */
  6: 'Percentile forecasts at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-7.shtml) */
  7: 'Analysis or forecast error at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-8.shtml) */
  8: 'Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-9.shtml) */
  9: 'Probability forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-10.shtml) */
  10: 'Percentile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-11.shtml) */
  11: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-12.shtml) */
  12: 'Derived forecasts based on all ensemble members at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-13.shtml) */
  13: 'Derived forecasts based on a cluster of ensemble members over a rectangular area at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-14.shtml) */
  14: 'Derived forecasts based on a cluster of ensemble members over a circular area at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-15.shtml) */
  15: 'Average, accumulation, extreme values or other statistically-processed values over a spatial area at a horizontal level or in a horizontal layer at a point in time.',
  // 16-19: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-32.shtml) */
  32: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for simulate (synthetic) satellite data.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-33.shtml) */
  33: 'Individual Ensemble Forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for simulated (synthetic) satellite data.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-34.shtml) */
  34: 'Individual Ensemble Forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval for simulated (synthetic) satellite data.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-35.shtml) */
  35: 'Satellite product with or without associated quality values.',
  // 36-39: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-40.shtml) */
  40: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-41.shtml) */
  41: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-42.shtml) */
  42: 'Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-43.shtml) */
  43: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for atmospheric chemical constituents.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-44.shtml) */
  44: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-45.shtml) */
  45: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-46.shtml) */
  46: 'Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-47.shtml) */
  47: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-48.shtml) */
  48: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-49.shtml) */
  49: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol.',
  // 50: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-51.shtml) */
  51: 'Categorical forecast at a horizontal level or in a horizontal layer at a point in time.',
  // 52: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-53.shtml) */
  53: 'Partitioned parameters at a horizontal level or horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-54.shtml) */
  54: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for partitioned parameters.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-55.shtml) */
  55: 'Spatio-temporal changing tiles at a horizontal level or horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-56.shtml) */
  56: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for spatio-temporal changing tile parameters (DEPRECATED).',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-57.shtml) */
  57: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents based on a distribution function.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-58.shtml) */
  58: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time interval for atmospheric chemical constituents based on a distribution function.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-59.shtml) */
  59: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for spatio-temporal changing tile parameters (corrected version of template 4.56).',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-60.shtml) */
  60: 'Individual Ensemble Reforecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-61.shtml) */
  61: 'Individual Ensemble Reforecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-62.shtml) */
  62: 'Average, accumulation and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for spatio-temporal changing tiles at a horizontal level or horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-63.shtml) */
  63: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for spatio-temporal changing tiles.',
  // 64-66: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-67.shtml) */
  67: 'Average, accumulation and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents based on a distribution function.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-68.shtml) */
  68: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents based on a distribution function.',
  // 69: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-70.shtml) */
  70: 'Post-processing analysis or forecast at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-71.shtml) */
  71: 'Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-72.shtml) */
  72: 'Post-processing average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-73.shtml) */
  73: 'Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.',
  // 74-75: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-76.shtml) */
  76: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-77.shtml) */
  77: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-78.shtml) */
  78: 'Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-79.shtml) */
  79: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-80.shtml) */
  80: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-81.shtml) */
  81: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-82.shtml) */
  82: 'Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-83.shtml) */
  83: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-84.shtml) */
  84: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-85.shtml) */
  85: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-86.shtml) */
  86: 'Quantile forecasts at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-87.shtml) */
  87: 'Quantile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-88.shtml) */
  88: 'Analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-89.shtml) */
  89: 'Post-processed quantile forecasts at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-90.shtml) */
  90: 'Post-processed quantile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-91.shtml) */
  91: 'Categorical forecast at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-92.shtml) */
  92: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-93.shtml) */
  93: 'Post-processing analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-94.shtml) */
  94: 'Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-95.shtml) */
  95: 'Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-96.shtml) */
  96: 'Average, accumulation, extreme values or other statistically processed values of an individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-97.shtml) */
  97: 'Average, accumulation, extreme values or other statistically processed values of post-processing analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-98.shtml) */
  98: 'Average, accumulation, extreme values or other statistically processed values of a post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-99.shtml) */
  99: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with explicit list of frequencies and directions.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-100.shtml) */
  100: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with explicit list of frequencies and directions.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-101.shtml) */
  101: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with frequencies and directions defined by formulae.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-102.shtml) */
  102: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with frequencies and directions defined by formulae.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-103.shtml) */
  103: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for waves selected by period range.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-104.shtml) */
  104: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for waves selected by period range.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-105.shtml) */
  105: 'Anomalies, significance and other derived products from an analysis or forecast in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-106.shtml) */
  106: 'Anomalies, significance and other derived products from an individual ensemble forecast, control and perturbed in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-107.shtml) */
  107: 'Anomalies, significance and other derived products from derived forecasts based on all ensemble members in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-108.shtml) */
  108: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for generic optical products.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-109.shtml) */
  109: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for generic optical products.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-110.shtml) */
  110: 'Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for generic optical products.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-111.shtml) */
  111: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval for generic optical products.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-112.shtml) */
  112: 'Anomalies, significance and other derived products as probability forecasts in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-113.shtml) */
  113: 'Generalized tiles at a horizontal level or horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-114.shtml) */
  114: 'Average, accumulation, and/or extreme values or other statistically processed values on generalized tiles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-115.shtml) */
  115: 'Individual ensemble forecast, control and perturbed on generalized tiles at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-116.shtml) */
  116: 'Individual ensemble forecast, control and perturbed on generalized tiles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-117.shtml) */
  117: 'Individual large ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-118.shtml) */
  118: 'Individual large ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-119.shtml) */
  119: 'Probability forecasts from large ensembles at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-120.shtml) */
  120: 'Probability forecasts from large ensembles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-121.shtml) */
  121: 'Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics at a horizontal level or in a horizontal layer at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-122.shtml) */
  122: 'Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-123.shtml) */
  123: 'Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-124.shtml) */
  124: 'Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for radionuclides.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-125.shtml) */
  125: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for radionuclides.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-126.shtml) */
  126: 'Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for radionuclides.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-127.shtml) */
  127: 'Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for radionuclides.',
  // 128-253: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-254.shtml) */
  254: 'CCITT IA5 character string.',
  // 255-999: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1000.shtml) */
  1000: 'Cross-section of analysis and forecast at a point in time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1001.shtml) */
  1001: 'Cross-section of averaged or otherwise statistically processed analysis or forecast over a range of time.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1002.shtml) */
  1002: 'Cross-section of analysis and forecast, averaged or otherwise statistically-processed over latitude or longitude.',
  // 1003-1099: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1100.shtml) */
  1100: 'Hovmoller-type grid with no averaging or other statistical processing.',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1101.shtml) */
  1101: 'Hovmoller-type grid with averaging or other statistical processing.',
  // 1102-32767: Reserved
  // 32768-65534: Reserved for Local Use
  65535: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.1: PARAMETER CATEGORY BY PRODUCT DISCIPLINE
 *
 * **Created**: 12/07/2023
 * **Revised**: 12/07/2023 (Red text depicts changes made since 10/30/2023)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)
 *
 * ## Notes
 * - The disciplines are given in Section 0, Octet 7 of the GRIB2 message and are defined in Table 0.0.
 * - When a new category is to be added to Code table 4.1 and more than one discipline applies,
 *   the choice of discipline should be made based on the intended use of the product.
 */
export const grib2LookupTable41: Record<number, Record<number, string>> = {
  /** Product Discipline 0 - Meteorological Products */
  0: {
    0: 'Temperature (see Table 4.2-0-0)',
    1: 'Moisture (see Table 4.2-0-1)',
    2: 'Momentum (see Table 4.2-0-2)',
    3: 'Mass (see Table 4.2-0-3)',
    4: 'Short-wave radiation (see Table 4.2-0-4)',
    5: 'Long-wave radiation (see Table 4.2-0-5)',
    6: 'Cloud (see Table 4.2-0-6)',
    7: 'Thermodynamic Stability indices (see Table 4.2-0-7)',
    8: 'Kinematic Stability indices',
    9: 'Temperature Probabilities*',
    10: 'Moisture Probabilities*',
    11: 'Momentum Probabilities*',
    12: 'Mass Probabilities*',
    13: 'Aerosols (see Table 4.2-0-13)',
    14: 'Trace gases (e.g. Ozone, CO2) (see Table 4.2-0-14)',
    15: 'Radar (see Table 4.2-0-15)',
    16: 'Forecast Radar Imagery (see Table 4.2-0-16)',
    17: 'Electrodynamics (see Table 4.2-0-17)',
    18: 'Nuclear/radiology (see Table 4.2-0-18)',
    19: 'Physical atmospheric properties (see Table 4.2-0-19)',
    20: 'Atmospheric chemical Constituents (see Table 4.2-0-20)',
    21: 'Thermodynamic Properties (see Table 4.2-0-21)',
    22: 'Drought Indices (see Table 4.2-0-22)',
    // 23-189 Reserved
    190: 'CCITT IA5 string (see Table 4.2-0-190)',
    191: 'Miscellaneous (see Table 4.2-0-191)',
    // 192-254 Reserved for Local Use
    192: 'Covariance (see Table 4.2-0-192)',
    255: 'Missing',
  },
  /** Product Discipline 1 - Hydrological Products */
  1: {
    0: 'Hydrology basic products (see Table 4.2-1-0)',
    1: 'Hydrology probabilities (see Table 4.2-1-1)',
    2: 'Inland water and sediment properties (see Table 4.2-1-2)',
    // 3-191 Reserved
    // 192-254 Reserved for Local Use
    255: 'Missing',
  },
  /** Product Discipline 2 - Land Surface Products */
  2: {
    0: 'Vegetation/Biomass (see Table 4.2-2-0)',
    1: 'Agricultural/Aquacultural Special Products (see Table 4.2-2-1)',
    2: 'Transportation-related Products',
    3: 'Soil Products (see Table 4.2-2-3)',
    4: 'Fire Weather Products (see Table 4.2-2-4)',
    5: 'Land Surface Products (see Table 4.2-2-5)',
    6: 'Urban areas (see Table 4.2-2-6)',
    // 7-191 Reserved
    // 192-254 Reserved for Local Use
    255: 'Missing',
  },
  /** Product Discipline 3 - Satellite Remote Sensing Products */
  3: {
    0: 'Image format products (See note 1) (see Table 4.2-3-0)',
    1: 'Quantitative products (See note 2) (see Table 4.2-3-1)',
    2: 'Cloud Properties (see Table 4.2-3-2)',
    3: 'Flight Rules Conditions (see Table 4.2-3-3)',
    4: 'Volcanic Ash (see Table 4.2-3-4)',
    5: 'Sea-surface Temperature (see Table 4.2-3-5)',
    6: 'Solar Radiation (see Table 4.2-3-6)',
    // 7-191 Reserved
    // 192-254 Reserved for Local Use
    192: 'Forecast Satellite Imagery (See note 2) (see Table 4.2-3-192)',
    255: 'Missing',
  },
  /** Product Discipline 4 - Space Weather Products */
  4: {
    0: 'Temperature (see Table 4.2-4-0)',
    1: 'Momentum (see Table 4.2-4-1)',
    2: 'Charged Particle Mass and Number (see Table 4.2-4-2)',
    3: 'Electric and Magnetic Fields (see Table 4.2-4-3)',
    4: 'Energetic Particles (see Table 4.2-4-4)',
    5: 'Waves (see Table 4.2-4-5)',
    6: 'Solar Electromagnetic Emissions (see Table 4.2-4-6)',
    7: 'Terrestrial Electromagnetic Emissions (see Table 4.2-4-7)',
    8: 'Imagery (see Table 4.2-4-8)',
    9: 'Ion-Neutral Coupling (see Table 4.2-4-9)',
    10: 'Space Weather Indices (see Table 4.2-4-10)',
    // 11-191 Reserved
    // 192-254 Reserved for Local Use
    255: 'Missing',
  },
  /** Product Discipline 10 - Oceanographic Products */
  10: {
    0: 'Waves (see Table 4.2-10-0)',
    1: 'Currents (see Table 4.2-10-1)',
    2: 'Ice (see Table 4.2-10-2)',
    3: 'Surface Properties (see Table 4.2-10-3)',
    4: 'Sub-surface Properties (see Table 4.2-10-4)',
    // 5-190 Reserved
    191: 'Miscellaneous (see Table 4.2-10-191)',
    // 192-254 Reserved for Local Use
    255: 'Missing',
  },
  /** Product Discipline 20 - Health and Socioeconomic impacts */
  20: {
    0: 'Health Indicators (see Table 4.2-20-0)',
    1: 'Epidemiology (see Table 4.2-20-1)',
    2: 'Socioeconomic indicators (see Table 4.2-20-2)',
    3: 'Renewable energy sector (see Table 4.2-20-3)',
    // 4-191 Reserved
    // 192-254 Reserved for Local Use
    255: 'Missing',
  },
};

/**
 * # GRIB2 - CODE TABLE 4.2-0-0
 *
 * **Classification**: Meteorological products, Temperature category
 *
 * **Available forms**: GRIB2
 *
 * **Defined area**: Meteorological parameters
 *
 * **Alias**: N/A
 *
 * **Domain**: Global
 *
 * **Input type**: Numeric (GRIB2 octets)
 *
 * **Output type**: Numeric (GRIB2 octets)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)
 *
 * ## Required Parameters
 * - `Parameter`: The name of the meteorological parameter.
 * - `Units`: The units of measurement for the parameter.
 * - `Abbrev`: The abbreviation for the parameter.
 *
 * ## Optional Parameters
 * - `Reserved`: Reserved values are placeholders for future use or non-used categories.
 *
 * ## Notes
 * (1) Parameter deprecated. See Regulation 92.6.2 and use another parameter instead.
 * (2) Apparent temperature is the perceived outdoor temperature, caused by a combination of phenomena, such as air temperature, relative humidity, and wind speed.
 */
export const grib2LookupTable42_00: Record<number, TableCategory> = {
  0: { parameter: 'Temperature', units: 'K', abbrev: 'TMP' },
  1: { parameter: 'Virtual Temperature', units: 'K', abbrev: 'VTMP' },
  2: { parameter: 'Potential Temperature', units: 'K', abbrev: 'POT' },
  3: {
    parameter: 'Pseudo-Adiabatic Potential Temperature (or Equivalent Potential Temperature)',
    units: 'K',
    abbrev: 'EPOT',
  },
  4: { parameter: 'Maximum Temperature', units: 'K', abbrev: 'TMAX' },
  5: { parameter: 'Minimum Temperature', units: 'K', abbrev: 'TMIN' },
  6: { parameter: 'Dew Point Temperature', units: 'K', abbrev: 'DPT' },
  7: { parameter: 'Dew Point Depression (or Deficit)', units: 'K', abbrev: 'DEPR' },
  8: { parameter: 'Lapse Rate', units: 'K m-1', abbrev: 'LAPR' },
  9: { parameter: 'Temperature Anomaly', units: 'K', abbrev: 'TMPA' },
  10: { parameter: 'Latent Heat Net Flux', units: 'W m-2', abbrev: 'LHTFL' },
  11: { parameter: 'Sensible Heat Net Flux', units: 'W m-2', abbrev: 'SHTFL' },
  12: { parameter: 'Heat Index', units: 'K', abbrev: 'HEATX' },
  13: { parameter: 'Wind Chill Factor', units: 'K', abbrev: 'WCF' },
  14: { parameter: 'Minimum Dew Point Depression', units: 'K', abbrev: 'MINDPD' },
  15: { parameter: 'Virtual Potential Temperature', units: 'K', abbrev: 'VPTMP' },
  16: { parameter: 'Snow Phase Change Heat Flux', units: 'W m-2', abbrev: 'SNOHF' },
  17: { parameter: 'Skin Temperature', units: 'K', abbrev: 'SKINT' },
  18: { parameter: 'Snow Temperature (top of snow)', units: 'K', abbrev: 'SNOT' },
  19: { parameter: 'Turbulent Transfer Coefficient for Heat', units: 'Numeric', abbrev: 'TTCHT' },
  20: { parameter: 'Turbulent Diffusion Coefficient for Heat', units: 'm2s-1', abbrev: 'TDCHT' },
  21: { parameter: 'Apparent Temperature', units: 'K', abbrev: 'APTMP' },
  22: {
    parameter: 'Temperature Tendency due to Short-Wave Radiation',
    units: 'K s-1',
    abbrev: 'TTSWR',
  },
  23: {
    parameter: 'Temperature Tendency due to Long-Wave Radiation',
    units: 'K s-1',
    abbrev: 'TTLWR',
  },
  24: {
    parameter: 'Temperature Tendency due to Short-Wave Radiation, Clear Sky',
    units: 'K s-1',
    abbrev: 'TTSWRCS',
  },
  25: {
    parameter: 'Temperature Tendency due to Long-Wave Radiation, Clear Sky',
    units: 'K s-1',
    abbrev: 'TTLWRCS',
  },
  26: {
    parameter: 'Temperature Tendency due to parameterizations',
    units: 'K s-1',
    abbrev: 'TTPARM',
  },
  27: { parameter: 'Wet Bulb Temperature', units: 'K', abbrev: 'WETBT' },
  28: { parameter: 'Unbalanced Component of Temperature', units: 'K', abbrev: 'UCTMP' },
  29: { parameter: 'Temperature Advection', units: 'K s-1', abbrev: 'TMPADV' },
  30: { parameter: 'Latent Heat Net Flux Due to Evaporation', units: 'W m-2', abbrev: 'LHFLXE' },
  31: { parameter: 'Latent Heat Net Flux Due to Sublimation', units: 'W m-2', abbrev: 'LHFLXS' },
  32: { parameter: 'Wet-Bulb Potential Temperature', units: 'K', abbrev: 'WETBPT' },
  // 33-191 Reserved
  // 192-254 Reserved for Local Use
  192: { parameter: 'Snow Phase Change Heat Flux', units: 'W m-2', abbrev: 'SNOHF' },
  193: { parameter: 'Temperature Tendency by All Radiation', units: 'K s-1', abbrev: 'TTRAD' },
  194: { parameter: 'Relative Error Variance', units: '', abbrev: 'REV' },
  195: { parameter: 'Large Scale Condensate Heating Rate', units: 'K s-1', abbrev: 'LRGHR' },
  196: { parameter: 'Deep Convective Heating Rate', units: 'K s-1', abbrev: 'CNVHR' },
  197: { parameter: 'Total Downward Heat Flux at Surface', units: 'W m-2', abbrev: 'THFLX' },
  198: { parameter: 'Temperature Tendency by All Physics', units: 'K s-1', abbrev: 'TTDIA' },
  199: {
    parameter: 'Temperature Tendency by Non-radiation Physics',
    units: 'K s-1',
    abbrev: 'TTPHY',
  },
  200: { parameter: 'Standard Dev. of IR Temp. over 1x1 deg. area', units: 'K', abbrev: 'TSD1D' },
  201: { parameter: 'Shallow Convective Heating Rate', units: 'K s-1', abbrev: 'SHAHR' },
  202: { parameter: 'Vertical Diffusion Heating rate', units: 'K s-1', abbrev: 'VDFHR' },
  203: {
    parameter: 'Potential Temperature at Top of Viscous Sublayer',
    units: 'K',
    abbrev: 'THZ0',
  },
  204: { parameter: 'Tropical Cyclone Heat Potential', units: 'J m-2 K', abbrev: 'TCHP' },
  205: { parameter: 'Effective Layer (EL) Temperature', units: 'C', abbrev: 'ELMELT' },
  206: { parameter: 'Wet Bulb Globe Temperature', units: 'K', abbrev: 'WETGLBT' },
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - CODE TABLE 4.2-0-1
 *
 * **Classification**: Meteorological products, Moisture category
 *
 * **Available forms**: GRIB2
 *
 * **Defined area**: Meteorological parameters
 *
 * **Alias**: N/A
 *
 * **Domain**: Global
 *
 * **Input type**: Numeric (GRIB2 octets)
 *
 * **Output type**: Numeric (GRIB2 octets)
 *
 * **Used by**:
 * - Section 0, Octet 7 = 0
 * - Section 4, Octet 10 = 1
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)
 *
 * ## Required Parameters
 * - `Parameter`: The name of the meteorological parameter.
 * - `Units`: The units of measurement for the parameter.
 * - `Abbrev`: The abbreviation for the parameter.
 *
 * ## Optional Parameters
 * - `Reserved`: Reserved values are placeholders for future use or non-used categories.
 *
 * ## Notes
 * (1) Parameter deprecated - See Regulation 92.6.2 and use another parameter instead.
 * (2) Total precipitation/snowfall rate stands for the sum of convective and large-scale precipitation/snowfall rate.
 * (3) Statistical process 1 (Accumulation) does not change units. It is recommended to use another parameter with "rate" in its name and accumulation in PDT.
 * (4) The listed units for this parameter appear to be inappropriate for the potential evaporation rate. Instead, it is recommended to use parameter 143.
 * (5) Total solid precipitation includes the sum of all types of solid water, e.g. graupel, snow, and hail.
 * (6) Assuming a cloud containing a bi-modal ice particle distribution, "cloud ice" refers to the small particle mode, whereas the large mode is usually called "snow". ("Ice pellets", in contrast, may refer to the precipitation of sleet, formed from freezing raindrops or refreezing (partially) melted snowflakes, or the precipitation of small hail.)
 * (7) It is recommended to use Snow melt rate instead (discipline 2, category 0, number 41).
 * (8) It is recommended to use parameter 148.
 * (9) Snow evaporation is the accumulated amount of water that has evaporated from snow from within the snow-covered area of a grid-box.
 *
 * Local Use Notes:
 * (A) The numeric value is a reference to a weather string and key table stored in the Local Use Section (Section 2) of the same GRIB2 message. See MDL Template 2.1 page and this page for more details.
 */
export const grib2LookupTable42_01: Record<number, TableCategory> = {
  0: { parameter: 'Specific Humidity', units: 'kg kg-1', abbrev: 'SPFH' },
  1: { parameter: 'Relative Humidity', units: '%', abbrev: 'RH' },
  2: { parameter: 'Humidity Mixing Ratio', units: 'kg kg-1', abbrev: 'MIXR' },
  3: { parameter: 'Precipitable Water', units: 'kg m-2', abbrev: 'PWAT' },
  4: { parameter: 'Vapour Pressure', units: 'Pa', abbrev: 'VAPP' },
  5: { parameter: 'Saturation Deficit', units: 'Pa', abbrev: 'SATD' },
  6: { parameter: 'Evaporation', units: 'kg m-2', abbrev: 'EVP' },
  7: { parameter: 'Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'PRATE' },
  8: { parameter: 'Total Precipitation', units: 'kg m-2', abbrev: 'APCP' },
  9: { parameter: 'Large-Scale Precipitation (non-convective)', units: 'kg m-2', abbrev: 'NCPCP' },
  10: { parameter: 'Convective Precipitation', units: 'kg m-2', abbrev: 'ACPCP' },
  11: { parameter: 'Snow Depth', units: 'm', abbrev: 'SNOD' },
  12: { parameter: 'Snowfall Rate Water Equivalent', units: 'kg m-2 s-1', abbrev: 'SRWEQ' },
  13: { parameter: 'Water Equivalent of Accumulated Snow Depth', units: 'kg m-2', abbrev: 'WEASD' },
  14: { parameter: 'Convective Snow', units: 'kg m-2', abbrev: 'SNOC' },
  15: { parameter: 'Large-Scale Snow', units: 'kg m-2', abbrev: 'SNOL' },
  16: { parameter: 'Snow Melt', units: 'kg m-2', abbrev: 'SNOM' },
  17: { parameter: 'Snow Age', units: 'day', abbrev: 'SNOAG' },
  18: { parameter: 'Absolute Humidity', units: 'kg m-3', abbrev: 'ABSH' },
  19: { parameter: 'Precipitation Type', units: 'See Table 4.201', abbrev: 'PTYPE' },
  20: { parameter: 'Integrated Liquid Water', units: 'kg m-2', abbrev: 'ILIQW' },
  21: { parameter: 'Condensate', units: 'kg kg-1', abbrev: 'TCOND' },
  22: { parameter: 'Cloud Mixing Ratio', units: 'kg kg-1', abbrev: 'CLMR' },
  23: { parameter: 'Ice Water Mixing Ratio', units: 'kg kg-1', abbrev: 'ICMR' },
  24: { parameter: 'Rain Mixing Ratio', units: 'kg kg-1', abbrev: 'RWMR' },
  25: { parameter: 'Snow Mixing Ratio', units: 'kg kg-1', abbrev: 'SNMR' },
  26: { parameter: 'Horizontal Moisture Convergence', units: 'kg kg-1 s-1', abbrev: 'MCONV' },
  27: { parameter: 'Maximum Relative Humidity', units: '%', abbrev: 'MAXRH' },
  28: { parameter: 'Maximum Absolute Humidity', units: 'kg m-3', abbrev: 'MAXAH' },
  29: { parameter: 'Total Snowfall', units: 'm', abbrev: 'ASNOW' },
  30: { parameter: 'Precipitable Water Category', units: 'See Table 4.202', abbrev: 'PWCAT' },
  31: { parameter: 'Hail', units: 'm', abbrev: 'HAIL' },
  32: { parameter: 'Graupel', units: 'kg kg-1', abbrev: 'GRLE' },
  33: { parameter: 'Categorical Rain', units: 'Code table 4.222', abbrev: 'CRAIN' },
  34: { parameter: 'Categorical Freezing Rain', units: 'Code table 4.222', abbrev: 'CFRZR' },
  35: { parameter: 'Categorical Ice Pellets', units: 'Code table 4.222', abbrev: 'CICEP' },
  36: { parameter: 'Categorical Snow', units: 'Code table 4.222', abbrev: 'CSNOW' },
  37: { parameter: 'Convective Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'CPRAT' },
  38: { parameter: 'Horizontal Moisture Divergence', units: 'kg kg-1 s-1', abbrev: 'MDIVER' },
  39: { parameter: 'Percent frozen precipitation', units: '%', abbrev: 'CPOFP' },
  40: { parameter: 'Potential Evaporation', units: 'kg m-2', abbrev: 'PEVAP' },
  41: { parameter: 'Potential Evaporation Rate', units: 'W m-2', abbrev: 'PEVPR' },
  42: { parameter: 'Snow Cover', units: '%', abbrev: 'SNOWC' },
  43: { parameter: 'Rain Fraction of Total Cloud Water', units: 'Proportion', abbrev: 'FRAIN' },
  44: { parameter: 'Rime Factor', units: 'Numeric', abbrev: 'RIME' },
  45: { parameter: 'Total Column Integrated Rain', units: 'kg m-2', abbrev: 'TCOLR' },
  46: { parameter: 'Total Column Integrated Snow', units: 'kg m-2', abbrev: 'TCOLS' },
  47: { parameter: 'Large Scale Water Precipitation', units: 'kg m-2', abbrev: 'LSWP' },
  48: { parameter: 'Convective Water Precipitation', units: 'kg m-2', abbrev: 'CWP' },
  49: { parameter: 'Total Water Precipitation', units: 'kg m-2', abbrev: 'TWATP' },
  50: { parameter: 'Total Snow Precipitation', units: 'kg m-2', abbrev: 'TSNOWP' },
  51: { parameter: 'Total Column Water', units: 'kg m-2', abbrev: 'TCWAT' },
  52: { parameter: 'Total Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'TPRATE' },
  53: { parameter: 'Total Snowfall Rate Water Equivalent', units: 'kg m-2 s-1', abbrev: 'TSRWE' },
  54: { parameter: 'Large Scale Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'LSPRATE' },
  55: {
    parameter: 'Convective Snowfall Rate Water Equivalent',
    units: 'kg m-2 s-1',
    abbrev: 'CSRWE',
  },
  56: {
    parameter: 'Large Scale Snowfall Rate Water Equivalent',
    units: 'kg m-2 s-1',
    abbrev: 'LSSRWE',
  },
  57: { parameter: 'Total Snowfall Rate', units: 'm s-1', abbrev: 'TSRATE' },
  58: { parameter: 'Convective Snowfall Rate', units: 'm s-1', abbrev: 'CSRATE' },
  59: { parameter: 'Large Scale Snowfall Rate', units: 'm s-1', abbrev: 'LSSRATE' },
  60: { parameter: 'Snow Depth Water Equivalent', units: 'kg m-2', abbrev: 'SDWE' },
  61: { parameter: 'Snow Density', units: 'kg m-3', abbrev: 'SDEN' },
  62: { parameter: 'Snow Evaporation', units: 'kg m-2', abbrev: 'SEVAP' },
  63: { parameter: 'Reserved', units: '', abbrev: '' }, // Reserved
  64: { parameter: 'Total Column Integrated Water Vapour', units: 'kg m-2', abbrev: 'TCIWV' },
  65: { parameter: 'Rain Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'RPRATE' },
  66: { parameter: 'Snow Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'SPRATE' },
  67: { parameter: 'Freezing Rain Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'FPRATE' },
  68: { parameter: 'Ice Pellets Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'IPRATE' },
  69: { parameter: 'Total Column Integrated Cloud Water', units: 'kg m-2', abbrev: 'TCOLW' },
  70: { parameter: 'Total Column Integrated Cloud Ice', units: 'kg m-2', abbrev: 'TCOLI' },
  71: { parameter: 'Hail Mixing Ratio', units: 'kg kg-1', abbrev: 'HAILMXR' },
  72: { parameter: 'Total Column Integrated Hail', units: 'kg m-2', abbrev: 'TCOLH' },
  73: { parameter: 'Hail Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'HAILPR' },
  74: { parameter: 'Total Column Integrated Graupel', units: 'kg m-2', abbrev: 'TCOLG' },
  75: {
    parameter: 'Graupel (Snow Pellets) Precipitation Rate',
    units: 'kg m-2 s-1',
    abbrev: 'GPRATE',
  },
  76: { parameter: 'Convective Rain Rate', units: 'kg m-2 s-1', abbrev: 'CRRATE' },
  77: { parameter: 'Large Scale Rain Rate', units: 'kg m-2 s-1', abbrev: 'LSRRATE' },
  78: {
    parameter: 'Total Column Integrated Water (All components including precipitation)',
    units: 'kg m-2',
    abbrev: 'TCOLWA',
  },
  79: { parameter: 'Evaporation Rate', units: 'kg m-2 s-1', abbrev: 'EVARATE' },
  80: { parameter: 'Total Condensate', units: 'kg kg-1', abbrev: 'TOTCON' },
  81: { parameter: 'Total Column-Integrated Condensate', units: 'kg m-2', abbrev: 'TCICON' },
  82: { parameter: 'Cloud Ice Mixing Ratio', units: 'kg kg-1', abbrev: 'CIMIXR' },
  83: { parameter: 'Specific Cloud Liquid Water Content', units: 'kg kg-1', abbrev: 'SCLLWC' },
  84: { parameter: 'Specific Cloud Ice Water Content', units: 'kg kg-1', abbrev: 'SCLIWC' },
  85: { parameter: 'Specific Rain Water Content', units: 'kg kg-1', abbrev: 'SRAINW' },
  86: { parameter: 'Specific Snow Water Content', units: 'kg kg-1', abbrev: 'SSNOWW' },
  87: { parameter: 'Stratiform Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'STRPRATE' },
  88: {
    parameter: 'Categorical Convective Precipitation',
    units: 'Code table 4.222',
    abbrev: 'CATCP',
  },
  89: { parameter: 'Reserved', units: '', abbrev: '' }, // Reserved
  90: { parameter: 'Total Kinematic Moisture Flux', units: 'kg kg-1 m s-1', abbrev: 'TKMFLX' },
  91: {
    parameter: 'U-component (zonal) Kinematic Moisture Flux',
    units: 'kg kg-1 m s-1',
    abbrev: 'UKMFLX',
  },
  92: {
    parameter: 'V-component (meridional) Kinematic Moisture Flux',
    units: 'kg kg-1 m s-1',
    abbrev: 'VKMFLX',
  },
  93: { parameter: 'Relative Humidity With Respect to Water', units: '%', abbrev: 'RHWATER' },
  94: { parameter: 'Relative Humidity With Respect to Ice', units: '%', abbrev: 'RHICE' },
  95: {
    parameter: 'Freezing or Frozen Precipitation Rate',
    units: 'kg m-2 s-1',
    abbrev: 'FZPRATE',
  },
  96: { parameter: 'Mass Density of Rain', units: 'kg m-3', abbrev: 'MASSDR' },
  97: { parameter: 'Mass Density of Snow', units: 'kg m-3', abbrev: 'MASSDS' },
  98: { parameter: 'Mass Density of Graupel', units: 'kg m-3', abbrev: 'MASSDG' },
  99: { parameter: 'Mass Density of Hail', units: 'kg m-3', abbrev: 'MASSDH' },
  100: { parameter: 'Specific Number Concentration of Rain', units: 'kg-1', abbrev: 'SPNCR' },
  101: { parameter: 'Specific Number Concentration of Snow', units: 'kg-1', abbrev: 'SPNCS' },
  102: { parameter: 'Specific Number Concentration of Graupel', units: 'kg-1', abbrev: 'SPNCG' },
  103: { parameter: 'Specific Number Concentration of Hail', units: 'kg-1', abbrev: 'SPNCH' },
  104: { parameter: 'Number Density of Rain', units: 'm-3', abbrev: 'NUMDR' },
  105: { parameter: 'Number Density of Snow', units: 'm-3', abbrev: 'NUMDS' },
  106: { parameter: 'Number Density of Graupel', units: 'm-3', abbrev: 'NUMDG' },
  107: { parameter: 'Number Density of Hail', units: 'm-3', abbrev: 'NUMDH' },
  108: {
    parameter: 'Specific Humidity Tendency due to Parameterizations',
    units: 'kg kg-1 s-1',
    abbrev: 'SHTPRM',
  },
  109: {
    parameter:
      'Mass Density of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Volume of Air',
    units: 'kg m-3',
    abbrev: 'MDLWHVA',
  },
  110: {
    parameter:
      'Specific Mass of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Mass of Moist Air',
    units: 'kg kg-1',
    abbrev: 'SMLWHMA',
  },
  111: {
    parameter:
      'Mass Mixing Ratio of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Mass of Dry Air',
    units: 'kg kg-1',
    abbrev: 'MMLWHDA',
  },
  112: {
    parameter:
      'Mass Density of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Volume of Air',
    units: 'kg m-3',
    abbrev: 'MDLWGVA',
  },
  113: {
    parameter:
      'Specific Mass of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Mass of Moist Air',
    units: 'kg kg-1',
    abbrev: 'SMLWGMA',
  },
  114: {
    parameter:
      'Mass Mixing Ratio of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Mass of Dry Air',
    units: 'kg kg-1',
    abbrev: 'MMLWGDA',
  },
  115: {
    parameter:
      'Mass Density of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Volume of Air',
    units: 'kg m-3',
    abbrev: 'MDLWSVA',
  },
  116: {
    parameter:
      'Specific Mass of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Mass of Moist Air',
    units: 'kg kg-1',
    abbrev: 'SMLWSMA',
  },
  117: {
    parameter:
      'Mass Mixing Ratio of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Mass of Dry Air',
    units: 'kg kg-1',
    abbrev: 'MMLWSDA',
  },
  118: {
    parameter: 'Unbalanced Component of Specific Humidity',
    units: 'kg kg-1',
    abbrev: 'UNCSH',
  },
  119: {
    parameter: 'Unbalanced Component of Specific Cloud Liquid Water content',
    units: 'kg kg-1',
    abbrev: 'UCSCLW',
  },
  120: {
    parameter: 'Unbalanced Component of Specific Cloud Ice Water content',
    units: 'kg kg-1',
    abbrev: 'UCSCIW',
  },
  121: { parameter: 'Fraction of Snow Cover', units: 'Proportion', abbrev: 'FSNOWC' },
  122: { parameter: 'Precipitation intensity index', units: 'See Table 4.247', abbrev: 'PIIDX' },
  123: { parameter: 'Dominant precipitation type', units: 'See Table 4.201', abbrev: 'DPTYPE' },
  124: { parameter: 'Presence of showers', units: 'See Table 4.222', abbrev: 'PSHOW' },
  125: { parameter: 'Presence of blowing snow', units: 'See Table 4.222', abbrev: 'PBSNOW' },
  126: { parameter: 'Presence of blizzard', units: 'See Table 4.222', abbrev: 'PBLIZZ' },
  127: {
    parameter: 'Ice pellets (non-water equivalent) precipitation rate',
    units: 'm s-1',
    abbrev: 'ICEP',
  },
  128: { parameter: 'Total solid precipitation rate', units: 'kg m-2 s-1', abbrev: 'TSPRATE' },
  129: { parameter: 'Effective Radius of Cloud Water', units: 'm', abbrev: 'EFRCWAT' },
  130: { parameter: 'Effective Radius of Rain', units: 'm', abbrev: 'EFRRAIN' },
  131: { parameter: 'Effective Radius of Cloud Ice', units: 'm', abbrev: 'EFRCICE' },
  132: { parameter: 'Effective Radius of Snow', units: 'm', abbrev: 'EFRSNOW' },
  133: { parameter: 'Effective Radius of Graupel', units: 'm', abbrev: 'EFRGRL' },
  134: { parameter: 'Effective Radius of Hail', units: 'm', abbrev: 'EFRHAIL' },
  135: { parameter: 'Effective Radius of Subgrid Liquid Clouds', units: 'm', abbrev: 'EFRSLC' },
  136: { parameter: 'Effective Radius of Subgrid Ice Clouds', units: 'm', abbrev: 'EFRSICEC' },
  137: { parameter: 'Effective Aspect Ratio of Rain', units: '', abbrev: 'EFARRAIN' },
  138: { parameter: 'Effective Aspect Ratio of Cloud Ice', units: '', abbrev: 'EFARCICE' },
  139: { parameter: 'Effective Aspect Ratio of Snow', units: '', abbrev: 'EFARSNOW' },
  140: { parameter: 'Effective Aspect Ratio of Graupel', units: '', abbrev: 'EFARGRL' },
  141: { parameter: 'Effective Aspect Ratio of Hail', units: '', abbrev: 'EFARHAIL' },
  142: { parameter: 'Effective Aspect Ratio of Subgrid Ice Clouds', units: '', abbrev: 'EFARSIC' },
  143: { parameter: 'Potential evaporation rate', units: 'kg m-2 s-1', abbrev: 'PERATE' },
  144: {
    parameter: 'Specific rain water content (convective)',
    units: 'kg kg-1',
    abbrev: 'SRWATERC',
  },
  145: {
    parameter: 'Specific snow water content (convective)',
    units: 'kg kg-1',
    abbrev: 'SSNOWWC',
  },
  146: { parameter: 'Cloud ice precipitation rate', units: 'kg m-2 s-1', abbrev: 'CICEPR' },
  147: { parameter: 'Character of precipitation', units: 'See Table 4.249', abbrev: 'CHPRECIP' },
  148: { parameter: 'Snow evaporation rate', units: 'kg m-2 s-1', abbrev: 'SNOWERAT' },
  149: { parameter: 'Cloud water mixing ratio', units: 'kg kg-1', abbrev: 'CWATERMR' },
  150: {
    parameter: 'Column integrated eastward water vapour mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CEWVMF',
  },
  151: {
    parameter: 'Column integrated northward water vapour mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CNWVMF',
  },
  152: {
    parameter: 'Column integrated eastward cloud liquid water mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CECLWMF',
  },
  153: {
    parameter: 'Column integrated northward cloud liquid water mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CNCLWMF',
  },
  154: {
    parameter: 'Column integrated eastward cloud ice mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CECIMF',
  },
  155: {
    parameter: 'Column integrated northward cloud ice mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CNCIMF',
  },
  156: {
    parameter: 'Column integrated eastward rain mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CERMF',
  },
  157: {
    parameter: 'Column integrated northward rain mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CNRMF',
  },
  158: {
    parameter: 'Column integrated eastward snow mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CEFMF',
  },
  159: {
    parameter: 'Column integrated northward snow mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CNSMF',
  },
  160: {
    parameter: 'Column integrated divergence of water vapour mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDWFMF',
  },
  161: {
    parameter: 'Column integrated divergence of cloud liquid water mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDCLWMF',
  },
  162: {
    parameter: 'Column integrated divergence of cloud ice mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDCIMF',
  },
  163: {
    parameter: 'Column integrated divergence of rain mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDRMF',
  },
  164: {
    parameter: 'Column integrated divergence of snow mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDSMF',
  },
  165: {
    parameter: 'Column integrated divergence of total water mass flux',
    units: 'kg m-1s-1',
    abbrev: 'CDTWMF',
  },
  166: { parameter: 'Column integrated water vapour flux', units: 'kg m-1s-1', abbrev: 'CWVF' },
  167: { parameter: 'Total column supercooled liquid water', units: 'kg m-2', abbrev: 'TCSLW' },
  168: {
    parameter: 'Saturation specific humidity with respect to water',
    units: 'kg m-3',
    abbrev: 'SSPFHW',
  },
  169: {
    parameter: 'Total column integrated saturation specific humidity with respect to water',
    units: 'kg m-2',
    abbrev: 'TCISSPFHW',
  },
  // 170-191 Reserved
  // 192-254 Reserved for Local Use
  192: { parameter: 'Categorical Rain', units: 'Code table 4.222', abbrev: 'CRAIN' },
  193: { parameter: 'Categorical Freezing Rain', units: 'Code table 4.222', abbrev: 'CFRZR' },
  194: { parameter: 'Categorical Ice Pellets', units: 'Code table 4.222', abbrev: 'CICEP' },
  195: { parameter: 'Categorical Snow', units: 'Code table 4.222', abbrev: 'CSNOW' },
  196: { parameter: 'Convective Precipitation Rate', units: 'kg m-2 s-1', abbrev: 'CPRAT' },
  197: { parameter: 'Horizontal Moisture Divergence', units: 'kg kg-1 s-1', abbrev: 'MDIV' },
  198: { parameter: 'Minimum Relative Humidity', units: '%', abbrev: 'MINRH' },
  199: { parameter: 'Potential Evaporation', units: 'kg m-2', abbrev: 'PEVAP' },
  200: { parameter: 'Potential Evaporation Rate', units: 'W m-2', abbrev: 'PEVPR' },
  201: { parameter: 'Snow Cover', units: '%', abbrev: 'SNOWC' },
  202: { parameter: 'Rain Fraction of Total Liquid Water', units: 'non-dim', abbrev: 'FRAIN' },
  203: { parameter: 'Rime Factor', units: 'non-dim', abbrev: 'RIME' },
  204: { parameter: 'Total Column Integrated Rain', units: 'kg m-2', abbrev: 'TCOLR' },
  205: { parameter: 'Total Column Integrated Snow', units: 'kg m-2', abbrev: 'TCOLS' },
  206: { parameter: 'Total Icing Potential Diagnostic', units: 'non-dim', abbrev: 'TIPD' },
  207: { parameter: 'Number concentration for ice particles', units: 'non-dim', abbrev: 'NCIP' },
  208: { parameter: 'Snow temperature', units: 'K', abbrev: 'SNOT' },
  209: {
    parameter: 'Total column-integrated supercooled liquid water',
    units: 'kg m-2',
    abbrev: 'TCLSW',
  },
  210: { parameter: 'Total column-integrated melting ice', units: 'kg m-2', abbrev: 'TCOLM' },
  211: { parameter: 'Evaporation - Precipitation', units: 'cm/day', abbrev: 'EMNP' },
  212: { parameter: 'Sublimation (evaporation from snow)', units: 'W m-2', abbrev: 'SBSNO' },
  213: { parameter: 'Deep Convective Moistening Rate', units: 'kg kg-1 s-1', abbrev: 'CNVMR' },
  214: { parameter: 'Shallow Convective Moistening Rate', units: 'kg kg-1 s-1', abbrev: 'SHAMR' },
  215: { parameter: 'Vertical Diffusion Moistening Rate', units: 'kg kg-1 s-1', abbrev: 'VDFMR' },
  216: { parameter: 'Condensation Pressure of Parcali', units: 'Pa', abbrev: 'CONDP' },
  217: { parameter: 'Large scale moistening rate', units: 'kg kg-1 s-1', abbrev: 'LRGMR' },
  218: {
    parameter: 'Specific humidity at top of viscous sublayer',
    units: 'kg kg-1',
    abbrev: 'QZ0',
  },
  219: { parameter: 'Maximum specific humidity at 2m', units: 'kg kg-1', abbrev: 'QMAX' },
  220: { parameter: 'Minimum specific humidity at 2m', units: 'kg kg-1', abbrev: 'QMIN' },
  221: { parameter: 'Liquid precipitation (Rainfall)', units: 'kg m-2', abbrev: 'ARAIN' },
  222: { parameter: 'Snow temperature, depth-avg', units: 'K', abbrev: 'SNOWT' },
  223: { parameter: 'Total precipitation (nearest grid point)', units: 'kg m-2', abbrev: 'APCPN' },
  224: {
    parameter: 'Convective precipitation (nearest grid point)',
    units: 'kg m-2',
    abbrev: 'ACPCPN',
  },
  225: { parameter: 'Freezing Rain', units: 'kg m-2', abbrev: 'FRZR' },
  226: { parameter: 'Dominant Weather', units: 'Numeric', abbrev: 'PWTHER' },
  227: { parameter: 'Frozen Rain', units: 'kg m-2', abbrev: 'FROZR' },
  228: { parameter: 'Flat Ice Accumulation (FRAM)', units: 'kg m-2', abbrev: 'FICEAC' },
  229: { parameter: 'Line Ice Accumulation (FRAM)', units: 'kg m-2', abbrev: 'LICEAC' },
  230: { parameter: 'Sleet Accumulation', units: 'kg m-2', abbrev: 'SLACC' },
  231: { parameter: 'Precipitation Potential Index', units: '%', abbrev: 'PPINDX' },
  232: { parameter: 'Probability Cloud Ice Present', units: '%', abbrev: 'PROBCIP' },
  233: { parameter: 'Snow Liquid Ratio', units: 'kg kg-1', abbrev: 'SNOWLR' },
  234: { parameter: 'Precipitation Duration', units: 'hour', abbrev: 'PCPDUR' },
  235: { parameter: 'Cloud Liquid Mixing Ratio', units: 'kg kg-1', abbrev: 'CLLMR' },
  // 236-240 Reserved
  241: { parameter: 'Total Snow', units: 'kg m-2', abbrev: 'TSNOW' },
  242: {
    parameter: 'Relative Humidity with Respect to Precipitable Water',
    units: '%',
    abbrev: 'RHPW',
  },
  245: {
    parameter: 'Hourly Maximum of Column Vertical Integrated Graupel',
    units: 'kg m-2',
    abbrev: 'MAXVIG',
  },
  255: { parameter: 'Missing', units: '', abbrev: '' }, // Missing
};

/**
 * # GRIB2 - TABLE 4.2-0-2
 *
 * **Classification**: Meteorological products, Momentum category
 *
 * **Available forms**: Numerical values
 *
 * **Defined area**: Meteorological domain
 *
 * **Alias**: Momentum, wind
 *
 * **Domain**: Meteorological
 *
 * **Input type**: Numerical
 *
 * **Output type**: Numerical
 */
export const grib2LookupTable42_02: Record<number, TableCategory> = {
  0: { parameter: 'Wind Direction (from which blowing)', units: '°', abbrev: 'WDIR' },
  1: { parameter: 'Wind Speed', units: 'm s-1', abbrev: 'WIND' },
  2: { parameter: 'U-Component of Wind', units: 'm s-1', abbrev: 'UGRD' },
  3: { parameter: 'V-Component of Wind', units: 'm s-1', abbrev: 'VGRD' },
  4: { parameter: 'Stream Function', units: 'm2 s-1', abbrev: 'STRM' },
  5: { parameter: 'Velocity Potential', units: 'm2 s-1', abbrev: 'VPOT' },
  6: { parameter: 'Montgomery Stream Function', units: 'm2 s-2', abbrev: 'MNTSF' },
  7: { parameter: 'Sigma Coordinate Vertical Velocity', units: 's-1', abbrev: 'SGCVV' },
  8: { parameter: 'Vertical Velocity (Pressure)', units: 'Pa s-1', abbrev: 'VVEL' },
  9: { parameter: 'Vertical Velocity (Geometric)', units: 'm s-1', abbrev: 'DZDT' },
  10: { parameter: 'Absolute Vorticity', units: 's-1', abbrev: 'ABSV' },
  11: { parameter: 'Absolute Divergence', units: 's-1', abbrev: 'ABSD' },
  12: { parameter: 'Relative Vorticity', units: 's-1', abbrev: 'RELV' },
  13: { parameter: 'Relative Divergence', units: 's-1', abbrev: 'RELD' },
  14: { parameter: 'Potential Vorticity', units: 'K m2 kg-1 s-1', abbrev: 'PVORT' },
  15: { parameter: 'Vertical U-Component Shear', units: 's-1', abbrev: 'VUCSH' },
  16: { parameter: 'Vertical V-Component Shear', units: 's-1', abbrev: 'VVCSH' },
  17: { parameter: 'Momentum Flux, U-Component', units: 'N m-2', abbrev: 'UFLX' },
  18: { parameter: 'Momentum Flux, V-Component', units: 'N m-2', abbrev: 'VFLX' },
  19: { parameter: 'Wind Mixing Energy', units: 'J', abbrev: 'WMIXE' },
  20: { parameter: 'Boundary Layer Dissipation', units: 'W m-2', abbrev: 'BLYDP' },
  21: { parameter: 'Maximum Wind Speed', units: 'm s-1', abbrev: 'MAXGUST' },
  22: { parameter: 'Wind Speed (Gust)', units: 'm s-1', abbrev: 'GUST' },
  23: { parameter: 'U-Component of Wind (Gust)', units: 'm s-1', abbrev: 'UGUST' },
  24: { parameter: 'V-Component of Wind (Gust)', units: 'm s-1', abbrev: 'VGUST' },
  25: { parameter: 'Vertical Speed Shear', units: 's-1', abbrev: 'VWSH' },
  26: { parameter: 'Horizontal Momentum Flux', units: 'N m-2', abbrev: 'MFLX' },
  27: { parameter: 'U-Component Storm Motion', units: 'm s-1', abbrev: 'USTM' },
  28: { parameter: 'V-Component Storm Motion', units: 'm s-1', abbrev: 'VSTM' },
  29: { parameter: 'Drag Coefficient', units: 'Numeric', abbrev: 'CD' },
  30: { parameter: 'Frictional Velocity', units: 'm s-1', abbrev: 'FRICV' },
  31: {
    parameter: 'Turbulent Diffusion Coefficient for Momentum',
    units: 'm2 s-1',
    abbrev: 'TDCMOM',
  },
  32: { parameter: 'Eta Coordinate Vertical Velocity', units: 's-1', abbrev: 'ETACVV' },
  33: { parameter: 'Wind Fetch', units: 'm', abbrev: 'WINDF' },
  34: { parameter: 'Normal Wind Component', units: 'm s-1', abbrev: 'NWIND' },
  35: { parameter: 'Tangential Wind Component', units: 'm s-1', abbrev: 'TWIND' },
  36: { parameter: 'Amplitude Function for Rossby Wave Envelope', units: 'm s-1', abbrev: 'AFRWE' },
  37: { parameter: 'Northward Turbulent Surface Stress', units: 'N m-2 s', abbrev: 'NTSS' },
  38: { parameter: 'Eastward Turbulent Surface Stress', units: 'N m-2 s', abbrev: 'ETSS' },
  39: {
    parameter: 'Eastward Wind Tendency Due to Parameterizations',
    units: 'm s-2',
    abbrev: 'EWTPARM',
  },
  40: {
    parameter: 'Northward Wind Tendency Due to Parameterizations',
    units: 'm s-2',
    abbrev: 'NWTPARM',
  },
  41: { parameter: 'U-Component of Geostrophic Wind', units: 'm s-1', abbrev: 'UGWIND' },
  42: { parameter: 'V-Component of Geostrophic Wind', units: 'm s-1', abbrev: 'VGWIND' },
  43: { parameter: 'Geostrophic Wind Direction', units: '°', abbrev: 'GEOWD' },
  44: { parameter: 'Geostrophic Wind Speed', units: 'm s-1', abbrev: 'GEOWS' },
  45: { parameter: 'Unbalanced Component of Divergence', units: 's-1', abbrev: 'UNDIV' },
  46: { parameter: 'Vorticity Advection', units: 's-2', abbrev: 'VORTADV' },
  47: { parameter: 'Surface Roughness for Heat', units: 'm', abbrev: 'SFRHEAT' },
  48: { parameter: 'Surface Roughness for Moisture', units: 'm', abbrev: 'SFRMOIST' },
  49: { parameter: 'Wind Stress', units: 'N m-2', abbrev: 'WINDSTR' },
  50: { parameter: 'Eastward Wind Stress', units: 'N m-2', abbrev: 'EWINDSTR' },
  51: { parameter: 'Northward Wind Stress', units: 'N m-2', abbrev: 'NWINDSTR' },
  52: { parameter: 'U-Component of Wind Stress', units: 'N m-2', abbrev: 'UWINDSTR' },
  53: { parameter: 'V-Component of Wind Stress', units: 'N m-2', abbrev: 'VWINDSTR' },
  54: {
    parameter: 'Natural Logarithm of Surface Roughness Length for Heat',
    units: 'm',
    abbrev: 'NLSRLH',
  },
  55: {
    parameter: 'Natural Logarithm of Surface Roughness Length for Moisture',
    units: 'm',
    abbrev: 'NLSRLM',
  },
  56: { parameter: 'U-Component of Neutral Wind', units: 'm s-1', abbrev: 'UNWIND' },
  57: { parameter: 'V-Component of Neutral Wind', units: 'm s-1', abbrev: 'VNWIND' },
  58: { parameter: 'Magnitude of Turbulent Surface Stress', units: 'N m-2', abbrev: 'TSFCSTR' },
  59: { parameter: 'Vertical Divergence', units: 's-1', abbrev: 'VDIV' },
  60: { parameter: 'Drag Thermal Coefficient', units: 'Numeric', abbrev: 'DTC' },
  61: { parameter: 'Drag Evaporation Coefficient', units: 'Numeric', abbrev: 'DEC' },
  62: { parameter: 'Eastward Turbulent Surface Stress', units: 'N m-2', abbrev: 'EASTTSS' },
  63: { parameter: 'Northward Turbulent Surface Stress', units: 'N m-2', abbrev: 'NRTHTSS' },
  // 64-191 Reserved
  192: { parameter: 'Vertical Speed Shear', units: 's-1', abbrev: 'VWSH' },
  193: { parameter: 'Horizontal Momentum Flux', units: 'N m-2', abbrev: 'MFLX' },
  194: { parameter: 'U-Component Storm Motion', units: 'm s-1', abbrev: 'USTM' },
  195: { parameter: 'V-Component Storm Motion', units: 'm s-1', abbrev: 'VSTM' },
  196: { parameter: 'Drag Coefficient', units: 'non-dim', abbrev: 'CD' },
  197: { parameter: 'Frictional Velocity', units: 'm s-1', abbrev: 'FRICV' },
  198: { parameter: 'Latitude of U Wind Component of Velocity', units: 'deg', abbrev: 'LAUV' },
  199: { parameter: 'Longitude of U Wind Component of Velocity', units: 'deg', abbrev: 'LOUV' },
  200: { parameter: 'Latitude of V Wind Component of Velocity', units: 'deg', abbrev: 'LAVV' },
  201: { parameter: 'Longitude of V Wind Component of Velocity', units: 'deg', abbrev: 'LOVV' },
  202: { parameter: 'Latitude of Pressure Point', units: 'deg', abbrev: 'LAPP' },
  203: { parameter: 'Longitude of Pressure Point', units: 'deg', abbrev: 'LOPP' },
  204: { parameter: 'Vertical Eddy Diffusivity Heat Exchange', units: 'm2 s-1', abbrev: 'VEDH' },
  205: {
    parameter: 'Covariance between Meridional and Zonal Components of the Wind',
    units: 'm2 s-2',
    abbrev: 'COVMZ',
  },
  206: {
    parameter: 'Covariance between Temperature and Zonal Components of the Wind',
    units: 'K*m s-1',
    abbrev: 'COVTZ',
  },
  207: {
    parameter: 'Covariance between Temperature and Meridional Components of the Wind',
    units: 'K*m s-1',
    abbrev: 'COVTM',
  },
  208: { parameter: 'Vertical Diffusion Zonal Acceleration', units: 'm s-2', abbrev: 'VDFUA' },
  209: { parameter: 'Vertical Diffusion Meridional Acceleration', units: 'm s-2', abbrev: 'VDFVA' },
  210: { parameter: 'Gravity Wave Drag Zonal Acceleration', units: 'm s-2', abbrev: 'GWDU' },
  211: { parameter: 'Gravity Wave Drag Meridional Acceleration', units: 'm s-2', abbrev: 'GWDV' },
  212: {
    parameter: 'Convective Zonal Momentum Mixing Acceleration',
    units: 'm s-2',
    abbrev: 'CNVU',
  },
  213: {
    parameter: 'Convective Meridional Momentum Mixing Acceleration',
    units: 'm s-2',
    abbrev: 'CNVV',
  },
  214: { parameter: 'Tendency of Vertical Velocity', units: 'm s-2', abbrev: 'WTEND' },
  215: { parameter: 'Omega (Dp/Dt) Divide by Density', units: 'K', abbrev: 'OMGALF' },
  216: {
    parameter: 'Convective Gravity Wave Drag Zonal Acceleration',
    units: 'm s-2',
    abbrev: 'CNGWDU',
  },
  217: {
    parameter: 'Convective Gravity Wave Drag Meridional Acceleration',
    units: 'm s-2',
    abbrev: 'CNGWDV',
  },
  218: { parameter: 'Velocity Point Model Surface', units: 'm', abbrev: 'LMV' },
  219: { parameter: 'Potential Vorticity (Mass-Weighted)', units: '1/s/m', abbrev: 'PVMWW' },
  220: {
    parameter: 'Hourly Maximum of Upward Vertical Velocity',
    units: 'm s-1',
    abbrev: 'MAXUVV',
  },
  221: {
    parameter: 'Hourly Maximum of Downward Vertical Velocity',
    units: 'm s-1',
    abbrev: 'MAXDVV',
  },
  222: {
    parameter: 'U Component of Hourly Maximum 10m Wind Speed',
    units: 'm s-1',
    abbrev: 'MAXUW',
  },
  223: {
    parameter: 'V Component of Hourly Maximum 10m Wind Speed',
    units: 'm s-1',
    abbrev: 'MAXVW',
  },
  224: { parameter: 'Ventilation Rate', units: 'm2 s-1', abbrev: 'VRATE' },
  225: { parameter: 'Transport Wind Speed', units: 'm s-1', abbrev: 'TRWSPD' },
  226: { parameter: 'Transport Wind Direction', units: '°', abbrev: 'TRWDIR' },
  227: {
    parameter: 'Earliest Reasonable Arrival Time (10% Exceedance)',
    units: 's',
    abbrev: 'TOA10',
  },
  228: { parameter: 'Most Likely Arrival Time (50% Exceedance)', units: 's', abbrev: 'TOA50' },
  229: { parameter: 'Most Likely Departure Time (50% Exceedance)', units: 's', abbrev: 'TOD50' },
  230: {
    parameter: 'Latest Reasonable Departure Time (90% Exceedance)',
    units: 's',
    abbrev: 'TOD90',
  },
  231: { parameter: 'Tropical Wind Direction', units: '°', abbrev: 'TPWDIR' },
  232: { parameter: 'Tropical Wind Speed', units: 'm s-1', abbrev: 'TPWSPD' },
  233: { parameter: 'Inflow Based (ESFC) to 50% EL Shear Magnitude', units: 'kt', abbrev: 'ESHR' },
  234: {
    parameter: 'U Component Inflow Based to 50% EL Shear Vector',
    units: 'kt',
    abbrev: 'UESH',
  },
  235: {
    parameter: 'V Component Inflow Based to 50% EL Shear Vector',
    units: 'kt',
    abbrev: 'VESH',
  },
  236: { parameter: 'U Component Bunkers Effective Right Motion', units: 'kt', abbrev: 'UEID' },
  237: { parameter: 'V Component Bunkers Effective Right Motion', units: 'kt', abbrev: 'VEID' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-3
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 3
 * (Meteorological products, Mass category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 3
 * Revised 12/07/2023
 * Red text depicts changes made since 06/23/2022
 * @see [GRIB2 - Table 4.2-0-3: Parameters for Discipline 0 Category 3 (Mass category)](https://www.example.com)
 */
export const grib2LookupTable42_03: Record<number, TableCategory> = {
  0: { parameter: 'Pressure', units: 'Pa', abbrev: 'PRES' },
  1: { parameter: 'Pressure Reduced to MSL', units: 'Pa', abbrev: 'PRMSL' },
  2: { parameter: 'Pressure Tendency', units: 'Pa s-1', abbrev: 'PTEND' },
  3: { parameter: 'ICAO Standard Atmosphere Reference Height', units: 'm', abbrev: 'ICAHT' },
  4: { parameter: 'Geopotential', units: 'm2 s-2', abbrev: 'GP' },
  5: { parameter: 'Geopotential Height', units: 'gpm', abbrev: 'HGT' },
  6: { parameter: 'Geometric Height', units: 'm', abbrev: 'DIST' },
  7: { parameter: 'Standard Deviation of Height', units: 'm', abbrev: 'HSTDV' },
  8: { parameter: 'Pressure Anomaly', units: 'Pa', abbrev: 'PRESA' },
  9: { parameter: 'Geopotential Height Anomaly', units: 'gpm', abbrev: 'GPA' },
  10: { parameter: 'Density', units: 'kg m-3', abbrev: 'DEN' },
  11: { parameter: 'Altimeter Setting', units: 'Pa', abbrev: 'ALTS' },
  12: { parameter: 'Thickness', units: 'm', abbrev: 'THICK' },
  13: { parameter: 'Pressure Altitude', units: 'm', abbrev: 'PRESALT' },
  14: { parameter: 'Density Altitude', units: 'm', abbrev: 'DENALT' },
  15: { parameter: '5-Wave Geopotential Height', units: 'gpm', abbrev: '5WAVH' },
  16: { parameter: 'Zonal Flux of Gravity Wave Stress', units: 'N m-2', abbrev: 'U-GWD' },
  17: { parameter: 'Meridional Flux of Gravity Wave Stress', units: 'N m-2', abbrev: 'V-GWD' },
  18: { parameter: 'Planetary Boundary Layer Height', units: 'm', abbrev: 'HPBL' },
  19: { parameter: '5-Wave Geopotential Height Anomaly', units: 'gpm', abbrev: '5WAVA' },
  20: { parameter: 'Standard Deviation of Sub-Grid Scale Orography', units: 'm', abbrev: 'SDSGSO' },
  21: { parameter: 'Angle of Sub-Grid Scale Orography', units: 'rad', abbrev: 'AOSGSO' },
  22: { parameter: 'Slope of Sub-Grid Scale Orography', units: 'Numeric', abbrev: 'SSGSO' },
  23: { parameter: 'Gravity Wave Dissipation', units: 'W m-2', abbrev: 'GWD' },
  24: { parameter: 'Anisotropy of Sub-Grid Scale Orography', units: 'Numeric', abbrev: 'ASGSO' },
  25: { parameter: 'Natural Logarithm of Pressure in Pa', units: 'Numeric', abbrev: 'NLPRES' },
  26: { parameter: 'Exner Pressure', units: 'Numeric', abbrev: 'EXPRES' },
  27: { parameter: 'Updraught Mass Flux', units: 'kg m-2 s-1', abbrev: 'UMFLX' },
  28: { parameter: 'Downdraught Mass Flux', units: 'kg m-2 s-1', abbrev: 'DMFLX' },
  29: { parameter: 'Updraught Detrainment Rate', units: 'kg m-3 s-1', abbrev: 'UDRATE' },
  30: { parameter: 'Downdraught Detrainment Rate', units: 'kg m-3 s-1', abbrev: 'DDRATE' },
  31: {
    parameter: 'Unbalanced Component of Logarithm of Surface Pressure',
    units: '-',
    abbrev: 'UCLSPRS',
  },
  32: { parameter: 'Saturation water vapour pressure', units: 'Pa', abbrev: 'SWATERVP' },
  33: { parameter: 'Geometric altitude above mean sea level', units: 'm', abbrev: 'GAMSL' },
  34: { parameter: 'Geometric height above ground level', units: 'm', abbrev: 'GHAGRD' },
  35: {
    parameter: 'Column integrated divergence of total mass flux',
    units: 'kg m-2 s-1',
    abbrev: 'CDTMF',
  },
  36: {
    parameter: 'Column integrated eastward total mass flux',
    units: 'kg m-2 s-1',
    abbrev: 'CETMF',
  },
  37: {
    parameter: 'Column integrated northward total mass flux',
    units: 'kg m-2 s-1',
    abbrev: 'CNTMF',
  },
  38: {
    parameter: 'Standard deviation of filtered subgrid orography',
    units: 'm',
    abbrev: 'SDFSO',
  },
  39: { parameter: 'Column integrated mass of atmosphere', units: 'kg m-2 s-1', abbrev: 'CMATMOS' },
  40: {
    parameter: 'Column integrated eastward geopotential flux',
    units: 'W m-1',
    abbrev: 'CEGFLUX',
  },
  41: {
    parameter: 'Column integrated northward geopotential flux',
    units: 'W m-1',
    abbrev: 'CNGFLUX',
  },
  42: {
    parameter: 'Column integrated divergence of water geopotential flux',
    units: 'W m-2',
    abbrev: 'CDWGFLUX',
  },
  43: {
    parameter: 'Column integrated divergence of geopotential flux',
    units: 'W m-2',
    abbrev: 'CDGFLUX',
  },
  44: { parameter: 'Height of zero-degree wet-bulb temperature', units: 'm', abbrev: 'HWBT' },
  45: { parameter: 'Height of one-degree wet-bulb temperature', units: 'm', abbrev: 'WOBT' },
  46: { parameter: 'Pressure departure from hydrostatic state', units: 'Pa', abbrev: 'PRESDHS' },
  // 47-191 Reserved
  192: { parameter: 'MSLP (Eta model reduction)', units: 'Pa', abbrev: 'MSLET' },
  193: { parameter: '5-Wave Geopotential Height', units: 'gpm', abbrev: '5WAVH' },
  194: { parameter: 'Zonal Flux of Gravity Wave Stress', units: 'N m-2', abbrev: 'U-GWD' },
  195: { parameter: 'Meridional Flux of Gravity Wave Stress', units: 'N m-2', abbrev: 'V-GWD' },
  196: { parameter: 'Planetary Boundary Layer Height', units: 'm', abbrev: 'HPBL' },
  197: { parameter: '5-Wave Geopotential Height Anomaly', units: 'gpm', abbrev: '5WAVA' },
  198: { parameter: 'MSLP (MAPS System Reduction)', units: 'Pa', abbrev: 'MSLMA' },
  199: {
    parameter: '3-hr pressure tendency (Std. Atmos. Reduction)',
    units: 'Pa s-1',
    abbrev: 'TSLSA',
  },
  200: { parameter: 'Pressure of level from which parcel was lifted', units: 'Pa', abbrev: 'PLPL' },
  201: { parameter: 'X-gradient of Log Pressure', units: 'm-1', abbrev: 'LPSX' },
  202: { parameter: 'Y-gradient of Log Pressure', units: 'm-1', abbrev: 'LPSY' },
  203: { parameter: 'X-gradient of Height', units: 'm-1', abbrev: 'HGTX' },
  204: { parameter: 'Y-gradient of Height', units: 'm-1', abbrev: 'HGTY' },
  205: { parameter: 'Layer Thickness', units: 'm', abbrev: 'LAYTH' },
  206: { parameter: 'Natural Log of Surface Pressure', units: 'ln (kPa)', abbrev: 'NLGSP' },
  207: { parameter: 'Convective updraft mass flux', units: 'kg m-2 s-1', abbrev: 'CNVUMF' },
  208: { parameter: 'Convective downdraft mass flux', units: 'kg m-2 s-1', abbrev: 'CNVDMF' },
  209: { parameter: 'Convective detrainment mass flux', units: 'kg m-2 s-1', abbrev: 'CNVDEMF' },
  210: { parameter: 'Mass Point Model Surface', units: '', abbrev: 'LMH' },
  211: { parameter: 'Geopotential Height (nearest grid point)', units: 'gpm', abbrev: 'HGTN' },
  212: { parameter: 'Pressure (nearest grid point)', units: 'Pa', abbrev: 'PRESN' },
  213: { parameter: 'Orographic Convexity', units: '', abbrev: 'ORCONV' },
  214: { parameter: 'Orographic Asymmetry, W Component', units: '', abbrev: 'ORASW' },
  215: { parameter: 'Orographic Asymmetry, S Component', units: '', abbrev: 'ORASS' },
  216: { parameter: 'Orographic Asymmetry, SW Component', units: '', abbrev: 'ORASSW' },
  217: { parameter: 'Orographic Asymmetry, NW Component', units: '', abbrev: 'ORASNW' },
  218: { parameter: 'Orographic Length Scale, W Component', units: '', abbrev: 'ORLSW' },
  219: { parameter: 'Orographic Length Scale, S Component', units: '', abbrev: 'ORLSS' },
  220: { parameter: 'Orographic Length Scale, SW Component', units: '', abbrev: 'ORLSSW' },
  221: { parameter: 'Orographic Length Scale, NW Component', units: '', abbrev: 'ORLSNW' },
  222: { parameter: 'Effective Surface Height', units: 'm', abbrev: 'EFSH' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-4
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 4
 * (Meteorological products, Short-wave radiation category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 4
 * Revised 11/02/2023
 * Red text depicts changes made since 06/23/2022
 * @see [GRIB2 - Table 4.2-0-4: Parameters for Discipline 0 Category 4 (Short-wave radiation category)](https://www.example.com)
 */
export const grib2LookupTable42_04: Record<number, TableCategory> = {
  0: { parameter: 'Net Short-Wave Radiation Flux (Surface)', units: 'W m-2', abbrev: 'NSWRS' },
  1: {
    parameter: 'Net Short-Wave Radiation Flux (Top of Atmosphere)',
    units: 'W m-2',
    abbrev: 'NSWRT',
  },
  2: { parameter: 'Short-Wave Radiation Flux', units: 'W m-2', abbrev: 'SWAVR' },
  3: { parameter: 'Global Radiation Flux', units: 'W m-2', abbrev: 'GRAD' },
  4: { parameter: 'Brightness Temperature', units: 'K', abbrev: 'BRTMP' },
  5: { parameter: 'Radiance (with respect to wave number)', units: 'W m-1 sr-1', abbrev: 'LWRAD' },
  6: { parameter: 'Radiance (with respect to wavelength)', units: 'W m-3 sr-1', abbrev: 'SWRAD' },
  7: { parameter: 'Downward Short-Wave Radiation Flux', units: 'W m-2', abbrev: 'DSWRF' },
  8: { parameter: 'Upward Short-Wave Radiation Flux', units: 'W m-2', abbrev: 'USWRF' },
  9: { parameter: 'Net Short Wave Radiation Flux', units: 'W m-2', abbrev: 'NSWRF' },
  10: { parameter: 'Photosynthetically Active Radiation', units: 'W m-2', abbrev: 'PHOTAR' },
  11: { parameter: 'Net Short-Wave Radiation Flux, Clear Sky', units: 'W m-2', abbrev: 'NSWRFCS' },
  12: { parameter: 'Downward UV Radiation', units: 'W m-2', abbrev: 'DWUVR' },
  13: { parameter: 'Direct Short Wave Radiation Flux', units: 'W m-2', abbrev: 'DSWRFLX' },
  14: { parameter: 'Diffuse Short Wave Radiation Flux', units: 'W m-2', abbrev: 'DIFSWRF' },
  15: {
    parameter: "Upward UV radiation emitted/reflected from the Earth's surface",
    units: 'W m-2',
    abbrev: 'UVVEARTH',
  },
  // 16-49 Reserved
  50: { parameter: 'UV Index (Under Clear Sky)', units: 'Numeric', abbrev: 'UVIUCS' },
  51: { parameter: 'UV Index', units: 'Numeric', abbrev: 'UVI' },
  52: {
    parameter: 'Downward Short-Wave Radiation Flux, Clear Sky',
    units: 'W m-2',
    abbrev: 'DSWRFCS',
  },
  53: {
    parameter: 'Upward Short-Wave Radiation Flux, Clear Sky',
    units: 'W m-2',
    abbrev: 'USWRFCS',
  },
  54: { parameter: 'Direct normal short-wave radiation flux', units: 'W m-2', abbrev: 'DNSWRFLX' },
  55: { parameter: 'UV visible albedo for diffuse radiation', units: '%', abbrev: 'UVALBDIF' },
  56: { parameter: 'UV visible albedo for direct radiation', units: '%', abbrev: 'UVALBDIR' },
  57: {
    parameter: 'UV visible albedo for direct radiation, geometric component',
    units: '%',
    abbrev: 'UBALBDIRG',
  },
  58: {
    parameter: 'UV visible albedo for direct radiation, isotropic component',
    units: '%',
    abbrev: 'UVALBDIRI',
  },
  59: {
    parameter: 'UV visible albedo for direct radiation, volumetric component',
    units: '%',
    abbrev: 'UVBDIRV',
  },
  60: {
    parameter: 'Photosynthetically active radiation flux, clear sky',
    units: 'W m-2',
    abbrev: 'PHOARFCS',
  },
  61: {
    parameter: 'Direct short-wave radiation flux, clear sky',
    units: 'W m-2',
    abbrev: 'DSWRFLXCS',
  },
  // 62-191 Reserved
  // 192-254 Reserved for Local Use
  192: { parameter: 'Downward Short-Wave Radiation Flux', units: 'W m-2', abbrev: 'DSWRF' },
  193: { parameter: 'Upward Short-Wave Radiation Flux', units: 'W m-2', abbrev: 'USWRF' },
  194: { parameter: 'UV-B Downward Solar Flux', units: 'W m-2', abbrev: 'DUVB' },
  195: { parameter: 'Clear sky UV-B Downward Solar Flux', units: 'W m-2', abbrev: 'CDUVB' },
  196: { parameter: 'Clear Sky Downward Solar Flux', units: 'W m-2', abbrev: 'CSDSF' },
  197: { parameter: 'Solar Radiative Heating Rate', units: 'K s-1', abbrev: 'SWHR' },
  198: { parameter: 'Clear Sky Upward Solar Flux', units: 'W m-2', abbrev: 'CSUSF' },
  199: { parameter: 'Cloud Forcing Net Solar Flux', units: 'W m-2', abbrev: 'CFNSF' },
  200: { parameter: 'Visible Beam Downward Solar Flux', units: 'W m-2', abbrev: 'VBDSF' },
  201: { parameter: 'Visible Diffuse Downward Solar Flux', units: 'W m-2', abbrev: 'VDDSF' },
  202: { parameter: 'Near IR Beam Downward Solar Flux', units: 'W m-2', abbrev: 'NBDSF' },
  203: { parameter: 'Near IR Diffuse Downward Solar Flux', units: 'W m-2', abbrev: 'NDDSF' },
  204: { parameter: 'Downward Total Radiation Flux', units: 'W m-2', abbrev: 'DTRF' },
  205: { parameter: 'Upward Total Radiation Flux', units: 'W m-2', abbrev: 'UTRF' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-5
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 5
 * (Meteorological products, Long-wave radiation category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 5
 * Revised 11/02/2023
 * Red text depicts changes made since 11/02/2023
 * @see [GRIB2 - Table 4.2-0-5: Parameters for Discipline 0 Category 5 (Long-wave radiation category)](https://www.example.com)
 */
export const grib2LookupTable42_05: Record<number, TableCategory> = {
  0: { parameter: 'Net Long-Wave Radiation Flux (Surface)', units: 'W m-2', abbrev: 'NLWRS' },
  1: {
    parameter: 'Net Long-Wave Radiation Flux (Top of Atmosphere)',
    units: 'W m-2',
    abbrev: 'NLWRT',
  },
  2: { parameter: 'Long-Wave Radiation Flux', units: 'W m-2', abbrev: 'LWAVR' },
  3: { parameter: 'Downward Long-Wave Rad. Flux', units: 'W m-2', abbrev: 'DLWRF' },
  4: { parameter: 'Upward Long-Wave Rad. Flux', units: 'W m-2', abbrev: 'ULWRF' },
  5: { parameter: 'Net Long-Wave Radiation Flux', units: 'W m-2', abbrev: 'NLWRF' },
  6: { parameter: 'Net Long-Wave Radiation Flux, Clear Sky', units: 'W m-2', abbrev: 'NLWRCS' },
  7: { parameter: 'Brightness Temperature', units: 'K', abbrev: 'BRTEMP' },
  8: {
    parameter: 'Downward Long-Wave Radiation Flux, Clear Sky',
    units: 'W m-2',
    abbrev: 'DLWRFCS',
  },
  9: { parameter: 'Near IR albedo for diffuse radiation', units: '%', abbrev: 'NIRALBDIF' },
  10: { parameter: 'Near IR albedo for direct radiation', units: '%', abbrev: 'NIRALBDIR' },
  11: {
    parameter: 'Near IR albedo for direct radiation, geometric component',
    units: '%',
    abbrev: 'NIRALBDIRG',
  },
  12: {
    parameter: 'Near IR albedo for direct radiation, isotropic component',
    units: '%',
    abbrev: 'NIRALBDIRI',
  },
  13: {
    parameter: 'Near IR albedo for direct radiation, volumetric component',
    units: '%',
    abbrev: 'NIRALBDIRV',
  },
  // 14-191 Reserved
  192: { parameter: 'Downward Long-Wave Rad. Flux', units: 'W m-2', abbrev: 'DLWRF' },
  193: { parameter: 'Upward Long-Wave Rad. Flux', units: 'W m-2', abbrev: 'ULWRF' },
  194: { parameter: 'Long-Wave Radiative Heating Rate', units: 'K s-1', abbrev: 'LWHR' },
  195: { parameter: 'Clear Sky Upward Long Wave Flux', units: 'W m-2', abbrev: 'CSULF' },
  196: { parameter: 'Clear Sky Downward Long Wave Flux', units: 'W m-2', abbrev: 'CSDLF' },
  197: { parameter: 'Cloud Forcing Net Long Wave Flux', units: 'W m-2', abbrev: 'CFNLF' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-6
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 6
 * (Meteorological products, Cloud category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 6
 * Revised 10/24/2023
 * Red text depicts changes made since 10/24/2023
 *
 * **Notes:**
 * - Parameter deprecated - Use another parameter in parameter category 1: moisture instead.
 * - The sum of the water and ice fractions may exceed the total due to overlap between the volumes containing ice and those containing liquid water.
 * - Fog is defined as cloud cover in the lowest model level.
 * - This parameter is the amount of sunshine in seconds over a given length of time in seconds. Sunshine is defined as a radiation intensity above 120 W m-2.
 * @see [GRIB2 - Table 4.2-0-6: Parameters for Discipline 0 Category 6 (Cloud category)](https://www.example.com)
 */
export const grib2LookupTable42_06: Record<number, TableCategory> = {
  0: { parameter: 'Cloud Ice', units: 'kg m-2', abbrev: 'CICE' },
  1: { parameter: 'Total Cloud Cover', units: '%', abbrev: 'TCDC' },
  2: { parameter: 'Convective Cloud Cover', units: '%', abbrev: 'CDCON' },
  3: { parameter: 'Low Cloud Cover', units: '%', abbrev: 'LCDC' },
  4: { parameter: 'Medium Cloud Cover', units: '%', abbrev: 'MCDC' },
  5: { parameter: 'High Cloud Cover', units: '%', abbrev: 'HCDC' },
  6: { parameter: 'Cloud Water', units: 'kg m-2', abbrev: 'CWAT' },
  7: { parameter: 'Cloud Amount', units: '%', abbrev: 'CDCA' },
  8: { parameter: 'Cloud Type', units: 'See Table 4.203', abbrev: 'CDCT' },
  9: { parameter: 'Thunderstorm Maximum Tops', units: 'm', abbrev: 'TMAXT' },
  10: { parameter: 'Thunderstorm Coverage', units: 'See Table 4.204', abbrev: 'THUNC' },
  11: { parameter: 'Cloud Base', units: 'm', abbrev: 'CDCB' },
  12: { parameter: 'Cloud Top', units: 'm', abbrev: 'CDCTOP' },
  13: { parameter: 'Ceiling', units: 'm', abbrev: 'CEIL' },
  14: { parameter: 'Non-Convective Cloud Cover', units: '%', abbrev: 'CDLYR' },
  15: { parameter: 'Cloud Work Function', units: 'J kg-1', abbrev: 'CWORK' },
  16: { parameter: 'Convective Cloud Efficiency', units: 'Proportion', abbrev: 'CUEFI' },
  17: { parameter: 'Total Condensate', units: 'kg kg-1', abbrev: 'TCONDO' }, // Deprecated
  18: { parameter: 'Total Column-Integrated Cloud Water', units: 'kg m-2', abbrev: 'TCOLWO' }, // Deprecated
  19: { parameter: 'Total Column-Integrated Cloud Ice', units: 'kg m-2', abbrev: 'TCOLIO' }, // Deprecated
  20: { parameter: 'Total Column-Integrated Condensate', units: 'kg m-2', abbrev: 'TCOLC' }, // Deprecated
  21: { parameter: 'Ice fraction of total condensate', units: 'Proportion', abbrev: 'FICE' },
  22: { parameter: 'Cloud Cover', units: '%', abbrev: 'CDCC' },
  23: { parameter: 'Cloud Ice Mixing Ratio', units: 'kg kg-1', abbrev: 'CDCIMR' }, // Deprecated
  24: { parameter: 'Sunshine', units: 'Numeric', abbrev: 'SUNS' },
  25: { parameter: 'Horizontal Extent of Cumulonimbus (CB)', units: '%', abbrev: 'CBHE' },
  26: { parameter: 'Height of Convective Cloud Base', units: 'm', abbrev: 'HCONCB' },
  27: { parameter: 'Height of Convective Cloud Top', units: 'm', abbrev: 'HCONCT' },
  28: { parameter: 'Number Concentration of Cloud Droplets', units: 'kg-1', abbrev: 'NCONCD' },
  29: { parameter: 'Number Concentration of Cloud Ice', units: 'kg-1', abbrev: 'NCCICE' },
  30: { parameter: 'Number Density of Cloud Droplets', units: 'm-3', abbrev: 'NDENCD' },
  31: { parameter: 'Number Density of Cloud Ice', units: 'm-3', abbrev: 'NDCICE' },
  32: { parameter: 'Fraction of Cloud Cover', units: 'Numeric', abbrev: 'FRACCC' },
  33: { parameter: 'Sunshine Duration', units: 's', abbrev: 'SUNSD' },
  34: {
    parameter: 'Surface Long Wave Effective Total Cloudiness',
    units: 'Numeric',
    abbrev: 'SLWTC',
  },
  35: {
    parameter: 'Surface Short Wave Effective Total Cloudiness',
    units: 'Numeric',
    abbrev: 'SSWTC',
  },
  36: {
    parameter: 'Fraction of Stratiform Precipitation Cover',
    units: 'Proportion',
    abbrev: 'FSTRPC',
  },
  37: {
    parameter: 'Fraction of Convective Precipitation Cover',
    units: 'Proportion',
    abbrev: 'FCONPC',
  },
  38: { parameter: 'Mass Density of Cloud Droplets', units: 'kg m-3', abbrev: 'MASSDCD' },
  39: { parameter: 'Mass Density of Cloud Ice', units: 'kg m-3', abbrev: 'MASSDCI' },
  40: {
    parameter: 'Mass Density of Convective Cloud Water Droplets',
    units: 'kg m-3',
    abbrev: 'MDCCWD',
  },
  // 41-46 Reserved
  47: { parameter: 'Volume Fraction of Cloud Water Droplets', units: 'Numeric', abbrev: 'VFRCWD' }, // Note 2
  48: { parameter: 'Volume Fraction of Cloud Ice Particles', units: 'Numeric', abbrev: 'VFRCICE' }, // Note 2
  49: {
    parameter: 'Volume Fraction of Cloud (Ice and/or Water)',
    units: 'Numeric',
    abbrev: 'VFRCIW',
  }, // Note 2
  50: { parameter: 'Fog', units: '%', abbrev: 'FOG' }, // Note 3
  51: { parameter: 'Sunshine Duration Fraction', units: 'Proportion', abbrev: 'SUNFRAC' }, // Note 4
  // 52-191 Reserved
  192: { parameter: 'Non-Convective Cloud Cover', units: '%', abbrev: 'CDLYR' },
  193: { parameter: 'Cloud Work Function', units: 'J kg-1', abbrev: 'CWORK' },
  194: { parameter: 'Convective Cloud Efficiency', units: 'non-dim', abbrev: 'CUEFI' },
  195: { parameter: 'Total Condensate', units: 'kg kg-1', abbrev: 'TCOND' },
  196: { parameter: 'Total Column-Integrated Cloud Water', units: 'kg m-2', abbrev: 'TCOLW' },
  197: { parameter: 'Total Column-Integrated Cloud Ice', units: 'kg m-2', abbrev: 'TCOLI' },
  198: { parameter: 'Total Column-Integrated Condensate', units: 'kg m-2', abbrev: 'TCOLC' },
  199: { parameter: 'Ice fraction of total condensate', units: 'non-dim', abbrev: 'FICE' },
  200: { parameter: 'Convective Cloud Mass Flux', units: 'Pa s-1', abbrev: 'MFLUX' },
  201: { parameter: 'Sunshine Duration', units: 's', abbrev: 'SUNSD' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-7
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 7
 * (Meteorological products, Thermodynamic Stability category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 7
 * Revised 06/23/2022
 * Red text depicts changes made since 01/25/2021
 *
 * **Notes:**
 * - Parameter deprecated - Use another parameter in parameter category 1: moisture instead.
 * @see [GRIB2 - Table 4.2-0-7: Parameters for Discipline 0 Category 7 (Thermodynamic Stability category)](https://www.example.com)
 */
export const grib2LookupTable42_07: Record<number, TableCategory> = {
  0: { parameter: 'Parcel Lifted Index (to 500 hPa)', units: 'K', abbrev: 'PLI' },
  1: { parameter: 'Best Lifted Index (to 500 hPa)', units: 'K', abbrev: 'BLI' },
  2: { parameter: 'K Index', units: 'K', abbrev: 'KX' },
  3: { parameter: 'KO Index', units: 'K', abbrev: 'KOX' },
  4: { parameter: 'Total Totals Index', units: 'K', abbrev: 'TOTALX' },
  5: { parameter: 'Sweat Index', units: 'Numeric', abbrev: 'SX' },
  6: { parameter: 'Convective Available Potential Energy', units: 'J kg-1', abbrev: 'CAPE' },
  7: { parameter: 'Convective Inhibition', units: 'J kg-1', abbrev: 'CIN' },
  8: { parameter: 'Storm Relative Helicity', units: 'm2 s-2', abbrev: 'HLCY' },
  9: { parameter: 'Energy Helicity Index', units: 'Numeric', abbrev: 'EHLX' },
  10: { parameter: 'Surface Lifted Index', units: 'K', abbrev: 'LFT X' },
  11: { parameter: 'Best (4 layer) Lifted Index', units: 'K', abbrev: '4LFTX' },
  12: { parameter: 'Richardson Number', units: 'Numeric', abbrev: 'RI' },
  13: { parameter: 'Showalter Index', units: 'K', abbrev: 'SHWINX' },
  // 14 Reserved
  15: { parameter: 'Updraft Helicity', units: 'm2 s-2', abbrev: 'UPHL' },
  16: { parameter: 'Bulk Richardson Number', units: 'Numeric', abbrev: 'BLKRN' },
  17: { parameter: 'Gradient Richardson Number', units: 'Numeric', abbrev: 'GRDRN' },
  18: { parameter: 'Flux Richardson Number', units: 'Numeric', abbrev: 'FLXRN' },
  19: {
    parameter: 'Convective Available Potential Energy Shear',
    units: 'm2 s-2',
    abbrev: 'CONAPES',
  },
  20: { parameter: 'Thunderstorm intensity index', units: 'See Table 4.246', abbrev: 'TIIDEX' },
  // 21-191 Reserved
  192: { parameter: 'Surface Lifted Index', units: 'K', abbrev: 'LFT X' },
  193: { parameter: 'Best (4 layer) Lifted Index', units: 'K', abbrev: '4LFTX' },
  194: { parameter: 'Richardson Number', units: 'Numeric', abbrev: 'RI' },
  195: { parameter: 'Convective Weather Detection Index', units: '', abbrev: 'CWDI' },
  196: { parameter: 'Ultra Violet Index', units: 'W m-2', abbrev: 'UVI' },
  197: { parameter: 'Updraft Helicity', units: 'm2 s-2', abbrev: 'UPHL' },
  198: { parameter: 'Leaf Area Index', units: 'Numeric', abbrev: 'LAI' },
  199: { parameter: 'Hourly Maximum of Updraft Helicity', units: 'm2 s-2', abbrev: 'MXUPHL' },
  200: { parameter: 'Hourly Minimum of Updraft Helicity', units: 'm2 s-2', abbrev: 'MNUPHL' },
  201: {
    parameter: 'Bourgoiun Negative Energy Layer (surface to freezing level)',
    units: 'J kg-1',
    abbrev: 'BNEGELAY',
  },
  202: {
    parameter: 'Bourgoiun Positive Energy Layer (2k ft AGL to 400 hPa)',
    units: 'J kg-1',
    abbrev: 'BPOSELAY',
  },
  203: { parameter: 'Downdraft CAPE', units: 'J kg-1', abbrev: 'DCAPE' },
  204: { parameter: 'Effective Storm Relative Helicity', units: 'm2 s-2', abbrev: 'EFHL' },
  205: { parameter: 'Enhanced Stretching Potential', units: 'Numeric', abbrev: 'ESP' },
  206: { parameter: 'Critical Angle', units: 'Degree', abbrev: 'CANGLE' },
  207: { parameter: 'Effective Surface Helicity', units: 'm2 s-2', abbrev: 'E3KH' },
  208: {
    parameter: 'Significant Tornado Parameter with CIN-Effective Layer',
    units: 'numeric',
    abbrev: 'STPC',
  },
  209: { parameter: 'Significant Hail Parameter', units: 'numeric', abbrev: 'SIGH' },
  210: {
    parameter: 'Supercell Composite Parameter-Effective Layer',
    units: 'numeric',
    abbrev: 'SCCP',
  },
  211: { parameter: 'Significant Tornado parameter-Fixed Layer', units: 'numeric', abbrev: 'SIGT' },
  212: { parameter: 'Mixed Layer (100 mb) Virtual LFC', units: 'numeric', abbrev: 'MLFC' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-13
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 13
 * (Meteorological products, Aerosols category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 13
 * Revised 02/13/2012
 * Red text depicts changes made since 01/26/2006
 *
 * **Notes:**
 * - Aerosol Type is described in Table 4.205.
 * @see [GRIB2 - Table 4.2-0-13: Aerosols Category](https://www.example.com)
 */
export const grib2LookupTable42_013: Record<number, TableCategory> = {
  0: { parameter: 'Aerosol Type', units: 'See Table 4.205', abbrev: 'AEROT' },
  // 1-191 Reserved
  192: { parameter: 'Particulate matter (coarse)', units: 'µg m-3', abbrev: 'PMTC' },
  193: { parameter: 'Particulate matter (fine)', units: 'µg m-3', abbrev: 'PMTF' },
  194: { parameter: 'Particulate matter (fine)', units: 'log10 (µg m-3)', abbrev: 'LPMTF' },
  195: {
    parameter: 'Integrated column particulate matter (fine)',
    units: 'log10 (µg m-3)',
    abbrev: 'LIPMF',
  },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-14
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 14
 * (Meteorological products, Trace Gases category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 14
 * Revised 12/04/2020
 * Red text depicts changes made since 02/13/2012
 *
 * **Notes:**
 * - Trace gases parameters, including Ozone and PM2.5 related metrics.
 * @see [GRIB2 - Table 4.2-0-14: Trace Gases Category](https://www.example.com)
 */
export const grib2LookupTable42_014: Record<number, TableCategory> = {
  0: { parameter: 'Total Ozone', units: 'DU', abbrev: 'TOZNE' },
  1: { parameter: 'Ozone Mixing Ratio', units: 'kg kg-1', abbrev: 'O3MR' },
  2: { parameter: 'Total Column Integrated Ozone', units: 'DU', abbrev: 'TCIOZ' },
  // 3-191 Reserved
  192: { parameter: 'Ozone Mixing Ratio', units: 'kg kg-1', abbrev: 'O3MR' },
  193: { parameter: 'Ozone Concentration', units: 'ppb', abbrev: 'OZCON' },
  194: { parameter: 'Categorical Ozone Concentration', units: 'Non-Dim', abbrev: 'OZCAT' },
  195: { parameter: 'Ozone Vertical Diffusion', units: 'kg kg-1 s-1', abbrev: 'VDFOZ' },
  196: { parameter: 'Ozone Production', units: 'kg kg-1 s-1', abbrev: 'POZ' },
  197: { parameter: 'Ozone Tendency', units: 'kg kg-1 s-1', abbrev: 'TOZ' },
  198: {
    parameter: 'Ozone Production from Temperature Term',
    units: 'kg kg-1 s-1',
    abbrev: 'POZT',
  },
  199: {
    parameter: 'Ozone Production from Column Ozone Term',
    units: 'kg kg-1 s-1',
    abbrev: 'POZO',
  },
  200: { parameter: 'Ozone Daily Max from 1-hour Average', units: 'ppbV', abbrev: 'OZMAX1' },
  201: { parameter: 'Ozone Daily Max from 8-hour Average', units: 'ppbV', abbrev: 'OZMAX8' },
  202: { parameter: 'PM 2.5 Daily Max from 1-hour Average', units: 'μg m-3', abbrev: 'PDMAX1' },
  203: { parameter: 'PM 2.5 Daily Max from 24-hour Average', units: 'μg m-3', abbrev: 'PDMAX24' },
  204: { parameter: 'Acetaldehyde & Higher Aldehydes', units: 'ppbV', abbrev: 'ALD2' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-15
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 15
 * (Meteorological products, Radar category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 15
 * Revised 12/05/2014
 * Red text depicts changes made since 04/08/2013
 *
 * **Notes:**
 * - Radar-related parameters such as reflectivity, velocity, and precipitation.
 * @see [GRIB2 - Table 4.2-0-15: Radar Category](https://www.example.com)
 */
export const grib2LookupTable42_015: Record<number, TableCategory> = {
  0: { parameter: 'Base Spectrum Width', units: 'm s-1', abbrev: 'BSWID' },
  1: { parameter: 'Base Reflectivity', units: 'dB', abbrev: 'BREF' },
  2: { parameter: 'Base Radial Velocity', units: 'm s-1', abbrev: 'BRVEL' },
  3: { parameter: 'Vertically-Integrated Liquid Water', units: 'kg m-2', abbrev: 'VIL' },
  4: { parameter: 'Layer Maximum Base Reflectivity', units: 'dB', abbrev: 'LMAXBR' },
  5: { parameter: 'Precipitation', units: 'kg m-2', abbrev: 'PREC' },
  6: { parameter: 'Radar Spectra (1)', units: '', abbrev: 'RDSP1' },
  7: { parameter: 'Radar Spectra (2)', units: '', abbrev: 'RDSP2' },
  8: { parameter: 'Radar Spectra (3)', units: '', abbrev: 'RDSP3' },
  9: { parameter: 'Reflectivity of Cloud Droplets', units: 'dB', abbrev: 'RFCD' },
  10: { parameter: 'Reflectivity of Cloud Ice', units: 'dB', abbrev: 'RFCI' },
  11: { parameter: 'Reflectivity of Snow', units: 'dB', abbrev: 'RFSNOW' },
  12: { parameter: 'Reflectivity of Rain', units: 'dB', abbrev: 'RFRAIN' },
  13: { parameter: 'Reflectivity of Graupel', units: 'dB', abbrev: 'RFGRPL' },
  14: { parameter: 'Reflectivity of Hail', units: 'dB', abbrev: 'RFHAIL' },
  15: { parameter: 'Hybrid Scan Reflectivity', units: 'dB', abbrev: 'HSR' },
  16: { parameter: 'Hybrid Scan Reflectivity Height', units: 'm', abbrev: 'HSRHT' },
  // 17-191 Reserved
  192: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-16
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 16
 * (Meteorological products, Forecast Radar Imagery category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 16
 * Revised 02/10/2021
 * Red text depicts changes made since 12/06/2011
 *
 * **Notes:**
 * - Radar reflectivity and Echo Top products.
 * - For Echo Top product, Use octet number 38 to store threshold value (e.g., 18.3 dB) in Product Definition Template 4.20.
 * - Decibel (dB) is a logarithmic measure of the relative power or radar reflectivity.
 * @see [GRIB2 - Table 4.2-0-16: Forecast Radar Imagery](https://www.example.com)
 */
export const grib2LookupTable42_016: Record<number, TableCategory> = {
  0: {
    parameter: 'Equivalent radar reflectivity factor for rain',
    units: 'm m6 m-3',
    abbrev: 'REFZR',
  },
  1: {
    parameter: 'Equivalent radar reflectivity factor for snow',
    units: 'm m6 m-3',
    abbrev: 'REFZI',
  },
  2: {
    parameter: 'Equivalent radar reflectivity factor for parameterized convection',
    units: 'm m6 m-3',
    abbrev: 'REFZC',
  },
  3: { parameter: 'Echo Top', units: 'm', abbrev: 'RETOP' },
  4: { parameter: 'Reflectivity', units: 'dB', abbrev: 'REFD' },
  5: { parameter: 'Composite reflectivity', units: 'dB', abbrev: 'REFC' },
  // 6-191 Reserved
  192: {
    parameter: 'Equivalent radar reflectivity factor for rain',
    units: 'm m6 m-3',
    abbrev: 'REFZR',
  },
  193: {
    parameter: 'Equivalent radar reflectivity factor for snow',
    units: 'm m6 m-3',
    abbrev: 'REFZI',
  },
  194: {
    parameter: 'Equivalent radar reflectivity factor for parameterized convection',
    units: 'm m6 m-3',
    abbrev: 'REFZC',
  },
  195: { parameter: 'Reflectivity', units: 'dB', abbrev: 'REFD' },
  196: { parameter: 'Composite reflectivity', units: 'dB', abbrev: 'REFC' },
  197: { parameter: 'Echo Top', units: 'm', abbrev: 'RETOP' },
  198: { parameter: 'Hourly Maximum of Simulated Reflectivity', units: 'dB', abbrev: 'MAXREF' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-17
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 17
 * (Meteorological products, Electrodynamics category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 17
 * Revised 02/23/2021
 * Red text depicts changes made since 05/28/2019
 *
 * **Notes:**
 * 1. Definition of LPI after Lynn et. al. (2010): Prediction of lightning flash density with the WRF model, Adv. Geosci., 23, 11-16.
 * 2. The total lightning flash density is the sum of cloud-to-ground and cloud-to-cloud lightning flash densities.
 * 3. The subgrid-scale lightning potential index is derived from subgrid-scale information for models with coarser resolution.
 * @see [GRIB2 - Table 4.2-0-17: Electrodynamics](https://www.example.com)
 */
export const grib2LookupTable42_017: Record<number, TableCategory> = {
  0: { parameter: 'Lightning Strike Density', units: 'm-2 s-1', abbrev: 'LTNGSD' },
  1: { parameter: 'Lightning Potential Index (LPI)', units: 'J kg-1', abbrev: 'LTPINX' },
  2: {
    parameter: 'Cloud-to-Ground Lightning Flash Density',
    units: 'km-2 day-1',
    abbrev: 'CDGDLTFD',
  },
  3: {
    parameter: 'Cloud-to-Cloud Lightning Flash Density',
    units: 'km-2 day-1',
    abbrev: 'CDCDLTFD',
  },
  4: { parameter: 'Total Lightning Flash Density', units: 'km-2 day-1', abbrev: 'TLGTFD' },
  5: { parameter: 'Subgrid-scale lightning potential index', units: 'J kg-1', abbrev: 'SLNGPIDX' },
  // 6-191 Reserved
  192: { parameter: 'Lightning', units: 'non-dim', abbrev: 'LTNG' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-18
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 18
 * (Meteorological products, Nuclear/Radiology Imagery category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 18
 * Revised 07/15/2024
 * Red text depicts changes made since 07/15/2024
 *
 * **Notes:**
 * 1. Statistical process 1 (Accumulation) does not change units. It is recommended to use another parameter
 * without the word "time-integrated" in its name and accumulation in PDT.
 * 2. Conversion factor between "Specific Activity Concentration" (14) and "Air Concentration" (10) is "Mass Density" [kg m-3].
 * 3. Use the radionuclide release start date as baseline to determine activity arrival or activity end.
 * @see [GRIB2 - Table 4.2-0-18: Nuclear/Radiology Imagery](https://www.example.com)
 */
export const grib2LookupTable42_018: Record<number, TableCategory> = {
  0: { parameter: 'Air Concentration of Caesium 137', units: 'Bq m-3', abbrev: 'ACCES' },
  1: { parameter: 'Air Concentration of Iodine 131', units: 'Bq m-3', abbrev: 'ACIOD' },
  2: { parameter: 'Air Concentration of Radioactive Pollutant', units: 'Bq m-3', abbrev: 'ACRADP' },
  3: { parameter: 'Ground Deposition of Caesium 137', units: 'Bq m-2', abbrev: 'GDCES' },
  4: { parameter: 'Ground Deposition of Iodine 131', units: 'Bq m-2', abbrev: 'GDIOD' },
  5: { parameter: 'Ground Deposition of Radioactive Pollutant', units: 'Bq m-2', abbrev: 'GDRADP' },
  6: {
    parameter: 'Time Integrated Air Concentration of Cesium Pollutant',
    units: 'Bq s m-3',
    abbrev: 'TIACCP',
  },
  7: {
    parameter: 'Time Integrated Air Concentration of Iodine Pollutant',
    units: 'Bq s m-3',
    abbrev: 'TIACIP',
  },
  8: {
    parameter: 'Time Integrated Air Concentration of Radioactive Pollutant',
    units: 'Bq s m-3',
    abbrev: 'TIACRP',
  },
  9: { parameter: 'Reserved', units: '', abbrev: 'Reserved' },
  10: { parameter: 'Air Concentration', units: 'Bq m-3', abbrev: 'AIRCON' },
  11: { parameter: 'Wet Deposition', units: 'Bq m-2', abbrev: 'WETDEP' },
  12: { parameter: 'Dry Deposition', units: 'Bq m-2', abbrev: 'DRYDEP' },
  13: { parameter: 'Total Deposition (Wet + Dry)', units: 'Bq m-2', abbrev: 'TOTLWD' },
  14: { parameter: 'Specific Activity Concentration', units: 'Bq kg-1', abbrev: 'SACON' },
  15: { parameter: 'Maximum of Air Concentration in Layer', units: 'Bq m-3', abbrev: 'MAXACON' },
  16: { parameter: 'Height of Maximum of Air Concentration', units: 'm', abbrev: 'HMXACON' },
  17: { parameter: 'Column-Integrated Air Concentration', units: 'Bq m-2', abbrev: 'CIAIRC' },
  18: { parameter: 'Column-Averaged Air Concentration in Layer', units: 'Bq m-3', abbrev: 'CAACL' },
  19: { parameter: 'Deposition activity arrival', units: 's', abbrev: 'DEPACTA' },
  20: { parameter: 'Deposition activity ended', units: 's', abbrev: 'DEPACTE' },
  21: { parameter: 'Cloud activity arrival', units: 's', abbrev: 'CLDACTA' },
  22: { parameter: 'Cloud activity ended', units: 's', abbrev: 'CLDACTE' },
  23: { parameter: 'Effective dose rate', units: 'nSv h-1', abbrev: 'EFFDOSER' },
  24: { parameter: 'Thyroid dose rate (adult)', units: 'nSv h-1', abbrev: 'THYDOSER' },
  25: { parameter: 'Gamma dose rate (adult)', units: 'nSv h-1', abbrev: 'GAMDOSER' },
  26: { parameter: 'Activity emission', units: 'Bq s-1', abbrev: 'ACTEMM' },
  // 27-191 Reserved
  192: { parameter: 'Lightning', units: 'non-dim', abbrev: 'LTNG' },
  255: { parameter: 'Missing', units: '', abbrev: 'Missing' },
};

/**
 * GRIB2 - TABLE 4.2-0-19
 * PARAMETERS FOR DISCIPLINE 0 - CATEGORY 19
 * (Meteorological products, Physical Atmospheric category)
 * In Section 0, Octet 7 = 0
 * In Section 4, Octet 10 = 19
 * Revised 12/07/2023
 * Red text depicts changes made since 01/19/2022
 *
 * **Notes:**
 * 1. Parameter deprecated - See Regulation 92.6.2 and use another parameter instead.
 * 2. Supercooled large droplets (SLD) are defined as those with a diameter greater than 50 microns.
 * 3. Eddy Dissipation parameter is third root of eddy dissipation rate [m2 s-3].
 * 4. In astronomy, sky transparency means the effect on the viewing experience caused by the scattering of light through atmospheric water vapour, aerosols or other constituents. Ideal transparency conditions produce a black night sky conducive to viewing faint astronomical objects, almost like being in outer space. In poor transparency conditions, which may occur even in cloud-free conditions, the deep sky background is greyish (not black), faint details are washed out and contrast is reduced.
 * 5. Seeing means the steadiness or turbulence of the atmosphere in the context of astronomical observation. Turbulence causes rapid random fluctuations of the optical path through the atmosphere. The twinkling of stars, for example, occurs in poor seeing conditions.
 * 6. A duct layer is an atmospheric layer with a refractivity which leads to a trapping of electromagnetic waves. In a trapping layer the refractivity leads to a bending of EM waves, which is stronger than the Earth's curvature.
 * @see [GRIB2 - Table 4.2-0-19: Physical Atmospheric](https://www.example.com)
 */
export const grib2LookupTable42_019: Record<number, TableCategory> = {
  0: { parameter: 'Visibility', units: 'm', abbrev: 'VIS' },
  1: { parameter: 'Albedo', units: '%', abbrev: 'ALBDO' },
  2: { parameter: 'Thunderstorm Probability', units: '%', abbrev: 'TSTM' },
  3: { parameter: 'Mixed Layer Depth', units: 'm', abbrev: 'MIXHT' },
  4: { parameter: 'Volcanic Ash', units: 'See Table 4.206', abbrev: 'VOLASH' },
  5: { parameter: 'Icing Top', units: 'm', abbrev: 'ICIT' },
  6: { parameter: 'Icing Base', units: 'm', abbrev: 'ICIB' },
  7: { parameter: 'Icing', units: 'See Table 4.207', abbrev: 'ICI' },
  8: { parameter: 'Turbulence Top', units: 'm', abbrev: 'TURBT' },
  9: { parameter: 'Turbulence Base', units: 'm', abbrev: 'TURBB' },
  10: { parameter: 'Turbulence', units: 'See Table 4.208', abbrev: 'TURB' },
  11: { parameter: 'Turbulent Kinetic Energy', units: 'J kg-1', abbrev: 'TKE' },
  12: { parameter: 'Planetary Boundary Layer Regime', units: 'See Table 4.209', abbrev: 'PBLREG' },
  13: { parameter: 'Contrail Intensity', units: 'See Table 4.210', abbrev: 'CONTI' },
  14: { parameter: 'Contrail Engine Type', units: 'See Table 4.211', abbrev: 'CONTET' },
  15: { parameter: 'Contrail Top', units: 'm', abbrev: 'CONTT' },
  16: { parameter: 'Contrail Base', units: 'm', abbrev: 'CONTB' },
  17: { parameter: 'Maximum Snow Albedo', units: '%', abbrev: 'MXSALB' },
  18: { parameter: 'Snow-Free Albedo', units: '%', abbrev: 'SNFALB' },
  19: { parameter: 'Snow Albedo', units: '%', abbrev: 'SALBD' },
  20: { parameter: 'Icing', units: '%', abbrev: 'ICIP' },
  21: { parameter: 'In-Cloud Turbulence', units: '%', abbrev: 'CTP' },
  22: { parameter: 'Clear Air Turbulence (CAT)', units: '%', abbrev: 'CAT' },
  23: { parameter: 'Supercooled Large Droplet Probability', units: '%', abbrev: 'SLDP' },
  24: { parameter: 'Convective Turbulent Kinetic Energy', units: 'J kg-1', abbrev: 'CONTKE' },
  25: { parameter: 'Weather', units: 'See Table 4.225', abbrev: 'WIWW' },
  26: { parameter: 'Convective Outlook', units: 'See Table 4.224', abbrev: 'CONVO' },
  27: { parameter: 'Icing Scenario', units: 'See Table 4.227', abbrev: 'ICESC' },
  28: {
    parameter: 'Mountain Wave Turbulence (Eddy Dissipation Rate)',
    units: 'm2/3 s-1',
    abbrev: 'MWTURB',
  },
  29: {
    parameter: 'Clear Air Turbulence (CAT) (Eddy Dissipation Rate)',
    units: 'm2/3 s-1',
    abbrev: 'CATEDR',
  },
  30: { parameter: 'Eddy Dissipation Parameter', units: 'm2/3 s-1', abbrev: 'EDPARM' },
  31: {
    parameter: 'Maximum of Eddy Dissipation Parameter in Layer',
    units: 'm2/3 s-1',
    abbrev: 'MXEDPRM',
  },
  32: { parameter: 'Highest Freezing Level', units: 'm', abbrev: 'HIFREL' },
  33: { parameter: 'Visibility Through Liquid Fog', units: 'm', abbrev: 'VISLFOG' },
  34: { parameter: 'Visibility Through Ice Fog', units: 'm', abbrev: 'VISIFOG' },
  35: { parameter: 'Visibility Through Blowing Snow', units: 'm', abbrev: 'VISBSN' },
  36: { parameter: 'Presence of Snow Squalls', units: 'See Table 4.222', abbrev: 'PSNOWS' },
  37: { parameter: 'Icing Severity', units: 'See Table 4.228', abbrev: 'ICESEV' },
  38: { parameter: 'Sky transparency index', units: 'See Table 4.214', abbrev: 'SKYIDX' },
  39: { parameter: 'Seeing index', units: 'See Table 4.214', abbrev: 'SEEINDEX' },
  40: { parameter: 'Snow level', units: 'm', abbrev: 'SNOWLVL' },
  41: { parameter: 'Duct base height', units: 'm', abbrev: 'DBHEIGHT' },
  42: { parameter: 'Trapping layer base height', units: 'm', abbrev: 'TLBHEIGHT' },
  43: { parameter: 'Trapping layer top height', units: 'm', abbrev: 'TLTHEIGHT' },
  44: {
    parameter: 'Mean vertical gradient of refractivity inside trapping layer',
    units: 'm-1',
    abbrev: 'MEANVGRTL',
  },
  45: {
    parameter: 'Minimum vertical gradient of refractivity inside trapping layer',
    units: 'm-1',
    abbrev: 'MINVGRTL',
  },
  46: { parameter: 'Net radiation flux', units: 'W m-2', abbrev: 'NETRADFLUX' },
  47: { parameter: 'Global irradiance on tilted surfaces', units: 'W m-2', abbrev: 'GLIRRTS' },
  48: { parameter: 'Top of persistent contrails', units: 'm', abbrev: 'PCONTT' },
  49: { parameter: 'Base of persistent contrails', units: 'm', abbrev: 'PCONTB' },
  50: {
    parameter: 'Convectively-induced turbulence (CIT) (eddy dissipation rate)',
    units: 'm2/3 s-1',
    abbrev: 'CITEDR',
  },
  // 51-191 Reserved
  192: { parameter: 'Maximum Snow Albedo', units: '%', abbrev: 'MXSALB' },
  193: { parameter: 'Snow-Free Albedo', units: '%', abbrev: 'SNFALB' },
  194: { parameter: 'Slight risk convective outlook', units: 'categorical', abbrev: 'SRCONO' },
  195: { parameter: 'Moderate risk convective outlook', units: 'categorical', abbrev: 'MRCONO' },
  196: { parameter: 'High risk convective outlook', units: 'categorical', abbrev: 'HRCONO' },
  197: { parameter: 'Tornado probability', units: '%', abbrev: 'TORPROB' },
  198: { parameter: 'Hail probability', units: '%', abbrev: 'HAILPROB' },
  199: { parameter: 'Wind probability', units: '%', abbrev: 'WINDPROB' },
  200: { parameter: 'Significant Tornado probability', units: '%', abbrev: 'STORPROB' },
  201: { parameter: 'Significant Hail probability', units: '%', abbrev: 'SHAILPRO' },
  202: { parameter: 'Significant Wind probability', units: '%', abbrev: 'SWINDPRO' },
  203: { parameter: 'Categorical Thunderstorm', units: 'Code table 4.222', abbrev: 'TSTMC' },
  204: { parameter: 'Number of mixed layers next to surface', units: 'integer', abbrev: 'MIXLY' },
  205: { parameter: 'Flight Category', units: '', abbrev: 'FLGHT' },
  206: { parameter: 'Confidence - Ceiling', units: '', abbrev: 'CICEL' },
  207: { parameter: 'Confidence - Visibility', units: '', abbrev: 'CIVIS' },
  208: { parameter: 'Confidence - Flight Category', units: '', abbrev: 'CIFLT' },
  209: { parameter: 'Low-Level aviation interest', units: '', abbrev: 'LAVNI' },
  210: { parameter: 'High-Level aviation interest', units: '', abbrev: 'HAVNI' },
  211: { parameter: 'Visible, Black Sky Albedo', units: '%', abbrev: 'SBSALB' },
  212: { parameter: 'Visible, White Sky Albedo', units: '%', abbrev: 'SWSALB' },
  213: { parameter: 'Near IR, Black Sky Albedo', units: '%', abbrev: 'NBSALB' },
  214: { parameter: 'Near IR, White Sky Albedo', units: '%', abbrev: 'NWSALB' },
  215: {
    parameter: 'Total Probability of Severe Thunderstorms (Days 2,3)',
    units: '%',
    abbrev: 'PRSVR',
  },
  216: {
    parameter: 'Total Probability of Extreme Severe Thunderstorms (Days 2,3)',
    units: '%',
    abbrev: 'PRSIGSVR',
  },
  217: {
    parameter: 'Supercooled Large Droplet (SLD) Icing',
    units: 'See Table 4.207',
    abbrev: 'SIPD',
  },
  218: { parameter: 'Radiative emissivity', units: '', abbrev: 'EPSR' },
  219: { parameter: 'Turbulence Potential Forecast Index', units: '', abbrev: 'TPFI' },
  220: { parameter: 'Categorical Severe Thunderstorm', units: 'Code table 4.222', abbrev: 'SVRTS' },
  221: { parameter: 'Probability of Convection', units: '%', abbrev: 'PROCON' },
  222: { parameter: 'Convection Potential', units: 'Code table 4.222', abbrev: 'CONVP' },
  // 223-231 Reserved
  232: {
    parameter: 'Volcanic Ash Forecast Transport and Dispersion',
    units: 'log10 (kg m-3)',
    abbrev: 'VAFTD',
  },
  233: { parameter: 'Icing probability', units: 'non-dim', abbrev: 'ICPRB' },
  234: { parameter: 'Icing Severity', units: 'non-dim', abbrev: 'ICSEV' },
  235: { parameter: 'Joint Fire Weather Probability', units: '%', abbrev: 'JFWPRB' },
  236: { parameter: 'Snow Level', units: 'm', abbrev: 'SNOWLVL' },
  237: { parameter: 'Dry Thunderstorm Probability', units: '%', abbrev: 'DRYTPROB' },
  238: { parameter: 'Ellrod Index', units: '', abbrev: 'ELLINX' },
  239: {
    parameter: 'Craven-Wiedenfeld Aggregate Severe Parameter',
    units: 'Numeric',
    abbrev: 'CWASP',
  },
  240: { parameter: 'Continuous Icing Severity', units: 'non-dim', abbrev: 'ICESEVCON' },
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-20
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 20
 * **(Meteorological products, Atmospheric Chemical Constituents category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 20 (Atmospheric Chemical Constituents)
 * - **Section**: 4
 * - **Octet 10**: 20
 * - **Revised**: 11/02/2023
 *
 * **Reserved Ranges**:
 * - `19-49`: Reserved
 * - `82-99`: Reserved
 * - `113-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2-0-20.shtml)
 *
 * ## Notes
 * 1. First Fixed Surface and Second Fixed Surface of Code Table 4.5 define the vertical extent.
 * 2. The term "Number Density" is synonymous with "Number Concentration" (Code 59).
 * 3. Net source represents the sum of all atmospheric processes creating and destroying constituents or aerosols.
 * 4. Use Snow melt rate instead for certain processes (Discipline 2, Category 0, Number 41).
 */
export const grib2LookupTable42_020: Record<number, TableCategory> = {
  0: { parameter: 'Mass Density (Concentration)', units: 'kg m-3', abbrev: 'MASSDEN' },
  1: { parameter: 'Column-Integrated Mass Density', units: 'kg m-2', abbrev: 'COLMD' },
  2: { parameter: 'Mass Mixing Ratio (Mass Fraction in Air)', units: 'kg kg-1', abbrev: 'MASSMR' },
  3: { parameter: 'Atmosphere Emission Mass Flux', units: 'kg m-2s-1', abbrev: 'AEMFLX' },
  4: { parameter: 'Atmosphere Net Production Mass Flux', units: 'kg m-2s-1', abbrev: 'ANPMFLX' },
  5: {
    parameter: 'Atmosphere Net Production And Emission Mass Flux',
    units: 'kg m-2s-1',
    abbrev: 'ANPEMFLX',
  },
  6: { parameter: 'Surface Dry Deposition Mass Flux', units: 'kg m-2s-1', abbrev: 'SDDMFLX' },
  7: { parameter: 'Surface Wet Deposition Mass Flux', units: 'kg m-2s-1', abbrev: 'SWDMFLX' },
  8: { parameter: 'Atmosphere Re-Emission Mass Flux', units: 'kg m-2s-1', abbrev: 'AREMFLX' },
  9: {
    parameter: 'Wet Deposition by Large-Scale Precipitation Mass Flux',
    units: 'kg m-2s-1',
    abbrev: 'WLSMFLX',
  },
  10: {
    parameter: 'Wet Deposition by Convective Precipitation Mass Flux',
    units: 'kg m-2s-1',
    abbrev: 'WDCPMFLX',
  },
  11: { parameter: 'Sedimentation Mass Flux', units: 'kg m-2s-1', abbrev: 'SEDMFLX' },
  12: { parameter: 'Dry Deposition Mass Flux', units: 'kg m-2s-1', abbrev: 'DDMFLX' },
  13: {
    parameter: 'Transfer From Hydrophobic to Hydrophilic',
    units: 'kg kg-1s-1',
    abbrev: 'TRANHH',
  },
  14: { parameter: 'Transfer From SO2 to SO4', units: 'kg kg-1s-1', abbrev: 'TRSDS' },
  15: { parameter: 'Dry deposition velocity', units: 'm s-1', abbrev: 'DDVEL' },
  16: {
    parameter: 'Mass mixing ratio with respect to dry air',
    units: 'kg kg-1',
    abbrev: 'MSSRDRYA',
  },
  17: {
    parameter: 'Mass mixing ratio with respect to wet air',
    units: 'kg kg-1',
    abbrev: 'MSSRWETA',
  },
  18: { parameter: 'Potential of hydrogen (pH)', units: 'pH', abbrev: 'POTHPH' },
  // 19-49: Reserved
  50: { parameter: 'Amount in Atmosphere', units: 'mol', abbrev: 'AIA' },
  51: { parameter: 'Concentration In Air', units: 'mol m-3', abbrev: 'CONAIR' },
  52: { parameter: 'Volume Mixing Ratio', units: 'mol mol-1', abbrev: 'VMXR' },
  53: {
    parameter: 'Chemical Gross Production Rate of Concentration',
    units: 'mol m-3s-1',
    abbrev: 'CGPRC',
  },
  54: {
    parameter: 'Chemical Gross Destruction Rate of Concentration',
    units: 'mol m-3s-1',
    abbrev: 'CGDRC',
  },
  55: { parameter: 'Surface Flux', units: 'mol m-2s-1', abbrev: 'SFLUX' },
  56: { parameter: 'Changes Of Amount in Atmosphere', units: 'mol s-1', abbrev: 'COAIA' },
  57: { parameter: 'Total Yearly Average Burden of The Atmosphere', units: 'mol', abbrev: 'TYABA' },
  58: { parameter: 'Total Yearly Average Atmospheric Loss', units: 'mol s-1', abbrev: 'TYAAL' },
  59: { parameter: 'Aerosol Number Concentration', units: 'm-3', abbrev: 'ANCON' },
  60: { parameter: 'Aerosol Specific Number Concentration', units: 'kg-1', abbrev: 'ASNCON' },
  61: { parameter: 'Maximum of Mass Density', units: 'kg m-3', abbrev: 'MXMASSD' },
  62: { parameter: 'Height of Mass Density', units: 'm', abbrev: 'HGTMD' },
  63: { parameter: 'Column-Averaged Mass Density in Layer', units: 'kg m-3', abbrev: 'CAVEMDL' },
  64: {
    parameter: 'Mole fraction with respect to dry air',
    units: 'mol mol-1',
    abbrev: 'MOLRDRYA',
  },
  65: {
    parameter: 'Mole fraction with respect to wet air',
    units: 'mol mol-1',
    abbrev: 'MOLRWETA',
  },
  66: {
    parameter: 'Column-integrated in-cloud scavenging rate by precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CINCLDSP',
  },
  67: {
    parameter: 'Column-integrated below-cloud scavenging rate by precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CBLCLDSP',
  },
  68: {
    parameter: 'Column-integrated release rate from evaporating precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CIRELREP',
  },
  69: {
    parameter: 'Column-integrated in-cloud scavenging rate by large-scale precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CINCSLSP',
  },
  70: {
    parameter: 'Column-integrated below-cloud scavenging rate by large-scale precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CBECSLSP',
  },
  71: {
    parameter: 'Column-integrated release rate from evaporating large-scale precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CRERELSP',
  },
  72: {
    parameter: 'Column-integrated in-cloud scavenging rate by convective precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CINCSRCP',
  },
  73: {
    parameter: 'Column-integrated below-cloud scavenging rate by convective precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CBLCSRCP',
  },
  74: {
    parameter: 'Column-integrated release rate from evaporating convective precipitation',
    units: 'kg m-2 s-1',
    abbrev: 'CIRERECP',
  },
  75: { parameter: 'Wildfire flux', units: 'kg m-2 s-1', abbrev: 'WFIREFLX' },
  76: { parameter: 'Emission Rate', units: 'kg kg-1 s-1', abbrev: 'EMISFLX' },
  77: { parameter: 'Surface Emission flux', units: 'kg m-2 s-1', abbrev: 'SFCEFLX' },
  78: { parameter: 'Column integrated eastward mass flux', units: 'kg m-2 s-1', abbrev: 'CEMF' },
  79: { parameter: 'Column integrated northward mass flux', units: 'kg m-2 s-1', abbrev: 'CNMF' },
  80: {
    parameter: 'Column integrated divergence of mass flux',
    units: 'kg m-2 s-1',
    abbrev: 'CDIVMF',
  },
  81: { parameter: 'Column integrated net source', units: 'kg m-2 s-1', abbrev: 'CNETS' },
  // Reserved 82-99
  100: { parameter: 'Surface Area Density (Aerosol)', units: 'm-1', abbrev: 'SADEN' },
  101: { parameter: 'Vertical Visual Range', units: 'm', abbrev: 'ATMTK' },
  102: { parameter: 'Aerosol Optical Thickness', units: 'Numeric', abbrev: 'AOTK' },
  103: { parameter: 'Single Scattering Albedo', units: 'Numeric', abbrev: 'SSALBK' },
  104: { parameter: 'Asymmetry Factor', units: 'Numeric', abbrev: 'ASYSFK' },
  105: { parameter: 'Aerosol Extinction Coefficient', units: 'm-1', abbrev: 'AECOEF' },
  106: { parameter: 'Aerosol Absorption Coefficient', units: 'm-1', abbrev: 'AACOEF' },
  107: {
    parameter: 'Aerosol Lidar Backscatter from Satellite',
    units: 'm-1sr-1',
    abbrev: 'ALBSAT',
  },
  108: {
    parameter: 'Aerosol Lidar Backscatter from the Ground',
    units: 'm-1sr-1',
    abbrev: 'ALBGRD',
  },
  109: { parameter: 'Aerosol Lidar Extinction from Satellite', units: 'm-1', abbrev: 'ALESAT' },
  110: { parameter: 'Aerosol Lidar Extinction from the Ground', units: 'm-1', abbrev: 'ALEGRD' },
  111: { parameter: 'Angstrom Exponent', units: 'Numeric', abbrev: 'ANGSTEXP' },
  112: { parameter: 'Scattering Aerosol Optical Thickness', units: 'Numeric', abbrev: 'SCTAOTK' },
  // Reserved 113-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-21
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 21
 * **(Meteorological products, Thermodynamic Properties category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 21 (Thermodynamic Properties)
 * - **Section**: 4
 * - **Octet 10**: 21
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `23-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Total energy is the sum of internal energy, potential energy, kinetic energy, and latent heat. The same applies to energy fluxes.
 * 2. Water enthalpy (flux) is associated with the temperature of the water mass.
 * 3. Water potential energy flux is the flux of potential energy associated with the water mass.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2-0-21.shtml)
 */
export const grib2LookupTable42_021: Record<number, TableCategory> = {
  0: {
    parameter: 'Column Integrated Potential + Internal Energy',
    units: 'J m-2',
    abbrev: 'POTINTENG',
  },
  1: { parameter: 'Column Integrated Kinetic Energy', units: 'J m-2', abbrev: 'KINENG' },
  2: { parameter: 'Column Integrated Total Energy', units: 'J m-2', abbrev: 'TOTENG' },
  3: { parameter: 'Column Integrated Enthalpy', units: 'J m-2', abbrev: 'ENTHALPY' },
  4: { parameter: 'Column Integrated Water Enthalpy', units: 'J m-2', abbrev: 'WATENTHALPY' },
  5: {
    parameter: 'Column Integrated Eastward Enthalpy Flux',
    units: 'W m-1',
    abbrev: 'EASTENTFLUX',
  },
  6: {
    parameter: 'Column Integrated Northward Enthalpy Flux',
    units: 'W m-1',
    abbrev: 'NRTHENTFLUX',
  },
  7: {
    parameter: 'Column Integrated Eastward Potential Energy Flux',
    units: 'W m-1',
    abbrev: 'EASTPOTFLUX',
  },
  8: {
    parameter: 'Column Integrated Northward Potential Energy Flux',
    units: 'W m-1',
    abbrev: 'NRTHPOTFLUX',
  },
  9: {
    parameter: 'Column Integrated Eastward Kinetic Energy Flux',
    units: 'W m-1',
    abbrev: 'EASTKINFLUX',
  },
  10: {
    parameter: 'Column Integrated Northward Kinetic Energy Flux',
    units: 'W m-1',
    abbrev: 'NRTHKINFLUX',
  },
  11: {
    parameter: 'Column Integrated Eastward Total Energy Flux',
    units: 'W m-1',
    abbrev: 'EASTTOTFLUX',
  },
  12: {
    parameter: 'Column Integrated Northward Total Energy Flux',
    units: 'W m-1',
    abbrev: 'NRTHTOTFLUX',
  },
  13: {
    parameter: 'Divergence of Column Integrated Enthalpy Flux',
    units: 'W m-1',
    abbrev: 'DIVENTFLUX',
  },
  14: {
    parameter: 'Divergence of Column Integrated Potential Energy Flux',
    units: 'W m-1',
    abbrev: 'DIVPOTFLUX',
  },
  15: {
    parameter: 'Divergence of Column Integrated Water Potential Energy Flux',
    units: 'W m-1',
    abbrev: 'DIVWPOTFLUX',
  },
  16: {
    parameter: 'Divergence of Column Integrated Kinetic Energy Flux',
    units: 'W m-1',
    abbrev: 'DIVKENGFLUX',
  },
  17: {
    parameter: 'Divergence of Column Integrated Total Energy Flux',
    units: 'W m-1',
    abbrev: 'DIVTOTFLUX',
  },
  18: {
    parameter: 'Divergence of Column Integrated Water Enthalpy Flux',
    units: 'W m-1',
    abbrev: 'DIVWENTFLUX',
  },
  19: { parameter: 'Column Integrated Eastward Heat Flux', units: 'W m-1', abbrev: 'EASTHFLUX' },
  20: { parameter: 'Column Integrated Northward Heat Flux', units: 'W m-1', abbrev: 'NRTHHFLUX' },
  21: {
    parameter: 'Column Integrated Potential + Internal + Latent Energy',
    units: 'J m-2',
    abbrev: 'PILENERGY',
  },
  22: { parameter: 'Eady Growth Rate', units: 'day-1', abbrev: 'EADYGR' },
  // Reserved 23-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-22
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 22
 * **(Meteorological products, Drought Indices category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 22 (Drought Indices)
 * - **Section**: 4
 * - **Octet 10**: 22
 * - **Created**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `7-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Descriptions of the drought indices are available in the Handbook of Drought Indicators and Indices (WMO-No. 1173).
 * 2. Indices are calculated over different time ranges specified in templates 4.107, 4.108, 4.109, and 4.112.
 * 3. All standardized drought indices follow the Standardized Precipitation Index User Guide (WMO-No. 1090).
 *
 * ## Links
 * - [Handbook of Drought Indicators and Indices](https://library.wmo.int/idurl/4/55169)
 * - [Standardized Precipitation Index User Guide](https://library.wmo.int/idurl/4/39629)
 */
export const grib2LookupTable42_022: Record<number, TableCategory> = {
  0: { parameter: 'Standard Precipitation Index (SPI)', units: 'dimensionless', abbrev: 'SPI' },
  1: {
    parameter: 'Standardized Precipitation Evapotranspiration Index (SPEI)',
    units: 'dimensionless',
    abbrev: 'SPEI',
  },
  2: { parameter: 'Standardized Streamflow Index (SSFI)', units: 'dimensionless', abbrev: 'SSFI' },
  3: {
    parameter: 'Standardized Reservoir Supply Index (SRSI)',
    units: 'dimensionless',
    abbrev: 'SRSI',
  },
  4: { parameter: 'Standardized Water-level Index (SWI)', units: 'dimensionless', abbrev: 'SWI' },
  5: {
    parameter: 'Standardized Snowmelt and Rain Index (SMRI)',
    units: 'dimensionless',
    abbrev: 'SMRI',
  },
  6: { parameter: 'Streamflow Drought Index (SDI)', units: 'dimensionless', abbrev: 'SDI' },
  // Reserved 7-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-190
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 190
 * **(Meteorological products, ASCII IA5 String category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 190 (ASCII IA5 String)
 * - **Section**: 4
 * - **Octet 10**: 190
 * - **Revised**: 12/14/2011
 *
 * **Reserved Ranges**:
 * - `1-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. This table defines ASCII IA5 strings used for arbitrary text.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/)
 */
export const grib2LookupTable42_190: Record<number, TableCategory> = {
  0: { parameter: 'Arbitrary Text String', units: 'CCITTIA5', abbrev: 'ATEXT' },
  // Reserved 1-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-191
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 191
 * **(Meteorological products, Miscellaneous category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 191 (Miscellaneous)
 * - **Section**: 4
 * - **Octet 10**: 191
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `8-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Hurricane, tropical storm, and tropical depression tracks use spatiotemporal vicinity logic.
 */
export const grib2LookupTable42_191: Record<number, TableCategory> = {
  0: {
    parameter: 'Seconds prior to initial reference time (defined in Section 1)',
    units: 's',
    abbrev: 'TSEC',
  },
  1: { parameter: 'Geographical Latitude', units: '° N', abbrev: 'GEOLAT' },
  2: { parameter: 'Geographical Longitude', units: '° E', abbrev: 'GEOLON' },
  3: { parameter: 'Days Since Last Observation', units: 'd', abbrev: 'DSLOBS' },
  4: { parameter: 'Tropical cyclone density track', units: 'Numeric', abbrev: 'TCDTRACK' },
  5: {
    parameter: 'Hurricane track in spatiotemporal vicinity',
    units: 'boolean',
    abbrev: 'HURTSV',
  },
  6: {
    parameter: 'Tropical storm track in spatiotemporal vicinity',
    units: 'boolean',
    abbrev: 'TSTSV',
  },
  7: {
    parameter: 'Tropical depression track in spatiotemporal vicinity',
    units: 'boolean',
    abbrev: 'TDTSV',
  },
  // Reserved 8-191
  192: { parameter: 'Latitude (-90 to 90)', units: '°', abbrev: 'NLAT' },
  193: { parameter: 'East Longitude (0 to 360)', units: '°', abbrev: 'ELON' },
  194: { parameter: 'Seconds prior to initial reference time', units: 's', abbrev: 'RTSEC' },
  195: { parameter: 'Model Layer number (From bottom up)', units: '', abbrev: 'MLYNO' },
  196: { parameter: 'Latitude (nearest neighbor) (-90 to 90)', units: '°', abbrev: 'NLATN' },
  197: { parameter: 'East Longitude (nearest neighbor) (0 to 360)', units: '°', abbrev: 'ELONN' },
  // Reserved for Local Use 198-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-0-192
 * PARAMETERS FOR DISCIPLINE 0, CATEGORY 192
 * **(Meteorological products, Covariance category)**
 *
 * **Details**:
 * - **Discipline**: 0 (Meteorological products)
 * - **Category**: 192 (Covariance)
 * - **Section**: 4
 * - **Octet 10**: 192
 * - **Created**: 03/12/2008
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * Covariances are defined as `[XY]-[X][Y]`, where `[]` indicates the mean over the specified time span.
 */
export const grib2LookupTable42_192: Record<number, TableCategory> = {
  1: {
    parameter: 'Covariance between zonal and meridional components of the wind',
    units: 'm2/s2',
    abbrev: 'COVMZ',
  },
  2: {
    parameter: 'Covariance between zonal component of the wind and temperature',
    units: 'K*m/s',
    abbrev: 'COVTZ',
  },
  3: {
    parameter: 'Covariance between meridional component of the wind and temperature',
    units: 'K*m/s',
    abbrev: 'COVTM',
  },
  4: {
    parameter: 'Covariance between temperature and vertical component of the wind',
    units: 'K*m/s',
    abbrev: 'COVTW',
  },
  5: {
    parameter: 'Covariance between zonal and zonal components of the wind',
    units: 'm2/s2',
    abbrev: 'COVZZ',
  },
  6: {
    parameter: 'Covariance between meridional and meridional components of the wind',
    units: 'm2/s2',
    abbrev: 'COVMM',
  },
  7: {
    parameter: 'Covariance between specific humidity and zonal components of the wind',
    units: 'kg/kg*m/s',
    abbrev: 'COVQZ',
  },
  8: {
    parameter: 'Covariance between specific humidity and meridional components of the wind',
    units: 'kg/kg*m/s',
    abbrev: 'COVQM',
  },
  9: {
    parameter: 'Covariance between temperature and vertical components of the wind',
    units: 'K*Pa/s',
    abbrev: 'COVTVV',
  },
  10: {
    parameter: 'Covariance between specific humidity and vertical components of the wind',
    units: 'kg/kg*Pa/s',
    abbrev: 'COVQVV',
  },
  11: {
    parameter: 'Covariance between surface pressure and surface pressure',
    units: 'Pa*Pa',
    abbrev: 'COVPSPS',
  },
  12: {
    parameter: 'Covariance between specific humidity and specific humidity',
    units: 'kg/kg*kg/kg',
    abbrev: 'COVQQ',
  },
  13: {
    parameter: 'Covariance between vertical and vertical components of the wind',
    units: 'Pa2/s2',
    abbrev: 'COVVVVV',
  },
  14: {
    parameter: 'Covariance between temperature and temperature',
    units: 'K*K',
    abbrev: 'COVTT',
  },
  255: {
    parameter: 'Missing',
    units: '',
    abbrev: '',
  },
};

/**
 * # GRIB2 - TABLE 4.2-1-0
 * PARAMETERS FOR DISCIPLINE 1, CATEGORY 0
 * **(Hydrological products, Hydrology Basic category)**
 *
 * **Details**:
 * - **Discipline**: 1 (Hydrological products)
 * - **Category**: 0 (Hydrology Basic)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `21-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Remotely sensed snow cover uses dimensionless thematic values (e.g., 50: no-snow/no-cloud, 100: clouds, 250: snow). See Table 4.215.
 * 2. Snow coverage by elevation portrays elevations with snow packs; see Table 4.216.
 * 3. Snow water equivalent percent of normal is stored in percent units (e.g., 110 = 110% of normal snow water equivalent).
 */
export const grib2LookupTable42_10: Record<number, TableCategory> = {
  0: {
    parameter:
      'Flash Flood Guidance (Encoded as an accumulation over a floating subinterval of time between the reference time and valid time)',
    units: 'kg m-2',
    abbrev: 'FFLDG',
  },
  1: {
    parameter:
      'Flash Flood Runoff (Encoded as an accumulation over a floating subinterval of time)',
    units: 'kg m-2',
    abbrev: 'FFLDRO',
  },
  2: {
    parameter: 'Remotely Sensed Snow Cover',
    units: 'See Table 4.215',
    abbrev: 'RSSC',
  },
  3: {
    parameter: 'Elevation of Snow Covered Terrain',
    units: 'See Table 4.216',
    abbrev: 'ESCT',
  },
  4: {
    parameter: 'Snow Water Equivalent Percent of Normal',
    units: '%',
    abbrev: 'SWEPON',
  },
  5: {
    parameter: 'Baseflow-Groundwater Runoff',
    units: 'kg m-2',
    abbrev: 'BGRUN',
  },
  6: {
    parameter: 'Storm Surface Runoff',
    units: 'kg m-2',
    abbrev: 'SSRUN',
  },
  7: {
    parameter: 'Discharge from Rivers or Streams',
    units: 'm3 s-1',
    abbrev: 'DISRS',
  },
  8: {
    parameter: 'Group Water Upper Storage',
    units: 'kg m-2',
    abbrev: 'GWUPS',
  },
  9: {
    parameter: 'Group Water Lower Storage',
    units: 'kg m-2',
    abbrev: 'GWLOWS',
  },
  10: {
    parameter: 'Side Flow into River Channel',
    units: 'm3 s-1 m-1',
    abbrev: 'SFLORC',
  },
  11: {
    parameter: 'River Storage of Water',
    units: 'm3',
    abbrev: 'RVERSW',
  },
  12: {
    parameter: 'Flood Plain Storage of Water',
    units: 'm3',
    abbrev: 'FLDPSW',
  },
  13: {
    parameter: 'Depth of Water on Soil Surface',
    units: 'kg m-2',
    abbrev: 'DEPWSS',
  },
  14: {
    parameter: 'Upstream Accumulated Precipitation',
    units: 'kg m-2',
    abbrev: 'UPAPCP',
  },
  15: {
    parameter: 'Upstream Accumulated Snow Melt',
    units: 'kg m-2',
    abbrev: 'UPASM',
  },
  16: {
    parameter: 'Percolation Rate',
    units: 'kg m-2 s-1',
    abbrev: 'PERRATE',
  },
  17: {
    parameter: 'River Outflow of Water',
    units: 'm3 s-1',
    abbrev: 'RVEROW',
  },
  18: {
    parameter: 'Floodplain Outflow of Water',
    units: 'm3 s-1',
    abbrev: 'FLDPOW',
  },
  19: {
    parameter: 'Floodpath Outflow of Water',
    units: 'm3 s-1',
    abbrev: 'FLDPATHOW',
  },
  20: {
    parameter: 'Water on Surface',
    units: 'kg m-2',
    abbrev: 'WATSURF',
  },
  // Reserved 21-191
  192: {
    parameter: 'Baseflow-Groundwater Runoff',
    units: 'kg m-2',
    abbrev: 'BGRUN',
  },
  193: {
    parameter: 'Storm Surface Runoff',
    units: 'kg m-2',
    abbrev: 'SSRUN',
  },
  // Reserved for Local Use 194-254
  255: {
    parameter: 'Missing',
    units: '',
    abbrev: '',
  },
};

/**
 * # GRIB2 - TABLE 4.2-1-1
 * PARAMETERS FOR DISCIPLINE 1, CATEGORY 1
 * **(Hydrological products, Hydrology Probabilities category)**
 *
 * **Details**:
 * - **Discipline**: 1 (Hydrological products)
 * - **Category**: 1 (Hydrology Probabilities)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Revised**: 11/02/2023
 *
 * **Reserved Ranges**:
 * - `3-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_11: Record<number, TableCategory> = {
  0: {
    parameter:
      'Conditional percent precipitation amount fractile for an overall period (encoded as an accumulation)',
    units: 'kg m-2',
    abbrev: 'CPPOP',
  },
  1: {
    parameter:
      'Percent Precipitation in a sub-period of an overall period (encoded as a percent accumulation over the sub-period)',
    units: '%',
    abbrev: 'PPOSP',
  },
  2: {
    parameter: 'Probability of 0.01 inch of precipitation (POP)',
    units: '%',
    abbrev: 'POP',
  },
  // Reserved 3-191
  192: {
    parameter: 'Probability of Freezing Precipitation',
    units: '%',
    abbrev: 'CPOZP',
  },
  193: {
    parameter: 'Percent of Frozen Precipitation',
    units: '%',
    abbrev: 'CPOFP',
  },
  194: {
    parameter: 'Probability of precipitation exceeding flash flood guidance values',
    units: '%',
    abbrev: 'PPFFG',
  },
  195: {
    parameter: 'Probability of Wetting Rain, exceeding in 0.10" in a given time period',
    units: '%',
    abbrev: 'CWR',
  },
  196: {
    parameter: 'Binary Probability of precipitation exceeding average recurrence intervals (ARI)',
    units: 'see Code table 4.222',
    abbrev: 'QPFARI',
  },
  197: {
    parameter: 'Binary Probability of precipitation exceeding flash flood guidance values',
    units: 'see Code table 4.222',
    abbrev: 'QPFFFG',
  },
  255: {
    parameter: 'Missing',
    units: '',
    abbrev: '',
  },
};

/**
 * # GRIB2 - TABLE 4.2-1-2
 * PARAMETERS FOR DISCIPLINE 1, CATEGORY 2
 * **(Hydrological products, Inland water and sediment properties category)**
 *
 * **Details**:
 * - **Discipline**: 1 (Hydrological products)
 * - **Category**: 2 (Inland water and sediment properties)
 * - **Section**: 4
 * - **Octet 10**: 2
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `24-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. The same parameter name may exist in multiple categories based on its intended use. For example, "Water Temperature" in this table applies to freshwater lakes and rivers, unlike its counterpart in oceanographic products.
 */
export const grib2LookupTable42_12: Record<number, TableCategory> = {
  0: { parameter: 'Water Depth', units: 'm', abbrev: 'WDPTHIL' },
  1: { parameter: 'Water Temperature', units: 'K', abbrev: 'WTMPIL' },
  2: { parameter: 'Water Fraction', units: 'Proportion', abbrev: 'WFRACT' },
  3: { parameter: 'Sediment Thickness', units: 'm', abbrev: 'SEDTK' },
  4: { parameter: 'Sediment Temperature', units: 'K', abbrev: 'SEDTMP' },
  5: { parameter: 'Ice Thickness', units: 'm', abbrev: 'ICTKIL' },
  6: { parameter: 'Ice Temperature', units: 'K', abbrev: 'ICETIL' },
  7: { parameter: 'Ice Cover', units: 'Proportion', abbrev: 'ICECIL' },
  8: { parameter: 'Land Cover (0=water, 1=land)', units: 'Proportion', abbrev: 'LANDIL' },
  9: { parameter: 'Shape Factor with Respect to Salinity Profile', units: '', abbrev: 'SFSAL' },
  10: {
    parameter: 'Shape Factor with Respect to Temperature Profile in Thermocline',
    units: '',
    abbrev: 'SFTMP',
  },
  11: {
    parameter: 'Attenuation Coefficient of Water with Respect to Solar Radiation',
    units: 'm-1',
    abbrev: 'ACWSR',
  },
  12: { parameter: 'Salinity', units: 'kg kg-1', abbrev: 'SALTIL' },
  13: { parameter: 'Cross Sectional Area of Flow in Channel', units: 'm2', abbrev: 'CSAFC' },
  14: { parameter: 'Snow Temperature', units: 'K', abbrev: 'LNDSNOWT' },
  15: { parameter: 'Lake Depth', units: 'm', abbrev: 'LDEPTH' },
  16: { parameter: 'River Depth', units: 'm', abbrev: 'RDEPTH' },
  17: { parameter: 'Floodplain Depth', units: 'm', abbrev: 'FLDPDEPTH' },
  18: { parameter: 'Floodplain Flooded Fraction', units: 'Proportion', abbrev: 'FLDPFLFR' },
  19: { parameter: 'Floodplain Flooded Area', units: 'm2', abbrev: 'FLDPFLAR' },
  20: { parameter: 'River Fraction', units: 'Proportion', abbrev: 'RVERFR' },
  21: { parameter: 'River Area', units: 'm2', abbrev: 'RVERAR' },
  22: {
    parameter: 'Fraction of River Coverage Plus River Related Flooding',
    units: 'Proportion',
    abbrev: 'FRCRF',
  },
  23: {
    parameter: 'Area of River Coverage Plus River Related Flooding',
    units: 'm2',
    abbrev: 'ARCRF',
  },
  // Reserved 24-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-0
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 0
 * **(Land Surface products, Vegetation/Biomass category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 0 (Vegetation/Biomass)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `64-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Deprecated parameters are marked and alternative parameters are recommended (e.g., categories for Soil Products).
 * 2. Statistical process 1 (Accumulation) does not alter units.
 * 3. C4 plants use a specific photosynthesis mechanism to avoid photorespiration.
 * 4. Net ecosystem fluxes can specify chemical species (e.g., CO₂ or CH₄) with chemical constituent templates.
 */
export const grib2LookupTable42_20: Record<number, TableCategory> = {
  0: { parameter: 'Land Cover (0=sea, 1=land)', units: 'Proportion', abbrev: 'LAND' },
  1: { parameter: 'Surface Roughness', units: 'm', abbrev: 'SFCR' },
  2: { parameter: 'Soil Temperature (Parameter Deprecated)', units: 'K', abbrev: 'TSOIL' },
  3: { parameter: 'Soil Moisture Content (Parameter Deprecated)', units: '', abbrev: '' },
  4: { parameter: 'Vegetation', units: '%', abbrev: 'VEG' },
  5: { parameter: 'Water Runoff', units: 'kg m-2', abbrev: 'WATR' },
  6: { parameter: 'Evapotranspiration', units: 'kg-2 s-1', abbrev: 'EVAPT' },
  7: { parameter: 'Model Terrain Height', units: 'm', abbrev: 'MTERH' },
  8: { parameter: 'Land Use', units: 'See Table 4.212', abbrev: 'LANDU' },
  9: { parameter: 'Volumetric Soil Moisture Content', units: 'Proportion', abbrev: 'SOILW' },
  10: { parameter: 'Ground Heat Flux', units: 'W m-2', abbrev: 'GFLUX' },
  11: { parameter: 'Moisture Availability', units: '%', abbrev: 'MSTAV' },
  12: { parameter: 'Exchange Coefficient', units: 'kg m-2 s-1', abbrev: 'SFEXC' },
  13: { parameter: 'Plant Canopy Surface Water', units: 'kg m-2', abbrev: 'CNWAT' },
  14: { parameter: "Blackadar's Mixing Length Scale", units: 'm', abbrev: 'BMIXL' },
  15: { parameter: 'Canopy Conductance', units: 'm s-1', abbrev: 'CCOND' },
  16: { parameter: 'Minimal Stomatal Resistance', units: 's m-1', abbrev: 'RSMIN' },
  17: { parameter: 'Wilting Point (Parameter Deprecated)', units: 'Proportion', abbrev: 'WILT' },
  18: { parameter: 'Solar parameter in canopy conductance', units: 'Proportion', abbrev: 'RCS' },
  19: { parameter: 'Temperature parameter in canopy', units: 'Proportion', abbrev: 'RCT' },
  20: { parameter: 'Humidity parameter in canopy conductance', units: 'Proportion', abbrev: 'RCQ' },
  21: {
    parameter: 'Soil moisture parameter in canopy conductance',
    units: 'Proportion',
    abbrev: 'RCSOL',
  },
  22: { parameter: 'Soil Moisture (Parameter Deprecated)', units: '', abbrev: '' },
  23: {
    parameter: 'Column-Integrated Soil Water (Parameter Deprecated)',
    units: 'kg m-2',
    abbrev: 'CISOILW',
  },
  24: { parameter: 'Heat Flux', units: 'W m-2', abbrev: 'HFLUX' },
  25: { parameter: 'Volumetric Soil Moisture', units: 'm3 m-3', abbrev: 'VSOILM' },
  26: { parameter: 'Wilting Point', units: 'kg m-3', abbrev: 'WILT' },
  27: { parameter: 'Volumetric Wilting Point', units: 'm3 m-3', abbrev: 'VWILTP' },
  28: { parameter: 'Leaf Area Index', units: 'Numeric', abbrev: 'LEAINX' },
  29: { parameter: 'Evergreen Forest Cover', units: 'Proportion', abbrev: 'EVGFC' },
  30: { parameter: 'Deciduous Forest Cover', units: 'Proportion', abbrev: 'DECFC' },
  31: {
    parameter: 'Normalized Differential Vegetation Index (NDVI)',
    units: 'Numeric',
    abbrev: 'NDVINX',
  },
  32: { parameter: 'Root Depth of Vegetation', units: 'm', abbrev: 'RDVEG' },
  33: { parameter: 'Water Runoff and Drainage', units: 'kg m-2', abbrev: 'WROD' },
  34: { parameter: 'Surface Water Runoff', units: 'kg m-2', abbrev: 'SFCWRO' },
  35: { parameter: 'Tile Class', units: 'See Table 4.243', abbrev: 'TCLASS' },
  36: { parameter: 'Tile Fraction', units: 'Proportion', abbrev: 'TFRCT' },
  37: { parameter: 'Tile Percentage', units: '%', abbrev: 'TPERCT' },
  38: {
    parameter: 'Soil Volumetric Ice Content (Water Equivalent)',
    units: 'm3 m-3',
    abbrev: 'SOILVIC',
  },
  39: { parameter: 'Evapotranspiration Rate', units: 'kg m-2 s-1', abbrev: 'EVAPTRAT' },
  40: { parameter: 'Potential Evapotranspiration Rate', units: 'kg m-2 s-1', abbrev: 'PEVAPTRAT' },
  41: { parameter: 'Snow Melt Rate', units: 'kg m-2 s-1', abbrev: 'SMRATE' },
  42: { parameter: 'Water Runoff and Drainage Rate', units: 'kg m-2 s-1', abbrev: 'WRDRATE' },
  43: { parameter: 'Drainage direction', units: 'See Table 4.250', abbrev: 'DRAINDIR' },
  44: { parameter: 'Upstream Area', units: 'm2', abbrev: 'UPSAREA' },
  45: { parameter: 'Wetland Cover', units: 'Proportion', abbrev: 'WETCOV' },
  46: { parameter: 'Wetland Type', units: 'See Table 4.239', abbrev: 'WETTYPE' },
  47: { parameter: 'Irrigation Cover', units: 'Proportion', abbrev: 'IRRCOV' },
  48: { parameter: 'C4 Crop Cover', units: 'Proportion', abbrev: 'CROPCOV' },
  49: { parameter: 'C4 Grass Cover', units: 'Proportion', abbrev: 'GRASSCOV' },
  50: { parameter: 'Skin Reservoir Content', units: 'kg m-2', abbrev: 'SKINRC' },
  51: { parameter: 'Surface Runoff Rate', units: 'kg m-2 s-1', abbrev: 'SURFRATE' },
  52: { parameter: 'Subsurface Runoff Rate', units: 'kg m-2 s-1', abbrev: 'SUBSRATE' },
  53: { parameter: 'Low-Vegetation Cover', units: 'Proportion', abbrev: 'LOVEGCOV' },
  54: { parameter: 'High-Vegetation Cover', units: 'Proportion', abbrev: 'HIVEGCOV' },
  55: { parameter: 'Leaf Area Index (Low-Vegetation)', units: 'm2 m-2', abbrev: 'LAILO' },
  56: { parameter: 'Leaf Area Index (High-Vegetation)', units: 'm2 m-2', abbrev: 'LAIHI' },
  57: { parameter: 'Type of Low-Vegetation', units: 'See Table 4.234', abbrev: 'TYPLOVEG' },
  58: { parameter: 'Type of High-Vegetation', units: 'See Table 4.234', abbrev: 'TYPHIVEG' },
  59: { parameter: 'Net Ecosystem Exchange Flux', units: 'kg-2 s-1', abbrev: 'NECOFLUX' },
  60: { parameter: 'Gross Primary Production Flux', units: 'kg-2 s-1', abbrev: 'GROSSFLUX' },
  61: { parameter: 'Ecosystem Respiration Flux', units: 'kg-2 s-1', abbrev: 'ECORFLUX' },
  62: { parameter: 'Emissivity', units: 'Proportion', abbrev: 'EMISS' },
  63: { parameter: 'Canopy Air Temperature', units: 'K', abbrev: 'CANTMP' },
  // Reserved 64-191
  // Reserved for Local Use 192-254
  192: { parameter: 'Volumetric Soil Moisture Content', units: 'Fraction', abbrev: 'SOILW' },
  193: { parameter: 'Ground Heat Flux', units: 'W m-2', abbrev: 'GFLUX' },
  194: { parameter: 'Moisture Availability', units: '%', abbrev: 'MSTAV' },
  195: { parameter: 'Exchange Coefficient', units: '(kg m-3) (m s-1)', abbrev: 'SFEXC' },
  196: { parameter: 'Plant Canopy Surface Water', units: 'kg m-2', abbrev: 'CNWAT' },
  197: { parameter: "Blackadar's Mixing Length Scale", units: 'm', abbrev: 'BMIXL' },
  198: { parameter: 'Vegetation Type', units: 'Integer (0-13)', abbrev: 'VGTYP' },
  199: { parameter: 'Canopy Conductance', units: 'm s-1', abbrev: 'CCOND' },
  200: { parameter: 'Minimal Stomatal Resistance', units: 's m-1', abbrev: 'RSMIN' },
  201: { parameter: 'Wilting Point', units: 'Fraction', abbrev: 'WILT' },
  202: { parameter: 'Solar parameter in canopy conductance', units: 'Fraction', abbrev: 'RCS' },
  203: {
    parameter: 'Temperature parameter in canopy conductance',
    units: 'Fraction',
    abbrev: 'RCT',
  },
  204: { parameter: 'Humidity parameter in canopy conductance', units: 'Fraction', abbrev: 'RCQ' },
  205: {
    parameter: 'Soil moisture parameter in canopy conductance',
    units: 'Fraction',
    abbrev: 'RCSOL',
  },
  206: { parameter: 'Rate of water dropping from canopy to ground', units: '', abbrev: 'RDRIP' },
  207: { parameter: 'Ice-free water surface', units: '%', abbrev: 'ICWAT' },
  208: {
    parameter: 'Surface exchange coefficients for T and Q divided by delta z',
    units: 'm s-1',
    abbrev: 'AKHS',
  },
  209: {
    parameter: 'Surface exchange coefficients for U and V divided by delta z',
    units: 'm s-1',
    abbrev: 'AKMS',
  },
  210: { parameter: 'Vegetation Canopy Temperature', units: 'K', abbrev: 'VEGT' },
  211: { parameter: 'Surface Water Storage', units: 'kg m-2', abbrev: 'SSTOR' },
  212: { parameter: 'Liquid Soil Moisture Content (non-frozen)', units: 'kg m-2', abbrev: 'LSOIL' },
  213: { parameter: 'Open Water Evaporation (standing water)', units: 'W m-2', abbrev: 'EWATR' },
  214: { parameter: 'Groundwater Recharge', units: 'kg m-2', abbrev: 'GWREC' },
  215: { parameter: 'Flood Plain Recharge', units: 'kg m-2', abbrev: 'QREC' },
  216: { parameter: 'Roughness Length for Heat', units: 'm', abbrev: 'SFCRH' },
  217: { parameter: 'Normalized Difference Vegetation Index', units: '', abbrev: 'NDVI' },
  218: {
    parameter: 'Land-Sea Coverage (nearest neighbor) [land=1, sea=0]',
    units: '',
    abbrev: 'LANDN',
  },
  219: { parameter: 'Asymptotic Mixing Length Scale', units: 'm', abbrev: 'AMIXL' },
  220: { parameter: 'Water Vapor Added by Precip Assimilation', units: 'kg m-2', abbrev: 'WVINC' },
  221: {
    parameter: 'Water Condensate Added by Precip Assimilation',
    units: 'kg m-2',
    abbrev: 'WCINC',
  },
  222: {
    parameter: 'Water Vapor Flux Convergence (Vertical Int)',
    units: 'kg m-2',
    abbrev: 'WVCONV',
  },
  223: {
    parameter: 'Water Condensate Flux Convergence (Vertical Int)',
    units: 'kg m-2',
    abbrev: 'WCCONV',
  },
  224: { parameter: 'Water Vapor Zonal Flux (Vertical Int)', units: 'kg m-2', abbrev: 'WVUFLX' },
  225: {
    parameter: 'Water Vapor Meridional Flux (Vertical Int)',
    units: 'kg m-2',
    abbrev: 'WVVFLX',
  },
  226: {
    parameter: 'Water Condensate Zonal Flux (Vertical Int)',
    units: 'kg m-2',
    abbrev: 'WCUFLX',
  },
  227: {
    parameter: 'Water Condensate Meridional Flux (Vertical Int)',
    units: 'kg m-2',
    abbrev: 'WCVFLX',
  },
  228: { parameter: 'Aerodynamic Conductance', units: 'm s-1', abbrev: 'ACOND' },
  229: { parameter: 'Canopy Water Evaporation', units: 'W m-2', abbrev: 'EVCW' },
  230: { parameter: 'Transpiration', units: 'W m-2', abbrev: 'TRANS' },
  231: {
    parameter: 'Seasonally Minimum Green Vegetation Fraction (over 1-year period)',
    units: '%',
    abbrev: 'VEGMIN',
  },
  232: {
    parameter: 'Seasonally Maximum Green Vegetation Fraction (over 1-year period)',
    units: '%',
    abbrev: 'VEGMAX',
  },
  233: { parameter: 'Land Fraction', units: 'Fraction', abbrev: 'LANDFRC' },
  234: { parameter: 'Lake Fraction', units: 'Fraction', abbrev: 'LAKEFRC' },
  235: { parameter: 'Precipitation Advected Heat Flux', units: 'W m-2', abbrev: 'PAHFLX' },
  236: { parameter: 'Water Storage in Aquifer', units: 'kg m-2', abbrev: 'WATERSA' },
  237: { parameter: 'Evaporation of Intercepted Water', units: 'kg m-2', abbrev: 'EIWATER' },
  238: { parameter: 'Plant Transpiration', units: 'kg m-2', abbrev: 'PLANTTR' },
  239: { parameter: 'Soil Surface Evaporation', units: 'kg m-2', abbrev: 'SOILSE' },
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-1
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 1
 * **(Land Surface products, Agricultural Special Products category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 1 (Agricultural Special Products)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Created**: 12/21/2012
 *
 * **Reserved Ranges**:
 * - `0-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_21: Record<number, TableCategory> = {
  // Reserved 0-191
  192: { parameter: 'Cold Advisory for Newborn Livestock', units: '', abbrev: 'CANL' },
  // Reserved for Local Use 193-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-3
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 3
 * **(Land Surface products, Soil category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 3 (Soil)
 * - **Section**: 4
 * - **Octet 10**: 3
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `31-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Deprecated parameters are marked. Refer to Regulation 92.6.2 for alternatives.
 * 2. It is recommended to avoid parameters flagged as less descriptive; alternatives are preferred.
 */
export const grib2LookupTable42_23: Record<number, TableCategory> = {
  0: { parameter: 'Soil Type', units: 'See Table 4.213', abbrev: 'SOTYP' },
  1: { parameter: 'Upper Layer Soil Temperature (Deprecated)', units: 'K', abbrev: 'UPLST' },
  2: { parameter: 'Upper Layer Soil Moisture (Deprecated)', units: 'kg m-3', abbrev: 'UPLSM' },
  3: { parameter: 'Lower Layer Soil Moisture (Deprecated)', units: 'kg m-3', abbrev: 'LOWLSM' },
  4: { parameter: 'Bottom Layer Soil Temperature (Deprecated)', units: 'K', abbrev: 'BOTLST' },
  5: {
    parameter: 'Liquid Volumetric Soil Moisture (non-frozen)',
    units: 'Proportion',
    abbrev: 'SOILL',
  },
  6: { parameter: 'Number of Soil Layers in Root Zone', units: 'Numeric', abbrev: 'RLYRS' },
  7: {
    parameter: 'Transpiration Stress-onset (soil moisture)',
    units: 'Proportion',
    abbrev: 'SMREF',
  },
  8: {
    parameter: 'Direct Evaporation Cease (soil moisture)',
    units: 'Proportion',
    abbrev: 'SMDRY',
  },
  9: { parameter: 'Soil Porosity', units: 'Proportion', abbrev: 'POROS' },
  10: {
    parameter: 'Liquid Volumetric Soil Moisture (Non-Frozen)',
    units: 'm3 m-3',
    abbrev: 'LIQVSM',
  },
  11: {
    parameter: 'Volumetric Transpiration Stress-Onset (Soil Moisture)',
    units: 'm3 m-3',
    abbrev: 'VOLTSO',
  },
  12: {
    parameter: 'Transpiration Stress-Onset (Soil Moisture)',
    units: 'kg m-3',
    abbrev: 'TRANSO',
  },
  13: {
    parameter: 'Volumetric Direct Evaporation Cease (Soil Moisture)',
    units: 'm3 m-3',
    abbrev: 'VOLDEC',
  },
  14: { parameter: 'Direct Evaporation Cease (Soil Moisture)', units: 'kg m-3', abbrev: 'DIREC' },
  15: { parameter: 'Soil Porosity', units: 'm3 m-3', abbrev: 'SOILP' },
  16: { parameter: 'Volumetric Saturation Of Soil Moisture', units: 'm3 m-3', abbrev: 'VSOSM' },
  17: { parameter: 'Saturation Of Soil Moisture', units: 'kg m-3', abbrev: 'SATOSM' },
  18: { parameter: 'Soil Temperature', units: 'K', abbrev: 'SOILTMP' },
  19: { parameter: 'Soil Moisture', units: 'kg m-3', abbrev: 'SOILMOI' },
  20: { parameter: 'Column-Integrated Soil Moisture', units: 'kg m-2', abbrev: 'CISOILM' },
  21: { parameter: 'Soil Ice', units: 'kg m-3', abbrev: 'SOILICE' },
  22: { parameter: 'Column-Integrated Soil Ice', units: 'kg m-2', abbrev: 'CISICE' },
  23: { parameter: 'Liquid Water in Snow Pack', units: 'kg m-2', abbrev: 'LWSNWP' },
  24: { parameter: 'Frost Index', units: 'kg day-1', abbrev: 'FRSTINX' },
  25: { parameter: 'Snow Depth at Elevation Bands', units: 'kg m-2', abbrev: 'SNWDEB' },
  26: { parameter: 'Soil Heat Flux', units: 'W m-2', abbrev: 'SHFLX' },
  27: { parameter: 'Soil Depth', units: 'm', abbrev: 'SOILDEP' },
  28: { parameter: 'Snow Temperature', units: 'K', abbrev: 'SNOWTMP' },
  29: { parameter: 'Ice Temperature', units: 'K', abbrev: 'ICETEMP' },
  30: { parameter: 'Soil Wetness Index', units: 'Numeric', abbrev: 'SWET' },
  // Reserved 31-191
  192: {
    parameter: 'Liquid Volumetric Soil Moisture (non Frozen)',
    units: 'Proportion',
    abbrev: 'SOILL',
  },
  193: { parameter: 'Number of Soil Layers in Root Zone', units: 'non-dim', abbrev: 'RLYRS' },
  194: { parameter: 'Surface Slope Type', units: 'Index', abbrev: 'SLTYP' },
  195: {
    parameter: 'Transpiration Stress-onset (soil moisture)',
    units: 'Proportion',
    abbrev: 'SMREF',
  },
  196: {
    parameter: 'Direct Evaporation Cease (soil moisture)',
    units: 'Proportion',
    abbrev: 'SMDRY',
  },
  197: { parameter: 'Soil Porosity', units: 'Proportion', abbrev: 'POROS' },
  198: { parameter: 'Direct Evaporation from Bare Soil', units: 'W m-2', abbrev: 'EVBS' },
  199: { parameter: 'Land Surface Precipitation Accumulation', units: 'kg m-2', abbrev: 'LSPA' },
  200: { parameter: 'Bare Soil Surface Skin Temperature', units: 'K', abbrev: 'BARET' },
  201: { parameter: 'Average Surface Skin Temperature', units: 'K', abbrev: 'AVSFT' },
  202: { parameter: 'Effective Radiative Skin Temperature', units: 'K', abbrev: 'RADT' },
  203: { parameter: 'Field Capacity', units: 'Fraction', abbrev: 'FLDCP' },
  204: {
    parameter: 'Soil Moisture Availability In The Top Soil Layer',
    units: '%',
    abbrev: 'MSTAV',
  },
  // Reserved for Local Use 205-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-4
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 4
 * **(Land Surface products, Fire Weather category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 4 (Fire Weather)
 * - **Section**: 4
 * - **Octet 10**: 4
 * - **Revised**: 10/30/2023
 *
 * **Reserved Ranges**:
 * - `37-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. The Fosberg index denotes the potential influence of weather on wildfire, factoring temperature, wind speed, relative humidity, and precipitation. Higher values indicate a greater potential impact.
 */
export const grib2LookupTable42_24: Record<number, TableCategory> = {
  0: { parameter: 'Fire Outlook', units: 'See Table 4.224', abbrev: 'FIREOLK' },
  1: {
    parameter: 'Fire Outlook Due to Dry Thunderstorm',
    units: 'See Table 4.224',
    abbrev: 'FIREODT',
  },
  2: { parameter: 'Haines Index', units: 'Numeric', abbrev: 'HINDEX' },
  3: { parameter: 'Fire Burned Area', units: '%', abbrev: 'FBAREA' },
  4: { parameter: 'Fosberg Index', units: 'Numeric', abbrev: 'FOSINDX' },
  5: {
    parameter: 'Fire Weather Index (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'FWINX',
  },
  6: {
    parameter: 'Fine Fuel Moisture Code (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'FFMCODE',
  },
  7: {
    parameter: 'Duff Moisture Code (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'DUFMCODE',
  },
  8: { parameter: 'Drought Code (Canadian Forest Service)', units: 'Numeric', abbrev: 'DRTCODE' },
  9: {
    parameter: 'Initial Fire Spread Index (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'INFSINX',
  },
  10: {
    parameter: 'Fire Build Up Index (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'FBUPINX',
  },
  11: {
    parameter: 'Fire Daily Severity Rating (Canadian Forest Service)',
    units: 'Numeric',
    abbrev: 'FDSRTE',
  },
  12: { parameter: 'Keetch-Byram Drought Index', units: 'Numeric', abbrev: 'KRIDX' },
  13: {
    parameter: 'Drought Factor (Australian forest service)',
    units: 'Numeric',
    abbrev: 'DRFACT',
  },
  14: {
    parameter: 'Rate of Spread (Australian forest service)',
    units: 'm s-1',
    abbrev: 'RATESPRD',
  },
  15: {
    parameter: 'Fire Danger Index (Australian forest service)',
    units: 'Numeric',
    abbrev: 'FIREDIDX',
  },
  16: {
    parameter: 'Spread Component (US Forest Service NFDRS)',
    units: 'Numeric',
    abbrev: 'SPRDCOMP',
  },
  17: {
    parameter: 'Burning Index (Australian forest service)',
    units: 'Numeric',
    abbrev: 'BURNIDX',
  },
  18: {
    parameter: 'Ignition Component (Australian forest service)',
    units: '%',
    abbrev: 'IGNCOMP',
  },
  19: {
    parameter: 'Energy Release Component (Australian forest service)',
    units: 'J m-2',
    abbrev: 'ENRELCOM',
  },
  20: { parameter: 'Burning Area', units: '%', abbrev: 'BURNAREA' },
  21: { parameter: 'Burnable Area', units: '%', abbrev: 'BURNABAREA' },
  22: { parameter: 'Unburnable Area', units: '%', abbrev: 'UNBURNAREA' },
  23: { parameter: 'Fuel Load', units: 'kg m-2', abbrev: 'FUELLOAD' },
  24: { parameter: 'Combustion Completeness', units: '%', abbrev: 'COMBCO' },
  25: { parameter: 'Fuel Moisture Content', units: 'kg kg-1', abbrev: 'FUELMC' },
  26: { parameter: 'Wildfire Potential (NOAA GSL)', units: 'Numeric', abbrev: 'WFIREPOT' },
  27: { parameter: 'Live Leaf Fuel Load', units: 'kg m-2', abbrev: 'LLFL' },
  28: { parameter: 'Live Wood Fuel Load', units: 'kg m-2', abbrev: 'LWFL' },
  29: { parameter: 'Dead Leaf Fuel Load', units: 'kg m-2', abbrev: 'DLFL' },
  30: { parameter: 'Dead Wood Fuel Load', units: 'kg m-2', abbrev: 'DWFL' },
  31: { parameter: 'Live Fuel Moisture Content', units: 'kg kg-1', abbrev: 'LFMC' },
  32: { parameter: 'Fine Dead Leaf Moisture Content', units: 'kg kg-1', abbrev: 'FDLMC' },
  33: { parameter: 'Dense Dead Leaf Moisture Content', units: 'kg kg-1', abbrev: 'DDLMC' },
  34: { parameter: 'Fine Dead Wood Moisture Content', units: 'kg kg-1', abbrev: 'FDWMC' },
  35: { parameter: 'Dense Dead Wood Moisture Content', units: 'kg kg-1', abbrev: 'DDWMC' },
  36: { parameter: 'Fire Radiative Power', units: 'W', abbrev: 'FRADPOW' },
  // Reserved 37-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-5
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 5
 * **(Land Surface products, Glaciers and Inland Ice category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 5 (Glaciers and Inland Ice)
 * - **Section**: 4
 * - **Octet 10**: 5
 * - **Revised**: 10/30/2023
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. A value strictly above 0.5 for Glacier Cover is treated as glacier. A value equal to or below 0.5 is treated as land without glacier.
 */
export const grib2LookupTable42_25: Record<number, TableCategory> = {
  0: { parameter: 'Glacier Cover', units: 'Proportion', abbrev: 'GLACCOV' },
  1: { parameter: 'Glacier Temperature', units: 'K', abbrev: 'GLACTMP' },
  // Reserved 2-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-2-6
 * PARAMETERS FOR DISCIPLINE 2, CATEGORY 6
 * **(Land Surface products, Urban Areas category)**
 *
 * **Details**:
 * - **Discipline**: 2 (Land Surface products)
 * - **Category**: 6 (Urban Areas)
 * - **Section**: 4
 * - **Octet 10**: 6
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `9-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_26: Record<number, TableCategory> = {
  0: { parameter: 'Urban Cover', units: 'Proportion', abbrev: 'URBCOVER' },
  1: { parameter: 'Road Cover', units: 'Proportion', abbrev: 'ROADCOVER' },
  2: { parameter: 'Building Cover', units: 'Proportion', abbrev: 'BUILDCOVER' },
  3: { parameter: 'Building Height', units: 'm', abbrev: 'BUILDHGT' },
  4: { parameter: 'Vertical-to-Horizontal Area Fraction', units: 'm2 m-2', abbrev: 'VZAFRAC' },
  5: { parameter: 'Standard Deviation of Building Height', units: 'm', abbrev: 'SDBUILDHGT' },
  6: { parameter: 'Distance downward from roof surface', units: 'm', abbrev: 'DDROOF' },
  7: { parameter: 'Distance inward from outer wall surface', units: 'm', abbrev: 'DIOWALL' },
  8: { parameter: 'Distance downward from road surface', units: 'm', abbrev: 'DDROAD' },
  // Reserved 9-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-0
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 0
 * **(Space products, Image Format category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 0 (Image Format)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Revised**: 06/27/2008
 *
 * **Reserved Ranges**:
 * - `10-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_30: Record<number, TableCategory> = {
  0: { parameter: 'Scaled Radiance', units: 'Numeric', abbrev: 'SRAD' },
  1: { parameter: 'Scaled Albedo', units: 'Numeric', abbrev: 'SALBEDO' },
  2: { parameter: 'Scaled Brightness Temperature', units: 'Numeric', abbrev: 'SBTMP' },
  3: { parameter: 'Scaled Precipitable Water', units: 'Numeric', abbrev: 'SPWAT' },
  4: { parameter: 'Scaled Lifted Index', units: 'Numeric', abbrev: 'SLFTI' },
  5: { parameter: 'Scaled Cloud Top Pressure', units: 'Numeric', abbrev: 'SCTPRES' },
  6: { parameter: 'Scaled Skin Temperature', units: 'Numeric', abbrev: 'SSTMP' },
  7: { parameter: 'Cloud Mask', units: 'See Table 4.217', abbrev: 'CLOUDM' },
  8: { parameter: 'Pixel Scene Type', units: 'See Table 4.218', abbrev: 'PIXST' },
  9: { parameter: 'Fire Detection Indicator', units: 'See Table 4.223', abbrev: 'FIREDI' },
  // Reserved 10-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-1
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 1
 * **(Space products, Quantitative category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 1 (Quantitative)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `18`: Reserved
 * - `24-26`: Reserved
 * - `33-97`: Reserved
 * - `100-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Bidirectional Reflectance Factor is the ratio of the radiant flux reflected by a surface to that reflected by an ideal, diffuse Lambertian standard surface under identical conditions.
 * 2. Scaled Radiance is the top-of-atmosphere radiance observed by a sensor, multiplied by π, divided by the in-band solar irradiance.
 */
export const grib2LookupTable42_31: Record<number, TableCategory> = {
  0: { parameter: 'Estimated Precipitation', units: 'kg m-2', abbrev: 'ESTP' },
  1: { parameter: 'Instantaneous Rain Rate', units: 'kg m-2 s-1', abbrev: 'IRRATE' },
  2: { parameter: 'Cloud Top Height', units: 'm', abbrev: 'CTOPH' },
  3: {
    parameter: 'Cloud Top Height Quality Indicator',
    units: 'Code table 4.219',
    abbrev: 'CTOPHQI',
  },
  4: { parameter: 'Estimated u-Component of Wind', units: 'm s-1', abbrev: 'ESTUGRD' },
  5: { parameter: 'Estimated v-Component of Wind', units: 'm s-1', abbrev: 'ESTVGRD' },
  6: { parameter: 'Number Of Pixels Used', units: 'Numeric', abbrev: 'NPIXU' },
  7: { parameter: 'Solar Zenith Angle', units: '°', abbrev: 'SOLZA' },
  8: { parameter: 'Relative Azimuth Angle', units: '°', abbrev: 'RAZA' },
  9: { parameter: 'Reflectance in 0.6 Micron Channel', units: '%', abbrev: 'RFL06' },
  10: { parameter: 'Reflectance in 0.8 Micron Channel', units: '%', abbrev: 'RFL08' },
  11: { parameter: 'Reflectance in 1.6 Micron Channel', units: '%', abbrev: 'RFL16' },
  12: { parameter: 'Reflectance in 3.9 Micron Channel', units: '%', abbrev: 'RFL39' },
  13: { parameter: 'Atmospheric Divergence', units: 's-1', abbrev: 'ATMDIV' },
  14: { parameter: 'Cloudy Brightness Temperature', units: 'K', abbrev: 'CBTMP' },
  15: { parameter: 'Clear Sky Brightness Temperature', units: 'K', abbrev: 'CSBTMP' },
  16: {
    parameter: 'Cloudy Radiance (with respect to wave number)',
    units: 'W m-1 sr-1',
    abbrev: 'CLDRAD',
  },
  17: {
    parameter: 'Clear Sky Radiance (with respect to wave number)',
    units: 'W m-1 sr-1',
    abbrev: 'CSKYRAD',
  },
  // Reserved 18
  19: { parameter: 'Wind Speed', units: 'm s-1', abbrev: 'WINDS' },
  20: { parameter: 'Aerosol Optical Thickness at 0.635 µm', units: '', abbrev: 'AOT06' },
  21: { parameter: 'Aerosol Optical Thickness at 0.810 µm', units: '', abbrev: 'AOT08' },
  22: { parameter: 'Aerosol Optical Thickness at 1.640 µm', units: '', abbrev: 'AOT16' },
  23: { parameter: 'Angstrom Coefficient', units: '', abbrev: 'ANGCOE' },
  // Reserved 24-26
  27: { parameter: 'Bidirectional Reflectance Factor', units: 'Numeric', abbrev: 'BRFLF' },
  28: { parameter: 'Brightness Temperature', units: 'K', abbrev: 'SPBRT' },
  29: { parameter: 'Scaled Radiance', units: 'Numeric', abbrev: 'SCRAD' },
  30: { parameter: 'Reflectance in 0.4 Micron Channel', units: '%', abbrev: 'RFL04' },
  31: { parameter: 'Cloudy Reflectance', units: '%', abbrev: 'CLDREF' },
  32: { parameter: 'Clear Reflectance', units: '%', abbrev: 'CLRREF' },
  // Reserved 33-97
  98: {
    parameter:
      'Correlation Coefficient Between MPE Rain Rates for Co-located IR Data and Microwave Data Rain Rates',
    units: 'Numeric',
    abbrev: 'CCMPEMRR',
  },
  99: {
    parameter:
      'Standard Deviation Between MPE Rain Rates for Co-located IR Data and Microwave Data Rain Rates',
    units: 'Numeric',
    abbrev: 'SDMPEMRR',
  },
  // Reserved 100-191
  192: { parameter: 'Scatterometer Estimated U Wind Component', units: 'm s-1', abbrev: 'USCT' },
  193: { parameter: 'Scatterometer Estimated V Wind Component', units: 'm s-1', abbrev: 'VSCT' },
  194: { parameter: 'Scatterometer Wind Quality', units: '', abbrev: 'SWQI' },
  // Reserved for Local Use 195-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-2
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 2
 * **(Space products, Cloud Properties category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 2 (Cloud Properties)
 * - **Section**: 4
 * - **Octet 10**: 2
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `12-29`: Reserved
 * - `41-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Numbers 31 to 40 are deprecated.
 */
export const grib2LookupTable42_32: Record<number, TableCategory> = {
  0: { parameter: 'Clear Sky Probability', units: '%', abbrev: 'CSKPROB' },
  1: { parameter: 'Cloud Top Temperature', units: 'K', abbrev: 'CTOPTMP' },
  2: { parameter: 'Cloud Top Pressure', units: 'Pa', abbrev: 'CTOPRES' },
  3: { parameter: 'Cloud Type', units: 'See Table 4.218', abbrev: 'CLDTYPE' },
  4: { parameter: 'Cloud Phase', units: 'See Table 4.218', abbrev: 'CLDPHAS' },
  5: { parameter: 'Cloud Optical Depth', units: 'Numeric', abbrev: 'CLDODEP' },
  6: { parameter: 'Cloud Particle Effective Radius', units: 'm', abbrev: 'CLDPER' },
  7: { parameter: 'Cloud Liquid Water Path', units: 'kg m-2', abbrev: 'CLDLWP' },
  8: { parameter: 'Cloud Ice Water Path', units: 'kg m-2', abbrev: 'CLDIWP' },
  9: { parameter: 'Cloud Albedo', units: 'Numeric', abbrev: 'CLDALB' },
  10: { parameter: 'Cloud Emissivity', units: 'Numeric', abbrev: 'CLDEMISS' },
  11: { parameter: 'Effective Absorption Optical Depth Ratio', units: 'Numeric', abbrev: 'EAODR' },
  // Reserved 12-29
  30: { parameter: 'Measurement Cost', units: 'Numeric', abbrev: 'MEACST' },
  31: { parameter: 'Upper Layer Cloud Optical Depth (Deprecated)', units: 'Numeric', abbrev: '' },
  32: { parameter: 'Upper Layer Cloud Top Pressure (Deprecated)', units: 'Pa', abbrev: '' },
  33: { parameter: 'Upper Layer Cloud Effective Radius (Deprecated)', units: 'm', abbrev: '' },
  34: {
    parameter: 'Error in Upper Layer Cloud Optical Depth (Deprecated)',
    units: 'Numeric',
    abbrev: '',
  },
  35: {
    parameter: 'Error in Upper Layer Cloud Top Pressure (Deprecated)',
    units: 'Pa',
    abbrev: '',
  },
  36: {
    parameter: 'Error in Upper Layer Cloud Effective Radius (Deprecated)',
    units: 'm',
    abbrev: '',
  },
  37: { parameter: 'Lower Layer Cloud Optical Depth (Deprecated)', units: 'Numeric', abbrev: '' },
  38: { parameter: 'Lower Layer Cloud Top Pressure (Deprecated)', units: 'Pa', abbrev: '' },
  39: {
    parameter: 'Error in Lower Layer Cloud Optical Depth (Deprecated)',
    units: 'Numeric',
    abbrev: '',
  },
  40: {
    parameter: 'Error in Lower Layer Cloud Top Pressure (Deprecated)',
    units: 'Pa',
    abbrev: '',
  },
  // Reserved 41-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-3
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 3
 * **(Space products, Flight Rules Conditions category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 3 (Flight Rules Conditions)
 * - **Section**: 4
 * - **Octet 10**: 3
 * - **Created**: 07/26/2016
 *
 * **Reserved Ranges**:
 * - `3-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_33: Record<number, TableCategory> = {
  0: {
    parameter: 'Probability of Encountering Marginal Visual Flight Rules Conditions',
    units: '%',
    abbrev: 'PBMVFRC',
  },
  1: {
    parameter: 'Probability of Encountering Low Instrument Flight Rules Conditions',
    units: '%',
    abbrev: 'PBLIFRC',
  },
  2: {
    parameter: 'Probability of Encountering Instrument Flight Rules Conditions',
    units: '%',
    abbrev: 'PBINFRC',
  },
  // Reserved 3-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-4
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 4
 * **(Space products, Volcanic Ash category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 4 (Volcanic Ash)
 * - **Section**: 4
 * - **Octet 10**: 4
 * - **Created**: 07/26/2016
 *
 * **Reserved Ranges**:
 * - `9-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_34: Record<number, TableCategory> = {
  0: { parameter: 'Volcanic Ash Probability', units: '%', abbrev: 'VOLAPROB' },
  1: { parameter: 'Volcanic Ash Cloud Top Temperature', units: 'K', abbrev: 'VOLACDTT' },
  2: { parameter: 'Volcanic Ash Cloud Top Pressure', units: 'Pa', abbrev: 'VOLACDTP' },
  3: { parameter: 'Volcanic Ash Cloud Top Height', units: 'm', abbrev: 'VOLACDTH' },
  4: { parameter: 'Volcanic Ash Cloud Emissivity', units: 'Numeric', abbrev: 'VOLACDEM' },
  5: {
    parameter: 'Volcanic Ash Effective Absorption Depth Ratio',
    units: 'Numeric',
    abbrev: 'VOLAEADR',
  },
  6: { parameter: 'Volcanic Ash Cloud Optical Depth', units: 'Numeric', abbrev: 'VOLACDOD' },
  7: { parameter: 'Volcanic Ash Column Density', units: 'kg m-2', abbrev: 'VOLACDEN' },
  8: { parameter: 'Volcanic Ash Particle Effective Radius', units: 'm', abbrev: 'VOLAPER' },
  // Reserved 9-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-5
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 5
 * **(Space products, Sea-Surface Temperature category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 5 (Sea-Surface Temperature)
 * - **Section**: 4
 * - **Octet 10**: 5
 * - **Created**: 07/26/2016
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Interface Sea-Surface Temperature: Theoretical temperature at the precise air-sea interface.
 * 2. Skin Sea-Surface Temperature: Temperature across a very small depth (~20 micrometers).
 * 3. Sub-Skin Sea-Surface Temperature: Temperature at the base of the thermal skin layer.
 * 4. Foundation Sea-Surface Temperature: Temperature in the water column free of diurnal variability.
 */
export const grib2LookupTable42_35: Record<number, TableCategory> = {
  0: { parameter: 'Interface Sea-Surface Temperature', units: 'K', abbrev: 'ISSTMP' },
  1: { parameter: 'Skin Sea-Surface Temperature', units: 'K', abbrev: 'SKSSTMP' },
  2: { parameter: 'Sub-Skin Sea-Surface Temperature', units: 'K', abbrev: 'SSKSSTMP' },
  3: { parameter: 'Foundation Sea-Surface Temperature', units: 'K', abbrev: 'FDNSSTMP' },
  4: {
    parameter: 'Estimated Bias Between Sea-Surface Temperature and Standard',
    units: 'K',
    abbrev: 'EBSSTSTD',
  },
  5: {
    parameter: 'Estimated Bias Standard Deviation Between Sea-Surface Temperature and Standard',
    units: 'K',
    abbrev: 'EBSDSSTS',
  },
  // Reserved 6-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-6
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 6
 * **(Space products, Solar Radiation category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 6 (Solar Radiation)
 * - **Section**: 4
 * - **Octet 10**: 6
 * - **Created**: 07/26/2016
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes
 * 1. Global Solar Irradiance: The solar flux per unit area received from a solid angle of 2π sr on a horizontal surface.
 * 2. Global Solar Exposure: The integral of global solar irradiance.
 * 3. Direct Solar Irradiance: Solar flux per unit area received from the Sun's disc on a surface normal to the Sun's direction.
 * 4. Direct Solar Exposure: Time integral of direct solar irradiance.
 * 5. Diffuse Solar Irradiance: Solar flux per unit area received from 2π sr, excluding the Sun's disc, on a horizontal surface.
 * 6. Diffuse Solar Exposure: Time integral of diffuse solar irradiance.
 */
export const grib2LookupTable42_36: Record<number, TableCategory> = {
  0: { parameter: 'Global Solar Irradiance', units: 'W m-2', abbrev: 'GSOLIRR' },
  1: { parameter: 'Global Solar Exposure', units: 'J m-2', abbrev: 'GSOLEXP' },
  2: { parameter: 'Direct Solar Irradiance', units: 'W m-2', abbrev: 'DIRSOLIR' },
  3: { parameter: 'Direct Solar Exposure', units: 'J m-2', abbrev: 'DIRSOLEX' },
  4: { parameter: 'Diffuse Solar Irradiance', units: 'W m-2', abbrev: 'DIFSOLIR' },
  5: { parameter: 'Diffuse Solar Exposure', units: 'J m-2', abbrev: 'DIFSOLEX' },
  // Reserved 6-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-3-192
 * PARAMETERS FOR DISCIPLINE 3, CATEGORY 192
 * **(Space products, Forecast Satellite Imagery category)**
 *
 * **Details**:
 * - **Discipline**: 3 (Space products)
 * - **Category**: 192 (Forecast Satellite Imagery)
 * - **Section**: 4
 * - **Octet 10**: 192
 * - **Revised**: 03/28/2022
 *
 * **Reserved Ranges**:
 * - `86-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_3192: Record<number, TableCategory> = {
  0: {
    parameter: 'Simulated Brightness Temperature for GOES 12, Channel 2',
    units: 'K',
    abbrev: 'SBT122',
  },
  1: {
    parameter: 'Simulated Brightness Temperature for GOES 12, Channel 3',
    units: 'K',
    abbrev: 'SBT123',
  },
  2: {
    parameter: 'Simulated Brightness Temperature for GOES 12, Channel 4',
    units: 'K',
    abbrev: 'SBT124',
  },
  3: {
    parameter: 'Simulated Brightness Temperature for GOES 12, Channel 6',
    units: 'K',
    abbrev: 'SBT126',
  },
  4: {
    parameter: 'Simulated Brightness Counts for GOES 12, Channel 3',
    units: 'Byte',
    abbrev: 'SBC123',
  },
  5: {
    parameter: 'Simulated Brightness Counts for GOES 12, Channel 4',
    units: 'Byte',
    abbrev: 'SBC124',
  },
  6: {
    parameter: 'Simulated Brightness Temperature for GOES 11, Channel 2',
    units: 'K',
    abbrev: 'SBT112',
  },
  7: {
    parameter: 'Simulated Brightness Temperature for GOES 11, Channel 3',
    units: 'K',
    abbrev: 'SBT113',
  },
  8: {
    parameter: 'Simulated Brightness Temperature for GOES 11, Channel 4',
    units: 'K',
    abbrev: 'SBT114',
  },
  9: {
    parameter: 'Simulated Brightness Temperature for GOES 11, Channel 5',
    units: 'K',
    abbrev: 'SBT115',
  },
  10: {
    parameter: 'Simulated Brightness Temperature for AMSRE on Aqua, Channel 9',
    units: 'K',
    abbrev: 'AMSRE9',
  },
  11: {
    parameter: 'Simulated Brightness Temperature for AMSRE on Aqua, Channel 10',
    units: 'K',
    abbrev: 'AMSRE10',
  },
  12: {
    parameter: 'Simulated Brightness Temperature for AMSRE on Aqua, Channel 11',
    units: 'K',
    abbrev: 'AMSRE11',
  },
  13: {
    parameter: 'Simulated Brightness Temperature for AMSRE on Aqua, Channel 12',
    units: 'K',
    abbrev: 'AMSRE12',
  },
  14: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-1',
    units: '',
    abbrev: 'SRFA161',
  },
  15: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-2',
    units: '',
    abbrev: 'SRFA162',
  },
  16: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-3',
    units: '',
    abbrev: 'SRFA163',
  },
  17: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-4',
    units: '',
    abbrev: 'SRFA164',
  },
  18: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-5',
    units: '',
    abbrev: 'SRFA165',
  },
  19: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-16, Band-6',
    units: '',
    abbrev: 'SRFA166',
  },
  20: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-7',
    units: 'K',
    abbrev: 'SBTA167',
  },
  21: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-8',
    units: 'K',
    abbrev: 'SBTA168',
  },
  22: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-9',
    units: 'K',
    abbrev: 'SBTA169',
  },
  23: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-10',
    units: 'K',
    abbrev: 'SBTA1610',
  },
  24: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-11',
    units: 'K',
    abbrev: 'SBTA1611',
  },
  25: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-12',
    units: 'K',
    abbrev: 'SBTA1612',
  },
  26: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-13',
    units: 'K',
    abbrev: 'SBTA1613',
  },
  27: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-14',
    units: 'K',
    abbrev: 'SBTA1614',
  },
  28: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-15',
    units: 'K',
    abbrev: 'SBTA1615',
  },
  29: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-16, Band-16',
    units: 'K',
    abbrev: 'SBTA1616',
  },
  30: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-1',
    units: '',
    abbrev: 'SRFA171',
  },
  31: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-2',
    units: '',
    abbrev: 'SRFA172',
  },
  32: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-3',
    units: '',
    abbrev: 'SRFA173',
  },
  33: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-4',
    units: '',
    abbrev: 'SRFA174',
  },
  34: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-5',
    units: '',
    abbrev: 'SRFA175',
  },
  35: {
    parameter: 'Simulated Reflectance Factor for ABI GOES-17, Band-6',
    units: '',
    abbrev: 'SRFA176',
  },
  36: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-7',
    units: 'K',
    abbrev: 'SBTA177',
  },
  37: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-8',
    units: 'K',
    abbrev: 'SBTA178',
  },
  38: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-9',
    units: 'K',
    abbrev: 'SBTA179',
  },
  39: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-10',
    units: 'K',
    abbrev: 'SBTA1710',
  },
  40: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-11',
    units: 'K',
    abbrev: 'SBTA1711',
  },
  41: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-12',
    units: 'K',
    abbrev: 'SBTA1712',
  },
  42: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-13',
    units: 'K',
    abbrev: 'SBTA1713',
  },
  43: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-14',
    units: 'K',
    abbrev: 'SBTA1714',
  },
  44: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-15',
    units: 'K',
    abbrev: 'SBTA1715',
  },
  45: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-17, Band-16',
    units: 'K',
    abbrev: 'SBTA1716',
  },
  46: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-1',
    units: '',
    abbrev: 'SRFAGR1',
  },
  47: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-2',
    units: '',
    abbrev: 'SRFAGR2',
  },
  48: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-3',
    units: '',
    abbrev: 'SRFAGR3',
  },
  49: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-4',
    units: '',
    abbrev: 'SRFAGR4',
  },
  50: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-5',
    units: '',
    abbrev: 'SRFAGR5',
  },
  51: {
    parameter: 'Simulated Reflectance Factor for nadir ABI GOES-R, Band-6',
    units: '',
    abbrev: 'SRFAGR6',
  },
  52: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-7',
    units: 'K',
    abbrev: 'SBTAGR7',
  },
  53: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-8',
    units: 'K',
    abbrev: 'SBTAGR8',
  },
  54: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-9',
    units: 'K',
    abbrev: 'SBTAGR9',
  },
  55: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-10',
    units: 'K',
    abbrev: 'SBTAGR10',
  },
  56: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-11',
    units: 'K',
    abbrev: 'SBTAGR11',
  },
  57: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-12',
    units: 'K',
    abbrev: 'SBTAGR12',
  },
  58: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-13',
    units: 'K',
    abbrev: 'SBTAGR13',
  },
  59: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-14',
    units: 'K',
    abbrev: 'SBTAGR14',
  },
  60: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-15',
    units: 'K',
    abbrev: 'SBTAGR15',
  },
  61: {
    parameter: 'Simulated Brightness Temperature for nadir ABI GOES-R, Band-16',
    units: 'K',
    abbrev: 'SBTAGR16',
  },
  62: {
    parameter: 'Simulated Brightness Temperature for SSMIS-F17, Channel 15',
    units: 'K',
    abbrev: 'SSMS1715',
  },
  63: {
    parameter: 'Simulated Brightness Temperature for SSMIS-F17, Channel 16',
    units: 'K',
    abbrev: 'SSMS1716',
  },
  64: {
    parameter: 'Simulated Brightness Temperature for SSMIS-F17, Channel 17',
    units: 'K',
    abbrev: 'SSMS1717',
  },
  65: {
    parameter: 'Simulated Brightness Temperature for SSMIS-F17, Channel 18',
    units: 'K',
    abbrev: 'SSMS1718',
  },
  66: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-7',
    units: 'K',
    abbrev: 'SBTAHI7',
  },
  67: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-8',
    units: 'K',
    abbrev: 'SBTAHI8',
  },
  68: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-9',
    units: 'K',
    abbrev: 'SBTAHI9',
  },
  69: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-10',
    units: 'K',
    abbrev: 'SBTAHI10',
  },
  70: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-11',
    units: 'K',
    abbrev: 'SBTAHI11',
  },
  71: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-12',
    units: 'K',
    abbrev: 'SBTAHI12',
  },
  72: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-13',
    units: 'K',
    abbrev: 'SBTAHI13',
  },
  73: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-14',
    units: 'K',
    abbrev: 'SBTAHI14',
  },
  74: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-15',
    units: 'K',
    abbrev: 'SBTAHI15',
  },
  75: {
    parameter: 'Simulated Brightness Temperature for Himawari-8, Band-16',
    units: 'K',
    abbrev: 'SBTAHI16',
  },
  76: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-7',
    units: 'K',
    abbrev: 'SBTA187',
  },
  77: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-8',
    units: 'K',
    abbrev: 'SBTA188',
  },
  78: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-9',
    units: 'K',
    abbrev: 'SBTA189',
  },
  79: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-10',
    units: 'K',
    abbrev: 'SBTA1810',
  },
  80: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-11',
    units: 'K',
    abbrev: 'SBTA1811',
  },
  81: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-12',
    units: 'K',
    abbrev: 'SBTA1812',
  },
  82: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-13',
    units: 'K',
    abbrev: 'SBTA1813',
  },
  83: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-14',
    units: 'K',
    abbrev: 'SBTA1814',
  },
  84: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-15',
    units: 'K',
    abbrev: 'SBTA1815',
  },
  85: {
    parameter: 'Simulated Brightness Temperature for ABI GOES-18, Band-16',
    units: 'K',
    abbrev: 'SBTA1816',
  },
  // Reserved 86-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-0
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 0
 * **(Space Weather Products, Temperature category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 0 (Temperature)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Created**: 02/27/2012
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_40: Record<number, TableCategory> = {
  0: { parameter: 'Temperature', units: 'K', abbrev: 'TMPSWP' },
  1: { parameter: 'Electron Temperature', units: 'K', abbrev: 'ELECTMP' },
  2: { parameter: 'Proton Temperature', units: 'K', abbrev: 'PROTTMP' },
  3: { parameter: 'Ion Temperature', units: 'K', abbrev: 'IONTMP' },
  4: { parameter: 'Parallel Temperature', units: 'K', abbrev: 'PRATMP' },
  5: { parameter: 'Perpendicular Temperature', units: 'K', abbrev: 'PRPTMP' },
  // Reserved 6-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-1
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 1
 * **(Space Weather Products, Momentum category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 1 (Momentum)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_41: Record<number, TableCategory> = {
  0: { parameter: 'Velocity Magnitude (Speed)', units: 'm s-1', abbrev: 'SPEED' },
  1: {
    parameter: '1st Vector Component of Velocity (Coordinate system dependent)',
    units: 'm s-1',
    abbrev: 'VEL1',
  },
  2: {
    parameter: '2nd Vector Component of Velocity (Coordinate system dependent)',
    units: 'm s-1',
    abbrev: 'VEL2',
  },
  3: {
    parameter: '3rd Vector Component of Velocity (Coordinate system dependent)',
    units: 'm s-1',
    abbrev: 'VEL3',
  },
  // Reserved 4-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-2
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 2
 * **(Space Weather Products, Charged Particle Mass and Number category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 2 (Charged Particle Mass and Number)
 * - **Section**: 4
 * - **Octet 10**: 2
 * - **Revised**: 10/30/2023
 *
 * **Reserved Ranges**:
 * - `14-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_42: Record<number, TableCategory> = {
  0: { parameter: 'Particle Number Density', units: 'm-3', abbrev: 'PLSMDEN' },
  1: { parameter: 'Electron Density', units: 'm-3', abbrev: 'ELCDEN' },
  2: { parameter: 'Proton Density', units: 'm-3', abbrev: 'PROTDEN' },
  3: { parameter: 'Ion Density', units: 'm-3', abbrev: 'IONDEN' },
  4: { parameter: 'Vertical Total Electron Content', units: 'TECU', abbrev: 'VTEC' },
  5: { parameter: 'HF Absorption Frequency', units: 'Hz', abbrev: 'ABSFRQ' },
  6: { parameter: 'HF Absorption', units: 'dB', abbrev: 'ABSRB' },
  7: { parameter: 'Spread F', units: 'm', abbrev: 'SPRDF' },
  8: { parameter: "h'F", units: 'm', abbrev: 'HPRIMF' },
  9: { parameter: 'Critical Frequency', units: 'Hz', abbrev: 'CRTFRQ' },
  10: { parameter: 'Maximal Usable Frequency (MUF)', units: 'Hz', abbrev: 'MAXUFZ' },
  11: { parameter: 'Peak Height (hm)', units: 'm', abbrev: 'PEAKH' },
  12: { parameter: 'Peak Density', units: 'm-3', abbrev: 'PEAKDEN' },
  13: { parameter: 'Equivalent Slab Thickness (tau)', units: 'km', abbrev: 'EQSLABT' },
  // Reserved 14-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-3
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 3
 * **(Space Weather Products, Electric and Magnetic Fields category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 3 (Electric and Magnetic Fields)
 * - **Section**: 4
 * - **Octet 10**: 3
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `8-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_43: Record<number, TableCategory> = {
  0: { parameter: 'Magnetic Field Magnitude', units: 'T', abbrev: 'BTOT' },
  1: { parameter: '1st Vector Component of Magnetic Field', units: 'T', abbrev: 'BVEC1' },
  2: { parameter: '2nd Vector Component of Magnetic Field', units: 'T', abbrev: 'BVEC2' },
  3: { parameter: '3rd Vector Component of Magnetic Field', units: 'T', abbrev: 'BVEC3' },
  4: { parameter: 'Electric Field Magnitude', units: 'V m-1', abbrev: 'ETOT' },
  5: { parameter: '1st Vector Component of Electric Field', units: 'V m-1', abbrev: 'EVEC1' },
  6: { parameter: '2nd Vector Component of Electric Field', units: 'V m-1', abbrev: 'EVEC2' },
  7: { parameter: '3rd Vector Component of Electric Field', units: 'V m-1', abbrev: 'EVEC3' },
  // Reserved 8-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-4
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 4
 * **(Space Weather Products, Energetic Particles category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 4 (Energetic Particles)
 * - **Section**: 4
 * - **Octet 10**: 4
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `7-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_44: Record<number, TableCategory> = {
  0: { parameter: 'Proton Flux (Differential)', units: '(m2 s sr eV)-1', abbrev: 'DIFPFLUX' },
  1: { parameter: 'Proton Flux (Integral)', units: '(m2 s sr)-1', abbrev: 'INTPFLUX' },
  2: { parameter: 'Electron Flux (Differential)', units: '(m2 s sr eV)-1', abbrev: 'DIFEFLUX' },
  3: { parameter: 'Electron Flux (Integral)', units: '(m2 s sr)-1', abbrev: 'INTEFLUX' },
  4: {
    parameter: 'Heavy Ion Flux (Differential)',
    units: '(m2 s sr eV / nuc)-1',
    abbrev: 'DIFIFLUX',
  },
  5: { parameter: 'Heavy Ion Flux (Integral)', units: '(m2 s sr)-1', abbrev: 'INTIFLUX' },
  6: { parameter: 'Cosmic Ray Neutron Flux', units: 'h-1', abbrev: 'NTRNFLUX' },
  // Reserved 7-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-5
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 5
 * **(Space Weather Products, Waves category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 5 (Waves)
 * - **Section**: 4
 * - **Octet 10**: 5
 * - **Revised**: 06/29/2022
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_45: Record<number, TableCategory> = {
  0: { parameter: 'Amplitude', units: 'rad', abbrev: 'AMPL' },
  1: { parameter: 'Phase', units: 'rad', abbrev: 'PHASE' },
  2: { parameter: 'Frequency', units: 'Hz', abbrev: 'FREQ' },
  3: { parameter: 'Wavelength', units: 'm', abbrev: 'WAVELGTH' },
  // Reserved 4-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-6
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 6
 * **(Space Weather Products, Solar Electromagnetic Emissions category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 6 (Solar Electromagnetic Emissions)
 * - **Section**: 4
 * - **Octet 10**: 6
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `7-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_46: Record<number, TableCategory> = {
  0: { parameter: 'Integrated Solar Irradiance', units: 'W m-2', abbrev: 'TSI' },
  1: { parameter: 'Solar X-ray Flux (XRS Long)', units: 'W m-2', abbrev: 'XLONG' },
  2: { parameter: 'Solar X-ray Flux (XRS Short)', units: 'W m-2', abbrev: 'XSHRT' },
  3: { parameter: 'Solar EUV Irradiance', units: 'W m-2', abbrev: 'EUVIRR' },
  4: { parameter: 'Solar Spectral Irradiance', units: 'W m-2 nm-1', abbrev: 'SPECIRR' },
  5: { parameter: 'F10.7', units: 'W m-2 Hz-1', abbrev: 'F107' },
  6: { parameter: 'Solar Radio Emissions', units: 'W m-2 Hz-1', abbrev: 'SOLRF' },
  // Reserved 7-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-7
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 7
 * **(Space Weather Products, Terrestrial Electromagnetic Emissions category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 7 (Terrestrial Electromagnetic Emissions)
 * - **Section**: 4
 * - **Octet 10**: 7
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_47: Record<number, TableCategory> = {
  0: { parameter: 'Limb Intensity', units: 'J m-2 s-1', abbrev: 'LMBINT' },
  1: { parameter: 'Disk Intensity', units: 'J m-2 s-1', abbrev: 'DSKINT' },
  2: { parameter: 'Disk Intensity Day', units: 'J m-2 s-1', abbrev: 'DSKDAY' },
  3: { parameter: 'Disk Intensity Night', units: 'J m-2 s-1', abbrev: 'DSKNGT' },
  // Reserved 4-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-8
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 8
 * **(Space Weather Products, Imagery category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 8 (Imagery)
 * - **Section**: 4
 * - **Octet 10**: 8
 * - **Revised**: 10/30/2023
 *
 * **Reserved Ranges**:
 * - `9-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_48: Record<number, TableCategory> = {
  0: { parameter: 'X-Ray Radiance', units: 'W sr-1 m-2', abbrev: 'XRAYRAD' },
  1: { parameter: 'EUV Radiance', units: 'W sr-1 m-2', abbrev: 'EUVRAD' },
  2: { parameter: 'H-Alpha Radiance', units: 'W sr-1 m-2', abbrev: 'HARAD' },
  3: { parameter: 'White Light Radiance', units: 'W sr-1 m-2', abbrev: 'WHTRAD' },
  4: { parameter: 'CaII-K Radiance', units: 'W sr-1 m-2', abbrev: 'CAIIRAD' },
  5: { parameter: 'White Light Coronagraph Radiance', units: 'W sr-1 m-2', abbrev: 'WHTCOR' },
  6: { parameter: 'Heliospheric Radiance', units: 'W sr-1 m-2', abbrev: 'HELCOR' },
  7: { parameter: 'Thematic Mask', units: 'Numeric', abbrev: 'MASK' },
  8: { parameter: 'Solar Induced Chlorophyll Fluorescence', units: 'W sr-1 m-2', abbrev: 'SICFL' },
  // Reserved 9-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-9
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 9
 * **(Space Weather Products, Ion-Neutral Coupling category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 9 (Ion-Neutral Coupling)
 * - **Section**: 4
 * - **Octet 10**: 9
 * - **Created**: 12/15/2011
 *
 * **Reserved Ranges**:
 * - `3-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_49: Record<number, TableCategory> = {
  0: { parameter: 'Pedersen Conductivity', units: 'S m-1', abbrev: 'SIGPED' },
  1: { parameter: 'Hall Conductivity', units: 'S m-1', abbrev: 'SIGHAL' },
  2: { parameter: 'Parallel Conductivity', units: 'S m-1', abbrev: 'SIGPAR' },
  // Reserved 3-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-4-10
 * PARAMETERS FOR DISCIPLINE 4, CATEGORY 10
 * **(Space Weather Products, Space Weather Indices Category)**
 *
 * **Details**:
 * - **Discipline**: 4 (Space Weather Products)
 * - **Category**: 10 (Space Weather Indices)
 * - **Section**: 4
 * - **Octet 10**: 10
 * - **Created**: 06/29/2022
 *
 * **Reserved Ranges**:
 * - `8-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_410: Record<number, TableCategory> = {
  0: { parameter: 'Scintillation Index (sigma phi)', units: 'rad', abbrev: 'SCINIDX' },
  1: { parameter: 'Scintillation Index S4', units: 'Numeric', abbrev: 'SCIDEXS4' },
  2: { parameter: 'Rate of Change of TEC Index (ROTI)', units: 'TECU/min', abbrev: 'ROTIDX' },
  3: {
    parameter: 'Disturbance Ionosphere Index Spatial Gradient (DIXSG)',
    units: 'Numeric',
    abbrev: 'DIDXSG',
  },
  4: { parameter: 'Along Arc TEC Rate (AATR)', units: 'TECU/min', abbrev: 'AATRATE' },
  5: { parameter: 'Kp', units: 'Numeric', abbrev: 'KP' },
  6: {
    parameter: 'Equatorial Disturbance Storm Time Index (Dst)',
    units: 'nT',
    abbrev: 'EDISSTIX',
  },
  7: { parameter: 'Auroral Electrojet (AE)', units: 'nT', abbrev: 'AURELEC' },
  // Reserved 8-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-0
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 0
 * **(Oceanographic products, Waves category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 0 (Waves)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `99-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes:
 * 1. Further information concerning the wave parameters can be found in the Guide to Wave Analysis and Forecasting (WMO-No. 702).
 * 2. The Charnock parameter accounts for increased aerodynamic roughness as wave heights grow due to increasing surface stress. It depends on the wind speed, wave age, and other aspects of the sea state.
 * 3. Parameters are normalized by being divided by the product of air density and the square of the friction velocity.
 */
export const grib2LookupTable42_100: Record<number, TableCategory> = {
  0: { parameter: 'Wave Spectra (1)', units: '-', abbrev: 'WVSP1' },
  1: { parameter: 'Wave Spectra (2)', units: '-', abbrev: 'WVSP2' },
  2: { parameter: 'Wave Spectra (3)', units: '-', abbrev: 'WVSP3' },
  3: {
    parameter: 'Significant Height of Combined Wind Waves and Swell',
    units: 'm',
    abbrev: 'HTSGW',
  },
  4: { parameter: 'Direction of Wind Waves', units: 'degree true', abbrev: 'WVDIR' },
  5: { parameter: 'Significant Height of Wind Waves', units: 'm', abbrev: 'WVHGT' },
  6: { parameter: 'Mean Period of Wind Waves', units: 's', abbrev: 'WVPER' },
  7: { parameter: 'Direction of Swell Waves', units: 'degree true', abbrev: 'SWDIR' },
  8: { parameter: 'Significant Height of Swell Waves', units: 'm', abbrev: 'SWELL' },
  9: { parameter: 'Mean Period of Swell Waves', units: 's', abbrev: 'SWPER' },
  10: { parameter: 'Primary Wave Direction', units: 'degree true', abbrev: 'DIRPW' },
  11: { parameter: 'Primary Wave Mean Period', units: 's', abbrev: 'PERPW' },
  12: { parameter: 'Secondary Wave Direction', units: 'degree true', abbrev: 'DIRSW' },
  13: { parameter: 'Secondary Wave Mean Period', units: 's', abbrev: 'PERSW' },
  14: {
    parameter: 'Direction of Combined Wind Waves and Swell',
    units: 'degree true',
    abbrev: 'WWSDIR',
  },
  15: { parameter: 'Mean Period of Combined Wind Waves and Swell', units: 's', abbrev: 'MWSPER' },
  16: { parameter: 'Coefficient of Drag With Waves', units: '-', abbrev: 'CDWW' },
  17: { parameter: 'Friction Velocity', units: 'm s-1', abbrev: 'FRICVW' },
  18: { parameter: 'Wave Stress', units: 'N m-2', abbrev: 'WSTR' },
  19: { parameter: 'Normalised Waves Stress', units: '-', abbrev: 'NWSTR' },
  20: { parameter: 'Mean Square Slope of Waves', units: '-', abbrev: 'MSSW' },
  21: { parameter: 'U-component Surface Stokes Drift', units: 'm s-1', abbrev: 'USSD' },
  22: { parameter: 'V-component Surface Stokes Drift', units: 'm s-1', abbrev: 'VSSD' },
  23: { parameter: 'Period of Maximum Individual Wave Height', units: 's', abbrev: 'PMAXWH' },
  24: { parameter: 'Maximum Individual Wave Height', units: 'm', abbrev: 'MAXWH' },
  25: { parameter: 'Inverse Mean Wave Frequency', units: 's', abbrev: 'IMWF' },
  26: { parameter: 'Inverse Mean Frequency of The Wind Waves', units: 's', abbrev: 'IMFWW' },
  27: { parameter: 'Inverse Mean Frequency of The Total Swell', units: 's', abbrev: 'IMFTSW' },
  28: { parameter: 'Mean Zero-Crossing Wave Period', units: 's', abbrev: 'MZWPER' },
  29: { parameter: 'Mean Zero-Crossing Period of The Wind Waves', units: 's', abbrev: 'MZPWW' },
  30: { parameter: 'Mean Zero-Crossing Period of The Total Swell', units: 's', abbrev: 'MZPTSW' },
  31: { parameter: 'Wave Directional Width', units: '-', abbrev: 'WDIRW' },
  32: { parameter: 'Directional Width of The Wind Waves', units: '-', abbrev: 'DIRWWW' },
  33: { parameter: 'Directional Width of The Total Swell', units: '-', abbrev: 'DIRWTS' },
  34: { parameter: 'Peak Wave Period', units: 's', abbrev: 'PWPER' },
  35: { parameter: 'Peak Period of The Wind Waves', units: 's', abbrev: 'PPERWW' },
  36: { parameter: 'Peak Period of The Total Swell', units: 's', abbrev: 'PPERTS' },
  37: { parameter: 'Altimeter Wave Height', units: 'm', abbrev: 'ALTWH' },
  38: { parameter: 'Altimeter Corrected Wave Height', units: 'm', abbrev: 'ALCWH' },
  39: { parameter: 'Altimeter Range Relative Correction', units: '-', abbrev: 'ALRRC' },
  40: { parameter: '10 Metre Neutral Wind Speed Over Waves', units: 'm s-1', abbrev: 'MNWSOW' },
  41: { parameter: '10 Metre Wind Direction Over Waves', units: 'degree true', abbrev: 'MWDIRW' },
  42: { parameter: 'Wave Engery Spectrum', units: 'm-2 s rad-1', abbrev: 'WESP' },
  43: {
    parameter: 'Kurtosis of The Sea Surface Elevation Due to Waves',
    units: '-',
    abbrev: 'KSSEW',
  },
  44: { parameter: 'Benjamin-Feir Index', units: '-', abbrev: 'BENINX' },
  45: { parameter: 'Spectral Peakedness Factor', units: 's-1', abbrev: 'SPFTR' },
  46: { parameter: 'Peak wave direction', units: '°', abbrev: 'PWAVEDIR' },
  47: {
    parameter: 'Significant wave height of first swell partition',
    units: 'm',
    abbrev: 'SWHFSWEL',
  },
  48: {
    parameter: 'Significant wave height of second swell partition',
    units: 'm',
    abbrev: 'SWHSSWEL',
  },
  49: {
    parameter: 'Significant wave height of third swell partition',
    units: 'm',
    abbrev: 'SWHTSWEL',
  },
  50: { parameter: 'Mean wave period of first swell partition', units: 's', abbrev: 'MWPFSWEL' },
  51: { parameter: 'Mean wave period of second swell partition', units: 's', abbrev: 'MWPSSWEL' },
  52: { parameter: 'Mean wave period of third swell partition', units: 's', abbrev: 'MWPTSWEL' },
  53: { parameter: 'Mean wave direction of first swell partition', units: '°', abbrev: 'MWDFSWEL' },
  54: {
    parameter: 'Mean wave direction of second swell partition',
    units: '°',
    abbrev: 'MWDSSWEL',
  },
  55: { parameter: 'Mean wave direction of third swell partition', units: '°', abbrev: 'MWDTSWEL' },
  56: {
    parameter: 'Wave directional width of first swell partition',
    units: '-',
    abbrev: 'WDWFSWEL',
  },
  57: {
    parameter: 'Wave directional width of second swell partition',
    units: '-',
    abbrev: 'WDWSSWEL',
  },
  58: {
    parameter: 'Wave directional width of third swell partition',
    units: '-',
    abbrev: 'WDWTSWEL',
  },
  59: {
    parameter: 'Wave frequency width of first swell partition',
    units: '-',
    abbrev: 'WFWFSWEL',
  },
  60: {
    parameter: 'Wave frequency width of second swell partition',
    units: '-',
    abbrev: 'WFWSSWEL',
  },
  61: {
    parameter: 'Wave frequency width of third swell partition',
    units: '-',
    abbrev: 'WFWTSWEL',
  },
  62: { parameter: 'Wave frequency width', units: '-', abbrev: 'WAVEFREW' },
  63: { parameter: 'Frequency width of wind waves', units: '-', abbrev: 'FREWWW' },
  64: { parameter: 'Frequency width of total swell', units: '-', abbrev: 'FREWTSW' },
  65: { parameter: 'Peak Wave Period of First Swell Partition', units: 's', abbrev: 'PWPFSPAR' },
  66: { parameter: 'Peak Wave Period of Second Swell Partition', units: 's', abbrev: 'PWPSSPAR' },
  67: { parameter: 'Peak Wave Period of Third Swell Partition', units: 's', abbrev: 'PWPTSPAR' },
  68: {
    parameter: 'Peak Wave Direction of First Swell Partition',
    units: 'degree true',
    abbrev: 'PWDFSPAR',
  },
  69: {
    parameter: 'Peak Wave Direction of Second Swell Partition',
    units: 'degree true',
    abbrev: 'PWDSSPAR',
  },
  70: {
    parameter: 'Peak Wave Direction of Third Swell Partition',
    units: 'degree true',
    abbrev: 'PWDTSPAR',
  },
  71: { parameter: 'Peak Direction of Wind Waves', units: 'degree true', abbrev: 'PDWWAVE' },
  72: { parameter: 'Peak Direction of Total Swell', units: 'degree true', abbrev: 'PDTSWELL' },
  73: { parameter: 'Whitecap Fraction', units: 'fraction', abbrev: 'WCAPFRAC' },
  74: { parameter: 'Mean Direction of Total Swell', units: 'degree', abbrev: 'MDTSWEL' },
  75: { parameter: 'Mean Direction of Wind Waves', units: 'degree', abbrev: 'MDWWAVE' },
  76: { parameter: 'Charnock (see Note 2)', units: 'Numeric', abbrev: 'CHNCK' },
  77: { parameter: 'Wave Spectral Skewness', units: 'Numeric', abbrev: 'WAVESPSK' },
  78: { parameter: 'Wave Energy Flux Magnitude', units: 'W m-1', abbrev: 'WAVEFMAG' },
  79: { parameter: 'Wave Energy Flux Mean Direction', units: 'degree true', abbrev: 'WAVEFDIR' },
  80: {
    parameter: 'Ratio of Wave Angular and Frequency Width',
    units: 'Numeric',
    abbrev: 'RWAVEAFW',
  },
  81: { parameter: 'Free Convective Velocity over the Oceans', units: 'm s-1', abbrev: 'FCVOCEAN' },
  82: { parameter: 'Air Density over the Oceans', units: 'kg m-3', abbrev: 'AIRDENOC' },
  83: {
    parameter: 'Normalized Energy Flux into Waves (see Note 3)',
    units: 'Numeric',
    abbrev: 'NEFW',
  },
  84: {
    parameter: 'Normalized Stress into Ocean (see Note 3)',
    units: 'Numeric',
    abbrev: 'NSOCEAN',
  },
  85: {
    parameter: 'Normalized Energy Flux into Ocean (see Note 3)',
    units: 'Numeric',
    abbrev: 'NEFOCEAN',
  },
  86: {
    parameter: 'Surface Elevation Variance due to Waves (over all frequencies and directions)',
    units: 'm2 s rad-1',
    abbrev: 'SEVWAVE',
  },
  87: { parameter: 'Wave Induced Mean Sea Level Correction', units: 'm', abbrev: 'WAVEMSLC' },
  88: { parameter: 'Spectral Width Index', units: 'Numeric', abbrev: 'SPECWI' },
  89: { parameter: 'Number of Events in Freak Wave Statistics', units: 'Numeric', abbrev: 'EFWS' },
  90: {
    parameter: 'U-Component of Surface Momentum Flux into Ocean',
    units: 'N m-2',
    abbrev: 'USMFO',
  },
  91: {
    parameter: 'V-Component of Surface Momentum Flux into Ocean',
    units: 'N m-2',
    abbrev: 'VSMFO',
  },
  92: { parameter: 'Wave Turbulent Energy Flux into Ocean', units: 'W m-2', abbrev: 'WAVETEFO' },
  93: { parameter: 'Envelope Maximum Individual Wave Height', units: 'm', abbrev: 'EMIWAVE' },
  94: { parameter: 'Time Domain Maximum Individual Crest Height', units: 'm', abbrev: 'TDMCREST' },
  95: { parameter: 'Time Domain Maximum Individual Wave Height', units: 'm', abbrev: 'TDMWAVE' },
  96: { parameter: 'Space Time Maximum Individual Crest Height', units: 'm', abbrev: 'STMCREST' },
  97: { parameter: 'Space Time Maximum Individual Wave Height', units: 'm', abbrev: 'STMWAVE' },
  98: { parameter: 'Goda Peakedness Factor', units: 'Numeric', abbrev: 'GODAPEAK' },
  // Reserved 99-191
  // Reserved for Local Use 192-254
  192: { parameter: 'Wave Steepness', units: 'proportion', abbrev: 'WSTP' },
  193: { parameter: 'Wave Length', units: '-', abbrev: 'WLENG' },
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-1
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 1
 * **(Oceanographic products, Currents category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 1 (Currents)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Revised**: 10/23/2023
 *
 * **Reserved Ranges**:
 * - `7-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_101: Record<number, TableCategory> = {
  0: { parameter: 'Current Direction', units: 'degree True', abbrev: 'DIRC' },
  1: { parameter: 'Current Speed', units: 'm s-1', abbrev: 'SPC' },
  2: { parameter: 'U-Component of Current', units: 'm s-1', abbrev: 'UOGRD' },
  3: { parameter: 'V-Component of Current', units: 'm s-1', abbrev: 'VOGRD' },
  4: { parameter: 'Rip Current Occurrence Probability', units: '%', abbrev: 'RIPCOP' },
  5: { parameter: 'Eastward Current', units: 'm s-1', abbrev: 'EASTCUR' },
  6: { parameter: 'Northward Current', units: 'm s-1', abbrev: 'NRTHCUR' },
  // Reserved 7-191
  192: { parameter: 'Ocean Mixed Layer U Velocity', units: 'm s-1', abbrev: 'OMLU' },
  193: { parameter: 'Ocean Mixed Layer V Velocity', units: 'm s-1', abbrev: 'OMLV' },
  194: { parameter: 'Barotropic U velocity', units: 'm s-1', abbrev: 'UBARO' },
  195: { parameter: 'Barotropic V velocity', units: 'm s-1', abbrev: 'VBARO' },
  // Reserved for Local Use 196-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-2
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 2
 * **(Oceanographic products, Ice category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 2 (Ice)
 * - **Section**: 4
 * - **Octet 10**: 2
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `26`: Reserved
 * - `30-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes:
 * 1. Ice internal pressure or stress (Pa m) is the integrated pressure across the vertical thickness of a layer of ice. It is produced when concentrated ice reacts to external forces such as wind and ocean currents.
 */
export const grib2LookupTable42_102: Record<number, TableCategory> = {
  0: { parameter: 'Ice Cover', units: 'Proportion', abbrev: 'ICEC' },
  1: { parameter: 'Ice Thickness', units: 'm', abbrev: 'ICETK' },
  2: { parameter: 'Direction of Ice Drift', units: 'degree True', abbrev: 'DICED' },
  3: { parameter: 'Speed of Ice Drift', units: 'm s-1', abbrev: 'SICED' },
  4: { parameter: 'U-Component of Ice Drift', units: 'm s-1', abbrev: 'UICE' },
  5: { parameter: 'V-Component of Ice Drift', units: 'm s-1', abbrev: 'VICE' },
  6: { parameter: 'Ice Growth Rate', units: 'm s-1', abbrev: 'ICEG' },
  7: { parameter: 'Ice Divergence', units: 's-1', abbrev: 'ICED' },
  8: { parameter: 'Ice Temperature', units: 'K', abbrev: 'ICETMP' },
  9: { parameter: 'Module of Ice Internal Pressure', units: 'Pa m', abbrev: 'ICEPRS' },
  10: {
    parameter: 'Zonal Vector Component of Vertically Integrated Ice Internal Pressure',
    units: 'Pa m',
    abbrev: 'ZVCICEP',
  },
  11: {
    parameter: 'Meridional Vector Component of Vertically Integrated Ice Internal Pressure',
    units: 'Pa m',
    abbrev: 'MVCICEP',
  },
  12: { parameter: 'Compressive Ice Strength', units: 'N m-1', abbrev: 'CICES' },
  13: { parameter: 'Snow Temperature (over sea ice)', units: 'K', abbrev: 'SNOWTSI' },
  14: { parameter: 'Albedo', units: 'Numeric', abbrev: 'ALBDOICE' },
  15: { parameter: 'Sea Ice Volume per Unit Area', units: 'm3m-2', abbrev: 'SICEVOL' },
  16: { parameter: 'Snow Volume Over Sea Ice per Unit Area', units: 'm3m-2', abbrev: 'SNVOLSI' },
  17: { parameter: 'Sea Ice Heat Content', units: 'J m-2', abbrev: 'SICEHC' },
  18: { parameter: 'Snow over Sea Ice Heat Content', units: 'J m-2', abbrev: 'SNCEHC' },
  19: { parameter: 'Ice Freeboard Thickness', units: 'm', abbrev: 'ICEFTHCK' },
  20: { parameter: 'Ice Melt Pond Fraction', units: 'fraction', abbrev: 'ICEMPF' },
  21: { parameter: 'Ice Melt Pond Depth', units: 'm', abbrev: 'ICEMPD' },
  22: { parameter: 'Ice Melt Pond Volume per Unit Area', units: 'm3m-2', abbrev: 'ICEMPV' },
  23: {
    parameter: 'Sea Ice Fraction Tendency due to Parameterization',
    units: 's-1',
    abbrev: 'SIFTP',
  },
  24: { parameter: 'x-component of ice drift', units: 'm s-1', abbrev: 'XICE' },
  25: { parameter: 'y-component of ice drift', units: 'm s-1', abbrev: 'YICE' },
  // Reserved 26
  27: {
    parameter: 'Freezing/melting potential (Tentatively accepted)',
    units: 'W m-2',
    abbrev: 'FRZMLTPOT',
  },
  28: { parameter: 'Melt onset date (Tentatively accepted)', units: 'Numeric', abbrev: 'MLTDATE' },
  29: {
    parameter: 'Freeze onset date (Tentatively accepted)',
    units: 'Numeric',
    abbrev: 'FRZDATE',
  },
  // Reserved 30-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-3
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 3
 * **(Oceanographic products, Surface Properties category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 3 (Surface Properties)
 * - **Section**: 4
 * - **Octet 10**: 3
 * - **Revised**: 10/30/2023
 *
 * **Reserved Ranges**:
 * - `22-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes:
 * 1. The x- and y- components of surface stress are not necessarily equivalent to the u- and v- components (eastward/northward).
 *    The x- and y- components strictly follow the defined coordinate system which may or may not follow the eastward and northward directions.
 */
export const grib2LookupTable42_103: Record<number, TableCategory> = {
  0: { parameter: 'Water Temperature', units: 'K', abbrev: 'WTMP' },
  1: { parameter: 'Deviation of Sea Level from Mean', units: 'm', abbrev: 'DSLM' },
  2: { parameter: 'Heat Exchange Coefficient', units: '', abbrev: 'CH' },
  3: { parameter: 'Practical Salinity', units: 'Numeric', abbrev: 'PRACTSAL' },
  4: { parameter: 'Downward Heat Flux', units: 'W m-2', abbrev: 'DWHFLUX' },
  5: { parameter: 'Eastward Surface Stress', units: 'N m-2', abbrev: 'EASTWSS' },
  6: { parameter: 'Northward Surface Stress', units: 'N m-2', abbrev: 'NORTHWSS' },
  7: { parameter: 'x-component Surface Stress', units: 'N m-2', abbrev: 'XCOMPSS' },
  8: { parameter: 'y-component Surface Stress', units: 'N m-2', abbrev: 'YCOMPSS' },
  9: { parameter: 'Thermosteric Change in Sea Surface Height', units: 'm', abbrev: 'THERCSSH' },
  10: { parameter: 'Halosteric Change in Sea Surface Height', units: 'm', abbrev: 'HALOCSSH' },
  11: { parameter: 'Steric Change in Sea Surface Height', units: 'm', abbrev: 'STERCSSH' },
  12: { parameter: 'Sea Salt Flux', units: 'kg m-2s-1', abbrev: 'SEASFLUX' },
  13: { parameter: 'Net Upward Water Flux', units: 'kg m-2s-1', abbrev: 'NETUPWFLUX' },
  14: { parameter: 'Eastward Surface Water Velocity', units: 'm s-1', abbrev: 'ESURFWVEL' },
  15: { parameter: 'Northward Surface Water Velocity', units: 'm s-1', abbrev: 'NSURFWVEL' },
  16: { parameter: 'x-component of Surface Water Velocity', units: 'm s-1', abbrev: 'XSURFWVEL' },
  17: { parameter: 'y-component of Surface Water Velocity', units: 'm s-1', abbrev: 'YSURFWVEL' },
  18: { parameter: 'Heat Flux Correction', units: 'W m-2', abbrev: 'HFLUXCOR' },
  19: {
    parameter: 'Sea Surface Height Tendency due to Parameterization',
    units: 'm s-1',
    abbrev: 'SSHGTPARM',
  },
  20: {
    parameter: 'Deviation of Sea Level from Mean with Inverse Barometer Correction',
    units: 'm',
    abbrev: 'DSLIBARCOR',
  },
  21: { parameter: 'Salinity', units: 'kg kg-1', abbrev: 'SALINITY' },
  // Reserved 22-191
  192: { parameter: 'Hurricane Storm Surge', units: 'm', abbrev: 'SURGE' },
  193: { parameter: 'Extra Tropical Storm Surge', units: 'm', abbrev: 'ETSRG' },
  194: { parameter: 'Ocean Surface Elevation Relative to Geoid', units: 'm', abbrev: 'ELEV' },
  195: { parameter: 'Sea Surface Height Relative to Geoid', units: 'm', abbrev: 'SSHG' },
  196: {
    parameter: 'Ocean Mixed Layer Potential Density (Reference 2000m)',
    units: 'kg m-3',
    abbrev: 'P2OMLT',
  },
  197: { parameter: 'Net Air-Ocean Heat Flux', units: 'W m-2', abbrev: 'AOHFLX' },
  198: { parameter: 'Assimilative Heat Flux', units: 'W m-2', abbrev: 'ASHFL' },
  199: { parameter: 'Surface Temperature Trend', units: 'degree per day', abbrev: 'SSTT' },
  200: { parameter: 'Surface Salinity Trend', units: 'psu per day', abbrev: 'SSST' },
  201: { parameter: 'Kinetic Energy', units: 'J kg-1', abbrev: 'KENG' },
  202: { parameter: 'Salt Flux', units: 'kg m-2s-1', abbrev: 'SLTFL' },
  203: { parameter: 'Heat Exchange Coefficient', units: '', abbrev: 'LCH' },
  204: { parameter: 'Freezing Spray', units: '', abbrev: 'FRZSPR' },
  205: {
    parameter: 'Total Water Level Accounting for Tide, Wind and Waves',
    units: 'm',
    abbrev: 'TWLWAV',
  },
  206: { parameter: 'Total Water Level Increase due to Waves', units: 'm', abbrev: 'RUNUP' },
  207: { parameter: 'Mean Increase in Water Level due to Waves', units: 'm', abbrev: 'SETUP' },
  208: {
    parameter: 'Time-varying Increase in Water Level due to Waves',
    units: 'm',
    abbrev: 'SWASH',
  },
  209: { parameter: 'Total Water Level Above Dune Toe', units: 'm', abbrev: 'TWLDT' },
  210: { parameter: 'Total Water Level Above Dune Crest', units: 'm', abbrev: 'TWLDC' },
  // Reserved 211-241
  242: { parameter: '20% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG20' },
  243: { parameter: '30% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG30' },
  244: { parameter: '40% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG40' },
  245: { parameter: '50% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG50' },
  246: { parameter: '60% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG60' },
  247: { parameter: '70% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG70' },
  248: { parameter: '80% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG80' },
  249: { parameter: '90% Tropical Cyclone Storm Surge Exceedance', units: 'm', abbrev: 'TCSRG90' },
  250: {
    parameter: 'Extra Tropical Storm Surge Combined Surge and Tide',
    units: 'm',
    abbrev: 'ETCWL',
  },
  251: { parameter: 'Tide', units: 'm', abbrev: 'TIDE' },
  252: { parameter: 'Erosion Occurrence Probability', units: '%', abbrev: 'EROSNP' },
  253: { parameter: 'Overwash Occurrence Probability', units: '%', abbrev: 'OWASHP' },
  // Reserved 254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-4
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 4
 * **(Oceanographic products, Sub-Surface Properties category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 4 (Sub-Surface Properties)
 * - **Section**: 4
 * - **Octet 10**: 4
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `8-10`: Reserved
 * - `52-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes:
 * 1. Numbers 17 and 20 are deviations from the reference value of 1000 kg m–3.
 * 2. The x- and y- components of water velocity are not necessarily equivalent to the u- and v- components (eastward/northward).
 *    The x- and y- components strictly follow the defined coordinate system which may or may not follow the eastward and northward directions.
 */
export const grib2LookupTable42_104: Record<number, TableCategory> = {
  0: { parameter: 'Main Thermocline Depth', units: 'm', abbrev: 'MTHD' },
  1: { parameter: 'Main Thermocline Anomaly', units: 'm', abbrev: 'MTHA' },
  2: { parameter: 'Transient Thermocline Depth', units: 'm', abbrev: 'TTHDP' },
  3: { parameter: 'Salinity', units: 'kg kg-1', abbrev: 'SALTY' },
  4: { parameter: 'Ocean Vertical Heat Diffusivity', units: 'm2 s-1', abbrev: 'OVHD' },
  5: { parameter: 'Ocean Vertical Salt Diffusivity', units: 'm2 s-1', abbrev: 'OVSD' },
  6: { parameter: 'Ocean Vertical Momentum Diffusivity', units: 'm2 s-1', abbrev: 'OVMD' },
  7: { parameter: 'Bathymetry', units: 'm', abbrev: 'BATHY' },
  // Reserved 8-10
  11: { parameter: 'Shape Factor With Respect To Salinity Profile', units: '', abbrev: 'SFSALP' },
  12: {
    parameter: 'Shape Factor With Respect To Temperature Profile In Thermocline',
    units: '',
    abbrev: 'SFTMPP',
  },
  13: {
    parameter: 'Attenuation Coefficient Of Water With Respect to Solar Radiation',
    units: 'm-1',
    abbrev: 'ACWSRD',
  },
  14: { parameter: 'Water Depth', units: 'm', abbrev: 'WDEPTH' },
  15: { parameter: 'Water Temperature', units: 'K', abbrev: 'WTMPSS' },
  16: { parameter: 'Water Density (rho)', units: 'kg m-3', abbrev: 'WATERDEN' },
  17: { parameter: 'Water Density Anomaly (sigma)', units: 'kg m-3', abbrev: 'WATDENA' },
  18: { parameter: 'Water Potential Temperature (theta)', units: 'K', abbrev: 'WATPTEMP' },
  19: { parameter: 'Water Potential Density (rho theta)', units: 'kg m-3', abbrev: 'WATPDEN' },
  20: {
    parameter: 'Water Potential Density Anomaly (sigma theta)',
    units: 'kg m-3',
    abbrev: 'WATPDENA',
  },
  21: { parameter: 'Practical Salinity', units: 'psu (numeric)', abbrev: 'PRTSAL' },
  22: { parameter: 'Water Column-integrated Heat Content', units: 'J m-2', abbrev: 'WCHEATC' },
  23: { parameter: 'Eastward Water Velocity', units: 'm s-1', abbrev: 'EASTWVEL' },
  24: { parameter: 'Northward Water Velocity', units: 'm s-1', abbrev: 'NRTHWVEL' },
  25: { parameter: 'X-Component Water Velocity', units: 'm s-1', abbrev: 'XCOMPWV' },
  26: { parameter: 'Y-Component Water Velocity', units: 'm s-1', abbrev: 'YCOMPWV' },
  27: { parameter: 'Upward Water Velocity', units: 'm s-1', abbrev: 'UPWWVEL' },
  28: { parameter: 'Vertical Eddy Diffusivity', units: 'm2 s-1', abbrev: 'VEDDYDIF' },
  29: { parameter: 'Bottom Pressure Equivalent Height', units: 'm', abbrev: 'BPEH' },
  30: {
    parameter: 'Fresh Water Flux into Sea Water from Rivers',
    units: 'kg m-2s-1',
    abbrev: 'FWFSW',
  },
  31: { parameter: 'Fresh Water Flux Correction', units: 'kg m-2s-1', abbrev: 'FWFC' },
  32: { parameter: 'Virtual Salt Flux into Sea Water', units: 'g kg-1 m-2s-1', abbrev: 'VSFSW' },
  33: { parameter: 'Virtual Salt Flux Correction', units: 'g kg-1 m-2s-1', abbrev: 'VSFC' },
  34: {
    parameter: 'Sea Water Temperature Tendency due to Newtonian Relaxation',
    units: 'K s-1',
    abbrev: 'SWTTNR',
  },
  35: {
    parameter: 'Sea Water Salinity Tendency due to Newtonian Relaxation',
    units: 'g kg-1s-1',
    abbrev: 'SWSTNR',
  },
  36: {
    parameter: 'Sea Water Temperature Tendency due to Parameterization',
    units: 'K s-1',
    abbrev: 'SWTTP',
  },
  37: {
    parameter: 'Sea Water Salinity Tendency due to Parameterization',
    units: 'g kg-1s-1',
    abbrev: 'SWSTP',
  },
  38: {
    parameter: 'Eastward Sea Water Velocity Tendency Due To Parameterization',
    units: 'm s-2',
    abbrev: 'ESWVP',
  },
  39: {
    parameter: 'Northward Sea Water Velocity Tendency Due To Parameterization',
    units: 'm s-2',
    abbrev: 'NSWVP',
  },
  40: {
    parameter: 'Sea Water Temperature Tendency Due to Direct Bias Correction',
    units: 'K s-1',
    abbrev: 'SWTTBC',
  },
  41: {
    parameter: 'Sea Water Salinity Tendency due to Direct Bias Correction',
    units: 'g kg-1s-1',
    abbrev: 'SWSTBC',
  },
  42: { parameter: 'Sea Water Meridional Volume Transport', units: 'm3 m-2 s-1', abbrev: 'SEAMVT' },
  43: { parameter: 'Sea Water Zonal Volume Transport', units: 'm3 m-2 s-1', abbrev: 'SEAZVT' },
  44: {
    parameter: 'Sea Water Column Integrated Meridional Volume Transport',
    units: 'm3 m-2 s-1',
    abbrev: 'SEACMVT',
  },
  45: {
    parameter: 'Sea Water Column Integrated Zonal Volume Transport',
    units: 'm3 m-2 s-1',
    abbrev: 'SEACZVT',
  },
  46: { parameter: 'Sea Water Meridional Mass Transport', units: 'kg m-2 s-1', abbrev: 'SEAMMT' },
  47: { parameter: 'Sea Water Zonal Mass Transport', units: 'kg m-2 s-1', abbrev: 'SEAZMT' },
  48: {
    parameter: 'Sea Water Column Integrated Meridional Mass Transport',
    units: 'kg m-2 s-1',
    abbrev: 'SEACMMT',
  },
  49: {
    parameter: 'Sea Water Column Integrated Zonal Mass Transport',
    units: 'kg m-2 s-1',
    abbrev: 'SEACZMT',
  },
  50: {
    parameter: 'Sea Water Column Integrated Practical Salinity',
    units: 'g kg-1 m',
    abbrev: 'SEACPSALT',
  },
  51: { parameter: 'Sea Water Column Integrated Salinity', units: 'kg kg-1 m', abbrev: 'SEACSALT' },
  // Reserved 52-191
  192: { parameter: '3-D Temperature', units: '°C', abbrev: 'WTMPC' },
  193: { parameter: '3-D Salinity', units: 'psu', abbrev: 'SALIN' },
  194: { parameter: 'Barotropic Kinetic Energy', units: 'J kg-1', abbrev: 'BKENG' },
  195: { parameter: 'Geometric Depth Below Sea Surface', units: 'm', abbrev: 'DBSS' },
  196: { parameter: 'Interface Depths', units: 'm', abbrev: 'INTFD' },
  197: { parameter: 'Ocean Heat Content', units: 'J m-2', abbrev: 'OHC' },
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-10-191
 * PARAMETERS FOR DISCIPLINE 10, CATEGORY 191
 * **(Oceanographic products, Miscellaneous category)**
 *
 * **Details**:
 * - **Discipline**: 10 (Oceanographic Products)
 * - **Category**: 191 (Miscellaneous)
 * - **Section**: 4
 * - **Octet 10**: 191
 * - **Revised**: 06/30/2022
 *
 * **Reserved Ranges**:
 * - `2`: Reserved
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_10191: Record<number, TableCategory> = {
  0: {
    parameter: 'Seconds Prior To Initial Reference Time (Defined In Section 1)',
    units: 's',
    abbrev: 'IRTSEC',
  },
  1: { parameter: 'Meridional Overturning Stream Function', units: 'm3 s-1', abbrev: 'MOSF' },
  // Reserved 2
  3: { parameter: 'Days Since Last Observation', units: 'd', abbrev: 'DSLOBSO' },
  4: { parameter: 'Barotropic Stream Function', units: 'm3 s-1', abbrev: 'BARDSF' },
  // Reserved 5-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-20-0
 * PARAMETERS FOR DISCIPLINE 20, CATEGORY 0
 * **(Health and Socioeconomic Impacts, Health Indicators category)**
 *
 * **Details**:
 * - **Discipline**: 20 (Health and Socioeconomic Impacts)
 * - **Category**: 0 (Health Indicators)
 * - **Section**: 4
 * - **Octet 10**: 0
 * - **Created**: 06/30/2022
 *
 * **Reserved Ranges**:
 * - `9-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Notes:
 * - Wet-bulb Globe Temperature (parameter 2) and Globe Temperature (parameter 3) may require additional environmental conditions to calculate.
 */
export const grib2LookupTable42_2000: Record<number, TableCategory> = {
  0: { parameter: 'Universal Thermal Climate Index', units: 'K', abbrev: 'UTHCIDX' },
  1: { parameter: 'Mean Radiant Temperature', units: 'K', abbrev: 'MEANRTMP' },
  2: { parameter: 'Wet-bulb Globe Temperature', units: 'K', abbrev: 'WETBGTMP' },
  3: { parameter: 'Globe Temperature', units: 'K', abbrev: 'GLOBETMP' },
  4: { parameter: 'Humidex', units: 'K', abbrev: 'HUMIDX' },
  5: { parameter: 'Effective Temperature', units: 'K', abbrev: 'EFFTEMP' },
  6: { parameter: 'Normal Effective Temperature', units: 'K', abbrev: 'NOREFTMP' },
  7: { parameter: 'Standard Effective Temperature', units: 'K', abbrev: 'STDEFTMP' },
  8: { parameter: 'Physiological Equivalent Temperature', units: 'K', abbrev: 'PEQUTMP' },
  // Reserved 9-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-20-1
 * PARAMETERS FOR DISCIPLINE 20, CATEGORY 1
 * **(Health and Socioeconomic Impacts, Epidemiology category)**
 *
 * **Details**:
 * - **Discipline**: 20 (Health and Socioeconomic Impacts)
 * - **Category**: 1 (Epidemiology)
 * - **Section**: 4
 * - **Octet 10**: 1
 * - **Created**: 06/30/2022
 *
 * **Reserved Ranges**:
 * - `10-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_2001: Record<number, TableCategory> = {
  0: { parameter: 'Malaria Cases', units: 'Fraction', abbrev: 'MALACASE' },
  1: { parameter: 'Malaria Circumsporozoite Protein Rate', units: 'Fraction', abbrev: 'MACPRATE' },
  2: {
    parameter: 'Plasmodium Falciparum Entomological Inoculation Rate',
    units: 'Bites per day per person',
    abbrev: 'PFEIRATE',
  },
  3: {
    parameter: 'Human Bite Rate by Anopheles Vectors',
    units: 'Bites per day per person',
    abbrev: 'HBRATEAV',
  },
  4: { parameter: 'Malaria Immunity', units: 'Fraction', abbrev: 'MALAIMM' },
  5: { parameter: 'Falciparum Parasite Rates', units: 'Fraction', abbrev: 'FALPRATE' },
  6: {
    parameter: 'Detectable Falciparum Parasite Ratio (after day 10)',
    units: 'Fraction',
    abbrev: 'DFPRATIO',
  },
  7: { parameter: 'Anopheles Vector to Host Ratio', units: 'Fraction', abbrev: 'AVHRATIO' },
  8: { parameter: 'Anopheles Vector Number', units: 'Number m-2', abbrev: 'AVECTNUM' },
  9: {
    parameter: 'Fraction of Malarial Vector Reproductive Habitat',
    units: 'Fraction',
    abbrev: 'FMALVRH',
  },
  // Reserved 10-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-20-2
 * PARAMETERS FOR DISCIPLINE 20, CATEGORY 2
 * **(Health and Socioeconomic Impacts, Socioeconomic indicators category)**
 *
 * **Details**:
 * - **Discipline**: 20 (Health and Socioeconomic Impacts)
 * - **Category**: 2 (Socioeconomic Indicators)
 * - **Section**: 4
 * - **Octet 10**: 2
 * - **Created**: 06/30/2022
 *
 * **Reserved Ranges**:
 * - `1-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_2002: Record<number, TableCategory> = {
  0: { parameter: 'Population Density', units: 'Person m-2', abbrev: 'POPDEN' },
  // Reserved 1-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * # GRIB2 - TABLE 4.2-20-3
 * PARAMETERS FOR DISCIPLINE 20, CATEGORY 3
 * **(Health and Socioeconomic Impacts, Renewable Energy Sector category)**
 *
 * **Details**:
 * - **Discipline**: 20 (Health and Socioeconomic Impacts)
 * - **Category**: 3 (Renewable Energy Sector)
 * - **Section**: 4
 * - **Octet 10**: 3
 * - **Created**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `10-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const grib2LookupTable42_2003: Record<number, TableCategory> = {
  0: { parameter: 'Renewable power capacity', units: 'W', abbrev: 'RENPCAP' },
  1: { parameter: 'Renewable power production rate', units: 'W', abbrev: 'RENPPROD' },
  2: { parameter: 'Wind power capacity', units: 'W', abbrev: 'WINDPCAP' },
  3: { parameter: 'Wind power production rate', units: 'W', abbrev: 'WINDPPROD' },
  4: { parameter: 'Solar photovoltaic (PV) power capacity', units: 'W', abbrev: 'SPVPCAP' },
  5: { parameter: 'Solar photovoltaic (PV) power production rate', units: 'W', abbrev: 'SPVPPROD' },
  6: { parameter: 'Solar non-photovoltaic (PV) power capacity', units: 'W', abbrev: 'SNPVPCAP' },
  7: {
    parameter: 'Solar non-photovoltaic (PV) power production rate',
    units: 'W',
    abbrev: 'SNPVPPROD',
  },
  8: { parameter: 'Concentrated solar power (CSP) power capacity', units: 'W', abbrev: 'CSPPCAP' },
  9: {
    parameter: 'Concentrated solar power (CSP) power production rate',
    units: 'W',
    abbrev: 'CSPPROD',
  },
  // Reserved 10-191
  // Reserved for Local Use 192-254
  255: { parameter: 'Missing', units: '', abbrev: '' },
};

/**
 * GRIB2 - CODE TABLE 4.2: PARAMETER NUMBER BY PRODUCT DISCIPLINE AND PARAMETER CATEGORY
 *
 * **Created**: 12/07/2023
 * **Revised**: 12/07/2023
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)
 *
 * ## Notes
 * - By convention, the flux sign is positive if downward.
 * - When a new parameter is to be added to Code table 4.2 and more than one category applies, the
 * choice of category should be made base on the intended use of product. The discipline and
 * category are an important part of any product definition, so it is possible to have the same
 * parameter name in more than one category. For example, "Water Temperature" in discipline 10
 * (Oceanographic Products), category 4 (sub-surface properties) is used to reporting water
 * temperature in the ocean or open sea, and is not the same as "Water temperature" in discipline
 * 1 (Hydrological Products), category 2 (Inland water and sediment properties) which is used for
 * reporting water temperature in freshwater lakes and rivers.
 *
 * ## Reads as
 * `{ [discipline]: { [param catagory]: { TableCategory } }}`
 */
export const grib2LookupTable42: Record<number, Record<number, Record<number, TableCategory>>> = {
  /** Product Discipline 0 - Meteorological products */
  0: {
    0: grib2LookupTable42_00,
    1: grib2LookupTable42_01,
    2: grib2LookupTable42_02,
    3: grib2LookupTable42_03,
    4: grib2LookupTable42_04,
    5: grib2LookupTable42_05,
    6: grib2LookupTable42_06,
    7: grib2LookupTable42_07,
    13: grib2LookupTable42_013,
    14: grib2LookupTable42_014,
    15: grib2LookupTable42_015,
    16: grib2LookupTable42_016,
    17: grib2LookupTable42_017,
    18: grib2LookupTable42_018,
    19: grib2LookupTable42_019,
    20: grib2LookupTable42_020,
    21: grib2LookupTable42_021,
    22: grib2LookupTable42_022,
    190: grib2LookupTable42_190,
    191: grib2LookupTable42_191,
    // 192-254 Reserved for Local Use
    192: grib2LookupTable42_192,
    255: {},
  },
  /** Product Discipline 1, Hydrologic products */
  1: {
    0: grib2LookupTable42_10,
    1: grib2LookupTable42_11,
    2: grib2LookupTable42_12,
    // 3-191 Reserved
    // 192-254 Reserved for Local Use
    255: {},
  },
  /** Product Discipline 2, Land Surface products */
  2: {
    0: grib2LookupTable42_20,
    1: grib2LookupTable42_21,
    3: grib2LookupTable42_23,
    4: grib2LookupTable42_24,
    5: grib2LookupTable42_25,
    6: grib2LookupTable42_26,
    // 7-191 Reserved
    // 192-254 Reserved for Local Use
    255: {},
  },
  /** Product Discipline 3, Space products */
  3: {
    0: grib2LookupTable42_30,
    1: grib2LookupTable42_31,
    2: grib2LookupTable42_32,
    3: grib2LookupTable42_33,
    4: grib2LookupTable42_34,
    5: grib2LookupTable42_35,
    6: grib2LookupTable42_36,
    // 7-191 Reserved
    // 192-254 Reserved for Local Use
    192: grib2LookupTable42_192,
    255: {},
  },
  /** Product Discipline 4, Space Weather products */
  4: {
    0: grib2LookupTable42_40,
    1: grib2LookupTable42_41,
    2: grib2LookupTable42_42,
    3: grib2LookupTable42_43,
    4: grib2LookupTable42_44,
    5: grib2LookupTable42_45,
    6: grib2LookupTable42_46,
    7: grib2LookupTable42_47,
    8: grib2LookupTable42_48,
    9: grib2LookupTable42_49,
    10: grib2LookupTable42_410,
    // 11-191 Reserved
    // 192-254 Reserved for Local Use
    255: {},
  },
  /** Product Discipline 10, Oceanographic products */
  10: {
    0: grib2LookupTable42_100,
    1: grib2LookupTable42_101,
    2: grib2LookupTable42_102,
    3: grib2LookupTable42_103,
    4: grib2LookupTable42_104,
    191: grib2LookupTable42_10191,
    // 192-254 Reserved for Local Use
    255: {},
  },
  /** Product Discipline 20, Health and Socioeconomic impacts */
  20: {
    0: grib2LookupTable42_2000,
    1: grib2LookupTable42_2001,
    2: grib2LookupTable42_2002,
    3: grib2LookupTable42_2003,
    // 4-191 Reserved
    // 192-254 Reserved for Local Use
    255: {},
  },
};

/**
 * # GRIB2 - CODE TABLE 4.3 - TYPE OF GENERATING PROCESS
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 12
 * - **Revised**: 10/24/2023
 *
 * **Reserved Ranges**:
 * - `22-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)
 *
 * ## Notes
 * 1. Code figures `12` and `13` are intended for cases where code figures `0` and `2` may not sufficiently indicate significant post-processing on initial analysis or forecast output.
 * 2. Analysis increment represents "analysis minus first guess."
 * 3. Initialized analysis increment represents "initialized analysis minus analysis."
 */
export const grib2LookupTable43: Record<number, string> = {
  0: 'Analysis',
  1: 'Initialization',
  2: 'Forecast',
  3: 'Bias Corrected Forecast',
  4: 'Ensemble Forecast',
  5: 'Probability Forecast',
  6: 'Forecast Error',
  7: 'Analysis Error',
  8: 'Observation',
  9: 'Climatological',
  10: 'Probability-Weighted Forecast',
  11: 'Bias-Corrected Ensemble Forecast',
  12: 'Post-processed Analysis',
  13: 'Post-processed Forecast',
  14: 'Nowcast',
  15: 'Hindcast',
  16: 'Physical Retrieval',
  17: 'Regression Analysis',
  18: 'Difference Between Two Forecasts',
  19: 'First guess',
  20: 'Analysis increment',
  21: 'Initialization increment for analysis',
  // 22-191: Reserved
  // 192-254: Reserved for Local Use
  192: 'Forecast Confidence Indicator',
  193: 'Probability-matched Mean',
  194: 'Neighborhood Probability',
  195: 'Bias-Corrected and Downscaled Ensemble Forecast',
  196: 'Perturbed Analysis for Ensemble Initialization',
  197: 'Ensemble Agreement Scale Probability',
  198: 'Post-Processed Deterministic-Expert-Weighted Forecast',
  199: 'Ensemble Forecast Based on Counting',
  200: 'Local Probability-matched Mean',
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.4 - INDICATOR OF UNIT OF TIME RANGE
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 18
 * - **Created**: 05/12/2005
 *
 * **Reserved Ranges**:
 * - `14-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable44: Record<number, string> = {
  0: 'Minute',
  1: 'Hour',
  2: 'Day',
  3: 'Month',
  4: 'Year',
  5: 'Decade (10 Years)',
  6: 'Normal (30 Years)',
  7: 'Century (100 Years)',
  8: 'Reserved',
  9: 'Reserved',
  10: '3 Hours',
  11: '6 Hours',
  12: '12 Hours',
  13: 'Second',
  // 14-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/** Type and Unit categorizing */
export interface TypeAndUnit {
  type: string;
  unit: string;
}

/**
 * # GRIB2 - CODE TABLE 4.5 - FIXED SURFACE TYPES AND UNITS
 *
 * **Details**:
 * - **Section**: 4
 * - **Octets**: 23 and 29
 * - **Revised**: 12/07/2023
 *
 * **Reserved Ranges**:
 * - `28-29`: Reserved
 * - `36-99`: Reserved
 * - `120-149`: Reserved
 * - `153-159`: Reserved
 * - `190-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)
 *
 * ## Notes
 * 1. The Eta vertical coordinate system involves normalizing the pressure at some point on a specific level by the mean sea level pressure at that point.
 * 2. Hybrid height level can be defined as: z(k)=A(k)+B(k)*orog.
 * 3. Hybrid pressure level is defined as: p(k)=A(k)+B(k)*sp.
 * 4. Sigma height level is the height-based terrain-following coordinate.
 * 5. The soil level represents a model level with varying depths provided by another GRIB message.
 * 6. The sea-ice level represents varying depths across the model domain.
 * 7. Ocean level types are defined by property differences from the near-surface.
 * 8. This level differs from entry 13, which is vertically accumulated from the surface.
 */
export const grib2LookupTable45: Record<number, TypeAndUnit> = {
  0: { type: 'Reserved', unit: '' },
  1: { type: 'Ground or Water Surface', unit: '' },
  2: { type: 'Cloud Base Level', unit: '' },
  3: { type: 'Level of Cloud Tops', unit: '' },
  4: { type: 'Level of 0°C Isotherm', unit: '°C' },
  5: { type: 'Level of Adiabatic Condensation Lifted from the Surface', unit: '' },
  6: { type: 'Maximum Wind Level', unit: '' },
  7: { type: 'Tropopause', unit: '' },
  8: { type: 'Nominal Top of the Atmosphere', unit: '' },
  9: { type: 'Sea Bottom', unit: '' },
  10: { type: 'Entire Atmosphere', unit: '' },
  11: { type: 'Cumulonimbus Base (CB)', unit: 'm' },
  12: { type: 'Cumulonimbus Top (CT)', unit: 'm' },
  13: {
    type: 'Lowest level where vertically integrated cloud cover exceeds the specified percentage',
    unit: '%',
  },
  14: { type: 'Level of free convection (LFC)', unit: '' },
  15: { type: 'Convection condensation level (CCL)', unit: '' },
  16: { type: 'Level of neutral buoyancy or equilibrium (LNB)', unit: '' },
  17: { type: 'Departure level of the most unstable parcel of air (MUDL)', unit: '' },
  18: {
    type: 'Departure level of a mixed layer parcel of air with specified layer depth',
    unit: 'Pa',
  },
  19: { type: 'Lowest level where cloud cover exceeds the specified percentage', unit: '%' },
  20: { type: 'Isothermal Level', unit: 'K' },
  21: { type: 'Lowest level where mass density exceeds the specified value', unit: 'kg m-3' },
  22: { type: 'Highest level where mass density exceeds the specified value', unit: 'kg m-3' },
  23: { type: 'Lowest level where air concentration exceeds the specified value', unit: 'Bq m-3' },
  24: { type: 'Highest level where air concentration exceeds the specified value', unit: 'Bq m-3' },
  25: { type: 'Highest level where radar reflectivity exceeds the specified value', unit: 'dBZ' },
  26: { type: 'Convective cloud layer base', unit: 'm' },
  27: { type: 'Convective cloud layer top', unit: 'm' },
  // 28-29: Reserved
  28: { type: 'Reserved', unit: '' },
  29: { type: 'Reserved', unit: '' },
  30: { type: 'Specified radius from the centre of the Sun', unit: 'm' },
  31: { type: 'Solar photosphere', unit: '' },
  32: { type: 'Ionospheric D-region level', unit: '' },
  33: { type: 'Ionospheric E-region level', unit: '' },
  34: { type: 'Ionospheric F1-region level', unit: '' },
  35: { type: 'Ionospheric F2-region level', unit: '' },
  // 36-99: Reserved
  100: { type: 'Isobaric Surface', unit: 'Pa' },
  101: { type: 'Mean Sea Level', unit: '' },
  102: { type: 'Specific Altitude Above Mean Sea Level', unit: 'm' },
  103: { type: 'Specified Height Level Above Ground', unit: 'm' },
  104: { type: 'Sigma Level', unit: '' },
  105: { type: 'Hybrid Level', unit: '' },
  106: { type: 'Depth Below Land Surface', unit: 'm' },
  107: { type: 'Isentropic (theta) Level', unit: 'K' },
  108: { type: 'Level at Specified Pressure Difference from Ground to Level', unit: 'Pa' },
  109: { type: 'Potential Vorticity Surface', unit: 'K m² kg⁻¹ s⁻¹' },
  110: { type: 'Reserved', unit: '' },
  111: { type: 'Eta Level', unit: '' },
  112: { type: 'Reserved', unit: '' },
  113: { type: 'Logarithmic Hybrid Level', unit: '' },
  114: { type: 'Snow Level', unit: '' }, // 'Numeric' interpreted as no specific unit
  115: { type: 'Sigma height level', unit: '' },
  116: { type: 'Reserved', unit: '' },
  117: { type: 'Mixed Layer Depth', unit: 'm' },
  118: { type: 'Hybrid Height Level', unit: '' },
  119: { type: 'Hybrid Pressure Level', unit: '' },
  // 120-149: Reserved
  150: { type: 'Generalized Vertical Height Coordinate', unit: '' },
  151: { type: 'Soil level', unit: '' }, // 'Numeric' interpreted as no specific unit
  152: { type: 'Sea-ice level', unit: '' }, // 'Numeric' interpreted as no specific unit
  // 153-159: Reserved
  160: { type: 'Depth Below Sea Level', unit: 'm' },
  161: { type: 'Depth Below Water Surface', unit: 'm' },
  162: { type: 'Lake or River Bottom', unit: '' },
  163: { type: 'Bottom Of Sediment Layer', unit: '' },
  164: { type: 'Bottom Of Thermally Active Sediment Layer', unit: '' },
  165: { type: 'Bottom Of Sediment Layer Penetrated By Thermal Wave', unit: '' },
  166: { type: 'Mixing Layer', unit: '' },
  167: { type: 'Bottom of Root Zone', unit: '' },
  168: { type: 'Ocean Model Level', unit: '' }, // 'Numeric' interpreted as no specific unit
  169: {
    type: 'Ocean level defined by water density (sigma-theta) difference from near-surface to level',
    unit: 'kg m-3',
  },
  170: {
    type: 'Ocean level defined by water potential temperature difference from near-surface to level',
    unit: 'K',
  },
  171: {
    type: 'Ocean level defined by vertical eddy diffusivity difference from near-surface to level',
    unit: 'm² s-1',
  },
  172: {
    type: 'Ocean level defined by water density (rho) difference from near-surface to level',
    unit: 'm',
  },
  173: { type: 'Top of Snow Over Sea Ice on Sea, Lake or River', unit: '' },
  174: { type: 'Top Surface of Ice on Sea, Lake or River', unit: '' },
  175: { type: 'Top Surface of Ice, under Snow, on Sea, Lake or River', unit: '' },
  176: { type: 'Bottom Surface (underside) Ice on Sea, Lake or River', unit: '' },
  177: { type: 'Deep Soil (of indefinite depth)', unit: '' },
  178: { type: 'Reserved', unit: '' },
  179: { type: 'Top Surface of Glacier Ice and Inland Ice', unit: '' },
  180: { type: 'Deep Inland or Glacier Ice (of indefinite depth)', unit: '' },
  181: { type: 'Grid Tile Land Fraction as a Model Surface', unit: '' },
  182: { type: 'Grid Tile Water Fraction as a Model Surface', unit: '' },
  183: { type: 'Grid Tile Ice Fraction on Sea, Lake or River as a Model Surface', unit: '' },
  184: { type: 'Grid Tile Glacier Ice and Inland Ice Fraction as a Model Surface', unit: '' },
  185: { type: 'Roof Level', unit: '' },
  186: { type: 'Wall level', unit: '' },
  187: { type: 'Road Level', unit: '' },
  188: { type: 'Melt pond Top Surface', unit: '' },
  189: { type: 'Melt Pond Bottom Surface', unit: '' },
  // 190-191: Reserved
  // 192-254: Reserved for Local Use
  200: { type: 'Entire atmosphere (considered as a single layer)', unit: '' },
  201: { type: 'Entire ocean (considered as a single layer)', unit: '' },
  202: { type: 'Reserved for Local Use', unit: '' },
  203: { type: 'Reserved for Local Use', unit: '' },
  204: { type: 'Highest tropospheric freezing level', unit: '' },
  205: { type: 'Reserved for Local Use', unit: '' },
  206: { type: 'Grid scale cloud bottom level', unit: '' },
  207: { type: 'Grid scale cloud top level', unit: '' },
  208: { type: 'Reserved for Local Use', unit: '' },
  209: { type: 'Boundary layer cloud bottom level', unit: '' },
  210: { type: 'Boundary layer cloud top level', unit: '' },
  211: { type: 'Boundary layer cloud layer', unit: '' },
  212: { type: 'Low cloud bottom level', unit: '' },
  213: { type: 'Low cloud top level', unit: '' },
  214: { type: 'Low cloud layer', unit: '' },
  215: { type: 'Cloud ceiling', unit: '' },
  216: { type: 'Effective Layer Top Level', unit: 'm' },
  217: { type: 'Effective Layer Bottom Level', unit: 'm' },
  218: { type: 'Effective Layer', unit: 'm' },
  219: { type: 'Reserved for Local Use', unit: '' },
  220: { type: 'Planetary Boundary Layer', unit: '' },
  221: { type: 'Layer Between Two Hybrid Levels', unit: '' },
  222: { type: 'Middle cloud bottom level', unit: '' },
  223: { type: 'Middle cloud top level', unit: '' },
  224: { type: 'Middle cloud layer', unit: '' },
  225: { type: 'Reserved for Local Use', unit: '' },
  226: { type: 'Reserved for Local Use', unit: '' },
  227: { type: 'Reserved for Local Use', unit: '' },
  228: { type: 'Reserved for Local Use', unit: '' },
  229: { type: 'Reserved for Local Use', unit: '' },
  230: { type: 'Reserved for Local Use', unit: '' },
  231: { type: 'Reserved for Local Use', unit: '' },
  232: { type: 'High cloud bottom level', unit: '' },
  233: { type: 'High cloud top level', unit: '' },
  234: { type: 'High cloud layer', unit: '' },
  235: { type: 'Ocean Isotherm Level (1/10 °C)', unit: '°C' },
  236: { type: 'Layer between two depths below ocean surface', unit: '' },
  237: { type: 'Bottom of Ocean Mixed Layer', unit: 'm' },
  238: { type: 'Bottom of Ocean Isothermal Layer', unit: 'm' },
  239: { type: 'Layer Ocean Surface and 26°C Ocean Isothermal Level', unit: '' },
  240: { type: 'Ocean Mixed Layer', unit: '' },
  241: { type: 'Ordered Sequence of Data', unit: '' },
  242: { type: 'Convective cloud bottom level', unit: '' },
  243: { type: 'Convective cloud top level', unit: '' },
  244: { type: 'Convective cloud layer', unit: '' },
  245: { type: 'Lowest level of the wet bulb zero', unit: '' },
  246: { type: 'Maximum equivalent potential temperature level', unit: '' },
  247: { type: 'Equilibrium level', unit: '' },
  248: { type: 'Shallow convective cloud bottom level', unit: '' },
  249: { type: 'Shallow convective cloud top level', unit: '' },
  250: { type: 'Reserved for Local Use', unit: '' },
  251: { type: 'Deep convective cloud bottom level', unit: '' },
  252: { type: 'Deep convective cloud top level', unit: '' },
  253: { type: 'Lowest bottom level of supercooled liquid water layer', unit: '' },
  254: { type: 'Highest top level of supercooled liquid water layer', unit: '' },
  255: { type: 'Missing', unit: '' },
};

/**
 * # GRIB2 - CODE TABLE 4.6 - TYPE OF ENSEMBLE FORECAST
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 35 (for product templates 1 and 11)
 * - **Revised**: 07/22/2010
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-6.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable46: Record<number, string> = {
  0: 'Unperturbed High-Resolution Control Forecast',
  1: 'Unperturbed Low-Resolution Control Forecast',
  2: 'Negatively Perturbed Forecast',
  3: 'Positively Perturbed Forecast',
  4: 'Multi-Model Forecast',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  192: 'Perturbed Ensemble Member',
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.7 - DERIVED FORECAST
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 35 (for product templates 2-4 and 12-14)
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `11-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml)
 *
 * ## Notes
 * 1. Large anomaly index is defined as:
 *    `{(number of members whose anomaly > 0.5 * SD) - (number of members whose anomaly < -0.5 * SD)} / (number of members)`.
 *    SD is the observed climatological standard deviation.
 * 2. The reference for "minimum of all ensemble members" and "maximum of all ensemble members" is the set of ensemble members
 *    and not a time interval; this differs from Product Definition Template 4.8.
 */
export const grib2LookupTable47: Record<number, string> = {
  0: 'Unweighted Mean of All Members',
  1: 'Weighted Mean of All Members',
  2: 'Standard Deviation with respect to Cluster Mean',
  3: 'Standard Deviation with respect to Cluster Mean, Normalized',
  4: 'Spread of All Members',
  5: 'Large Anomaly Index of All Members',
  6: 'Unweighted Mean of the Cluster Members',
  7: 'Interquartile Range (Range between the 25th and 75th quantile)',
  8: 'Minimum Of All Ensemble Members',
  9: 'Maximum Of All Ensemble Members',
  10: 'Variance of all ensemble members',
  // 11-191: Reserved
  // 192-254: Reserved for Local Use
  192: 'Unweighted Mode of All Members',
  193: 'Percentile value (10%) of All Members',
  194: 'Percentile value (50%) of All Members',
  195: 'Percentile value (90%) of All Members',
  196: 'Statistically decided weights for each ensemble member',
  197: 'Climate Percentile (percentile values from climate distribution)',
  198: 'Deviation of Ensemble Mean from Daily Climatology',
  199: 'Extreme Forecast Index',
  200: 'Equally Weighted Mean',
  201: 'Percentile value (5%) of All Members',
  202: 'Percentile value (25%) of All Members',
  203: 'Percentile value (75%) of All Members',
  204: 'Percentile value (95%) of All Members',
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.8 - CLUSTERING METHOD
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 41 (for product templates 3-4 and 13-14)
 * - **Created**: 05/12/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-8.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable48: Record<number, string> = {
  0: 'Anomoly Correlation',
  1: 'Root Mean Square',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.9 - PROBABILITY TYPE
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 37 (for product templates 5 and 9)
 * - **Revised**: 07/15/2024
 *
 * **Reserved Ranges**:
 * - `10-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-9.shtml)
 *
 * ## Notes
 * 1. Above normal, near normal, and below normal are defined as three equiprobable categories based on climatology at each point.
 *    The methodology and reference climatology are unspecified and should be documented by the data producer.
 * 2. Product definition templates using this table may include octets for lower and upper limits. For categorical probabilities
 *    (e.g., below, near, or above normal), these octets are set to "all ones" (missing).
 * 3. Scale Factor and Scaled Values for lower/upper limits must be set to missing for entry `9`.
 *    This is primarily for categorical boolean counts.
 */
export const grib2LookupTable49: Record<number, string> = {
  0: 'Probability of event below lower limit',
  1: 'Probability of event above upper limit',
  2: 'Probability of event between upper and lower limits (range includes lower limit but not the upper limit)',
  3: 'Probability of event above lower limit',
  4: 'Probability of event below upper limit',
  5: 'Probability of event equal to lower limit',
  6: 'Probability of event in above normal category',
  7: 'Probability of event in near normal category',
  8: 'Probability of event in below normal category',
  9: 'Probability based on counts of categorical boolean',
  // 10-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.10 - TYPE OF STATISTICAL PROCESSING
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 47 (for product template 8)
 * - **Revised**: 10/24/2023
 *
 * **Reserved Ranges**:
 * - `14-99`: Reserved
 * - `103-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-10.shtml)
 *
 * ## Notes
 * 1. The original data value has units of Code Table 4.2 multiplied by seconds, unless otherwise specified.
 * 2. Covariance (Code 7) has squared units of Code Table 4.2.
 * 3. Ratio (Code 9) is a non-dimensional number without units.
 * 4. For code number 102, the drought index is defined by discipline 0, parameter category 22, and the corresponding parameter number.
 */
export const grib2LookupTable410: Record<number, string> = {
  0: 'Average',
  1: 'Accumulation',
  2: 'Maximum',
  3: 'Minimum',
  4: 'Difference (value at the end of the time range minus value at the beginning)',
  5: 'Root Mean Square',
  6: 'Standard Deviation',
  7: 'Covariance (temporal variance)',
  8: 'Difference (value at the beginning of the time range minus value at the end)',
  9: 'Ratio',
  10: 'Standardized Anomaly',
  11: 'Summation',
  12: 'Return period',
  13: 'Median',
  // 14-99: Reserved
  100: 'Severity',
  101: 'Mode',
  102: 'Index processing',
  // 103-191: Reserved
  // 192-254: Reserved for Local Use
  192: 'Climatological Mean Value',
  193: 'Average of N forecasts (or initialized analyses)',
  194: 'Average of N uninitialized analyses',
  195: 'Average of forecast accumulations (24-hour intervals)',
  196: 'Average of successive forecast accumulations',
  197: 'Average of forecast averages (24-hour intervals)',
  198: 'Average of successive forecast averages',
  199: 'Climatological Average of N analyses',
  200: 'Climatological Average of N forecasts',
  201: 'Climatological Root Mean Square difference between N forecasts and their verifying analyses',
  202: 'Climatological Standard Deviation of N forecasts',
  203: 'Climatological Standard Deviation of N analyses',
  204: 'Average of forecast accumulations (6-hour intervals)',
  205: 'Average of forecast averages (6-hour intervals)',
  206: 'Average of forecast accumulations (12-hour intervals)',
  207: 'Average of forecast averages (12-hour intervals)',
  208: 'Variance',
  209: 'Coefficient',
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.11 - TYPE OF TIME INTERVALS
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 48 (for product templates 8)
 * - **Revised**: 12/21/2011
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-11.shtml)
 *
 * ## Notes
 * 1. Code figure `5` applies when a single time subinterval is used to calculate the statistically processed field.
 *    The exact starting and ending times of the subinterval are not specified but are inclusively within the overall interval.
 */
export const grib2LookupTable411: Record<number, string> = {
  0: 'Reserved',
  1: 'Successive times processed have same forecast time, start time of forecast is incremented.',
  2: 'Successive times processed have same start time of forecast, forecast time is incremented.',
  3: 'Successive times processed have start time of forecast incremented and forecast time decremented so that valid time remains constant.',
  4: 'Successive times processed have start time of forecast decremented and forecast time incremented so that valid time remains constant.',
  5: 'Floating subinterval of time between forecast time and end of overall time interval.',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.12 - OPERATING MODE
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 31 (for product template 20)
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `3-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-12.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable412: Record<number, string> = {
  0: 'Maintenance Mode',
  1: 'Clear Air',
  2: 'Precipitation',
  // 3-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.13 - QUALITY CONTROL INDICATOR
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 33 (for Product Definition Template 20)
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-13.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable413: Record<number, string> = {
  0: 'No Quality Control Applied',
  1: 'Quality Control Applied',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.14 - CLUTTER FILTER INDICATOR
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 34 (for product template 20)
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-14.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable414: Record<number, string> = {
  0: 'No Clutter Filter Used',
  1: 'Clutter Filter Used',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.15 - TYPE OF SPATIAL PROCESSING
 *
 * **Details**:
 * - **Created**: 12/10/2009
 *
 * **Reserved Ranges**:
 * - `7-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-15.shtml)
 *
 * ## Notes
 * 1. This method assumes that each field represents box averages/maxima/minima extending halfway to neighboring grid points.
 * 2. Budget interpolation quasi-conserves area averages, useful for budget fields such as precipitation. It averages bilinearly
 *    interpolated values within a square array of points distributed in each output grid box.
 * 3. Neighbor-budget interpolation performs a budget interpolation at the grid point nearest to the nominal grid point.
 */
export const grib2LookupTable415: Record<number, string> = {
  0: 'Data is calculated directly from the source grid with no interpolation',
  1: 'Bilinear interpolation using the 4 source grid-point values surrounding the nominal grid-point',
  2: 'Bicubic interpolation using the 4 source grid-point values surrounding the nominal grid-point',
  3: 'Using the value from the source grid-point which is nearest to the nominal grid-point',
  4: 'Budget interpolation using the 4 source grid-point values surrounding the nominal grid-point',
  5: 'Spectral interpolation using the 4 source grid-point values surrounding the nominal grid-point',
  6: 'Neighbor-budget interpolation using the 4 source grid-point values surrounding the nominal grid-point',
  // 7-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.16 - QUALITY VALUE ASSOCIATED WITH PARAMETER
 *
 * **Details**:
 * - **Section**: 4
 * - **Octet**: 14 (for product templates 35)
 * - **Revised**: 07/01/2022
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-16.shtml)
 *
 * ## Notes
 * 1. When a non-missing value is used, it represents a quality value associated with the parameter defined by octets 10 and 11 of the product definition template.
 * 2. For "Confidence index" (Code 0), the value ranges from 0 (no confidence) to 1 (maximal confidence) and is non-dimensional.
 * 3. "Quality indicator" (Code 1) values are defined by Code Table 4.244.
 * 4. "Correlation of Product with used Calibration Product" (Code 2) is a non-dimensional value without units.
 * 5. For "Standard deviation" (Code 3) and "Random error" (Code 4), the value uses the same units as the parameter defined by octets 10 and 11.
 */
export const grib2LookupTable416: Record<number, string> = {
  0: 'Confidence index',
  1: 'Quality indicator',
  2: 'Correlation of Product with used Calibration Product',
  3: 'Standard deviation',
  4: 'Random error',
  5: 'Probability',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.91 - TYPE OF INTERVAL
 *
 * **Details**:
 * - **Created**: 12/21/2011
 *
 * **Reserved Ranges**:
 * - `12-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-91.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable491: Record<number, string> = {
  0: 'Smaller than first limit',
  1: 'Greater than second limit',
  2: 'Between first and second limit (includes first limit but not the second)',
  3: 'Greater than first limit',
  4: 'Smaller than second limit',
  5: 'Smaller or equal first limit',
  6: 'Greater or equal second limit',
  7: 'Between first and second limit (includes both first and second limits)',
  8: 'Greater or equal first limit',
  9: 'Smaller or equal second limit',
  10: 'Between first and second limit (includes second limit but not the first)',
  11: 'Equal to first limit',
  // 12-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.100 - TYPE OF REFERENCE DATASET
 *
 * **Details**:
 * - **Created**: 10/24/2023
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-100.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4100: Record<number, string> = {
  0: 'Analysis',
  1: 'Forecast',
  2: 'Reforecast (Hindcast)',
  3: 'Reanalysis',
  4: 'Climate Projection',
  5: 'Gridded observations',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.101 - TYPE OF RELATIONSHIP TO REFERENCE DATASET
 *
 * **Details**:
 * - **Revised**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `4-19`: Reserved
 * - `24-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-101.shtml)
 *
 * ## Notes
 * 1. No additional parameter is needed for entries `0` and `1` (NA=0).
 * 2. Entry `2` (Significance) requires a confidence interval (NA=1).
 * 3. Entry `20` (EFI) is defined in [https://doi.org/10.1256/qj.02.152](https://doi.org/10.1256/qj.02.152). No additional parameter is needed.
 * 4. Entry `21` (SOT) requires lower and upper quantiles to be defined (NA=2).
 * 5. Entry `22` (Anomaly of probabilities) applies to templates `4.112` and `4.123`.
 * 6. Entry `23` (Standardized Drought Index) follows definitions from the WMO Handbook on Drought Indicators and Indices.
 */
export const grib2LookupTable4101: Record<number, string> = {
  0: 'Anomaly',
  1: 'Standardized Anomaly',
  2: 'Significance (Wilcoxon-Mann-Whitney)',
  3: 'Climatology',
  // 4-19: Reserved
  20: 'Extreme Forecast Index (EFI)',
  21: 'Shift of Tails (SOT)',
  22: 'Anomaly of probabilities',
  23: 'Standardized Drought Index',
  // 24-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.102 - STATISTICAL PROCESSING OF REFERENCE PERIOD
 *
 * **Details**:
 * - **Revised**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `5-19`: Reserved
 * - `32-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-102.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4102: Record<number, string> = {
  0: 'Average',
  1: 'Accumulation',
  2: 'Maximum',
  3: 'Minimum',
  4: 'Median',
  // 5-19: Reserved
  20: 'Model Climate',
  21: 'Index based on normal distribution',
  22: 'Index based on log-normal distribution',
  23: 'Index based on generalised log-normal distribution',
  24: 'Index based on gamma distribution',
  25: 'Index based on logistic distribution',
  26: 'Index based on log-logistic distribution',
  27: 'Index based on generalised logistic distribution',
  28: 'Index based on Weibull distribution',
  29: 'Index based on generalised extreme value distribution',
  30: 'Index based on Pearson III distribution',
  31: 'Index based on empirical distribution',
  // 32-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.103 - SPATIAL VICINITY TYPE
 *
 * **Details**:
 * - **Created**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-103.shtml)
 *
 * ## Notes
 * 1. The following additional arguments must be specified:
 *    - **Circle**: 1 argument for the radius in meters.
 *    - **Rectangle**: 2 arguments for length: 1. west-east and 2. south-north, in meters.
 *    - **Square**: 1 argument for the length of equal-length sides in meters.
 *    - **Wedge**: 3 arguments: 1. radius in meters, 2. start angle, and 3. end angle in degrees.
 *      Angles are measured counter-clockwise from 0° along the positive west-east axis.
 *    - **Span of grid cells**: 2 arguments for grid cells: 1. west-east span `i +/- x`
 *      and 2. south-north span `j +/- y`.
 */
export const grib2LookupTable4103: Record<number, string> = {
  0: 'Circle [m]',
  1: 'Rectangle [m, m]',
  2: 'Square [m]',
  3: 'Wedge [m, degree, degree]',
  4: 'Span of grid boxes centered around grid box i,j [x, y]',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.104 - SPATIAL AND TEMPORAL VICINITY PROCESSING
 *
 * **Details**:
 * - **Created**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `1`: Reserved
 * - `5`: Reserved
 * - `7-10`: Reserved
 * - `12-189`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-104.shtml)
 *
 * ## Notes
 * 1. The option **Quantile** (Code 190) requires two additional arguments:
 *    - The total number of quantiles.
 *    - The quantile value.
 */
export const grib2LookupTable4104: Record<number, string> = {
  0: 'Average',
  1: 'Reserved',
  2: 'Maximum',
  3: 'Minimum',
  4: 'Range',
  5: 'Reserved',
  6: 'Standard deviation',
  // 7-10: Reserved
  11: 'Sum',
  // 12-189: Reserved
  190: 'Quantile',
  191: 'Categorical (boolean)',
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.105 - SPATIAL AND TEMPORAL VICINITY MISSING DATA
 *
 * **Details**:
 * - **Created**: 07/12/2024
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-105.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4105: Record<number, string> = {
  0: 'Ignore missing data',
  1: 'No data',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.201 - PRECIPITATION TYPE
 *
 * **Details**:
 * - **Revised**: 05/29/2019
 *
 * **Reserved Ranges**:
 * - `13-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-201.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4201: Record<number, string> = {
  0: 'Reserved',
  1: 'Rain',
  2: 'Thunderstorm',
  3: 'Freezing Rain',
  4: 'Mixed/Ice',
  5: 'Snow',
  6: 'Wet Snow',
  7: 'Mixture of Rain and Snow',
  8: 'Ice Pellets',
  9: 'Graupel',
  10: 'Hail',
  11: 'Drizzle',
  12: 'Freezing Drizzle',
  // 13-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.202 - PRECIPITABLE WATER CATEGORY
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `0-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-202.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4202: Record<number, string> = {
  // 0-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.203 - CLOUD TYPE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `21-190`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `191`: Unknown
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-203.shtml)
 *
 * ## Notes
 * 1. Code figures `11-20` indicate all four layers were used and ground-based fog is below the lowest layer.
 */
export const grib2LookupTable4203: Record<number, string> = {
  0: 'Clear',
  1: 'Cumulonimbus',
  2: 'Stratus',
  3: 'Stratocumulus',
  4: 'Cumulus',
  5: 'Altostratus',
  6: 'Nimbostratus',
  7: 'Altocumulus',
  8: 'Cirrostratus',
  9: 'Cirrorcumulus',
  10: 'Cirrus',
  11: 'Cumulonimbus - ground-based fog beneath the lowest layer',
  12: 'Stratus - ground-based fog beneath the lowest layer',
  13: 'Stratocumulus - ground-based fog beneath the lowest layer',
  14: 'Cumulus - ground-based fog beneath the lowest layer',
  15: 'Altostratus - ground-based fog beneath the lowest layer',
  16: 'Nimbostratus - ground-based fog beneath the lowest layer',
  17: 'Altocumulus - ground-based fog beneath the lowest layer',
  18: 'Cirrostratus - ground-based fog beneath the lowest layer',
  19: 'Cirrorcumulus - ground-based fog beneath the lowest layer',
  20: 'Cirrus - ground-based fog beneath the lowest layer',
  // 21-190: Reserved
  191: 'Unknown',
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.204 - THUNDERSTORM COVERAGE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-204.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4204: Record<number, string> = {
  0: 'None',
  1: 'Isolated (1-2%)',
  2: 'Few (3-5%)',
  3: 'Scattered (16-45%)',
  4: 'Numerous (>45%)',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.205 - PRESENCE OF AEROSOL
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-205.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4205: Record<number, string> = {
  0: 'Aerosol not present',
  1: 'Aerosol present',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.206 - VOLCANIC ASH
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-206.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4206: Record<number, string> = {
  0: 'Not Present',
  1: 'Present',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.207 - ICING
 *
 * **Details**:
 * - **Revised**: 04/22/2009
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-207.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4207: Record<number, string> = {
  0: 'None',
  1: 'Light',
  2: 'Moderate',
  3: 'Severe',
  4: 'Trace',
  5: 'Heavy',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.208 - TURBULENCE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-208.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4208: Record<number, string> = {
  0: 'None',
  1: 'Light',
  2: 'Moderate',
  3: 'Severe',
  4: 'Extreme',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.209 - PLANETARY BOUNDARY-LAYER REGIME
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-209.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4209: Record<number, string> = {
  0: 'Reserved',
  1: 'Stable',
  2: 'Mechanically-Driven Turbulence',
  3: 'Force Convection',
  4: 'Free Convection',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.210 - CONTRAIL INTENSITY
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-210.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4210: Record<number, string> = {
  0: 'Contrail Not Present',
  1: 'Contrail Present',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.211 - CONTRAIL ENGINE TYPE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `3-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-211.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4211: Record<number, string> = {
  0: 'Low Bypass',
  1: 'High Bypass',
  2: 'Non-Bypass',
  // 3-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.212 - LAND USE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `14-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-212.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4212: Record<number, string> = {
  0: 'Reserved',
  1: 'Urban Land',
  2: 'Agricultural',
  3: 'Range Land',
  4: 'Deciduous Forest',
  5: 'Coniferous Forest',
  6: 'Forest/Wetland',
  7: 'Water',
  8: 'Wetlands',
  9: 'Desert',
  10: 'Tundra',
  11: 'Ice',
  12: 'Tropical Forest',
  13: 'Savannah',
  // 14-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.213 - SOIL TYPE
 *
 * **Details**:
 * - **Revised**: 07/16/2013
 *
 * **Reserved Ranges**:
 * - `12-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-213.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4213: Record<number, string> = {
  0: 'Reserved',
  1: 'Sand',
  2: 'Loamy Sand',
  3: 'Sandy Loam',
  4: 'Silt Loam',
  5: 'Organic',
  6: 'Sandy Clay Loam',
  7: 'Silt Clay Loam',
  8: 'Clay Loam',
  9: 'Sandy Clay',
  10: 'Silty Clay',
  11: 'Clay',
  // 12-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.214 - ENVIRONMENTAL FACTOR QUALIFIER
 *
 * **Details**:
 * - **Created**: 10/24/2023
 *
 * **Reserved Ranges**:
 * - `6-190`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `191`: Unknown
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-214.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4214: Record<number, string> = {
  0: 'Worst',
  1: 'Very poor',
  2: 'Poor',
  3: 'Average',
  4: 'Good',
  5: 'Excellent',
  // 6-190: Reserved
  191: 'Unknown',
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.215 - REMOTELY-SENSED SNOW COVERAGE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `0-49`: Reserved
 * - `51-99`: Reserved
 * - `101-249`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `50`: No-Snow/No-Cloud
 * - `100`: Clouds
 * - `250`: Snow
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-215.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4215: Record<number, string> = {
  // 0-49: Reserved
  50: 'No-Snow/No-Cloud',
  // 51-99: Reserved
  100: 'Clouds',
  // 101-249: Reserved
  250: 'Snow',
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.216 - ELEVATION OF SNOW COVERED TERRAIN
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `91-253`: Reserved
 *
 * **Special Values**:
 * - `0-90`: Elevation in increments of 100 m
 * - `254`: Clouds
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-216.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4216: Record<number, string> = {
  // 0-90: Elevation in increments of 100 m
  254: 'Clouds',
  // 91-253: Reserved
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.217 - CLOUD MASK TYPE
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-217.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4217: Record<number, string> = {
  0: 'Clear over water',
  1: 'Clear over land',
  2: 'Cloud',
  3: 'No data',
  // 4-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.218 - PIXEL SCENE TYPE
 *
 * **Details**:
 * - **Revised**: 05/29/2019
 *
 * **Reserved Ranges**:
 * - `25-96`: Reserved
 * - `113-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-218.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4218: Record<number, string> = {
  0: 'No Scene Identified',
  1: 'Green Needle-Leafed Forest',
  2: 'Green Broad-Leafed Forest',
  3: 'Deciduous Needle-Leafed Forest',
  4: 'Deciduous Broad-Leafed Forest',
  5: 'Deciduous Mixed Forest',
  6: 'Closed Shrub-Land',
  7: 'Open Shrub-Land',
  8: 'Woody Savannah',
  9: 'Savannah',
  10: 'Grassland',
  11: 'Permanent Wetland',
  12: 'Cropland',
  13: 'Urban',
  14: 'Vegetation / Crops',
  15: 'Permanent Snow / Ice',
  16: 'Barren Desert',
  17: 'Water Bodies',
  18: 'Tundra',
  19: 'Warm Liquid Water Cloud',
  20: 'Supercooled Liquid Water Cloud',
  21: 'Mixed Phase Cloud',
  22: 'Optically Thin Ice Cloud',
  23: 'Optically Thick Ice Cloud',
  24: 'Multi-Layeblack Cloud',
  // 25-96: Reserved
  97: 'Snow / Ice on Land',
  98: 'Snow / Ice on Water',
  99: 'Sun-Glint',
  100: 'General Cloud',
  101: 'Low Cloud / Fog / Stratus',
  102: 'Low Cloud / Stratocumulus',
  103: 'Low Cloud / Unknown Type',
  104: 'Medium Cloud / Nimbostratus',
  105: 'Medium Cloud / Altostratus',
  106: 'Medium Cloud / Unknown Type',
  107: 'High Cloud / Cumulus',
  108: 'High Cloud / Cirrus',
  109: 'High Cloud / Unknown Type',
  110: 'Unknown Cloud Type',
  111: 'Single layer water cloud',
  112: 'Single layer ice cloud',
  // 113-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.219 - CLOUD TOP HEIGHT QUALITY INDICATOR
 *
 * **Details**:
 * - **Created**: 12/07/2010
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-219.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4219: Record<number, string> = {
  0: 'Nominal Cloud Top Height Quality',
  1: 'Fog In Segment',
  2: 'Poor Quality Height Estimation',
  3: 'Fog In Segment and Poor Quality Height Estimation',
  // 4-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.220 - HORIZONTAL DIMENSION PROCESSED
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-220.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4220: Record<number, string> = {
  0: 'Latitude',
  1: 'Longitude',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.221 - TREATMENT OF MISSING DATA
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-221.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4221: Record<number, string> = {
  0: 'Not included',
  1: 'Extrapolated',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.222 - CATEGORICAL RESULT
 *
 * **Details**:
 * - **Revised**: 07/16/2013
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-222.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4222: Record<number, string> = {
  0: 'No',
  1: 'Yes',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.223 - FIRE DETECTION INDICATOR
 *
 * **Details**:
 * - **Created**: 11/05/2007
 *
 * **Reserved Ranges**:
 * - `4-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-223.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4223: Record<number, string> = {
  0: 'No Fire Detected',
  1: 'Possible Fire Detected',
  2: 'Probable Fire Detected',
  3: 'Missing',
  // 4-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.224 - CATEGORICAL OUTLOOK
 *
 * **Details**:
 * - **Created**: 12/21/2011
 *
 * **Reserved Ranges**:
 * - `1`: Reserved
 * - `3`: Reserved
 * - `5`: Reserved
 * - `7`: Reserved
 * - `9-10`: Reserved
 * - `12-13`: Reserved
 * - `15-17`: Reserved
 * - `19-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-224.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4224: Record<number, string> = {
  0: 'No Risk Area',
  2: 'General Thunderstorm Risk Area',
  4: 'Slight Risk Area',
  6: 'Moderate Risk Area',
  8: 'High Risk Area',
  11: 'Dry Thunderstorm (Dry Lightning) Risk Area',
  14: 'Critical Risk Area',
  18: 'Extremely Critical Risk Area',
  // 1, 3, 5, 7, 9-10, 12-13, 15-17, 19-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

// TODO: GRIB2 - CODE TABLE 4.225 (https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-225.shtml)

/**
 * # GRIB2 - CODE TABLE 4.227 - ICING SCENARIO (Weather/Cloud Classification)
 *
 * **Details**:
 * - **Created**: 04/09/2013
 *
 * **Reserved Ranges**:
 * - `5-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-227.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4227: Record<number, string> = {
  0: 'None',
  1: 'General',
  2: 'Convective',
  3: 'Stratiform',
  4: 'Freezing',
  // 5-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.228 - ICING SEVERITY
 *
 * **Details**:
 * - **Created**: 01/19/2022
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-228.shtml)
 *
 * ## Notes
 * None.
 */
export const grib2LookupTable4228: Record<number, string> = {
  0: 'None',
  1: 'Trace',
  2: 'Light',
  3: 'Moderate',
  4: 'Severe',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - CODE TABLE 4.230 - ATMOSPHERIC CHEMICAL OR PHYSICAL CONSTITUENT TYPE
 *
 * **Details**:
 * - **Revised**: 04/12/2022
 *
 * **Reserved Ranges**:
 * - `39-9999`: Reserved
 * - `10003`: Reserved
 * - `10024-10499`: Reserved
 * - `10501-20000`: Reserved
 * - `20022-29999`: Reserved
 * - `30001-50000`: Reserved
 * - `60017-61999`: Reserved
 * - `62035-65534`: Reserved
 *
 * **Special Value**:
 * - `65535`: Missing
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-230.shtml)
 * - [More data...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/WMO306_vI2_CommonTable_en_v23.0.0.pdf)
 */
export const grib2LookupTable4230: Record<number, string> = {
  0: 'Ozone - O3',
  1: 'Water Vapour - H2O',
  2: 'Methane - CH4',
  3: 'Carbon Dioxide - CO2',
  4: 'Carbon Monoxide - CO',
  5: 'Nitrogen Dioxide - NO2',
  6: 'Nitrous Oxide - N2O',
  7: 'Formaldehyde - HCHO',
  8: 'Sulphur Dioxide - SO2',
  9: 'Ammonia - NH3',
  10: 'Ammonium - NH4+',
  11: 'Nitrogen Monoxide - NO',
  12: 'Atomic Oxygen - O',
  13: 'Nitrate Radical - NO3',
  14: 'Hydroperoxyl Radical - HO2',
  15: 'Dinitrogen Pentoxide - H2O5',
  16: 'Nitrous Acid - HONO',
  17: 'Nitric Acid - HNO3',
  18: 'Peroxynitric Acid - HO2NO2',
  19: 'Hydrogen Peroxide - H2O2',
  20: 'Molecular Hydrogen - H',
  21: 'Atomic Nitrogen - N',
  22: 'Sulphate - SO42-',
  23: 'Radon - Rn',
  24: 'Elemental Mercury - Hg(O)',
  25: 'Divalent Mercury - Hg2+',
  26: 'Atomic Chlorine - Cl',
  27: 'Chlorine Monoxide - ClO',
  28: 'Dichlorine Peroxide - Cl2O2',
  29: 'Hypochlorous Acid - HClO',
  30: 'Chlorine Nitrate - ClONO2',
  31: 'Chlorine Dioxide - ClO2',
  32: 'Atomic Bromide - Br',
  33: 'Bromine Monoxide - BrO',
  34: 'Bromine Chloride - BrCl',
  35: 'Hydrogen Bromide - HBr',
  36: 'Hypobromous Acid - HBrO',
  37: 'Bromine Nitrate - BrONO2',
  38: 'Oxygen - O2',
  // 39-9999 Reserved
  10000: 'Hydroxyl Radical - OH',
  10001: 'Methyl Peroxy Radical - CH3O2',
  10002: 'Methyl Hydroperoxide - CH3O2H',
  // 10003 Reserved
  10004: 'Methanol - CH3OH',
  10005: 'Formic Acid - CH3OOH',
  10006: 'Hydrogen Cyanide - HCN',
  10007: 'Aceto Nitrile - CH3CN',
  10008: 'Ethane - C2H6',
  10009: 'Ethene (Ethylene) - C2H4',
  10010: 'Ethyne (Acetylene) - C2H2',
  10011: 'Ethanol - C2H5OH',
  10012: 'Acetic Acid - C2H5OOH',
  10013: 'Peroxyacetyl Nitrate - CH3C(O)OONO2',
  10014: 'Propane - C3H8',
  10015: 'Propene - C3H6',
  10016: 'Butanes - C4H10',
  10017: 'Isoprene - C5H10',
  10018: 'Alpha Pinene - C10H16',
  10019: 'Beta Pinene - C10H16',
  10020: 'Limonene - C10H16',
  10021: 'Benzene - C6H6',
  10022: 'Toluene - C7H8',
  10023: 'Xylene - C8H10',
  // 10024-10499 Reserved
  10500: 'Dimethyl Sulphide - CH3SCH3',
  // 10501-20000 Reserved
  20001: 'Hydrogen Chloride - HCL',
  20002: 'CFC-11',
  20003: 'CFC-12',
  20004: 'CFC-113',
  20005: 'CFC-113a',
  20006: 'CFC-114',
  20007: 'CFC-115',
  20008: 'HCFC-22',
  20009: 'HCFC-141b',
  20010: 'HCFC-142b',
  20011: 'Halon-1202',
  20012: 'Halon-1211',
  20013: 'Halon-1301',
  20014: 'Halon-2402',
  20015: 'Methyl Chloride (HCC-40)',
  20016: 'Carbon Tetrachloride (HCC-10)',
  20017: 'HCC-140a - CH3CCl3',
  20018: 'Methyl Bromide (HBC-40B1)',
  20019: 'Hexachlorocyclohexane (HCH)',
  20020: 'Alpha Hexachlorocyclohexane',
  20021: 'Hexachlorobiphenyl (PCB-153)',
  // 20022-29999 Reserved
  30000: 'Radioactive Pollutant (Tracer, defined by originating centre)',
  // 30001-50000 Reserved
  60000: 'HOx Radical (OH+HO2)',
  60001: 'Total Inorganic and Organic Peroxy Radicals (HO2+RO2) - RO2',
  60002: 'Passive Ozone',
  60003: 'NOx Expressed As Nitrogen - NOx',
  60004: 'All Nitrogen Oxides (NOy) Expressed As Nitrogen - NOy',
  60005: 'Total Inorganic Chlorine - Clx',
  60006: 'Total Inorganic Bromine - Brx',
  60007: 'Total Inorganic Chlorine Except HCl, ClONO2: ClOx',
  60008: 'Total Inorganic Bromine Except HBr, BrONO2: BrOx',
  60009: 'Lumped Alkanes',
  60010: 'Lumped Alkenes',
  60011: 'Lumped Aromatic Compounds',
  60012: 'Lumped Terpenes',
  60013: 'Non-Methane Volatile Organic Compounds Expressed as Carbon - NMVOC',
  60014: 'Anthropogenic Non-Methane Volatile Organic Compounds Expressed as Carbon - aNMVOC',
  60015: 'Biogenic Non-Methane Volatile Organic Compounds Expressed as Carbon - bNMVOC',
  60016: 'Lumped Oxygenated Hydrocarbons - OVOC',
  // 60017-61999 Reserved
  62000: 'Total Aerosol',
  62001: 'Dust Dry',
  62002: 'Water In Ambient',
  62003: 'Ammonium Dry',
  62004: 'Nitrate Dry',
  62005: 'Nitric Acid Trihydrate',
  62006: 'Sulphate Dry',
  62007: 'Mercury Dry',
  62008: 'Sea Salt Dry',
  62009: 'Black Carbon Dry',
  62010: 'Particulate Organic Matter Dry',
  62011: 'Primary Particulate Organic Matter Dry',
  62012: 'Secondary Particulate Organic Matter Dry',
  62034: 'Brown Carbon Dry',
  // 62035-65534 Reserved
  65535: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.233: AEROSOL TYPE
 *
 * **Created**: 05/16/2005
 * **Revised**: 07/18/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-233.shtml)
 * - [More data...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/WMO306_vI2_CommonTable_en_v23.0.0.pdf)
 *
 * ## Notes
 * Red text depicts changes made since 05/29/2019.
 */
export const grib2LookupTable4233: Record<number, string> = {
  0: 'Ozone - O3',
  1: 'Water Vapour - H2O',
  2: 'Methane - CH4',
  3: 'Carbon Dioxide - CO2',
  4: 'Carbon Monoxide - CO',
  5: 'Nitrogen Dioxide - NO2',
  6: 'Nitrous Oxide - N2O',
  7: 'Formaldehyde - HCHO',
  8: 'Sulphur Dioxide - SO2',
  9: 'Ammonia - NH3',
  10: 'Ammonium - NH4+',
  11: 'Nitrogen Monoxide - NO',
  12: 'Atomic Oxygen - O',
  13: 'Nitrate Radical - NO3',
  14: 'Hydroperoxyl Radical - HO2',
  15: 'Dinitrogen Pentoxide - H2O5',
  16: 'Nitrous Acid - HONO',
  17: 'Nitric Acid - HNO3',
  18: 'Peroxynitric Acid - HO2NO2',
  19: 'Hydrogen Peroxide - H2O2',
  20: 'Molecular Hydrogen - H',
  21: 'Atomic Nitrogen - N',
  22: 'Sulphate - SO42-',
  23: 'Radon - Rn',
  24: 'Elemental Mercury - Hg(O)',
  25: 'Divalent Mercury - Hg2+',
  26: 'Atomic Chlorine - Cl',
  27: 'Chlorine Monoxide - ClO',
  28: 'Dichlorine Peroxide - Cl2O2',
  29: 'Hypochlorous Acid - HClO',
  30: 'Chlorine Nitrate - ClONO2',
  31: 'Chlorine Dioxide - ClO2',
  32: 'Atomic Bromide - Br',
  33: 'Bromine Monoxide - BrO',
  34: 'Bromine Chloride - BrCl',
  35: 'Hydrogen Bromide - HBr',
  36: 'Hypobromous Acid - HBrO',
  37: 'Bromine Nitrate - BrONO2',
  38: 'Oxygen - O2',
  39: 'Reserved',
  10000: 'Hydroxyl Radical - OH',
  10001: 'Methyl Peroxy Radical - CH3O2',
  10002: 'Methyl Hydroperoxide - CH3O2H',
  10003: 'Reserved',
  10004: 'Methanol - CH3OH',
  10005: 'Formic Acid - CH3OOH',
  10006: 'Hydrogen Cyanide - HCN',
  10007: 'Aceto Nitrile - CH3CN',
  10008: 'Ethane - C2H6',
  10009: 'Ethene (Ethylene) - C2H4',
  10010: 'Ethyne (Acetylene) - C2H2',
  10011: 'Ethanol - C2H5OH',
  10012: 'Acetic Acid - C2H5OOH',
  10013: 'Peroxyacetyl Nitrate - CH3C(O)OONO2',
  10014: 'Propane - C3H8',
  10015: 'Propene - C3H6',
  10016: 'Butanes - C4H10',
  10017: 'Isoprene - C5H10',
  10018: 'Alpha Pinene - C10H16',
  10019: 'Beta Pinene - C10H16',
  10020: 'Limonene - C10H16',
  10021: 'Benzene - C6H6',
  10022: 'Toluene - C7H8',
  10023: 'Xylene - C8H10',
  10024: 'Reserved',
  10500: 'Dimethyl Sulphide - CH3SCH3',
  20001: 'Hydrogen Chloride - HCL',
  20002: 'CFC-11',
  20003: 'CFC-12',
  20004: 'CFC-113',
  20005: 'CFC-113a',
  20006: 'CFC-114',
  20007: 'CFC-115',
  20008: 'HCFC-22',
  20009: 'HCFC-141b',
  20010: 'HCFC-142b',
  20011: 'Halon-1202',
  20012: 'Halon-1211',
  20013: 'Halon-1301',
  20014: 'Halon-2402',
  20015: 'Methyl Chloride (HCC-40)',
  20016: 'Carbon Tetrachloride (HCC-10)',
  20017: 'HCC-140a - CH3CCl3',
  20018: 'Methyl Bromide (HBC-40B1)',
  20019: 'Hexachlorocyclohexane (HCH)',
  20020: 'Alpha Hexachlorocyclohexane',
  20021: 'Hexachlorobiphenyl (PCB-153)',
  30000: 'Radioactive Pollutant (Tracer, defined by originating centre)',
  60000: 'HOx Radical (OH+HO2)',
  60001: 'Total Inorganic and Organic Peroxy Radicals (HO2+RO2) - RO2',
  60002: 'Passive Ozone',
  60003: 'NOx Expressed As Nitrogen - NOx',
  60004: 'All Nitrogen Oxides (NOy) Expressed As Nitrogen - NOy',
  60005: 'Total Inorganic Chlorine Except HCl, ClONO2: ClOx',
  60006: 'Total Inorganic Bromine Except HBr, BrONO2: BrOx',
  60007: 'Lumped Alkanes',
  60008: 'Lumped Alkenes',
  60009: 'Lumped Aromatic Compounds',
  60010: 'Lumped Terpenes',
  60011: 'Non-Methane Volatile Organic Compounds Expressed as Carbon - NMVOC',
  60012: 'Anthropogenic NMVOC Expressed as Carbon - aNMVOC',
  60013: 'Biogenic NMVOC Expressed as Carbon - bNMVOC',
  60014: 'Lumped Oxygenated Hydrocarbons - OVOC',
  60015: 'Reserved',
  62000: 'Total Aerosol',
  62001: 'Dust Dry',
  62002: 'Water In Ambient',
  62003: 'Ammonium Dry',
  62004: 'Nitrate Dry',
  62005: 'Nitric Acid Trihydrate',
  62006: 'Sulphate Dry',
  62007: 'Mercury Dry',
  62008: 'Sea Salt Dry',
  62009: 'Black Carbon Dry',
  62010: 'Particulate Organic Matter Dry',
  62011: 'Primary Particulate Organic Matter Dry',
  62012: 'Secondary Particulate Organic Matter Dry',
  62013: 'Black Carbon Hydrophilic Dry',
  62014: 'Black Carbon Hydrophobic Dry',
  62015: 'Particulate Organic Matter Hydrophilic Dry',
  62016: 'Particulate Organic Matter Hydrophobic Dry',
  62017: 'Nitrate Hydrophilic Dry',
  62018: 'Nitrate Hydrophobic Dry',
  62019: 'Reserved',
  62020: 'Smoke - High Absorption',
  62021: 'Smoke - Low Absorption',
  62022: 'Aerosol - High Absorption',
  62023: 'Aerosol - Low Absorption',
  62024: 'Reserved',
  62025: 'Volcanic Ash',
  62036: 'Brown Carbon Dry',
  // 62037-65534: Reserved
  65535: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.234: CANOPY COVER FRACTION
 *
 * **Created**: 07/12/2013
 *
 * **Description**:
 * To be used as partitioned parameter in Product Definition Templates (PDT) 4.53 or 4.54.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-234.shtml)
 */
export const grib2LookupTable4234: Record<number, string> = {
  1: 'Crops, mixed farming',
  2: 'Short grass',
  3: 'Evergreen needleleaf trees',
  4: 'Deciduous needleleaf trees',
  5: 'Deciduous broadleaf trees',
  6: 'Evergreen broadleaf trees',
  7: 'Tall grass',
  8: 'Desert',
  9: 'Tundra',
  10: 'Irrigated crops',
  11: 'Semidesert',
  12: 'Ice caps and glaciers',
  13: 'Bogs and marshes',
  14: 'Inland water',
  15: 'Ocean',
  16: 'Evergreen shrubs',
  17: 'Deciduous shrubs',
  18: 'Mixed forest',
  19: 'Interrupted forest',
  20: 'Water and land mixtures',
  // 21-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.235: Wind-Generated Wave Spectral Description
 *
 * **Created**: 02/15/2012
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-235.shtml)
 */
export const grib2LookupTable4235: Record<number, string> = {
  0: 'Total Wave Spectrum (combined wind waves and swell)',
  1: 'Generalized Partition',
  // 2-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.236: Soil Texture Fraction
 * (to be used as partitioned parameter in PDT 4.53 or 4.54)
 *
 * **Created**: 07/12/2013
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-236.shtml)
 */
export const grib2LookupTable4236: Record<number, string> = {
  1: 'Coarse',
  2: 'Medium',
  3: 'Medium-fine',
  4: 'Fine',
  5: 'Very-fine',
  6: 'Organic',
  7: 'Tropical-organic',
  // 8-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.238: Source or Sink
 *
 * **Revised**: 07/15/2024
 * Red text depicts changes made since 07/15/2024
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-238.shtml)
 */
export const grib2LookupTable4238: Record<number, string> = {
  0: 'Reserved',
  1: 'Aviation',
  2: 'Lightning',
  3: 'Biogenic Sources',
  4: 'Anthropogenic sources',
  5: 'Wild fires',
  6: 'Natural sources',
  7: 'Bio-fuel',
  8: 'Volcanoes',
  9: 'Fossil-fuel',
  10: 'Wetlands',
  11: 'Oceans',
  12: 'Elevated anthropogenic sources',
  13: 'Surface anthropogenic sources',
  14: 'Agriculture livestock',
  15: 'Agriculture soils',
  16: 'Agriculture waste burning',
  17: 'Agriculture (all)',
  18: 'Residential, commercial and other combustion',
  19: 'Power generation',
  20: 'Super power stations',
  21: 'Fugitives',
  22: 'Industrial process',
  23: 'Solvents',
  24: 'Ships',
  25: 'Wastes',
  26: 'Road transportation',
  27: 'Off-road transportation',
  28: 'Nuclear power plant',
  29: 'Nuclear weapon',
  // 30-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.239: Wetland Type
 *
 * **Created**: 10/24/2023
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-239.shtml)
 */
export const grib2LookupTable4239: Record<number, string> = {
  0: 'Reserved',
  1: 'Bog',
  2: 'Drained',
  3: 'Fen',
  4: 'Floodplain',
  5: 'Mangrove',
  6: 'Marsh',
  7: 'Rice',
  8: 'Riverine',
  9: 'Salt Marsh',
  10: 'Swamp',
  11: 'Upland',
  12: 'Wet tundra',
  // 13-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.240: Type of Distribution Function
 *
 * **Revised**: 07/07/2017
 *
 * ## Notes
 * 1. Bin-Model or delta function with N concentration cl(r) in class (or mode) l.
 *    Concentration-density function:
 *    ƒ(r;d) = ∑l=1N cl(r) δ(d-Dl)
 *    - N: Number of modes in the distribution
 *    - δ: Delta-Function
 *    - d: Diameter
 *    - Dl: Diameter of mode l(p1)
 *
 * 2. Bin-Model or delta function with N concentration cl(r) in class (or mode) l.
 *    Concentration-density function:
 *    ƒ(r;m) = ∑l=1N cl(r) δ(m-Ml)
 *    - N: Number of modes in the distribution
 *    - δ: Delta-Function
 *    - m: Mass
 *    - Ml: Mass of mode (p1)
 *
 * 3. N-Modal concentration-density function consisting of Gaussian-functions:
 *    ƒ(r;d) = ∑l=1N cl(r) (1 / √(2πδl)) * e-((d-Dl)/δl)^2
 *    - N: Number of modes in the distribution
 *    - d: Diameter
 *    - Dl: Mean diameter of mode l(p1)
 *    - δl: Variance of Mode l (p2)
 *    - cl(r): Concentration
 *
 * 4. N-Modal concentration-density function consisting of Gaussian-functions:
 *    ƒ(r;d) = ∑l=1N cl(r) (1 / √(2πδl(r))) * e-((d-Dl(r))/δl(r))^2
 *    - N: Fields of concentration cl(r)
 *    - δl(r): Variance
 *    - Dl(r): Mean diameter
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-240.shtml)
 */
export const grib2LookupTable4240: Record<number, string> = {
  0: 'No specific distribution function given',
  1: 'Delta functions with spatially variable concentration and fixed diameters Dl(p1) in meter',
  2: 'Delta functions with spatially variable concentration and fixed masses Ml(p1) in kg',
  3: 'Gaussian (Normal) distribution with spatially variable concentration and fixed mean diameter Dl(p1) and variance δ(p2)',
  4: 'Gaussian (Normal) distribution with spatially variable concentration, mean diameter and variance',
  5: 'Log-normal distribution with spatially variable number density, mean diameter and variance',
  6: 'Log-normal distribution with spatially variable number density, mean diameter and fixed variance δ(p1)',
  7: 'Log-normal distribution with spatially variable number density and mass density and fixed variance δ and fixed particle density ρ(p2)',
  8: 'No distribution function. The encoded variable is derived from variables characterized by type of distribution function of type No. 7 with fixed variance σ(p1) and fixed particle density ρ(p2)',
  // 9-49151 Reserved
  // 49152-65534 Reserved for Local Use
  65535: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.241: Coverage Attributes
 *
 * **Updated**: 12/07/2023
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-241.shtml)
 */
export const grib2LookupTable4241: Record<number, string> = {
  0: 'Undefined',
  1: 'Unmodified',
  2: 'Snow-covered',
  3: 'Flooded',
  4: 'Ice Covered',
  5: 'With intercepted water',
  6: 'With intercepted snow',
  7: 'Aggregated',
  // 8-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.242: Tile Classification
 *
 * **Updated**: 12/07/2023
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-242.shtml)
 */
export const grib2LookupTable4242: Record<number, string> = {
  0: 'Reserved',
  1: 'Land use classes according to ESA-GLOBCOVER GCV2009',
  2: 'Land use classes according to European Commission-Global Land Cover Project GLC2000',
  3: 'Land use classes according to ECOCLIMAP',
  4: 'Land use classes according to ECOCLIMAP-SG',
  5: 'Land use classes according to USGS EROS Global Land Cover Characterization (GLCC) v2.0 BATS Classification',
  // 6-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.243: Tile Class
 *
 * **Created**: 04/09/2015
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-243.shtml)
 */
export const grib2LookupTable4243: Record<number, string> = {
  0: 'Reserved',
  1: 'Evergreen broadleaved forest',
  2: 'Deciduous broadleaved closed forest',
  3: 'Deciduous broadleaved open forest',
  4: 'Evergreen needle-leaf forest',
  5: 'Deciduous needle-leaf forest',
  6: 'Mixed leaf trees',
  7: 'Fresh water flooded trees',
  8: 'Saline water flooded trees',
  9: 'Mosaic tree/natural vegetation',
  10: 'Burnt tree cover',
  11: 'Evergreen shurbs closed-open',
  12: 'Deciduous shurbs closed-open',
  13: 'Herbaceous vegetation closed-open',
  14: 'Sparse herbaceous or grass',
  15: 'Flooded shurbs or herbaceous',
  16: 'Cultivated and managed areas',
  17: 'Mosaic crop/tree/natural vegetation',
  18: 'Mosaic crop/shrub/grass',
  19: 'Bare areas',
  20: 'Water',
  21: 'Snow and ice',
  22: 'Artificial surface',
  23: 'Ocean',
  24: 'Irrigated croplands',
  25: 'Rain fed croplands',
  26: 'Mosaic cropland (50-70%)-vegetation (20-50%)',
  27: 'Mosaic vegetation (50-70%)-cropland (20-50%)',
  28: 'Closed broadleaved evergreen forest',
  29: 'Closed needle-leaved evergreen forest',
  30: 'Open needle-leaved deciduous forest',
  31: 'Mixed broadleaved and needle-leave forest',
  32: 'Mosaic shrubland (50-70%)-grassland (20-50%)',
  33: 'Mosaic grassland (50-70%)-shrubland (20-50%)',
  34: 'Closed to open shrubland',
  35: 'Sparse vegetation',
  36: 'Closed to open forest regularly flooded',
  37: 'Closed forest or shrubland permanently flooded',
  38: 'Closed to open grassland regularly flooded',
  39: 'Undefined',
  // 40-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.244: QUALITY INDICATOR
 *
 * **Created**: 07/09/2018
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-244.shtml)
 */
export const grib2LookupTable4244: Record<number, string> = {
  0: 'No Quality Information Available',
  1: 'Failed',
  2: 'Passed',
  // 3-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.246: THUNDERSTORM INTENSITY INDEX
 *
 * **Created**: 06/23/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-246.shtml)
 */
export const grib2LookupTable4246: Record<number, string> = {
  0: 'No thunderstorm occurrence',
  1: 'Weak thunderstorm',
  2: 'Moderate thunderstorm',
  3: 'Severe thunderstorm',
  // 4-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.247: PRECIPITATION INTENSITY
 *
 * **Created**: 06/23/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-247.shtml)
 */
export const grib2LookupTable4247: Record<number, string> = {
  0: 'No precipitation occurrence',
  1: 'Light precipitation',
  2: 'Moderate precipitation',
  3: 'Heavy precipitation',
  // 4-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.248: METHOD USED TO DERIVE DATA VALUE FOR A GIVEN LOCAL TIME
 *
 * **Created**: 06/23/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-248.shtml)
 */
export const grib2LookupTable4248: Record<number, string> = {
  0: 'Nearest forecast or analysis time to specified local time',
  1: 'Interpolated to be valid at the specified local time',
  // 2-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.249: CHARACTER OF PRECIPITATION
 *
 * **Created**: 06/23/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-249.shtml)
 */
export const grib2LookupTable4249: Record<number, string> = {
  0: 'None',
  1: 'Showers',
  2: 'Intermittent',
  3: 'Continuous',
  // 4-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.250: DRAINAGE DIRECTION
 *
 * **Created**: 06/23/2022
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-250.shtml)
 */
export const grib2LookupTable4250: Record<number, string> = {
  0: 'Reserved',
  1: 'South-West',
  2: 'South',
  3: 'South-East',
  4: 'West',
  5: 'No direction',
  6: 'East',
  7: 'North-West',
  8: 'North',
  9: 'North-East',
  // 10-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.251: WAVE DIRECTION AND FREQUENCY FORMULAE
 *
 * **Created**: 10/24/2023
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-251.shtml)
 *
 * ## Notes
 * (1).  Geometric sequence: x_n = x_0 * r^(n-1) with 'x_0' first parameter and 'r' second parameter.
 * (2).  Arithmetic sequence: a_n = a_1 + (n-1) d with 'a_1' first parameter and 'd' second parameter.
 */
export const grib2LookupTable4251: Record<number, string> = {
  0: 'Undefined Sequence',
  1: 'Geometric sequence (see Note 1)',
  2: 'Arithmetic sequence (see Note 2)',
  // 3-191 Reserved
  // 192-254 Reserved for Local Use
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.333: Transport Dispersion Model
 *
 * **Created**: 07/15/2024
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-333.shtml)
 *
 * ## Notes
 * (No additional notes provided for this table)
 */
export const grib2LookupTable4333: Record<number, string> = {
  0: 'Reserved',
  1: 'DERMA (Danish Emergency Response Model of the Atmosphere)',
  2: 'E-EMEP (Emergency EMEP model)',
  3: 'FLEXPART (Particle dispersion model)',
  4: 'MLDP (Modèle lagrangien de dispersion de particules)',
  5: 'MATCH (Multi-scale Atmospheric Transport Model)',
  6: 'SILAM (System for Integrated modeLling of Atmospheric composition)',
  7: 'SNAP (Severe Nuclear Accident Program)',
  8: 'WRF-Chem (Weather Research and Forecasting Chemical model)',
  9: 'Trajectoire (Trajectory model)',
  // 10-254 Reserved
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.335: Emission Scenario Origin
 *
 * **Created**: 07/15/2024
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-335.shtml)
 *
 * ## Notes
 * (No additional notes provided for this table)
 */
export const grib2LookupTable4335: Record<number, string> = {
  0: 'Reserved',
  1: 'ARGOS (Accident Reporting and Guiding Operational System)',
  2: 'JRODOS (Java version of Real time Online Decision SuppOrt System)',
  3: 'Assimilated (Scenario retrieved from measurements)',
  4: 'Center (scenario by originating center)',
  // 5-254 Reserved
  255: 'Missing',
};

/**
 * GRIB2 - CODE TABLE 4.336: NWP Model
 *
 * **Created**: 07/15/2024
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-336.shtml)
 *
 * ## Notes
 * (No additional notes provided for this table)
 */
export const grib2LookupTable4336: Record<number, string> = {
  0: 'Reserved',
  1: 'AROME (Meso scale NWP, Meteo-France)',
  2: 'ARPEGE (Global scale NWP, Meteo-France)',
  3: 'GFS (Global forecast system, NCEP)',
  4: 'HARMONIE (HIRLAM-ALADIN Research on Mesoscale Operational NWP)',
  5: 'HIRLAM (HIgh resolution Limited Area Model)',
  6: 'IFS (Integrated Forecast System)',
  7: 'GEM GDPS (Canadian Global Deterministic Prediction System)',
  8: 'GEM RDPS (Canadian Regional Deterministic Prediction System)',
  9: 'GEM HRDPS (Canadian High Resolution Deterministic Prediction System)',
  10: 'WRF (Weather Research and Forecasting)',
  // 11-254 Reserved
  255: 'Missing',
};
