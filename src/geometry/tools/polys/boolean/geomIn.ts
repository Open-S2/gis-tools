import { segmentFromRing } from './segment';
import { extendBBox, mergeBBoxes } from '../../..';

import type { Operation } from './operation';
import type { Segment } from './segment';
import type { SweepEvent } from './sweepEvent';
import type { BBOX, LineString, MultiPolygon, Polygon, VectorPoint } from '../../..';

/** Represents a polygon ring via an array of segments. */
export class RingIn {
  poly: PolyIn;
  isExterior: boolean;
  segments: Segment[];
  bbox: BBOX;

  /**
   * @param geomRing - an array of points
   * @param poly - the parent polygon
   * @param isExterior - true if this is the exterior ring
   * @param operation - Operation store to be passed around for reuse
   */
  constructor(geomRing: LineString, poly: PolyIn, isExterior: boolean, operation: Operation) {
    if (geomRing.length === 0) {
      throw new Error('Input geometry is not a valid Polygon or MultiPolygon');
    }

    this.poly = poly;
    this.isExterior = isExterior;
    this.segments = [];

    if (typeof geomRing[0][0] !== 'number' || typeof geomRing[0][1] !== 'number') {
      throw new Error('Input geometry is not a valid Polygon or MultiPolygon');
    }

    const firstPoint = operation.precision.snap({
      x: geomRing[0][0],
      y: geomRing[0][1],
    });
    this.bbox = [firstPoint.x, firstPoint.y, firstPoint.x, firstPoint.y];

    let prevPoint = firstPoint;
    for (let i = 1, iMax = geomRing.length; i < iMax; i++) {
      if (typeof geomRing[i][0] !== 'number' || typeof geomRing[i][1] !== 'number') {
        throw new Error('Input geometry is not a valid Polygon or MultiPolygon');
      }
      const point = operation.precision.snap({
        x: geomRing[i][0],
        y: geomRing[i][1],
      });
      // skip repeated points
      if (point.x === prevPoint.x && point.y === prevPoint.y) continue;
      this.segments.push(segmentFromRing(prevPoint, point, this, operation));
      this.bbox = extendBBox(this.bbox, point as unknown as VectorPoint);
      prevPoint = point;
    }
    // add segment from last to first if last is not the same as first
    if (firstPoint.x !== prevPoint.x || firstPoint.y !== prevPoint.y) {
      this.segments.push(segmentFromRing(prevPoint, firstPoint, this, operation));
    }
  }

  /**
   * @returns - an array of sweep events
   */
  getSweepEvents(): SweepEvent[] {
    const sweepEvents = [];
    for (let i = 0, iMax = this.segments.length; i < iMax; i++) {
      const segment = this.segments[i];
      sweepEvents.push(segment.leftSE);
      sweepEvents.push(segment.rightSE);
    }

    return sweepEvents;
  }
}

/**
 * Represents a polygon via an array of rings.
 */
export class PolyIn {
  multiPoly: MultiPolyIn;
  exteriorRing: RingIn;
  interiorRings: RingIn[];
  bbox: BBOX;

  /**
   * @param geomPoly - an array of rings
   * @param multiPoly - the parent multipolygon
   * @param operation - Operation store to be passed around for reuse
   */
  constructor(geomPoly: Polygon, multiPoly: MultiPolyIn, operation: Operation) {
    this.exteriorRing = new RingIn(geomPoly[0], this, true, operation);
    // copy by value
    this.bbox = [...this.exteriorRing.bbox];
    this.interiorRings = [];
    for (let i = 1, iMax = geomPoly.length; i < iMax; i++) {
      if (geomPoly[i].length === 0) continue;
      const ring = new RingIn(geomPoly[i], this, false, operation);
      this.bbox = mergeBBoxes(this.bbox, ring.bbox);
      this.interiorRings.push(ring);
    }
    this.multiPoly = multiPoly;
  }

  /**
   * @returns - an array of sweep events
   */
  getSweepEvents(): SweepEvent[] {
    const sweepEvents = this.exteriorRing.getSweepEvents();
    for (const interiorRing of this.interiorRings) {
      const ringSweepEvents = interiorRing.getSweepEvents();
      for (const ringSweepEvent of ringSweepEvents) {
        sweepEvents.push(ringSweepEvent);
      }
    }

    return sweepEvents;
  }
}

/**
 * Represents a multipolygon via an array of polygons
 */
export class MultiPolyIn {
  isSubject: boolean;
  polys: PolyIn[];
  bbox: BBOX;

  /**
   * @param geom - an array of polygons
   * @param isSubject - true if the multipolygon is the "primary" geometry that we are operating on
   * @param operation - Operation state to be passed around for reuse
   */
  constructor(geom: MultiPolygon | Polygon, isSubject: boolean, operation: Operation) {
    // @ts-expect-error - let's expand Polygons to MultiPolygons if needed
    const multiGeom: MultiPolygon = typeof geom[0][0][0] === 'number' ? [geom] : geom;
    this.polys = [];
    this.bbox = [Infinity, Infinity, -Infinity, -Infinity];
    for (const geo of multiGeom) {
      const poly = new PolyIn(geo, this, operation);
      this.bbox = mergeBBoxes(this.bbox, poly.bbox);
      this.polys.push(poly);
    }
    this.isSubject = isSubject;
  }

  /**
   * @returns - an array of sweep events
   */
  getSweepEvents(): SweepEvent[] {
    const sweepEvents = [];
    for (const poly of this.polys) {
      const polySweepEvents = poly.getSweepEvents();
      for (const polySweepEvent of polySweepEvents) {
        sweepEvents.push(polySweepEvent);
      }
    }

    return sweepEvents;
  }
}
