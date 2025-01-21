import { fromS2Points } from '../geometry/s1/chordAngle';
import {
  PointIndex,
  PointShape,
  Tile,
  defaultGetInterpolateCurrentValue,
  getInterpolation,
  getRGBAInterpolation,
} from '..';
import {
  addMut,
  divMutScalar,
  fromLonLat,
  fromST,
  mulScalar,
  normalize,
  toST,
} from '../geometry/s2/point';
import {
  convert,
  fromFacePosLevel,
  getVertices,
  level,
  parent,
  range,
  toS2Point,
  toWM,
} from '../geometry';

import type { S1ChordAngle } from '../geometry/s1/chordAngle';
import type {
  Face,
  JSONCollection,
  MValue,
  Projection,
  Properties,
  S2CellId,
  VectorPoint,
} from '../geometry';
import type {
  FeatureIterator,
  GetInterpolateValue,
  InterpolationFunction,
  InterpolationMethod,
  RGBA,
  RGBAInterpolationFunction,
  VectorFeatures,
} from '..';

import type { VectorStore, VectorStoreConstructor } from '../dataStore/vector';

/** The kind of input required to store a point for proper indexing */
export type ClusterStore<M extends MValue = Properties> = VectorStoreConstructor<
  PointShape<Cluster<M>>
>;

/** The type of search to use */
export type ClusterSearch = 'radial' | 'cell';

/** Options for point clustering */
export interface BaseClusterOptions<M extends MValue = Properties> {
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
  /** Specify the type of clustering [default: 'radial'] */
  search?: ClusterSearch;
  /** Used by cell search to specify the type of interpolation to use [default: 'lanczos'] */
  interpolation?: InterpolationMethod;
  /** Used by cell search to specify the interpolation function to use [default: 'z' value of the point] */
  getInterpolationValue?: 'rgba' | GetInterpolateValue<M>;
  /** Used by the cell search to specify the tile buffer size in pixels. [default: 0] */
  bufferSize?: number;
  /** Grid size, assumed pixel ratio. */
  gridSize?: number;
}

/** Options for point clustering */
export interface ClusterOptions<M extends MValue = Properties> extends BaseClusterOptions<M> {
  /** search must be `radial` */
  search: 'radial';
}

/** Options for grid clustering */
export interface ClusterRasterOptions<M extends MValue = Properties> extends BaseClusterOptions<M> {
  /** search must be `cell` */
  search: 'cell';
  /** Used by cell search to specify the type of interpolation to use. [Recommend: 'lanczos'] */
  interpolation: InterpolationMethod;
  /** Used by cell search to specify the interpolation function to use */
  getInterpolationValue: 'rgba';
}

/** Options for grid clustering */
export interface ClusterGridOptions<M extends MValue = Properties> extends BaseClusterOptions<M> {
  /** search must be `cell` */
  search: 'cell';
  /** Used by cell search to specify the type of interpolation to use. [Recommend: 'lanczos'] */
  interpolation: InterpolationMethod;
  /** Used by cell search to specify the interpolation function to use. */
  getInterpolationValue: GetInterpolateValue<M>;
  /** Used by the cell search to specify the tile buffer size in pixels. [default: 0] */
  bufferSize: number;
}

/** An export of the data as a grid */
export interface TileGrid {
  name: string;
  size: number;
  data: number[];
}

/** A cluster is a storage device to maintain groups of information in a cluster */
export interface Cluster<M extends MValue = Properties> extends Properties {
  data: M;
  visited: boolean;
  value: number | RGBA;
}

/**
 * Create a cluster with the correct value
 * @param data - the m-value of the point
 * @param value - the interpolated value as a number or RGBA of the cluster
 * @returns - a new cluster with the correct value and m-value
 */
function toCluster<M extends MValue = Properties>(data: M, value: number | RGBA): Cluster<M> {
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
  bufferSize: number;
  search: ClusterSearch;
  interpolation: InterpolationFunction<M> | RGBAInterpolationFunction;
  getValue: GetInterpolateValue<M>;
  gridSize: number; // a default is a 512x512 pixel tile
  indexes = new Map<number, PointIndex<Cluster<M>>>();

  /**
   * @param data - if provided, the data to index
   * @param options - cluster options on how to build the cluster
   * @param maxzoomStore - the store to use for the maxzoom index
   */
  constructor(
    data?: JSONCollection<Record<string, unknown>, M, M>,
    options?: BaseClusterOptions<M>,
    maxzoomStore?: VectorStore<PointShape<Cluster<M>>>,
  ) {
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? 'default';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.radius = options?.radius ?? 40;
    this.bufferSize = options?.bufferSize ?? 0;
    this.gridSize = options?.gridSize ?? 512;
    this.search = options?.search ?? 'radial';
    const isRGBA = options?.getInterpolationValue === 'rgba';
    const interpolation = options?.interpolation ?? 'lanczos';
    this.interpolation = isRGBA
      ? getRGBAInterpolation(interpolation)
      : getInterpolation<M>(interpolation);
    this.getValue =
      options?.getInterpolationValue === 'rgba' || this.search === 'radial'
        ? () => 1
        : (options?.getInterpolationValue ?? defaultGetInterpolateCurrentValue);
    // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
    for (let zoom = this.minzoom; zoom <= this.maxzoom + 1; zoom++) {
      this.indexes.set(zoom, new PointIndex<Cluster<M>>(options?.store));
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
   * Add a point to the maxzoom index. The point is a Point3D
   * @param point - the point to add
   */
  insert(point: VectorPoint<M>): void {
    const { x, y, z, m } = point;
    const maxzoomIndex = this.indexes.get(this.maxzoom);
    const value = this.getValue(point);
    maxzoomIndex?.insert({ x, y, z, m: toCluster<M>(m!, value) });
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
   * Add a vector feature. It will try to use the M-value first, but if it doesn't exist
   * it will use the feature properties data
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
   * Build the clusters when done adding points
   * @param cmp_ - custom compare function
   */
  async buildClusters(cmp_?: Comparitor<M>): Promise<void> {
    const { minzoom, maxzoom, search } = this;
    const cmp: Comparitor<M> = cmp_ ?? ((_a: M, _b: M) => true);
    let zoom = search === 'radial' ? maxzoom - 1 : maxzoom;
    for (; zoom >= minzoom; zoom--) {
      const curIndex = this.indexes.get(zoom);
      const queryIndex = this.indexes.get(zoom + 1);
      if (curIndex === undefined || queryIndex === undefined) throw new Error('Index not found');
      if (search === 'radial') await this.#clusterRadius(zoom, queryIndex, curIndex, cmp);
      else await this.#clusterCells(zoom, queryIndex, curIndex);
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
      const clusterData = point.m!;
      if (clusterData.visited) continue;
      clusterData.visited = true;
      // setup a new weighted cluster point
      const newClusterPoint = mulScalar(point, clusterData.value as number);
      let newNumPoints = clusterData.value as number;
      // joining all points found within radius
      const points = await queryIndex.searchRadius(point, radius);
      for (const { point: foundPoint } of points) {
        const foundData = foundPoint.m!;
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
   * Cell clustering
   * TODO: We build a catagorized cell of size x size with buffer
   * @param zoom - the zoom level
   * @param queryIndex - the index to query
   * @param currIndex - the index to insert into
   */
  async #clusterCells(
    zoom: number,
    queryIndex: PointIndex<Cluster<M>>,
    currIndex: PointIndex<Cluster<M>>,
  ): Promise<void> {
    const { interpolation, getValue, maxzoom } = this;
    for await (const clusterPoint of queryIndex) {
      const {
        cell,
        point: { m: clusterData },
      } = clusterPoint;
      const parentID = parent(cell, zoom);
      const [minParent, maxParent] = range(parentID);
      const parentPoint = toS2Point(parentID);
      // get all cells in parentID who haven't been visited yet. maxzoom does a radial search
      const cellPoints = (
        maxzoom === zoom
          ? await queryIndex.searchRadius(parentPoint, this.#getLevelRadius(zoom))
          : await queryIndex.searchRange(minParent, maxParent)
      ).filter(({ point }) => point.m!.visited);
      // use interpolation
      if (cellPoints.length > 0) {
        for (const { point } of cellPoints) point.m!.visited = true;
        const cellPointsData = cellPoints.map(({ point }) => {
          const { x, y, z } = point;
          return { x, y, z, m: point.m!.m };
        });
        // @ts-expect-error - we know M is correct
        const interpValue = interpolation(parentPoint, cellPointsData, getValue);
        const { x, y, z } = parentPoint;
        currIndex.insert({ x, y, z, m: toCluster(clusterData!.data, interpValue) });
      }
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
  ): Promise<undefined | Tile<Record<string, unknown>, { value: number | RGBA }, M>> {
    const data = await this.getCellData(id);
    if (data === undefined) return;
    const tile = new Tile<Record<string, unknown>, { value: number | RGBA }, M>(id);
    for (const { point } of data) {
      const [face, s, t] = toST(point);
      const { value, data } = point.m!;
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
   * Get the point data as a grid of a tile
   * @param id - the cell id
   * @returns - a tile grid
   */
  async getTileGrid(id: S2CellId): Promise<undefined | TileGrid> {
    const { layerName, gridSize } = this;
    const cellData = await this.getCellData(id);
    if (cellData === undefined) return;

    // TODO: Organize all the cell data into a grid of gridSize. Then flatten if RGBA.

    return {
      name: layerName,
      size: gridSize,
      // TODO: Build data
      data: [],
    };
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
