import { SplayTreeSet, orient2d } from '../../../../index.js';

import type { BoolVec } from './vector.js';

/**
 * # Precision Tree
 *
 * ## Overview
 * A data structure for rounding/managing precision.
 */
export class PrecisionTree {
  xTree: SplayTreeSet<number>;
  yTree: SplayTreeSet<number>;
  /** @param epsilon - set a default epsilon for snapping/rounding managment */
  constructor(public epsilon: number = Number.EPSILON * 2) {
    this.xTree = new SplayTreeSet(this.compare.bind(this));
    this.yTree = new SplayTreeSet(this.compare.bind(this));
    this.snap({ x: 0, y: 0 });
  }

  /**
   * Snap a coordinate
   * @param coord - the input coordinate
   * @param tree - the tree to snap to
   * @returns - the snapped coordinate
   */
  snapCoord(coord: number, tree: SplayTreeSet<number>): number {
    return tree.add(coord);
  }

  /**
   * Snap a vector's coordinates
   * @param v - the vector to snap
   * @returns - the snapped vector
   */
  snap(v: BoolVec): BoolVec {
    return {
      x: this.snapCoord(v.x, this.xTree),
      y: this.snapCoord(v.y, this.yTree),
    };
  }

  /**
   * Get the orientation of the given points.
   * @param a - the start point of the segment
   * @param b - the end point of the segment
   * @param c - the reference point
   * @returns - 0 if collinear, -1 if counterclockwise, 1 if clockwise
   */
  orient(a: BoolVec, b: BoolVec, c: BoolVec): number {
    const area = orient2d(a.x, a.y, b.x, b.y, c.x, c.y);

    const area2 = Math.pow(area, 2);
    if (area2 <= (Math.pow(c.x - a.x, 2) + Math.pow(c.y - a.y, 2)) * this.epsilon) return 0;

    return area > 0 ? 1 : area < 0 ? -1 : 0;
  }

  /**
   * set a default epsilon tolerance for snapping/rounding managment
   * @param a - the first number
   * @param b - the second number
   * @returns - -1 if a < b, 0 if a == b, 1 if a > b
   */
  compare(a: number, b: number): number {
    if (Math.abs(a - b) <= this.epsilon) return 0;

    return a > b ? 1 : a < b ? -1 : 0;
  }
}
