import { lonLatToXYZ } from '../s2/coords';
import { Point, Point3D } from '../geometry.spec';
import { degToRad, radToDeg } from '../util';

import type { S1Angle } from '../s1/angle';

/** Just another way of defining a standard 2D point. */
export type LonLat = Point;

/**
 * Converts an LonLat to the equivalent unit-length vector.  Unnormalized
 * values (see Normalize()) are wrapped around the sphere as would be expected
 * based on their definition as spherical angles.  So for example the
 * following pairs yield equivalent points (modulo numerical error):
 *     (90.5, 10) =~ (89.5, -170)
 *     (a, b) =~ (a + 360 * n, b)
 * The maximum error in the result is 1.5 * DBL_EPSILON.  (This does not
 * include the error of converting degrees, E5, E6, or E7 to radians.)
 *
 * Can be used just like an S2Point constructor.  For example:
 *   S2Cap cap;
 *   cap.AddPoint(S2Point(latlon));
 * @param ll - input LonLat
 * @returns - equivalent unit-length vector 3D point
 */
export function toS2Point(ll: LonLat): Point3D {
  return lonLatToXYZ(ll[0], ll[1]);
}

/**
 * Convert a direction vector (not necessarily unit length) to an LonLat.
 * @param p - input direction vector
 * @returns - LonLat
 */
export function fromS2Point(p: Point3D): LonLat {
  const { atan2, sqrt } = Math;
  const [x, y, z] = p;

  return [radToDeg(atan2(y, x)), radToDeg(atan2(z, sqrt(x * x + y * y)))];
}

/**
 * @param ll - input LonLat
 * @returns a lon-lat in radians
 */
export function toAngles(ll: LonLat): [S1Angle, S1Angle] {
  return ll.map(degToRad) as [S1Angle, S1Angle];
}

/**
 * @param ll - input lon-lat in degrees
 * @returns - ensures lon in [-180, 180], lat in [-90, 90]
 */
export function normalize(ll: LonLat): [number, number] {
  let [lon, lat] = ll;
  // Normalize longitude using modulo
  lon = ((((lon + 180) % 360) + 360) % 360) - 180;
  // Clamp latitude between -90 and 90
  lat = Math.max(-90, Math.min(90, lat));

  return [lon, lat];
}

/**
 * Returns the distance (measured along the surface of the sphere) to the
 * given LonLat, implemented using the Haversine formula.  This is
 * equivalent to
 *
 *   S1Angle(ToPoint(), o.ToPoint())
 *
 * except that this function is slightly faster, and is also somewhat less
 * accurate for distances approaching 180 degrees (see s1angle.h for
 * details).  Both LngLats must be normalized.
 * @param a - input LonLat
 * @param b - input LonLat
 * @returns - distance in radians
 */
export function getDistance(a: LonLat, b: LonLat): number {
  const { asin, sin, cos, sqrt, min } = Math;
  // This implements the Haversine formula, which is numerically stable for
  // small distances but only gets about 8 digits of precision for very large
  // distances (e.g. antipodal points).  Note that 8 digits is still accurate
  // to within about 10cm for a sphere the size of the Earth.
  //
  // This could be fixed with another sin() and cos() below, but at that point
  // you might as well just convert both arguments to S2Points and compute the
  // distance that way (which gives about 15 digits of accuracy for all
  // distances).
  let [lonA, latA] = a;
  let [lonB, latB] = b;
  // conver all to radians
  lonA = degToRad(lonA);
  latA = degToRad(latA);
  lonB = degToRad(lonB);
  latB = degToRad(latB);
  const dlat = sin(0.5 * (latB - latA));
  const dlon = sin(0.5 * (lonB - lonA));
  const x = dlat * dlat + dlon * dlon * cos(latA) * cos(latB);
  return 2 * asin(sqrt(min(1, x)));
}
