import { MultiPolyIn } from './geomIn.js';
import { PrecisionTree } from './precision.js';
import { SweepLine } from './sweepLine.js';
import { compareSweepEvents } from './sweepEvent.js';
import { MultiPolyOut, ringOutFactory } from './geomOut.js';
import { SplayTreeSet, bboxOverlap } from '../../../../index.js';

import type { MultiPolygon, Polygon } from '../../../index.js';

/** Types of operations to apply to MultiPolygons */
export type OperationType = 'union' | 'intersection' | 'xor' | 'difference';

/** Operation Store to be passed around for reuse */
export interface Operation {
  type: OperationType;
  precision: PrecisionTree;
  numPolys: number;
}

/**
 * Create a basic operation for testing
 * @param epsilon - set the precision
 * @returns - a basic operation for testing
 */
export function testOperation(epsilon?: number): Operation {
  return {
    type: 'union',
    precision: new PrecisionTree(epsilon),
    numPolys: 1,
  };
}

/**
 * Run an operation on a set of MultiPolygons
 * @param type - Type of operation
 * @param epsilon - set the precision
 * @param polys - collection of polygons
 * @returns - the resultant MultiPolygon geometry
 */
export function booleanOp(
  type: OperationType,
  epsilon?: number,
  ...polys: (MultiPolygon | Polygon)[]
): MultiPolygon {
  const operation: Operation = {
    type,
    precision: new PrecisionTree(epsilon),
    numPolys: polys.length,
  };
  // Convert inputs to MultiPoly objects
  const multipolys = [];
  for (let i = 0, pl = polys.length; i < pl; i++) {
    const next = polys[i];
    if (next.length === 0) continue; // skip empty multipolygons
    multipolys.push(new MultiPolyIn(next, i === 0, operation));
  }

  // BBox optimization for difference operation
  // If the bbox of a multipolygon that's part of the clipping doesn't
  // intersect the bbox of the subject at all, we can just drop that
  // multiploygon.
  if (type === 'difference') {
    // in place removal
    const subject = multipolys[0];
    let i = 1;
    while (i < multipolys.length) {
      if (bboxOverlap(multipolys[i].bbox, subject.bbox) !== undefined) i++;
      else multipolys.splice(i, 1);
    }
  }

  // BBox optimization for intersection operation
  // If we can find any pair of multipolygons whose bbox does not overlap,
  // then the result will be empty.
  if (type === 'intersection') {
    // TODO: this is O(n^2) in number of polygons. By sorting the bboxes,
    // it could be optimized to O(n * ln(n))
    for (let i = 0, iMax = multipolys.length; i < iMax; i++) {
      const mpA = multipolys[i];
      for (let j = i + 1, jMax = multipolys.length; j < jMax; j++) {
        if (bboxOverlap(mpA.bbox, multipolys[j].bbox) === undefined) return [];
      }
    }
  }

  // Put segment endpoints in a priority queue
  const queue = new SplayTreeSet(compareSweepEvents);
  for (const multipoly of multipolys) {
    const sweepEvents = multipoly.getSweepEvents();
    for (const sweepEvent of sweepEvents) queue.add(sweepEvent);
  }

  // Pass the sweep line over those endpoints
  const sweepLine = new SweepLine(queue);
  let evt = undefined;
  if (queue.length !== 0) {
    evt = queue.first()!;
    queue.delete(evt);
  }
  while (evt !== undefined) {
    const newEvents = sweepLine.process(evt);
    for (let i = 0, iMax = newEvents.length; i < iMax; i++) {
      const evt = newEvents[i];
      if (evt.consumedBy === undefined) queue.add(evt);
    }
    if (queue.length !== 0) {
      evt = queue.first()!;
      queue.delete(evt);
    } else {
      evt = undefined;
    }
  }

  // Collect and compile segments we're keeping into a multipolygon
  const ringsOut = ringOutFactory(sweepLine.segments, operation);
  const result = new MultiPolyOut(ringsOut);
  return result.getGeom();
}
