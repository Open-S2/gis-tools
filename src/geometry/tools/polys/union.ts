import {
  PolyPath,
  bboxInside,
  buildPathsAndChunks,
  equalPoints,
  mergeBBoxes,
  mergeIntersectionPairs,
  polygonRingArea,
  polygonsIntersectionsLookup,
} from '../../../index.js';

import type {
  BBox,
  InterPointLookup,
  MValue,
  Properties,
  RingChunk,
  RingIntersectionLookup,
  VectorFeatures,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
} from '../../../index.js';

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
  const ringIntersectLookup: RingIntersectionLookup<D> =
    polygonsIntersectionsLookup(vectorPolygons);

  // 2) Build Poly Pieces
  // Setup result paths with chunks that are the final structure of joined polygons.
  // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
  // If no intersections for the polyIndex+RingIndex -> it's immediately consumed into paths. Otherwise it's a chunk
  const [paths, pathLookup, chnks, ints, bboxes] = buildPathsAndChunks(
    vectorPolygons,
    ringIntersectLookup,
  );

  // 3) Consume chunks into PolyPaths
  // If no intersections for the polyIndex+RingIndex -> push as completed ring
  buildPathsFromChunks(paths, pathLookup, ints, chnks, bboxes);

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
 * @param bboxes - the bboxes of all polygons outer rings
 */
function buildPathsFromChunks<D extends MValue = Properties>(
  paths: PolyPath<D>[],
  pathLookup: Map<number, PolyPath<D>>,
  intersections: InterPointLookup<D>,
  chunks: RingChunk<D>[],
  bboxes: BBox[],
): void {
  // merge in all potential "dead" outer-rings (do we need this?)
  for (const path of paths) storeInnerOldOuters(path, bboxes);
  // for each intersections, connect all the from and to, smallest angle between from->to first slowly work your way through
  for (const xs of Object.values(intersections.lookup)) {
    for (const ys of Object.values(xs)) mergeIntersectionPairs(ys);
  }
  // run through all chunks, if unvisited, add to paths
  for (const chunk of chunks) {
    if (chunk.visted) continue;
    const start = chunk.from;
    // console.log(`START: `, start);
    let currChunk = chunk;
    const foundPolygons = new Set<number>();
    const lineString: VectorLineString<D> = [{ ...start }];
    let bbox = currChunk.bbox;
    while (true) {
      if (currChunk.visted) break;
      currChunk.visted = true;
      if (currChunk.mid.length !== 0) {
        // console.log(`ADD MID:`, currChunk.mid);
      }
      lineString.push(...currChunk.mid);
      foundPolygons.add(currChunk.polyIndex);
      bbox = mergeBBoxes(bbox, currChunk.bbox) as BBox;
      if (currChunk.next === undefined) break;
      const { chunk: nextChunk, intPoint } = currChunk.next;
      lineString.push({ ...intPoint });
      // console.log(`ADD INT:`, intPoint);
      currChunk = nextChunk;
      if (equalPoints(intPoint, start)) break;
    }
    const area = polygonRingArea(lineString, 1);
    if (area === 0 || lineString.length < 4 || !equalPoints(lineString.at(0)!, lineString.at(-1)!))
      continue;
    // now build the path or add to an existing path
    const isCCW = area > 0;
    // Find the correct PolyPath to insert into, otherwise create a new one, update the lookup to
    // include all new polygon indexes used in the path
    let foundPaths: PolyPath<D>[] = [];
    // Pull in all the old paths to merge with this one (may expand upon multiple paths, consume the holes)
    let currID = 0;
    for (const polyIndex of foundPolygons) {
      const path = pathLookup.get(polyIndex);
      if (path !== undefined) {
        path.id = currID++;
        foundPaths.push(path);
      }
    }
    // filter foundPaths if they have the same id as the previous
    foundPaths = foundPaths
      .sort((p1, p2) => p1.id - p2.id)
      .filter((p, i) => i === 0 || p.id !== foundPaths[i - 1]?.id);
    let path: PolyPath<D>;
    if (foundPaths.length === 0) {
      path = new PolyPath(lineString, foundPolygons, isCCW, bbox);
      paths.push(path);
    } else {
      // TODO: `chunk.ringIndex !== 0` may not be enough as may contain a hole chunk but started as an outer chunk
      // if only one found, update that one, otherwise create a new merged path and store the new result
      path = foundPaths.length === 1 ? foundPaths[0] : mergePaths(foundPaths);
      addChunkToPath(path, lineString, foundPolygons, bbox, isCCW, chunk.ringIndex !== 0);
    }
    // Store all inner outer rings that have not yet been consumed by the new outer but are inside the new outer
    storeInnerOldOuters(path, bboxes);
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

  // If one poly outer ring is entirely in another poly AND its CCW, it gets "consumed" (deleted. path is
  // because of the ordering, the first chunk to be an outer will be the one creating the path,
  // so we know all future CCW chunks that share a path will be "outers" that are inside the existing
  // path outer)
  // If one poly outer ring is entirely in another poly AND its CW, it converts to a hole
  // If one poly inner ring is CW, it gets consumed by an associated outer
  // If one poly inner ring is CCW, remove it
  // If a hole is found, it didn't come from a hole, and it's inside one of the old outer's bboxes, delete the pair.
  if (isCCW) {
    if (wasHole) return;
    if (path.outer === undefined) {
      path.outer = ring;
      path.bbox = mergeBBoxes(path.bbox, bbox) as BBox;
    } else {
      // If the ring's bbox is smaller than the existing outer, store. Otherwise replace
      if (bboxInside(bbox, path.bbox)) {
        path.oldOuters.push(bbox);
      } else {
        path.oldOuters.push(path.bbox);
        path.outer = ring;
        path.bbox = mergeBBoxes(path.bbox, bbox) as BBox;
      }
    }
  } else {
    if (!wasHole) {
      // Store discarded smaller outer rings, if hole is inside inner outer-ring, it cancels out the hole
      for (const oldOuter of path.oldOuters) {
        if (bboxInside(bbox, oldOuter)) return;
      }
    }
    path.holes.push(ring);
  }
}

/**
 * Store all inner old outers that have not yet been consumed by the new outer
 * @param path - the path
 * @param bboxes - the bboxes of all outer rings we are merging
 */
function storeInnerOldOuters<D extends MValue = Properties>(
  path: PolyPath<D>,
  bboxes: BBox[],
): void {
  if (path.outer === undefined) return;
  const { oldOuters, polysConsumed } = path;
  for (let i = 0; i < bboxes.length; i++) {
    if (polysConsumed.has(i)) continue;
    const bbox = bboxes[i];
    if (bboxInside(bbox, path.bbox)) oldOuters.push(bbox);
  }
}

/**
 * Merge in a collection of paths
 * @param pathsToMerge - the collection of paths
 * @returns - the result of all paths merged into one
 */
function mergePaths<D extends MValue = Properties>(pathsToMerge: PolyPath<D>[]): PolyPath<D> {
  const res = pathsToMerge[0];
  for (let i = 1; i < pathsToMerge.length; i++) {
    const other = pathsToMerge[i];
    // If this bbox is smaller than the existing outer, replace
    if (other.outer !== undefined && bboxInside(res.bbox, other.bbox)) {
      if (res.outer !== undefined) res.oldOuters.push(res.bbox);
      res.outer = other.outer;
    }
    res.holes.push(...other.holes);
    res.oldOuters.push(...other.oldOuters);
    res.polysConsumed = res.polysConsumed.union(other.polysConsumed);
    res.bbox = mergeBBoxes(res.bbox, other.bbox) as BBox;
    // clear the path now that we comsumed it
    other.outer = undefined;
    other.holes = [];
    other.oldOuters = [];
  }

  return res;
}
