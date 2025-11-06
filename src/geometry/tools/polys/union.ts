import { PolyPath, buildPathsAndChunks, buildRingIntersectLookup } from './util.js';
import { bboxInside, equalPoints, mergeBBoxes, polygonRingArea } from '../../../index.js';

import type {
  BBox,
  MValue,
  Properties,
  VectorFeatures,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPoint,
} from '../../../index.js';
import type {
  InterPointLookup,
  IntersectionPoint,
  RingChunk,
  RingIntersectionLookup,
} from './util.js';

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

  // not enough data, just clone
  if (vectorPolygons.length === 0) return undefined;

  // 1) build intersections `[polyIndex][ringIndex] -> Intersections`. Store where on the ring other rings intersect
  const ringIntersectLookup: RingIntersectionLookup<D> = buildRingIntersectLookup(vectorPolygons);

  // 2) Build Poly Pieces
  // Setup result paths with chunks that are the final structure of joined polygons.
  // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
  // If no intersections for the polyIndex+RingIndex -> it's immediately consumed into paths. Otherwise it's a chunk
  const [paths, pathLookup, chnks, ints] = buildPathsAndChunks(vectorPolygons, ringIntersectLookup);

  // 3) Consume chunks into PolyPaths
  // If no intersections for the polyIndex+RingIndex -> push as completed ring
  buildPathsFromChunks(paths, pathLookup, ints, chnks);

  // 4) Convert PolyPaths into the resultant MultiPolygon
  let bbox: BBox | undefined;
  for (const { outer, bbox: pathBBox } of paths) {
    if (outer === undefined) continue;
    bbox = mergeBBoxes(bbox, pathBBox) as BBox;
  }
  const coordinates = paths
    .map((p) => p.getPath())
    .filter((p) => p !== undefined) as VectorMultiPolygon<D>;
  if (coordinates.length === 0) return undefined;
  return { type: 'MultiPolygon', coordinates, is3D: false, bbox };
}

/**
 * Given a set of chunks, build a set of paths
 * @param paths - a set of paths to add to
 * @param pathLookup - a lookup of existing paths
 * @param intersections - all intersections
 * @param chunks - a set of chunks
 */
function buildPathsFromChunks<D extends MValue = Properties>(
  paths: PolyPath<D>[],
  pathLookup: Map<number, PolyPath<D>>,
  intersections: InterPointLookup<D>,
  chunks: RingChunk<D>[],
): void {
  // console.log('\n\n\nBUILDING PATHS FROM CHUNKS');
  // for each intersections, connect all the from and to, smallest angle between from->to first slowly work your way through
  for (const xs of Object.values(intersections.lookup)) {
    for (const ys of Object.values(xs)) mergePairs(ys);
  }
  // console.log(
  //   'CHUNKS!',
  //   ...chunks.map((c) => ({
  //     line: c.mid.map((p) => [p.x, p.y]),
  //     from: [c.from.x, c.from.y],
  //     to: [c.to.x, c.to.y],
  //     visited: c.visted,
  //   })),
  // );
  // run through all chunks, if unvisited, add to paths
  for (const chunk of chunks) {
    if (chunk.visted) continue;
    const start = chunk.from;
    // console.log('START', [start.x, start.y]);
    let currChunk = chunk;
    const foundPolygons = new Set<number>();
    const lineString: VectorLineString<D> = [{ ...start }];
    let bbox = currChunk.bbox;
    while (true) {
      if (currChunk.visted) break;
      currChunk.visted = true;
      // console.log(
      //   'ADD MID',
      //   currChunk.mid.map((p) => [p.x, p.y]),
      // );
      lineString.push(...currChunk.mid);
      foundPolygons.add(currChunk.polyIndex);
      bbox = mergeBBoxes(bbox, currChunk.bbox) as BBox;
      if (currChunk.next === undefined) break;
      const { chunk: nextChunk, intPoint } = currChunk.next;
      // console.log('ADD INT', [intPoint.x, intPoint.y]);
      lineString.push({ ...intPoint });
      currChunk = nextChunk;
      if (equalPoints(intPoint, start)) break;
    }
    if (lineString.length < 4 || !equalPoints(lineString.at(0)!, lineString.at(-1)!)) continue;
    // now build the path or add to an existing path
    // console.log('STORE line', lineString);
    const isCCW = polygonRingArea(lineString, 1) > 0;
    // Find the correct PolyPath to insert into, otherwise create a new one, update the lookup to
    // include all new polygon indexes used in the path
    const foundPaths: PolyPath<D>[] = [];
    // Pull in all the old paths to merge with this one (may expand upon multiple paths, consume the holes)
    for (const polyIndex of foundPolygons) {
      const path = pathLookup.get(polyIndex);
      if (path !== undefined) foundPaths.push(path);
    }
    // console.log('foundPolygons', foundPolygons, foundPaths.length);
    let path: PolyPath<D>;
    if (foundPaths.length === 0) {
      path = new PolyPath(lineString, foundPolygons, isCCW, bbox);
      paths.push(path);
    } else if (foundPaths.length === 1) {
      path = foundPaths[0];
      // TODO: If a singular chunk was a hole, we need to specify it may have started as a hole
      addChunkToPath(path, lineString, foundPolygons, bbox, isCCW, chunk.ringIndex !== 0);
    } else {
      path = new PolyPath(lineString, foundPolygons, isCCW, bbox);
      path.consumePaths(foundPaths);
      paths.push(path);
    }
    // All found polyIndex references now point to the new path
    for (const polyIndex of foundPolygons) pathLookup.set(polyIndex, path);
  }
  // TODO: Poly's may still be able to consume eachother
}

/**
 * Add a chunks built into a line+bbox to a path
 * @param path - the path to add to
 * @param ring - the linestring to add
 * @param polyIndexes - all polygon indexes touched
 * @param bbox - the bounding box of the collection of chunks (ring)
 * @param isCCW - whether the ring is CCW
 * @param wasHole - whether the ring is a hole
 */
function addChunkToPath<D extends MValue = Properties>(
  path: PolyPath<D>,
  ring: VectorLineString<D>,
  polyIndexes: Set<number>,
  bbox: BBox,
  isCCW: boolean,
  wasHole: boolean,
): void {
  path.polysConsumed = path.polysConsumed.union(polyIndexes);

  // TODO: Store discarded smaller outer rings, if hole is inside discarded ring, they cancel eachother out
  // If one poly outer ring is entirely in another poly AND its CCW, it gets "consumed" (deleted. path is
  // because of the ordering, the first chunk to be an outer will be the one creating the path,
  // so we know all future CCW chunks that share a path will be "outers" that are inside the existing
  // path outer)
  // If one poly outer ring is entirely in another poly AND its CW, it converts to a hole
  // If one poly inner ring is CW, it gets consumed by an associated outer
  // If one poly inner ring is CCW, remove it
  if (isCCW) {
    if (wasHole) return;
    if (path.outer === undefined) {
      path.outer = ring;
    } else {
      // If the ring's bbox is smaller than the existing outer, store. Otherwise replace
      if (bboxInside(bbox, path.bbox)) {
        path.oldOuters.push(ring);
        return;
      } else {
        path.oldOuters.push(path.outer);
        path.outer = ring;
      }
    }
  } else {
    path.holes.push(ring);
  }

  path.bbox = mergeBBoxes(path.bbox, bbox) as BBox;
}

/**
 * Given an of intersection, find the best way to connect the from->to chunks
 * @param intersection - the intersection to analyze
 */
function mergePairs<D extends MValue = Properties>(intersection: IntersectionPoint<D>): void {
  const { from, to, point: intPoint } = intersection;
  if (from.length === 0 || to.length === 0) return;
  // if only one pair, connect the two chunks and move on
  if (from.length === 1 && to.length === 1) {
    from[0].next = { chunk: to[0], intPoint };
    return;
  }

  // remove "duplicate"/"same" chunks
  const froms: RingChunk<D>[] = [];
  for (const c of from) {
    if (c.visted) continue;
    const exists = froms.some((r) => r.equalChunk(c));
    if (!exists) froms.push(c);
    else {
      // console.log('DUPLICATE!', [c.from.x, c.from.y], c.mid, [c.to.x, c.to.y]);
      c.visted = true;
    }
  }
  const tos: RingChunk<D>[] = [];
  for (const c of to) {
    if (c.visted) continue;
    const exists = tos.some((r) => r.equalChunk(c));
    if (!exists) tos.push(c);
    else {
      // console.log('DUPLICATE 2!', [c.from.x, c.from.y], c.mid, [c.to.x, c.to.y]);
      c.visted = true;
    }
  }

  // console.log('\n\n\n\n');

  const pairs = [];
  for (const f of froms) {
    for (const t of tos) {
      // console.log(
      //   'PAIR',
      //   f.polyIndex,
      //   f.ringIndex,
      //   f.mid.at(-1) ?? f.from,
      //   intPoint,
      //   t.mid.at(0) ?? t.to,
      //   angleRad(f.mid.at(-1) ?? f.from, intPoint, t.mid.at(0) ?? t.to),
      //   // orient2dVector(f.mid.at(-1) ?? f.from, intPoint, t.mid.at(0) ?? t.to),
      //   t.polyIndex,
      //   t.ringIndex,
      //   '\n',
      // );
      const start = f.mid.at(-1) ?? f.from;
      const end = t.mid.at(0) ?? t.to;
      // if (equalPoints(start, intPoint)) console.log('SAME START', start);
      // if (equalPoints(end, intPoint)) console.log('SAME END', end);
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
    // connect the two chunks if neither are visited yet
    // if (intDesired) {
    //   console.log(
    //     'CONNECT!',
    //     [from.from.x, from.from.y],
    //     from.mid.map((p) => [p.x, p.y]),
    //     [intPoint.x, intPoint.y],
    //     to.mid.map((p) => [p.x, p.y]),
    //     [to.to.x, to.to.y],
    //   );
    //   console.log('\n');
    // }
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
