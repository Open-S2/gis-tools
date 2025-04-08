import { compareSegments } from './segment';
import { cosineOfAngle, sineOfAngle } from './vector';

import type { BoolVec } from './vector';
import type { Segment } from './segment';

/** A sweep event */
export class SweepEvent {
  point: BoolVec;
  isLeft: boolean;
  segment!: Segment;
  otherSE!: SweepEvent;
  consumedBy?: SweepEvent;
  /**
   * Warning: 'point' input will be modified and re-used (for performance)
   * @param point - the point to associate with this event
   * @param isLeft - whether this is a left event or right
   */
  constructor(point: BoolVec, isLeft: boolean) {
    if (point.m === undefined) point.m = [this];
    else point.m.push(this);
    this.point = point;
    this.isLeft = isLeft;
    // this.segment, this.otherSE set by factory
  }

  /**
   * Link this sweep event to another
   * @param other - the other sweep event to link to
   */
  link(other: SweepEvent): void {
    if (other.point === this.point) {
      throw new Error('Tried to link already linked events');
    }
    const otherEvents = other.point.m ?? [];
    if (this.point.m === undefined) this.point.m = [];
    for (let i = 0, iMax = otherEvents.length; i < iMax; i++) {
      const evt = otherEvents[i];
      this.point.m.push(evt);
      evt.point = this.point;
    }
    this.checkForConsuming();
  }

  /**
   * Do a pass over our linked events and check to see if any pair
   * of segments match, and should be consumed.
   */
  checkForConsuming(): void {
    // FIXME: The loops in this method run O(n^2) => no good.
    //        Maintain little ordered sweep event trees?
    //        Can we maintaining an ordering that avoids the need
    //        for the re-sorting with getLeftmostComparator in geom-out?

    // Compare each pair of events to see if other events also match
    if (this.point.m === undefined) return;
    const numEvents = this.point.m.length;
    for (let i = 0; i < numEvents; i++) {
      const evt1 = this.point.m[i];
      if (evt1.segment.consumedBy !== undefined) continue;
      for (let j = i + 1; j < numEvents; j++) {
        const evt2 = this.point.m[j];
        if (evt2.consumedBy !== undefined) continue;
        if (evt1.otherSE.point.m !== evt2.otherSE.point.m) continue;
        evt1.segment.consume(evt2.segment);
      }
    }
  }

  /**
   * Get the array of all linked events that are available to be consumed
   * @returns - an array of all linked events that are available to be consumed
   */
  getAvailableLinkedEvents(): SweepEvent[] {
    if (this.point.m === undefined) return [];
    // point.events is always of length 2 or greater
    const events = [];
    for (let i = 0, iMax = this.point.m.length; i < iMax; i++) {
      const evt = this.point.m[i];
      if (evt !== this && evt.segment.ringOut === undefined && evt.segment.isInResult()) {
        events.push(evt);
      }
    }
    return events;
  }

  /**
   * Returns a comparator function for sorting linked events that will
   * favor the event that will give us the smallest left-side angle.
   * All ring construction starts as low as possible heading to the right,
   * so by always turning left as sharp as possible we'll get polygons
   * without uncessary loops & holes.
   *
   * The comparator function has a compute cache such that it avoids
   * re-computing already-computed values.
   * @param baseEvent - the base event to compare against
   * @returns - the comparator for sorting the linked events
   */
  getLeftmostComparator(baseEvent: SweepEvent): (a: SweepEvent, b: SweepEvent) => number {
    const cache = new Map<SweepEvent, { sine: number; cosine: number }>();

    /**
     * Fill the cache for a linked event
     * @param linkedEvent - the event to fill the cache for
     */
    const fillCache = (linkedEvent: SweepEvent): void => {
      const nextEvent = linkedEvent.otherSE;
      cache.set(linkedEvent, {
        sine: sineOfAngle(this.point, baseEvent.point, nextEvent.point),
        cosine: cosineOfAngle(this.point, baseEvent.point, nextEvent.point),
      });
    };

    return (a: SweepEvent, b: SweepEvent): number => {
      if (!cache.has(a)) fillCache(a);
      if (!cache.has(b)) fillCache(b);

      const { sine: asine, cosine: acosine } = cache.get(a)!;
      const { sine: bsine, cosine: bcosine } = cache.get(b)!;

      // both on or above x-axis
      if (asine >= 0 && bsine >= 0) {
        if (acosine < bcosine) return 1;
        if (acosine > bcosine) return -1;
        return 0;
      }

      // both below x-axis
      if (asine < 0 && bsine < 0) {
        if (acosine < bcosine) return -1;
        if (acosine > bcosine) return 1;
        return 0;
      }

      // one above x-axis, one below
      if (bsine < asine) return -1;
      if (bsine > asine) return 1;
      return 0;
    };
  }
}

/**
 * for ordering sweep events in the sweep event queue
 * @param a - the first sweep event
 * @param b - the second sweep event
 * @returns - -1 if a < b, 0 if a == b, 1 if a > b
 */
export function compareSweepEvents(a: SweepEvent, b: SweepEvent): number {
  // favor event with a point that the sweep line hits first
  const ptCmp = comparePoints(a.point, b.point);
  if (ptCmp !== 0) return ptCmp;
  // the points are the same, so link them if needed
  if (a.point !== b.point) a.link(b);
  // favor right events over left
  if (a.isLeft !== b.isLeft) return a.isLeft ? 1 : -1;
  // we have two matching left or right endpoints
  // ordering of this case is the same as for their segments
  return compareSegments(a.segment, b.segment);
}

/**
 * for ordering points in sweep line order
 * @param aPt - the first point
 * @param bPt - the second point
 * @returns - -1 if a < b, 0 if a == b, 1 if a > b
 */
export function comparePoints(aPt: BoolVec, bPt: BoolVec): number {
  if (aPt.x < bPt.x) return -1;
  if (aPt.x > bPt.x) return 1;

  if (aPt.y < bPt.y) return -1;
  if (aPt.y > bPt.y) return 1;

  return 0;
}
