import { pointDistance as distance } from '../../geometry/s2/point';
import { averageInterpolation, defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { MValue, Properties, RGBA, VectorPoint } from '../..';

/**
 * # Lanczos Interpolation
 *
 * ## Description
 * Perform interpolation using the Lanczos filter. This method uses a kernel-based approach
 * to weigh contributions from nearby points, providing a balance between smoothing and sharpness.
 *
 * ## Usage
 * ```ts
 * import { lanczosInterpolation, PointIndexFast } from 'gis-tools-ts';
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
 * const interpolatedValue = lanczosInterpolation<TempData>(point, data, (p) => p.m.temp);
 * ```
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @param getValue - Function to extract the value from a reference point
 * @param kernelRadius - Lanczos kernel radius (default is 2) Recommend to only use 2 or 3.
 * @returns - The interpolated value
 */
export function lanczosInterpolation<T extends MValue = Properties>(
  point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T> = defaultGetInterpolateCurrentValue,
  kernelRadius: number = 2,
): number {
  if (refData.length === 0) throw new Error('Reference data cannot be empty.');
  let numerator = 0;
  let denom = 0;

  for (const refPoint of refData) {
    const weight = lanczosKernel(distance(point, refPoint), kernelRadius);
    const value = getValue(refPoint);

    numerator += value * weight;
    denom += weight;
  }

  // Avoid division by zero
  if (denom === 0) return 0;
  return numerator / denom;
}

/**
 * Helper function for {@link lanczosInterpolation} on RGB(A) data.
 * Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
 * get the correct weight for each component.
 * @param point - Point to interpolate
 * @param refData - Reference data points
 * @param kernelRadius - Lanczos kernel radius (default is 2) Recommend to only use 2 or 3.
 * @returns - The interpolated RGBA data.
 */
export function rgbaLanczosInterpolation(
  point: VectorPoint,
  refData: VectorPoint<RGBA>[],
  kernelRadius: number = 2,
): RGBA {
  const { pow, sqrt } = Math;
  if (refData.length === 0) return { r: 0, g: 0, b: 0, a: 255 };
  const rData = lanczosInterpolation(point, refData, (p) => pow(p.m?.r ?? 0, 2), kernelRadius);
  const gData = lanczosInterpolation(point, refData, (p) => pow(p.m?.g ?? 0, 2), kernelRadius);
  const bData = lanczosInterpolation(point, refData, (p) => pow(p.m?.b ?? 0, 2), kernelRadius);
  const a = averageInterpolation(point, refData, (p) => p.m?.a ?? 255);

  return {
    r: sqrt(rData),
    g: sqrt(gData),
    b: sqrt(bData),
    a,
  };
}

/**
 * Lanczos kernel function - returns the weight based on the distance from the target point
 * https://en.wikipedia.org/wiki/Lanczos_resampling
 * @param x - distance from the target point
 * @param a - Lanczos kernel radius (default 2)
 * @returns - weight based on the Lanczos kernel
 */
function lanczosKernel(x: number, a: number): number {
  if (x === 0) return 1; // sinc(0) = 1
  if (Math.abs(x) >= a) return 0; // Outside the kernel radius
  const piX = Math.PI * x;
  return (Math.sin(piX) / piX) * (Math.sin(piX / a) / (piX / a));
}
