import { equalPoints, orient2d } from '../../index.js';

import type { MValue, Properties, VectorPoint } from '../../index.js';

/**
 * An intersection of two segments
 * u and t are where the intersection occurs
 */
export interface IntersectionOfSegments<D extends MValue = Properties> {
  point: VectorPoint<D>;
  u: number; // where along the first segment the intersection occurs
  t: number; // where along the second segment the intersection occurs
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
  const [p, p2] = a;
  const [q, q2] = b;

  const r = { x: p2.x - p.x, y: p2.y - p.y };
  const s = { x: q2.x - q.x, y: q2.y - q.y };

  const cross = r.x * s.y - r.y * s.x;
  if (cross === 0) {
    return;
  }

  const u = ((q.x - p.x) * s.y - (q.y - p.y) * s.x) / cross;
  const t = ((q.x - p.x) * r.y - (q.y - p.y) * r.x) / cross;

  if (t >= 0 && t <= 1 && u >= 0 && u <= 1) {
    return { point: { x: p.x + u * r.x, y: p.y + u * r.y }, u, t };
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
 * @param a - the first segment
 * @param b - the second segment
 * @param aRingID - the ring id of the first segment if provided
 * @param bRingID - the ring id of the second segment if provided
 * @returns a point if the segments intersect where the intersection occurs, otherwise undefined
 */
export function intersectionOfSegmentsRobust<D extends MValue = Properties>(
  a: [VectorPoint<D>, VectorPoint<D>],
  b: [VectorPoint<D>, VectorPoint<D>],
  aRingID?: number,
  bRingID?: number,
): IntersectionOfSegments<D> | undefined {
  const x1 = a[0].x;
  const y1 = a[0].y;
  const x2 = a[1].x;
  const y2 = a[1].y;
  const x3 = b[0].x;
  const y3 = b[0].y;
  const x4 = b[1].x;
  const y4 = b[1].y;

  if (aRingID === bRingID) {
    if (
      equalPoints(a[1], b[0]) ||
      equalPoints(a[1], b[1]) ||
      equalPoints(a[0], b[0]) ||
      equalPoints(a[0], b[1])
    )
      return undefined;
  } else {
    if (equalPoints(a[1], b[0])) return { point: { x: x2, y: y2 }, u: 1, t: 0 };
    if (equalPoints(a[1], b[1])) return { point: { x: x2, y: y2 }, u: 1, t: 1 };
    if (equalPoints(a[0], b[0])) return { point: { x: x1, y: y1 }, u: 0, t: 0 };
    if (equalPoints(a[0], b[1])) return { point: { x: x1, y: y1 }, u: 0, t: 1 };
  }

  const orient1 = orient2d(x1, y1, x2, y2, x3, y3);
  const orient2 = orient2d(x1, y1, x2, y2, x4, y4);

  if (orient1 > 0 && orient2 > 0) return undefined;
  else if (orient1 < 0 && orient2 < 0) return undefined;

  const denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
  const numeA = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
  const numeB = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

  if (denom === 0) {
    // if (numeA === 0 && numeB === 0) return undefined;
    return undefined;
  }

  const uA = numeA / denom;
  const uB = numeB / denom;

  if (uA >= 0 && uA <= 1 && uB >= 0 && uB <= 1) {
    const x = x1 + uA * (x2 - x1);
    const y = y1 + uA * (y2 - y1);
    return { point: { x, y }, u: uA, t: uB };
  }
  return undefined;
}
