import { Vector } from '../dataStore';
import { chordAngFromS2Points } from '../geometry/s1/chordAngle';
import { pointFromST } from '../geometry/s2/point';
import { capFromS1ChordAngle, capGetIntersectingCells } from '../geometry/s2/cap';
import { compareIDs, convert, idFromS2Point, idRange } from '../geometry';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type {
  Face,
  FeatureIterator,
  JSONCollection,
  MValue,
  Projection,
  Properties,
  RGBA,
  S2CellId,
  VectorPoint,
  VectorPointM,
} from '..';
import type { VectorStore, VectorStoreConstructor } from '../dataStore';

/** The kind of input required to store a point for proper indexing */
export interface PointShape<M extends MValue = Properties> {
  cell: S2CellId;
  point: VectorPointM<M>;
}

/**
 * # Point Index
 *
 * ## Description
 * An index of cells with radius queries
 * Assumes the data is compatible with {@link https://open-s2.github.io/s2json/types/Properties.html}
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
export class PointIndex<M extends MValue = Properties | RGBA> {
  #store: VectorStore<PointShape<M>>;
  #unsorted: boolean = false;

  /**
   * @param store - the store to index. May be an in memory or disk
   * @param projection - the projection of the data, defaults to S2
   */
  constructor(
    store: VectorStoreConstructor<PointShape<M>> = Vector,
    private projection: Projection = 'S2',
  ) {
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
  insertID(cell: S2CellId, point: VectorPointM<M>): void {
    this.#store.push({ cell, point });
    this.#unsorted = true;
  }

  /**
   * Insert a point3D and its corresponding data to the index
   * @param point - the point to be indexed
   */
  insert(point: VectorPointM<M>): void {
    this.insertID(idFromS2Point(point), point);
  }

  /**
   * Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
   * it will use the feature properties data
   * @param reader - a reader containing the input data
   */
  async insertReader(reader: FeatureIterator<unknown, M, M>): Promise<void> {
    for await (const feature of reader) this.insertFeature(feature);
  }

  /**
   * Add a vector feature. It will try to use the M-value first, but if it doesn't exist
   * it will use the feature properties data
   * @param data - any source of data like a feature collection or features themselves
   */
  insertFeature(data: JSONCollection<unknown, M, M>): void {
    const features = convert(this.projection, data, undefined, undefined, undefined, true);
    for (const { face = 0, geometry, properties } of features) {
      const { type, coordinates } = geometry;
      if (type === 'Point') {
        const { x: s, y: t, m } = coordinates;
        this.#insertFaceST(face, s, t, m ?? properties);
      } else if (type === 'MultiPoint') {
        for (const point of coordinates) {
          const { x: s, y: t, m } = point;
          this.#insertFaceST(face, s, t, m ?? properties);
        }
      }
    }
  }

  /**
   * Add a lon-lat pair to the cluster
   * @param ll - lon-lat vector point in degrees
   */
  insertLonLat(ll: VectorPoint<M>): void {
    this.insertFeature({
      type: 'VectorFeature',
      properties: ll.m ?? ({} as M),
      geometry: { type: 'Point', coordinates: ll, is3D: false },
    });
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: M): void {
    this.insertFeature({
      type: 'S2Feature',
      face,
      properties: data,
      geometry: { type: 'Point', coordinates: { x: s, y: t, m: data }, is3D: false },
    });
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  #insertFaceST(face: Face, s: number, t: number, data: M): void {
    this.insert(pointFromST(face, s, t, data) as VectorPointM<M>);
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
   * @param low - the lower bound. If high is not provided, the low-high range will be created from the low
   * @param high - the upper bound
   * @param maxResults - the maximum number of results to return
   * @returns the points in the range
   */
  async searchRange(
    low: S2CellId,
    high?: S2CellId,
    maxResults = Infinity,
  ): Promise<PointShape<M>[]> {
    await this.sort();
    const res: PointShape<M>[] = [];
    if (high === undefined) {
      const [lo, hi] = idRange(low);
      low = lo;
      high = hi;
    }
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
   * TODO: Adjust the radius for the WM projection. Really not a massive issue thogh just adjust your calcuation for now
   * Search for points within a given radius of a target point
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
    const cap = capFromS1ChordAngle<undefined>(target, radius, undefined);
    for (const cell of capGetIntersectingCells(cap)) {
      // iterate each covering s2cell min-max range on store. check distance from found
      // store Cells to target and if within radius add to results
      const [min, max] = idRange(cell);
      for (const point of await this.searchRange(min, max)) {
        if (chordAngFromS2Points(target, point.point) < radius) res.push(point);
        if (res.length >= maxResults) break;
      }
    }

    return res;
  }
}
