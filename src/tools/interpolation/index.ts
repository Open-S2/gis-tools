import type { VectorPoint } from '../..';

export * from './bilinear';
export * from './idw';
export * from './kriging';

/** Function to get the value of a point */
export type GetInterpolateValue = (point: VectorPoint) => number;

/**
 * Default function to get the value of a point
 * @param point - vector point to pull data from
 * @returns - the z value
 */
export const defaultGetInterpolateCurrentValue: GetInterpolateValue = (
  point: VectorPoint,
): number => point.z ?? 0;
