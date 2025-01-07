import { Tile } from '../dataStructures';
import { convert } from '../geometry/tools/convert';
import { fromS2Points } from '../geometry/s1/chordAngle';
import { PointShape as Point, PointIndex } from './pointIndex';
import {
  addMut,
  divMutScalar,
  fromLonLat,
  fromST,
  mulScalar,
  normalize,
  toST,
} from '../geometry/s2/point';
import { fromFacePosLevel, getVertices, level, range } from '../geometry';

import type { PointShape } from './pointIndex';
import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type { Face, JSONCollection, Point3D, Projection, Properties, S2CellId } from '../geometry';

import type { VectorStore, VectorStoreConstructor } from '../dataStore/vector';

/** The kind of input required to store a point for proper indexing */
export type ClusterStore = VectorStoreConstructor<PointShape<Cluster>>;

/** Options for point clustering */
export interface ClusterOptions {
  /** type of store to use. Defaults to an in memory store */
  store?: ClusterStore;
  /** projection to use */
  projection?: Projection;
  /** Name of the layer to build when requesting a tile */
  layerName?: string;
  /** min zoom to generate clusters on */
  minzoom?: number;
  /** max zoom level to cluster the points on */
  maxzoom?: number;
  /** cluster radius in pixels relative to a 512x512 pixel tile */
  radius?: number;
}

/** A cluster is a storage device to maintain groups of information in a cluster */
export interface Cluster {
  properties: Properties;
  visited: boolean;
  sum: number;
}

/**
 * @param properties - the properties associated with the cluster
 * @returns - a new cluster
 */
function newCluster(properties: Properties): Cluster {
  return { properties, visited: false, sum: 1 };
}

/**
 * Create a cluster with the correct sum
 * @param properties - the properties associated with the cluster
 * @param sum - the sum of the cluster
 * @returns - a new cluster with the correct sum and properties data
 */
function sumToCluster(properties: Properties, sum: number): Cluster {
  return { properties, visited: false, sum };
}

/** Compare two data items, return true to merge data */
export type Comparitor = (a: Properties, b: Properties) => boolean;

/**
 * # Point Cluster
 *
 * ## Description
 * A cluster store to index points at each zoom level
 *
 * ## Usage
 * ```ts
 * import { PointCluster } from 's2-tools';
 * const pointCluster = new PointCluster();
 *
 * // add a lon-lat
 * pointCluster.insertLonLat(lon, lat, data);
 * // add an STPoint
 * pointCluster.insertFaceST(face, s, t, data);
 *
 * // after adding data build the clusters
 * await pointCluster.buildClusters();
 *
 * // get the clusters for a tile
 * const tile = await pointCluster.getTile(id);
 * // or get the raw cluster data
 * const clusters = await pointCluster.getCellData(id);
 * ```
 */
export class PointCluster {
  projection: Projection;
  layerName: string;
  minzoom: number;
  maxzoom: number;
  radius: number;
  extent = 512; // a 512x512 pixel tile
  indexes = new Map<number, PointIndex<Cluster>>();

  /**
   * @param data - if provided, the data to index
   * @param options - cluster options on how to build the cluster
   * @param maxzoomStore - the store to use for the maxzoom index
   */
  constructor(
    data?: JSONCollection,
    options?: ClusterOptions,
    maxzoomStore?: VectorStore<PointShape<Cluster>>,
  ) {
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? 'default';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.radius = options?.radius ?? 40;
    for (let zoom = this.minzoom; zoom <= this.maxzoom; zoom++) {
      this.indexes.set(zoom, new PointIndex<Cluster>(options?.store));
    }
    if (maxzoomStore !== undefined) {
      const maxzoomIndex = this.indexes.get(this.maxzoom);
      maxzoomIndex?.setStore(maxzoomStore);
    }
    // convert features if provided
    if (data !== undefined) {
      const features = convert(this.projection, data, false, undefined, this.maxzoom, true);
      for (const feature of features) {
        const face = feature.face ?? 0;
        const { type, coordinates } = feature.geometry;
        if (type === 'Point') {
          const { x: s, y: t } = coordinates;
          this.insertFaceST(face, s, t, feature.properties);
        }
      }
    }
  }

  /**
   * Add a point to the maxzoom index
   * @param point - the point to add
   * @param data - the data associated with the point
   */
  insert(point: Point3D, data: Properties): void {
    const maxzoomIndex = this.indexes.get(this.maxzoom);
    maxzoomIndex?.insert(point, newCluster(data));
  }

  /**
   * Add a lon-lat pair to the cluster
   * @param lon - longitude in degrees
   * @param lat - latitude in degrees
   * @param data - the data associated with the point
   */
  insertLonLat(lon: number, lat: number, data: Properties): void {
    this.insert(fromLonLat(lon, lat), data);
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: Properties): void {
    this.insert(fromST(face, s, t), data);
  }

  /**
   * Build the clusters when done adding points
   * @param cmp_ - custom compare function
   */
  async buildClusters(cmp_?: Comparitor): Promise<void> {
    const { minzoom, maxzoom } = this;
    const cmp: Comparitor = cmp_ ?? ((_a: Properties, _b: Properties) => true);
    for (let zoom = maxzoom - 1; zoom >= minzoom; zoom--) {
      const curIndex = this.indexes.get(zoom);
      const queryIndex = this.indexes.get(zoom + 1);
      if (curIndex === undefined || queryIndex === undefined) throw new Error('Index not found');
      await this.#cluster(zoom, queryIndex, curIndex, cmp);
    }
    // ensure all point indexes are sorted
    for (const index of this.indexes.values()) await index.sort();
  }

  /**
   * @param zoom - the zoom level
   * @param queryIndex - the index to query
   * @param currIndex - the index to insert into
   * @param cmp - the compare function
   */
  async #cluster(
    zoom: number,
    queryIndex: PointIndex<Cluster>,
    currIndex: PointIndex<Cluster>,
    cmp: Comparitor,
  ): Promise<void> {
    const radius = this.#getLevelRadius(zoom);
    for await (const clusterPoint of queryIndex) {
      const { point, data: clusterData } = clusterPoint;
      if (clusterData.visited) continue;
      clusterData.visited = true;
      // setup a new weighted cluster point
      const newClusterPoint = mulScalar(point, clusterData.sum);
      let newNumPoints = clusterData.sum;
      // joining all points found within radius
      const points = await queryIndex.searchRadius(point, radius);
      for (const { point: foundPoint, data: foundData } of points) {
        // only add points that match or have not been visited already
        if (!cmp(clusterData.properties, foundData.properties) || foundData.visited) continue;
        foundData.visited = true;
        // weighted add to newClusterPoint position
        addMut(newClusterPoint, mulScalar(foundPoint, foundData.sum));
        newNumPoints += foundData.sum;
      }
      // finish position average
      divMutScalar(newClusterPoint, newNumPoints);
      normalize(newClusterPoint);
      // store the new cluster point
      currIndex.insert(newClusterPoint, sumToCluster(clusterData.properties, newNumPoints));
    }
  }

  /**
   * @param id - the cell id
   * @returns - the data within the range of the tile id
   */
  async getCellData(id: S2CellId): Promise<undefined | Point<Cluster>[]> {
    const { minzoom, maxzoom, indexes } = this;
    const zoom = level(id);
    if (zoom < minzoom) return;
    const [min, max] = range(id);
    const levelIndex = indexes.get(Math.min(zoom, maxzoom));

    return await levelIndex?.searchRange(min, max);
  }

  /**
   * @param id - the id of the vector tile
   * @returns - the vector tile
   */
  async getTile(id: S2CellId): Promise<undefined | Tile> {
    const data = await this.getCellData(id);
    if (data === undefined) return;
    const tile = new Tile(id);
    for (const { point, data: cluster } of data) {
      const [face, s, t] = toST(point);
      const { sum, properties } = cluster;
      tile.addFeature(
        {
          type: 'VectorFeature',
          face,
          geometry: { is3D: false, type: 'Point', coordinates: { x: s, y: t, m: { sum } } },
          properties,
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
    const multiplier = this.radius / this.extent;
    const cell = fromFacePosLevel(0, 0n, zoom);
    const [lo, hi] = getVertices(cell);
    const angle = fromS2Points(lo, hi);
    return angle * multiplier;
  }
}
