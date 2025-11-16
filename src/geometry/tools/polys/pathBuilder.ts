import { equalLines, equalPoints, fromLineString, mergeBBoxes } from '../../../index.js';

import type {
  BBox,
  MValue,
  Properties,
  RingIntersection,
  RingIntersectionLookup,
  VectorLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPolygon,
} from '../../../index.js';

/** Reconstructing a poly line that interacts with intersections */
export class PolyPath<D extends MValue = Properties> {
  id = 0; // helps down the road to spot duplicate pulls of this Path
  outer?: VectorLineString<D>;
  oldOuters: BBox[] = [];
  holes: VectorLineString<D>[] = [];
  polysConsumed: Set<number> = new Set(); // indexes of the polygons in the multipolygon. So we can quickly consume holes.
  bbox: BBox;
  // eslint-disable-next-line jsdoc/require-jsdoc
  constructor(ring: VectorLineString<D>, polysConsumed: Set<number>, outer: boolean, bbox?: BBox) {
    if (outer) this.outer = ring;
    else this.holes.push(ring);
    this.polysConsumed = polysConsumed;
    this.bbox = bbox ?? (fromLineString(ring) as BBox);
  }
  // eslint-disable-next-line jsdoc/require-jsdoc
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
}

/** The next poly chunk */
export interface NextRingChunk<D extends MValue = Properties> {
  chunk: RingChunk<D>;
  intPoint: VectorPoint<D>;
}

/** A path/piece/chunk from a polygon */
export class RingChunk<D extends MValue = Properties> {
  visted: boolean = false;
  next?: NextRingChunk<D>; // used in final step, to link all chunks together.
  // eslint-disable-next-line jsdoc/require-jsdoc
  constructor(
    public polyIndex: number,
    public ringIndex: number,
    public bbox: BBox,
    public mid: VectorLineString<D>, // Always starts with either the beginning of the poly ring OR an intersection point.
    public from: VectorPoint<D>,
    public to: VectorPoint<D>,
    public fromAngle: number,
    public toAngle: number,
  ) {}
  // eslint-disable-next-line jsdoc/require-jsdoc
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
  // eslint-disable-next-line jsdoc/require-jsdoc
  get(point: VectorPoint<D>): IntersectionPoint<D> {
    return ((this.lookup[point.x] ??= {})[point.y] ??= { point, from: [], to: [] });
  }
  // eslint-disable-next-line jsdoc/require-jsdoc
  linkInts(
    polyIndex: number,
    ringIndex: number,
    from: VectorPoint<D>,
    to: VectorPoint<D>,
    mid: VectorLineString<D>,
    fromAngle?: number,
    toAngle?: number,
  ): RingChunk<D> {
    // first build a chunk
    const bbox = mergeBBoxes(fromLineString(mid), fromLineString([from, to])) as BBox;
    fromAngle = fromAngle ?? angle(mid.at(-1) ?? from, to);
    toAngle = toAngle ?? angle(mid.at(0) ?? to, from);
    const chunk = new RingChunk(polyIndex, ringIndex, bbox, mid, from, to, fromAngle, toAngle);
    this.get(from).to.push(chunk);
    this.get(to).from.push(chunk);
    return chunk;
  }
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
): [PolyPath<D>[], Map<number, PolyPath<D>>, RingChunk<D>[], InterPointLookup<D>, BBox[]] {
  // Setup result. Paths are the final structure of joined polygons.
  const paths: PolyPath<D>[] = [];
  // Lookup is a helper for quickly finding paths in the future
  const pathLookup: Map<number, PolyPath<D>> = new Map();
  // Track all bboxes for all outer-rings
  const outerRingBBoxes: BBox[] = new Array(vectorPolygons.length);

  // 2) Build Poly Pieces
  // If no intersections for the polyIndex+RingIndex -> push as completed ring (into paths)
  const chunks: RingChunk<D>[] = [];
  const intLookup = new InterPointLookup<D>();
  for (let pI = 0; pI < vectorPolygons.length; pI++) {
    const poly = vectorPolygons[pI];
    for (let rI = 0; rI < poly.length; rI++) {
      const ring = poly[rI].map((point) => ({ ...point }));
      let intersections = ringIntersectLookup.get(pI, rI);
      // Case 1: Insert into paths because it's already completed or expand existing path
      if (intersections.length === 0) {
        const existingPath = pathLookup.get(pI);
        if (existingPath === undefined) {
          const path: PolyPath<D> = new PolyPath(ring, new Set([pI]), rI === 0);
          if (rI === 0) outerRingBBoxes[pI] = path.bbox;
          pathLookup.set(pI, path);
          paths.push(path);
        } else {
          if (rI === 0) {
            existingPath.outer = ring;
            existingPath.bbox = mergeBBoxes(existingPath.bbox, fromLineString(ring)) as BBox;
            outerRingBBoxes[pI] = existingPath.bbox;
          } else existingPath.holes.push(ring);
        }
        continue;
      }
      // Case 2: Handle the intersections and build RingChunks
      if (rI === 0) outerRingBBoxes[pI] = fromLineString(ring) as BBox;
      intersections = intersections.filter((i) => i.t !== 0);
      let currIndex = 0;
      let intIndex = 0;
      let curInt: RingIntersection<D> | undefined = intersections.at(intIndex);
      while (currIndex < ring.length - 1) {
        // if we are still working with intersections, build points with them
        if (curInt !== undefined) {
          // until we get to the next intersection, we link the points
          if (currIndex !== curInt.from) {
            const start = currIndex;
            while (currIndex !== curInt.from) currIndex++;
            const mid = ring.slice(start + 1, currIndex);
            chunks.push(intLookup.linkInts(pI, rI, ring[start], ring[currIndex], mid));
          }
          // now build links with the intersections until we get to the next intersection that isn't the same index
          let from = ring[currIndex];
          while (curInt !== undefined && curInt.from === currIndex) {
            if (!equalPoints(from, curInt.point)) {
              // NOTE: For robustness, we have to store the angles we found when studying the intersections.
              // We make decisions about the polygons during the analysis of the intersections using
              // robust predicates. otherwise we would actually compute slightly different angles
              // that could percieve the intersection lines as swapped (non-existent).
              const ang = curInt.tAngle;
              chunks.push(
                intLookup.linkInts(pI, rI, from, curInt.point, [], invertAngle(ang), ang),
              );
            }
            intIndex++;
            from = curInt.point;
            curInt = intersections.at(intIndex);
          }
          // if the intersection t is not 1, then we need to link the point to the end of the currIndex
          const next = ring[currIndex + 1];
          if (!equalPoints(from, next)) {
            const { tAngle } = intersections[intIndex - 1];
            chunks.push(intLookup.linkInts(pI, rI, from, next, [], invertAngle(tAngle), tAngle));
          }
        } else {
          // no intersection, just build the point
          chunks.push(intLookup.linkInts(pI, rI, ring[currIndex], ring[currIndex + 1], []));
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

  return [paths, pathLookup, chunks, intLookup, outerRingBBoxes];
}

/**
 * Given an of intersection, find the best way to connect the from->to chunks
 * @param intersection - the intersection to analyze
 */
export function mergeIntersectionPairs<D extends MValue = Properties>(
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
    if (!froms.some((r) => r.equalChunk(c))) froms.push(c);
    else c.visted = true;
  }
  const tos: RingChunk<D>[] = [];
  for (const c of to) {
    if (c.visted) continue;
    if (!tos.some((r) => r.equalChunk(c))) tos.push(c);
    else c.visted = true;
  }

  const pairs = [];
  for (const f of froms) {
    for (const t of tos) {
      const angle = t.toAngle - f.fromAngle;
      pairs.push({ from: f, to: t, angle: angle < 0 ? angle + Math.PI * 2 : angle });
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
 * @param b - Second Point
 * @returns Angle in degrees [-PI, PI]
 */
function angle<D extends MValue = Properties>(a: VectorPoint<D>, b: VectorPoint<D>): number {
  return Math.atan2(a.y - b.y, a.x - b.x);
}

/**
 * Returns the absolute angle between points A->B->C
 * @param angle - Angle in degrees [-PI, PI]
 * @returns Angle in degrees [-PI, PI]
 */
function invertAngle(angle: number): number {
  return angle >= 0 ? angle - Math.PI : angle + Math.PI;
}
