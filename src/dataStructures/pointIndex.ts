import { Vector } from '../dataStore';
import { fromS2Points } from '../geometry/s1/chordAngle';
import { range } from '../geometry';
import { compare, fromS2Point, toCell } from '../dataStructures/uint64';
import { fromS1ChordAngle, getIntersectingCells } from '../geometry/s2/cap';

import { fromLonLat, fromST } from '../geometry/s2/point';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type { Face, Point3D, Properties } from '..';
import type { Uint64, Uint64Cell } from '../dataStructures/uint64';
import type { VectorStore, VectorStoreConstructor } from '../dataStore';

/** The kind of input required to store a point for proper indexing */
export interface PointShape<T extends Properties = Properties> {
  cell: Uint64Cell;
  point: Point3D;
  data: T;
}

/**
 * # Point Index
 *
 * ## Description
 * An index of cells with radius queries
 * Assumes the data is compatible with {@link Properties}
 *
 * ## Usage
 * ```ts
 * import { PointIndex } from 'gis-tools';
 * import { FileVector } from 'gis-tools/file';
 *
 * const pointIndex = new PointIndex();
 * // or used a file based store
 * const pointIndex = new PointIndex(FileVector);
 *
 * // insert a lon-lat
 * pointIndex.insertLonLat(lon, lat, data);
 * // insert an STPoint
 * pointIndex.insertFaceST(face, s, t, data);
 *
 * // after adding data build the index. NOTE: You don't have to call this, it will be called
 * // automatically when making a query
 * await pointIndex.sort();
 *
 * // you can search a range
 * const points = await pointIndex.searchRange(low, high);
 * // or a radius
 * const points = await pointIndex.searchRadius(center, radius);
 * ```
 */
export class PointIndex<T extends Properties = Properties> {
  #store: VectorStore<PointShape<T>>;
  #unsorted: boolean = false;

  /** @param store - the store to index. May be an in memory or disk */
  constructor(store: VectorStoreConstructor<PointShape<T>> = Vector) {
    this.#store = new store();
  }

  /**
   * Set the index store to a defined one. Useful for file based stores where we want to reuse data
   * @param store - the index store
   */
  setStore(store: VectorStore<PointShape<T>>): void {
    this.#store = store;
  }

  /**
   * Insert a point3D and its corresponding data to the index
   * @param point - the point to be indexed
   * @param data - the data associated with the point
   */
  insert(point: Point3D, data: T): void {
    this.#store.push({ cell: fromS2Point(point), point, data });
    this.#unsorted = true;
  }

  /**
   * Add a lon-lat pair to the cluster
   * @param lon - longitude in degrees
   * @param lat - latitude in degrees
   * @param data - the data associated with the point
   */
  insertLonLat(lon: number, lat: number, data: T): void {
    this.insert(fromLonLat(lon, lat), data);
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: T): void {
    this.insert(fromST(face, s, t), data);
  }

  /**
   * iterate through the points
   * @yields a PointShape<T>
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<PointShape<T>> {
    await this.sort();
    yield* this.#store;
  }

  /** Sort the index in place if unsorted */
  async sort(): Promise<void> {
    if (!this.#unsorted) return;
    await this.#store.sort();
    this.#unsorted = false;
  }

  /**
   * Find the starting index of a search
   * @param id - input id to seek the starting index of the search
   * @returns the starting index
   */
  async lowerBound(id: Uint64): Promise<number> {
    const cellID = toCell(id);
    await this.sort();
    // lower bound search
    let lo: number = 0;
    let hi: number = this.#store.length;
    let mid: number;

    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const { cell: midCell } = await this.#store.get(mid);
      if (compare(midCell, cellID) === -1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    return lo;
  }

  /**
   * Search for points given a range of low and high ids
   * @param low - the lower bound
   * @param high - the upper bound
   * @param maxResults - the maximum number of results to return
   * @returns the points in the range
   */
  async searchRange(low: Uint64, high: Uint64, maxResults = Infinity): Promise<PointShape<T>[]> {
    await this.sort();
    const res: PointShape<T>[] = [];
    let loIdx = await this.lowerBound(low);
    const hiID = toCell(high);

    while (true) {
      if (loIdx >= this.#store.length) break;
      const currLo = await this.#store.get(loIdx);
      if (compare(currLo.cell, hiID) > 0) break;
      res.push(currLo);
      if (res.length >= maxResults) break;
      loIdx++;
    }

    return res;
  }

  /**
   * @param target - the point to search
   * @param radius - the search radius
   * @param maxResults - the maximum number of results
   * @returns the points within the radius
   */
  async searchRadius(
    target: Point3D,
    radius: S1ChordAngle,
    maxResults = Infinity,
  ): Promise<PointShape<T>[]> {
    await this.sort();
    const res: PointShape<T>[] = [];
    if (radius < 0) return res;
    const cap = fromS1ChordAngle<undefined>(target, radius, undefined);
    for (const cell of getIntersectingCells(cap)) {
      // iterate each covering s2cell min-max range on store. check distance from found
      // store Cells to target and if within radius add to results
      const [min, max] = range(cell);
      for (const point of await this.searchRange(min, max)) {
        if (fromS2Points(target, point.point) < radius) res.push(point);
        if (res.length >= maxResults) break;
      }
    }

    return res;
  }
}
