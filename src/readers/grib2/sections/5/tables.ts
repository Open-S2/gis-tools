/**
 * # GRIB2 - TABLE 5.0 - DATA REPRESENTATION TEMPLATE NUMBER
 *
 * **Details**:
 * - **Section**: 5
 * - **Octets**: 10-11
 * - **Revised**: 07/01/2022
 *
 * **Reserved Ranges**:
 * - `5-39`: Reserved
 * - `43-49`: Reserved
 * - `52`: Reserved
 * - `54-60`: Reserved
 * - `62-199`: Reserved
 * - `201-49151`: Reserved
 * - `49152-65534`: Reserved for Local Use
 *
 * **Special Value**:
 * - `65535`: Missing
 *
 * ## Notes
 */
export const lookupTable50: Record<number, string> = {
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-0.shtml) */
  0: 'Grid Point Data - Simple Packing (see Template 5.0)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-1.shtml) */
  1: 'Matrix Value at Grid Point - Simple Packing (see Template 5.1)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-2.shtml) */
  2: 'Grid Point Data - Complex Packing (see Template 5.2)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml) */
  3: 'Grid Point Data - Complex Packing and Spatial Differencing (see Template 5.3)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-4.shtml) */
  4: 'Grid Point Data - IEEE Floating Point Data (see Template 5.4)',
  // 5-39: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-40.shtml) */
  40: 'Grid point data - JPEG 2000 code stream format (see Template 5.40)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-41.shtml) */
  41: 'Grid point data - Portable Network Graphics (PNG) (see Template 5.41)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-42.shtml) */
  42: 'Grid point data - CCSDS recommended lossless compression (see Template 5.42)',
  // 43-49: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-50.shtml) */
  50: 'Spectral Data - Simple Packing (see Template 5.50)',
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-51.shtml) */
  51: 'Spectral Data - Complex Packing (see Template 5.51)',
  // 52: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-53.shtml) */
  53: 'Spectral data for limited area models - complex packing (see Template 5.53)',
  // 54-60: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-61.shtml) */
  61: 'Grid Point Data - Simple Packing With Logarithm Pre-processing (see Template 5.61)',
  // 62-199: Reserved
  /** [See more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-200.shtml) */
  200: 'Run Length Packing With Level Values (see Template 5.200)',
  // 201-49151: Reserved
  // 49152-65534: Reserved for Local Use
  65535: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.1 - TYPE OF ORIGINAL FIELD VALUES
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
 */
export const lookupTable51: Record<number, string> = {
  0: 'Floating Point',
  1: 'Integer',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.2 - MATRIX COORDINATE VALUE FUNCTION DEFINITION
 *
 * **Details**:
 * - **Revised**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `2-10`: Reserved
 * - `12-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const lookupTable52: Record<number, string> = {
  0: 'Explicit Coordinate Values Set',
  1: 'Linear Coordinates: f(1) = C1, f(n) = f(n-1) + C2',
  // 2-10: Reserved
  11: 'Geometric Coordinates: f(1) = C1, f(n) = C2 x f(n-1)',
  // 12-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.3 - MATRIX COORDINATE PARAMETER
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
 */
export const lookupTable53: Record<number, string> = {
  // 0: Reserved
  1: 'Direction Degrees True',
  2: 'Frequency (s-1)',
  3: 'Radial Number (2pi/lambda) (m-1)',
  // 4-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.4 - GROUP SPLITTING METHOD
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
 */
export const lookupTable54: Record<number, string> = {
  0: 'Row by Row Splitting',
  1: 'General Group Splitting',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.5 - MISSING VALUE MANAGEMENT FOR COMPLEX PACKING
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
 */
export const lookupTable55: Record<number, string> = {
  0: 'No explicit missing values included within the data values',
  1: 'Primary missing values included within the data values',
  2: 'Primary and secondary missing values included within the data values',
  // 3-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.6 - ORDER OF SPATIAL DIFFERENCING
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
 */
export const lookupTable56: Record<number, string> = {
  // 0: Reserved
  1: 'First-Order Spatial Differencing',
  2: 'Second-Order Spatial Differencing',
  // 3-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.7 - PRECISION OF FLOATING POINT NUMBERS
 *
 * **Details**:
 * - **Created**: 05/16/2005
 *
 * **Reserved Ranges**:
 * - `4-254`: Reserved
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const lookupTable57: Record<number, string> = {
  // 0: Reserved
  1: 'IEEE 32-bit (I=4 in Section 7)',
  2: 'IEEE 64-bit (I=8 in Section 7)',
  3: 'IEEE 128-bit (I=16 in Section 7)',
  // 4-254: Reserved
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.25 - TYPE OF BI-FOURIER SUBTRUNCATION
 *
 * **Details**:
 * - **Created**: 05/29/2019
 *
 * **Reserved Ranges**:
 * - `0-76`: Reserved
 * - `78-87`: Reserved
 * - `89-98`: Reserved
 * - `100-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const lookupTable525: Record<number, string> = {
  // 0-76: Reserved
  77: 'Rectangular',
  // 78-87: Reserved
  88: 'Elliptic',
  // 89-98: Reserved
  99: 'Diamond',
  // 100-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.26 - PACKING MODE FOR AXES
 *
 * **Details**:
 * - **Created**: 05/29/2019
 *
 * **Reserved Ranges**:
 * - `2-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const lookupTable526: Record<number, string> = {
  0: 'Spectral coefficients for axes are packed',
  1: 'Spectral coefficients for axes included in the unpacked subset',
  // 2-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # GRIB2 - TABLE 5.40 - TYPE OF COMPRESSION
 *
 * **Details**:
 * - **Created**: 02/14/2006
 *
 * **Reserved Ranges**:
 * - `2-254`: Reserved
 *
 * **Special Value**:
 * - `255`: Missing
 */
export const lookupTable540: Record<number, string> = {
  0: 'Lossless',
  1: 'Lossy',
  // 2-254: Reserved
  255: 'Missing',
};
