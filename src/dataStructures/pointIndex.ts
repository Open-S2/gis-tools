import { Vector } from '../dataStore';
import { fromS2Points } from '../geometry/s1/chordAngle';
import { compareIDs, fromS2Point, range, toWM } from '../geometry';
import { fromS1ChordAngle, getIntersectingCells } from '../geometry/s2/cap';

import { fromLonLat, fromST } from '../geometry/s2/point';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type {
  Face,
  FeatureIterator,
  MValue,
  Properties,
  S2CellId,
  VectorFeatures,
  VectorPoint,
} from '..';
import type { VectorStore, VectorStoreConstructor } from '../dataStore';

/** The kind of input required to store a point for proper indexing */
export interface PointShape<M extends MValue = Properties> {
  cell: S2CellId;
  point: VectorPoint<M>;
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
 * import { PointIndex } from 'gis-tools-ts';
 * import { FileVector } from 'gis-tools-ts/file';
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
export class PointIndex<M extends MValue = Properties> {
  #store: VectorStore<PointShape<M>>;
  #unsorted: boolean = false;

  /** @param store - the store to index. May be an in memory or disk */
  constructor(store: VectorStoreConstructor<PointShape<M>> = Vector) {
    this.#store = new store();
  }

  /**
   * Set the index store to a defined one. Useful for file based stores where we want to reuse data
   * @param store - the index store
   */
  setStore(store: VectorStore<PointShape<M>>): void {
    this.#store = store;
  }

  /**
   * Insert a cell with the point and its corresponding data to the index
   * @param cell - the cell id to be indexed
   * @param point - the point to be indexed
   */
  insertID(cell: S2CellId, point: VectorPoint<M>): void {
    this.#store.push({ cell, point });
    this.#unsorted = true;
  }

  /**
   * Insert a point3D and its corresponding data to the index
   * @param point - the point to be indexed
   */
  insert(point: VectorPoint<M>): void {
    this.insertID(fromS2Point(point), point);
  }

  /**
   * Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
   * it will use the feature properties data
   * @param reader - a reader containing the input data
   */
  async insertReader(reader: FeatureIterator<Record<string, unknown>, M, M>): Promise<void> {
    for await (const feature of reader) this.insertFeature(feature);
  }

  /**
   * Add a feature to the index
   * @param feature - vector feature (either S2 or WM)
   */
  insertFeature(feature: VectorFeatures<Record<string, unknown>, M, M>): void {
    if (feature.geometry.type !== 'Point' && feature.geometry.type !== 'MultiPoint') return;
    const {
      geometry: { coordinates, type },
    } = feature.type === 'S2Feature' ? toWM(feature) : feature;
    if (type === 'Point') {
      if (coordinates.m === undefined) coordinates.m = feature.properties;
      this.insertLonLat(coordinates);
    } else if (type === 'MultiPoint') {
      for (const point of coordinates) {
        if (point.m === undefined) point.m = feature.properties;
        this.insertLonLat(point);
      }
    }
  }

  /**
   * Add a lon-lat pair to the cluster
   * @param ll - lon-lat vector point in degrees
   */
  insertLonLat(ll: VectorPoint<M>): void {
    this.insert(fromLonLat(ll));
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: M): void {
    this.insert(fromST(face, s, t, data));
  }

  /**
   * iterate through the points
   * @yields a PointShape<T>
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<PointShape<M>> {
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
  async lowerBound(id: S2CellId): Promise<number> {
    await this.sort();
    // lower bound search
    let lo: number = 0;
    let hi: number = this.#store.length;
    let mid: number;

    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const { cell: midCell } = await this.#store.get(mid);
      if (compareIDs(midCell, id) === -1) {
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
  async searchRange(
    low: S2CellId,
    high: S2CellId,
    maxResults = Infinity,
  ): Promise<PointShape<M>[]> {
    await this.sort();
    const res: PointShape<M>[] = [];
    let loIdx = await this.lowerBound(low);

    while (true) {
      if (loIdx >= this.#store.length) break;
      const currLo = await this.#store.get(loIdx);
      if (compareIDs(currLo.cell, high) > 0) break;
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
    target: VectorPoint,
    radius: S1ChordAngle,
    maxResults = Infinity,
  ): Promise<PointShape<M>[]> {
    await this.sort();
    const res: PointShape<M>[] = [];
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
