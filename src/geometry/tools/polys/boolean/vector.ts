import { pointDot, pointLength } from '../../../index.js';

import type { SweepEvent } from './sweepEvent.js';
import type { VectorPoint } from '../../../index.js';

/** A boolean vector type used by the boolean operations */
// @ts-expect-error - ignore for now
export type BoolVec = VectorPoint<SweepEvent[]>;

/**
 * Cross Product of two vectors with first point at origin
 * @param a - the first vector
 * @param b - the second vector
 * @returns - the cross product
 */
export function crossProduct(a: BoolVec, b: BoolVec): number {
  return a.x * b.y - a.y * b.x;
}

/**
 * Get the sine of the angle from pShared -> pAngle to pShaed -> pBase
 * @param pShared - the shared point
 * @param pBase - the base point
 * @param pAngle - the angle
 * @returns - the sine angle
 */
export function sineOfAngle(pShared: BoolVec, pBase: BoolVec, pAngle: BoolVec): number {
  const vBase = { x: pBase.x - pShared.x, y: pBase.y - pShared.y };
  const vAngle = { x: pAngle.x - pShared.x, y: pAngle.y - pShared.y };
  return crossProduct(vAngle, vBase) / pointLength(vAngle) / pointLength(vBase);
}

/**
 * Get the cosine of the angle from pShared -> pAngle to pShaed -> pBase
 * @param pShared - the shared point
 * @param pBase - the base point
 * @param pAngle - the angle
 * @returns - the cosine angle
 */
export function cosineOfAngle(pShared: BoolVec, pBase: BoolVec, pAngle: BoolVec): number {
  const vBase = { x: pBase.x - pShared.x, y: pBase.y - pShared.y };
  const vAngle = { x: pAngle.x - pShared.x, y: pAngle.y - pShared.y };
  return pointDot(vAngle, vBase) / pointLength(vAngle) / pointLength(vBase);
}

/**
 * Get the x coordinate where the given line (defined by a point and vector)
 * crosses the horizontal line with the given y coordiante.
 * In the case of parrallel lines (including overlapping ones) returns undefined.
 * @param pt - the base point
 * @param v - the vector
 * @param y - the y coordinate
 * @returns - the horizontal intersection
 */
export function horizontalIntersection(pt: BoolVec, v: BoolVec, y: number): BoolVec | undefined {
  if (v.y === 0) return undefined;
  return { x: pt.x + (v.x / v.y) * (y - pt.y), y };
}

/**
 * Get the y coordinate where the given line (defined by a point and vector)
 * crosses the vertical line with the given x coordiante.
 * In the case of parrallel lines (including overlapping ones) returns undefined.
 * @param pt - the base point
 * @param v - the vector
 * @param x - the x coordinate
 * @returns - the vertical intersection
 */
export function verticalIntersection(pt: BoolVec, v: BoolVec, x: number): BoolVec | undefined {
  if (v.x === 0) return undefined;
  return { x, y: pt.y + (v.y / v.x) * (x - pt.x) };
}

/**
 * Get the intersection of two lines, each defined by a base point and a vector.
 * In the case of parrallel lines (including overlapping ones) returns undefined.
 * @param pt1 - the base point of the first line
 * @param v1 - the vector of the first line
 * @param pt2 - the base point of the second line
 * @param v2 - the vector of the second line
 * @returns - the intersection between the two lines
 */
export function vectorIntersection(
  pt1: BoolVec,
  v1: BoolVec,
  pt2: BoolVec,
  v2: BoolVec,
): BoolVec | undefined {
  // take some shortcuts for vertical and horizontal lines
  // this also ensures we don't calculate an intersection and then discover
  // it's actually outside the bounding box of the line
  if (v1.x === 0) return verticalIntersection(pt2, v2, pt1.x);
  if (v2.x === 0) return verticalIntersection(pt1, v1, pt2.x);
  if (v1.y === 0) return horizontalIntersection(pt2, v2, pt1.y);
  if (v2.y === 0) return horizontalIntersection(pt1, v1, pt2.y);

  // General case for non-overlapping segments.
  // This algorithm is based on Schneider and Eberly.
  // http://www.cimec.org.ar/~ncalvo/Schneider_Eberly.pdf - pg 244

  const kross = crossProduct(v1, v2);
  if (kross === 0) return undefined;

  const ve = { x: pt2.x - pt1.x, y: pt2.y - pt1.y };
  const d1 = crossProduct(ve, v1) / kross;
  const d2 = crossProduct(ve, v2) / kross;

  // take the average of the two calculations to minimize rounding error
  const x1 = pt1.x + d2 * v1.x;
  const x2 = pt2.x + d1 * v2.x;
  const y1 = pt1.y + d2 * v1.y;
  const y2 = pt2.y + d1 * v2.y;
  const x = (x1 + x2) / 2;
  const y = (y1 + y2) / 2;
  return { x, y };
}

/**
 * Given a vector, return one that is perpendicular
 * @param v - the vector
 * @returns - the perpendicular vector
 */
export function perpendicular(v: BoolVec): BoolVec {
  return { x: -v.y, y: v.x };
}
