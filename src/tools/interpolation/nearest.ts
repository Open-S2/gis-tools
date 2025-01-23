import { defaultGetInterpolateCurrentValue } from '.';
import { distance } from '../../geometry/s2/point';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, RGBA, VectorPoint } from '../..';

/**
 * # Nearest Neighbor Interpolation
 *
 * ## Description
 * Finds the nearest point in the reference data to the given point and returns its value.
 *
 * ## Usage
 * ```ts
 * import { nearestInterpolation, PointIndexFast } from 'gis-tools-ts';
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
 * const interpolatedValue = nearestInterpolation<TempData>(point, data, (p) => p.m.temp);
 * ```
 * @param point - Point to interpolate
 * @param refData - Reference data to search from
 * @param getValue - Function to get value from reference data
 * defaults to function that returns the z value or 0 if the z value is undefined
 * @returns - The value of the nearest point
 */
export function nearestInterpolation<T extends MValue = Properties>(
  point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T> = defaultGetInterpolateCurrentValue,
): number {
  if (refData.length === 0) return 0;

  // Find the nearest point
  let nearestPoint: VectorPoint<T> | undefined;
  let minDistance = Infinity;

  for (const refPoint of refData) {
    const dist = distance(point, refPoint);
    if (dist < minDistance || nearestPoint === undefined) {
      minDistance = dist;
      nearestPoint = refPoint;
    }
  }

  // Return the value of the nearest point
  if (nearestPoint !== undefined) return getValue(nearestPoint);
  return getValue(refData[0]);
}

/**
 * Helper function for {@link nearestInterpolation} on RGB(A) data.
 * Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
 * get the correct weight for each component.
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @returns - The interpolated RGBA data.
 */
export function rgbaNearestInterpolation(point: VectorPoint, refData: VectorPoint<RGBA>[]): RGBA {
  const { pow, sqrt } = Math;
  if (refData.length === 0) return { r: 0, g: 0, b: 0, a: 255 };
  const rData = nearestInterpolation(point, refData, (p) => pow(p.m?.r ?? 0, 2));
  const gData = nearestInterpolation(point, refData, (p) => pow(p.m?.g ?? 0, 2));
  const bData = nearestInterpolation(point, refData, (p) => pow(p.m?.b ?? 0, 2));
  const a = nearestInterpolation(point, refData, (p) => p.m?.a ?? 255);

  return {
    r: sqrt(rData),
    g: sqrt(gData),
    b: sqrt(bData),
    a,
  };
}
