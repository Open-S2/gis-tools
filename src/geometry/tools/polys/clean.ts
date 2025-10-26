import { cleanLineString, deKinkPolygon, equalPoints, polygonRingArea } from '../index.js';

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
): VectorMultiPolygon<D> {
  const vectorPolygons: VectorMultiPolygon<D> =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;
  return vectorPolygons.flatMap((p) => cleanPolygon(p, removeCollinearPoints));
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
): VectorMultiPolygon<D> {
  const vectorPolygon: VectorPolygon<D> =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;

  // clone vectorPolygon so we can return a new object
  const cloned: VectorPolygon<D> = vectorPolygon.map((ring) => ring.map((p) => ({ ...p })));

  // remove duplicates from the rings (and optionally remove superfluous/collinear points)
  const res: VectorPolygon<D> = [];
  for (const ring of cloned) {
    let lastPoint: VectorPoint<D> | undefined;
    if (removeCollinearPoints) {
      res.push(cleanLineString(ring, true));
    } else {
      const newRing: VectorPoint<D>[] = [];
      for (const point of ring) {
        if (lastPoint === undefined || !equalPoints(point, lastPoint)) {
          newRing.push(point);
          lastPoint = point;
        }
      }
      res.push(newRing);
    }
  }

  // run polygonRingArea for each ring and invert if it's direction is wrong for the ring type
  for (let i = 0; i < res.length; i++) {
    const ring = res[i];
    const area = polygonRingArea(ring, 1);
    // flip the ring if outer-ring and area is negative OR inner-ring and area is positive
    if (i === 0 ? area < 0 : area > 0) res[i] = ring.reverse();
  }

  return deKinkPolygon(res);
}
