import {
  InterPointLookup,
  PolyPath,
  RingChunk,
  bboxArea,
  bboxInside,
  buildPathsAndChunks,
  equalPoints,
  mergeBBoxes,
  mergeIntersectionPairs,
  polygonRingArea,
  polygonsIntersectionsLookup,
  polylineInPolyline,
} from '../../../index.js';

import type {
  BBox,
  MValue,
  Properties,
  RingIntersectionLookup,
  Segment,
  VectorFeatures,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../../../index.js';

/**
 * Given a collection of polygons, if any of the polygons are kinked, dekink them
 *
 * NOTE: This algorithm assumes the ring order is correct. Outer rings must be counter-clockwise and
 * inner rings must be clockwise
 * @param polygons - the polygons are from either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @returns - the dekinked polygons
 */
export function dekinkPolygons<
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

  const res: VectorMultiPolygonGeometry<D> = { type: 'MultiPolygon', coordinates: [], is3D: false };
  for (const vectorPoly in vectorPolygons) {
    const dekinked = dekinkPolygon(vectorPolygons[vectorPoly]);
    if (dekinked !== undefined) {
      if (dekinked.bbox !== undefined) res.bbox = mergeBBoxes(res.bbox, dekinked.bbox);
      res.coordinates.push(...dekinked.coordinates);
    }
  }

  return res;
}

/**
 * Given a polygon, if the polygon is kinked, dekink it
 *
 * NOTE: This algorithm assumes the ring order is correct. Outer rings must be counter-clockwise and
 * inner rings must be clockwise
 * @param polygon - the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @returns - the dekinked polygon
 */
export function dekinkPolygon<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygon:
    | VectorPolygon<D>
    | VectorPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorPolygonGeometry<D>>,
): VectorMultiPolygonGeometry<D> | undefined {
  const vectorPolygon: VectorPolygon<D> =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;
  // not enough data, just return undefined
  if (vectorPolygon.length === 0) return undefined;
  const vectorPolygons = [vectorPolygon];

  // 1) build intersections `[polyIndex][ringIndex] -> Intersections`. Store where on the ring other rings intersect
  const ringIntLookup: RingIntersectionLookup<D> = polygonsIntersectionsLookup(
    vectorPolygons,
    (seg1: Segment) => {
      return (seg2: Segment): boolean =>
        // if same id ignore
        seg2.id !== seg1.id &&
        // only pass forward not backward
        seg2.id > seg1.id &&
        // TODO: At some point intersections of inner rings against the outer ring should be considered.
        // For now the ringIndex must be the same, polygonsIntersectionsLookup should return
        // two problem sets down the road, one for cleaning individual rings, and one for fixing holes that go out of bounds
        seg2.ringIndex === seg1.ringIndex;
    },
  );

  // 2) Build Poly Pieces
  // Setup result paths with chunks that are the final structure of joined polygons.
  // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
  // If no intersections for the polyIndex+RingIndex -> it's immediately consumed into paths. Otherwise it's a chunk
  const [paths, _, chunks, intLookup] = buildPathsAndChunks(vectorPolygons, ringIntLookup);

  // 3) Consume chunks into PolyPaths
  // If no intersections for the polyIndex+RingIndex -> push as completed ring
  buildPathsFromChunks(paths, intLookup, chunks);

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

/** Simplified ring with guide of how it was rebuild */
interface Ring<D extends MValue = Properties> {
  lineString: VectorLineString<D>;
  isCCW: boolean;
  isHole: boolean;
  bbox: BBox;
  area: number; // bbox area
}
/** Collection of rings sorted by polyIndex */
interface RingStore<D extends MValue = Properties> {
  [polyIndex: number]: Ring<D>[];
}

/**
 * Given a set of chunks, build a set of paths
 * @param paths - a set of paths to add to
 * @param intersections - all intersections
 * @param chunks - a set of chunks
 */
function buildPathsFromChunks<D extends MValue = Properties>(
  paths: PolyPath<D>[],
  intersections: InterPointLookup<D>,
  chunks: RingChunk<D>[],
): void {
  const ringStore: RingStore<D> = {};
  // store existing paths and reset.
  for (const { outer, polysConsumed, bbox, holes } of paths) {
    const polyIndex = [...polysConsumed][0];
    const polyStore = (ringStore[polyIndex] ??= []);
    if (outer !== undefined)
      polyStore.push({ lineString: outer, isCCW: true, isHole: false, bbox, area: bboxArea(bbox) });
    for (const hole of holes)
      polyStore.push({ lineString: hole, isCCW: false, isHole: true, bbox, area: bboxArea(bbox) });
  }
  paths.splice(0, paths.length);
  // for each intersections, connect all the from and to, smallest angle between from->to first slowly work your way through
  for (const xs of Object.values(intersections.lookup)) {
    for (const ys of Object.values(xs)) mergeIntersectionPairs(ys);
  }
  // run through all chunks, if unvisited, add to paths
  for (const chunk of chunks) {
    if (chunk.visted) continue;
    const start = chunk.from;
    let currChunk = chunk;
    const lineString: VectorLineString<D> = [{ ...start }];
    let bbox = currChunk.bbox;
    while (true) {
      if (currChunk.visted) break;
      currChunk.visted = true;
      lineString.push(...currChunk.mid);
      bbox = mergeBBoxes(bbox, currChunk.bbox) as BBox;
      if (currChunk.next === undefined) break;
      const { chunk: nextChunk, intPoint } = currChunk.next;
      lineString.push({ ...intPoint });
      currChunk = nextChunk;
      if (equalPoints(intPoint, start)) break;
    }
    const area = polygonRingArea(lineString, 1);
    if (area === 0 || lineString.length < 4 || !equalPoints(lineString.at(0)!, lineString.at(-1)!))
      continue;
    // now build the path or add to an existing path
    const isCCW = area > 0;
    const isHole = chunk.ringIndex !== 0;
    // store in correct location
    const polyStore = (ringStore[chunk.polyIndex] ??= []);
    polyStore.push({ lineString, isCCW, isHole, bbox, area: bboxArea(bbox) });
  }
  // For each ringStore, build out polys and store them in paths
  for (const rings of Object.values(ringStore)) ringSetToPaths(paths, rings);
}

/**
 * Convert a set of rings into a set of paths
 * @param paths - the collection of paths to store the results in
 * @param ringSet - the current set of rings re-built from a polygon
 */
function ringSetToPaths<D extends MValue = Properties>(
  paths: PolyPath<D>[],
  ringSet: Ring<D>[],
): void {
  if (ringSet.length === 0) return;
  // sort by bbox area desc
  ringSet.sort((a, b) => b.area - a.area);
  // filter by case type
  const outers = ringSet.filter((r) => !r.isHole && r.isCCW);
  const outersMaybeHole = ringSet.filter((r) => !r.isHole && !r.isCCW);
  const holes = ringSet.filter((r) => r.isHole && !r.isCCW);
  // const holesMaybeHole = ringSet.filter((r) => r.isHole && r.isCCW);
  const actualOuters: PolyPath<D>[] = [];

  // store all true outers. We know the first outer is the largest original outer ring.
  // The future outers are holes either kink inside or outside the original. Only store kinks that are outside the original
  if (outers.length > 0) {
    // store the first one
    const { lineString: firstLink, bbox: firstBBox } = outers[0];
    actualOuters.push(new PolyPath(firstLink, new Set(), true, firstBBox));
    // check all the others
    for (const outer of outers.slice(1)) {
      if (bboxInside(outer.bbox, firstBBox) && polylineInPolyline(outer.lineString, firstLink))
        continue;
      actualOuters.push(new PolyPath(outer.lineString, new Set(), true, outer.bbox));
    }
  }

  // If outer in `outersMaybeHole` is inside an actual outer, it's a hole; Otherwise it's another outer
  for (const { lineString, bbox } of outersMaybeHole) {
    let found = false;
    for (const actualOuter of actualOuters) {
      if (
        bboxInside(bbox, actualOuter.bbox) &&
        polylineInPolyline(lineString, actualOuter.outer!)
      ) {
        // store the hole in this outer
        actualOuter.holes.push(lineString);
        found = true;
        break;
      }
    }
    if (found) continue;
    // otherwise, it's a new outer
    actualOuters.push(new PolyPath(lineString.reverse(), new Set(), true, bbox));
  }

  // now organize holes
  if (actualOuters.length !== 0) {
    for (const { lineString, bbox } of holes) {
      if (actualOuters.length === 1) {
        actualOuters[0].holes.push(lineString);
      } else {
        // find the outer this hole belongs to
        for (const actualOuter of actualOuters) {
          if (
            bboxInside(bbox, actualOuter.bbox) &&
            polylineInPolyline(lineString, actualOuter.outer!)
          ) {
            // store the hole in this outer
            actualOuter.holes.push(lineString);
            break;
          }
        }
      }
    }
  }

  // Now store all actualOuters we built
  paths.push(...actualOuters);
}
