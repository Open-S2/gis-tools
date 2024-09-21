import Uint64CellGenerator from '../wasm/uint64';
import { fromS2Points } from '../geometry/s1/chordAngle';
import { range } from '../geometry';
import { fromS1ChordAngle, getIntersectingCells } from '../geometry/s2/cap';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type { Uint64Cell } from '../wasm/uint64';
import type { Point3D, S2CellId } from '../geometry';

/** A point shape to be indexed */
export class PointShape<T> {
  /**
   * @param cell - the cell that defines the point
   * @param point - the point to track current location
   * @param data - the data associated with the point
   */
  constructor(
    public cell: Uint64Cell,
    public point: Point3D,
    public data: T,
  ) {}
}

/** An index of cells with radius queries */
export default class PointIndex<T> {
  #store: PointShape<T>[] = [];
  #unsorted: boolean = false;
  cellGen = new Uint64CellGenerator();

  /**
   * @param point - the point to be indexed
   * @param data - the data associated with the point
   */
  insert(point: Point3D, data: T): void {
    const cell = this.cellGen.fromS2Point(point);
    this.#store.push(new PointShape(cell, point, data));
    this.#unsorted = true;
  }

  /**
   * iterate through the points
   * @returns an iterator
   */
  [Symbol.iterator](): IterableIterator<PointShape<T>> {
    this.#sort();
    return this.#store.values();
  }

  /**
   * add points from perhaps another index
   * @param points - array of the points to add
   */
  insertPoints(points: PointShape<T>[]): void {
    this.#store.push(...points);
    this.#unsorted = true;
  }

  /** Sort the index in place if unsorted */
  #sort(): void {
    if (!this.#unsorted) return;

    this.#store.sort((a, b) => {
      return a.cell.compare(b.cell);
    });
    this.#unsorted = false;
  }

  /**
   * Find the starting index of a search
   * @param id - input id to seek the starting index of the search
   * @returns the starting index
   */
  lowerBound(id: S2CellId): number {
    const cellID = this.cellGen.fromBigInt(id);
    this.#sort();
    // lower bound search
    let lo: number = 0;
    let hi: number = this.#store.length;
    let mid: number;

    while (lo < hi) {
      mid = lo + (hi - lo) / 2;
      if (this.#store[mid].cell.compare(cellID) === -1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    return lo;
  }

  /**
   * @param low - the lower bound
   * @param high - the upper bound
   * @returns the points in the range
   */
  searchRange(low: S2CellId, high: S2CellId): PointShape<T>[] {
    this.#sort();
    const res: PointShape<T>[] = [];
    let lo = this.lowerBound(low);
    const hiID = this.cellGen.fromBigInt(high);

    while (lo < this.#store.length && this.#store[lo].cell.compare(hiID) <= 0) {
      res.push(this.#store[lo]);
      lo++;
    }

    return res;
  }

  /**
   * @param target - the point to search
   * @param radius - the search radius
   * @returns the points within the radius
   */
  searchRadius(target: Point3D, radius: S1ChordAngle): PointShape<T>[] {
    this.#sort();
    const res: PointShape<T>[] = [];
    if (radius < 0) return res;
    const cap = fromS1ChordAngle<undefined>(target, radius, undefined);
    for (const cell of getIntersectingCells(cap)) {
      // iterate each covering s2cell min-max range on store. check distance from found
      // store Cells to target and if within radius add to results
      const [min, max] = range(cell);
      for (const point of this.searchRange(min, max)) {
        if (fromS2Points(target, point.point) < radius) res.push(point);
      }
    }

    return res;
  }
}
