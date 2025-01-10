import { defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, VectorPoint } from '../..';

/**
 * # Inverse Distance Weighting Interpolation
 *
 * ## Description
 * Given a reference of data, interpolate a point using inverse distance weighting
 *
 * ## Usage
 * ```ts
 * import { idwInterpolation, PointIndexFast } from 'gis-tools';
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
  let numerator = 0;
  let denom = 0;
  for (const refPoint of refData) {
    const distance = Math.sqrt(
      Math.pow(refPoint.x - point.x, 2) + Math.pow(refPoint.y - point.y, 2),
    );
    const d2 = Math.pow(distance, 2);
    const value = getValue(refPoint);
    if (d2 === 0) return value; // if distance is 0, return value
    numerator += value / d2;
    denom += 1 / d2;
  }
  return numerator / denom;
}
