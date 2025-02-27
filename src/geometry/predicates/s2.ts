import { pointCross, pointDot } from '../../geometry/s2/point';

import type { VectorPoint } from '../..';

const DBL_EPSILON = 2.2204460492503131e-16;

// A predefined S1ChordAngle representing (approximately) 45 degrees.
// static const S1ChordAngle k45Degrees = S1ChordAngle::FromLength2(2 - M_SQRT2);

/**
 * Returns +1 if the points A, B, C are counterclockwise, -1 if the points
 * are clockwise, and 0 if any two points are the same.  This function is
 * essentially like taking the sign of the determinant of ABC, except that
 * it has additional logic to make sure that the above properties hold even
 * when the three points are coplanar, and to deal with the limitations of
 * floating-point arithmetic.
 *
 * Sign satisfies the following conditions:
 *
 *  (1) Sign(a,b,c) == 0 if and only if a == b, b == c, or c == a
 *  (2) Sign(b,c,a) == Sign(a,b,c) for all a,b,c
 *  (3) Sign(c,b,a) == -Sign(a,b,c) for all a,b,c
 *
 * In other words:
 *
 *  (1) The result is zero if and only if two points are the same.
 *  (2) Rotating the order of the arguments does not affect the result.
 *  (3) Exchanging any two arguments inverts the result.
 *
 * On the other hand, note that it is not true in general that
 * Sign(-a,b,c) == -Sign(a,b,c), or any similar identities
 * involving antipodal points.
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @param c - The third XYZ Point
 * @returns - This version of Sign returns +1 if the points are definitely CCW, -1 if they are
 * definitely CW, and 0 if two points are identical or the result is uncertain.
 */
export function s2Sign(a: VectorPoint, b: VectorPoint, c: VectorPoint): 0 | 1 | -1 {
  // We don't need RobustCrossProd() here because Sign() does its own
  // error estimation and calls ExpensiveSign() if there is any uncertainty
  // about the result.
  return _s2Sign(a, b, c, pointCross(a, b));
}

/**
 * Most clients will not need the following methods.  They can be slightly
 * more efficient but are harder to use, since they require the client to do
 * all the actual crossing tests.
 *
 * A more efficient version of Sign that allows the precomputed
 * cross-product of A and B to be specified.  (Unlike the 3 argument
 * version this method is also inlined.)  Note that "aCrossB" must be
 * computed using CrossProd rather than S2::RobustCrossProd.
 *
 * REQUIRES: aCrossB == a.CrossProd(b)
 * @param _a - The first XYZ Point
 * @param _b - The second XYZ Point
 * @param c - The third XYZ Point
 * @param aCrossB - The cross product of a and b
 * @returns - This version of Sign returns +1 if the points are definitely CCW, -1 if they are
 * definitely CW, and 0 if two points are identical or the result is uncertain.
 */
function _s2Sign(
  _a: VectorPoint,
  _b: VectorPoint,
  c: VectorPoint,
  aCrossB: VectorPoint,
): 0 | 1 | -1 {
  return _s2TriageSign(c, aCrossB);
  // NOTE: We drop expensive sign until I come up with a reasonable way to reproduce it.
  // if (sign === 0) sign = ExpensiveSign(a, b, c);
  // return sign;
}

/**
 * This version of Sign returns +1 if the points are definitely CCW, -1 if
 * they are definitely CW, and 0 if two points are identical or the result
 * is uncertain.  Uncertain cases can be resolved, if desired, by calling
 * ExpensiveSign.
 *
 * The purpose of this method is to allow additional cheap tests to be done,
 * where possible, in order to avoid calling ExpensiveSign unnecessarily.
 *
 * REQUIRES: aCrossB == a.CrossProd(b)
 * @param c - The third XYZ Point
 * @param aCrossB - The cross product of a and b
 * @returns - This version of Sign returns +1 if the points are definitely CCW, -1 if they are
 * definitely CW, and 0 if two points are identical or the result is uncertain.
 */
function _s2TriageSign(c: VectorPoint, aCrossB: VectorPoint) {
  // kMaxDetError is the maximum error in computing (AxB).C where all vectors
  // are unit length.  Using standard inequalities, it can be shown that
  //
  //  fl(AxB) = AxB + D where |D| <= (|AxB| + (2/sqrt(3))*|A|*|B|) * e
  //
  // where "fl()" denotes a calculation done in floating-point arithmetic,
  // |x| denotes either absolute value or the L2-norm as appropriate, and
  // e = 0.5*DBL_EPSILON. Similarly,
  //
  //  fl(B.C) = B.C + d where |d| <= (1.5*|B.C| + 1.5*|B|*|C|) * e .
  //
  // Applying these bounds to the unit-length vectors A,B,C and neglecting
  // relative error (which does not affect the sign of the result), we get
  //
  //  fl((AxB).C) = (AxB).C + d where |d| <= (2.5 + 2/sqrt(3)) * e
  //
  // which is about 3.6548 * e, or 1.8274 * DBL_EPSILON.
  //
  // In order to support vectors of magnitude <= sqrt(2), we double this value.
  const kMaxDetError = 3.6548 * DBL_EPSILON;
  const det = pointDot(aCrossB, c);

  if (det > kMaxDetError) return 1;
  if (det < -kMaxDetError) return -1;
  return 0;
}

/**
 * Given 4 points on the unit sphere, return true if the edges OA, OB, and
 * OC are encountered in that order while sweeping CCW around the point O.
 * You can think of this as testing whether A <= B <= C with respect to the
 * CCW ordering around O that starts at A, or equivalently, whether B is
 * contained in the range of angles (inclusive) that starts at A and extends
 * CCW to C.  Properties:
 *
 *  (1) If OrderedCCW(a,b,c,o) && OrderedCCW(b,a,c,o), then a == b
 *  (2) If OrderedCCW(a,b,c,o) && OrderedCCW(a,c,b,o), then b == c
 *  (3) If OrderedCCW(a,b,c,o) && OrderedCCW(c,b,a,o), then a == b == c
 *  (4) If a == b or b == c, then OrderedCCW(a,b,c,o) is true
 *  (5) Otherwise if a == c, then OrderedCCW(a,b,c,o) is false
 *
 * REQUIRES: a != o && b != o && c != o
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @param c - The third XYZ Point
 * @param o - The reference XYZ Point
 * @returns - Returns true if the edges OA, OB, and OC are encountered in that order while sweeping CCW around the point O
 * (see above).
 */
export function s2OrderedCCW(
  a: VectorPoint,
  b: VectorPoint,
  c: VectorPoint,
  o: VectorPoint,
): boolean {
  // The last inequality below is ">" rather than ">=" so that we return true
  // if A == B or B == C, and otherwise false if A == C.  Recall that
  // Sign(x,y,z) == -Sign(z,y,x) for all x,y,z.
  let sum = 0;
  if (s2Sign(b, o, a) >= 0) ++sum;
  if (s2Sign(c, o, b) >= 0) ++sum;
  if (s2Sign(a, o, c) > 0) ++sum;
  return sum >= 2;
}
