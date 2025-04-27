import { extendBBox } from '../../bbox.js';

import type {
  BBOX,
  Feature,
  Geometry,
  MValue,
  Point,
  Point3D,
  Properties,
  VectorCoordinates,
  VectorFeature,
  VectorGeometry,
  VectorPoint,
} from '../../index.js';

/**
 * Convert a GeoJSON Feature to a GeoJSON Vector Feature
 * @param data - GeoJSON Feature
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - GeoJson Vector Feature
 */
export function toVector<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
  G extends Geometry<D> = Geometry<D>,
>(data: Feature<M, D, P, G>, buildBBox?: boolean): VectorFeature<M, D, P, VectorGeometry<D>> {
  const { id, properties, metadata } = data;
  const vectorGeo = geoToVector<D>(data.geometry, buildBBox);
  return {
    id,
    type: 'VectorFeature',
    properties,
    metadata,
    geometry: vectorGeo,
  };
}

/**
 * Convert a GeoJSON Geometry to an Vector Geometry
 * @param geometry - GeoJSON Geometry
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - GeoJson Vector Geometry
 */
export function geoToVector<M extends MValue = Properties>(
  geometry: Geometry<M>,
  buildBBox?: boolean,
): VectorGeometry<M> {
  const { type, coordinates: coords, mValues, bbox } = geometry;
  const newBBox: BBOX | undefined =
    buildBBox !== false && bbox === undefined ? ([] as unknown as BBOX) : undefined;

  let coordinates: VectorCoordinates;
  if (type === 'Point' || type === 'Point3D') coordinates = convertPoint(coords, mValues, newBBox);
  else if (type === 'MultiPoint' || type === 'MultiPoint3D')
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i], newBBox));
  else if (type === 'LineString' || type === 'LineString3D')
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i], newBBox));
  else if (type === 'MultiLineString' || type === 'MultiLineString3D')
    coordinates = coords.map((line, i) =>
      line.map((point, j) => convertPoint(point, mValues?.[i]?.[j], newBBox)),
    );
  else if (type === 'Polygon' || type === 'Polygon3D')
    coordinates = coords.map((line, i) =>
      line.map((point, j) => convertPoint(point, mValues?.[i]?.[j], newBBox)),
    );
  else if (type === 'MultiPolygon' || type === 'MultiPolygon3D')
    coordinates = coords.map((polygon, i) =>
      polygon.map((line, j) =>
        line.map((point, k) => convertPoint(point, mValues?.[i]?.[j]?.[k], newBBox)),
      ),
    );
  else {
    throw new Error('Invalid GeoJSON type');
  }
  const is3D = type.slice(-2) === '3D';
  // @ts-expect-error - coordinates complains, but the way this is all written is simpler
  return { type: type.replace('3D', ''), is3D, coordinates, bbox: newBBox ?? bbox };
}

/**
 * Mutate a GeoJSON Point to a GeoJSON Vector Point
 * @param point - GeoJSON flat Point
 * @param m - optional m-value
 * @param bbox - if bbox is provided, we will extend the bbox
 * @returns - GeoJSON Vector Point
 */
function convertPoint(point: Point | Point3D, m?: MValue, bbox?: BBOX): VectorPoint {
  const newPoint: VectorPoint = { x: point[0], y: point[1], z: point[2], m };
  if (bbox !== undefined) {
    const newBBox = extendBBox(bbox, newPoint);
    for (let i = 0; i < newBBox.length; i++) bbox[i] = newBBox[i];
  }
  return newPoint;
}
