// TEMPLATE INFO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
import {
  grib2LookupTable32,
  grib2LookupTable33,
  grib2LookupTable34,
  grib2LookupTable35,
  grib2LookupTable36,
  grib2LookupTable37,
} from './tables.js';
import { llNormalize, readGribInt } from '../../../../index.js';

import type { Reader } from '../../../index.js';
import type { VectorPoint } from '../../../../geometry/index.js';

// grib individually ports all of proj4 as well:
// https://github.com/NOAA-EMC/wgrib2/blob/develop/src/geo.c#L262

/** The output of `getGrib2Template4` */
export type Grib2SphericalHarmonicCoefficients = ReturnType<typeof grib2Template350>;

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
    case 1:
      return grib2Template31(section);
    case 10:
      return grib2Template310(section);
    case 20:
      return grib2Template320(section);
    case 30:
      return grib2Template330(section);
    case 40:
      return grib2Template340(section);
    case 50:
      return grib2Template350(section);
    default:
      throw new Error(`Template 3.${template} not defined`);
  }
}
// complex templates can look like -> '+proj=tmerc +lat_0=0 +lon_0=75 +k=1 +x_0=500000 +y_0=0 +ellps=IAU76'

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
    lat1: readGribInt(section.getUint32(46)) * ratio,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(50)) * ratio,
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
    lat2: readGribInt(section.getUint32(55)) * ratio,
    /** Longitude of last grid point */
    lon2: readGribInt(section.getUint32(59)) * ratio,
    /** i direction increment */
    dx: readGribInt(section.getUint32(63)) * ratio,
    /** j direction increment */
    dy: readGribInt(section.getUint32(67)) * ratio,
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
        const lat = lat1 + y * latStep;
        for (let x = 0; x < nx; x++) {
          // Interpolate longitude and latitude
          const lon = lon1 + x * lonStep;
          // create point and apply transform if provided (this grid is already in the correct projection)
          res.push(llNormalize({ x: lon, y: lat, m: {} }));
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.1
 *
 * ## Rotate Latitude/Longitude (or equidistant/cylindrical, or Plate Carree)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1.shtml)
 *
 * ## Notes
 * - Basic angle of the initial production domain and subdivisions of this basic angle are provided
 * to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
 * extreme longitudes and latitudes, and direction increments. For these last six descriptors, the
 * unit is equal to the ratio of the basic angle and the subdivisions number. For ordinary cases,
 * zero and missing values should be coded, equivalent to respective values of 1 and 106  (10-6
 * degrees unit).
 * - Three parameters define a general latitude/longitude coordinate system, formed by a general
 * rotation of the sphere. One choice for these parameters is:
 *   - (a) The geographic latitude in degrees of the southern pole of the coordinate system,06 for example.
 *   - (b) The geographic longitude in degrees of the southern pole of the coordinate system,λp for example.
 *   - (c) The angle of rotation in degrees about the new polar axis (measured clockwise when looking
 *         from the southern to the northern pole) of the coordinate system, assuming the new axis to
 *         have been obtained by first rotating the sphere through λp degrees about the geographic
 *         polar axis and then rotating through (90 + 0p) degrees so that the southern pole moved along
 *         the (previously rotated) Greenwich meridian.
 * - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or columns,
 * but not both simultaneously, may have variable numbers of points or variable spacing. The first
 * point in each row (column) shall be positioned at the meridian (parallel) indicted by octets 47-54.
 * The grid points shall be evenly spaced in latitude (longitude).
 * - It is recommended to use unsigned direction increments.
 * @param section - byte block for template 3.1
 * @returns - The parsed template
 */
export function grib2Template31(section: Reader) {
  const shape = section.getUint8(14);
  const basicAngle = section.getUint32(38);
  const subdivisions = section.getUint32(42);
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
    lat1: readGribInt(section.getUint32(46)) * ratio,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(50)) * ratio,
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
    lat2: readGribInt(section.getUint32(55)) * ratio,
    /** Longitude of last grid point */
    lon2: readGribInt(section.getUint32(59)) * ratio,
    /** i direction increment */
    dx: readGribInt(section.getUint32(63)) * ratio,
    /** j direction increment */
    dy: readGribInt(section.getUint32(67)) * ratio,
    /** Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml) */
    scanMode: {
      code: scanModeCode,
      value: parseScanMode(scanModeCode),
    },
    /** Latitude of the southern pole of projection */
    latSP: readGribInt(section.getUint32(72)) * ratio,
    /** Longitude of the southern pole of projection */
    lonSP: readGribInt(section.getUint32(76)) * ratio,
    /** Angle of rotation of projection */
    rotAngle: readGribInt(section.getUint32(80)) * ratio,
    /** Grid Units */
    gridUnits: 'degrees',
    /**
     * Convert this section into grid data
     * @returns - grid data
     */
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // https://github.com/NOAA-EMC/wgrib2/blob/develop/src/rotll.c#L89
      const { sin, cos, asin, atan2, PI } = Math;
      const deg2rad = PI / 180;
      const rad2deg = 180 / PI;
      const { lat1, lat2, lon1, lon2, nx, ny, latSP, lonSP, rotAngle } = this;

      // inverse transformation, reverse rotation angle
      const angleRot = -rotAngle;

      const a = deg2rad * (90.0 + latSP);
      const b = deg2rad * lonSP;
      const r = deg2rad * angleRot;
      const sinA = sin(a);
      const cosA = cos(a);

      // south pole to north pole
      const lonStep = (lon2 - lon1) / (nx - 1);
      const latStep = (lat2 - lat1) / (ny - 1);
      const res = [];

      for (let y = 0; y < ny; y++) {
        const pr = (lat1 + y * latStep) * deg2rad;
        for (let x = 0; x < nx; x++) {
          // Current point in rotated degrees, set to radian, adjust for rotation
          const gr = (lon1 + x * lonStep) * -deg2rad;
          const pm = asin(cos(pr) * cos(gr));
          const gm = atan2(cos(pr) * sin(gr), -sin(pr));

          const glat = rad2deg * asin(sinA * sin(pm) - cosA * cos(pm) * cos(gm - r));
          const glon =
            -rad2deg *
            (-b + atan2(cos(pm) * sin(gm - r), sinA * cos(pm) * cos(gm - r) + cosA * sin(pm)));

          res.push(llNormalize({ x: glon, y: glat, m: {} }));
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.10
 *
 * ## Mercator
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-10.shtml)
 *
 * ## Notes
 * - Limited to the range of  0 to 90 degrees; if the angle of orientation of the grid is neither 0
 *   nor 90 degrees, Di and Dj must be equal to each other.
 * - Grid lengths are in units of 10-3  m, at the latitude specified by LaD.
 * - A scale value of radius of spherical Earth, or major or minor axis of oblate spheroid Earth is
 *   derived from applying appropriate scale factor to the value expressed in metres.
 * @param section - byte block for template 3.10
 * @returns - The parsed template
 */
export function grib2Template310(section: Reader) {
  const shape = section.getUint8(14);
  // build resolution values
  const resolutionCode = section.getUint8(46);
  // Bit #3 from the left is (resolution >> (8 - 3)) & 1 == (resolution >> 5) & 1
  // But commonly people do the simpler approach: "bit #3" means shifting by 2 if reading docs carefully.
  // Let’s do it systematically to avoid confusion:
  const bit3 = (resolutionCode >> (8 - 3)) & 0x1; // i increments
  const bit4 = (resolutionCode >> (8 - 4)) & 0x1; // j increments
  const bit5 = (resolutionCode >> (8 - 5)) & 0x1; // vector resolution approach
  // build scanMode values
  const scanModeCode = section.getUint8(59);

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
    /** Latitude of first grid point */
    lat1: readGribInt(section.getUint32(38)) * 1e-6,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(42)) * 1e-6,
    /** Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml) */
    resolution: {
      code: resolutionCode,
      value: {
        iDirectionIncrements: { code: bit3, value: grib2LookupTable33[3][bit3] },
        jDirectionIncrements: { code: bit4, value: grib2LookupTable33[4][bit4] },
        vectorComponentResolution: { code: bit5, value: grib2LookupTable33[5][bit5] },
      },
    },
    /** LaD — latitude(s) at which the Mercator projection intersects the Earth (Latitude(s) where Di and Dj are specified) */
    latD: readGribInt(section.getUint32(47)) * 1e-6,
    /** Latitude of last grid point */
    lat2: readGribInt(section.getUint32(51)) * 1e-6,
    /** Longitude of last grid point */
    lon2: readGribInt(section.getUint32(55)) * 1e-6,
    /** Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml) */
    scanMode: {
      code: scanModeCode,
      value: parseScanMode(scanModeCode),
    },
    /** Orientation of the grid, angle between i direction on the map and the Equator  (see Note1) */
    orient: readGribInt(section.getUint32(60)) * 1e-6,
    /** Dj —  longitudinal direction grid length (see Note 2) */
    dx: readGribInt(section.getUint32(64)) * 1e-3,
    /** Di —  latitudinal direction grid length (see Note 2) */
    dy: readGribInt(section.getUint32(68)) * 1e-3,
    /** Grid Units */
    gridUnits: 'degrees',
    /**
     * Convert this section into grid data
     * @returns - grid data
     */
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/geo.c#L577
      const { cos, exp, atan, PI, abs, log, tan, min, max } = Math;
      const deg2rad = PI / 180;
      const rad2deg = 180 / PI;
      const { lat1, lat2, lon1, lon2, latD, nx, ny, scanMode, dx } = this;

      if (lon1 < 0.0 || lon2 < 0.0 || lon1 > 360.0 || lon2 > 360.0)
        throw new Error('BAD grid definition lon');
      if (lat1 < -90.0 || lat2 < -90.0 || lat1 > 90.0 || lat2 > 90.0)
        throw new Error('BAD grid definition lat');

      if (nx < 1 || ny < 1)
        throw new Error('Sorry geo/mercator code does not handle variable nx/ny yet\n');

      // Mercator projection math requires s < n.
      const sLat = min(lat1, lat2);
      const nLat = max(lat1, lat2);

      // Longitude handling
      const [wLon, eLon] = scanMode.value.xDir.code === 0 ? [lon1, lon2] : [lon2, lon1];

      let eLonAdj = eLon;
      if (eLonAdj <= wLon) eLonAdj += 360.0;

      const dlon = (eLonAdj - wLon) / (nx - 1.0);
      const radius = getEarthRadius(
        this.shape.code,
        this.radiusScaleFactor,
        this.radiusScaleValue,
        this.majorAxisScaleFactor,
        this.majorAxisScaleValue,
        this.minorAxisScaleFactor,
        this.minorAxisScaleValue,
      );

      const circum = 2.0 * PI * radius * cos(latD * deg2rad);
      let dxDeg = (dx * 360.0) / circum;

      if (dxDeg !== 0.0) {
        const error = abs(dxDeg - dlon) / abs(dxDeg);
        if (error >= 0.001) {
          console.warn(`Warning: Inconsistent dx vs dlon: ${dxDeg} vs ${dlon}`);
        }
        dxDeg = dlon; // Domain trumps calculated resolution
      }

      // Map degrees to the Mercator projected Y space
      const sProj = log(tan((45.0 + sLat / 2.0) * deg2rad));
      const nProj = log(tan((45.0 + nLat / 2.0) * deg2rad));
      const dyProj = (nProj - sProj) / (ny - 1.0);

      // Determine Iteration Start and Step
      // Bit 2: 0 = Southward (Top-Down), 1 = Northward (Bottom-Up)
      const [yStartProj, yStep] =
        scanMode.value.yDir.code === 1 ? [sProj, dyProj] : [nProj, -dyProj];
      // Bit 1: 0 = Eastward (+i), 1 = Westward (-i)
      const xStep = scanMode.value.xDir.code === 0 ? dxDeg : -dxDeg;

      const res = [];
      for (let j = 0; j < ny; j++) {
        const currentYProj = yStartProj + j * yStep;
        // Reverse Mercator: Convert projected Y back to Latitude
        const rowLat = (atan(exp(currentYProj)) * rad2deg - 45.0) * 2.0;
        for (let i = 0; i < nx; i++) {
          const colLon = lon1 + i * xStep;
          res.push(llNormalize({ x: colLon, y: rowLat, m: {} }));
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
    lat1: readGribInt(section.getUint32(38)) * 1e-6,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(42)) * 1e-6,
    /** Latitude where Dx and Dy are specified */
    latD: readGribInt(section.getUint32(47)) * 1e-6,
    /** Orientation of the grid (LoV) */
    lonV: readGribInt(section.getUint32(51)) * 1e-6,
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
    dx: readGribInt(section.getUint32(55)) * 1e-3,
    /** y-direction grid length (meters at LaD) */
    dy: readGribInt(section.getUint32(59)) * 1e-3,
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
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/Proj4.c#L154
      const { lat1, dx, lon1, dy, nx, ny, lonV, latD, projCenter, scanMode } = this;
      const { abs, sin, cos, atan2, asin, PI } = Math;
      const rad2deg = 180 / Math.PI;
      const deg2rad = Math.PI / 180;

      // 1. Math always uses absolute dx/dy for projection constants
      const absDx = abs(dx);
      const absDy = abs(dy);

      const lat1Rad = lat1 * deg2rad;
      const lon1Rad = lon1 * deg2rad;
      let orient = lonV * deg2rad;
      const latDRad = abs(latD) * deg2rad;
      let h = 1.0;

      if (projCenter.value.plane.code === 1) {
        h = -1;
        orient -= PI;
      }

      const radius = getEarthRadius(
        this.shape.code,
        this.radiusScaleFactor,
        this.radiusScaleValue,
        this.majorAxisScaleFactor,
        this.majorAxisScaleValue,
        this.minorAxisScaleFactor,
        this.minorAxisScaleValue,
      );

      const de = (1.0 + sin(latDRad)) * radius;
      const dr = (de * cos(lat1Rad)) / (1 + h * sin(lat1Rad));

      // Calculate xp/yp using absolute values as in the original iplib/wgrib2 logic
      let xp = (-h * sin(lon1Rad - orient) * dr) / absDx;
      let yp = (cos(lon1Rad - orient) * dr) / absDy;

      // 2. Adjust xp/yp offsets based on Scan Mode (mimicking wgrib2 logic)
      // Bit 1: 0 = +i (East), 1 = -i (West)
      if (scanMode.value.xDir.code === 1) {
        xp = xp - nx + 1.0;
      }
      // Bit 2: 0 = -j (South/Top-Down), 1 = +j (North/Bottom-Up)
      if (scanMode.value.yDir.code === 0) {
        yp = yp - ny + 1.0;
      }

      const de2 = de * de;

      const res = [];
      for (let iy = 0; iy < ny; iy++) {
        const dj = (iy - yp) * absDy;
        for (let ix = 0; ix < nx; ix++) {
          const di = (ix - xp) * absDx;

          const dr2 = di * di + dj * dj;
          const [lon, lat] =
            dr2 < de2 * 1e-6
              ? [0.0, h * 90.0]
              : [
                  (orient + h * atan2(di, -dj)) * rad2deg,
                  h * asin((de2 - dr2) / (de2 + dr2)) * rad2deg,
                ];

          res.push(llNormalize({ x: lon, y: lat, m: {} }));
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.30
 *
 * ## Lambert Conformal
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)
 *
 * ## Notes
 * - Grid lengths are in units on 10-3 m, at the latitude specified by LaD.
 * - If Latin 1 = Latin 2, then the projection is on a tangent cone.
 * - The resolution flags (bits 3-4 of Flag table 3.3) are not applicable.
 * - LoV is the longitude value of the meridian which is parallel to the y-axis (or columns of the
 * grid) along which latitude increases as the y-coordinate increase (the orientation longitude may
 * or may not appear on a particular grid).
 * - A scale value of radius of spherical Earth, or major or minor axis of oblate spheroid Earth is
 * derived from applying appropriate scale factor to the value expressed in metres.
 * @param section - byte block for template 3.30
 * @returns - The parsed template
 */
export function grib2Template330(section: Reader) {
  const shape = section.getUint8(14);
  const projCenter = section.getUint8(63);
  // build resolution values
  const resolutionCode = section.getUint8(46);
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
    lat1: readGribInt(section.getUint32(38)) * 1e-6,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(42)) * 1e-6,
    /** Latitude where Dx and Dy are specified */
    latD: readGribInt(section.getUint32(47)) * 1e-6,
    /** Orientation of the grid (LoV) */
    lonV: readGribInt(section.getUint32(51)) * 1e-6,
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
    dx: readGribInt(section.getUint32(55)) * 1e-3,
    /** y-direction grid length (meters at LaD) */
    dy: readGribInt(section.getUint32(59)) * 1e-3,
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
    /** Latin 1 ― first latitude from the pole at which the secant cone cuts the sphere */
    latin1: readGribInt(section.getUint32(65)) * 1e-6,
    /** Latin 2 ― second latitude from the pole at which the secant cone cuts the sphere */
    latin2: readGribInt(section.getUint32(69)) * 1e-6,
    /** Latitude of the southern pole of projection */
    latSouth: readGribInt(section.getUint32(73)) * 1e-6,
    /** Longitude of the southern pole of projection */
    lonSouth: readGribInt(section.getUint32(77)) * 1e-6,
    /** Grid Units */
    gridUnits: 'meters',
    /**
     * Convert this section into grid data
     * @param transformer - projection transformer
     * @returns - grid data
     */
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/Proj4.c#L154
      const { lat1, dx, lon1, dy, nx, ny, lonV, latD, latin1, latin2, scanMode } = this;
      const { abs, sin, cos, log, tan, PI, pow, atan, sqrt } = Math;
      const deg2rad = Math.PI / 180;
      const rad2deg = 180 / Math.PI;
      const fracPi4 = PI / 4;
      const fracPi2 = PI / 2;

      // Step 1: Prep all variables for polar projection. Convert degrees to radians
      const lat1r = lat1 * deg2rad;
      const lon1r = lon1 * deg2rad;
      const lon2d = lonV;
      const lon2r = lonV * deg2rad;
      const latin1r = latin1 * deg2rad;
      const latin2r = latin2 * deg2rad;
      const latdr = latD * deg2rad;

      if (lon1r < 0) {
        throw new Error!('bad grid definition, lon1r < 0.0');
      }

      const n =
        abs(latin1r - latin2r) < 1e-9
          ? sin(latin1r)
          : log(cos(latin1r) / cos(latin2r)) /
            log(tan(fracPi4 + latin2r / 2.0) / tan(fracPi4 + latin1r / 2.0));
      const radius = getEarthRadius(
        this.shape.code,
        this.radiusScaleFactor,
        this.radiusScaleValue,
        this.majorAxisScaleFactor,
        this.majorAxisScaleValue,
        this.minorAxisScaleFactor,
        this.minorAxisScaleValue,
      );
      const f = (cos(latin1r) * pow(tan(fracPi4 + latin1r / 2.0), n)) / n;
      const rho = radius * f * pow(tan(fracPi4 + lat1r / 2.0), -n);
      // old rhoref = radius * f * pow(tan(fracPi4 + latin1r/2.0),-n);
      const rhoref = radius * f * pow(tan(fracPi4 + latdr / 2.0), -n);

      // 2/2009 .. new code
      let dLon = lon1r - lon2r;
      if (dLon > PI) {
        dLon -= 2 * PI;
      }
      if (dLon < -PI) {
        dLon += 2 * PI;
      }

      const thetaAngle = n * dLon;
      const startx = rho * sin(thetaAngle);
      const starty = rhoref - rho * cos(thetaAngle);

      const iStep = scanMode.value.xDir.code === 1 ? -dx : dx;
      const jStep = scanMode.value.yDir.code === 0 ? -dy : dy;

      const res: VectorPoint<Record<string, number>>[] = [];
      // for j in 0..ny {
      for (let j = 0; j < ny; j++) {
        const y = starty + j * jStep;
        // for i in 0..nx {
        for (let i = 0; i < nx; i++) {
          // Interpolate longitude and latitude
          const x = startx + i * iStep;
          const tmp = rhoref - y;
          const theta = atan(x / tmp);
          let rho = sqrt(x * x + tmp * tmp);
          rho = n > 0 ? rho : -rho;
          let lond = lon2d + (theta / n) * rad2deg;
          const lat = (2.0 * atan(pow((radius * f) / rho, 1.0 / n)) - fracPi2) * rad2deg;
          lond = lond >= 360.0 ? lond - 360.0 : lond;
          lond = lond < 0.0 ? lond + 360.0 : lond;

          res.push(llNormalize({ x: lond, y: lat, m: {} }));
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.40
 *
 * ## Gaussian Latitude/Longitude
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)
 *
 * ## Notes
 * - Basic angle of the initial production domain and subdivisions of this basic angle are provided
 * to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
 * extreme longitudes and latitudes, and direction increments. For these last six descriptors, unit
 * is equal to the ratio of the equivalent to respective values of 1 and 106 (10-6 degrees unit).
 * -  The number of parallels between a pole and the equator is used to establish the variable
 * (Gaussian) spacing of the parallels; this value must always be given.
 * - A scaled value of radius of spherical Earth, or major or minor axis of oblate spheriod Earth
 * is derived from applying appropriate scale factor to the value expressed in metres.
 * - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or
 * columns, but not both simultaneously, may have variable numbers of points. The first point in
 * each row(column) shall be positioned at the meridian (parallel) indicated by Octets 47-54. The
 * grid points shall be evenly spaced in latitude (longitude).
 * - It is recommended to use unsigned direction increments.
 * @param section - byte block for template 3.40
 * @returns - The parsed template
 */
export function grib2Template340(section: Reader) {
  const shape = section.getUint8(14);
  const basicAngle = section.getUint32(38);
  const subdivisions = section.getUint32(42);
  // build resolution values
  const resolutionCode = section.getUint8(54);
  // Bit #3 from the left is (resolution >> (8 - 3)) & 1 == (resolution >> 5) & 1
  // But commonly people do the simpler approach: "bit #3" means shifting by 2 if reading docs carefully.
  // Let’s do it systematically to avoid confusion:
  const bit3 = (resolutionCode >> (8 - 3)) & 0x1; // i increments
  const bit4 = (resolutionCode >> (8 - 4)) & 0x1; // j increments
  const bit5 = (resolutionCode >> (8 - 5)) & 0x1; // vector resolution approach
  // build scan mode
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
    /** Number of points along the x-axis */
    nx: section.getUint32(30),
    /** Number of points along the y-axis */
    ny: section.getUint32(34),
    /** Basic angle of the initial production domain */
    basicAngle,
    /** Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments */
    subdivisions,
    /** Latitude of first grid point */
    lat1: readGribInt(section.getUint32(46)) * ratio,
    /** Longitude of first grid point */
    lon1: readGribInt(section.getUint32(50)) * ratio,
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
    lat2: readGribInt(section.getUint32(55)) * ratio,
    /** Longitude of last grid point */
    lon2: readGribInt(section.getUint32(59)) * ratio,
    /** i direction increment */
    dx: readGribInt(section.getUint32(63)) * ratio,
    /** N - number of paralells between a pole and the equator (see Note 2) */
    n: section.getUint32(67),
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
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/Proj4.c#L154
      const { lon1, nx, ny, scanMode, dx } = this;
      const res = [];

      // 1. Generate latitudes for the full globe
      const allLats = generateGaussianLats(ny);

      // 2. Find where our specific grid starts in the global Gaussian sequence
      const isScanJPositive = scanMode.value.yDir.code === 1;
      if (isScanJPositive) allLats.reverse();

      // 3. Step sizes for Longitude (Regularly spaced)
      // Note: GRIB2 Gaussian grids are often global (lon2 is lon1 + span)
      const dLon = scanMode.value.xDir.code === 0 ? Math.abs(dx) : -Math.abs(dx);

      for (let j = 0; j < ny; j++) {
        const lat = allLats[j];

        for (let i = 0; i < nx; i++) {
          const lon = lon1 + i * dLon;

          res.push(llNormalize({ x: lon, y: lat, m: {} }));
        }
      }

      return res;
    },
  };
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.50
 *
 * ## Spherical Harmonic Coefficients
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-50.shtml)
 *
 * ## Notes
 * - The pentagonal representation of resolution is general. Some common truncations are special
 *   cases of the pentagonal one:
 *   - Triangular: M = J = K
 *   - Rhomboidal: K = J + M
 *   - Trapezoidal: K = J, K > M
 * @param section - byte block for template 3.50
 * @returns - The parsed template
 */
export function grib2Template350(section: Reader) {
  const repTypeCode = section.getUint8(26);
  const repModeCode = section.getUint8(27);
  return {
    /** J ― pentagonal resolution parameter */
    j: section.getUint32(14),
    /** K ― pentagonal resolution parameter */
    k: section.getUint32(18),
    /** M ― pentagonal resolution parameter */
    m: section.getUint32(22),
    /** Representation type indicating the method used to define the norm (see Code Table 3.6) */
    representationType: {
      code: repTypeCode,
      value: grib2LookupTable36[repTypeCode],
    },
    /** Representation mode indicating the order of the coefficients (see Code Table 3.7) */
    representationMode: {
      code: repModeCode,
      value: grib2LookupTable37[repModeCode],
    },
    /**
     * Convert this section into grid data
     * @param transformer - projection transformer
     * @returns - grid data
     */
    buildGrid: function (): VectorPoint<Record<string, number>>[] {
      throw new Error('Spherical Harmonic Coefficients buildGrid is Not implemented');
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

function getEarthRadius(
  code: number,
  radiusScaleFactor: number,
  radiusScaleValue: number,
  majorAxisScaleFactor: number,
  majorAxisScaleValue: number,
  minorAxisScaleFactor: number,
  minorAxisScaleValue: number,
): number {
  const { a: radius_major, b: radius_minor } = getEarthDimensions(
    code,
    radiusScaleFactor,
    radiusScaleValue,
    majorAxisScaleFactor,
    majorAxisScaleValue,
    minorAxisScaleFactor,
    minorAxisScaleValue,
  );
  const radius = 0.5 * (radius_major + radius_minor);

  if (radius < 6300000.0 || radius > 6400000.0) {
    // default is WGS84
    return 6_367_444.6225;
  } else {
    return radius;
  }
}

/**
 * Calculates the semi-major (a) and semi-minor (b) axes of the Earth
 * based on GRIB2 Table 3.2.
 *
 * @param code - Earth shape code
 * @param radiusScaleFactor - scale factor for radius
 * @param radiusScaleValue - scale value for radius
 * @param majorAxisScaleFactor - scale factor for major axis
 * @param majorAxisScaleValue - scale value for major axis
 * @param minorAxisScaleFactor - scale factor for minor axis
 * @param minorAxisScaleValue - scale value for minor axis
 * @returns - semi-major and semi-minor axes
 */
export function getEarthDimensions(
  code: number,
  radiusScaleFactor: number,
  radiusScaleValue: number,
  majorAxisScaleFactor: number,
  majorAxisScaleValue: number,
  minorAxisScaleFactor: number,
  minorAxisScaleValue: number,
): { a: number; b: number } {
  // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/Earth.c#L169
  // Helper to convert GRIB2 scale factor/value to a real number
  const calc = (factor: number, value: number) => value * Math.pow(10, -factor);

  switch (code) {
    case 0:
      return { a: 6367470.0, b: 6367470.0 };
    case 1: {
      const r = calc(radiusScaleFactor, radiusScaleValue);
      return { a: r, b: r };
    }
    case 2:
      return { a: 6378160.0, b: 6356775.0 };
    case 3: {
      // Code 3 specifies values in km, convert to meters
      const a = calc(majorAxisScaleFactor, majorAxisScaleValue) * 1_000;
      const b = calc(minorAxisScaleFactor, minorAxisScaleValue) * 1_000;
      return { a, b };
    }
    case 4: // GRS80
      return { a: 6378137.0, b: 6356752.14 };
    case 5: // WGS84 (GRIB2 treats as GRS80 basis)
      return { a: 6378137.0, b: 6356752.245 };
    case 6:
      return { a: 6371229.0, b: 6371229.0 };
    case 7: {
      const a = calc(majorAxisScaleFactor, majorAxisScaleValue);
      const b = calc(minorAxisScaleFactor, minorAxisScaleValue);
      return { a, b };
    }
    case 8:
      return { a: 6371200.0, b: 6371200.0 };
    case 9: // Airy 1830
      return { a: 6377563.396, b: 6356256.909 };
    default:
      // Fallback to a standard spherical radius if unknown/missing
      return { a: 6378137.0, b: 6356752.245 };
  }
}

/**
 * Calculates Gaussian latitudes in degrees using the Newton-Raphson method
 * following the logic of the Rust 'grib' crate.
 * @param n - number of latitudes
 * @returns array of latitudes
 */
function generateGaussianLats(n: number): number[] {
  const { PI, abs, asin, cos } = Math;
  const MAX_ITER = 10;
  const coeff = 1.0 - 1.0 / (8 * n * n) + 1.0 / (8 * n * n * n);
  const lats: number[] = [];

  for (let i = 0; i < n; i++) {
    // 1. Initial Guess (Tricomi/Lether)
    let x = coeff * cos(((4 * i + 3) * PI) / (4 * n + 2));

    // 2. Newton-Raphson Root Finding
    for (let iter = 0; iter < MAX_ITER; iter++) {
      // legendre_polynomial returns (P_{n-1}, P_n)
      const [pPrev, p] = legendrePolynomial(n, x);
      // derivative: n * (P_{n-1} - x * P_n) / (1 - x^2)
      const fpx = (n * (pPrev - x * p)) / (1.0 - x * x);

      const dx = p / fpx;
      x -= dx;

      if (abs(dx) < Number.EPSILON) break;
    }

    // 3. Convert root to Latitude: asin(x) converted to degrees
    lats.push(asin(x) * (180 / PI));
  }

  return lats;
}

function legendrePolynomial(n: number, x: number): [number, number] {
  let p0 = 1.0;
  let p1 = x;
  for (let k = 2; k <= n; k++) {
    const pk = ((2 * k - 1) * x * p1 - (k - 1) * p0) / k;
    p0 = p1;
    p1 = pk;
  }
  return [p0, p1];
}
