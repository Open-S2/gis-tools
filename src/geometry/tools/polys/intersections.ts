import { BoxIndex, intersectionOfSegmentsRobust } from '../../../index.js';

import type {
  BoxIndexAccessor,
  MValue,
  Properties,
  S2Feature,
  VectorFeature,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
} from '../../../index.js';

/** A segment in a polygon */
export interface Segment {
  id: number;
  polyIndex: number;
  ringIndex: number;
  from: number; // index in the polys[polygon][ring][from]
  to: number; // index in the polys[polygon][ring][to]
}

/** An intersection of two segments */
export interface Intersection {
  segment1: Segment;
  segment2: Segment;
  point: VectorPoint;
}

/**
 * Find the intersection of a collection of polygons
 * @param polygons - the collection of polygons
 * @returns - found intersections
 */
export function polygonsIntersections<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygons:
    | VectorMultiPolygon<D>
    | VectorMultiPolygonGeometry<D>
    | VectorFeature<M, D, P, VectorMultiPolygonGeometry<D>>
    | S2Feature<M, D, P, VectorMultiPolygonGeometry<D>>,
): Intersection[] {
  const res: Intersection[] = [];
  // setup accessing data
  const vectorPolygons: VectorMultiPolygon =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;
  // build all segments
  const segments = buildPolygonSegments(vectorPolygons);

  /**
   * Setup a function for accessing the minX, minY, maxX, and maxY properties of the items.
   * @param segment - the segment
   * @returns - the minX, minY, maxX, and maxY
   */
  const getBounds: BoxIndexAccessor<Segment> = (segment: Segment) => {
    const { min, max } = Math;
    const { polyIndex, ringIndex, from, to } = segment;
    const fromPoint = vectorPolygons[polyIndex][ringIndex][from];
    const toPoint = vectorPolygons[polyIndex][ringIndex][to];
    return [
      min(fromPoint.x, toPoint.x),
      min(fromPoint.y, toPoint.y),
      max(fromPoint.x, toPoint.x),
      max(fromPoint.y, toPoint.y),
    ];
  };
  // setup a 2D box index
  const boxIndex = new BoxIndex(segments, getBounds);

  // iterate each segment and check for intersections with other segments
  for (const segment1 of segments) {
    const potentialIntersections = boxIndex.search(
      ...getBounds(segment1),
      (seg: Segment) =>
        seg.id !== segment1.id && seg.polyIndex !== segment1.polyIndex && seg.id > segment1.id,
    );
    for (const segment2 of potentialIntersections) {
      const point = findPolygonIntersections(vectorPolygons, segment1, segment2);
      if (point !== undefined) res.push({ segment1, segment2, point });
    }
  }

  return res;
}

/**
 * Build all segments
 * @param vectorPolygons - the collection of polygons
 * @returns - the collection of segments
 */
export function buildPolygonSegments(vectorPolygons: VectorMultiPolygon): Segment[] {
  const segments: Segment[] = [];
  for (let p = 0; p < vectorPolygons.length; p++) {
    const polygon = vectorPolygons[p];
    for (let r = 0; r < polygon.length; r++) {
      const ring = polygon[r];
      for (let s = 0; s < ring.length - 1; s++) {
        segments.push({ id: segments.length, polyIndex: p, ringIndex: r, from: s, to: s + 1 });
      }
    }
  }

  return segments;
}

/**
 * Find the intersection of two segments if it exists
 * @param vectorPolygons - the collection of polygons
 * @param segment1 - the first segment
 * @param segment2 - the second segment
 * @returns - the intersection if it exists. Undefined otherwise.
 */
export function findPolygonIntersections(
  vectorPolygons: VectorMultiPolygon,
  segment1: Segment,
  segment2: Segment,
): VectorPoint | undefined {
  const p1 = vectorPolygons[segment1.polyIndex][segment1.ringIndex][segment1.from];
  const p2 = vectorPolygons[segment1.polyIndex][segment1.ringIndex][segment1.to];
  const q1 = vectorPolygons[segment2.polyIndex][segment2.ringIndex][segment2.from];
  const q2 = vectorPolygons[segment2.polyIndex][segment2.ringIndex][segment2.to];
  return intersectionOfSegmentsRobust([p1, p2], [q1, q2]);
}
