import { pointDistance as distance } from '../../geometry/s2/point';
import { averageInterpolation, defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, RGBA, VectorPoint } from '../..';

/**
 * # Inverse Distance Weighting Interpolation
 *
 * ## Description
 * Given a reference of data, interpolate a point using inverse distance weighting
 *
 * ## Usage
 * ```ts
 * import { idwInterpolation, PointIndexFast } from 'gis-tools-ts';
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
 * const interpolatedValue = idwInterpolation<TempData>(point, data, (p) => p.m.temp);
 * ```
 * @param point - point to interpolate
 * @param refData - reference data to interpolate from
 * @param getValue - function to get value from reference data. Can be the z value or a property in the m-values
 * defaults to function that returns the z value or 0 if the z value is undefined
 * @returns - the interpolated value
 */
export function idwInterpolation<T extends MValue = Properties>(
  point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T> = defaultGetInterpolateCurrentValue,
): number {
  if (refData.length === 0) return 0;
  let numerator = 0;
  let denom = 0;
  for (const refPoint of refData) {
    const d2 = Math.pow(distance(point, refPoint), 2);
    const value = getValue(refPoint);
    if (d2 === 0) return value; // if distance is 0, return value
    numerator += value / d2;
    denom += 1 / d2;
  }
  return numerator / denom;
}

/**
 * Helper function for {@link idwInterpolation} on RGB(A) data.
 * Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
 * get the correct weight for each component.
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @returns - The interpolated RGBA data.
 */
export function rgbaIDWInterpolation(point: VectorPoint, refData: VectorPoint<RGBA>[]): RGBA {
  const { pow, sqrt } = Math;
  if (refData.length === 0) return { r: 0, g: 0, b: 0, a: 255 };
  const rData = idwInterpolation(point, refData, (p) => pow(p.m?.r ?? 0, 2));
  const gData = idwInterpolation(point, refData, (p) => pow(p.m?.g ?? 0, 2));
  const bData = idwInterpolation(point, refData, (p) => pow(p.m?.b ?? 0, 2));
  const a = averageInterpolation(point, refData, (p) => p.m?.a ?? 255);

  return {
    r: sqrt(rData),
    g: sqrt(gData),
    b: sqrt(bData),
    a,
  };
}
