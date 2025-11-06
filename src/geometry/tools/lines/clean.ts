import { clampWGS84Point, equalPoints } from '../../index.js';

import type {
  MValue,
  Properties,
  VectorFeatures,
  VectorLineString,
  VectorLineStringGeometry,
  VectorMultiLineString,
  VectorMultiLineStringGeometry,
} from '../../../index.js';

/**
 * Removes duplicates and superfluous/collinear points from a collection of linestrings
 * @param linestrings - the linestring to clean
 * @param isPoly - true if the linestring is from a polygon ring
 * @param eps - the tolerance. Defaults to `1e-12`
 * @param cleanWGS84 - if true, clean WGS84 points to be in bounds
 * @returns New cleaned lines (points are duplicated)
 */
export function cleanMultiLineString<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  linestrings:
    | VectorMultiLineString<D>
    | VectorMultiLineStringGeometry<D>
    | VectorFeatures<M, D, P, VectorMultiLineStringGeometry<D>>,
  isPoly: boolean = false,
  eps: number = 1e-12,
  cleanWGS84 = false,
): VectorMultiLineString<D> | undefined {
  const lines: VectorMultiLineString<D> =
    'geometry' in linestrings
      ? linestrings.geometry.coordinates
      : 'coordinates' in linestrings
        ? linestrings.coordinates
        : linestrings;
  const res = lines
    .map((line) => cleanLineString(line, isPoly, eps, cleanWGS84))
    .filter((line) => line !== undefined) as VectorMultiLineString<D>;

  return res.length > 0 ? res : undefined;
}

/**
 * Removes duplicates and superfluous/collinear points from a linestring
 * @param linestring - the linestring to clean
 * @param isPoly - true if the linestring is from a polygon ring
 * @param eps - the toleranc to check if the segments are superfluous/collinear. Defaults to `1e-12`
 * @param cleanWGS84 - if true, clean WGS84 points to be in bounds
 * @returns A new cleaned lines (points are duplicated)
 */
export function cleanLineString<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  linestring:
    | VectorLineString<D>
    | VectorLineStringGeometry<D>
    | VectorFeatures<M, D, P, VectorLineStringGeometry<D>>,
  isPoly: boolean = false,
  eps: number = 1e-12,
  cleanWGS84 = false,
): VectorLineString<D> | undefined {
  const line: VectorLineString<D> =
    'geometry' in linestring
      ? linestring.geometry.coordinates
      : 'coordinates' in linestring
        ? linestring.coordinates
        : linestring;
  if (isPoly ? line.length < 4 : line.length < 2) return;

  // first remove all duplicates
  const noDups: VectorLineString<D> = [];
  for (let i = 0; i < line.length; i++) {
    if (i > 0 && equalPoints(line[i], line[i - 1])) continue;
    const p = { ...line[i] };
    if (cleanWGS84) clampWGS84Point(p);
    noDups.push(p);
  }

  // then remove superfluous/collinear points
  const cleaned: VectorLineString<D> = [noDups[0]];
  for (let i = 1, end = noDups.length - 1; i < end; i++) {
    const prev = noDups[i - 1];
    const curr = noDups[i];
    const next = noDups[i + 1];
    const area = (curr.y - prev.y) * (next.x - curr.x) - (curr.x - prev.x) * (next.y - curr.y);
    if (Math.abs(area) > eps) cleaned.push(curr);
  }
  cleaned.push(noDups[noDups.length - 1]);
  if (isPoly ? cleaned.length < 4 : cleaned.length < 2) return;
  return cleaned;
}
