import { orient2dVector } from '../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineString,
  VectorLineStringGeometry,
  VectorPoint,
} from '../../index.js';

/**
 * Check to see if a point is on a line. Uses predicates to ensure the point is truly on the line
 * @param line - the line to check against
 * @param point - the point to check if it is on the line
 * @param epsilon - the buffer to use to check if the point is on the line within epsilon. Defaults to 0
 * @returns True if the point is on the line
 */
export function pointOnLine<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  line:
    | VectorLineString<D>
    | VectorLineStringGeometry<D>
    | VectorFeature<M, D, P, VectorLineStringGeometry<D>>,
  point: VectorPoint,
  epsilon: number = 0,
): boolean {
  const { min, max, abs } = Math;
  const vectorLines: VectorLineString<D> =
    'geometry' in line
      ? line.geometry.coordinates
      : 'coordinates' in line
        ? line.coordinates
        : line;

  if (vectorLines.length < 2) return false;

  for (let i = 0; i < vectorLines.length - 1; i++) {
    // check if in bounding box of each segment
    const a = vectorLines[i];
    const b = vectorLines[i + 1];
    if (
      point.x >= min(a.x, b.x) &&
      point.x <= max(a.x, b.x) &&
      point.y >= min(a.y, b.y) &&
      point.y <= max(a.y, b.y)
    ) {
      // lastly check if the point is on the segment
      const cross = orient2dVector(a, b, point);
      if (abs(cross) <= epsilon) return true;
    }
  }

  return false;
}
