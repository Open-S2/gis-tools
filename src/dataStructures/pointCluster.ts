import { fromS2Points } from '../geometry/s1/chordAngle';
import { PointIndex, PointShape, Tile } from '..';
import {
  pointAddMut as addMut,
  pointDivMutScalar as divMutScalar,
  pointFromST as fromST,
  pointMulScalar as mulScalar,
  pointNormalize as normalize,
  pointToST as toST,
} from '../geometry/s2/point';
import { convert, fromFacePosLevel, getVertices, level, range } from '../geometry';

import type { FeatureIterator } from '..';
import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type {
  Face,
  JSONCollection,
  MValue,
  Projection,
  Properties,
  S2CellId,
  VectorPoint,
  VectorPointM,
} from '../geometry';

import type { VectorStore, VectorStoreConstructor } from '../dataStore/vector';

/** The kind of input required to store a point for proper indexing */
export type ClusterStore<M extends MValue = Properties> = VectorStoreConstructor<
  PointShape<Cluster<M>>
>;

/** The type of search to use */
export type ClusterSearch = 'radial' | 'cell';

/** Options for point clustering */
export interface ClusterOptions<M extends MValue = Properties> {
  /** type of store to use. Defaults to an in memory store */
  store?: ClusterStore<M>;
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
export interface Cluster<M extends MValue = Properties> extends Properties {
  data: M;
  visited: boolean;
  value: number;
}

/**
 * Create a cluster with the correct value
 * @param data - the m-value of the point
 * @param value - the interpolated value as a number or RGBA of the cluster
 * @returns - a new cluster with the correct value and m-value
 */
function toCluster<M extends MValue = Properties>(data: M, value: number): Cluster<M> {
  return { data, visited: false, value };
}

/** Compare two data items, return true to merge data */
export type Comparitor<M extends MValue = Properties> = (a: M, b: M) => boolean;

/**
 * # Point Cluster
 *
 * ## Description
 * A cluster store to index points at each zoom level
 *
 * ## Usage
 * ```ts
 * import { PointCluster } from 'gis-tools-ts';
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
export class PointCluster<M extends MValue = Properties> {
  projection: Projection;
  layerName: string;
  minzoom: number;
  maxzoom: number;
  radius: number;
  gridSize = 512; // a default is a 512x512 pixel tile
  indexes = new Map<number, PointIndex<Cluster<M>>>();

  /**
   * @param data - if provided, the data to index
   * @param options - cluster options on how to build the cluster
   * @param maxzoomStore - the store to use for the maxzoom index
   */
  constructor(
    data?: JSONCollection<unknown, M, M>,
    options?: ClusterOptions<M>,
    maxzoomStore?: VectorStore<PointShape<Cluster<M>>>,
  ) {
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? 'default';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.radius = options?.radius ?? 40;
    // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
    for (let zoom = this.minzoom; zoom <= this.maxzoom + 1; zoom++) {
      this.indexes.set(zoom, new PointIndex<Cluster<M>>(options?.store, this.projection));
    }
    if (maxzoomStore !== undefined) {
      const maxzoomIndex = this.indexes.get(this.maxzoom);
      maxzoomIndex?.setStore(maxzoomStore);
    }
    // convert features if provided
    if (data !== undefined) this.insertFeature(data);
  }

  /**
   * Add a point to the maxzoom index. The point is a Point3D
   * @param point - the point to add
   */
  insert(point: VectorPointM<M>): void {
    const { x, y, z, m } = point;
    const maxzoomIndex = this.indexes.get(this.maxzoom);
    maxzoomIndex?.insert({ x, y, z, m: toCluster<M>(m, 1) });
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
    this.insert(fromST(face, s, t, data) as VectorPointM<M>);
  }

  /**
   * Build the clusters when done adding points
   * @param cmp_ - custom compare function
   */
  async buildClusters(cmp_?: Comparitor<M>): Promise<void> {
    const { minzoom, maxzoom } = this;
    const cmp: Comparitor<M> = cmp_ ?? ((_a: M, _b: M) => true);
    for (let zoom = maxzoom; zoom >= minzoom; zoom--) {
      const curIndex = this.indexes.get(zoom);
      const queryIndex = this.indexes.get(zoom + 1);
      if (curIndex === undefined || queryIndex === undefined) throw new Error('Index not found');
      await this.#clusterRadius(zoom, queryIndex, curIndex, cmp);
    }
    // ensure all point indexes are sorted
    for (const index of this.indexes.values()) await index.sort();
  }

  /**
   * Radial clustering
   * @param zoom - the zoom level
   * @param queryIndex - the index to query
   * @param currIndex - the index to insert into
   * @param cmp - the compare function
   */
  async #clusterRadius(
    zoom: number,
    queryIndex: PointIndex<Cluster<M>>,
    currIndex: PointIndex<Cluster<M>>,
    cmp: Comparitor<M>,
  ): Promise<void> {
    const radius = this.#getLevelRadius(zoom);
    for await (const clusterPoint of queryIndex) {
      const { point } = clusterPoint;
      const clusterData = point.m;
      if (clusterData.visited) continue;
      clusterData.visited = true;
      // setup a new weighted cluster point
      const newClusterPoint = mulScalar(point, clusterData.value as number);
      let newNumPoints = clusterData.value as number;
      // joining all points found within radius
      const points = await queryIndex.searchRadius(point, radius);
      for (const { point: foundPoint } of points) {
        const foundData = foundPoint.m;
        // only add points that match or have not been visited already
        if (!cmp(clusterData.data, foundData.data) || foundData.visited) continue;
        foundData.visited = true;
        // weighted add to newClusterPoint position
        addMut(newClusterPoint, mulScalar(foundPoint, foundData.value as number));
        newNumPoints += foundData.value as number;
      }
      // finish position average
      divMutScalar(newClusterPoint, newNumPoints);
      normalize(newClusterPoint);
      // store the new cluster point
      const { x, y, z } = newClusterPoint;
      currIndex.insert({ x, y, z, m: toCluster(clusterData.data, newNumPoints) });
    }
  }

  /**
   * @param id - the cell id
   * @returns - the data within the range of the tile id
   */
  async getCellData(id: S2CellId): Promise<undefined | PointShape<Cluster<M>>[]> {
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
  async getTile(
    id: S2CellId,
  ): Promise<undefined | Tile<Record<string, unknown>, { value: number }, M>> {
    const data = await this.getCellData(id);
    if (data === undefined) return;
    const tile = new Tile<Record<string, unknown>, { value: number }, M>(id);
    for (const { point } of data) {
      const [face, s, t] = toST(point);
      const { value, data } = point.m;
      tile.addFeature(
        {
          type: 'VectorFeature',
          face,
          geometry: { is3D: false, type: 'Point', coordinates: { x: s, y: t, m: { value } } },
          properties: data,
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
    const multiplier = this.radius / this.gridSize;
    const cell = fromFacePosLevel(0, 0n, zoom);
    const [lo, hi] = getVertices(cell);
    const angle = fromS2Points(lo, hi);
    return angle * multiplier;
  }
}
