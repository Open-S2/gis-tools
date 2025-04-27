import { extendBBox } from '../../bbox.js';
import { radToDeg } from '../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorGeometry,
  VectorPoint,
} from '../../index.js';

/**
 * Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
 * @param feature - input GeoJSON
 */
export function toUnitScale<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(feature: VectorFeature<M, D, P>): void {
  const { geometry } = feature;
  const { type, coordinates } = geometry;
  if (type === 'Point') projectPoint(coordinates, geometry);
  else if (type === 'MultiPoint') coordinates.map((p) => projectPoint(p, geometry));
  else if (type === 'LineString') coordinates.map((p) => projectPoint(p, geometry));
  else if (type === 'MultiLineString')
    coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === 'Polygon') coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === 'MultiPolygon')
    coordinates.map((p) => p.map((l) => l.map((p) => projectPoint(p, geometry))));
  else {
    throw new Error('Either the conversion is not yet supported or Invalid VectorGeometry type.');
  }
}

/**
 * Reproject GeoJSON vector geometry coordinates from 0->1 coordinate system to lon-lat in place
 * @param feature - input GeoJSON
 */
export function toLL<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(feature: VectorFeature<M, D, P>): void {
  const { type, coordinates } = feature.geometry;
  if (type === 'Point') unprojectPoint(coordinates);
  else if (type === 'MultiPoint') coordinates.map((p) => unprojectPoint(p));
  else if (type === 'LineString') coordinates.map((p) => unprojectPoint(p));
  else if (type === 'MultiLineString') coordinates.map((l) => l.map((p) => unprojectPoint(p)));
  else if (type === 'Polygon') coordinates.map((l) => l.map((p) => unprojectPoint(p)));
  else if (type === 'MultiPolygon')
    coordinates.map((p) => p.map((l) => l.map((p) => unprojectPoint(p))));
  else {
    throw new Error('Either the conversion is not yet supported or Invalid S2Geometry type.');
  }
}

/**
 * Project a point from lon-lat to a 0->1 coordinate system in place
 * @param input - input point
 * @param geo - input geometry (used to update the bbox)
 */
function projectPoint<M extends MValue = Properties>(
  input: VectorPoint<M>,
  geo: VectorGeometry<M>,
): void {
  const { x, y } = input;
  input.x = projectX(x);
  input.y = projectY(y);
  // update bbox
  geo.vecBBox = extendBBox(geo.vecBBox, input);
}

/**
 * Project a point from 0->1 coordinate space to lon-lat in place
 * @param input - input vector to mutate
 */
function unprojectPoint<M extends MValue = Properties>(input: VectorPoint<M>): void {
  const { x, y } = input;

  // Revert the x coordinate
  const lon = (x - 0.5) * 360;
  // Revert the y coordinate
  const y2 = 0.5 - y;
  const lat = radToDeg(Math.atan(Math.sinh(Math.PI * (y2 * 2))));

  input.x = lon;
  input.y = lat;
}

/**
 * Convert a longitude to a 0->1 coordinate
 * @param x - longitude
 * @returns a 0->1 coordinate
 */
export function projectX(x: number): number {
  return x / 360 + 0.5;
}

/**
 * Convert a latitude to a 0->1 coordinate
 * @param y - latitude
 * @returns a 0->1 coordinate
 */
export function projectY(y: number): number {
  const sin = Math.sin((y * Math.PI) / 180);
  const y2 = 0.5 - (0.25 * Math.log((1 + sin) / (1 - sin))) / Math.PI;
  return y2 < 0 ? 0 : y2 > 1 ? 1 : y2;
}
