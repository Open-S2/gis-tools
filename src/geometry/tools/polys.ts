import { EARTH_RADIUS } from '../../space/planets';
import { orient2d } from '../predicates';
import { pointOverlap } from '../bbox';

import type {
  Properties,
  S2Feature,
  VectorFeature,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../..';

// TODO: polygon(s)AreaS2(...)

/**
 * Find the area of a collection of polygons. Assumes geometry is in lon-lat space
 * @param polygons - the collection of polygons
 * @param planetRadius - the radius of the planet (Earth by default)
 * @returns - the total area of the polygon
 */
export function polygonsArea(
  polygons:
    | VectorMultiPolygon
    | VectorMultiPolygonGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorMultiPolygonGeometry>,
  planetRadius = EARTH_RADIUS,
): number {
  const vectorPolygons: VectorMultiPolygon =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;

  let area = 0;
  for (const polygon of vectorPolygons) area += polygonArea(polygon, planetRadius);

  return area;
}

/**
 * Find the area of a polygon. Assumes geometry is in Lon-Lat space
 * @param polygon - the polygon
 * @param planetRadius - the radius of the planet (Earth by default)
 * @returns - The approximate signed geodesic area of the polygon in square meters.
 */
export function polygonArea(
  polygon:
    | VectorPolygon
    | VectorPolygonGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorPolygonGeometry>,
  planetRadius = EARTH_RADIUS,
): number {
  // check poly against the point
  const vectorPolygon: VectorPolygon =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;

  // grab the area of the outer ring
  let area = _ringArea(vectorPolygon[0], planetRadius);
  // subtract the area of the inner rings (holes)
  for (let i = 1; i < vectorPolygon.length; i++) {
    area -= _ringArea(vectorPolygon[i], planetRadius);
  }

  return area;
}

/**
 * A robust method to see if a point is in a collection of polygons or not.
 * Be sure the point and polygon are in the same projection space.
 * @param point - the point to check
 * @param polygons - the collection of polygons
 * @param ignoreBoundary - if true, ignore when the point is on the boundary
 * @returns - true if the point is in the polygon
 */
export function pointInPolygons(
  point: VectorPoint,
  polygons:
    | VectorMultiPolygon
    | VectorMultiPolygonGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorMultiPolygonGeometry>
    | S2Feature<Record<string, unknown>, Properties, Properties, VectorMultiPolygonGeometry>,
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
export function pointInPolygon(
  point: VectorPoint,
  polygon:
    | VectorPolygon
    | VectorPolygonGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorPolygonGeometry>
    | S2Feature<Record<string, unknown>, Properties, Properties, VectorPolygonGeometry>,
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
function _pointInPolygon(point: VectorPoint, polygon: VectorPolygon): boolean | 0 {
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

/**
 * Calculate the approximate area of the polygon were it projected onto the planet.
 * Note that this area will be positive if ring is oriented counter-clockwise,
 * otherwise it will be negative.
 *
 * Reference:
 * Robert. G. Chamberlain and William H. Duquette, "Some Algorithms for Polygons on a Sphere",
 * JPL Publication 07-03, Jet Propulsion
 * Laboratory, Pasadena, CA, June 2007 https://trs.jpl.nasa.gov/handle/2014/40409
 * @param coords - ring Coordinates in lon-lat space
 * @param planetRadius - the radius of the planet (Earth by default)
 * @returns - The approximate signed geodesic area of the polygon in square meters.
 */
function _ringArea(coords: VectorLineString, planetRadius: number): number {
  const RAD = 0.017453292519943295; // Math.PI / 180;
  const coordsLength = coords.length - 1;
  const factor = (planetRadius * planetRadius) / 2;

  if (coordsLength <= 2) return 0;
  let total = 0;

  let i = 0;
  while (i < coordsLength) {
    const lower = coords[i];
    const middle = coords[i + 1 === coordsLength ? 0 : i + 1];
    const upper = coords[i + 2 >= coordsLength ? (i + 2) % coordsLength : i + 2];

    const lowerX = lower.x * RAD;
    const middleY = middle.y * RAD;
    const upperX = upper.x * RAD;

    total += (upperX - lowerX) * Math.sin(middleY);

    i++;
  }

  return -(total * factor);
}
