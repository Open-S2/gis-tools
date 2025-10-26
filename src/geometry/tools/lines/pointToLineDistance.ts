import { llGetDistance, pointDistance, radToDeg } from '../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineString,
  VectorLineStringGeometry,
  VectorPoint,
} from '../../index.js';

/**
 * Check to see how far away the point is from the line
 * @param line - the line to check against
 * @param point - the point to check against the line
 * @param method - the method to use, either 'euclidean' or 'haversine'. Defaults to 'euclidean'
 * @returns The shortest distance between the point and a line. Returns -1 if line is empty
 */
export function pointToLineDistance<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  line:
    | VectorLineString<D>
    | VectorLineStringGeometry<D>
    | VectorFeature<M, D, P, VectorLineStringGeometry<D>>,
  point: VectorPoint,
  method: 'euclidean' | 'haversine' = 'euclidean',
): number {
  const haversine = method === 'haversine';
  const vectorLines: VectorLineString<D> =
    'geometry' in line
      ? line.geometry.coordinates
      : 'coordinates' in line
        ? line.coordinates
        : line;

  let closestIndex:
    | undefined
    | {
        index: number;
        dist: number;
      };
  for (let i = 0; i < vectorLines.length; i++) {
    // get the distance between the point and the line's point at index
    const dist = haversine
      ? radToDeg(llGetDistance(point, vectorLines[i]))
      : pointDistance(point, vectorLines[i]);
    if (dist === 0) {
      return 0;
    }
    if (closestIndex === undefined || dist < closestIndex.dist) {
      closestIndex = { index: i, dist };
    }
  }

  // If there is no closest point, return -1
  if (closestIndex === undefined) {
    return -1;
  }

  // If line is a single point, return distance to that point
  if (vectorLines.length === 1) {
    return closestIndex.dist;
  }

  const curr = vectorLines[closestIndex.index];
  const prev = vectorLines[closestIndex.index - 1];
  const next = vectorLines[closestIndex.index + 1];
  // If the point is the start or end of the line, return distance to that point and next/prev
  if (closestIndex.index === 0) {
    return distancePointToSegment(curr, next, point, method);
  }
  if (closestIndex.index === vectorLines.length - 1) {
    return distancePointToSegment(curr, prev, point, method);
  }

  // Check against both sides of the line's closest point
  const dist1 = distancePointToSegment(curr, prev, point, method);
  const dist2 = distancePointToSegment(curr, next, point, method);

  return dist1 < dist2 ? dist1 : dist2;
}

/**
 * Get the distance between a point and a segment
 * @param a - the segment start point
 * @param b - the segment end point
 * @param p - the point to check
 * @param method - the method to use, either 'euclidean' or 'haversine'. Defaults to 'euclidean'
 * @returns - the distance
 */
function distancePointToSegment(
  a: VectorPoint,
  b: VectorPoint,
  p: VectorPoint,
  method: 'euclidean' | 'haversine' = 'euclidean',
): number {
  const { max, min, hypot } = Math;
  if (method === 'haversine') {
    // approximate by sampling along the great-circle segment
    // project p onto AB using Euclidean math in lat/lon degrees
    // but compute distances with llGetDistance
    const abx = b.x - a.x;
    const aby = b.y - a.y;
    const apx = p.x - a.x;
    const apy = p.y - a.y;
    const abLenSq = abx * abx + aby * aby;

    if (abLenSq === 0) return radToDeg(llGetDistance(p, a));

    let t = (apx * abx + apy * aby) / abLenSq;
    t = max(0, min(1, t));

    const proj = { x: a.x + t * abx, y: a.y + t * aby };
    return radToDeg(llGetDistance(p, proj));
  }

  // Euclidean fallback
  const ab = { x: b.x - a.x, y: b.y - a.y };
  const ap = { x: p.x - a.x, y: p.y - a.y };
  const abLenSq = ab.x * ab.x + ab.y * ab.y;

  if (abLenSq === 0) return hypot(p.x - a.x, p.y - a.y);

  let t = (ap.x * ab.x + ap.y * ab.y) / abLenSq;
  t = max(0, min(1, t));

  const closest = { x: a.x + t * ab.x, y: a.y + t * ab.y };
  return hypot(p.x - closest.x, p.y - closest.y);
}
