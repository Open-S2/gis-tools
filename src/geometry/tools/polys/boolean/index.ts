import { booleanOp } from './operation.js';

import type { MultiPolygon, Polygon } from '../../../index.js';

/**
 * Union of multiple geometries
 * @param geoms - input geometries, the first one is the "subject" to modify
 * @returns - a MultiPoly of the result
 */
export function polyUnion(...geoms: (MultiPolygon | Polygon)[]): MultiPolygon {
  return booleanOp('union', undefined, ...geoms);
}

/**
 * Intersection of multiple geometries
 * @param geoms - input geometries, the first one is the "subject" to modify
 * @returns - a MultiPoly of the result
 */
export function polyIntersection(...geoms: (MultiPolygon | Polygon)[]): MultiPolygon {
  return booleanOp('intersection', undefined, ...geoms);
}

/**
 * XOR of multiple geometries
 * @param geoms - input geometries, the first one is the "subject" to modify
 * @returns - a MultiPoly of the result
 */
export function polyXor(...geoms: (MultiPolygon | Polygon)[]): MultiPolygon {
  return booleanOp('xor', undefined, ...geoms);
}

/**
 * Difference of multiple geometries
 * @param geoms - input geometries, the first one is the "subject" to modify
 * @returns - a MultiPoly of the result
 */
export function polyDifference(...geoms: (MultiPolygon | Polygon)[]): MultiPolygon {
  return booleanOp('difference', undefined, ...geoms);
}
