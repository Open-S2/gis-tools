import { defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, RGBA, VectorPoint } from '../..';

/**
 * # Average Neighbor Interpolation
 *
 * ## Description
 * Finds the avarage point in the reference data to the given point and returns its value.
 *
 * ## Usage
 * ```ts
 * import { averageInterpolation, PointIndexFast } from 'gis-tools-ts';
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
 * const interpolatedValue = averageInterpolation<TempData>(point, data, (p) => p.m.temp);
 * ```
 * @param _point - Point to interpolate around. Unused for this method
 * @param refData - Reference data to search from
 * @param getValue - Function to get value from reference data
 * defaults to function that returns the z value or 0 if the z value is undefined
 * @returns - The avarage value of the collection of points
 */
export function averageInterpolation<T extends MValue = Properties>(
  _point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T> = defaultGetInterpolateCurrentValue,
): number {
  let total = 0;
  for (const refPoint of refData) total += getValue(refPoint);

  return total / refData.length;
}

/**
 * Helper function for {@link averageInterpolation} on RGB(A) data.
 * Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
 * get the correct weight for each component.
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @returns - The interpolated RGBA data.
 */
export function rgbaAverageInterpolation(point: VectorPoint, refData: VectorPoint<RGBA>[]): RGBA {
  const { pow, sqrt } = Math;
  if (refData.length === 0) return { r: 0, g: 0, b: 0, a: 255 };
  const rData = averageInterpolation(point, refData, (p) => pow(p.m?.r ?? 0, 2));
  const gData = averageInterpolation(point, refData, (p) => pow(p.m?.g ?? 0, 2));
  const bData = averageInterpolation(point, refData, (p) => pow(p.m?.b ?? 0, 2));
  const a = averageInterpolation(point, refData, (p) => p.m?.a ?? 255);

  return {
    r: sqrt(rData),
    g: sqrt(gData),
    b: sqrt(bData),
    a,
  };
}
