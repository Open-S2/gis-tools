import { EARTH_RADIUS, degToRad, radToDeg } from '../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorGeometry,
  VectorMultiPointGeometry,
  VectorPoint,
} from '../../index.js';

/**
 * Check if two XYZ Points are equal
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - True if the two XYZ Points are equal
 */
export function equalPoints(a?: VectorPoint, b?: VectorPoint): boolean {
  if (a === undefined || b === undefined) return false;
  return a.x === b.x && a.y === b.y && a.z === b.z;
}

/**
 * Find the average of a collection of Vector points
 * @param vectorPoints - collection of Vector points, whether from a VectorFeature, geometry, or raw coordinates
 * @returns - the average of the vector points
 */
export function averageOfPoints<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  vectorPoints:
    | VectorPoint<D>[]
    | VectorMultiPointGeometry<D>
    | VectorFeature<M, D, P, VectorMultiPointGeometry<D>>,
): VectorPoint<D> {
  const coords =
    'geometry' in vectorPoints
      ? vectorPoints.geometry.coordinates
      : 'coordinates' in vectorPoints
        ? vectorPoints.coordinates
        : vectorPoints;
  if (coords.length === 0) return { x: 0, y: 0 };
  let xAvg = 0;
  let yAvg = 0;
  let zAvg = 0;
  let hasZ = false;
  for (const { x, y, z } of coords) {
    xAvg += x;
    yAvg += y;
    if (z !== undefined) {
      zAvg += z;
      hasZ = true;
    }
  }
  xAvg /= coords.length;
  yAvg /= coords.length;
  zAvg /= coords.length;
  if (hasZ) return { x: xAvg, y: yAvg, z: zAvg };
  return { x: xAvg, y: yAvg };
}

/**
 * Find the center of a collection of Vector points
 * @param vectorPoints - collection of Vector points, whether from a VectorFeature, geometry, or raw coordinates
 * @returns - the center of the vector points
 */
export function centerOfPoints<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  vectorPoints:
    | VectorPoint<D>[]
    | VectorMultiPointGeometry<D>
    | VectorFeature<M, D, P, VectorMultiPointGeometry<D>>,
): VectorPoint<D> {
  const { min, max } = Math;
  const coords =
    'geometry' in vectorPoints
      ? vectorPoints.geometry.coordinates
      : 'coordinates' in vectorPoints
        ? vectorPoints.coordinates
        : vectorPoints;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  for (const { x, y, z } of coords) {
    minX = min(minX, x);
    maxX = max(maxX, x);
    minY = min(minY, y);
    maxY = max(maxY, y);
    if (z !== undefined) {
      minZ = min(minZ, z);
      maxZ = max(maxZ, z);
    }
  }
  const x = (minX + maxX) / 2;
  const y = (minY + maxY) / 2;
  if (minZ !== Infinity && maxZ !== -Infinity) return { x, y, z: (minZ + maxZ) / 2 };
  return { x, y };
}

/**
 * Given an input vector feature, create a collection of points
 * @param data - vector feature with various geometry types
 * @returns - all features as a collection of points
 */
export function toPoints<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(data: VectorFeature<M, D, P, VectorGeometry<D>>): VectorMultiPointGeometry<D> {
  const { type, is3D, coordinates } = data.geometry;

  const res: VectorPoint<D>[] = [];

  if (type === 'Point') {
    res.push(coordinates);
  } else if (type === 'MultiPoint') {
    res.push(...coordinates);
  } else if (type === 'LineString') {
    res.push(...coordinates);
  } else if (type === 'MultiLineString') {
    res.push(...coordinates.flat());
  } else if (type === 'Polygon') {
    res.push(...coordinates.flat());
  } else if (type === 'MultiPolygon') {
    res.push(...coordinates.flat(2));
  }

  return {
    type: 'MultiPoint',
    is3D,
    coordinates: res,
  };
}

/**
 * Get the bearing in degrees between two points
 * @param start - starting point
 * @param end - ending point
 * @returns - the bearing
 */
export function pointBearing<A extends MValue = Properties, B extends MValue = Properties>(
  start: VectorPoint<A>,
  end: VectorPoint<B>,
): number {
  const { atan2, cos, sin } = Math;
  const lat1 = degToRad(start.y);
  const lat2 = degToRad(end.y);
  const lon1 = degToRad(start.x);
  const lon2 = degToRad(end.x);
  const y = sin(lon2 - lon1) * cos(lat2);
  const x = cos(lat1) * sin(lat2) - sin(lat1) * cos(lat2) * cos(lon2 - lon1);

  return (radToDeg(atan2(y, x)) + 360) % 360;
}

/**
 * Get the destination given a start point, bearing, and distance
 * @param start - starting point (in Lon-Lat degrees)
 * @param bearing - the bearing (in degrees)
 * @param distance - the distance (in meters)
 * @param radius - the radius of the sphere. Defaults to the Earth's radius in meters.
 * @returns - the destination point in Lon-Lat degrees
 */
export function pointDestination<D extends MValue = Properties>(
  start: VectorPoint<D>,
  bearing: number,
  distance: number,
  radius: number = EARTH_RADIUS,
): VectorPoint {
  const { atan2, cos, sin, asin } = Math;
  const s_lon = degToRad(start.x);
  const s_lat = degToRad(start.y);
  bearing = degToRad(bearing);
  const radians = distance / radius;
  const e_lat = asin(sin(s_lat) * cos(radians) + cos(s_lat) * sin(radians) * cos(bearing));
  const e_lon =
    s_lon + atan2(sin(bearing) * sin(radians) * cos(s_lat), cos(radians) - sin(s_lat) * sin(e_lat));

  return {
    x: radToDeg(e_lon),
    y: radToDeg(e_lat),
  };
}

/**
 * Updates the WGS84 point's x and y values as needed
 * @param point - the WGS 84 point to clamp/wrap
 * @returns the point itself post update
 */
export function clampWGS84Point<D extends MValue = Properties>(
  point: VectorPoint<D>,
): VectorPoint<D> {
  // Don't touch the point if it's already in bounds
  if (point.x < -180 || point.x >= 180) {
    point.x = ((((point.x + 180) % 360) + 360) % 360) - 180;
  }
  if (point.y < -90 || point.y > 90) point.y = Math.min(Math.max(point.y, -90), 90);

  return point;
}
