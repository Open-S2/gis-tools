import type {
  MValue,
  Properties,
  VectorFeature,
  VectorGeometry,
  VectorMultiLineString,
  VectorMultiLineStringGeometry,
} from '../../index.js';

export * from './along.js';
export * from './area.js';
export * from './clean.js';
export * from './intersection.js';
export * from './length.js';
export * from './pointOnLine.js';
export * from './pointToLineDistance.js';

/**
 * Given an input vector feature, create a collection of lines
 * @param data - vector feature with various geometry types
 * @returns - all features as a collection of points
 */
export function toLines<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(data: VectorFeature<M, D, P, VectorGeometry<D>>): VectorMultiLineStringGeometry<D> | undefined {
  const { type, is3D, coordinates } = data.geometry;
  const res: VectorMultiLineString<D> = [];

  if (type === 'Point' || type === 'MultiPoint') {
    return;
  } else if (type === 'LineString') {
    res.push(coordinates);
  } else if (type === 'MultiLineString') {
    res.push(...coordinates);
  } else if (type === 'Polygon') {
    res.push(...coordinates);
  } else if (type === 'MultiPolygon') {
    res.push(...coordinates.flat());
  }

  return {
    type: 'MultiLineString',
    is3D,
    coordinates: res,
  };
}
