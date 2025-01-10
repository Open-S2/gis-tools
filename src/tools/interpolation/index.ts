import type { MValue, Properties, VectorPoint } from '../..';

export * from './bilinear';
export * from './idw';
export * from './kriging';

/** Function to get the value of a point */
export type GetInterpolateValue<T extends MValue = Properties> = (point: VectorPoint<T>) => number;

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
