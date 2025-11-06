import { equalPoints, orient2d } from '../../index.js';

import type { MValue, Properties, VectorPoint } from '../../index.js';

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
  const [{ x: x1, y: y1 }, { x: x2, y: y2 }] = a;
  const [{ x: x3, y: y3 }, { x: x4, y: y4 }] = b;
  const dxA = x2 - x1;
  const dyA = y2 - y1;
  const dxB = x4 - x3;
  const dyB = y4 - y3;

  // build numerators and denominator. Extrapolate vectors from them
  const denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
  const numeA = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
  const numeB = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
  const uA = numeA / denom;
  const uB = numeB / denom;
  const uVec = { x: uA * dxA, y: uA * dyA };
  const tVec = { x: uB * dxB, y: uB * dyB };

  if (sameRing) {
    if (
      equalPoints(a[1], b[0]) ||
      equalPoints(a[1], b[1]) ||
      equalPoints(a[0], b[0]) ||
      equalPoints(a[0], b[1])
    )
      return undefined;
  } else {
    if (equalPoints(a[1], b[0])) return { point: { x: x2, y: y2 }, u: 1, t: 0, uVec, tVec };
    if (equalPoints(a[1], b[1])) return { point: { x: x2, y: y2 }, u: 1, t: 1, uVec, tVec };
    if (equalPoints(a[0], b[0])) return { point: { x: x1, y: y1 }, u: 0, t: 0, uVec, tVec };
    if (equalPoints(a[0], b[1])) return { point: { x: x1, y: y1 }, u: 0, t: 1, uVec, tVec };
  }
  if (denom === 0) return undefined;

  const orient1 = orient2d(x1, y1, x2, y2, x3, y3);
  const orient2 = orient2d(x1, y1, x2, y2, x4, y4);
  if (orient1 > 0 && orient2 > 0) return undefined;
  else if (orient1 < 0 && orient2 < 0) return undefined;

  if (uA >= 0 && uA <= 1 && uB >= 0 && uB <= 1) {
    return { point: { x: x1 + uA * (x2 - x1), y: y1 + uA * (y2 - y1) }, u: uA, t: uB, uVec, tVec };
  }
  return undefined;
}
