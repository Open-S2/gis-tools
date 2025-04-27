import { extendBBox } from '../../bbox.js';
import { s2ToVector } from '../../index.js';

import type {
  BBOX,
  Coordinates,
  Feature,
  Geometry,
  GeometryType,
  LineString,
  LineString3D,
  LineStringMValues,
  MValue,
  MValues,
  MultiLineString,
  MultiLineString3D,
  MultiLineStringMValues,
  MultiPolygon,
  MultiPolygon3D,
  MultiPolygonMValues,
  Point,
  Point3D,
  Polygon3D,
  Properties,
  VectorFeatures,
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
} from '../../index.js';

/**
 * Convert a GeoJSON Vector Feature to a GeoJSON Feature
 * @param data - GeoJSON Vector Feature
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - GeoJson Feature
 */
export function toFlatGeometry<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
  G extends VectorGeometry<D> = VectorGeometry<D>,
>(data: VectorFeatures<M, D, P, G>, buildBBox?: boolean): Feature<M, D, P, Geometry<D>> {
  const { id, type, face, properties, metadata } = data;
  const vectorGeo = data.geometry;
  // adjust S2 to Vector if needed
  if (type === 'S2Feature') s2ToVector<D>(face ?? 0, data.geometry);
  const flatGeo = vectorToFlat<D>(vectorGeo, buildBBox);
  return {
    id,
    type: 'Feature',
    properties,
    metadata,
    geometry: flatGeo,
  };
}

/**
 * Convert a GeoJSON Vector Geometry to a Flat GeoJSON Geometry
 * @param geometry - GeoJSON Vector Geometry
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - GeoJSON Geometry
 */
export function vectorToFlat<M extends MValue = Properties>(
  geometry: VectorGeometry<M>,
  buildBBox?: boolean,
): Geometry<M> {
  const { type: vType, coordinates: coords, bbox, is3D } = geometry;
  const newBBox: BBOX | undefined =
    buildBBox === true && bbox === undefined ? ([] as unknown as BBOX) : undefined;

  let res: [Coordinates, MValue | MValues | undefined];
  if (vType === 'Point') res = convertPoint(coords, newBBox);
  else if (vType === 'MultiPoint' || vType === 'LineString')
    res = convertLineString(coords, newBBox);
  else if (vType === 'MultiLineString' || vType === 'Polygon')
    res = convertMultiLineString(coords, newBBox);
  else if (vType === 'MultiPolygon') res = convertMultiPolygon(coords, newBBox);
  else {
    throw new Error('Invalid Geometry type');
  }

  const [coordinates, mValues] = res;
  const type: GeometryType = is3D ? `${vType}3D` : vType;
  // @ts-expect-error - type complains but it works
  return { type, coordinates, bbox: newBBox ?? bbox, mValues };
}

/**
 * Convert a GeoJSON Vector MultiPolygon to a GeoJSON MultiPolygon
 * @param input - GeoJSON Vector MultiPolygon
 * @param bbox - if bbox is provided, we will extend the bbox
 * @returns - GeoJSON MultiPolygon
 */
function convertMultiPolygon(
  input: VectorMultiPolygon,
  bbox?: BBOX,
): [MultiPolygon | MultiPolygon3D, MultiPolygonMValues | undefined] {
  const polygons: MultiPolygon3D = [];
  const mValues: MultiPolygonMValues = [];
  let mValuesFound = false;
  for (const polygon of input) {
    const [poly, mValue] = convertMultiLineString(polygon, bbox);
    polygons.push(poly as unknown as Polygon3D);
    if (mValue !== undefined) mValuesFound = true;
    mValues.push(mValue ?? []);
  }
  return [polygons, mValuesFound ? mValues : undefined];
}

/**
 * Convert a GeoJSON Vector MultiLineString to a GeoJSON MultiLineString
 * @param input - GeoJSON Vector MultiLineString
 * @param bbox - if bbox is provided, we will extend the bbox
 * @returns - GeoJSON MultiLineString
 */
function convertMultiLineString(
  input: VectorMultiLineString,
  bbox?: BBOX,
): [MultiLineString | MultiLineString3D, MultiLineStringMValues | undefined] {
  const lines: MultiLineString3D = [];
  const mValues: MultiLineStringMValues = [];
  let mValuesFound = false;
  for (const line of input) {
    const [lineString, mValue] = convertLineString(line, bbox);
    lines.push(lineString as unknown as LineString3D);
    if (mValue !== undefined) mValuesFound = true;
    mValues.push(mValue ?? []);
  }
  return [lines, mValuesFound ? mValues : undefined];
}

/**
 * Convert a GeoJSON Vector LineString to a GeoJSON LineString
 * @param input - GeoJSON Vector LineString
 * @param bbox - if bbox is provided, we will extend the bbox
 * @returns - GeoJSON LineString
 */
function convertLineString(
  input: VectorLineString,
  bbox?: BBOX,
): [LineString | LineString3D, LineStringMValues | undefined] {
  const lines: LineString3D = [];
  const mValues: LineStringMValues = [];
  let mValuesFound = false;
  for (const vp of input) {
    const [point, mValue] = convertPoint(vp, bbox);
    lines.push(point as unknown as Point3D);
    if (mValue !== undefined) mValuesFound = true;
    mValues.push(mValue ?? {});
  }
  return [lines, mValuesFound ? mValues : undefined];
}

/**
 * Mutate a GeoJSON Vector Point to a GeoJSON Point or Point3D
 * @param point - GeoJSON Vector Point
 * @param bbox - if bbox is provided, we will extend the bbox
 * @returns - GeoJSON Point or Point3D
 */
function convertPoint(point: VectorPoint, bbox?: BBOX): [Point | Point3D, MValue | undefined] {
  if (bbox !== undefined) {
    const newBBox = extendBBox(bbox, point);
    for (let i = 0; i < newBBox.length; i++) bbox[i] = newBBox[i];
  }
  if (point.z === undefined) return [[point.x, point.y], point.m];
  return [[point.x, point.y, point.z], point.m];
}
