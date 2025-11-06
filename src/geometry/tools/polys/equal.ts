import { equalLines } from '../../../index.js';

import type { MValue, Properties, VectorPolygon } from '../../../index.js';

/**
 * Check if two XY(Z) Polygons are equal
 * @param a - The first XY(Z) Polygon
 * @param b - The second XY(Z) Polygon
 * @returns - True if the two XY(Z) Polygons are equal
 */
export function equalPolys<D extends MValue = Properties>(
  a: VectorPolygon<D>,
  b: VectorPolygon<D>,
): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (!equalLines(a[i], b[i])) return false;
  }
  return true;
}
