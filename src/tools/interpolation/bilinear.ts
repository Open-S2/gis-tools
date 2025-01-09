import { defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { Properties, VectorPoint } from '../..';

/** The 4 corner points closest to a point */
export type BilinearCorners<T extends Properties = Properties> = [
  VectorPoint<T>,
  VectorPoint<T>,
  VectorPoint<T>,
  VectorPoint<T>,
];

/**
 * Sometimes you're given a large swathe of points, and so this function helps find the closest 4
 * "corners" relative to a pont
 * @param point - point to interpolate
 * @param refData - reference data to interpolate from
 * @returns - the corner points closest to the reference point
 */
export function getBilinearPoints<T extends Properties = Properties>(
  point: VectorPoint,
  refData: VectorPoint<T>[],
): BilinearCorners<T> {
  const { x: refX, y: refY } = point;
  /**
   * Returns the distance between any point and the reference point
   * @param point - point to get distance from
   * @returns - the distance
   */
  const distance = (point: VectorPoint): number =>
    Math.sqrt(Math.pow(point.x - refX, 2) + Math.pow(point.y - refY, 2));
  // 1) reduce to four corner points
  const topLeft = refData
    .filter((p) => p.x <= refX && p.y > refY)
    .reduce((a, b) => (distance(a) < distance(b) ? a : b));
  const topRight = refData
    .filter((p) => p.x > refX && p.y > refY)
    .reduce((a, b) => (distance(a) < distance(b) ? a : b));
  const bottomLeft = refData
    .filter((p) => p.x <= refX && p.y <= refY)
    .reduce((a, b) => (distance(a) < distance(b) ? a : b));
  const bottomRight = refData
    .filter((p) => p.x > refX && p.y <= refY)
    .reduce((a, b) => (distance(a) < distance(b) ? a : b));

  return [topLeft, topRight, bottomLeft, bottomRight];
}

/**
 * # Bilinear Interpolation
 *
 * ## Description
 * Given a reference of data, interpolate a point using bilinear interpolation
 *
 * ## Usage
 * ```ts
 * import { bilinearInterpolation, getBilinearPoints, PointIndexFast } from 'gis-tools';
 * import type { VectorPoint } from 'gis-tools';
 *
 * // We have m-value data that we want to interpolate
 * interface TempData { temp: number; }
 *
 * const pointIndex = new PointIndexFast<TempData>();
 * // add lots of points
 * pointIndex.insertLonLat(lon, lat, data);
 * // ....
 *
 * // given a point we are interested in
 * const point: VectorPoint = { x: 20, y: -40 };
 * //  get a collection of points relative to the point
 * const data = await pointIndex.searchRadius(point.x, point.y, radius);
 *
 * // interpolate
 * const corners = getBilinearPoints(point, data);
 * const interpolatedValue = bilinearInterpolation<TempData>(point, corners, (p) => p.m.temp);
 * ```
 * @param point - point to interpolate
 * @param corners - the 4 corner points relative to the reference point
 * @param getValue - function to get value from reference data. Can be the z value or a property in the m-values
 * defaults to function that returns the z value or 0 if the z value is undefined
 * @returns - the interpolated value
 */
export function bilinearInterpolation<T extends Properties = Properties>(
  point: VectorPoint,
  corners: BilinearCorners<T>,
  getValue: GetInterpolateValue = defaultGetInterpolateCurrentValue,
): number {
  // 1) Extract corner points and values
  const [tl, tr, bl, br] = corners;
  const { x: px, y: py } = point;
  const { x: x1, y: y1 } = tl;
  const { x: x2, y: y2 } = br;
  const q11 = getValue(tl);
  const q21 = getValue(tr);
  const q12 = getValue(bl);
  const q22 = getValue(br);

  // 2) Calculate the weights for bilinear interpolation
  const weight11 = ((x2 - px) * (y2 - py)) / ((x2 - x1) * (y2 - y1));
  const weight21 = ((px - x1) * (y2 - py)) / ((x2 - x1) * (y2 - y1));
  const weight12 = ((x2 - px) * (py - y1)) / ((x2 - x1) * (y2 - y1));
  const weight22 = ((px - x1) * (py - y1)) / ((x2 - x1) * (y2 - y1));

  // 3) Compute the weighted average of the values
  const interpolatedValue = weight11 * q11 + weight21 * q21 + weight12 * q12 + weight22 * q22;

  return interpolatedValue;
}
