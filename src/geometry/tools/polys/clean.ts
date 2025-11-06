import {
  clampWGS84Point,
  cleanLineString,
  dekinkPolygon,
  equalPoints,
  mergeBBoxes,
  polygonRingArea,
} from '../../../index.js';

import type {
  MValue,
  Properties,
  VectorFeatures,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../../../index.js';

/**
 * Ensures the collection of polygon ring order is correct, removes duplicate points,
 * and runs a dekink to be thorough.
 *
 * NOTE: This will not remove/reduce points that follow a path angle like [[0, 0], [0, 1], [0, 2], ...].
 * The decision to leave this to the user is due to the fact that not all projections are guaranteed
 * to support a linear relationship. Also sometimes the user want's to have these extra points for
 * future/cleaner projection changes. For example, having higher precision works well when
 * translating to spherical projections for instance. If you do want to remove these points, pass
 * in true to `removeCollinearPoints`
 * @param polygons - the collection of polygon as either a VectorFeature, VectorMultiPolygonGeometry, or raw VectorMultiPolygon
 * @param removeCollinearPoints - if true, remove superfluous points
 * @param cleanWGS84 - if true, clean WGS84 points to be in bounds
 * @returns - the cleaned polygon
 */
export function cleanPolygons<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygons:
    | VectorMultiPolygon<D>
    | VectorMultiPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorMultiPolygonGeometry<D>>,
  removeCollinearPoints = false,
  cleanWGS84 = false,
): VectorMultiPolygonGeometry<D> | undefined {
  const vectorPolygons: VectorMultiPolygon<D> =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;

  const res: VectorMultiPolygonGeometry<D> = { type: 'MultiPolygon', coordinates: [], is3D: false };
  for (const vectorPoly in vectorPolygons) {
    const cleaned = cleanPolygon(vectorPolygons[vectorPoly], removeCollinearPoints, cleanWGS84);
    if (cleaned !== undefined) {
      if (cleaned.bbox !== undefined) res.bbox = mergeBBoxes(res.bbox, cleaned.bbox);
      res.coordinates.push(...cleaned.coordinates);
    }
  }

  return res;
}

/**
 * Ensures the polygon ring order is correct, removes duplicate points, and runs a dekink to be
 * thorough.
 *
 * NOTE: This will not remove/reduce points that follow a path angle like [[0, 0], [0, 1], [0, 2], ...].
 * The decision to leave this to the user is due to the fact that not all projections are guaranteed
 * to support a linear relationship. Also sometimes the user want's to have these extra points for
 * future/cleaner projection changes. For example, having higher precision works well when
 * translating to spherical projections for instance. If you do want to remove these points, pass
 * in true to `removeCollinearPoints`
 * @param polygon - the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @param removeCollinearPoints - if true, remove superfluous points
 * @param cleanWGS84 - if true, clean WGS84 points to be in bounds
 * @returns - the cleaned polygon
 */
export function cleanPolygon<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygon:
    | VectorPolygon<D>
    | VectorPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorPolygonGeometry<D>>,
  removeCollinearPoints = false,
  cleanWGS84 = false,
): VectorMultiPolygonGeometry<D> | undefined {
  const vectorPolygon: VectorPolygon<D> =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;

  // clone vectorPolygon so we can return a new object
  const cloned: VectorPolygon<D> = vectorPolygon.map((ring) =>
    ring.map((p) => {
      const dup = { ...p };
      return cleanWGS84 ? clampWGS84Point(dup) : dup;
    }),
  );

  // remove duplicates from the rings (and optionally remove superfluous/collinear points)
  // If outer ring is dropped, we kill the polygon
  const res: VectorPolygon<D> = [];
  for (let i = 0; i < cloned.length; i++) {
    const ring = cloned[i];
    let lastPoint: VectorPoint<D> | undefined;
    if (removeCollinearPoints) {
      const line = cleanLineString(ring, true);
      if (line === undefined) {
        if (i === 0) return;
        continue;
      } else res.push(line);
    } else {
      const newRing: VectorPoint<D>[] = [];
      for (const point of ring) {
        if (lastPoint === undefined || !equalPoints(point, lastPoint)) {
          newRing.push(point);
          lastPoint = point;
        }
      }
      if (newRing.length >= 4) res.push(newRing);
      else if (i === 0) return;
    }
  }

  // run polygonRingArea for each ring and invert if it's direction is wrong for the ring type
  for (let i = 0; i < res.length; i++) {
    const ring = res[i];
    const area = polygonRingArea(ring, 1);
    // flip the ring if outer-ring and area is negative OR inner-ring and area is positive
    if (i === 0 ? area < 0 : area > 0) res[i] = ring.reverse();
  }

  return dekinkPolygon(res);
}
