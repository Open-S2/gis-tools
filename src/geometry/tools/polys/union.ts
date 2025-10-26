import {
  BoxIndex,
  bboxInside,
  buildPolygonSegments,
  equalPoints,
  findPolygonIntersections,
  fromLineString,
  fromMultiPolygon,
  mergeBBoxes,
  polygonRingArea,
} from '../../../index.js';

import type {
  BBox,
  BoxIndexAccessor,
  MValue,
  Properties,
  Segment,
  VectorFeatures,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
} from '../../../index.js';

/** Reconstructing a poly line that interacts with intersections */
export class PolyPath<D extends MValue = Properties> {
  outer?: VectorLineString<D>;
  holes: VectorLineString<D>[] = [];
  polysConsumed: Set<number> = new Set(); // indexes of the polygons in the multipolygon. So we can quickly consume holes.
  bbox?: BBox;
  /**
   * @param ring - the linestring
   * @param polyIndex - the index of the polygon
   * @param outer - whether the ring is the outer ring
   */
  constructor(ring: VectorLineString<D>, polyIndex: number, outer: boolean) {
    if (outer) this.outer = ring;
    else this.holes.push(ring);
    this.polysConsumed.add(polyIndex);
    this.bbox = mergeBBoxes(this.bbox, fromLineString(ring)) as BBox;
  }

  /**
   * Add a collection of chunks built into a line+bbox to the path
   * @param ring - the linestring to add
   * @param polyIndexes - all polygon indexes touched
   * @param bbox - the bounding box of the collection of chunks (ring)
   * @param isCCW - whether the ring is CCW
   * @param wasHole - whether the ring is a hole
   */
  addChunks(
    ring: VectorLineString<D>,
    polyIndexes: Set<number>,
    bbox: BBox,
    isCCW: boolean,
    wasHole: boolean,
  ): void {
    this.polysConsumed = this.polysConsumed.union(polyIndexes);

    // If one poly outer ring is entirely in another poly AND its CCW, it gets "consumed" (deleted. this is
    // because of the ordering, the first chunk to be an outer will be the one creating the path,
    // so we know all future CCW chunks that share a path will be "outers" that are inside the existing
    // path outer)
    // If one poly outer ring is entirely in another poly AND its CW, it converts to a hole
    // If one poly inner ring is CW, it gets consumed by an associated outer
    // If one poly inner ring is CCW, remove it
    if (isCCW) {
      if (wasHole) return;
      if (this.outer === undefined) {
        this.outer = ring;
      } else {
        // If this bbox is smaller than the existing outer, delete. Otherwise replace
        if (bboxInside(bbox, this.bbox!)) {
          return;
        } else {
          this.outer = ring;
        }
      }
    } else {
      this.holes.push(ring);
    }

    this.bbox = mergeBBoxes(this.bbox, bbox) as BBox;
  }
}

/** A path/piece/chunk from a polygon */
export interface PolyChunk<D extends MValue = Properties> {
  visted: boolean;
  polyIndex: number;
  ringIndex: number;
  bbox: BBox;
  line: VectorLineString<D>; // Always stars with either the beginning of the poly ring OR an intersection point.
  next: IntersectionPoint<D>; // can point to just one or multiple chunks. Many polys can touch the same point. If none provided could be a start-end point
}

/** Local Intersection to a [polyIndex][ringIndex] */
export interface RingIntersection<D extends MValue = Properties> {
  from: number; // index in the polys[polygon][ring][from]
  to: number; // index in the polys[polygon][ring][to]
  point: VectorPoint<D>;
}
/** [polyIndex][ringIndex] -> Intersections */
export type RingIntersectionLookup<D extends MValue = Properties> = Record<
  number,
  Record<number, RingIntersection<D>[]>
>;

/** Intersection Point */
export interface IntersectionPoint<D extends MValue = Properties> {
  point: VectorPoint<D>;
  chunks: PolyChunk<D>[]; // reference to all chunks
}

/**
 * Given a collection of polygons, if any of the polygons interact, merge them as a union
 * @param polygons - the polygons are from either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @returns - a union of polygons should a union exist.
 */
export function polygonsUnion<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygons:
    | VectorMultiPolygon<D>
    | VectorMultiPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorMultiPolygonGeometry<D>>,
): VectorMultiPolygonGeometry<D> | undefined {
  const vectorPolygons: VectorMultiPolygon<D> =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;
  // bbox
  let bbox =
    'geometry' in polygons
      ? polygons.geometry.bbox
      : 'coordinates' in polygons
        ? polygons.bbox
        : undefined;
  if (bbox === undefined) bbox = fromMultiPolygon(vectorPolygons);

  // not enough data, just clone
  if (vectorPolygons.length < 2) {
    // clone coords
    const coordinates = vectorPolygons.map((p) => p.map((r) => r.map((p) => ({ ...p }))));
    return { type: 'MultiPolygon', coordinates, is3D: false, bbox };
  }

  // 1) build intersections `[polyIndex][ringIndex] -> Intersections`. Store where on the ring other rings intersect
  const ringIntersectLookup: RingIntersectionLookup<D> = buildRingIntersectLookup(vectorPolygons);

  // 2) Build Poly Pieces
  // Setup result paths with chunks that are the final structure of joined polygons.
  // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
  // If no intersections for the polyIndex+RingIndex -> it's immediately consumed into paths. Otherwise it's a chunk
  const [paths, pathLookup, chunks] = buildPathsAndChunks(vectorPolygons, ringIntersectLookup);

  // 3) Consume chunks into PolyPaths
  // If no intersections for the polyIndex+RingIndex -> push as completed ring
  buildPathsFromChunks(chunks, pathLookup, paths);

  // 4) Convert PolyPaths into the resultant MultiPolygon
  if (paths.length === 0) return;
  const coordinates = paths
    .map((p) => (p.outer !== undefined ? [p.outer, ...p.holes] : undefined))
    .filter((p) => p !== undefined);
  return { type: 'MultiPolygon', coordinates, is3D: false, bbox };
}

/**
 * Run through the vectorPolygons and Builds the ring intersection lookup
 * @param vectorPolygons - the collection of polygons
 * @returns - the ring intersection lookup for all rings in the multipolygon collection
 */
function buildRingIntersectLookup<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
): RingIntersectionLookup<D> {
  const segments = buildPolygonSegments(vectorPolygons);
  const ringIntersectLookup: RingIntersectionLookup<D> = {};

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
    const segIsOuter = segment1.ringIndex === 0;
    const potentialIntersections = boxIndex.search(
      ...getBounds(segment1),
      (seg: Segment) =>
        // if same id ignore
        seg.id !== segment1.id &&
        // only pass forward not backward
        seg.id > segment1.id &&
        // if same polyIndex ignore
        seg.polyIndex !== segment1.polyIndex &&
        // only pass if both inner or if both outer.
        ((seg.ringIndex === 0 && segIsOuter) || (seg.ringIndex !== 0 && !segIsOuter)),
    );
    for (const segment2 of potentialIntersections) {
      const pInt = findPolygonIntersections<D>(vectorPolygons, segment1, segment2);
      // ignore points that interact tangentially or precisely at an existing edge or vertex.
      if (pInt !== undefined) {
        // first segment intersection
        const s1 = ((ringIntersectLookup[segment1.polyIndex] ??= {})[segment1.ringIndex] ??= []);
        s1.push({ from: segment1.from, to: segment1.to, point: pInt.point });
        // second segment intersection
        const s2 = ((ringIntersectLookup[segment2.polyIndex] ??= {})[segment2.ringIndex] ??= []);
        s2.push({ from: segment2.from, to: segment2.to, point: pInt.point });
      }
    }
  }

  return ringIntersectLookup;
}

/**
 * Build the PolyPaths and PolyChunks
 * @param vectorPolygons - the collection of polygons
 * @param ringIntersectLookup - the ring intersection lookup for all rings in the multipolygon collection
 * @returns - the PolyPaths, their lookups, and PolyChunks
 */
function buildPathsAndChunks<D extends MValue = Properties>(
  vectorPolygons: VectorMultiPolygon<D>,
  ringIntersectLookup: RingIntersectionLookup<D>,
): [PolyPath<D>[], Map<number, PolyPath<D>>, PolyChunk<D>[]] {
  // Setup result. Paths are the final structure of joined polygons.
  const paths: PolyPath<D>[] = [];
  // Lookup is a helper for quickly finding paths in the future
  const pathLookup: Map<number, PolyPath<D>> = new Map();

  // 2) Build Poly Pieces
  // If no intersections for the polyIndex+RingIndex -> push as completed ring (into paths)
  const chunks: PolyChunk<D>[] = [];
  const interPointLookup: { [x: number]: { [y: number]: IntersectionPoint<D> } } = {};
  for (let polyIndex = 0; polyIndex < vectorPolygons.length; polyIndex++) {
    const poly = vectorPolygons[polyIndex];
    for (let ringIndex = 0; ringIndex < poly.length; ringIndex++) {
      const ring = poly[ringIndex].map((point) => ({ ...point }));
      const intersections = ringIntersectLookup[polyIndex]?.[ringIndex];
      // Case 1: Insert into paths because it's already completed or expand existing path
      if (intersections === undefined || intersections.length === 0) {
        const existingPath = pathLookup.get(polyIndex);
        if (existingPath === undefined) {
          const path: PolyPath<D> = new PolyPath(ring, polyIndex, ringIndex === 0);
          pathLookup.set(polyIndex, path);
          paths.push(path);
        } else {
          existingPath.polysConsumed.add(polyIndex);
          if (ringIndex === 0) existingPath.outer = ring;
          else existingPath.holes.push(ring);
        }
        continue;
      }
      // Case 2: Insert into chunks for further processing
      // ensure we split the full ring in order
      intersections.sort((a, b) => a.from - b.from);
      let currIndex = 0;
      let currIntP: VectorPoint<D> | undefined;
      for (const { from, to, point: nextIntP } of intersections) {
        // TODO: Sometimes we want tangential intersections. Build a test case to ensure this works
        // skip points that interact tangentially or precisely at ends of the ring
        if (
          (from === 0 && equalPoints(nextIntP, ring[0])) ||
          (to === ring.length - 1 && equalPoints(nextIntP, ring[to]))
        ) {
          continue;
        }
        // build the chunk's line
        const line = currIntP !== undefined ? [{ ...currIntP }] : [];
        line.push(...ring.slice(currIndex, from + 1)); // include to from.
        // add to the lookup if needed otherwise grab the existing one
        const intrP = ((interPointLookup[nextIntP.x] ??= {})[nextIntP.y] ??= {
          point: nextIntP,
          chunks: [],
        });
        // build the chunk and point it to the next intersection "point"
        const bbox = fromLineString(line) as BBox;
        const chunk = { visted: false, polyIndex, ringIndex, bbox, line, next: intrP };
        // Place this chunk in the lookup where it began if it started at an intersection, otherwise
        // the "intersection" is the start of the ring
        const startPoint = currIntP !== undefined ? currIntP : ring[0];
        const intrPS = ((interPointLookup[startPoint.x] ??= {})[startPoint.y] ??= {
          point: startPoint,
          chunks: [],
        });
        intrPS.chunks.push(chunk);
        chunks.push(chunk);
        // update current
        currIntP = nextIntP;
        currIndex = to;
      }
      // lastly if we have an open ring add it
      if (currIndex !== ring.length) {
        const line = currIntP !== undefined ? [{ ...currIntP }] : [];
        line.push(...ring.slice(currIndex));
        const bbox = fromLineString(line) as BBox;
        // more than likely previous intersection is the start of the ring
        const startPoint = currIntP !== undefined ? currIntP : ring[0];
        const intrPS = ((interPointLookup[startPoint.x] ??= {})[startPoint.y] ??= {
          point: startPoint,
          chunks: [],
        });
        // gaurenteed the beginning of the ring
        const intrPF = ((interPointLookup[ring[0].x] ??= {})[ring[0].y] ??= {
          point: ring[0],
          chunks: [],
        });
        intrPS.chunks.push({ visted: false, polyIndex, ringIndex, bbox, line, next: intrPF });
      }
    }
  }
  // sort the chunks by leftmost bboxes then bottom most
  chunks.sort((a, b) => {
    let diff = a.bbox[0] - b.bbox[0];
    if (diff === 0) diff = a.bbox[1] - b.bbox[1];
    return diff;
  });

  return [paths, pathLookup, chunks];
}

/**
 * Given a set of chunks, build a set of paths
 * @param chunks - a set of chunks
 * @param pathLookup - a lookup of existing paths
 * @param paths - a set of paths to add to
 */
function buildPathsFromChunks<D extends MValue = Properties>(
  chunks: PolyChunk<D>[],
  pathLookup: Map<number, PolyPath<D>>,
  paths: PolyPath<D>[],
) {
  for (const chunk of chunks) {
    if (chunk.visted) continue;
    // follow along a chunk until we find our start point again
    const start = chunk.line[0];
    // console.log('STARTING NEW CHUNK!!!!', start);

    const mergedChunks: PolyChunk<D>[] = [];
    let currChunk = chunk;
    const foundPolygons = new Set<number>();
    while (true) {
      // add the chunk and mark it as visited
      currChunk.visted = true;
      // console.log('ADDING CHUNK', currChunk.line);
      mergedChunks.push(currChunk);
      foundPolygons.add(currChunk.polyIndex);
      // if the current chunk ends at the start, we are done.
      if (
        mergedChunks.length > 1 &&
        equalPoints(start, currChunk.line[currChunk.line.length - 1])
      ) {
        // console.log('DONE CHUNKS - ', currChunk.next === undefined);
        break;
      }
      // two directions now:
      // A) if the next intersection is the start, we are done
      const { chunks, point } = currChunk.next;
      if (equalPoints(point, start)) {
        // console.log('FOUND START', point, start);
        currChunk.line.push({ ...point });
        break;
      }
      // B) Grab the needed chunks from ringIntersectLookup, filter by visited, grab the chunk that is the most counter-clockwise with where we are.
      const unusedChunks = chunks.filter((c) => !c.visted);
      // console.log(
      //   'UNUSED CHUNKS for ',
      //   point,
      //   unusedChunks.map((c) => c.line),
      // );
      if (unusedChunks.length === 0) break; // failure case
      // For all unusedChunks, find the one that continues the chunks as most counter-clockwise as possible
      // Using chunkEnd->point->unusedChunk.line[1]
      const chunkEnd = currChunk.line[currChunk.line.length - 1];
      const nextChunk = maximumAngle<D>(chunkEnd, point, unusedChunks);
      if (nextChunk === undefined) break; // failure case
      // console.log('FOUND NEXT CHUNK', nextChunk.line);
      currChunk = nextChunk;
    }
    // console.log('MERGED CHUNKS');
    // Ensure mergedChunks starts and ends at the same point, otherwise drop
    const first = mergedChunks[0].line[0];
    const lastChunk = mergedChunks[mergedChunks.length - 1];
    const last = lastChunk.line[lastChunk.line.length - 1];
    if (!equalPoints(first, last)) {
      // console.log('DROPPING CHUNK!');
      continue;
    }
    // Convert mergedChunks to a ring and find the orientation
    const lineString: VectorLineString<D> = mergedChunks.flatMap((c) =>
      c.line.map((p) => ({ ...p })),
    );
    if (lineString.length < 4) continue;
    // console.log('CONVERTING TO LINESTRING', lineString);
    let bbox = mergedChunks[0].bbox;
    for (let i = 1; i < mergedChunks.length; i++)
      bbox = mergeBBoxes(bbox, mergedChunks[i].bbox) as BBox;
    const isCCW = polygonRingArea(lineString, 1) > 0;
    // Find the correct PolyPath to insert into, otherwise create a new one, update the lookup to include all new polygon indexes used in the path
    let path: PolyPath<D> | undefined;
    for (const polyIndex of foundPolygons) {
      path = pathLookup.get(polyIndex);
      if (path !== undefined) break;
    }
    if (path === undefined) {
      // console.log('NEW PATH');
      path = new PolyPath(lineString, mergedChunks[0].polyIndex, isCCW);
      paths.push(path);
    } else {
      // console.log('EXISTING PATH - ADD RING');
      path.addChunks(lineString, foundPolygons, bbox, isCCW, mergedChunks[0].ringIndex !== 0);
    }
    // Update the lookup and consumed polygons found
    for (const polyIndex of foundPolygons) pathLookup.set(polyIndex, path);
  }
}

/**
 * Returns the PolyChunk with the largest angle relative to A->B->Chunk
 * @param a - starting point
 * @param b - pivot point
 * @param chunks - list of chunks to choose from
 * @returns - the chunk with the largest angle
 */
function maximumAngle<D extends MValue = Properties>(
  a: VectorPoint,
  b: VectorPoint,
  chunks: PolyChunk<D>[],
): PolyChunk<D> | undefined {
  // TODO: Handle cases where line[1] is undefined
  // TODO: Might have to do angle < maxAngle if hole and not outer-ring
  // TODO: What if ange is equal? Do I choose shorter?
  let maxAngle = 0;
  let maxChunk: PolyChunk<D> | undefined;

  for (const chunk of chunks) {
    const c = chunk.line.at(1);
    const angle = angleRad(a, b, c);
    if (maxChunk === undefined || angle > maxAngle) {
      maxAngle = angle;
      maxChunk = chunk;
    }
    // else if (angle === maxAngle) {
    //   const chunkLength = len(a, c ?? b);
    //   const maxChunkLength = len(a, maxChunk.line.at(1) ?? b);
    //   if (chunkLength < maxChunkLength) {
    //     maxAngle = angle;
    //     maxChunk = chunk;
    //   }
    // }
  }

  return maxChunk;
}

// /**
//  * Get the length between two points
//  * @param a - First point
//  * @param b - Second point
//  * @returns Length in euclidean space
//  */
// function len(a: VectorPoint, b: VectorPoint): number {
//   return Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2);
// }

/**
 * Returns the absolute angle between points A->B->C
 * @param a - First point
 * @param b - Vertex point (angle at this point)
 * @param c - Third point
 * @returns Angle in degrees [-PI, PI]
 */
function angleRad(a: VectorPoint, b: VectorPoint, c?: VectorPoint): number {
  if (c === undefined) return 0;
  const { atan2, PI } = Math;

  const angleBA = atan2(a.y - b.y, a.x - b.x);
  const angleBC = atan2(c.y - b.y, c.x - b.x);

  // Difference in radians
  const angle = angleBA - angleBC;
  return angle < 0 ? angle + 2 * PI : angle;
}
