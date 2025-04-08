import { compareSweepEvents } from './sweepEvent';

import type { Operation } from './operation';
import type { Segment } from './segment';
import type { SweepEvent } from './sweepEvent';
import type { LineString, MultiPolygon, Polygon } from '../../..';

/** Represents a polygon ring via an array of segments. */
export class RingOut {
  events: SweepEvent[];
  poly: PolyOut | undefined;
  #isExteriorRing: boolean | undefined;
  #enclosingRing: RingOut | undefined;
  #operation: Operation;

  /**
   * @param events - an array of sweep events
   * @param operation - Operation state to be passed around for reuse
   */
  constructor(events: SweepEvent[], operation: Operation) {
    this.events = events;
    this.#operation = operation;
    for (let i = 0, iMax = events.length; i < iMax; i++) events[i].segment.ringOut = this;
    this.poly = undefined;
  }

  /** @returns - the geometry of the ring or undefined if the ring is empty */
  getGeom(): LineString | undefined {
    // Remove superfluous points (ie extra points along a straight line),
    let prevPt = this.events[0].point;
    const points = [prevPt];
    for (let i = 1, iMax = this.events.length - 1; i < iMax; i++) {
      const pt = this.events[i].point;
      const nextPt = this.events[i + 1].point;
      if (this.#operation.precision.orient(pt, prevPt, nextPt) === 0) continue;
      points.push(pt);
      prevPt = pt;
    }

    // ring was all (within rounding error of angle calc) colinear points
    if (points.length === 1) return undefined;

    // check if the starting point is necessary
    const pt = points[0];
    const nextPt = points[1];
    if (this.#operation.precision.orient(pt, prevPt, nextPt) === 0) points.shift();

    points.push(points[0]);
    const step = this.isExteriorRing() ? 1 : -1;
    const iStart = this.isExteriorRing() ? 0 : points.length - 1;
    const iEnd = this.isExteriorRing() ? points.length : -1;
    const orderedPoints: LineString = [];
    for (let i = iStart; i !== iEnd; i += step) orderedPoints.push([points[i].x, points[i].y]);

    return orderedPoints;
  }

  /**
   * Used by tests
   * @internal
   * @param isExteriorRing - true if this is the exterior ring
   */
  _setIsExteriorRing(isExteriorRing: boolean): void {
    this.#isExteriorRing = isExteriorRing;
  }

  /** @returns - true if this is the exterior ring */
  isExteriorRing(): boolean {
    if (this.#isExteriorRing === undefined) {
      const enclosing = this.enclosingRing();
      this.#isExteriorRing = enclosing !== undefined ? !enclosing.isExteriorRing() : true;
    }
    return this.#isExteriorRing;
  }

  /** @returns - the ring that encloses this one, if any */
  enclosingRing(): RingOut | undefined {
    if (this.#enclosingRing === undefined) {
      this.#enclosingRing = this.#calcEnclosingRing();
    }
    return this.#enclosingRing;
  }

  /** @returns - the ring that encloses this one, if any */
  #calcEnclosingRing(): RingOut | undefined {
    // start with the ealier sweep line event so that the prevSeg
    // chain doesn't lead us inside of a loop of ours
    let leftMostEvt = this.events[0];
    for (let i = 1, iMax = this.events.length; i < iMax; i++) {
      const evt = this.events[i];
      if (compareSweepEvents(leftMostEvt, evt) > 0) leftMostEvt = evt;
    }

    let prevSeg: Segment | undefined = leftMostEvt.segment.prevInResult();
    let prevPrevSeg: Segment | undefined =
      prevSeg !== undefined ? prevSeg.prevInResult() : undefined;

    while (true) {
      // no segment found, thus no ring can enclose us
      if (prevSeg === undefined) return undefined;

      // no segments below prev segment found, thus the ring of the prev
      // segment must loop back around and enclose us
      if (prevPrevSeg === undefined) return prevSeg.ringOut;

      // if the two segments are of different rings, the ring of the prev
      // segment must either loop around us or the ring of the prev prev
      // seg, which would make us and the ring of the prev peers
      if (prevPrevSeg.ringOut !== prevSeg.ringOut) {
        if (prevPrevSeg.ringOut?.enclosingRing() !== prevSeg.ringOut) {
          return prevSeg.ringOut;
        } else return prevSeg.ringOut?.enclosingRing();
      }

      // two segments are from the same ring, so this was a penisula
      // of that ring. iterate downward, keep searching
      prevSeg = prevPrevSeg.prevInResult();
      prevPrevSeg = prevSeg !== undefined ? prevSeg.prevInResult() : undefined;
    }
  }
}

/** A collection of rings that form a polygon */
export class PolyOut {
  exteriorRing: RingOut;
  interiorRings: RingOut[];

  /**
   * @param exteriorRing - the exterior ring
   */
  constructor(exteriorRing: RingOut) {
    this.exteriorRing = exteriorRing;
    exteriorRing.poly = this;
    this.interiorRings = [];
  }

  /**
   * Add an interior ring to this polygon
   * @param ring - a new interior ring
   */
  addInterior(ring: RingOut): void {
    this.interiorRings.push(ring);
    ring.poly = this;
  }

  /**
   * Get the geometry of the polygon
   * @returns - the geometry
   */
  getGeom(): Polygon | undefined {
    const geom0 = this.exteriorRing.getGeom();
    // exterior ring was all (within rounding error of angle calc) colinear points
    if (geom0 === undefined) return undefined;
    const geom: Polygon = [geom0];
    for (let i = 0, iMax = this.interiorRings.length; i < iMax; i++) {
      const ringGeom = this.interiorRings[i].getGeom();
      // interior ring was all (within rounding error of angle calc) colinear points
      if (ringGeom === undefined) continue;
      geom.push(ringGeom);
    }
    return geom;
  }
}

/** A collection of rings that form a multipolygon */
export class MultiPolyOut {
  rings: RingOut[];
  polys: PolyOut[];

  /** @param rings - the rings */
  constructor(rings: RingOut[]) {
    this.rings = rings;
    this.polys = this.#composePolys(rings);
  }

  /**
   * Get the geometry of the multipolygon
   * @returns - the geometry
   */
  getGeom(): MultiPolygon {
    const geom: MultiPolygon = [];
    for (let i = 0, iMax = this.polys.length; i < iMax; i++) {
      const polyGeom = this.polys[i].getGeom();
      // exterior ring was all (within rounding error of angle calc) colinear points
      if (polyGeom === undefined) continue;
      geom.push(polyGeom);
    }
    return geom;
  }

  /**
   * Compose the polygons into a multipolygon output
   * @param rings - the rings to compose
   * @returns - the composed polygons
   */
  #composePolys(rings: RingOut[]): PolyOut[] {
    const polys = [];
    for (let i = 0, iMax = rings.length; i < iMax; i++) {
      const ring = rings[i];
      if (ring.poly !== undefined) continue;
      if (ring.isExteriorRing()) polys.push(new PolyOut(ring));
      else {
        const enclosingRing = ring.enclosingRing();
        if (enclosingRing?.poly === undefined) polys.push(new PolyOut(enclosingRing!));
        enclosingRing?.poly?.addInterior(ring);
      }
    }
    return polys;
  }
}

/**
 * Given the segments from the sweep line pass, compute & return a series
 * of closed rings from all the segments marked to be part of the result
 * @param allSegments - all the segments
 * @param operation - the operation state
 * @returns - an array of rings
 */
export function ringOutFactory(allSegments: Segment[], operation: Operation): RingOut[] {
  const ringsOut = [];

  for (let i = 0, iMax = allSegments.length; i < iMax; i++) {
    const segment = allSegments[i];
    if (!segment.isInResult() || segment.ringOut !== undefined) continue;

    let prevEvent = undefined;
    let event = segment.leftSE;
    let nextEvent = segment.rightSE;
    const events = [event];

    const startingPoint = event.point;
    const intersectionLEs = [];

    /* Walk the chain of linked events to form a closed ring */
    while (true) {
      prevEvent = event;
      event = nextEvent;
      events.push(event);

      /* Is the ring complete? */
      if (event.point === startingPoint) break;

      while (true) {
        const availableLEs = event.getAvailableLinkedEvents();

        /* Did we hit a dead end? This shouldn't happen. Indicates some earlier
         * part of the algorithm malfunctioned... please file a bug report. */
        if (availableLEs.length === 0) {
          const firstPt = events[0].point;
          const lastPt = events[events.length - 1].point;
          throw new Error(
            `Unable to complete output ring starting at [${firstPt.x},` +
              ` ${firstPt.y}]. Last matching segment found ends at` +
              ` [${lastPt.x}, ${lastPt.y}].`,
          );
        }

        /* Only one way to go, so continue on the path */
        if (availableLEs.length === 1) {
          nextEvent = availableLEs[0].otherSE;
          break;
        }

        /* We must have an intersection. Check for a completed loop */
        let indexLE = undefined;
        for (let j = 0, jMax = intersectionLEs.length; j < jMax; j++) {
          if (intersectionLEs[j].point === event.point) {
            indexLE = j;
            break;
          }
        }
        /* Found a completed loop. Cut that off and make a ring */
        if (indexLE !== undefined) {
          const intersectionLE = intersectionLEs.splice(indexLE)[0];
          const ringEvents = events.splice(intersectionLE.index);
          ringEvents.unshift(ringEvents[0].otherSE);
          ringsOut.push(new RingOut(ringEvents.reverse(), operation));
          continue;
        }
        /* register the intersection */
        intersectionLEs.push({
          index: events.length,
          point: event.point,
        });
        /* Choose the left-most option to continue the walk */
        const comparator = event.getLeftmostComparator(prevEvent);
        nextEvent = availableLEs.sort(comparator)[0].otherSE;
        break;
      }
    }

    ringsOut.push(new RingOut(events, operation));
  }
  return ringsOut;
}
