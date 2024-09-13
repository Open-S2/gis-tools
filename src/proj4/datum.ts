import { adjustLon } from './common';
import {
  HALF_PI,
  PJD_3PARAM,
  PJD_7PARAM,
  PJD_GRIDSHIFT,
  PJD_NODATUM,
  PJD_WGS84,
  R2D,
  SEC_TO_RAD,
  SRS_WGS84_ESQUARED,
  SRS_WGS84_SEMIMAJOR,
  SRS_WGS84_SEMIMINOR,
} from './constants';

import type { VectorPoint } from 's2-tools/geometry';

/**
 * @param datumCode
 * @param datum_params
 * @param a
 * @param b
 * @param es
 * @param ep2
 * @param nadgrids
 */
export function buildDatum(datumCode, datum_params, a, b, es, ep2, nadgrids) {
  const out = {};

  if (datumCode === undefined || datumCode === 'none') {
    out.datum_type = PJD_NODATUM;
  } else {
    out.datum_type = PJD_WGS84;
  }

  if (datum_params) {
    out.datum_params = datum_params.map(parseFloat);
    if (out.datum_params[0] !== 0 || out.datum_params[1] !== 0 || out.datum_params[2] !== 0) {
      out.datum_type = PJD_3PARAM;
    }
    if (out.datum_params.length > 3) {
      if (
        out.datum_params[3] !== 0 ||
        out.datum_params[4] !== 0 ||
        out.datum_params[5] !== 0 ||
        out.datum_params[6] !== 0
      ) {
        out.datum_type = PJD_7PARAM;
        out.datum_params[3] *= SEC_TO_RAD;
        out.datum_params[4] *= SEC_TO_RAD;
        out.datum_params[5] *= SEC_TO_RAD;
        out.datum_params[6] = out.datum_params[6] / 1000000.0 + 1.0;
      }
    }
  }

  if (nadgrids) {
    out.datum_type = PJD_GRIDSHIFT;
    out.grids = nadgrids;
  }
  out.a = a; //datum object also uses these values
  out.b = b;
  out.es = es;
  out.ep2 = ep2;

  return out;
}

/**
 * @param source
 * @param dest
 */
export function compareDatums(source, dest) {
  if (source.datum_type !== dest.datum_type) {
    return false; // false, datums are not equal
  } else if (source.a !== dest.a || Math.abs(source.es - dest.es) > 0.00000000005) {
    // the tolerance for es is to ensure that GRS80 and WGS84
    // are considered identical
    return false;
  } else if (source.datum_type === PJD_3PARAM) {
    return (
      source.datum_params[0] === dest.datum_params[0] &&
      source.datum_params[1] === dest.datum_params[1] &&
      source.datum_params[2] === dest.datum_params[2]
    );
  } else if (source.datum_type === PJD_7PARAM) {
    return (
      source.datum_params[0] === dest.datum_params[0] &&
      source.datum_params[1] === dest.datum_params[1] &&
      source.datum_params[2] === dest.datum_params[2] &&
      source.datum_params[3] === dest.datum_params[3] &&
      source.datum_params[4] === dest.datum_params[4] &&
      source.datum_params[5] === dest.datum_params[5] &&
      source.datum_params[6] === dest.datum_params[6]
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
 * @param p
 * @param es
 * @param a
 */
export function geodeticToGeocentric(p, es, a) {
  let Longitude = p.x;
  let Latitude = p.y;
  const Height = p.z ? p.z : 0; //Z value not always supplied

  let Rn; /*  Earth radius at location  */
  let Sin_Lat; /*  Math.sin(Latitude)  */
  let Sin2_Lat; /*  Square of Math.sin(Latitude)  */
  let Cos_Lat; /*  Math.cos(Latitude)  */

  /*
   ** Don't blow up if Latitude is just a little out of the value
   ** range as it may just be a rounding issue.  Also removed longitude
   ** test, it should be wrapped by Math.cos() and Math.sin().  NFW for PROJ.4, Sep/2001.
   */
  if (Latitude < -HALF_PI && Latitude > -1.001 * HALF_PI) {
    Latitude = -HALF_PI;
  } else if (Latitude > HALF_PI && Latitude < 1.001 * HALF_PI) {
    Latitude = HALF_PI;
  } else if (Latitude < -HALF_PI) {
    /* Latitude out of range */
    //..reportError('geocent:lat out of range:' + Latitude);
    return { x: -Infinity, y: -Infinity, z: p.z };
  } else if (Latitude > HALF_PI) {
    /* Latitude out of range */
    return { x: Infinity, y: Infinity, z: p.z };
  }

  if (Longitude > Math.PI) {
    Longitude -= 2 * Math.PI;
  }
  Sin_Lat = Math.sin(Latitude);
  Cos_Lat = Math.cos(Latitude);
  Sin2_Lat = Sin_Lat * Sin_Lat;
  Rn = a / Math.sqrt(1.0 - es * Sin2_Lat);
  return {
    x: (Rn + Height) * Cos_Lat * Math.cos(Longitude),
    y: (Rn + Height) * Cos_Lat * Math.sin(Longitude),
    z: (Rn * (1 - es) + Height) * Sin_Lat,
  };
} // cs_geodetic_to_geocentric()

/**
 * @param p
 * @param es
 * @param a
 * @param b
 */
export function geocentricToGeodetic(p, es, a, b) {
  /* local defintions and variables */
  /* end-criterium of loop, accuracy of sin(Latitude) */
  const genau = 1e-12;
  const genau2 = genau * genau;
  const maxiter = 30;

  let P; /* distance between semi-minor axis and location */
  let RR; /* distance between center and location */
  let CT; /* sin of geocentric latitude */
  let ST; /* cos of geocentric latitude */
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
  const Z = p.z ? p.z : 0.0; //Z value not always supplied
  let Longitude;
  let Latitude;
  let Height;

  P = Math.sqrt(X * X + Y * Y);
  RR = Math.sqrt(X * X + Y * Y + Z * Z);

  /*      special cases for latitude and longitude */
  if (P / a < genau) {
    /*  special case, if P=0. (X=0., Y=0.) */
    Longitude = 0.0;

    /*  if (X,Y,Z)=(0.,0.,0.) then Height becomes semi-minor axis
     *  of ellipsoid (=center of mass), Latitude becomes PI/2 */
    if (RR / a < genau) {
      Latitude = HALF_PI;
      Height = -b;
      return {
        x: p.x,
        y: p.y,
        z: p.z,
      };
    }
  } else {
    /*  ellipsoidal (geodetic) longitude
     *  interval: -PI < Longitude <= +PI */
    Longitude = Math.atan2(Y, X);
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
  CT = Z / RR;
  ST = P / RR;
  RX = 1.0 / Math.sqrt(1.0 - es * (2.0 - es) * ST * ST);
  CPHI0 = ST * (1.0 - es) * RX;
  SPHI0 = CT * RX;
  iter = 0;

  /* loop to find sin(Latitude) resp. Latitude
   * until |sin(Latitude(iter)-Latitude(iter-1))| < genau */
  do {
    iter++;
    RN = a / Math.sqrt(1.0 - es * SPHI0 * SPHI0);

    /*  ellipsoidal (geodetic) height */
    Height = P * CPHI0 + Z * SPHI0 - RN * (1.0 - es * SPHI0 * SPHI0);

    RK = (es * RN) / (RN + Height);
    RX = 1.0 / Math.sqrt(1.0 - RK * (2.0 - RK) * ST * ST);
    CPHI = ST * (1.0 - RK) * RX;
    SPHI = CT * RX;
    SDPHI = SPHI * CPHI0 - CPHI * SPHI0;
    CPHI0 = CPHI;
    SPHI0 = SPHI;
  } while (SDPHI * SDPHI > genau2 && iter < maxiter);

  /*      ellipsoidal (geodetic) latitude */
  Latitude = Math.atan(SPHI / Math.abs(CPHI));
  return {
    x: Longitude,
    y: Latitude,
    z: Height,
  };
} // cs_geocentric_to_geodetic()

/****************************************************************/
// pj_geocentic_to_wgs84( p )
//  p = point to transform in geocentric coordinates (x,y,z)

/**
     point object, nothing fancy, just allows values to be
    passed back and forth by reference rather than by value.
    Other point classes may be used as long as they have
    x and y properties, which will get modified in the transform method.
 * @param p
 * @param datum_type
 * @param datum_params
 */
export function geocentricToWgs84(p, datum_type, datum_params) {
  if (datum_type === PJD_3PARAM) {
    // if( x[io] === HUGE_VAL )
    //    continue;
    return {
      x: p.x + datum_params[0],
      y: p.y + datum_params[1],
      z: p.z + datum_params[2],
    };
  } else if (datum_type === PJD_7PARAM) {
    const Dx_BF = datum_params[0];
    const Dy_BF = datum_params[1];
    const Dz_BF = datum_params[2];
    const Rx_BF = datum_params[3];
    const Ry_BF = datum_params[4];
    const Rz_BF = datum_params[5];
    const M_BF = datum_params[6];
    // if( x[io] === HUGE_VAL )
    //    continue;
    return {
      x: M_BF * (p.x - Rz_BF * p.y + Ry_BF * p.z) + Dx_BF,
      y: M_BF * (Rz_BF * p.x + p.y - Rx_BF * p.z) + Dy_BF,
      z: M_BF * (-Ry_BF * p.x + Rx_BF * p.y + p.z) + Dz_BF,
    };
  }
} // cs_geocentric_to_wgs84

/****************************************************************/
// pj_geocentic_from_wgs84()
//  coordinate system definition,
//  point to transform in geocentric coordinates (x,y,z)
/**
 * @param p
 * @param datum_type
 * @param datum_params
 */
export function geocentricFromWgs84(p, datum_type, datum_params) {
  if (datum_type === PJD_3PARAM) {
    //if( x[io] === HUGE_VAL )
    //    continue;
    return {
      x: p.x - datum_params[0],
      y: p.y - datum_params[1],
      z: p.z - datum_params[2],
    };
  } else if (datum_type === PJD_7PARAM) {
    const Dx_BF = datum_params[0];
    const Dy_BF = datum_params[1];
    const Dz_BF = datum_params[2];
    const Rx_BF = datum_params[3];
    const Ry_BF = datum_params[4];
    const Rz_BF = datum_params[5];
    const M_BF = datum_params[6];
    const x_tmp = (p.x - Dx_BF) / M_BF;
    const y_tmp = (p.y - Dy_BF) / M_BF;
    const z_tmp = (p.z - Dz_BF) / M_BF;
    //if( x[io] === HUGE_VAL )
    //    continue;

    return {
      x: x_tmp + Rz_BF * y_tmp - Ry_BF * z_tmp,
      y: -Rz_BF * x_tmp + y_tmp + Rx_BF * z_tmp,
      z: Ry_BF * x_tmp - Rx_BF * y_tmp + z_tmp,
    };
  } //cs_geocentric_from_wgs84()
}

/**
 * @param type
 */
function checkParams(type) {
  return type === PJD_3PARAM || type === PJD_7PARAM;
}

/**
 * @param source
 * @param dest
 * @param point
 */
export default function (source, dest, point) {
  // Short cut if the datums are identical.
  if (compareDatums(source, dest)) {
    return point; // in this case, zero is sucess,
    // whereas cs_compare_datums returns 1 to indicate TRUE
    // confusing, should fix this
  }

  // Explicitly skip datum transform by setting 'datum=none' as parameter for either source or dest
  if (source.datum_type === PJD_NODATUM || dest.datum_type === PJD_NODATUM) {
    return point;
  }

  // If this datum requires grid shifts, then apply it to geodetic coordinates.
  let source_a = source.a;
  let source_es = source.es;
  if (source.datum_type === PJD_GRIDSHIFT) {
    const gridShiftCode = applyGridShift(source, false, point);
    if (gridShiftCode !== 0) {
      return undefined;
    }
    source_a = SRS_WGS84_SEMIMAJOR;
    source_es = SRS_WGS84_ESQUARED;
  }

  let dest_a = dest.a;
  let dest_b = dest.b;
  let dest_es = dest.es;
  if (dest.datum_type === PJD_GRIDSHIFT) {
    dest_a = SRS_WGS84_SEMIMAJOR;
    dest_b = SRS_WGS84_SEMIMINOR;
    dest_es = SRS_WGS84_ESQUARED;
  }

  // Do we need to go through geocentric coordinates?
  if (
    source_es === dest_es &&
    source_a === dest_a &&
    !checkParams(source.datum_type) &&
    !checkParams(dest.datum_type)
  ) {
    return point;
  }

  // Convert to geocentric coordinates.
  point = geodeticToGeocentric(point, source_es, source_a);
  // Convert between datums
  if (checkParams(source.datum_type)) {
    point = geocentricToWgs84(point, source.datum_type, source.datum_params);
  }
  if (checkParams(dest.datum_type)) {
    point = geocentricFromWgs84(point, dest.datum_type, dest.datum_params);
  }
  point = geocentricToGeodetic(point, dest_es, dest_a, dest_b);

  if (dest.datum_type === PJD_GRIDSHIFT) {
    const destGridShiftResult = applyGridShift(dest, true, point);
    if (destGridShiftResult !== 0) {
      return undefined;
    }
  }

  return point;
}

/**
 * @param source
 * @param inverse
 * @param point
 */
export function applyGridShift(source, inverse, point) {
  if (source.grids === null || source.grids.length === 0) {
    throw new Error('Grid shift grids not found');
  }
  const input = { x: -point.x, y: point.y };
  let output = { x: Number.NaN, y: Number.NaN };
  let onlyMandatoryGrids = false;
  const attemptedGrids = [];
  outer: for (let i = 0; i < source.grids.length; i++) {
    const grid = source.grids[i];
    attemptedGrids.push(grid.name);
    if (grid.isNull) {
      output = input;
      break;
    }
    onlyMandatoryGrids = grid.mandatory;
    if (grid.grid === null) {
      if (grid.mandatory) {
        throw new Error(`Unable to find mandatory grid '${grid.name}'`);
      }
      continue;
    }
    const subgrids = grid.grid.subgrids;
    for (let j = 0, jj = subgrids.length; j < jj; j++) {
      const subgrid = subgrids[j];
      // skip tables that don't match our point at all
      const epsilon = (Math.abs(subgrid.del[1]) + Math.abs(subgrid.del[0])) / 10000.0;
      const minX = subgrid.ll[0] - epsilon;
      const minY = subgrid.ll[1] - epsilon;
      const maxX = subgrid.ll[0] + (subgrid.lim[0] - 1) * subgrid.del[0] + epsilon;
      const maxY = subgrid.ll[1] + (subgrid.lim[1] - 1) * subgrid.del[1] + epsilon;
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
    throw new Error(
      `Failed to find a grid shift table for location '${-input.x * R2D} ${input.y * R2D}' tried: '${attemptedGrids}'`,
    );
  }
  point.x = -output.x;
  point.y = output.y;
  return 0;
}

/**
 * @param pin
 * @param inverse
 * @param ct
 */
function applySubgridShift(pin, inverse, ct) {
  const val = { x: Number.NaN, y: Number.NaN };
  if (isNaN(pin.x)) {
    return val;
  }
  const tb = { x: pin.x, y: pin.y };
  tb.x -= ct.ll[0];
  tb.y -= ct.ll[1];
  tb.x = adjustLon(tb.x - Math.PI) + Math.PI;
  const t = nadInterpolate(tb, ct);
  if (inverse) {
    if (isNaN(t.x)) {
      return val;
    }
    t.x = tb.x - t.x;
    t.y = tb.y - t.y;
    let i = 9,
      tol = 1e-12;
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
    } while (i-- && Math.abs(dif.x) > tol && Math.abs(dif.y) > tol);
    if (i < 0) {
      throw new Error('Inverse grid shift iterator failed to converge.');
    }
    val.x = adjustLon(t.x + ct.ll[0]);
    val.y = t.y + ct.ll[1];
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
function nadInterpolate(pin: VectorPoint, ct): VectorPoint {
  const t = { x: pin.x / ct.del[0], y: pin.y / ct.del[1] };
  const indx = { x: Math.floor(t.x), y: Math.floor(t.y) };
  const frct = { x: t.x - 1.0 * indx.x, y: t.y - 1.0 * indx.y };
  const val = { x: NaN, y: NaN };
  let inx;
  if (indx.x < 0 || indx.x >= ct.lim[0]) return val;
  if (indx.y < 0 || indx.y >= ct.lim[1]) return val;
  inx = indx.y * ct.lim[0] + indx.x;
  const f00 = { x: ct.cvs[inx][0], y: ct.cvs[inx][1] };
  inx++;
  const f10 = { x: ct.cvs[inx][0], y: ct.cvs[inx][1] };
  inx += ct.lim[0];
  const f11 = { x: ct.cvs[inx][0], y: ct.cvs[inx][1] };
  inx--;
  const f01 = { x: ct.cvs[inx][0], y: ct.cvs[inx][1] };
  const m11 = frct.x * frct.y,
    m10 = frct.x * (1.0 - frct.y),
    m00 = (1.0 - frct.x) * (1.0 - frct.y),
    m01 = (1.0 - frct.x) * frct.y;
  val.x = m00 * f00.x + m10 * f10.x + m01 * f01.x + m11 * f11.x;
  val.y = m00 * f00.y + m10 * f10.y + m01 * f01.y + m11 * f11.y;

  return val;
}
