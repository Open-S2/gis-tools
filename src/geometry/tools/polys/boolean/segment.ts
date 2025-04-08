import { BoolVec, vectorIntersection } from './vector';
import { SweepEvent, comparePoints } from './sweepEvent';
import { bboxOverlap, pointOverlap } from '../../..';

import type { Operation } from './operation';
import type { RingOut } from './geomOut';
import type { BBOX, VectorPoint } from '../../..';
import type { MultiPolyIn, RingIn } from './geomIn';

/** A segment state */
export interface State {
  rings: RingIn[];
  windings: number[];
  multiPolys: MultiPolyIn[];
}

// Give segments unique ID's to get consistent sorting of
// segments and sweep events when all else is identical
let segmentId = 0;

/** A segment */
export class Segment {
  id: number;
  leftSE: SweepEvent;
  rightSE: SweepEvent;
  rings?: RingIn[];
  windings?: number[];
  ringOut?: RingOut;
  consumedBy?: Segment;
  prev?: Segment;
  #prevInResult?: Segment;
  #beforeState?: State;
  #afterState?: State;
  #isInResult?: boolean;
  #operation: Operation;

  /**
   * Warning: a reference to ringWindings input will be stored,
   *  and possibly will be later modified
   * @param leftSE - left sweep event
   * @param rightSE - right sweep event
   * @param rings - array of rings that make up this segment
   * @param windings - array of windings that make up this segment
   * @param operation - Operation state
   */
  constructor(
    leftSE: SweepEvent,
    rightSE: SweepEvent,
    rings: RingIn[],
    windings: number[],
    operation: Operation,
  ) {
    this.id = ++segmentId;
    this.leftSE = leftSE;
    leftSE.segment = this;
    leftSE.otherSE = rightSE;
    this.rightSE = rightSE;
    rightSE.segment = this;
    rightSE.otherSE = leftSE;
    this.rings = rings;
    this.windings = windings;
    this.#operation = operation;
    // left unset for performance, set later in algorithm
    // this.ringOut, this.consumedBy, this.prev
  }

  /**
   * When a segment is split, the rightSE is replaced with a new sweep event
   * @param newRightSE - new right sweep event
   */
  replaceRightSE(newRightSE: SweepEvent): void {
    this.rightSE = newRightSE;
    this.rightSE.segment = this;
    this.rightSE.otherSE = this.leftSE;
    this.leftSE.otherSE = this.rightSE;
  }

  /**
   * Get the bounding box of this segment
   * @returns - the bounding box
   */
  bbox(): BBOX {
    const y1 = this.leftSE.point.y;
    const y2 = this.rightSE.point.y;
    return [this.leftSE.point.x, y1 < y2 ? y1 : y2, this.rightSE.point.x, y1 > y2 ? y1 : y2];
  }

  /**
   * @returns - A vector from the left point to the right
   */
  vector(): BoolVec {
    return {
      x: this.rightSE.point.x - this.leftSE.point.x,
      y: this.rightSE.point.y - this.leftSE.point.y,
    };
  }

  /**
   * Check if a point is an endpoint of the segment
   * @param pt - the point to check
   * @returns - true if the point is an endpoint
   */
  isAnEndpoint(pt: BoolVec): boolean {
    return (
      (pt.x === this.leftSE.point.x && pt.y === this.leftSE.point.y) ||
      (pt.x === this.rightSE.point.x && pt.y === this.rightSE.point.y)
    );
  }

  /**
   * Compare this segment with a point.
   *
   * A point P is considered to be colinear to a segment if there
   * exists a distance D such that if we travel along the segment
   * from one * endpoint towards the other a distance D, we find
   * ourselves at point P.
   *
   * Return value indicates:
   *
   *   1: point lies above the segment (to the left of vertical)
   *   0: point is colinear to segment
   *  -1: point lies below the segment (to the right of vertical)
   * @param point - the point to compare against
   * @returns - the comparison
   */
  comparePoint(point: BoolVec): number {
    return this.#operation.precision.orient(this.leftSE.point, point, this.rightSE.point);
  }

  /**
   * Given another segment, returns the first non-trivial intersection
   * between the two segments (in terms of sweep line ordering), if it exists.
   *
   * A 'non-trivial' intersection is one that will cause one or both of the
   * segments to be split(). As such, 'trivial' vs. 'non-trivial' intersection:
   *
   *   endpoint of segA with endpoint of segB --> trivial
   *   endpoint of segA with point along segB --> non-trivial
   *   endpoint of segB with point along segA --> non-trivial
   *   point along segA with point along segB --> non-trivial
   *
   * If no non-trivial intersection exists, return undefined.
   * Else, return undefined.
   * @param other - the other segment to compare against
   * @returns - the intersection if it exists. Undefined otherwise
   */
  getIntersection(other: Segment): BoolVec | undefined {
    // If bboxes don't overlap, there can't be any intersections
    const tBbox = this.bbox();
    const oBbox = other.bbox();
    const bboxOv = bboxOverlap(tBbox, oBbox);
    if (bboxOv === undefined) return undefined;

    // We first check to see if the endpoints can be considered intersections.
    // This will 'snap' intersections to endpoints if possible, and will
    // handle cases of colinearity.

    const tlp = this.leftSE.point;
    const trp = this.rightSE.point;
    const olp = other.leftSE.point;
    const orp = other.rightSE.point;

    // does each endpoint touch the other segment?
    // note that we restrict the 'touching' definition to only allow segments
    // to touch endpoints that lie forward from where we are in the sweep line pass
    const touchesOtherLSE =
      pointOverlap(tBbox, olp as unknown as VectorPoint) && this.comparePoint(olp) === 0;
    const touchesThisLSE =
      pointOverlap(oBbox, tlp as unknown as VectorPoint) && other.comparePoint(tlp) === 0;
    const touchesOtherRSE =
      pointOverlap(tBbox, orp as unknown as VectorPoint) && this.comparePoint(orp) === 0;
    const touchesThisRSE =
      pointOverlap(oBbox, trp as unknown as VectorPoint) && other.comparePoint(trp) === 0;

    // do left endpoints match?
    if (touchesThisLSE && touchesOtherLSE) {
      // these two cases are for colinear segments with matching left
      // endpoints, and one segment being longer than the other
      if (touchesThisRSE && !touchesOtherRSE) return trp;
      if (!touchesThisRSE && touchesOtherRSE) return orp;
      // either the two segments match exactly (two trival intersections)
      // or just on their left endpoint (one trivial intersection
      return undefined;
    }

    // does this left endpoint matches (other doesn't)
    if (touchesThisLSE) {
      // check for segments that just intersect on opposing endpoints
      if (touchesOtherRSE) {
        if (tlp.x === orp.x && tlp.y === orp.y) return undefined;
      }
      // t-intersection on left endpoint
      return tlp;
    }

    // does other left endpoint matches (this doesn't)
    if (touchesOtherLSE) {
      // check for segments that just intersect on opposing endpoints
      if (touchesThisRSE) {
        if (trp.x === olp.x && trp.y === olp.y) return undefined;
      }
      // t-intersection on left endpoint
      return olp;
    }

    // trivial intersection on right endpoints
    if (touchesThisRSE && touchesOtherRSE) return undefined;

    // t-intersections on just one right endpoint
    if (touchesThisRSE) return trp;
    if (touchesOtherRSE) return orp;

    // None of our endpoints intersect. Look for a general intersection between
    // infinite lines laid over the segments
    const pt = vectorIntersection(tlp, this.vector(), olp, other.vector());

    // are the segments parrallel? Note that if they were colinear with overlap,
    // they would have an endpoint intersection and that case was already handled above
    if (pt === undefined) return undefined;

    // is the intersection found between the lines not on the segments?
    if (!pointOverlap(bboxOv, pt as unknown as VectorPoint)) return undefined;

    // round the the computed point if needed
    return pt;
  }

  /**
   * Split the given segment into multiple segments on the given points.
   *  Each existing segment will retain its leftSE and a new rightSE will be
   *    generated for it.
   *  A new segment will be generated which will adopt the original segment's
   *    rightSE, and a new leftSE will be generated for it.
   *  If there are more than two points given to split on, new segments
   *    in the middle will be generated with new leftSE and rightSE's.
   *  An array of the newly generated SweepEvents will be returned.
   *
   * Warning: input array of points is modified
   * @param point - the points to split on
   * @returns - the newly generated SweepEvents
   */
  split(point: BoolVec): SweepEvent[] {
    const newEvents = [];
    const alreadyLinked = 'events' in point;

    const newLeftSE = new SweepEvent(point, true);
    const newRightSE = new SweepEvent(point, false);
    const oldRightSE = this.rightSE;
    this.replaceRightSE(newRightSE);
    newEvents.push(newRightSE);
    newEvents.push(newLeftSE);
    const newSeg = new Segment(
      newLeftSE,
      oldRightSE,
      this.rings!.slice(),
      this.windings!.slice(),
      this.#operation,
    );

    // when splitting a nearly vertical downward-facing segment,
    // sometimes one of the resulting new segments is vertical, in which
    // case its left and right events may need to be swapped
    if (comparePoints(newSeg.leftSE.point, newSeg.rightSE.point) > 0) {
      newSeg.swapEvents();
    }
    if (comparePoints(this.leftSE.point, this.rightSE.point) > 0) {
      this.swapEvents();
    }

    // in the point we just used to create new sweep events with was already
    // linked to other events, we need to check if either of the affected
    // segments should be consumed
    if (alreadyLinked) {
      newLeftSE.checkForConsuming();
      newRightSE.checkForConsuming();
    }

    return newEvents;
  }

  /** Swap which event is left and right */
  swapEvents(): void {
    const tmpEvt = this.rightSE;
    this.rightSE = this.leftSE;
    this.leftSE = tmpEvt;
    this.leftSE.isLeft = true;
    this.rightSE.isLeft = false;
    for (let i = 0, iMax = this.windings!.length; i < iMax; i++) {
      this.windings![i] *= -1;
    }
  }

  /**
   * Consume another segment. We take their rings under our wing
   * and mark them as consumed. Use for perfectly overlapping segments
   * @param other - the segment to consume
   */
  consume(other: Segment): void {
    let consumer = this as Segment;
    let consumee = other;
    while (consumer.consumedBy !== undefined) consumer = consumer.consumedBy;
    while (consumee.consumedBy !== undefined) consumee = consumee.consumedBy;

    const cmp = compareSegments(consumer, consumee);
    if (cmp === 0) return; // already consumed
    // the winner of the consumption is the earlier segment
    // according to sweep line ordering
    if (cmp > 0) {
      const tmp = consumer;
      consumer = consumee;
      consumee = tmp;
    }

    // make sure a segment doesn't consume it's prev
    if (consumer.prev === consumee) {
      const tmp = consumer;
      consumer = consumee;
      consumee = tmp;
    }

    for (let i = 0, iMax = consumee.rings!.length; i < iMax; i++) {
      const ring = consumee.rings![i];
      const winding = consumee.windings![i];
      const index = consumer.rings!.indexOf(ring);
      if (index === -1) {
        consumer.rings!.push(ring);
        consumer.windings!.push(winding);
      } else consumer.windings![index] += winding;
    }
    consumee.rings = undefined;
    consumee.windings = undefined;
    consumee.consumedBy = consumer;

    // mark sweep events consumed as to maintain ordering in sweep event queue
    consumee.leftSE.consumedBy = consumer.leftSE;
    consumee.rightSE.consumedBy = consumer.rightSE;
  }

  /**
   * The first segment previous segment chain that is in the result
   * @returns - the first segment previous segment chain that is in the result
   */
  prevInResult(): Segment | undefined {
    if (this.#prevInResult !== undefined) return this.#prevInResult;
    if (this.prev === undefined) this.#prevInResult = undefined;
    else if (this.prev.isInResult()) this.#prevInResult = this.prev;
    else this.#prevInResult = this.prev.prevInResult();
    return this.#prevInResult;
  }

  /** @returns - the state of the segment before the operation */
  beforeState(): State {
    if (this.#beforeState !== undefined) return this.#beforeState;
    if (this.prev === undefined)
      this.#beforeState = {
        rings: [],
        windings: [],
        multiPolys: [],
      };
    else {
      const seg = this.prev.consumedBy ?? this.prev;
      this.#beforeState = seg.afterState();
    }
    return this.#beforeState;
  }

  /** @returns - the state of the segment after the operation */
  afterState(): State {
    if (this.#afterState !== undefined) return this.#afterState;

    const beforeState = this.beforeState();
    this.#afterState = {
      rings: beforeState.rings.slice(0),
      windings: beforeState.windings.slice(0),
      multiPolys: [],
    };
    const ringsAfter = this.#afterState.rings;
    const windingsAfter = this.#afterState.windings;
    const mpsAfter = this.#afterState.multiPolys;

    // calculate ringsAfter, windingsAfter
    for (let i = 0, iMax = this.rings!.length; i < iMax; i++) {
      const ring = this.rings![i];
      const winding = this.windings![i];
      const index = ringsAfter.indexOf(ring);
      if (index === -1) {
        ringsAfter.push(ring);
        windingsAfter.push(winding);
      } else windingsAfter[index] += winding;
    }

    // calcualte polysAfter
    const polysAfter = [];
    const polysExclude = [];
    for (let i = 0, iMax = ringsAfter.length; i < iMax; i++) {
      if (windingsAfter[i] === 0) continue; // non-zero rule
      const ring = ringsAfter[i];
      const poly = ring.poly;
      if (polysExclude.indexOf(poly) !== -1) continue;
      if (ring.isExterior) polysAfter.push(poly);
      else {
        if (polysExclude.indexOf(poly) === -1) polysExclude.push(poly);
        const index = polysAfter.indexOf(ring.poly);
        if (index !== -1) polysAfter.splice(index, 1);
      }
    }

    // calculate multiPolysAfter
    for (let i = 0, iMax = polysAfter.length; i < iMax; i++) {
      const mp = polysAfter[i].multiPoly;
      if (mpsAfter.indexOf(mp) === -1) mpsAfter.push(mp);
    }

    return this.#afterState;
  }

  /**
   * @internal
   * @param isInResult - true if this segment is in the result
   */
  _setIsInResult(isInResult: boolean): void {
    this.#isInResult = isInResult;
  }

  /**
   * Is this segment part of the final result?
   * @returns - true if this segment is in the result
   */
  isInResult(): boolean {
    // if we've been consumed, we're not in the result
    if (this.consumedBy !== undefined) return false;

    if (this.#isInResult !== undefined) return this.#isInResult;

    const mpsBefore = this.beforeState().multiPolys;
    const mpsAfter = this.afterState().multiPolys;

    switch (this.#operation.type) {
      case 'union': {
        // UNION - included iff:
        //  * On one side of us there is 0 poly interiors AND
        //  * On the other side there is 1 or more.
        const noBefores = mpsBefore.length === 0;
        const noAfters = mpsAfter.length === 0;
        this.#isInResult = noBefores !== noAfters;
        break;
      }

      case 'intersection': {
        // INTERSECTION - included iff:
        //  * on one side of us all multipolys are rep. with poly interiors AND
        //  * on the other side of us, not all multipolys are repsented
        //    with poly interiors
        let least;
        let most;
        if (mpsBefore.length < mpsAfter.length) {
          least = mpsBefore.length;
          most = mpsAfter.length;
        } else {
          least = mpsAfter.length;
          most = mpsBefore.length;
        }
        this.#isInResult = most === this.#operation.numPolys && least < most;
        break;
      }

      case 'xor': {
        // XOR - included iff:
        //  * the difference between the number of multipolys represented
        //    with poly interiors on our two sides is an odd number
        const diff = Math.abs(mpsBefore.length - mpsAfter.length);
        this.#isInResult = diff % 2 === 1;
        break;
      }

      case 'difference': {
        /**
         * DIFFERENCE included iff:
         * on exactly one side, we have just the subject
         * @param mps - a list of multipolys
         * @returns - true if we have just the subject
         */
        const isJustSubject = (mps: MultiPolyIn[]) => mps.length === 1 && mps[0].isSubject;
        this.#isInResult = isJustSubject(mpsBefore) !== isJustSubject(mpsAfter);
        break;
      }
    }

    return this.#isInResult ?? false;
  }
}

/**
 * @param pt1 - first point
 * @param pt2 - second point
 * @param ring - ring to create segment from
 * @param operation - operation state
 * @returns - a new segment from a ring
 */
export function segmentFromRing(
  pt1: BoolVec,
  pt2: BoolVec,
  ring: RingIn,
  operation: Operation,
): Segment {
  let leftPt: BoolVec, rightPt: BoolVec, winding: number;

  // ordering the two points according to sweep line ordering
  const cmpPts = comparePoints(pt1, pt2);
  if (cmpPts < 0) {
    leftPt = pt1;
    rightPt = pt2;
    winding = 1;
  } else if (cmpPts > 0) {
    leftPt = pt2;
    rightPt = pt1;
    winding = -1;
  } else throw new Error(`Tried to create degenerate segment at [${pt1.x}, ${pt1.y}]`);

  const leftSE = new SweepEvent(leftPt, true);
  const rightSE = new SweepEvent(rightPt, false);
  return new Segment(leftSE, rightSE, [ring], [winding], operation);
}

/**
 * This compare() function is for ordering segments in the sweep
 * line tree, and does so according to the following criteria:
 *
 * Consider the vertical line that lies an infinestimal step to the
 * right of the right-more of the two left endpoints of the input
 * segments. Imagine slowly moving a point up from negative infinity
 * in the increasing y direction. Which of the two segments will that
 * point intersect first? That segment comes 'before' the other one.
 *
 * If neither segment would be intersected by such a line, (if one
 * or more of the segments are vertical) then the line to be considered
 * is directly on the right-more of the two left inputs.
 * @param a - first segment
 * @param b - second segment
 * @returns - 1 if a comes before b, 0 if they are equal, -1 if b comes before a
 */
export function compareSegments(a: Segment, b: Segment): number {
  const alx = a.leftSE.point.x;
  const blx = b.leftSE.point.x;
  const arx = a.rightSE.point.x;
  const brx = b.rightSE.point.x;

  // check if they're even in the same vertical plane
  if (brx < alx) return 1;
  if (arx < blx) return -1;

  const aly = a.leftSE.point.y;
  const bly = b.leftSE.point.y;
  const ary = a.rightSE.point.y;
  const bry = b.rightSE.point.y;

  // is left endpoint of segment B the right-more?
  if (alx < blx) {
    // are the two segments in the same horizontal plane?
    if (bly < aly && bly < ary) return 1;
    if (bly > aly && bly > ary) return -1;

    // is the B left endpoint colinear to segment A?
    const aCmpBLeft = a.comparePoint(b.leftSE.point);
    if (aCmpBLeft < 0) return 1;
    if (aCmpBLeft > 0) return -1;

    // is the A right endpoint colinear to segment B ?
    const bCmpARight = b.comparePoint(a.rightSE.point);
    if (bCmpARight !== 0) return bCmpARight;

    // colinear segments, consider the one with left-more
    // left endpoint to be first (arbitrary?)
    return -1;
  }

  // is left endpoint of segment A the right-more?
  if (alx > blx) {
    if (aly < bly && aly < bry) return -1;
    if (aly > bly && aly > bry) return 1;

    // is the A left endpoint colinear to segment B?
    const bCmpALeft = b.comparePoint(a.leftSE.point);
    if (bCmpALeft !== 0) return bCmpALeft;

    // is the B right endpoint colinear to segment A?
    const aCmpBRight = a.comparePoint(b.rightSE.point);
    if (aCmpBRight < 0) return 1;
    if (aCmpBRight > 0) return -1;

    // colinear segments, consider the one with left-more
    // left endpoint to be first (arbitrary?)
    return 1;
  }

  // if we get here, the two left endpoints are in the same
  // vertical plane, ie alx === blx

  // consider the lower left-endpoint to come first
  if (aly < bly) return -1;
  if (aly > bly) return 1;

  // left endpoints are identical
  // check for colinearity by using the left-more right endpoint

  // is the A right endpoint more left-more?
  if (arx < brx) {
    const bCmpARight = b.comparePoint(a.rightSE.point);
    if (bCmpARight !== 0) return bCmpARight;
  }

  // is the B right endpoint more left-more?
  if (arx > brx) {
    const aCmpBRight = a.comparePoint(b.rightSE.point);
    if (aCmpBRight < 0) return 1;
    if (aCmpBRight > 0) return -1;
  }

  if (arx !== brx) {
    // are these two [almost] vertical segments with opposite orientation?
    // if so, the one with the lower right endpoint comes first
    const ay = ary - aly;
    const ax = arx - alx;
    const by = bry - bly;
    const bx = brx - blx;
    if (ay > ax && by < bx) return 1;
    if (ay < ax && by > bx) return -1;
  }

  // we have colinear segments with matching orientation
  // consider the one with more left-more right endpoint to be first
  if (arx > brx) return 1;
  if (arx < brx) return -1;

  // if we get here, two two right endpoints are in the same
  // vertical plane, ie arx === brx

  // consider the lower right-endpoint to come first
  if (ary < bry) return -1;
  if (ary > bry) return 1;

  // right endpoints identical as well, so the segments are idential
  // fall back on creation order as consistent tie-breaker
  if (a.id < b.id) return -1;
  if (a.id > b.id) return 1;

  // identical segment, ie a === b
  return 0;
}
