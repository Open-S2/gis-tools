import {
  averageInterpolation,
  idwInterpolation,
  lanczosInterpolation,
  nearestInterpolation,
  rgbaAverageInterpolation,
  rgbaIDWInterpolation,
  rgbaLanczosInterpolation,
  rgbaNearestInterpolation,
} from '.';

import type { MValue, Properties, RGBA, VectorPoint } from '../..';

export * from './average';
export * from './bilinear';
export * from './idw';
export * from './kriging';
export * from './lanczos';
export * from './nearest';

/** Interpolation method */
export type InterpolationMethod = 'average' | 'nearest' | 'idw' | 'lanczos';

/**
 * Get the interpolation function based on the method type.
 * Options are:
 * - average
 * - nearest
 * - idw
 * - lanczos [default]
 * @param method - interpolation method as a string
 * @returns - interpolation function
 */
export function getInterpolation<T extends MValue = Properties>(
  method: InterpolationMethod,
): InterpolationFunction<T> {
  switch (method) {
    case 'average':
      return averageInterpolation;
    case 'nearest':
      return nearestInterpolation;
    case 'idw':
      return idwInterpolation;
    case 'lanczos':
    default:
      return lanczosInterpolation;
  }
}

/**
 * Get the interpolation function based on the method type.
 * Options are:
 * - average
 * - nearest
 * - idw
 * - lanczos [default]
 * @param method - interpolation method as a string
 * @returns - interpolation function
 */
export function getRGBAInterpolation(method: InterpolationMethod): RGBAInterpolationFunction {
  switch (method) {
    case 'average':
      return rgbaAverageInterpolation;
    case 'nearest':
      return rgbaNearestInterpolation;
    case 'idw':
      return rgbaIDWInterpolation;
    case 'lanczos':
    default:
      return rgbaLanczosInterpolation;
  }
}

/** Function to get the value of a point */
export type GetInterpolateValue<T extends MValue = Properties> = (point: VectorPoint<T>) => number;

/** The standard interpolation function */
export type InterpolationFunction<T extends MValue = Properties> = (
  point: VectorPoint,
  refData: VectorPoint<T>[],
  getValue: GetInterpolateValue<T>,
) => number;

/** The standard RGBA interpolation function */
export type RGBAInterpolationFunction = (point: VectorPoint, refData: VectorPoint<RGBA>[]) => RGBA;

/**
 * Default function to get the value of a point
 * @param point - vector point to pull data from
 * @returns - the z value
 */
export function defaultGetInterpolateCurrentValue<T extends Properties>(
  point: VectorPoint<T>,
): number {
  return point.z ?? 0;
}
