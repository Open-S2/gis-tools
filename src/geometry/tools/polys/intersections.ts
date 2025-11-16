import {
  BoxIndex,
  equalPoints,
  intersectionOfSegmentsRobust,
  nextDown,
  nextUp,
} from '../../../index.js';

import type {
  BBox,
  BoxIndexAccessor,
  IntersectionOfSegmentsRobust,
  MValue,
  Properties,
  VectorFeatures,
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
export interface Intersection<D extends MValue = Properties> {
  segment1: Segment;
  segment2: Segment;
  point: VectorPoint<D>;
  u: number; // where the intersection occurs on segment1 from 0 to 1
  t: number; // where the intersection occurs on segment2 from 0 to 1
}

/** Local Intersection to a [polyIndex][ringIndex] */
export interface RingIntersection<D extends MValue = Properties> {
  from: number;
  to: number;
  point: VectorPoint<D>;
  t: number;
  tVec: VectorPoint<D>;
  tAngle: number;
}
/** Intersection Lookup for mapped by polyIndex and ringIndex */
export class RingIntersectionLookup<D extends MValue = Properties> {
  /** [polyIndex][ringIndex] -> Intersections */
  store: Map<number, Map<number, RingIntersection<D>[]>> = new Map();
  // eslint-disable-next-line jsdoc/require-jsdoc
  get(polyIndex: number, ringIndex: number): RingIntersection<D>[] {
    return this.store.get(polyIndex)?.get(ringIndex) ?? [];
  }
  // eslint-disable-next-line jsdoc/require-jsdoc
  set(polyIndex: number, ringIndex: number, int: RingIntersection<D>): void {
    let poly = this.store.get(polyIndex);
    if (poly === undefined) this.store.set(polyIndex, (poly = new Map()));
    let ring = poly.get(ringIndex);
    if (ring === undefined) poly.set(ringIndex, (ring = []));
    ring.push(int);
  }
}

/**
 * Find the intersection of a collection of polygons
 * @param polygons - the collection of polygons
 * @param includeSelfIntersections - if true, include self intersections
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
    | VectorFeatures<M, D, P, VectorMultiPolygonGeometry<D>>,
  includeSelfIntersections = false,
): Intersection<D>[] {
  const res: Intersection<D>[] = [];
  // setup accessing data
  const vectorPolygons: VectorMultiPolygon<D> =
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
  const getBounds: BoxIndexAccessor<Segment> = (segment: Segment): BBox => {
    const { min, max } = Math;
    const { polyIndex, ringIndex, from, to } = segment;
    const { x: fromX, y: fromY } = vectorPolygons[polyIndex][ringIndex][from];
    const { x: toX, y: toY } = vectorPolygons[polyIndex][ringIndex][to];
    return [min(fromX, toX), min(fromY, toY), max(fromX, toX), max(fromY, toY)];
  };
  // setup a 2D box index
  const boxIndex = new BoxIndex(segments, getBounds);

  // iterate each segment and check for intersections with other segments
  for (const segment1 of segments) {
    const potentialIntersections = boxIndex.search(
      ...getBounds(segment1),
      (seg: Segment) =>
        seg.id !== segment1.id &&
        // if self-intersections are not included skip all segments from the same polyIndex
        // otherwise skip all segments from the same ringIndex whose end points interact
        (!includeSelfIntersections
          ? seg.polyIndex !== segment1.polyIndex
          : seg.ringIndex !== segment1.ringIndex ||
            (seg.from !== segment1.from &&
              seg.to !== segment1.to &&
              seg.to !== segment1.from &&
              seg.from !== segment1.to)) &&
        seg.id > segment1.id,
    );
    for (const segment2 of potentialIntersections) {
      const intP = findPolygonIntersections<D>(vectorPolygons, segment1, segment2);
      if (intP !== undefined)
        res.push({ segment1, segment2, point: intP.point, u: intP.u, t: intP.t });
    }
  }

  return res;
}

/**
 * Run through the vectorPolygons and Builds the ring intersection lookup
 * @param vectorPolygons - the collection of polygons
 * @param segmentFilter -  the function to filter the segments, default ignores self intersections
 * @returns - the ring intersection lookup for all rings in the multipolygon collection
 */
export function polygonsIntersectionsLookup<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
  segmentFilter?: (seg1: Segment) => { (seg2: Segment): boolean },
): RingIntersectionLookup<D> {
  const segments = buildPolygonSegments(vectorPolygons);
  const ringIntersectLookup = new RingIntersectionLookup<D>();

  if (segmentFilter === undefined) {
    /**
     * Default segment filter
     * @param seg1 - the first segment
     * @returns - filter on the second segment
     */
    segmentFilter = (seg1: Segment) => {
      return (seg2: Segment): boolean =>
        // if same id ignore
        seg2.id !== seg1.id &&
        // only pass forward not backward
        seg2.id > seg1.id &&
        // if same polyIndex ignore
        seg2.polyIndex !== seg1.polyIndex;
    };
  }

  /**
   * Setup a function for accessing the minX, minY, maxX, and maxY properties of the items.
   * @param segment - the segment
   * @returns - the minX, minY, maxX, and maxY
   */
  const getBounds: BoxIndexAccessor<Segment> = (segment: Segment): BBox => {
    const { min, max } = Math;
    const { polyIndex, ringIndex, from, to } = segment;
    const { x: fromX, y: fromY } = vectorPolygons[polyIndex][ringIndex][from];
    const { x: toX, y: toY } = vectorPolygons[polyIndex][ringIndex][to];
    return [min(fromX, toX), min(fromY, toY), max(fromX, toX), max(fromY, toY)];
  };
  // setup a 2D box index
  const boxIndex = new BoxIndex(segments, getBounds);
  // iterate each segment and check for intersections with other segments
  for (const segment1 of segments) {
    const { from: s1f, to: s1t, polyIndex: s1pi, ringIndex: s1ri } = segment1;
    const potentialIntersections = boxIndex.search(...getBounds(segment1), segmentFilter(segment1));
    for (const segment2 of potentialIntersections) {
      const { from: s2f, to: s2t, polyIndex: s2pi, ringIndex: s2ri } = segment2;
      const pInt = findPolygonIntersections<D>(vectorPolygons, segment1, segment2);
      // ignore points that interact at their edges if both segments leaving or coming
      if (pInt !== undefined) {
        // NOTE: It's important both segments share the same point as it may be updated
        const { point, u, t, uVec, tVec, uAngle, tAngle } = pInt;
        // skip if u and t are equal
        if (u === t && (u === 0 || u === 1)) continue;
        // first segment intersection
        const uInt = { from: s1f, to: s1t, point, t: u, tVec: uVec, tAngle: uAngle };
        ringIntersectLookup.set(s1pi, s1ri, uInt);
        // second segment intersection
        const tInt = { from: s2f, to: s2t, point, t, tVec, tAngle };
        ringIntersectLookup.set(s2pi, s2ri, tInt);
      }
    }
  }

  // finally clean the intersections before return
  for (const [_, polys] of ringIntersectLookup.store) {
    for (const [ringKey, intersections] of polys) {
      polys.set(ringKey, cleanIntersections(intersections));
    }
  }

  return ringIntersectLookup;
}

/**
 * Build all segments
 * @param vectorPolygons - the collection of polygons
 * @returns - the collection of segments
 */
export function buildPolygonSegments<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
): Segment[] {
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
export function findPolygonIntersections<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
  segment1: Segment,
  segment2: Segment,
): IntersectionOfSegmentsRobust<D> | undefined {
  const p1 = vectorPolygons[segment1.polyIndex][segment1.ringIndex][segment1.from];
  const p2 = vectorPolygons[segment1.polyIndex][segment1.ringIndex][segment1.to];
  const q1 = vectorPolygons[segment2.polyIndex][segment2.ringIndex][segment2.from];
  const q2 = vectorPolygons[segment2.polyIndex][segment2.ringIndex][segment2.to];
  return intersectionOfSegmentsRobust(
    [p1, p2],
    [q1, q2],
    segment1.polyIndex === segment2.polyIndex && segment1.ringIndex === segment2.ringIndex,
  );
}

/**
 * Given a ring's of intersections, clean them up
 * @param intersections - a collection of intersections to clean up
 * @returns - the cleaned up intersections
 */
export function cleanIntersections<D extends MValue = Properties>(
  intersections: RingIntersection<D>[],
): RingIntersection<D>[] {
  if (intersections.length === 0) return [];
  intersections.sort((a, b) => {
    let diff = a.from - b.from;
    if (diff === 0) diff = a.t - b.t;
    return diff;
  });

  // 1) Remove duplicates
  const dedupInts: RingIntersection<D>[] = [];
  for (const int of intersections) {
    if (
      dedupInts.some((c) => c.from === int.from && c.t === int.t && equalPoints(c.point, int.point))
    )
      continue;

    dedupInts.push(int);
  }
  // 2) Cancel out any intersections with other rings we only touch once with a single point
  if (dedupInts.length === 2) {
    const [first, second] = dedupInts;
    if (
      (first.t === 0 || first.t === 1) &&
      (second.t === 0 || second.t === 1) &&
      equalPoints(first.point, second.point)
    ) {
      return [];
    }
  }
  // 3) Intersections whose t values are not 0 or 1 but are equal to the start, end, or other
  // intersections with different t values need to be shifted by the smallest float possible to ensure
  // it doesn't conflict but on the line segment.
  updateIntersectionPoints(dedupInts);

  return dedupInts;
}

/**
 * Update all intersection points to ensure they are not equal to the start or end points if their t
 * values are not 0 or 1.
 *
 * When there is an intersection that the resultant point is equal to one of the segment edges,
 * then we shift the point by the smallest float possible.
 *
 * NOTE, If we have two or more points that are equal to one of the segment edges BUT the t values
 * are barely different, we need to keep shifting forward as needed.
 *
 * NOTE, What if we have TWO points that are equal to one of the segment edges BUT the t values
 * are different? We need to shift again as needed. There are also cases where two different lines
 * intersect another line and the resultant intersection is the same point but the t value along
 * the line is different.
 *
 * TODO: There are some corner cases I definitely haven't covered. Like a shift pushes an intersection into another intersection
 * @param intersections - the collection of intersections
 */
function updateIntersectionPoints<D extends MValue = Properties>(
  intersections: RingIntersection<D>[],
): void {
  const starts = [];
  const ends = [];
  for (let i = 1; i < intersections.length; i++) {
    const int = intersections[i];
    const prev = intersections[i - 1];
    if (int.from !== prev.from || int.t === 0 || int.t === 1 || prev.t === 0 || prev.t === 1)
      continue;
    if (i !== 0 && equalPoints(int.point, prev.point) && int.t !== prev.t) {
      // because they are sorted by t, starts we want to inc "forward" the NEXT one; ends we want to dec "back" the PREVIOUS one
      if (int.t <= 0.5) starts.push(int);
      else ends.push(prev);
    }
  }
  // Choose direction as further away from the end point it's closer to.
  for (let i = 0; i < starts.length; i++) {
    const { point, tVec } = starts[i];
    if (tVec.x !== 0) point.x = tVec.x > 0 ? nextUp(point.x, i + 1) : nextDown(point.x, i + 1);
    if (tVec.y !== 0) point.y = tVec.y > 0 ? nextUp(point.y, i + 1) : nextDown(point.y, i + 1);
  }
  for (let i = 0; i < ends.length; i++) {
    const { point, tVec } = ends[i];
    if (tVec.x !== 0) point.x = tVec.x < 0 ? nextUp(point.x, i + 1) : nextDown(point.x, i + 1);
    if (tVec.y !== 0) point.y = tVec.y < 0 ? nextUp(point.y, i + 1) : nextDown(point.y, i + 1);
  }
}
