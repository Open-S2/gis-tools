import { toKM as angleToKM, toMeters as angleToMeters } from './angle';
import { norm2, sub } from '../s2/point';

import type { Point3D } from '../';
import type { S1Angle } from './angle';

/**
 * S1ChordAngle represents the angle subtended by a chord (i.e., the straight
 * line segment connecting two points on the sphere).  Its representation
 * makes it very efficient for computing and comparing distances, but unlike
 * S1Angle it is only capable of representing angles between 0 and Pi radians.
 * S1ChordAngle is intended for applications where many angles need to be
 * computed and compared, otherwise it is simpler to use S1Angle.
 *
 * S1ChordAngle also loses some accuracy as the angle approaches Pi radians.
 * There are several different ways to measure this error, including the
 * representational error (i.e., how accurately S1ChordAngle can represent
 * angles near Pi radians), the conversion error (i.e., how much precision is
 * lost when an S1Angle is converted to an S1ChordAngle), and the measurement
 * error (i.e., how accurate the S1ChordAngle(a, b) constructor is when the
 * points A and B are separated by angles close to Pi radians).  All of these
 * errors differ by a small constant factor.
 *
 * For the measurement error (which is the largest of these errors and also
 * the most important in practice), let the angle between A and B be (Pi - x)
 * radians, i.e. A and B are within "x" radians of being antipodal.  The
 * corresponding chord length is
 *
 *    r = 2 * sin((Pi - x) / 2) = 2 * cos(x / 2) .
 *
 * For values of x not close to Pi the relative error in the squared chord
 * length is at most 4.5 * DBL_EPSILON (see GetS2PointConstructorMaxError).
 * The relative error in "r" is thus at most 2.25 * DBL_EPSILON ~= 5e-16.  To
 * convert this error into an equivalent angle, we have
 *
 *    |dr / dx| = sin(x / 2)
 *
 * and therefore
 *
 *    |dx| = dr / sin(x / 2)
 *         = 5e-16 * (2 * cos(x / 2)) / sin(x / 2)
 *         = 1e-15 / tan(x / 2)
 *
 * The maximum error is attained when
 *
 *    x  = |dx|
 *       = 1e-15 / tan(x / 2)
 *      ~= 1e-15 / (x / 2)
 *      ~= sqrt(2e-15)
 *
 * In summary, the measurement error for an angle (Pi - x) is at most
 *
 *    dx  = min(1e-15 / tan(x / 2), sqrt(2e-15))
 *      (~= min(2e-15 / x, sqrt(2e-15)) when x is small).
 *
 * On the Earth's surface (assuming a radius of 6371km), this corresponds to
 * the following worst-case measurement errors:
 *
 *     Accuracy:             Unless antipodal to within:
 *     ---------             ---------------------------
 *     6.4 nanometers        10,000 km (90 degrees)
 *     1 micrometer          81.2 kilometers
 *     1 millimeter          81.2 meters
 *     1 centimeter          8.12 meters
 *     28.5 centimeters      28.5 centimeters
 *
 * The representational and conversion errors referred to earlier are somewhat
 * smaller than this.  For example, maximum distance between adjacent
 * representable S1ChordAngle values is only 13.5 cm rather than 28.5 cm.  To
 * see this, observe that the closest representable value to r^2 = 4 is
 * r^2 =  4 * (1 - DBL_EPSILON / 2).  Thus r = 2 * (1 - DBL_EPSILON / 4) and
 * the angle between these two representable values is
 *
 *    x  = 2 * acos(r / 2)
 *       = 2 * acos(1 - DBL_EPSILON / 4)
 *      ~= 2 * asin(sqrt(DBL_EPSILON / 2)
 *      ~= sqrt(2 * DBL_EPSILON)
 *      ~= 2.1e-8
 *
 * which is 13.5 cm on the Earth's surface.
 *
 * The worst case rounding error occurs when the value halfway between these
 * two representable values is rounded up to 4.  This halfway value is
 * r^2 = (4 * (1 - DBL_EPSILON / 4)), thus r = 2 * (1 - DBL_EPSILON / 8) and
 * the worst case rounding error is
 *
 *    x  = 2 * acos(r / 2)
 *       = 2 * acos(1 - DBL_EPSILON / 8)
 *      ~= 2 * asin(sqrt(DBL_EPSILON / 4)
 *      ~= sqrt(DBL_EPSILON)
 *      ~= 1.5e-8
 *
 * which is 9.5 cm on the Earth's surface.
 *
 * This class is intended to be copied by value as desired.  It uses
 * the default copy constructor and assignment operator.
 */
export type S1ChordAngle = number;

/** The Maximum allowed squared chord length. */
export const K_MAX_LENGTH_2 = 4.0;

/**
 * Conversion from an S1Angle.  Angles outside the range [0, Pi] are handled
 * as follows: Infinity() is mapped to Infinity(), negative angles are
 * mapped to Negative(), and finite angles larger than Pi are mapped to
 * Straight().
 *
 * Note that this operation is relatively expensive and should be avoided.
 * To use S1ChordAngle effectively, you should structure your code so that
 * input arguments are converted to S1ChordAngles at the beginning of your
 * algorithm, and results are converted back to S1Angles only at the end.
 *
 * S1ChordAngles are represented by the squared chord length, which can
 * range from 0 to 4.  Infinity() uses an infinite squared length.
 * @param angle - An angle in radians.
 * @returns The corresponding ChordAngle.
 */
export function fromAngle(angle: S1Angle): S1ChordAngle {
  const { sin, min, PI } = Math;
  let length2_ = 0.0;
  if (angle < 0.0) {
    return -1;
  } else if (angle === Infinity) {
    return Infinity;
  } else {
    // The chord length is 2 * sin(angle / 2).
    const length = 2.0 * sin(0.5 * min(PI, angle));
    length2_ = length * length;
  }

  return length2_;
}

/**
 * Construct an S1ChordAngle from the squared chord length.  Note that the
 * argument is automatically clamped to a maximum of 4.0 to handle possible
 * roundoff errors.  The argument must be non-negative.
 * @param length2_ - The squared chord length.
 * @returns The corresponding ChordAngle.
 */
export function fromLength2(length2_: number): S1ChordAngle {
  return Math.min(K_MAX_LENGTH_2, length2_);
}

/**
 * Construct the S1ChordAngle corresponding to the distance between the two
 * given points.  The points must be unit length.
 * @param a - The first point.
 * @param b - The second point.
 * @returns The corresponding ChordAngle.
 */
export function fromS2Points(a: Point3D, b: Point3D): S1ChordAngle {
  // The squared distance may slightly exceed 4.0 due to roundoff errors.
  // The maximum error in the result is 2 * DBL_EPSILON * length2_.
  return Math.min(K_MAX_LENGTH_2, norm2(sub(a, b)));
}

/**
 * Return a chord angle of 90 degrees (a "right angle").
 * @returns The right angle.
 */
export function rightAngle(): S1ChordAngle {
  return 2.0;
}

/**
 * Return a chord angle of 180 degrees (a "straight angle").  This is the
 * maximum finite chord angle.
 * @returns The straight angle.
 */
export function straightAngle(): S1ChordAngle {
  return K_MAX_LENGTH_2;
}

/**
 * Return a chord angle smaller than Zero().  The only valid operations on
 * Negative() are comparisons, S1Angle conversions, and successor() /
 * predecessor().
 * @returns The negative angle.
 */
export function negativeAngle(): S1ChordAngle {
  return -1;
}

/**
 * Construct an S1ChordAngle that is an upper bound on the given S1Angle.
 * i.i. such that FastUpperBoundFrom(x).toAngle() >= x. Unlike the S1Angle
 * constructor above, this method is very fast, and the bound is accurate to
 * within 1% for distances up to about 3100km on the Earth's surface.
 * @param angle - The S1Angle.
 * @returns The corresponding S1ChordAngle.
 */
export function fastUpperBoundFrom(angle: S1Angle): S1ChordAngle {
  // This method uses the distance along the surface of the sphere as an upper
  // bound on the distance through the sphere's interior.
  return fromLength2(angle * angle);
}

/**
 * Convenience function to test if a ChordAngle is special.
 * @param cAngle - The ChordAngle to test.
 * @returns - true if the ChordAngle is special.
 */
export function isSpecial(cAngle: S1ChordAngle): boolean {
  return cAngle < 0 || cAngle === Infinity;
}

/**
 * Convert to an S1Angle.
 * Infinity() is converted to S1Angle.Infinity(), and Negative() is
 * converted to an unspecified negative S1Angle.
 *
 * Note that the conversion uses trigonometric functions and therefore
 * should be avoided in inner loops.
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding S1Angle.
 */
export function toAngle(cAngle: S1ChordAngle): S1Angle {
  if (cAngle < 0) return -1.0;
  if (cAngle === Infinity) return Infinity;
  return 2 * Math.asin(0.5 * Math.sqrt(cAngle));
}

/**
 * Convert to meters.
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding number of meters.
 */
export function toMeters(cAngle: S1ChordAngle): number {
  return angleToMeters(toAngle(cAngle));
}

/**
 * Convert to kilometers.
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding number of kilometers.
 */
export function toKM(cAngle: S1ChordAngle): number {
  return angleToKM(toAngle(cAngle));
}

// Trigonmetric functions.  It is more accurate and efficient to call these
// rather than first converting to an S1Angle.

/**
 * apply a sine function on a ChordAngle
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding sin(S1Angle).
 */
export function chordAngleSin(cAngle: S1ChordAngle): number {
  return Math.sqrt(chordAngleSin2(cAngle));
}

/**
 * apply a cosine function on a ChordAngle
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding cos(S1Angle).
 */
export function chordAngleCos(cAngle: S1ChordAngle): number {
  // cos(2*A) = cos^2(A) - sin^2(A) = 1 - 2*sin^2(A)
  return 1.0 - 0.5 * cAngle;
}

/**
 * apply a tangent function on a ChordAngle
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding cos(S1Angle).
 */
export function chordAngleTan(cAngle: S1ChordAngle): number {
  return chordAngleSin(cAngle) / chordAngleCos(cAngle);
}

/**
 * Returns sin(a)^2, but computed more efficiently.
 * @param cAngle - The ChordAngle to convert.
 * @returns The corresponding sin(S1Angle)^2.
 */
export function chordAngleSin2(cAngle: S1ChordAngle): number {
  // Let "a" be the (non-squared) chord length, and let A be the corresponding
  // half-angle (a = 2*sin(A)).  The formula below can be derived from:
  //   sin(2*A) = 2 * sin(A) * cos(A)
  //   cos^2(A) = 1 - sin^2(A)
  // This is much faster than converting to an angle and computing its sine.
  return cAngle * (1.0 - 0.25 * cAngle);
}
