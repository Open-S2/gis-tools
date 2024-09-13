import { EPSLN, RA4, RA6, SIXTH } from './values';
import ellipsoids, { WGS84 } from './ellipsoid';

import type { Ellipsoid } from './ellipsoid';

import match from '../util/match';

/** Describes an ellipsoid's eccentricity */
export interface Eccentricity {
  es: number;
  e: number;
  ep2: number;
}

/**
 * @param ellipsoid - ellipsoid
 * @param RA - true if apply radius of curvature correction
 * @returns - eccentricity
 */
export function eccentricity(ellipsoid: Ellipsoid, RA = false): Eccentricity {
  let { a } = ellipsoid;
  const { b } = ellipsoid;
  let a2 = a * a; // used in geocentric
  const b2 = b * b; // used in geocentric
  let es = (a2 - b2) / a2; // e ^ 2
  let e = 0;
  if (RA) {
    a *= 1 - es * (SIXTH + es * (RA4 + es * RA6));
    a2 = a * a;
    es = 0;
  } else {
    e = Math.sqrt(es); // eccentricity
  }
  const ep2 = (a2 - b2) / b2; // used in geocentric
  return { es: es, e: e, ep2: ep2 };
}

/** Describes a sphere's eccentricity and if it is a true sphere or not */
export interface Sphere {
  a: number;
  b: number;
  rf: number;
  sphere: boolean;
}

/**
 * Builds a sphere with ellipsoid parameters
 * @param ellps - name of ellipsoid
 * @param a - semi-major axis
 * @param b - semi-minor axis
 * @param rf - inverse flattening
 * @param sphere - true if ellipsoid is a true sphere
 * @returns - Sphere with ellipsoid parameters
 */
export function sphere(
  ellps?: string,
  a?: number,
  b?: number,
  rf?: number,
  sphere = false,
): Sphere {
  if (a === undefined) {
    // do we have an ellipsoid?
    let ellipse = match<Ellipsoid>(ellipsoids, ellps);
    if (ellipse === undefined) ellipse = WGS84;
    a = ellipse.a;
    b = ellipse.b;
    rf = ellipse.rf;
  }
  if (rf && b === undefined) {
    b = (1.0 - 1.0 / rf) * a;
  } else {
    b = a;
  }
  if (rf === undefined) {
    rf = (a - b) / a;
  }
  if (rf === 0 || Math.abs(a - b) < EPSLN) {
    sphere = true;
    b = a;
  }

  return { a, b, rf, sphere };
}
