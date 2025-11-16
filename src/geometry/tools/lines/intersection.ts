import { equalPoints as equalP, nextDown, nextUp, orient2d } from '../../../index.js';

import type { MValue, Properties, VectorPoint } from '../../../index.js';

/**
 * An intersection of two segments
 * u and t are where the intersection occurs
 */
export interface IntersectionOfSegments<D extends MValue = Properties> {
  /** the intersection point */
  point: VectorPoint<D>;
  /** where along the first segment the intersection occurs */
  u: number;
  /** where along the second segment the intersection occurs */
  t: number;
}

/**
 * An intersection of two segments including displacement vectors
 * u and t are where the intersection occurs
 */
export interface IntersectionOfSegmentsRobust<D extends MValue = Properties> {
  /** the intersection point */
  point: VectorPoint<D>;
  /** where along the first segment the intersection occurs */
  u: number;
  /** where along the second segment the intersection occurs */
  t: number;
  /** displacement vector from the first segment */
  uVec: VectorPoint<D>;
  /** displacement vector from the second segment */
  tVec: VectorPoint<D>;
  /** Absolute angle of segment 'a' in radians [-PI, PI] */
  uAngle: number;
  /** Absolute angle of segment 'b' in radians [-PI, PI] */
  tAngle: number;
}

/**
 * Find the intersection of two segments
 *
 * NOTE: Segments that are only touching eachothers endpoints are considered intersections
 * @param a - the first segment
 * @param b - the second segment
 * @returns A point if the segments intersect where the intersection occurs, otherwise undefined
 */
export function intersectionOfSegments<D extends MValue = Properties>(
  a: [VectorPoint<D>, VectorPoint<D>],
  b: [VectorPoint<D>, VectorPoint<D>],
): IntersectionOfSegments<D> | undefined {
  const [{ x: x1, y: y1 }, { x: x2, y: y2 }] = a;
  const [{ x: x3, y: y3 }, { x: x4, y: y4 }] = b;

  const r = { x: x2 - x1, y: y2 - y1 };
  const s = { x: x4 - x3, y: y4 - y3 };

  const cross = r.x * s.y - r.y * s.x;
  if (cross === 0) {
    return;
  }

  const u = ((x3 - x1) * s.y - (y3 - y1) * s.x) / cross;
  const t = ((x3 - x1) * r.y - (y3 - y1) * r.x) / cross;

  if (t >= 0 && t <= 1 && u >= 0 && u <= 1) {
    return { point: { x: x1 + u * r.x, y: y1 + u * r.y }, u, t };
  }
}

/**
 * Find the intersection of two segments. A more robust approach that uses predicates to ensure no
 * false positives/negatives
 *
 * NOTE:
 * If the segments are touching at end points, they PASS in this function. However, the caviat is
 * that if the segments are coming from the same ring, then the result will be undefined (not
 * considered an intersection).
 *
 * NOTE: The resultant vectors are displacement vectors not normalized.
 * @param a - the first segment
 * @param b - the second segment
 * @param sameRing - if both segments are from the same ring. By default it assumes they are
 * @returns a point if the segments intersect where the intersection occurs, otherwise undefined
 */
export function intersectionOfSegmentsRobust<D extends MValue = Properties>(
  a: [VectorPoint<D>, VectorPoint<D>],
  b: [VectorPoint<D>, VectorPoint<D>],
  sameRing: boolean = true,
): IntersectionOfSegmentsRobust<D> | undefined {
  const { atan2 } = Math;
  const [{ x: x1, y: y1 }, { x: x2, y: y2 }] = a;
  const [{ x: x3, y: y3 }, { x: x4, y: y4 }] = b;
  const [dxA, dyA] = [x2 - x1, y2 - y1];
  const [dxB, dyB] = [x4 - x3, y4 - y3];
  const [dxC, dyC] = [x1 - x3, y1 - y3];

  // corner case: A segment has 0 length
  if ((dxA === 0 && dyA === 0) || (dxB === 0 && dyB === 0)) return undefined;

  // Handle zero-length segments specifically (atan2(0,0) is 0, but good to be explicit)
  const uAngle = dxA === 0 && dyA === 0 ? 0 : atan2(dyA, dxA);
  const tAngle = dxB === 0 && dyB === 0 ? 0 : atan2(dyB, dxB);

  if (sameRing) {
    if (equalP(a[1], b[0]) || equalP(a[1], b[1]) || equalP(a[0], b[0]) || equalP(a[0], b[1]))
      return undefined;
  } else {
    const first = { x: x1, y: y1 };
    const second = { x: x2, y: y2 };
    const zero = { x: 0, y: 0 };
    if (equalP(a[1], b[0])) {
      return { point: second, u: 1, t: 0, uVec: { x: dxA, y: dyA }, tVec: zero, uAngle, tAngle };
    }
    if (equalP(a[1], b[1])) {
      const uVec = { x: dxA, y: dyA };
      const tVec = { x: dxB, y: dyB };
      return { point: second, u: 1, t: 1, uVec, tVec, uAngle, tAngle };
    }
    if (equalP(a[0], b[0])) {
      return { point: first, u: 0, t: 0, uVec: zero, tVec: zero, uAngle, tAngle };
    }
    if (equalP(a[0], b[1])) {
      const tVec = { x: dxB, y: dyB };
      return { point: first, u: 0, t: 1, uVec: zero, tVec, uAngle, tAngle };
    }
  }

  // Check the denominator and orientations
  const denom = dyB * dxA - dxB * dyA;
  if (denom === 0) return undefined;
  const orient1 = orient2d(x1, y1, x2, y2, x3, y3);
  const orient2 = orient2d(x1, y1, x2, y2, x4, y4);
  if ((orient1 > 0 && orient2 > 0) || (orient1 < 0 && orient2 < 0)) return undefined;

  // build vectors and angles, then normalize the vectors
  const numeA = dxB * dyC - dyB * dxC;
  const numeB = dxA * dyC - dyA * dxC;
  const uA = numeA / denom;
  const uB = numeB / denom;

  if (uA >= 0 && uA <= 1 && uB >= 0 && uB <= 1) {
    const point = { x: x1 + uA * dxA, y: y1 + uA * dyA };
    const uVec = { x: uA * dxA, y: uA * dyA };
    const tVec = { x: uB * dxB, y: uB * dyB };
    // If the intersection is at one of the endpoints becauase of float errors, move it to towards
    // the other endpoint by the smallest amount possible
    if (uA !== 0 && uA !== 1 && uB !== 0 && uB !== 1) {
      if (uA <= 0.5 && equalP(point, a[0])) {
        if (dxA !== 0) point.x = dxA > 0 ? nextUp(point.x) : nextDown(point.x);
        if (dyA !== 0) point.y = dyA > 0 ? nextUp(point.y) : nextDown(point.y);
      } else if (uA > 0.5 && equalP(point, a[1])) {
        if (dxA !== 0) point.x = dxA < 0 ? nextUp(point.x) : nextDown(point.x);
        if (dyA !== 0) point.y = dyA < 0 ? nextUp(point.y) : nextDown(point.y);
      } else if (uB <= 0.5 && equalP(point, b[0])) {
        if (dxB !== 0) point.x = dxB > 0 ? nextUp(point.x) : nextDown(point.x);
        if (dyB !== 0) point.y = dyB > 0 ? nextUp(point.y) : nextDown(point.y);
      } else if (uB > 0.5 && equalP(point, b[1])) {
        if (dxB !== 0) point.x = dxB < 0 ? nextUp(point.x) : nextDown(point.x);
        if (dyB !== 0) point.y = dyB < 0 ? nextUp(point.y) : nextDown(point.y);
      }
    }
    return { point, u: uA, t: uB, uVec, tVec, uAngle, tAngle };
  }
  return undefined;
}
