import { clipLine } from '../../tools/clip.js';
import { pointFromLonLat as fromLonLat, pointToST as toST } from '../../s2/point.js';
import { fromPoint, mergeBBoxes } from '../../bbox.js';

import { geoToVector } from './vector.js';

import type {
  BBOX,
  Face,
  Feature,
  Geometry,
  MValue,
  Properties,
  S2Feature,
  STPoint,
  VectorFeature,
  VectorGeometry,
  VectorLineString,
  VectorLineStringGeometry,
  VectorMultiLineStringGeometry,
  VectorMultiPointGeometry,
  VectorMultiPolygonGeometry,
  VectorPoint,
  VectorPointGeometry,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../../index.js';

/**
 * Convet a GeoJSON Feature to an S2Feature
 * @param data - GeoJSON Feature
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - S2Feature
 */
export function toS2<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  data: Feature<M, D, P, Geometry<D>> | VectorFeature<M, D, P, VectorGeometry<D>>,
  buildBBox?: boolean,
): S2Feature<M, D, P, VectorGeometry<D>>[] {
  const { id, properties, metadata } = data;
  const res: S2Feature<M, D, P>[] = [];
  const vectorGeo =
    data.type === 'VectorFeature' ? data.geometry : geoToVector<D>(data.geometry, buildBBox);
  for (const { geometry, face } of vectorGeoToS2<D>(vectorGeo)) {
    res.push({
      id,
      type: 'S2Feature',
      face,
      properties,
      metadata,
      geometry,
    });
  }

  return res;
}

/** The resultant geometry after conversion */
export interface ConvertedGeometry<M extends MValue = Properties> {
  /** The vector geometry that was converted */
  geometry: VectorGeometry<M>;
  /** The face of the vector geometry that was converted */
  face: Face;
}
/** A list of converted geometries */
export type ConvertedGeometryList<M extends MValue = Properties> = ConvertedGeometry<M>[];

/**
 * Underlying conversion mechanic to move GeoJSON Geometry to S2Geometry
 * @param geometry - GeoJSON Geometry
 * @returns - S2Geometry
 */
export function vectorGeoToS2<M extends MValue = Properties>(
  geometry: VectorGeometry<M>,
): ConvertedGeometryList<M> {
  const { type } = geometry;
  let cGeo: ConvertedGeometryList<M>;
  if (type === 'Point') cGeo = convertGeometryPoint(geometry);
  else if (type === 'MultiPoint') cGeo = convertGeometryMultiPoint(geometry);
  else if (type === 'LineString') cGeo = convertGeometryLineString(geometry);
  else if (type === 'MultiLineString') cGeo = convertGeometryMultiLineString(geometry);
  else if (type === 'Polygon') cGeo = convertGeometryPolygon(geometry);
  else if (type === 'MultiPolygon') cGeo = convertGeometryMultiPolygon(geometry);
  else {
    throw new Error('Either the conversion is not yet supported or Invalid S2Geometry type.');
  }
  return cGeo;
}

/**
 * @param geometry - GeoJSON PointGeometry
 * @returns - S2 PointGeometry
 */
function convertGeometryPoint<M extends MValue = Properties>(
  geometry: VectorPointGeometry<M>,
): ConvertedGeometryList<M> {
  const { type, is3D, coordinates, bbox } = geometry;
  const { z, m } = coordinates;
  const [face, s, t] = toST(fromLonLat(coordinates));
  const vecBBox = fromPoint({ x: s, y: t, z });
  return [{ face, geometry: { type, is3D, coordinates: { x: s, y: t, z, m }, bbox, vecBBox } }];
}

/**
 * @param geometry - GeoJSON PointGeometry
 * @returns - S2 PointGeometry
 */
function convertGeometryMultiPoint<M extends MValue = Properties>(
  geometry: VectorMultiPointGeometry<M>,
): ConvertedGeometryList<M> {
  const { is3D, coordinates, bbox } = geometry;
  return coordinates.flatMap((coordinates) =>
    convertGeometryPoint({ type: 'Point', is3D, coordinates, bbox }),
  );
}

/**
 * @param geometry - GeoJSON LineStringGeometry
 * @returns - S2 LineStringGeometry
 */
function convertGeometryLineString<M extends MValue = Properties>(
  geometry: VectorLineStringGeometry<M>,
): ConvertedGeometryList<M> {
  const { type, is3D, coordinates, bbox } = geometry;

  return convertLineString(coordinates, false).map(({ face, line, offset, vecBBox }) => {
    return { face, geometry: { type, is3D, coordinates: line, bbox, offset, vecBBox } };
  });
}

/**
 * @param geometry - GeoJSON MultiLineStringGeometry
 * @returns - S2 MultiLineStringGeometry
 */
function convertGeometryMultiLineString<M extends MValue = Properties>(
  geometry: VectorMultiLineStringGeometry<M>,
): ConvertedGeometryList<M> {
  const { coordinates, is3D, bbox } = geometry;
  return coordinates
    .flatMap((line) => convertLineString(line, false))
    .map(({ face, line, offset, vecBBox }) => ({
      face,
      geometry: { type: 'LineString', is3D, coordinates: line, bbox, offset, vecBBox },
    }));
}

/**
 * @param geometry - GeoJSON PolygonGeometry
 * @returns - S2 PolygonGeometry
 */
function convertGeometryPolygon<M extends MValue = Properties>(
  geometry: VectorPolygonGeometry<M>,
): ConvertedGeometryList<M> {
  const { type, is3D, coordinates, bbox } = geometry;
  const res: ConvertedGeometryList<M> = [];

  // conver all lines
  const outerRing = convertLineString(coordinates[0], true);
  const innerRings = coordinates.slice(1).flatMap((line) => convertLineString(line, true));

  // for each face, build a new polygon
  for (const { face, line, offset, vecBBox: polyBBox } of outerRing) {
    const polygon: VectorPolygon<M> = [line];
    const polygonOffsets = [offset];
    for (const { face: innerFace, line: innerLine, offset: innerOffset, vecBBox } of innerRings) {
      if (innerFace === face) {
        polygon.push(innerLine);
        polygonOffsets.push(innerOffset);
        mergeBBoxes(polyBBox, vecBBox);
      }
    }

    res.push({
      face,
      geometry: {
        type,
        coordinates: polygon,
        is3D,
        bbox,
        offset: polygonOffsets,
        vecBBox: polyBBox,
      },
    });
  }

  return res;
}

/**
 * @param geometry - GeoJSON MultiPolygonGeometry
 * @returns - S2 MultiPolygonGeometry
 */
function convertGeometryMultiPolygon<M extends MValue = Properties>(
  geometry: VectorMultiPolygonGeometry<M>,
): ConvertedGeometryList<M> {
  const { is3D, coordinates, bbox, offset } = geometry;
  return coordinates.flatMap((polygon, i) =>
    convertGeometryPolygon({
      type: 'Polygon',
      is3D,
      coordinates: polygon,
      bbox,
      offset: offset?.[i],
    }),
  );
}

/** LineString converted from WM to S2 */
interface ConvertedLineString<M extends MValue = Properties> {
  face: Face;
  line: VectorLineString<M>;
  offset: number;
  vecBBox: BBOX;
}

/**
 * @param line - GeoJSON LineString
 * @param isPolygon - true if the line originates from a polygon
 * @returns - S2 LineStrings clipped to it's 0->1 coordinate system
 */
function convertLineString<M extends MValue = Properties>(
  line: VectorLineString<M>,
  isPolygon: boolean,
): ConvertedLineString<M>[] {
  const res: ConvertedLineString<M>[] = [];
  // find all the faces that exist in the line while reprojectiong
  const faces = new Set<Face>();
  // first re-project all the coordinates to S2
  const newGeometry: STPoint<M>[] = [];
  for (const { x, y, z, m } of line) {
    const [face, s, t] = toST(fromLonLat({ x, y }));
    const point: STPoint<M> = { face, s, t, z, m };
    faces.add(face);
    newGeometry.push(point);
  }
  // for each face, build a line
  for (const face of faces) {
    const line: VectorLineString<M> = [];
    for (const stPoint of newGeometry) line.push(stPointToFace(face, stPoint));
    const clippedLines = clipLine(line, [0, 0, 1, 1], isPolygon);
    for (const { line, offset, vecBBox } of clippedLines) res.push({ face, line, offset, vecBBox });
  }

  return res;
}

/**
 * @param targetFace - face you want to project to
 * @param stPoint - the point you want to project
 * @returns - the projected point
 */
function stPointToFace<M extends MValue = Properties>(
  targetFace: Face,
  stPoint: STPoint<M>,
): VectorPoint<M> {
  const { face: curFace, s, t, z, m } = stPoint;
  if (targetFace === curFace) return { x: s, y: t, z, m };

  const [rot, x, y] = FACE_RULE_SET[targetFace][curFace];
  const [newS, newT] = rotate(rot as 0 | 90 | -90, s, t);

  return { x: newS + x, y: newT + y, z, m };
}

/**
 * @param rot - rotation
 * @param s - input s
 * @param t - input t
 * @returns - new [s, t] after rotating
 */
function rotate(rot: 0 | 90 | -90, s: number, t: number): [s: number, t: number] {
  if (rot === 90) return [t, 1 - s];
  else if (rot === -90) return [1 - t, s];
  else return [s, t]; // Handles the 0° case and any other unspecified rotations
}

/**
 * Ruleset for converting an S2Point from a face to another.
 * While this this set includes opposite side faces, without axis mirroring,
 * it is not technically accurate and shouldn't be used. Instead, data should let two points travel
 * further than a full face width.
 * FACE_RULE_SET[targetFace][currentFace] = [rot, x, y]
 */
const FACE_RULE_SET: [rotation: number, moveX: number, MoveY: number][][] = [
  // Target Face 0
  [
    [0, 0, 0], // Current Face 0
    [0, 1, 0], // Current Face 1
    [90, 0, 1], // Current Face 2
    [-90, 2, 0], // Current Face 3
    [-90, -1, 0], ///  Current Face 4
    [0, 0, -1], ///  Current Face 5
  ],
  // Target Face 1
  [
    [0, -1, 0], // Current Face 0
    [0, 0, 0], // Current Face 1
    [0, 0, 1], // Current Face 2
    [-90, 1, 0], // Current Face 3
    [-90, 2, 0], // Current Face 4
    [90, 0, -1], // Current Face 5
  ],
  // Target Face 2
  [
    [-90, -1, 0], // Current Face 0
    [0, 0, -1], // Current Face 1
    [0, 0, 0], // Current Face 2
    [0, 1, 0], // Current Face 3
    [90, 0, 1], // Current Face 4
    [-90, 2, 0], // Current Face 5
  ],
  // Target Face 3
  [
    [-90, 2, 0], // Current Face 0
    [90, 0, -1], // Current Face 1
    [0, -1, 0], // Current Face 2
    [0, 0, 0], // Current Face 3
    [0, 0, 1], // Current Face 4
    [-90, 1, 0], // Current Face 5
  ],
  // Target Face 4
  [
    [90, 0, 1], // Current Face 0
    [-90, 2, 0], // Current Face 1
    [-90, -1, 0], // Current Face 2
    [0, 0, -1], // Current Face 3
    [0, 0, 0], // Current Face 4
    [0, 1, 0], // Current Face 5
  ],
  // Target Face 5
  [
    [0, 0, 1], // Current Face 0
    [-90, 1, 0], // Current Face 1
    [-90, 2, 0], // Current Face 2
    [90, 0, -1], // Current Face 3
    [0, -1, 0], // Current Face 4
    [0, 0, 0], // Current Face 5
  ],
];
