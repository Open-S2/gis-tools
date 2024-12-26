/**
 * # Table 3.0 - Source of Grid Definition
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 6
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: A grid definition does not apply to this product.
 *
 * ## Description
 * This table specifies the source of grid definitions used in GRIB2 files,
 * providing context for how the grid is defined, whether through predefined templates or originating centers.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-0.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 */
export const grib2LookupTable30: Record<number, string> = {
  0: 'Specified in Code Table 3.1',
  1: 'Predetermined Grid Definition - Defined by Originating Center',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'A grid definition does not apply to this product.',
};

/**
 * # Table 3.1 - Grid Definition Template Number
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 13-14
 *
 * **Reserved Ranges**:
 * - `3-32767`: Reserved
 * - `32768-65534`: Reserved for Local Use
 *
 * **Special Value**:
 * - `65535`: Missing
 *
 * ## Description
 * This table enumerates the grid definition templates used in GRIB2 files,
 * providing detailed classifications for various grid types, projections, and modeling subdomains.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
 *
 * ## Notes
 * - Revised 12/07/2023
 * - (1). WGS84 is a geodetic system that uses IAG-GRS80 as a basis.
 * - (2). With respect to code figures 0, 1, 3, 6, and 7, coordinates can only be unambiguously interpreted if the coordinate reference system in which they are embedded is known. Therefore, defining the shape of the Earth alone without coordinate system axis origins is ambiguous. Generally, the prime meridian defined in the geodetic system WGS-84 can be safely assumed to be the longitudinal origin. However, because these code figures do not specify the longitudinal origin explicitly, it is suggested to contact the originating center if high precision coordinates are needed to obtain the precise details of the coordinate system used (effective as from 16 November 2016).
 */
export const grib2LookupTable31: Record<number, string> = {
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-0.shtml) */
  0: 'Latitude/Longitude (See Template 3.0) Also called Equidistant Cylindrical or Plate Caree',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1.shtml) */
  1: 'Rotated Latitude/Longitude (See Template 3.1)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-2.shtml) */
  2: 'Stretched Latitude/Longitude (See Template 3.2)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-3.shtml) */
  3: 'Rotated and Stretched Latitude/Longitude (See Template 3.3)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-4.shtml) */
  4: 'Variable Resolution Latitude/longitude (See Template 3.4)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-5.shtml) */
  5: 'Variable Resolution Rotated Latitude/longitude (See Template 3.5)',
  // 6-9: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-10.shtml) */
  10: 'Mercator (See Template 3.10)',
  11: 'Reserved',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-12.shtml) */
  12: 'Transverse Mercator (See Template 3.12)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-13.shtml) */
  13: 'Mercator with modelling subdomains definition (See Template 3.13)',
  // 14-19: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-20.shtml) */
  20: 'Polar Stereographic Projection (Can be North or South) (See Template 3.20)',
  // 21-22: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-23.shtml) */
  23: 'Polar Stereographic with modelling subdomains definition (See Template 3.23)',
  // 24-29: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml) */
  30: 'Lambert Conformal (Can be Secant, Tangent, Conical, or Bipolar) (See Template 3.30)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-31.shtml) */
  31: 'Albers Equal Area (See Template 3.31)',
  32: 'Reserved',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-33.shtml) */
  33: 'Lambert conformal with modelling subdomains definition (See Template 3.33)',
  // 34-39: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml) */
  40: 'Gaussian Latitude/Longitude (See Template 3.40)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-41.shtml) */
  41: 'Rotated Gaussian Latitude/Longitude (See Template 3.41)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-42.shtml) */
  42: 'Stretched Gaussian Latitude/Longitude (See Template 3.42)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-43.shtml) */
  43: 'Rotated and Stretched Gaussian Latitude/Longitude (See Template 3.43)',
  // 44-49: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-50.shtml) */
  50: 'Spherical Harmonic Coefficients (See Template 3.50)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-51.shtml) */
  51: 'Rotated Spherical Harmonic Coefficients (See Template 3.51)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-52.shtml) */
  52: 'Stretched Spherical Harmonic Coefficients (See Template 3.52)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-53.shtml) */
  53: 'Rotated and Stretched Spherical Harmonic Coefficients (See Template 3.53)',
  // 54-59: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-60.shtml) */
  60: 'Cubed-Sphere Gnomonic (See Template 3.60) Validation',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-61.shtml) */
  61: 'Spectral Mercator with modelling subdomains definition (See Template 3.61)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-62.shtml) */
  62: 'Spectral Polar Stereographic with modelling subdomains definition (See Template 3.62)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-63.shtml) */
  63: 'Spectral Lambert conformal with modelling subdomains definition (See Template 3.63)',
  // 64-89: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-90.shtml) */
  90: 'Space View Perspective or Orthographic (See Template 3.90)',
  // 91-99: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-100.shtml) */
  100: 'Triangular Grid Based on an Icosahedron (See Template 3.100)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-101.shtml) */
  101: 'General Unstructured Grid (see Template 3.101)',
  // 102-109: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-110.shtml) */
  110: 'Equatorial Azimuthal Equidistant Projection (See Template 3.110)',
  // 111-119: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-120.shtml) */
  120: 'Azimuth-Range Projection (See Template 3.120)',
  // 121-139: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-140.shtml) */
  140: 'Lambert Azimuthal Equal Area Projection (See Template 3.140)',
  // 141-149: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-150.shtml) */
  150: 'Hierarchical Equal Area isoLatitude Pixelization grid (HEALPix) (See Template 3.150)',
  // 151-203: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-204.shtml) */
  204: 'Curvilinear Orthogonal Grids (See Template 3.204)',
  // 205-999: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1000.shtml) */
  1000: 'Cross Section Grid with Points Equally Spaced on the Horizontal (See Template 3.1000)',
  // 1001-1099: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1100.shtml) */
  1100: 'Hovmoller Diagram with Points Equally Spaced on the Horizontal (See Template 3.1100)',
  // 1101-1199: Reserved
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1200.shtml) */
  1200: 'Time Section Grid (See Template 3.1200)',
  // 1201-32767: Reserved
  // 32768-65534: Reserved for Local Use
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-32768.shtml) */
  32768: 'Rotated Latitude/Longitude (Arakawa Staggered E-Grid) (See Template 3.32768)',
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-32769.shtml) */
  32769: 'Rotated Latitude/Longitude (Arakawa Non-E Staggered Grid) (See Template 3.32769)',
  65535: 'Missing',
};

/**
 * # Table 3.2 - Shape of the Reference System
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 15
 *
 * **Reserved Ranges**:
 * - `12-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the shape of the reference system used in GRIB2 files,
 * providing context for interpreting the Earth's shape and the coordinate reference system.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
 *
 * ## Notes
 * - (1) WGS84 is a geodetic system that uses IAG-GRS80 as a basis.
 * - (2) With respect to code figures 0, 1, 3, 6, and 7, coordinates can only be unambiguously interpreted if the coordinate reference system in which they are embedded is known. Therefore, defining the shape of the Earth alone without coordinate system axis origins is ambiguous. Generally, the prime meridian defined in the geodetic system WGS-84 can be safely assumed to be the longitudinal origin. However, because these code figures do not specify the longitudinal origin explicitly, it is suggested to contact the originating center if high precision coordinates are needed to obtain the precise details of the coordinate system used (effective as from 16 November 2016).
 */
export const grib2LookupTable32: Record<number, string> = {
  0: 'Earth assumed spherical with radius = 6,367,470.0 m',
  1: 'Earth assumed spherical with radius specified (in m) by data producer',
  2: 'Earth assumed oblate spheroid with size as determined by IAU in 1965 (major axis = 6,378,160.0 m, minor axis = 6,356,775.0 m, f = 1/297.0)',
  3: 'Earth assumed oblate spheroid with major and minor axes specified (in km) by data producer',
  4: 'Earth assumed oblate spheroid as defined in IAG-GRS80 model (major axis = 6,378,137.0 m, minor axis = 6,356,752.314 m, f = 1/298.257222101)',
  5: 'Earth assumed represented by WGS84 (as used by ICAO since 1998) (Uses IAG-GRS80 as a basis)',
  6: 'Earth assumed spherical with radius = 6,371,229.0 m',
  7: 'Earth assumed oblate spheroid with major and minor axes specified (in m) by data producer',
  8: 'Earth model assumed spherical with radius 6,371,200 m, but the horizontal datum of the resulting Latitude/Longitude field is the WGS84 reference frame',
  9: 'Earth represented by the OSGB 1936 Datum, using the Airy_1830 Spheroid, the Greenwich meridian as 0 Longitude, the Newlyn datum as mean sea level, 0 height.',
  10: 'Earth model assumed WGS84 with corrected geomagnetic coordinates (latitude and longitude) defined by Gustafsson et al., 1992". (see Note 1)',
  11: 'Sun assumed spherical with radius = 695 990 000 m (Allen, C.W., Astrophysical Quantities, 3rd ed.; Athlone: London, 1976) and Stonyhurst latitude and longitude system with origin at the intersection of the solar central meridian (as seen from Earth) and the solar equator (Thompson, W., Coordinate systems for solar image data, Astron. Astrophys. 2006, 449, 791-803)',
  // 12-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 3.3 - RESOLUTION AND COMPONENT FLAGS
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 55
 * - **Applicable Grid Templates**: 0-3, 40-43
 *
 * **Reserved Bits**:
 * - `1-2`: Reserved
 * - `6-8`: Reserved - set to zero
 *
 * **Special Values**:
 * - None
 *
 * ## Description
 * This table defines the resolution and component flags used in GRIB2 files,
 * specifying various increments and component resolutions for vector quantities.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 */
export const grib2LookupTable33: Record<number, Record<number, string>> = {
  /** Bit 3 - i Direction Increments */
  3: {
    0: 'i direction increments not given',
    1: 'i direction increments given',
  },
  /** Bit 4 - j Direction Increments */
  4: {
    0: 'j direction increments not given',
    1: 'j direction increments given',
  },
  /** Bit 5 - Resolved Components of Vector Quantities */
  5: {
    0: 'Resolved u and v components of vector quantities relative to easterly and northerly directions',
    1: 'Resolved u and v components of vector quantities relative to the defined grid in the direction of increasing x and y (or i and j) coordinates, respectively.',
  },
};

/**
 * # Table 3.4 - SCANNING MODE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 72
 * - **Applicable Grid Templates**: 0-3, 40-43, 204
 *
 * **Reserved Bits**:
 * - None
 *
 * **Special Values**:
 * - None
 *
 * ## Description
 * This table defines the scanning mode flags used in GRIB2 files,
 * specifying the scanning direction and row/column offsets.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
 *
 * ## Notes
 * - (1).  i direction - West to east along a parallel or left to right along an x-axis.
 * - (2).  j direction - South to north along a meridian, or bottom to top along a y-axis.
 * - (3).  If bit number 4 is set, the first row scan is defined by previous flags.
 * - (4).  La1 and Lo1 define the first row, which is an odd row.
 * - (5).  Di and Dj are assumed to be positive, with the direction of i and j being given by bits 1 and 2.
 * - (6).  Bits 5 through 8 may be used to generate staggered grids, such as Arakawa grids
 *        (see Attachment, Volume 1.2, Part A, Att. GRIB).
 * - (7).  If any of bits 5, 6, 7 or 8 are set, Di and Dj are not optional.
 */
export const grib2LookupTable34: Record<number, Record<number, string>> = {
  1: {
    0: 'Points in the first row or column scan in the +i (+x) direction',
    1: 'Points in the first row or column scan in the -i (-x) direction',
  },
  2: {
    0: 'Points in the first row or column scan in the -j (-y) direction',
    1: 'Points in the first row or column scan in the +j (+y) direction',
  },
  3: {
    0: 'Adjacent points in the i (x) direction are consecutive',
    1: 'Adjacent points in the j (y) direction are consecutive',
  },
  4: {
    0: 'All rows scan in the same direction',
    1: 'Adjacent rows scan in the opposite direction',
  },
  5: {
    0: 'Points within odd rows are not offset in i(x) direction',
    1: 'Points within odd rows are offset by Di/2 in i(x) direction',
  },
  6: {
    0: 'Points within even rows are not offset in i(x) direction',
    1: 'Points within even rows are offset by Di/2 in i(x) direction',
  },
  7: {
    0: 'Points are not offset in j(y) direction',
    1: 'Points are offset by Dj/2 in j(y) direction',
  },
  8: {
    0: 'Rows have Ni grid points and columns have Nj grid points',
    1: 'Rows have Ni grid points if points are not offset in i direction; Rows have Ni-1 grid points if points are offset by Di/2 in i direction. Columns have Nj grid points if points are not offset in j direction; Columns have Nj-1 grid points if points are offset by Dj/2 in j(y) direction.',
  },
};

/**
 * # Table 3.5 - PROJECTION CENTER
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 55
 * - **Applicable Grid Templates**: 20, 30, 31
 *
 * **Reserved Bits**:
 * - `3-8`: Reserved
 *
 * **Special Values**:
 * - None
 *
 * ## Description
 * This table defines the projection center flags used in GRIB2 files,
 * specifying the pole location and projection type.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 */
export const grib2LookupTable35: Record<number, Record<number, string>> = {
  1: {
    0: 'North Pole is on the projection plane',
    1: 'South Pole is on the projection plane',
  },
  2: {
    0: 'Only one projection center is used',
    1: 'Projection is bi-polar and symmetric',
  },
};

/**
 * # Table 3.6 - SPECTRAL DATA REPRESENTATION TYPE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: [Not Specified]
 *
 * **Reserved Ranges**:
 * - `3-254`: Reserved
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the spectral data representation types used in GRIB2 files,
 * specifying the mathematical representations employed for spectral data.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-6.shtml)
 *
 * ## Notes
 * - Revised 08/23/2023
 */
export const grib2LookupTable36: Record<number, string> = {
  1: 'The Associated Legendre Functions of the first kind are defined by:',
  2: 'Bi-Fourier representation',
  // 3-254: Reserved
  255: 'Missing',
};

/**
 * # Table 3.7 - SPECTRAL DATA REPRESENTATION MODE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 55
 *
 * **Reserved Ranges**:
 * - `2-254`: Reserved
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the spectral data representation modes used in GRIB2 files,
 * specifying how spectral data is represented, including the mathematical representations employed.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-6.shtml)
 *
 * ## Notes
 * - (1) Values of N(m) for common truncation cases are as follows:
 *   - Triangular:     M = J = K,        N(m) = J
 *   - Rhomboidal:     K = J + M,        N(m) = J + m
 *   - Trapezoidal:    K = J, K > M,     N(m) = J
 */
export const grib2LookupTable37: Record<number, string> = {
  /** [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-6.shtml) */
  1: 'The complex numbers Fnm (See Code Table 3.6) are stored for M>=0 as pairs of real numbers Re(Fnm), lm(Fnm) ordered with n increasing from m to N(m), first for m=0 and then for m=1, 2, ... M (see note below).',
  // 0: Reserved
  // 2-254: Reserved
  255: 'Missing',
};

/**
 * # Table 3.8 - GRID POINT POSITION
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 32
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Ranges**:
 * - `6-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the grid point positions used in GRIB2 files,
 * specifying where grid points are located relative to grid shapes.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-8.shtml)
 *
 * ## Notes
 * - Revised 12/07/2023
 */
export const grib2LookupTable38: Record<number, string> = {
  0: 'Grid points at triangle vertices',
  1: 'Grid points at centers of triangles',
  2: 'Grid points at midpoints of triangle sides',
  3: 'Grid points at shape vertices',
  4: 'Grid points at centre of shapes',
  5: 'Grid points at midpoints of shape sides',
  // 6-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 3.9 - GRID POINT POSITION AS SEEN FROM THE CORRESPONDING POLE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 33
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Bits**:
 * - `2-8`: Reserved
 *
 * **Special Values**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the grid point positions as seen from the corresponding pole in GRIB2 files,
 * specifying where grid points are located relative to grid shapes.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-9.shtml)
 *
 * ## Notes
 * - Revised 12/07/2023
 */
export const grib2LookupTable39: Record<number, Record<number, string>> = {
  1: {
    0: 'Clockwise orientation',
    1: 'Counter-clockwise orientation',
  },
  // 2-8: Reserved
  255: 'Missing',
};

/**
 * # Table 3.10 - SCANNING MODE FOR ONE DIAMOND AS SEEN FROM THE CORRESPONDING POLE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 34
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Bits**:
 * - `4-8`: Reserved
 *
 * **Special Values**:
 * - None
 *
 * ## Description
 * This table defines the scanning mode flags for one diamond as seen from the corresponding pole in GRIB2 files,
 * specifying the scanning directions and grid points alignment.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-10.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 */
export const grib2LookupTable310: Record<number, Record<number, string>> = {
  1: {
    0: 'Points scan in the +i direction, i.e. from pole to Equator',
    1: 'Points scan in the -i direction, i.e. from Equator to pole',
  },
  2: {
    0: 'Points scan in the +j direction, i.e. from west to east',
    1: 'Points scan in the -j direction, i.e. from east to west',
  },
  3: {
    0: 'Adjacent points in the i direction are consecutive',
    1: 'Adjacent points in the j direction are consecutive',
  },
  // 4-8: Reserved
};

/**
 * # Table 3.11 - Interpretation of List of Numbers at End of Section 3
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 12
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Ranges**:
 * - `4-254`: Reserved
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the interpretation of the list of numbers appended at the end of Section 3 in GRIB2 files,
 * specifying how the numbers correspond to points in the grid based on various definitions.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-11.shtml)
 *
 * ## Notes
 * - (1) For entry 1, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, effective number of points per row may be less than the number of points on the current circle.
 * - (2) For value for the constant direction increment Di (or Dx) in the accompanying Grid Definition Template should be set to all ones (missing).
 */
export const grib2LookupTable311: Record<number, string> = {
  0: 'There is no appended list',
  1: 'Numbers define number of points corresponding to full coordinate circles (i.e. parallels). Coordinate values on each circle are multiple of the circle mesh, and extreme coordinate values given in grid definition may not be reached in all rows.',
  2: 'Numbers define number of points corresponding to coordinate lines delimited by extreme coordinate values given in grid definition which are present in each row.',
  3: 'Numbers define the actual latitudes for each row in the grid. The list of numbers are integer values of the valid latitudes in microdegrees (scale by 106) or in unit equal to the ratio of the basic angle and the subdivisions number for each row, in the same order as specified in the "scanning mode flag" (bit no. 2) (see note 2)',
  // 4-254: Reserved
  255: 'Missing',
};

/**
 * # Table 3.12 - HEALPix Rhomboids or Points Ordering
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 34
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the ordering of HEALPix rhomboids or points in GRIB2 files,
 * specifying how points are ordered within the HEALPix grid structure.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-12.shtml)
 *
 * ## Notes
 * - Created 12/07/2023
 */
export const grib2LookupTable312: Record<number, string> = {
  0: 'Reserved',
  1: 'Ring ordering',
  2: 'Nested ordering',
  // 3-191: Reserved
  // 192-254: Reserved for local use
  255: 'Missing',
};

/**
 * # Table 3.13 - HEALPix Scanning Mode
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 34
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Bits**:
 * - `4-8`: Reserved
 *
 * **Special Value**:
 * - None
 *
 * ## Description
 * This table defines the HEALPix scanning mode flags used in GRIB2 files,
 * specifying the scanning directions and grid points alignment.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-13.shtml)
 *
 * ## Notes
 * - Created 12/07/2023
 */
export const grib2LookupTable313: Record<number, Record<number, string>> = {
  1: {
    0: 'Points scan in the +i (+x) direction',
    1: 'Points scan in the -i (-x) direction',
  },
  2: {
    0: 'Points scan in -j (-y) direction',
    1: 'Points scan in +j (+y) direction',
  },
  3: {
    0: 'Adjacent points in the i (x) direction are consecutive',
    1: 'Adjacent points in the j (y) direction are consecutive',
  },
  // 4-8: Reserved
};

/**
 * # Table 3.15 - PHYSICAL MEANING OF VERTICAL COORDINATE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 63
 * - **Applicable Grid Templates**: 100
 *
 * **Reserved Ranges**:
 * - `0-19`: Reserved
 * - `21-99`: Reserved
 * - `114-159`: Reserved
 * - `161-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the physical meanings of vertical coordinates used in GRIB2 files,
 * specifying various vertical coordinate systems and their corresponding units.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-15.shtml)
 *
 * ## Notes
 * - (1) For entry 103, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, the effective number of points per row may be less than the number of points on the current circle.
 * - (2) For the value of the constant direction increment Di (or Dx) in the accompanying Grid Definition Template, it should be set to all ones (missing).
 */
export const grib2LookupTable315: Record<number, string> = {
  0: 'Reserved',
  // 1-19: Reserved
  20: 'Temperature (K)',
  // 21-99: Reserved
  100: 'Pressure (Pa)',
  101: 'Pressure deviation from mean sea level (Pa)',
  102: 'Altitude above mean sea level (m)',
  103: 'Height above ground (see note 1) (m)',
  104: 'Sigma coordinate',
  105: 'Hybrid coordinate',
  106: 'Depth below land surface (m)',
  107: 'Potential temperature (theta) (K)',
  108: 'Pressure deviation from ground to level (Pa)',
  109: 'Potential vorticity (K m-2 kg-1 s-1)',
  110: 'Geometric height (m)',
  111: 'Eta coordinate (see note 2)',
  112: 'Geopotential height (gpm)',
  113: 'Logarithmic hybrid coordinate',
  // 114-159: Reserved
  160: 'Depth below sea level (m)',
  // 161-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 3.20 - TYPE OF HORIZONTAL LINE AS SEEN FROM THE CORRESPONDING POLE
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 60
 * - **Applicable Grid Templates**: 1000, 1100
 *
 * **Reserved Ranges**:
 * - `2-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the types of horizontal lines used in GRIB2 files,
 * specifying whether lines are Rhumb or Great Circle, among other definitions.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-20.shtml)
 *
 * ## Notes
 * - Created 05/11/2005
 * - Red text in the original table depicts changes made since 05/11/2005.
 */
export const grib2LookupTable320: Record<number, string> = {
  0: 'Rhumb',
  1: 'Great Circle',
  // 2-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 3.21 - PHYSICAL MEANING OF VERTICAL COORDINATE VALUES DEFINITION
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: 64
 * - **Applicable Grid Templates**: 1000
 *
 * **Reserved Ranges**:
 * - `0-19`: Reserved
 * - `21-99`: Reserved
 * - `114-159`: Reserved
 * - `161-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the physical meanings of vertical coordinates used in GRIB2 files,
 * specifying various vertical coordinate systems and their corresponding units.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-21.shtml)
 *
 * ## Notes
 * - (1) For entry 103, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, the effective number of points per row may be less than the number of points on the current circle.
 * - (2) For the value for the constant direction increment Di (or Dx) in the accompanying Grid Definition Template should be set to all ones (missing).
 */
export const grib2LookupTable321: Record<number, string> = {
  // 0-19: Reserved
  20: 'Temperature (K)',
  // 21-99: Reserved
  100: 'Pressure (Pa)',
  101: 'Pressure deviation from mean sea level (Pa)',
  102: 'Altitude above mean sea level (m)',
  103: 'Height above ground (see note 1) (m)',
  104: 'Sigma coordinate',
  105: 'Hybrid coordinate',
  106: 'Depth below land surface (m)',
  107: 'Potential temperature (theta) (K)',
  108: 'Pressure deviation from ground to level (Pa)',
  109: 'Potential vorticity (K m-2 kg-1 s-1)',
  110: 'Geometric height (m)',
  111: 'Eta coordinate (see note 2)',
  112: 'Geopotential height (gpm)',
  113: 'Logarithmic hybrid coordinate',
  // 114-159: Reserved
  160: 'Depth below sea level (m)',
  // 161-191: Reserved
  // 192-254: Reserved for Local Use
  255: 'Missing',
};

/**
 * # Table 3.25 - TYPE OF BI-FOURIER TRUNCATION
 *
 * **Details**:
 * - **Section**: 3
 * - **Octet**: [Not Specified]
 * - **Applicable Grid Templates**: [Not Specified]
 *
 * **Reserved Ranges**:
 * - `0-76`: Reserved
 * - `78-87`: Reserved
 * - `89-98`: Reserved
 * - `100-191`: Reserved
 * - `192-254`: Reserved for Local Use
 *
 * **Special Value**:
 * - `255`: Missing
 *
 * ## Description
 * This table defines the types of Bi-Fourier truncation used in GRIB2 files,
 * specifying how spectral data is truncated in the horizontal direction.
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-25.shtml)
 *
 * ## Notes
 * - Created 06/22/2022
 */
export const grib2LookupTable325: Record<number, string> = {
  77: 'Rectangular',
  88: 'Elliptic',
  99: 'Diamond',
  255: 'Missing',
  // 0-76: Reserved
  // 78-87: Reserved
  // 89-98: Reserved
  // 100-191: Reserved
  // 192-254: Reserved for Local Use
};
