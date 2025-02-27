/* eslint-disable no-loss-of-precision */
import { EPSLN, HALF_PI, SPI, TWO_PI } from './constants';

/**
 * Returns the arc hyperbolic cosine of x.
 * @param x - input
 * @returns - acosh(x)
 */
export function acosh(x: number): number {
  return 2 * Math.log(Math.sqrt((x + 1) / 2) + Math.sqrt((x - 1) / 2));
}

/**
 * Returns an adjusted latitude
 * @param x - input
 * @returns - the adjusted latitude
 */
export function adjustLat(x: number): number {
  return Math.abs(x) < HALF_PI ? x : x - sign(x) * Math.PI;
}

/**
 * Returns an adjusted longitude
 * @param x - input
 * @returns - the adjusted longitude
 */
export function adjustLon(x: number): number {
  return Math.abs(x) <= SPI ? x : x - sign(x) * TWO_PI;
}

/**
 * Returns an adjusted zone relative to the input zone and longitude
 * @param zone - the input zone
 * @param lon - the input longitude
 * @returns - the adjusted zone
 */
export function adjustZone(zone: number | undefined, lon: number): number {
  zone = zone ?? Math.floor(((adjustLon(lon) + Math.PI) * 30) / Math.PI) + 1;

  if (zone < 0) {
    return 0;
  } else if (zone > 60) {
    return 60;
  } else {
    return zone;
  }
}

/**
 * Returns the arc hyperbolic sine of x.
 * @param x - input
 * @returns - asinh(x)
 */
export function asinh(x: number): number {
  const s = x >= 0 ? 1 : -1;
  return s * Math.log(Math.abs(x) + Math.sqrt(x * x + 1));
}

/**
 * Returns the arc hyperbolic tangent of x.
 * @param x - input
 * @returns - asinhy(x)
 */
export function asinhy(x: number): number {
  let y = Math.abs(x);
  y = log1py(y * (1 + y / (hypot(1, y) + 1)));

  return x < 0 ? -y : y;
}

/**
 * Returns the absolute value of the arc sine of x.
 * @param x - input
 * @returns - asinz(x)
 */
export function asinz(x: number): number {
  if (Math.abs(x) > 1) x = x > 1 ? 1 : -1;
  return Math.asin(x);
}

/**
 * Returns the complex form of coefficients
 * @param pp - array of coefficients
 * @param arg_r - input
 * @param arg_i - input
 * @returns the complex result as a 2D array
 */
export function clensCmplx(pp: number[], arg_r: number, arg_i: number): [number, number] {
  const sin_arg_r = Math.sin(arg_r);
  const cos_arg_r = Math.cos(arg_r);
  const sinh_arg_i = sinh(arg_i);
  const cosh_arg_i = cosh(arg_i);
  let r = 2 * cos_arg_r * cosh_arg_i;
  let i = -2 * sin_arg_r * sinh_arg_i;
  let j = pp.length - 1;
  let hr = pp[j];
  let hi1 = 0;
  let hr1 = 0;
  let hi = 0;
  let hr2;
  let hi2;

  while (--j >= 0) {
    hr2 = hr1;
    hi2 = hi1;
    hr1 = hr;
    hi1 = hi;
    hr = -hr2 + r * hr1 - i * hi1 + pp[j];
    hi = -hi2 + i * hr1 + r * hi1;
  }

  r = sin_arg_r * cosh_arg_i;
  i = cos_arg_r * sinh_arg_i;

  return [r * hr - i * hi, r * hi + i * hr];
}

/**
 * Returns the complex form of coefficients
 * @param pp - array of coefficients
 * @param arg_r - input
 * @returns the resultant compex number
 */
export function clens(pp: number[], arg_r: number): number {
  const r = 2 * Math.cos(arg_r);
  let i = pp.length - 1;
  let hr1 = pp[i];
  let hr2 = 0;
  let hr = 0;

  while (--i >= 0) {
    hr = -hr2 + r * hr1 + pp[i];
    hr2 = hr1;
    hr1 = hr;
  }

  return Math.sin(arg_r) * hr;
}

/**
 * Returns the hyperbolic cosine of x.
 * @param x - input
 * @returns - cosh(x)
 */
export function cosh(x: number): number {
  let r = Math.exp(x);
  r = (r + 1 / r) / 2;
  return r;
}

/**
 * Returns eOfn(x)
 * @param x - input
 * @returns - eOfn(x)
 */
export function e0fn(x: number): number {
  return 1 - 0.25 * x * (1 + (x / 16) * (3 + 1.25 * x));
}

/**
 * Returns e1fn(x)
 * @param x - input
 * @returns - e1fn(x)
 */
export function e1fn(x: number): number {
  return 0.375 * x * (1 + 0.25 * x * (1 + 0.46875 * x));
}

/**
 * Returns e2fn(x)
 * @param x - input
 * @returns - e2fn(x)
 */
export function e2fn(x: number): number {
  return 0.05859375 * x * x * (1 + 0.75 * x);
}

/**
 * Returns e3fn(x)
 * @param x - input
 * @returns - e3fn(x)
 */
export function e3fn(x: number): number {
  return x * x * x * (35 / 3072);
}

/**
 * Convenience function to compute fL(x, L)
 * @param x - input
 * @param L - exponent
 * @returns - fL(x, L)
 */
export function fL(x: number, L: number): number {
  return 2 * Math.atan(x * Math.exp(L)) - HALF_PI;
}

/**
 * Convenience function to compute gatg(pp, B)
 * @param pp - array of coefficients
 * @param B - input
 * @returns - gatg(pp, B)
 */
export function gatg(pp: number[], B: number): number {
  const cos_2B = 2 * Math.cos(2 * B);
  let i = pp.length - 1;
  let h1 = pp[i];
  let h2 = 0;
  let h = 0;

  while (--i >= 0) {
    h = -h2 + cos_2B * h1 + pp[i];
    h2 = h1;
    h1 = h;
  }

  return B + h * Math.sin(2 * B);
}

/**
 * Returns gN(a, e, sinphi)
 * @param a - input
 * @param e - input
 * @param sinphi - sin of latitude
 * @returns - gN(a, e, sinphi)
 */
export function gN(a: number, e: number, sinphi: number): number {
  const temp = e * sinphi;
  return a / Math.sqrt(1 - temp * temp);
}

/**
 * Returns the hypotenuse of x and y
 * @param x - input
 * @param y - input
 * @returns - hypot(x, y)
 */
export function hypot(x: number, y: number): number {
  x = Math.abs(x);
  y = Math.abs(y);
  const a = Math.max(x, y);
  const b = Math.min(x, y) / (a !== 0 ? a : 1);

  return a * Math.sqrt(1 + Math.pow(b, 2));
}

/**
 * Convenience function to compute iMLfn(ml, e0, e1, e2, e3)
 * @param ml - input
 * @param e0 - input
 * @param e1 - input
 * @param e2 - input
 * @param e3 - input
 * @returns - iMLfn(ml, e0, e1, e2, e3)
 */
export function imlfn(ml: number, e0: number, e1: number, e2: number, e3: number): number {
  let phi;
  let dphi;

  phi = ml / e0;
  for (let i = 0; i < 15; i++) {
    dphi =
      (ml - (e0 * phi - e1 * Math.sin(2 * phi) + e2 * Math.sin(4 * phi) - e3 * Math.sin(6 * phi))) /
      (e0 - 2 * e1 * Math.cos(2 * phi) + 4 * e2 * Math.cos(4 * phi) - 6 * e3 * Math.cos(6 * phi));
    phi += dphi;
    if (Math.abs(dphi) <= 0.0000000001) {
      return phi;
    }
  }

  throw new Error('IMLFN-CONV:Latitude failed to converge after 15 iterations');
}

/**
 * Inverse of iLfn
 * @param eccent - eccentricity
 * @param ts - input
 * @returns - invlatiso(eccent, ts)
 */
export function invlatiso(eccent: number, ts: number): number {
  let phi = fL(1, ts);
  let Iphi = 0;
  let con = 0;
  do {
    Iphi = phi;
    con = eccent * Math.sin(Iphi);
    phi = fL(Math.exp((eccent * Math.log((1 + con) / (1 - con))) / 2), ts);
  } while (Math.abs(phi - Iphi) > 1.0e-12);
  return phi;
}

/**
 * Convienience function to compute iqsfnz(eccent, q)
 * @param eccent - eccentricity
 * @param q - input
 * @returns - iqsfnz(eccent, q)
 */
export function iqsfnz(eccent: number, q: number): number {
  const temp = 1 - ((1 - eccent * eccent) / (2 * eccent)) * Math.log((1 - eccent) / (1 + eccent));
  if (Math.abs(Math.abs(q) - temp) < 1.0e-6) {
    if (q < 0) {
      return -1 * HALF_PI;
    } else {
      return HALF_PI;
    }
  }
  //var phi = 0.5* q/(1-eccent*eccent);
  let phi = Math.asin(0.5 * q);
  let dphi;
  let sin_phi;
  let cos_phi;
  let con;
  for (let i = 0; i < 30; i++) {
    sin_phi = Math.sin(phi);
    cos_phi = Math.cos(phi);
    con = eccent * sin_phi;
    dphi =
      (Math.pow(1 - con * con, 2) / (2 * cos_phi)) *
      (q / (1 - eccent * eccent) -
        sin_phi / (1 - con * con) +
        (0.5 / eccent) * Math.log((1 - con) / (1 + con)));
    phi += dphi;
    if (Math.abs(dphi) <= 0.0000000001) {
      return phi;
    }
  }

  throw new Error('IQSFN-CONV:Latitude failed to converge after 30 iterations');
}

/**
 * Convenience function to compute latiso(eccent, phi, sinphi)
 * @param eccent - eccentricity
 * @param phi - latitude
 * @param sinphi - sin of latitude
 * @returns - latiso(eccent, phi, sinphi)
 */
export function latiso(eccent: number, phi: number, sinphi: number): number {
  if (Math.abs(phi) > HALF_PI) return Number.NaN;
  if (phi === HALF_PI) return Number.POSITIVE_INFINITY;
  if (phi === -1 * HALF_PI) return Number.NEGATIVE_INFINITY;

  const con = eccent * sinphi;
  return Math.log(Math.tan((HALF_PI + phi) / 2)) + (eccent * Math.log((1 - con) / (1 + con))) / 2;
}

/**
 * Convenience function to compute log1py(x)
 * @param x - input
 * @returns - log1py(x)
 */
export function log1py(x: number): number {
  const y = 1 + x;
  const z = y - 1;

  return z === 0 ? x : (x * Math.log(y)) / z;
}

/**
 * Convienience function to compute mlfn
 * @param e0 - input
 * @param e1 - input
 * @param e2 - input
 * @param e3 - input
 * @param phi - latitude
 * @returns - mlfn(e0, e1, e2, e3, phi)
 */
export function mlfn(e0: number, e1: number, e2: number, e3: number, phi: number): number {
  return e0 * phi - e1 * Math.sin(2 * phi) + e2 * Math.sin(4 * phi) - e3 * Math.sin(6 * phi);
}

/**
 * Convienience function to compute msfnz(eccent, sinphi, cosphi)
 * @param eccent - eccentricity
 * @param sinphi - sin of latitude
 * @param cosphi - cos of latitude
 * @returns - msfnz(eccent, sinphi, cosphi)
 */
export function msfnz(eccent: number, sinphi: number, cosphi: number): number {
  const con = eccent * sinphi;
  return cosphi / Math.sqrt(1 - con * con);
}

/**
 * Convenience function to compute phi2z(eccent, ts)
 * @param eccent - eccentricity
 * @param ts - input
 * @returns - phi2z(eccent, ts)
 */
export function phi2z(eccent: number, ts: number): number {
  const eccnth = 0.5 * eccent;
  let con, dphi;
  let phi = HALF_PI - 2 * Math.atan(ts);
  for (let i = 0; i <= 15; i++) {
    con = eccent * Math.sin(phi);
    dphi = HALF_PI - 2 * Math.atan(ts * Math.pow((1 - con) / (1 + con), eccnth)) - phi;
    phi += dphi;
    if (Math.abs(dphi) <= 0.0000000001) {
      return phi;
    }
  }
  throw new Error('phi2z has NoConvergence');
}

/** The 5 elements of the eccentricity vector. */
export type En = [number, number, number, number, number];

/**
 * Convenience function to compute enfn(es)
 * @param es - eccentricity
 * @returns - enfn(es)
 */
export function pjEnfn(es: number): En {
  const C00 = 1;
  const C02 = 0.25;
  const C04 = 0.046875;
  const C06 = 0.01953125;
  const C08 = 0.01068115234375;
  const C22 = 0.75;
  const C44 = 0.46875;
  const C46 = 0.01302083333333333333;
  const C48 = 0.00712076822916666666;
  const C66 = 0.36458333333333333333;
  const C68 = 0.00569661458333333333;
  const C88 = 0.3076171875;
  const en: En = [0, 0, 0, 0, 0];

  en[0] = C00 - es * (C02 + es * (C04 + es * (C06 + es * C08)));
  en[1] = es * (C22 - es * (C04 + es * (C06 + es * C08)));
  let t = es * es;
  en[2] = t * (C44 - es * (C46 + es * C48));
  t *= es;
  en[3] = t * (C66 - es * C68);
  en[4] = t * es * C88;

  return en;
}

/**
 * Convenience function for pjInvMlfn(arg, es, en)
 * @param arg - latitude
 * @param es - eccentricity
 * @param en - input
 * @returns - pjInvMlfn(arg, es, en)
 */
export function pjInvMlfn(arg: number, es: number, en: En): number {
  const MAX_ITER = 20;
  const k = 1 / (1 - es);
  let phi = arg;
  for (let i = MAX_ITER; i !== 0; --i) {
    /* rarely goes over 2 iterations */
    const s = Math.sin(phi);
    let t = 1 - es * s * s;
    //t = this.pj_mlfn(phi, s, Math.cos(phi), en) - arg;
    //phi -= t * (t * Math.sqrt(t)) * k;
    t = (pjMlfn(phi, s, Math.cos(phi), en) - arg) * (t * Math.sqrt(t)) * k;
    phi -= t;
    if (Math.abs(t) < EPSLN) {
      return phi;
    }
  }

  throw new Error('cass:pj_inv_mlfn: Convergence error');
}

/**
 * Convenience function for pjMlfn(phi, sphi, cphi, en)
 * @param phi - latitude
 * @param sphi - sin of latitude
 * @param cphi - cos of latitude
 * @param en - input
 * @returns - pjMlfn(phi, sphi, cphi, en)
 */
export function pjMlfn(phi: number, sphi: number, cphi: number, en: En): number {
  cphi *= sphi;
  sphi *= sphi;
  return en[0] * phi - cphi * (en[1] + sphi * (en[2] + sphi * (en[3] + sphi * en[4])));
}

/**
 * Convenience function for qsfnz(eccent, sinphi)
 * @param eccent - eccentricity
 * @param sinphi - sin of latitude
 * @returns - qsfnz(eccent, sinphi)
 */
export function qsfnz(eccent: number, sinphi: number): number {
  let con;
  if (eccent > 1.0e-7) {
    con = eccent * sinphi;
    return (
      (1 - eccent * eccent) *
      (sinphi / (1 - con * con) - (0.5 / eccent) * Math.log((1 - con) / (1 + con)))
    );
  } else {
    return 2 * sinphi;
  }
}

/**
 * Returns the sign of x
 * @param x - The value to get the sign of
 * @returns - 1 if x is positive, -1 if x is negative
 */
export function sign(x: number): 1 | -1 {
  return x < 0 ? -1 : 1;
}

/**
 * Returns the hyperbolic sine of x.
 * @param x - The value to calculate the hyperbolic sine of
 * @returns - sinh(x)
 */
export function sinh(x: number): number {
  let r = Math.exp(x);
  r = (r - 1 / r) / 2;
  return r;
}

/**
 * Returns the ratio of the exponential of two numbers
 * @param esinp - The ratio
 * @param exp - The exponent
 * @returns - srat(esinp, exp)
 */
export function srat(esinp: number, exp: number): number {
  return Math.pow((1 - esinp) / (1 + esinp), exp);
}

/**
 * Returns the hyperbolic tangent of x.
 * @param x - The value to calculate the hyperbolic tangent of
 * @returns - tanh(x)
 */
export function tanh(x: number): number {
  let r = Math.exp(x);
  r = (r - 1 / r) / (r + 1 / r);
  return r;
}

/**
 * Apply the spherical formulae to obtain the conformal latitude
 * @param eccent - eccentricity
 * @param phi - latitude
 * @param sinphi - Math.sin(latitude)
 * @returns - conformal latitude
 */
export function tsfnz(eccent: number, phi: number, sinphi: number): number {
  let con = eccent * sinphi;
  const com = 0.5 * eccent;
  con = Math.pow((1 - con) / (1 + con), com);
  return Math.tan(0.5 * (HALF_PI - phi)) / con;
}
