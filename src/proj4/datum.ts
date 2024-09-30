import { adjustLon } from './common';
import {
  DATUMS,
  HALF_PI,
  PJD_3PARAM,
  PJD_7PARAM,
  PJD_GRIDSHIFT,
  PJD_NODATUM,
  PJD_WGS84,
  SEC_TO_RAD,
  SRS_WGS84_ESQUARED,
  SRS_WGS84_SEMIMAJOR,
  SRS_WGS84_SEMIMINOR,
} from './constants';

import type { DatumParams } from 's2-tools/readers/wkt';
import type { NadSubGrid } from 's2-tools/readers/nadgrid';
import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform, Transformer } from '.';

const { abs, sin, cos, sqrt, atan2, atan, PI, floor } = Math;

/**
 * @param params - projection specific parameters to be adjusted
 * @param nadgrids - nad grid data if applicable
 * @param transformer - the projection transformer to potentially pull data from
 */
export function buildDatum(params: ProjectionParams, transformer: Transformer): void {
  if (params.datumCode === undefined || params.datumCode === 'none') {
    params.datumType = PJD_NODATUM;
  } else {
    params.datumType = PJD_WGS84;
  }

  // If datumParams is undefined, check against datum constants using datumCode
  if (params.datumParams === undefined) {
    const datum = DATUMS[params.datumCode?.toLowerCase() ?? ''];
    if (datum !== undefined) {
      // @ts-expect-error - this will be fixed in the next line
      params.datumParams = datum.datumParams;
      params.ellps = datum.ellipse;
    }
  }

  if (params.datumParams !== undefined) {
    if (params.datumParams[0] !== 0 || params.datumParams[1] !== 0 || params.datumParams[2] !== 0) {
      params.datumType = PJD_3PARAM;
    }
    if (params.datumParams.length > 3) {
      if (
        params.datumParams[3] !== 0 ||
        params.datumParams[4] !== 0 ||
        params.datumParams[5] !== 0 ||
        params.datumParams[6] !== 0
      ) {
        params.datumType = PJD_7PARAM;
        params.datumParams[3] *= SEC_TO_RAD;
        params.datumParams[4] *= SEC_TO_RAD;
        params.datumParams[5] *= SEC_TO_RAD;
        params.datumParams[6] = params.datumParams[6] / 1000000.0 + 1.0;
      }
    }
  }

  // Upgrade datumType if grids exists in params and pull in the grids we need
  if (params.nadgrids !== undefined) {
    params.datumType = PJD_GRIDSHIFT;
    params.grids = transformer.getGridsFromString(params.nadgrids);
  }
}

/**
 * @param source
 * @param dest
 */
export function compareDatums(source: ProjectionTransform, dest: ProjectionTransform): boolean {
  if (source.datumType !== dest.datumType) {
    return false; // false, datums are not equal
  } else if (source.a !== dest.a || abs(source.es - dest.es) > 0.00000000005) {
    // the tolerance for es is to ensure that GRS80 and WGS84
    // are considered identical
    return false;
  } else if (source.datumType === PJD_3PARAM) {
    return (
      source.datumParams[0] === dest.datumParams[0] &&
      source.datumParams[1] === dest.datumParams[1] &&
      source.datumParams[2] === dest.datumParams[2]
    );
  } else if (source.datumType === PJD_7PARAM) {
    return (
      source.datumParams[0] === dest.datumParams[0] &&
      source.datumParams[1] === dest.datumParams[1] &&
      source.datumParams[2] === dest.datumParams[2] &&
      source.datumParams[3] === dest.datumParams[3] &&
      source.datumParams[4] === dest.datumParams[4] &&
      source.datumParams[5] === dest.datumParams[5] &&
      source.datumParams[6] === dest.datumParams[6]
    );
  } else {
    return true; // datums are equal
  }
} // cs_compare_datums()

/*
 * The function Convert_Geodetic_To_Geocentric converts geodetic coordinates
 * (latitude, longitude, and height) to geocentric coordinates (X, Y, Z),
 * according to the current ellipsoid parameters.
 *
 *    Latitude  : Geodetic latitude in radians                     (input)
 *    Longitude : Geodetic longitude in radians                    (input)
 *    Height    : Geodetic height, in meters                       (input)
 *    X         : Calculated Geocentric X coordinate, in meters    (output)
 *    Y         : Calculated Geocentric Y coordinate, in meters    (output)
 *    Z         : Calculated Geocentric Z coordinate, in meters    (output)
 *
 */
/**
 * @param p - lon-lat WGS84 point
 * @param es - eccentricity
 * @param a - semi-major axis
 */
export function geodeticToGeocentric(p: VectorPoint, es: number, a: number): void {
  let Longitude = p.x;
  let Latitude = p.y;
  const Height = p.z !== undefined ? p.z : 0; //Z value not always supplied

  /*
   ** Don't blow up if Latitude is just a little out of the value
   ** range as it may just be a rounding issue.  Also removed longitude
   ** test, it should be wrapped by cos() and sin().  NFW for PROJ.4, Sep/2001.
   */
  if (Latitude < -HALF_PI && Latitude > -1.001 * HALF_PI) {
    Latitude = -HALF_PI;
  } else if (Latitude > HALF_PI && Latitude < 1.001 * HALF_PI) {
    Latitude = HALF_PI;
  } else if (Latitude < -HALF_PI) {
    throw new Error('geocent:lat out of range:' + Latitude);
  } else if (Latitude > HALF_PI) {
    throw new Error('geocent:lat out of range:' + Latitude);
  }

  if (Longitude > PI) Longitude -= 2 * PI;
  const Sin_Lat = sin(Latitude); /*  sin(Latitude)  */
  const Cos_Lat = cos(Latitude); /*  cos(Latitude)  */
  const Sin2_Lat = Sin_Lat * Sin_Lat; /*  Square of sin(Latitude)  */
  const Rn = a / sqrt(1.0 - es * Sin2_Lat); /*  Earth radius at location  */

  p.x = (Rn + Height) * Cos_Lat * cos(Longitude);
  p.y = (Rn + Height) * Cos_Lat * sin(Longitude);
  p.z = (Rn * (1 - es) + Height) * Sin_Lat;
}

/**
 * @param p - Geocentric point
 * @param es - ellipsoid eccentricity
 * @param a - ellipsoid semimajor axis
 * @param b - ellipsoid semiminor axis
 */
export function geocentricToGeodetic(p: VectorPoint, es: number, a: number, b: number): void {
  /* local defintions and variables */
  /* end-criterium of loop, accuracy of sin(Latitude) */
  const genau = 1e-12;
  const genau2 = genau * genau;
  const maxiter = 30;

  let RX;
  let RK;
  let RN; /* Earth radius at location */
  let CPHI0; /* cos of start or old geodetic latitude in iterations */
  let SPHI0; /* sin of start or old geodetic latitude in iterations */
  let CPHI; /* cos of searched geodetic latitude */
  let SPHI; /* sin of searched geodetic latitude */
  let SDPHI; /* end-criterium: addition-theorem of sin(Latitude(iter)-Latitude(iter-1)) */
  let iter; /* # of continous iteration, max. 30 is always enough (s.a.) */

  const X = p.x;
  const Y = p.y;
  const Z = p.z !== undefined ? p.z : 0.0; //Z value not always supplied
  let Longitude;
  let Latitude;
  let Height;

  const P = sqrt(X * X + Y * Y); /* distance between semi-minor axis and location */
  const RR = sqrt(X * X + Y * Y + Z * Z); /* distance between center and location */

  /*      special cases for latitude and longitude */
  if (P / a < genau) {
    /*  special case, if P=0. (X=0., Y=0.) */
    Longitude = 0.0;

    /*  if (X,Y,Z)=(0.,0.,0.) then Height becomes semi-minor axis
     *  of ellipsoid (=center of mass), Latitude becomes PI/2 */
    if (RR / a < genau) {
      Latitude = HALF_PI;
      Height = -b;
      return;
    }
  } else {
    /*  ellipsoidal (geodetic) longitude
     *  interval: -PI < Longitude <= +PI */
    Longitude = atan2(Y, X);
  }

  /* --------------------------------------------------------------
   * Following iterative algorithm was developped by
   * "Institut for Erdmessung", University of Hannover, July 1988.
   * Internet: www.ife.uni-hannover.de
   * Iterative computation of CPHI,SPHI and Height.
   * Iteration of CPHI and SPHI to 10**-12 radian resp.
   * 2*10**-7 arcsec.
   * --------------------------------------------------------------
   */
  const CT = Z / RR; /* sin of geocentric latitude */
  const ST = P / RR; /* cos of geocentric latitude */
  RX = 1.0 / sqrt(1.0 - es * (2.0 - es) * ST * ST);
  CPHI0 = ST * (1.0 - es) * RX;
  SPHI0 = CT * RX;
  iter = 0;

  /* loop to find sin(Latitude) resp. Latitude
   * until |sin(Latitude(iter)-Latitude(iter-1))| < genau */
  do {
    iter++;
    RN = a / sqrt(1.0 - es * SPHI0 * SPHI0);

    /*  ellipsoidal (geodetic) height */
    Height = P * CPHI0 + Z * SPHI0 - RN * (1.0 - es * SPHI0 * SPHI0);

    RK = (es * RN) / (RN + Height);
    RX = 1.0 / sqrt(1.0 - RK * (2.0 - RK) * ST * ST);
    CPHI = ST * (1.0 - RK) * RX;
    SPHI = CT * RX;
    SDPHI = SPHI * CPHI0 - CPHI * SPHI0;
    CPHI0 = CPHI;
    SPHI0 = SPHI;
  } while (SDPHI * SDPHI > genau2 && iter < maxiter);

  /*      ellipsoidal (geodetic) latitude */
  Latitude = atan(SPHI / abs(CPHI));

  p.x = Longitude;
  p.y = Latitude;
  p.z = Height;
}

/**
 * pj_geocentic_to_wgs84( p )
 * p = point to transform in geocentric coordinates (x,y,z)
 * point object, nothing fancy, just allows values to be
 * passed back and forth by reference rather than by value.
 * Other point classes may be used as long as they have
 * x and y properties, which will get modified in the transform method.
 * @param p - Geocentric point
 * @param datumType - datum type
 * @param datumParams - datum parameters
 */
export function geocentricToWgs84(
  p: VectorPoint,
  datumType: number,
  datumParams: DatumParams,
): void {
  const z = p.z ?? 0;
  if (datumType === PJD_3PARAM) {
    // if( x[io] === HUGE_VAL )
    //    continue;
    p.x += datumParams[0];
    p.y += datumParams[1];
    p.z = z + datumParams[2];
  } else if (datumType === PJD_7PARAM) {
    const Dx_BF = datumParams[0];
    const Dy_BF = datumParams[1];
    const Dz_BF = datumParams[2];
    const Rx_BF = datumParams[3];
    const Ry_BF = datumParams[4];
    const Rz_BF = datumParams[5];
    const M_BF = datumParams[6];
    // if( x[io] === HUGE_VAL )
    //    continue;
    p.x = M_BF * (p.x - Rz_BF * p.y + Ry_BF * z) + Dx_BF;
    p.y = M_BF * (Rz_BF * p.x + p.y - Rx_BF * z) + Dy_BF;
    p.z = M_BF * (-Ry_BF * p.x + Rx_BF * p.y + z) + Dz_BF;
  } else {
    throw new Error(`geocentricToWgs84: unknown datum type: ${datumType}`);
  }
}

/**
 * pj_geocentic_from_wgs84() coordinate system definition,
 * point to transform in geocentric coordinates (x,y,z)
 * @param p - lon-lat WGS84 point
 * @param datumType - datum type
 * @param datumParams - datum parameters
 */
export function geocentricFromWgs84(
  p: VectorPoint,
  datumType: number,
  datumParams: DatumParams,
): void {
  const z = p.z ?? 0;
  if (datumType === PJD_3PARAM) {
    //if( x[io] === HUGE_VAL )
    //    continue;
    p.x -= datumParams[0];
    p.y -= datumParams[1];
    p.z = z - datumParams[2];
  } else if (datumType === PJD_7PARAM) {
    const Dx_BF = datumParams[0];
    const Dy_BF = datumParams[1];
    const Dz_BF = datumParams[2];
    const Rx_BF = datumParams[3];
    const Ry_BF = datumParams[4];
    const Rz_BF = datumParams[5];
    const M_BF = datumParams[6];
    const x_tmp = (p.x - Dx_BF) / M_BF;
    const y_tmp = (p.y - Dy_BF) / M_BF;
    const z_tmp = (z - Dz_BF) / M_BF;
    //if( x[io] === HUGE_VAL )
    //    continue;
    p.x = x_tmp + Rz_BF * y_tmp - Ry_BF * z_tmp;
    p.y = -Rz_BF * x_tmp + y_tmp + Rx_BF * z_tmp;
    p.z = Ry_BF * x_tmp - Rx_BF * y_tmp + z_tmp;
  } else {
    throw new Error(`geocentricToWgs84: unknown datum type: ${datumType}`);
  }
}

/**
 * @param type - datum type
 * @returns - true if 1 or 2
 */
function checkParams(type: number): boolean {
  return type === PJD_3PARAM || type === PJD_7PARAM;
}

/**
 * @param source
 * @param dest
 */
export function checkNotWGS(source: ProjectionTransform, dest: ProjectionTransform): boolean {
  return (
    ((source.datumType === PJD_3PARAM ||
      source.datumType === PJD_7PARAM ||
      source.datumType === PJD_GRIDSHIFT) &&
      dest.datumCode !== 'WGS84') ||
    ((dest.datumType === PJD_3PARAM ||
      dest.datumType === PJD_7PARAM ||
      dest.datumType === PJD_GRIDSHIFT) &&
      source.datumCode !== 'WGS84')
  );
}

/**
 * @param point - lon-lat WGS84 point to mutate
 * @param source - source projection
 * @param dest - destination projection
 */
export function datumTransform(
  point: VectorPoint,
  source: ProjectionTransform,
  dest: ProjectionTransform,
): void {
  // Short cut if the datums are identical.
  if (compareDatums(source, dest)) return;
  // Explicitly skip datum transform by setting 'datum=none' as parameter for either source or dest
  if (source.datumType === PJD_NODATUM || dest.datumType === PJD_NODATUM) return;

  // If this datum requires grid shifts, then apply it to geodetic coordinates.
  let sourceA = source.a;
  let sourceEs = source.es;
  if (source.datumType === PJD_GRIDSHIFT) {
    // source
    applyGridShift(source, false, point);
    sourceA = SRS_WGS84_SEMIMAJOR;
    sourceEs = SRS_WGS84_ESQUARED;
  }

  let dest_a = dest.a;
  let dest_b = dest.b;
  let dest_es = dest.es;
  if (dest.datumType === PJD_GRIDSHIFT) {
    dest_a = SRS_WGS84_SEMIMAJOR;
    dest_b = SRS_WGS84_SEMIMINOR;
    dest_es = SRS_WGS84_ESQUARED;
  }

  // Do we need to go through geocentric coordinates?
  if (
    sourceEs === dest_es &&
    sourceA === dest_a &&
    !checkParams(source.datumType) &&
    !checkParams(dest.datumType)
  )
    return;

  // Convert to geocentric coordinates.
  geodeticToGeocentric(point, sourceEs, sourceA);
  // Convert between datums
  if (checkParams(source.datumType)) geocentricToWgs84(point, source.datumType, source.datumParams);
  if (checkParams(dest.datumType)) geocentricFromWgs84(point, dest.datumType, dest.datumParams);
  // Convert back to geodetic coordinates.
  geocentricToGeodetic(point, dest_es, dest_a, dest_b);

  if (dest.datumType === PJD_GRIDSHIFT) applyGridShift(dest, true, point);
}

/**
 * @param source
 * @param inverse
 * @param point
 */
export function applyGridShift(
  source: ProjectionTransform,
  inverse: boolean,
  point: VectorPoint,
): void {
  const { grids } = source;
  if (grids === undefined) throw new Error('Grid shift grids not found');
  const input = { x: -point.x, y: point.y };
  let output = { x: Number.NaN, y: Number.NaN };
  const attemptedGrids = [];
  outer: for (const grid of grids) {
    attemptedGrids.push(grid.name);
    if (grid.isNull) {
      output = input;
      break;
    }
    if (grid.grid === undefined) {
      if (grid.mandatory) {
        console.warn(
          `Unable to find mandatory grid '${grid.name}'. Maybe have an incorrect result.`,
        );
      }
      continue;
    }
    const subgrids = grid.grid.subgrids;
    for (let j = 0, jj = subgrids.length; j < jj; j++) {
      const subgrid = subgrids[j];
      // skip tables that don't match our point at all
      const epsilon = (abs(subgrid.del.y) + abs(subgrid.del.x)) / 10000.0;
      const minX = subgrid.ll.x - epsilon;
      const minY = subgrid.ll.y - epsilon;
      const maxX = subgrid.ll.x + (subgrid.lim.x - 1) * subgrid.del.x + epsilon;
      const maxY = subgrid.ll.y + (subgrid.lim.y - 1) * subgrid.del.y + epsilon;
      if (minY > input.y || minX > input.x || maxY < input.y || maxX < input.x) {
        continue;
      }
      output = applySubgridShift(input, inverse, subgrid);
      if (!isNaN(output.x)) {
        break outer;
      }
    }
  }
  if (isNaN(output.x)) {
    return;
    // throw new Error(
    //   `Failed to find a grid shift table for location '${-input.x * R2D} ${input.y * R2D}' tried: '${attemptedGrids}'`,
    // );
  }
  point.x = -output.x;
  point.y = output.y;
}

/**
 * @param pin
 * @param inverse
 * @param ct
 */
function applySubgridShift(pin: VectorPoint, inverse: boolean, ct: NadSubGrid): VectorPoint {
  const val = { x: Number.NaN, y: Number.NaN };
  const tb = { x: pin.x, y: pin.y };
  tb.x -= ct.ll.x;
  tb.y -= ct.ll.y;
  tb.x = adjustLon(tb.x - PI) + PI;
  const t = nadInterpolate(tb, ct);
  if (inverse) {
    if (isNaN(t.x)) return val;
    t.x = tb.x - t.x;
    t.y = tb.y - t.y;
    let i = 9;
    const tol = 1e-12;
    let dif, del;
    do {
      del = nadInterpolate(t, ct);
      if (isNaN(del.x)) {
        throw new Error(
          'Inverse grid shift iteration failed, presumably at grid edge.  Using first approximation.',
        );
      }
      dif = { x: tb.x - (del.x + t.x), y: tb.y - (del.y + t.y) };
      t.x += dif.x;
      t.y += dif.y;
    } while (i-- > 0 && abs(dif.x) > tol && abs(dif.y) > tol);
    if (i < 0) throw new Error('Inverse grid shift iterator failed to converge.');
    val.x = adjustLon(t.x + ct.ll.x);
    val.y = t.y + ct.ll.y;
  } else {
    if (!isNaN(t.x)) {
      val.x = pin.x + t.x;
      val.y = pin.y + t.y;
    }
  }
  return val;
}

/**
 * @param pin
 * @param ct
 */
function nadInterpolate(pin: VectorPoint, ct: NadSubGrid): VectorPoint {
  const t = { x: pin.x / ct.del.x, y: pin.y / ct.del.y };
  const indx: VectorPoint = { x: floor(t.x), y: floor(t.y) };
  const frct = { x: t.x - indx.x, y: t.y - indx.y };
  const val = { x: NaN, y: NaN };
  let inx;
  if (indx.x < 0 || indx.x >= ct.lim.x) return val;
  if (indx.y < 0 || indx.y >= ct.lim.y) return val;
  inx = indx.y * ct.lim.x + indx.x;
  const f00 = { x: ct.cvs[inx].x, y: ct.cvs[inx].y };
  inx++;
  const f10 = { x: ct.cvs[inx].x, y: ct.cvs[inx].y };
  inx += ct.lim.x;
  const f11 = { x: ct.cvs[inx].x, y: ct.cvs[inx].y };
  inx--;
  const f01 = { x: ct.cvs[inx].x, y: ct.cvs[inx].y };
  const m11 = frct.x * frct.y,
    m10 = frct.x * (1.0 - frct.y),
    m00 = (1.0 - frct.x) * (1.0 - frct.y),
    m01 = (1.0 - frct.x) * frct.y;
  val.x = m00 * f00.x + m10 * f10.x + m01 * f01.x + m11 * f11.x;
  val.y = m00 * f00.y + m10 * f10.y + m01 * f01.y + m11 * f11.y;

  return val;
}
