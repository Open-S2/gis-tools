import { averageInterpolation, defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, RGBA, VectorPoint } from '../..';

/** The 4 corner points closest to a point */
export type BilinearCorners<T extends MValue = Properties> = [
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
export function getBilinearPoints<T extends MValue = Properties>(
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

  // Check if any of the corners are undefined
  if (
    topLeft === undefined ||
    topRight === undefined ||
    bottomLeft === undefined ||
    bottomRight === undefined
  ) {
    throw new Error('Insufficient data to determine all four bilinear corner points.');
  }

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
 * import { bilinearInterpolation, getBilinearPoints, PointIndexFast } from 'gis-tools-ts';
 * import type { VectorPoint } from 'gis-tools-ts';
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
 * const interpolatedValue = bilinearInterpolation<TempData>(point, data, (p) => p.m.temp);
 *
 * // if you reuse the same data, you can pass in the corners for performance gains
 * const corners = getBilinearPoints(point, data);
 * const interpolatedValue = bilinearInterpolation<TempData>(point, data, (p) => p.m.temp, corners);
 * ```
 * @param point - point to interpolate
 * @param refData - reference data to interpolate from
 * @param getValue - function to get value from reference data. Can be the z value or a property in the m-values
 * defaults to function that returns the z value or 0 if the z value is undefined
 * @param corners - the 4 corner points relative to the reference point. Add this if you don't want to keep
 * recomputing the corners on the same data.
 * @returns - the interpolated value
 */
export function bilinearInterpolation<T extends MValue = Properties>(
  point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T> = defaultGetInterpolateCurrentValue,
  corners?: BilinearCorners<T>,
): number {
  if (refData.length === 0) return 0;
  // 1) Extract corner points and values
  const [tl, tr, bl, br] = corners ?? getBilinearPoints(point, refData);
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

/**
 * Helper function for {@link bilinearInterpolation} on RGB(A) data.
 * Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
 * get the correct weight for each component.
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @returns - The interpolated RGBA data.
 */
export function rgbaBilinearInterpolation(point: VectorPoint, refData: VectorPoint<RGBA>[]): RGBA {
  const { pow, sqrt } = Math;
  if (refData.length === 0) return { r: 0, g: 0, b: 0, a: 255 };
  const corners = getBilinearPoints(point, refData);
  const rData = bilinearInterpolation(point, refData, (p) => pow(p.m?.r ?? 0, 2), corners);
  const gData = bilinearInterpolation(point, refData, (p) => pow(p.m?.g ?? 0, 2), corners);
  const bData = bilinearInterpolation(point, refData, (p) => pow(p.m?.b ?? 0, 2), corners);
  const a = averageInterpolation(point, refData, (p) => p.m?.a ?? 255);

  return {
    r: sqrt(rData),
    g: sqrt(gData),
    b: sqrt(bData),
    a,
  };
}
