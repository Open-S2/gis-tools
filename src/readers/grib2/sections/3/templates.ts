// TEMPLATE INFO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
import {
  grib2LookupTable32,
  grib2LookupTable33,
  grib2LookupTable34,
  grib2LookupTable35,
} from './tables';

import type { Reader } from '../../..';
import type { Transformer } from '../../../../proj4';
import type { VectorPoint } from '../../../../geometry';

/**
 * Returns a template generator for the given template number
 * All templates are listed [here](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
 * @param template - template number parse block
 * @param section - byte block
 * @returns Template generator
 */
export function getGrib2Template3(template: number, section: Reader) {
  switch (template) {
    case 0:
      return grib2Template30(section);
    case 20:
      return grib2Template320(section);
    default:
      throw new Error(`Template 3.${template} not defined`);
  }
}

// TODO: template -> '+proj=tmerc +lat_0=0 +lon_0=75 +k=1 +x_0=500000 +y_0=0 +ellps=IAU76 +units=m +no_defs +type=crs'

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.0
 *
 * ## Latitude/Longitude (or equidistant cylindrical, or Plate Carree)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-0.shtml)
 *
 * ## Notes
 * - Basic angle of the initial production domain and subdivisions of this basic angle are provided
 * to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
 * extreme longitudes and latitudes, and direction increments. For these last six descriptors, the
 * unit is equal to the ratio of the basic angle and the subdivisions number. For ordinary cases,
 * zero and missing values should be coded, equivalent to respective values of 1 and 106  (10-6
 * degrees unit).
 * - For data on a quasi-regular grid, in which all the rows or columns do not necessarily have the
 * same number of grid points either Ni (octets 31-34) of Nj (octets 35-38) and the corresponding Di
 * (octets 64-67) or Dj (octets 68-71) shall be coded with all bits set to 1 (missing). The actual
 * number of points along each parallel or meridian shall be coded in the octets immediately following
 * the grid definition template (octets [xx+1]-nn), as described in the description of the grid
 * definition section.
 * - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or columns,
 * but not both simultaneously, may have variable numbers of points or variable spacing. The first
 * point in each row (column) shall be positioned at the meridian (parallel) indicted by octets 47-54.
 * The grid points shall be evenly spaced in latitude (longitude).
 * A scale value of radius of spherical Earth, or major axis of oblate spheroid Earth is delivered
 * from applying appropriate scale factor to the value expressed in meters.
 * - It is recommended to use unsigned direction increments.
 * - In most cases, multiplying Ni (octets 31-34) by Nj (octets 35-38) yields the total number of
 * points in the grid. However, this may not be true if bit 8 of the scanning mode flags (octet 72)
 * is set to 1.
 * @param section - byte block for template 3.0
 * @returns - The parsed template
 */
export function grib2Template30(section: Reader) {
  const shape = section.getUint8(14);
  const basicAngle = section.getUint32(38);
  const subdivisions = section.getUint32(42);
  const lat1 = section.getInt32(46);
  const lat2 = section.getInt32(55);
  // build resolution values
  const resolutionCode = section.getUint8(54);
  // Bit #3 from the left is (resolution >> (8 - 3)) & 1 == (resolution >> 5) & 1
  // But commonly people do the simpler approach: "bit #3" means shifting by 2 if reading docs carefully.
  // Let’s do it systematically to avoid confusion:
  const bit3 = (resolutionCode >> (8 - 3)) & 0x1; // i increments
  const bit4 = (resolutionCode >> (8 - 4)) & 0x1; // j increments
  const bit5 = (resolutionCode >> (8 - 5)) & 0x1; // vector resolution approach
  // build scanMode values
  const scanModeCode = section.getUint8(71);

  const ratio = basicAngle === 0 ? 1e-6 : basicAngle / subdivisions;

  return {
    /** Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml) */
    shape: {
      code: shape,
      value: grib2LookupTable32[shape],
    },
    /** Scale Factor of radius of spherical Earth */
    radiusScaleFactor: section.getUint8(15),
    /** Scale value of radius of spherical Earth */
    radiusScaleValue: section.getUint32(16),
    /** Scale factor of major axis of oblate spheroid Earth */
    majorAxisScaleFactor: section.getUint8(20),
    /** Scale value of major axis of oblate spheroid Earth */
    majorAxisScaleValue: section.getUint32(21),
    /** Scale factor of minor axis of oblate spheroid Earth */
    minorAxisScaleFactor: section.getUint8(25),
    /** Scale value of minor axis of oblate spheroid Earth */
    minorAxisScaleValue: section.getUint32(26),
    /** Number of points along a parallel (W-E) */
    nx: section.getUint32(30),
    /** Number of points along a meridian (N-S) */
    ny: section.getUint32(34),
    /** Basic angle of the initial production domain */
    basicAngle,
    /** Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments */
    subdivisions,
    /** Latitude of first grid point */
    lat1: (lat1 < 0 ? -(lat1 ^ 0x80000000) : lat1) * ratio,
    /** Longitude of first grid point */
    lon1: section.getInt32(50) * ratio,
    /** Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml) */
    resolution: {
      code: resolutionCode,
      value: {
        iDirectionIncrements: { code: bit3, value: grib2LookupTable33[3][bit3] },
        jDirectionIncrements: { code: bit4, value: grib2LookupTable33[4][bit4] },
        vectorComponentResolution: { code: bit5, value: grib2LookupTable33[5][bit5] },
      },
    },
    /** Latitude of last grid point */
    lat2: (lat2 < 0 ? -(lat2 ^ 0x80000000) : lat2) * ratio,
    /** Longitude of last grid point */
    lon2: section.getInt32(59) * ratio,
    /** i direction increment */
    dx: section.getInt32(63) * ratio,
    /** j direction increment */
    dy: section.getInt32(67) * ratio,
    /** Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml) */
    scanMode: {
      code: scanModeCode,
      value: parseScanMode(scanModeCode),
    },
    /** Grid Units */
    gridUnits: 'degrees',
    /**
     * Convert this section into grid data
     * @returns - grid data
     */
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // for now let's just follow the most basic scan mode
      const { lat1, lat2, lon1, lon2, nx, ny } = this;
      // Step sizes for interpolation
      const lonStep = (lon2 - lon1) / (nx - 1);
      const latStep = (lat2 - lat1) / (ny - 1);

      const res: VectorPoint<Record<string, number>>[] = [];

      for (let y = 0; y < ny; y++) {
        for (let x = 0; x < nx; x++) {
          // Interpolate longitude and latitude
          const lon = lon1 + x * lonStep;
          const lat = lat1 + y * latStep;
          // create point and apply transform if provided (this grid is already in the correct projection)
          res.push({ x: lon, y: lat, m: {} });
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.20
 *
 * ## Polar Stereographic Projection (Can be North or South)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-20.shtml)
 *
 * ## Notes
 * - The orientation of the grid is given by the longitude of the meridian along which the
 *   y-axis increases, LoV.
 * - The projection is defined by the latitude at which Dx and Dy are specified, LaD.
 * - Grid lengths Dx and Dy are in meters at the latitude LaD.
 * - Bit 3 of the resolution and component flags should be set to 1 to indicate that Dx and Dy
 *   are given in meters.
 * @param section - byte block for template 3.20
 * @returns - The parsed template
 */
export function grib2Template320(section: Reader) {
  const shape = section.getUint8(14);
  const lat1 = section.getInt32(38);
  const lon1 = section.getInt32(42);
  const latD = section.getInt32(47);
  const lonV = section.getInt32(51);
  const dx = section.getInt32(55);
  const dy = section.getInt32(59);
  const projCenter = section.getUint8(63);
  // build resolution values
  const resolutionCode = section.getUint8(54);
  // Bit #3 from the left is (resolution >> (8 - 3)) & 1 == (resolution >> 5) & 1
  // But commonly people do the simpler approach: "bit #3" means shifting by 2 if reading docs carefully.
  // Let’s do it systematically to avoid confusion:
  const bit3 = (resolutionCode >> (8 - 3)) & 0x1; // i increments
  const bit4 = (resolutionCode >> (8 - 4)) & 0x1; // j increments
  const bit5 = (resolutionCode >> (8 - 5)) & 0x1; // vector resolution approach
  // build scan mode
  const scanModeCode = section.getUint8(64);

  return {
    /** Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml) */
    shape: {
      code: shape,
      value: grib2LookupTable32[shape],
    },
    /** Scale Factor of radius of spherical Earth */
    radiusScaleFactor: section.getUint8(15),
    /** Scale value of radius of spherical Earth */
    radiusScaleValue: section.getUint32(16),
    /** Scale factor of major axis of oblate spheroid Earth */
    majorAxisScaleFactor: section.getUint8(20),
    /** Scale value of major axis of oblate spheroid Earth */
    majorAxisScaleValue: section.getUint32(21),
    /** Scale factor of minor axis of oblate spheroid Earth */
    minorAxisScaleFactor: section.getUint8(25),
    /** Scale value of minor axis of oblate spheroid Earth */
    minorAxisScaleValue: section.getUint32(26),
    /** Number of points along the x-axis */
    nx: section.getUint32(30),
    /** Number of points along the y-axis */
    ny: section.getUint32(34),
    /** Latitude of first grid point */
    lat1,
    /** Longitude of first grid point */
    lon1,
    /** Latitude where Dx and Dy are specified */
    latD,
    /** Orientation of the grid (LoV) */
    lonV,
    /** Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml) */
    resolution: {
      code: resolutionCode,
      value: {
        iDirectionIncrements: { code: bit3, value: grib2LookupTable33[3][bit3] },
        jDirectionIncrements: { code: bit4, value: grib2LookupTable33[4][bit4] },
        vectorComponentResolution: { code: bit5, value: grib2LookupTable33[5][bit5] },
      },
    },
    /** x-direction grid length (meters at LaD) */
    dx,
    /** y-direction grid length (meters at LaD) */
    dy,
    /** Projection center flag [Table 3.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml) */
    projCenter: {
      code: projCenter,
      value: parseProjectionCenter(projCenter),
    },
    /** Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml) */
    scanMode: {
      code: scanModeCode,
      value: parseScanMode(scanModeCode),
    },
    /** Grid Units */
    gridUnits: 'meters',
    /**
     * Convert this section into grid data
     * @param transformer - projection transformer
     * @returns - grid data
     */
    buildGrid: function (transformer?: Transformer): VectorPoint<Record<string, number>>[] {
      // for now let's just follow the most basic scan mode
      const { lat1, dx, lon1, dy, nx, ny } = this;
      const res: VectorPoint<Record<string, number>>[] = [];

      // TODO: Setup transformer for polar stereographic. The `projCenter.value.plane` chooses North and South

      for (let y = 0; y < ny; y++) {
        for (let x = 0; x < nx; x++) {
          // Interpolate longitude and latitude
          const lon = lon1 + x * dx;
          const lat = lat1 + y * dy;
          // create point
          let point: VectorPoint<Record<string, number>> = { x: lon, y: lat, m: {} };
          // apply transform if provided
          if (transformer !== undefined) point = transformer.forward(point);
          res.push(point);
        }
      }

      return res;
    },
  };
}

/**
 * Get all projection center values
 * @param projCenter - projection center code
 * @returns - parsed projection center
 */
function parseProjectionCenter(projCenter: number) {
  /**
   * For bits #1..8, shift to put that bit in LSB position and mask.
   * GRIB2 docs say "Bit 1" is the leftmost bit, so bit #1 is (projCenter >> (8 - 1)) & 1, etc.
   * @param bitPos - bit position not index
   * @returns - binary value
   */
  const getBit = (bitPos: number) => (projCenter >> (8 - bitPos)) & 0x1;

  return {
    // 0 = North, 1 = South
    plane: { code: getBit(1), value: grib2LookupTable35[1][getBit(1)] },
    // 0 = Only one projection center is used, 1 = Projection is bi-polar and symmetric
    biPolar: { code: getBit(2), value: grib2LookupTable35[2][getBit(2)] },
  };
}

/**
 * Get all scan mode values describing how to read the data
 * @param scanMode - scan mode code
 * @returns - parsed scan mode
 */
function parseScanMode(scanMode: number) {
  /**
   * For bits #1..8, shift to put that bit in LSB position and mask.
   * GRIB2 docs say "Bit 1" is the leftmost bit, so bit #1 is (scanMode >> (8 - 1)) & 1, etc.
   * @param bitPos - bit position not index
   * @returns - binary value
   */
  const getBit = (bitPos: number) => (scanMode >> (8 - bitPos)) & 0x1;

  const bit1 = getBit(1);
  const bit2 = getBit(2);
  const bit3 = getBit(3);
  const bit4 = getBit(4);
  const bit5 = getBit(5);
  const bit6 = getBit(6);
  const bit7 = getBit(7);
  const bit8 = getBit(8);

  return {
    xDir: { code: bit1, value: grib2LookupTable34[1][bit1] },
    yDir: { code: bit2, value: grib2LookupTable34[2][bit2] },
    adjacentDir: { code: bit3, value: grib2LookupTable34[3][bit3] },
    rowDir: { code: bit4, value: grib2LookupTable34[4][bit4] },
    rowOffset: { code: bit5, value: grib2LookupTable34[5][bit5] },
    pointOffsetX: { code: bit6, value: grib2LookupTable34[6][bit6] },
    pointOffsetY: { code: bit7, value: grib2LookupTable34[7][bit7] },
    rowGridRule: { code: bit8, value: grib2LookupTable34[8][bit8] },
  };
}
