import { polygonsIntersections } from '../index.js';

import type {
  MValue,
  Properties,
  VectorFeatures,
  VectorLineString,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPolygon,
  VectorPolygonGeometry,
} from '../../../index.js';

// TODO: At some point intersections of inner rings against the outer ring should be considered
// be sure to address the `segment1.ringIndex === segment2.ringIndex` filter when implementing

/**
 * Given a collection of polygons, if any of the polygons are kinked, dekink them
 * @param polygons - the polygons are from either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @returns - the dekinked polygons
 */
export function deKinkPolygons<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygons:
    | VectorMultiPolygon<D>
    | VectorMultiPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorMultiPolygonGeometry<D>>,
): VectorMultiPolygon<D> {
  const vectorPolygons: VectorMultiPolygon<D> =
    'geometry' in polygons
      ? polygons.geometry.coordinates
      : 'coordinates' in polygons
        ? polygons.coordinates
        : polygons;
  return vectorPolygons.flatMap((p) => deKinkPolygon(p));
}

/**
 * Given a polygon, if the polygon is kinked, dekink it
 * @param polygon - the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
 * @returns - the dekinked polygon
 */
export function deKinkPolygon<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  polygon:
    | VectorPolygon<D>
    | VectorPolygonGeometry<D>
    | VectorFeatures<M, D, P, VectorPolygonGeometry<D>>,
): VectorMultiPolygon<D> {
  const vectorPolygon: VectorPolygon<D> =
    'geometry' in polygon
      ? polygon.geometry.coordinates
      : 'coordinates' in polygon
        ? polygon.coordinates
        : polygon;

  // build all segments, filter out "segments" that are endpoints and intersections that are not on the same ring
  const intersections = polygonsIntersections<M, D, P>([vectorPolygon], true)
    .filter(({ segment1, segment2, u, t }) => {
      return u !== 0 && u !== 1 && t !== 0 && t !== 1 && segment1.ringIndex === segment2.ringIndex;
    })
    // Sort intersections by `ringIndex` then `from`
    .sort((a, b) => {
      const diff = a.segment1.ringIndex - b.segment1.ringIndex;
      return diff !== 0 ? diff : a.segment1.from - b.segment1.from;
    });

  const res: VectorMultiPolygon<D> = [];

  // if there are no intersections, return a clone of the original polygon
  if (intersections.length === 0) {
    res.push(vectorPolygon.map((ring) => ring.map((point) => ({ ...point }))));
    return res;
  }

  // The points outside the kinks are summed up from the beginning of the polygon ring till it reaches intersections,
  // then each intersection you move to the intersection point itself and keep going onwards
  // the "self-intersecting" ring data are the intersection segment from -> intersection to IF the ring length is
  // greater than 4 total points
  const dekinkedPolygon: VectorPolygon<D> = [];
  for (let r = 0; r < vectorPolygon.length; r++) {
    const ringIntersections = intersections.filter((i) => i.segment1.ringIndex === r);
    const ring = vectorPolygon[r];
    const dekinkedRing: VectorLineString<D> = [];
    // build the outer ring slicing around intersections
    let index = 0;
    for (const { point, segment1: startSegment, segment2: endSegment } of ringIntersections) {
      dekinkedRing.push(...ring.slice(index, startSegment.from + 1).map((point) => ({ ...point })));
      dekinkedRing.push({ ...point });
      index = endSegment.to;
    }
    dekinkedRing.push(...ring.slice(index).map((point) => ({ ...point })));
    dekinkedPolygon.push(dekinkedRing);

    // build the portions inside the kinks of the ring using inside each segment intersection
    for (const { segment1, segment2, point } of ringIntersections) {
      const selfIntersectRing: VectorLineString<D> = [];
      selfIntersectRing.push({ ...point }); // begin at intersection
      selfIntersectRing.push(
        ...ring.slice(segment1.to, segment2.from + 1).map((point) => ({ ...point })),
      ); // add all internal points
      selfIntersectRing.push({ ...point }); // end at intersection
      // If the ring is an inner polygon ring (hole), keep adding the holes to the dekinkedPolygon
      // otherwise its a new poylgon outer ring
      if (r !== 0) dekinkedPolygon.push(selfIntersectRing);
      else res.push([selfIntersectRing]); // add the ring that's now it's own polygon outer-ring
    }
  }
  res.unshift(dekinkedPolygon);

  return res;
}
