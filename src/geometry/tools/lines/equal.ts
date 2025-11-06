import { equalPoints } from '../../../index.js';

import type { MValue, Properties, VectorLineString } from '../../../index.js';

/**
 * Check if two XY(Z) LineStrings are equal
 * @param a - The first XY(Z) LineString
 * @param b - The second XY(Z) LineString
 * @returns - True if the two XY(Z) LineStrings are equal
 */
export function equalLines<D extends MValue = Properties>(
  a: VectorLineString<D>,
  b: VectorLineString<D>,
): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (!equalPoints(a[i], b[i])) return false;
  }
  return true;
}
