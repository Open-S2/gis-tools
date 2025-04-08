import { SplayTreeSet } from '../../../..';
import { comparePoints } from './sweepEvent';
import { compareSegments } from './segment';

import type { BoolVec } from './vector';
import type { Segment } from './segment';
import type { SweepEvent } from './sweepEvent';

/**
 * Sweep line data structure
 *
 * NOTE:  We must be careful not to change any segments while
 *        they are in the SplayTree. AFAIK, there's no way to tell
 *        the tree to rebalance itself - thus before splitting
 *        a segment that's in the tree, we remove it from the tree,
 *        do the split, then re-insert it. (Even though splitting a
 *        segment *shouldn't* change its correct position in the
 *        sweep line tree, the reality is because of rounding errors,
 *        it sometimes does.)
 */
export class SweepLine {
  private queue: SplayTreeSet<SweepEvent>;
  private tree: SplayTreeSet<Segment>;
  segments: Segment[];

  /**
   * @param queue - sweep event queue
   * @param comparator - comparator function
   */
  constructor(queue: SplayTreeSet<SweepEvent>, comparator = compareSegments) {
    this.queue = queue;
    this.tree = new SplayTreeSet(comparator);
    this.segments = [];
  }

  /**
   * Process a sweep event
   * @param event - sweep event
   * @returns - array of new sweep events
   */
  process(event: SweepEvent): SweepEvent[] {
    const segment = event.segment;
    const newEvents: SweepEvent[] = [];

    // if we've already been consumed by another segment,
    // clean up our body parts and get out
    if (event.consumedBy !== undefined) {
      if (event.isLeft) this.queue.delete(event.otherSE);
      else this.tree.delete(segment);
      return newEvents;
    }

    if (event.isLeft) this.tree.add(segment);

    let prevSeg: Segment | undefined = segment;
    let nextSeg: Segment | undefined = segment;

    // skip consumed segments still in tree
    do {
      prevSeg = this.tree.lastBefore(prevSeg) ?? undefined;
    } while (prevSeg !== undefined && prevSeg.consumedBy !== undefined);

    // skip consumed segments still in tree
    do {
      nextSeg = this.tree.firstAfter(nextSeg) ?? undefined;
    } while (nextSeg !== undefined && nextSeg.consumedBy !== undefined);

    if (event.isLeft) {
      // Check for intersections against the previous segment in the sweep line
      let prevMySplitter = undefined;
      if (prevSeg !== undefined) {
        const prevInter = prevSeg.getIntersection(segment);
        if (prevInter !== undefined) {
          if (!segment.isAnEndpoint(prevInter)) prevMySplitter = prevInter;
          if (!prevSeg.isAnEndpoint(prevInter)) {
            const newEventsFromSplit = this.#splitSafely(prevSeg, prevInter);
            for (let i = 0, iMax = newEventsFromSplit.length; i < iMax; i++) {
              newEvents.push(newEventsFromSplit[i]);
            }
          }
        }
      }

      // Check for intersections against the next segment in the sweep line
      let nextMySplitter = undefined;
      if (nextSeg !== undefined) {
        const nextInter = nextSeg.getIntersection(segment);
        if (nextInter !== undefined) {
          if (!segment.isAnEndpoint(nextInter)) nextMySplitter = nextInter;
          if (!nextSeg.isAnEndpoint(nextInter)) {
            const newEventsFromSplit = this.#splitSafely(nextSeg, nextInter);
            for (let i = 0, iMax = newEventsFromSplit.length; i < iMax; i++) {
              newEvents.push(newEventsFromSplit[i]);
            }
          }
        }
      }

      // For simplicity, even if we find more than one intersection we only
      // spilt on the 'earliest' (sweep-line style) of the intersections.
      // The other intersection will be handled in a future process().
      if (prevMySplitter !== undefined || nextMySplitter !== undefined) {
        let mySplitter = undefined;
        if (prevMySplitter === undefined) mySplitter = nextMySplitter;
        else if (nextMySplitter === undefined) mySplitter = prevMySplitter;
        else {
          const cmpSplitters = comparePoints(prevMySplitter, nextMySplitter);
          mySplitter = cmpSplitters <= 0 ? prevMySplitter : nextMySplitter;
        }

        // Rounding errors can cause changes in ordering,
        // so remove afected segments and right sweep events before splitting
        this.queue.delete(segment.rightSE);
        newEvents.push(segment.rightSE);

        const newEventsFromSplit = segment.split(mySplitter!);
        for (let i = 0, iMax = newEventsFromSplit.length; i < iMax; i++) {
          newEvents.push(newEventsFromSplit[i]);
        }
      }

      if (newEvents.length > 0) {
        // We found some intersections, so re-do the current event to
        // make sure sweep line ordering is totally consistent for later
        // use with the segment 'prev' pointers
        this.tree.delete(segment);
        newEvents.push(event);
      } else {
        // done with left event
        this.segments.push(segment);
        segment.prev = prevSeg;
      }
    } else {
      // event.isRight

      // since we're about to be removed from the sweep line, check for
      // intersections between our previous and next segments
      if (prevSeg !== undefined && nextSeg !== undefined) {
        const inter = prevSeg.getIntersection(nextSeg);
        if (inter !== undefined) {
          if (!prevSeg.isAnEndpoint(inter)) {
            const newEventsFromSplit = this.#splitSafely(prevSeg, inter);
            for (let i = 0, iMax = newEventsFromSplit.length; i < iMax; i++) {
              newEvents.push(newEventsFromSplit[i]);
            }
          }
          if (!nextSeg.isAnEndpoint(inter)) {
            const newEventsFromSplit = this.#splitSafely(nextSeg, inter);
            for (let i = 0, iMax = newEventsFromSplit.length; i < iMax; i++) {
              newEvents.push(newEventsFromSplit[i]);
            }
          }
        }
      }

      this.tree.delete(segment);
    }

    return newEvents;
  }

  /**
   * Safely split a segment that is currently in the datastructures
   * IE - a segment other than the one that is currently being processed.
   * @param seg - the segment
   * @param pt - the point
   * @returns - the new events
   */
  #splitSafely(seg: Segment, pt: BoolVec): SweepEvent[] {
    // Rounding errors can cause changes in ordering,
    // so remove afected segments and right sweep events before splitting
    // removeNode() doesn't work, so have re-find the seg
    // https://github.com/w8r/splay-tree/pull/5
    this.tree.delete(seg);
    const rightSE = seg.rightSE;
    this.queue.delete(rightSE);
    const newEvents = seg.split(pt);
    newEvents.push(rightSE);
    // splitting can trigger consumption
    if (seg.consumedBy === undefined) this.tree.add(seg);
    return newEvents;
  }
}
