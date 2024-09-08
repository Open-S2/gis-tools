import { fromS2Points } from 's2-tools/geometry/s1/chordAngle';
import PointIndex, { Point } from './pointIndex';
import { Tile, fromFacePosLevel, getVertices, level, range } from '../geometry';
import {
  addMut,
  divMutScalar,
  fromLonLat,
  fromST,
  mulScalar,
  normalize,
  toST,
} from '../geometry/s2/point';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type { Face, Point3D, Projection, S2CellId } from '../geometry';

/** Options for point clustering */
export interface ClusterOptions {
  /** projection to use */
  projection?: Projection;
  /** Name of the layer to build when requesting a tile */
  layerName?: string;
  /** min zoom to generate clusters on */
  minzoom?: number;
  /** max zoom level to cluster the points on */
  maxzoom?: number;
  /** cluster radius in pixels */
  radius?: number;
}

/** A cluster is a storage device to maintain groups of information in a cluster */
export class Cluster<T> {
  visited: boolean = false;
  sum: number = 1;

  /**
   * @param ref - the reference data
   */
  constructor(public ref: T) {}

  /**
   * @param ref - the reference data
   * @param sum - the sum of points
   * @returns - a new cluster
   */
  static fromSum<T>(ref: T, sum: number): Cluster<T> {
    const cluster = new Cluster(ref);
    cluster.sum = sum;
    return cluster;
  }
}

/** Compare two data items, return true to merge data */
export type Comparitor<T> = (a: T, b: T) => boolean;

/** A cluster store to index points at each zoom level */
export default class PointCluster<T> {
  projection: Projection;
  layerName: string;
  minzoom: number;
  maxzoom: number;
  radius: number;
  // { [zoom]: Index }
  indexes = new Map<number, PointIndex<Cluster<T>>>();

  /** @param options - cluster options on how to build the cluster */
  constructor(options?: ClusterOptions) {
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? '__cluster';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.radius = options?.radius ?? 40;
    for (let zoom = this.minzoom; zoom <= this.maxzoom; zoom++) {
      this.indexes.set(zoom, new PointIndex<Cluster<T>>());
    }
  }

  /**
   * Add a point to the maxzoom index
   * @param point - the point to add
   * @param data - the data associated with the point
   */
  insert(point: Point3D, data: T): void {
    const maxzoomIndex = this.indexes.get(this.maxzoom);
    maxzoomIndex?.insert(point, new Cluster(data));
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
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: T): void {
    this.insert(fromST(face, s, t), data);
  }

  /**
   * Build the clusters when done adding points
   * @param cmp_ - custom compare function
   */
  buildClusters(cmp_?: Comparitor<T>): void {
    const { minzoom, maxzoom } = this;
    // const cmp = cmp_ orelse defaultCmp;
    const cmp: Comparitor<T> = cmp_ ?? ((_a: T, _b: T) => true);
    for (let zoom = maxzoom; zoom > minzoom; zoom--) {
      const curIndex = this.indexes.get(zoom);
      const queryIndex = this.indexes.get(zoom - 1);
      if (curIndex === undefined || queryIndex === undefined) throw new Error('Index not found');
      if (zoom === maxzoom) this.#cluster(zoom, queryIndex, curIndex, cmp);
    }
  }

  /**
   * @param zoom - the zoom level
   * @param queryIndex - the index to query
   * @param currIndex - the index to insert into
   * @param cmp - the compare function
   */
  #cluster(
    zoom: number,
    queryIndex: PointIndex<Cluster<T>>,
    currIndex: PointIndex<Cluster<T>>,
    cmp: Comparitor<T>,
  ): void {
    const radius = this.#getLevelRadius(zoom);
    for (const clusterPoint of queryIndex.iterate()) {
      const { point, data: clusterData } = clusterPoint;
      if (clusterData.visited) continue;
      clusterData.visited = true;
      // setup a new weighted cluster point
      const newClusterPoint = mulScalar(point, clusterData.sum);
      let newNumPoints = clusterData.sum;
      // joining all points found within radius
      const points = queryIndex.searchRadius(point, radius);
      for (const { point: foundPoint, data: foundData } of points) {
        // only add points that match or have not been visited already
        if (!cmp(clusterData.ref, foundData.ref) || foundData.visited) continue;
        foundData.visited = true;
        // weighted add to newClusterPoint position
        addMut(newClusterPoint, mulScalar(foundPoint, foundData.sum));
        newNumPoints += foundData.sum;
      }
      // finish position average
      divMutScalar(newClusterPoint, newNumPoints);
      normalize(newClusterPoint);
      // store the new cluster point
      currIndex.insert(newClusterPoint, Cluster.fromSum(clusterData.ref, newNumPoints));
    }
  }

  /**
   * @param id - the cell id
   * @returns - the data within the range of the tile id
   */
  getCellData(id: S2CellId): undefined | Point<Cluster<T>>[] {
    const { minzoom, maxzoom, indexes } = this;
    const zoom = level(id);
    if (zoom < minzoom) return;
    const [min, max] = range(id);
    const levelIndex = indexes.get(Math.min(zoom, maxzoom));

    return levelIndex?.searchRange(min, max);
  }

  /**
   * @param id - the id of the vector tile
   * @returns - the vector tile
   */
  getTile(id: S2CellId): undefined | Tile {
    const data = this.getCellData(id);
    if (data === undefined) return;
    const tile = new Tile(id);
    for (const { point, data: cluster } of data) {
      const [face, s, t] = toST(point);
      const { sum, ref } = cluster;
      tile.addFeature(
        {
          type: 'VectorFeature',
          face,
          geometry: { is3D: false, type: 'Point', coordinates: { x: s, y: t } },
          properties: { ...ref, __sum: sum },
        },
        this.layerName,
      );
    }

    // transform the geometry to be relative to the tile
    tile.transform(0, this.maxzoom);

    return tile;
  }

  /**
   * Get a S1ChordAngle relative to a tile zoom level
   * @param zoom - the zoom level to build a radius
   * @returns - the appropriate radius for the given zoom
   */
  #getLevelRadius(zoom: number): S1ChordAngle {
    const multiplier = this.radius;
    const cell = fromFacePosLevel(0, 0n, zoom);
    const [lo, hi] = getVertices(cell);
    const angle = fromS2Points(lo, hi);
    return angle * multiplier;
  }
}
