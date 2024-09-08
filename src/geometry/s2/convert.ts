import { fromST, toLonLat } from './point';

import type { Face, S2Feature, VectorFeature, VectorGeometry, VectorPoint } from '../';

/**
 * Convert an S2Feature to a GeoJSON Feature
 * @param data - S2Feature
 * @returns - GeoJSON Feature
 */
export function toWM(data: S2Feature): VectorFeature {
  const { id, face, properties, metadata, geometry } = data;
  convertGeometry(face, geometry);
  return {
    id,
    type: 'VectorFeature',
    properties,
    metadata,
    geometry,
  };
}

/**
 * Underlying conversion mechanic to move S2Geometry to GeoJSON Geometry
 * @param face - Face
 * @param geometry - S2 Geometry
 */
function convertGeometry(face: Face, geometry: VectorGeometry): void {
  const { type, coordinates } = geometry;
  if (type === 'Point') convertGeometryPoint(face, coordinates);
  else if (type === 'MultiPoint') coordinates.forEach((point) => convertGeometryPoint(face, point));
  else if (type === 'LineString') coordinates.forEach((point) => convertGeometryPoint(face, point));
  else if (type === 'MultiLineString')
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint(face, point)));
  else if (type === 'Polygon')
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint(face, point)));
  else if (type === 'MultiPolygon')
    coordinates.forEach((polygon) =>
      polygon.forEach((line) => line.forEach((point) => convertGeometryPoint(face, point))),
    );
  else {
    throw new Error('Invalid S2Geometry type');
  }
}

/**
 * Mutate an S2 Point to a GeoJSON Point
 * @param face - Face
 * @param point - S2 Point
 */
function convertGeometryPoint(face: Face, point: VectorPoint): void {
  const { x: s, y: t } = point;
  const [lon, lat] = toLonLat(fromST(face, s, t));
  point.x = lon;
  point.y = lat;
}
