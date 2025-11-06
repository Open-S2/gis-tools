import {
  BoxIndex,
  bboxInside,
  buildPolygonSegments,
  equalLines,
  equalPoints,
  findPolygonIntersections,
  fromLineString,
  mergeBBoxes,
} from '../../../index.js';

import type {
  BBox,
  BoxIndexAccessor,
  MValue,
  Properties,
  Segment,
  VectorLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPolygon,
} from '../../../index.js';

/** Reconstructing a poly line that interacts with intersections */
export class PolyPath<D extends MValue = Properties> {
  outer?: VectorLineString<D>;
  oldOuters: VectorLineString<D>[] = [];
  holes: VectorLineString<D>[] = [];
  polysConsumed: Set<number> = new Set(); // indexes of the polygons in the multipolygon. So we can quickly consume holes.
  bbox: BBox;
  /**
   * @param ring - the linestring
   * @param polysConsumed - the collection of polygons consumed
   * @param outer - whether the ring is the outer ring
   * @param bbox - if provided, the bounding box
   */
  constructor(ring: VectorLineString<D>, polysConsumed: Set<number>, outer: boolean, bbox?: BBox) {
    if (outer) this.outer = ring;
    else this.holes.push(ring);
    this.polysConsumed = polysConsumed;
    this.bbox = bbox ?? (fromLineString(ring) as BBox);
  }

  /**
   * Get the path as a vector polygon
   * @returns - the resultant poly if it exists
   */
  getPath(): VectorPolygon | undefined {
    if (this.outer === undefined) return undefined;
    if (this.outer.length < 4) return undefined;
    const res = [this.outer];
    for (const hole of this.holes) {
      if (hole.length < 4) continue;
      res.push(hole);
    }
    return res;
  }

  /**
   * Merge in a collection of paths
   * @param others - the collection of paths
   */
  consumePaths(others: PolyPath<D>[]): void {
    for (const other of others) {
      // If this bbox is smaller than the existing outer, replace
      if (other.outer !== undefined && bboxInside(this.bbox, other.bbox)) {
        if (this.outer !== undefined) this.oldOuters.push(this.outer);
        this.outer = other.outer;
      }
      this.holes.push(...other.holes);
      this.polysConsumed = this.polysConsumed.union(other.polysConsumed);
      this.bbox = mergeBBoxes(this.bbox, other.bbox) as BBox;
      // clear the path now that we comsumed it
      other.outer = undefined;
      other.holes = [];
    }
  }
}

/** The next poly chunk */
export interface NextRingChunk<D extends MValue = Properties> {
  chunk: RingChunk<D>;
  intPoint: VectorPoint<D>;
}

/** A path/piece/chunk from a polygon */
export class RingChunk<D extends MValue = Properties> {
  visted: boolean = false;
  // isHole: boolean = false;
  next?: NextRingChunk<D>; // used in final step, to link all chunks together.
  /**
   * @param polyIndex - the index of the polygon
   * @param ringIndex - the index of the ring
   * @param bbox - the bounding box
   * @param mid - the linestring
   * @param from - from point
   * @param to - to point
   */
  constructor(
    public polyIndex: number,
    public ringIndex: number,
    public bbox: BBox,
    public mid: VectorLineString<D>, // Always starts with either the beginning of the poly ring OR an intersection point.
    public from: VectorPoint<D>,
    public to: VectorPoint<D>,
  ) {}

  /**
   * Check if two chunks are equal
   * @param other - the other chunk
   * @returns - true if the two chunks are equal
   */
  equalChunk(other: RingChunk<D>): boolean {
    return (
      this.ringIndex > 0 === other.ringIndex > 0 &&
      equalPoints(this.from, other.from) &&
      equalPoints(this.to, other.to) &&
      equalLines(this.mid, other.mid)
    );
  }
}

/** Intersection Point */
export interface IntersectionPoint<D extends MValue = Properties> {
  point: VectorPoint<D>;
  from: RingChunk<D>[];
  to: RingChunk<D>[];
}

/** Intersection Lookup for chunks */
export class InterPointLookup<D extends MValue = Properties> {
  lookup: { [x: number]: { [y: number]: IntersectionPoint<D> } } = {};
  /**
   * Get the intersection point
   * @param point - the intersection point
   * @returns - the intersection point, creates if it doesn't exist
   */
  get(point: VectorPoint<D>): IntersectionPoint<D> {
    return ((this.lookup[point.x] ??= {})[point.y] ??= { point, from: [], to: [] });
  }

  /**
   * Link two points to eachother
   * @param polyIndex - the index of the polygon
   * @param ringIndex - the index of the ring
   * @param from - the from intersection point
   * @param to - the to intersection point
   * @param mid - if provided the linestring
   * @returns the created chunk
   */
  linkInts(
    polyIndex: number,
    ringIndex: number,
    from: VectorPoint<D>,
    to: VectorPoint<D>,
    mid: VectorLineString<D> = [],
  ): RingChunk<D> {
    // first build a chunk
    const bbox = mergeBBoxes(fromLineString(mid), fromLineString([from, to])) as BBox;
    const chunk = new RingChunk(polyIndex, ringIndex, bbox, mid, from, to);
    const fromPoint = this.get(from);
    const toPoint = this.get(to);
    fromPoint.to.push(chunk);
    toPoint.from.push(chunk);
    return chunk;
  }
}

/** Local Intersection to a [polyIndex][ringIndex] */
interface RingIntersection<D extends MValue = Properties> {
  from: number;
  to: number;
  point: VectorPoint<D>;
  t: number;
}
/** [polyIndex][ringIndex] -> Intersections */
export type RingIntersectionLookup<D extends MValue = Properties> = Record<
  number,
  Record<number, RingIntersection<D>[]>
>;

/**
 * Run through the vectorPolygons and Builds the ring intersection lookup
 * @param vectorPolygons - the collection of polygons
 * @param segmentFilter -  the function to filter the segments, default ignores self intersections
 * @returns - the ring intersection lookup for all rings in the multipolygon collection
 */
export function buildRingIntersectLookup<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
  segmentFilter?: (seg1: Segment) => { (seg2: Segment): boolean },
): RingIntersectionLookup<D> {
  const segments = buildPolygonSegments(vectorPolygons);
  const ringIntersectLookup: RingIntersectionLookup<D> = {};

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
        const { point, u, t } = pInt;
        // skip if u and t are equal
        if (u === t && (u === 0 || u === 1)) continue;
        // first segment intersection
        const s1 = ((ringIntersectLookup[s1pi] ??= {})[s1ri] ??= []);
        s1.push({ from: s1f, to: s1t, point, t: u });
        // second segment intersection
        const s2 = ((ringIntersectLookup[s2pi] ??= {})[s2ri] ??= []);
        s2.push({ from: s2f, to: s2t, point, t });
      }
    }
  }

  return ringIntersectLookup;
}

/**
 * Build the PolyPaths and RingChunks
 * @param vectorPolygons - the collection of polygons
 * @param ringIntersectLookup - the ring intersection lookup for all rings in the multipolygon collection
 * @returns - the PolyPaths, their lookups, and RingChunks
 */
export function buildPathsAndChunks<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
  ringIntersectLookup: RingIntersectionLookup<D>,
): [PolyPath<D>[], Map<number, PolyPath<D>>, RingChunk<D>[], InterPointLookup<D>] {
  // Setup result. Paths are the final structure of joined polygons.
  const paths: PolyPath<D>[] = [];
  // Lookup is a helper for quickly finding paths in the future
  const pathLookup: Map<number, PolyPath<D>> = new Map();

  // 2) Build Poly Pieces
  // If no intersections for the polyIndex+RingIndex -> push as completed ring (into paths)
  const chunks: RingChunk<D>[] = [];
  const intLookup = new InterPointLookup<D>();
  for (let pI = 0; pI < vectorPolygons.length; pI++) {
    const poly = vectorPolygons[pI];
    for (let rI = 0; rI < poly.length; rI++) {
      const ring = poly[rI].map((point) => ({ ...point }));
      let intersections = cleanIntersections(ringIntersectLookup[pI]?.[rI] ?? []);
      // Case 1: Insert into paths because it's already completed or expand existing path
      if (intersections.length === 0) {
        const existingPath = pathLookup.get(pI);
        if (existingPath === undefined) {
          const path: PolyPath<D> = new PolyPath(ring, new Set([pI]), rI === 0);
          pathLookup.set(pI, path);
          paths.push(path);
        } else {
          if (rI === 0) existingPath.outer = ring;
          else existingPath.holes.push(ring);
          existingPath.bbox = mergeBBoxes(existingPath.bbox, fromLineString(ring)) as BBox;
        }
        continue;
      }
      // Case 2: Handle the intersections and build RingChunks
      intersections = intersections.filter((i) => i.t !== 0);
      let currIndex = 0;
      let intIndex = 0;
      let curInt: RingIntersection<D> | undefined = intersections.at(intIndex);
      while (currIndex < ring.length - 1) {
        // console.log('curInt', curInt);
        // if we are still working with intersections, build points with them
        if (curInt !== undefined) {
          // until we get to the next intersection, we link the points
          if (currIndex !== curInt.from) {
            const start = currIndex;
            while (currIndex !== curInt.from) {
              currIndex++;
            }
            const mid = ring.slice(start + 1, currIndex);
            chunks.push(intLookup.linkInts(pI, rI, ring[start], ring[currIndex], mid));
            // console.log(
            //   'LINK 1',
            //   [ring[start].x, ring[start].y],
            //   ring.slice(start + 1, currIndex).map((p) => [p.x, p.y]),
            //   [ring[currIndex].x, ring[currIndex].y],
            // );
          }
          // now build links with the intersections until we get to the next intersection that isn't the same index
          let from = ring[currIndex];
          while (curInt !== undefined && curInt.from === currIndex) {
            if (!equalPoints(from, curInt.point))
              chunks.push(intLookup.linkInts(pI, rI, from, curInt.point));
            // console.log('LINK 2', [from.x, from.y], [curInt.point.x, curInt.point.y], curInt.t);
            intIndex++;
            from = curInt.point;
            curInt = intersections.at(intIndex);
          }
          // if the intersection t is not 1, then we need to link the point to the end of the currIndex
          // if ((curInt === undefined ? intersections[intIndex - 1].t : curInt.t) !== 1) {
          if (!equalPoints(from, ring[currIndex + 1])) {
            chunks.push(intLookup.linkInts(pI, rI, from, ring[currIndex + 1]));
            // console.log(
            //   'LINK 2.2',
            //   [from.x, from.y],
            //   [ring[currIndex + 1].x, ring[currIndex + 1].y],
            //   curInt?.t,
            // );
          }
        } else {
          // no intersection, just build the point
          chunks.push(intLookup.linkInts(pI, rI, ring[currIndex], ring[currIndex + 1]));
          // console.log(
          //   'LINK 3',
          //   [ring[currIndex].x, ring[currIndex].y],
          //   [ring[currIndex + 1].x, ring[currIndex + 1].y],
          // );
        }
        currIndex++;
      }
    }
  }

  // sort chunks by left then bottom for the eventual final run through
  chunks.sort((a, b) => {
    let diff = a.bbox[0] - b.bbox[0];
    if (diff === 0) diff = a.bbox[1] - b.bbox[1];
    return diff;
  });

  return [paths, pathLookup, chunks, intLookup];
}

/**
 * Given a ring's of intersections, clean them up
 * @param intersections - a collection of intersections to clean up
 * @returns - the cleaned up intersections
 */
function cleanIntersections<D extends MValue = Properties>(
  intersections: RingIntersection<D>[],
): RingIntersection<D>[] {
  intersections.sort((a, b) => {
    let diff = a.from - b.from;
    if (diff === 0) diff = a.t - b.t;
    return diff;
  });

  // 1) Remove duplicates
  const dedupInts: RingIntersection<D>[] = [];
  for (let i = 0; i < intersections.length; i++) {
    const int = intersections[i];
    // c.otherPI === int.otherPI &&
    // c.otherRI === int.otherRI,
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

  return dedupInts;
}

/**
 * Given an of intersection, find the best way to connect the from->to chunks
 * @param intersection - the intersection to analyze
 */
export function mergePairs<D extends MValue = Properties>(
  intersection: IntersectionPoint<D>,
): void {
  const { from, to, point: intPoint } = intersection;
  if (from.length === 0 || to.length === 0) return;
  if (from.length === 1 && to.length === 1) {
    // connect the two chunks and move on
    from[0].next = { chunk: to[0], intPoint };
    return;
  }

  // remove "duplicate"/"same" chunks
  const froms: RingChunk<D>[] = [];
  for (const c of from) {
    if (c.visted) continue;
    const exists = froms.some((r) => r.equalChunk(c));
    if (!exists) froms.push(c);
    else c.visted = true;
  }
  const tos: RingChunk<D>[] = [];
  for (const c of to) {
    if (c.visted) continue;
    const exists = tos.some((r) => r.equalChunk(c));
    if (!exists) tos.push(c);
    else c.visted = true;
  }

  const pairs = [];
  for (const f of froms) {
    for (const t of tos) {
      const start = f.mid.at(-1) ?? f.from;
      const end = t.mid.at(0) ?? t.to;
      const angle = angleRad(start, intPoint, end);
      pairs.push({
        from: f,
        to: t,
        angle,
      });
    }
  }
  pairs.sort((a, b) => a.angle - b.angle);

  for (const { from, to } of pairs) {
    if (from.visted || to.visted) continue;
    from.next = { chunk: to, intPoint };
    from.visted = true;
    to.visted = true;
  }

  // cleanup visited
  for (const f of froms) f.visted = false;
  for (const t of tos) t.visted = false;
}

/**
 * Returns the absolute angle between points A->B->C
 * @param a - First point
 * @param b - Vertex point (angle at this point)
 * @param c - Third point
 * @returns Angle in degrees [0, 2*PI]
 */
function angleRad<D extends MValue = Properties>(
  a: VectorPoint<D>,
  b: VectorPoint<D>,
  c: VectorPoint<D>,
): number {
  const { atan2, PI } = Math;
  const twoPI = PI * 2;

  // If b->c this algo considers this a full revolution, not 0
  if (equalPoints(b, c)) return twoPI;

  const angleBA = atan2(a.y - b.y, a.x - b.x);
  const angleBC = atan2(c.y - b.y, c.x - b.x);
  // Difference in radians
  const angle = angleBC - angleBA;
  return angle < 0 ? angle + twoPI : angle;
}
