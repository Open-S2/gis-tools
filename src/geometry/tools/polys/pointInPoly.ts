import { orient2d } from '../../predicates/index.js';
import { pointOverlap } from '../../bbox.js';

import type {
  MValue,
  Properties,
  S2Feature,
  VectorFeature,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../../../index.js';

/**
 * A robust method to see if a point is in a collection of polygons or not.
 * Be sure the point and polygon are in the same projection space.
 * @param point - the point to check
 * @param polygons - the collection of polygons
 * @param ignoreBoundary - if true, ignore when the point is on the boundary
 * @returns - true if the point is in the polygon
 */
export function pointInPolygons<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  point: VectorPoint,
  polygons:
    | VectorMultiPolygon<D>
    | VectorMultiPolygonGeometry<D>
    | VectorFeature<M, D, P, VectorMultiPolygonGeometry<D>>
    | S2Feature<M, D, P, VectorMultiPolygonGeometry<D>>,
  ignoreBoundary = false,
): boolean {
  const vectorPolygons: VectorMultiPolygon =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;
  for (const polygon of vectorPolygons)
    if (pointInPolygon(point, polygon, ignoreBoundary)) return true;
  return false;
}

/**
 * A robust method to see if a point is in a polygon or not.
 * Be sure the point and polygon are in the same projection space.
 * @param point - the point to check
 * @param polygon - the polygon
 * @param ignoreBoundary - if true, ignore when the point is on the boundary
 * @returns - true if the point is in the polygon
 */
export function pointInPolygon<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  point: VectorPoint,
  polygon:
    | VectorPolygon<D>
    | VectorPolygonGeometry<D>
    | VectorFeature<M, D, P, VectorPolygonGeometry<D>>
    | S2Feature<M, D, P, VectorPolygonGeometry<D>>,
  ignoreBoundary = false,
): boolean {
  // bbox test case - if it doesn't even fit within the bbox, we know it's not in the polygon
  const bbox =
    'geometry' in polygon ? polygon.geometry.bbox : 'bbox' in polygon ? polygon.bbox : undefined;
  if (bbox !== undefined && !pointOverlap(bbox, point)) return false;
  // check poly against the point
  const vectorPolygon: VectorPolygon =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;

  const pip = _pointInPolygon(point, vectorPolygon);
  if (pip === 0) {
    if (ignoreBoundary) return false;
    return true;
  } else return pip;
}

/**
 * A Robust point in polygon test
 * @param point - the point
 * @param polygon - the polygon
 * @returns - true if the point is in the polygon, 0 if on the boundary, false otherwise
 */
function _pointInPolygon<M extends MValue = Properties>(
  point: VectorPoint<M>,
  polygon: VectorPolygon<M>,
): boolean | 0 {
  let i;
  let ii;
  let k = 0;
  let f;
  let u1;
  let v1;
  let u2;
  let v2;
  let currentP;
  let nextP;

  const { x, y } = point;

  const numContours = polygon.length;
  for (i = 0; i < numContours; i++) {
    ii = 0;
    const contour = polygon[i];
    const contourLen = contour.length - 1;

    currentP = contour[0];
    if (currentP.x !== contour[contourLen].x && currentP.y !== contour[contourLen].y) {
      // since the first and last coordinates in a ring are not the same, assume it's not a polygon and return false
      return false;
    }

    u1 = currentP.x - x;
    v1 = currentP.y - y;

    for (ii; ii < contourLen; ii++) {
      nextP = contour[ii + 1];

      u2 = nextP.x - x;
      v2 = nextP.y - y;

      if (v1 === 0 && v2 === 0) {
        if ((u2 <= 0 && u1 >= 0) || (u1 <= 0 && u2 >= 0)) return 0;
      } else if ((v2 >= 0 && v1 <= 0) || (v2 <= 0 && v1 >= 0)) {
        f = orient2d(u1, u2, v1, v2, 0, 0);
        if (f === 0) return 0;
        if ((f > 0 && v2 > 0 && v1 <= 0) || (f < 0 && v2 <= 0 && v1 > 0)) k++;
      }
      currentP = nextP;
      v1 = v2;
      u1 = u2;
    }
  }

  if (k % 2 === 0) return false;
  return true;
}
