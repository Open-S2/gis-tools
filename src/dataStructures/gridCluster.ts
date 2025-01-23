import { fromS2Points } from '../geometry/s1/chordAngle';
import {
  KV,
  PointIndex,
  PointShape,
  defaultGetInterpolateCurrentValue,
  getInterpolation,
  getRGBAInterpolation,
} from '..';
import {
  boundsST,
  childrenIJ,
  convert,
  face as faceST,
  fromFacePosLevel,
  getVertices,
  parent,
  toFaceIJ,
  toWM,
} from '../geometry';
import { fromLonLat, fromST } from '../geometry/s2/point';

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

import type { KVStore, KVStoreConstructor, VectorStoreConstructor } from '../dataStore';

/** Options for grid clustering */
export interface BaseGridOptions<M extends MValue = Properties | RGBA> {
  /** type of point index store to use. Defaults to an in memory store */
  store?: VectorStoreConstructor<PointShape<M>>;
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
  /** Used by cell search to specify the type of interpolation to use [default: 'lanczos'] */
  interpolation?: InterpolationMethod;
  /** Used by cell search to specify the interpolation function to use [default: 'z' value of the point] */
  getInterpolationValue?: 'rgba' | GetInterpolateValue<M>;
  /** Grid size, assumed pixel ratio. */
  gridSize?: number;
  /** Used by the cell search to specify the tile buffer size in pixels. [default: 0] */
  bufferSize: number;
  /** Set a null value for grid cells that are empty */
  nullValue?: number | RGBA;
}

/** Options for grid clustering */
export interface ClusterGridOptions<M extends MValue = Properties | RGBA>
  extends BaseGridOptions<M> {
  /** Used by cell search to specify the interpolation function to use [default: 'z' value of the point] */
  getInterpolationValue: GetInterpolateValue<M>;
  /** Set a null value for grid cells that are empty */
  nullValue?: number;
}

/** Options for raster clustering */
export interface ClusterRasterOptions<M extends MValue = Properties | RGBA>
  extends BaseGridOptions<M> {
  /** Used by cell search to specify the interpolation function to use [default: 'z' value of the point] */
  getInterpolationValue: 'rgba';
  /** Set a null value for grid cells that are empty */
  nullValue?: RGBA;
}

/** An export of the data as a grid */
export interface TileGrid extends Properties {
  /** name of the layer */
  name: string;
  /** size of the grid including the buffer */
  size: number;
  /**
   * flattened array of number or RGBA.
   * The size of the array is gridSize * gridSize
   * Access the position as `gridSize * y + x`
   */
  data: number[] | RGBA[];
}

/**
 * # Grid Cluster
 *
 * ## Description
 * A cluster store to build grid data of gridSize x gridSize. The resultant tiles are filled.
 *
 * ## Usage
 * ```ts
 * import { GridCluster } from 'gis-tools-ts';
 * const pointCluster = new GridCluster();
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
export class GridCluster<M extends MValue = Properties | RGBA> {
  projection: Projection;
  layerName: string;
  minzoom: number;
  maxzoom: number;
  radius: number;
  bufferSize: number;
  interpolation: InterpolationFunction<M> | RGBAInterpolationFunction;
  getValue: GetInterpolateValue<M>;
  gridSize: number; // a default is a 512x512 pixel tile
  pointIndex: PointIndex<M>;
  gridTileStore: KVStore<number[] | RGBA[]>;
  nullValue: number | RGBA;

  /**
   * @param data - if provided, the data to index
   * @param options - cluster options on how to build the cluster
   * @param store - the store to use for storing all the grid tiles
   */
  constructor(
    data?: JSONCollection<Record<string, unknown>, M, M>,
    options?: ClusterGridOptions<M> | ClusterRasterOptions<M>,
    store: KVStoreConstructor<number[] | RGBA[]> = KV<number[] | RGBA[]>,
  ) {
    this.gridTileStore = new store();
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? 'default';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.radius = options?.radius ?? 40;
    this.bufferSize = options?.bufferSize ?? 0;
    this.gridSize = options?.gridSize ?? 512;
    const isRGBA = options?.getInterpolationValue === 'rgba';
    this.nullValue = options?.nullValue ?? (isRGBA ? { r: 0, g: 0, b: 0, a: 255 } : 0);
    const interpolation = options?.interpolation ?? 'lanczos';
    this.interpolation = isRGBA
      ? getRGBAInterpolation(interpolation)
      : getInterpolation<M>(interpolation);
    this.getValue =
      options?.getInterpolationValue === 'rgba'
        ? () => 1
        : (options?.getInterpolationValue ?? defaultGetInterpolateCurrentValue);
    // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
    this.pointIndex = new PointIndex<M>(options?.store);
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
    this.pointIndex?.insert(point);
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
   * Build the grid cluster tiles
   */
  async buildClusters(): Promise<void> {
    // build tiles at maxzoom
    let parents = await this.#clusterMaxzoom();
    // work upwards, take the 4 children and merge them
    for (let zoom = this.maxzoom - 1; zoom >= this.minzoom; zoom--) {
      parents = await this.#custerZoom(zoom, parents);
    }
  }

  /**
   * Using the point index, build grids at maxzoom by doing searches for each gridpoint.
   * @returns - the parent cells
   */
  async #clusterMaxzoom(): Promise<Set<S2CellId>> {
    const { maxzoom, pointIndex, interpolation, getValue, gridSize, bufferSize, gridTileStore } =
      this;
    const parents = new Set<S2CellId>();
    const gridLength = gridSize + bufferSize * 2;
    const radius = this.#getLevelRadius(maxzoom);

    for await (const { cell } of pointIndex) {
      const maxzoomID = parent(cell, maxzoom);
      // if maxzoomID grid tile already exists, skip
      if (await gridTileStore.has(maxzoomID)) continue;
      // prep variables and grid result
      const face = faceST(cell);
      const [sMin, tMin, sMax, tMax] = boundsST(maxzoomID, maxzoom);
      const sPixel = (sMax - sMin) / gridSize;
      const tPixel = (tMax - tMin) / gridSize;
      const sStart = sMin - sPixel * bufferSize;
      const tStart = tMin - tPixel * bufferSize;
      const grid: number[] | RGBA[] = new Array(gridLength * gridLength).fill(this.nullValue);
      // iterate through the grid and do searches for each position. Interpolate the data to the
      // position and store the result in the grid.
      for (let y = 0; y < gridLength; y++) {
        for (let x = 0; x < gridLength; x++) {
          const s = sStart + x * sPixel;
          const t = tStart + y * tPixel;
          const point = fromST(face, s, t);
          const pointShapes = await pointIndex.searchRadius(point, radius);
          const cluster = pointShapes.map(({ point }) => point);
          if (cluster.length === 0) continue;
          // @ts-expect-error - RGBA is already accounted for, typescript is being lame
          grid[y * gridLength + x] = interpolation(point, cluster, getValue);
        }
      }
      // store the grid and add the parent cell for future upscaling
      gridTileStore.set(maxzoomID, grid);
      if (maxzoom !== 0) parents.add(parent(maxzoomID, maxzoom - 1));
    }

    return parents;
  }

  /**
   * Build the parent cells. We simply search for the children of the cell and merge/upscale.
   * @param zoom - the current zoom we are upscaling to
   * @param cells - the cells to build grids for
   * @returns - the parent cells for the next round of upscaling
   */
  async #custerZoom(zoom: number, cells: Set<S2CellId>): Promise<Set<S2CellId>> {
    const { gridSize, bufferSize, gridTileStore } = this;
    const parents = new Set<S2CellId>();
    const gridLength = gridSize + bufferSize * 2;
    const halfGridLength = gridLength / 2;

    for (const cell of cells) {
      const grid: number[] | RGBA[] = new Array(gridLength * gridLength).fill(this.nullValue);
      const [face, cellZoom, i, j] = toFaceIJ(cell);
      const [blID, brID, tlID, trID] = childrenIJ(face, cellZoom, i, j);
      // for each child, upscale into the result grid
      await this.#upscaleGrid(blID, grid, 0, 0);
      await this.#upscaleGrid(brID, grid, halfGridLength, 0);
      await this.#upscaleGrid(tlID, grid, 0, halfGridLength);
      await this.#upscaleGrid(trID, grid, halfGridLength, halfGridLength);
      // store the grid and add the parent cell for future upscaling
      gridTileStore.set(cell, grid);
      if (zoom !== 0) parents.add(parent(cell, zoom - 1));
    }

    return parents;
  }

  /**
   * Upscale a grid into the target grid at x,y position
   * @param cellID - the cell id for the grid to upscale
   * @param target - the target grid
   * @param x - the x offset
   * @param y - the y offset
   */
  async #upscaleGrid(
    cellID: S2CellId,
    target: number[] | RGBA[],
    x: number,
    y: number,
  ): Promise<void> {
    const grid = await this.gridTileStore.get(cellID);
    if (grid === undefined) return;

    const { gridSize, bufferSize, interpolation, getValue } = this;
    const gridLength = gridSize + bufferSize * 2;
    const halfGridLength = gridLength / 2;
    const halfPoint = { x: 0.5, y: 0.5 };

    for (let j = 0; j < halfGridLength; j++) {
      for (let i = 0; i < halfGridLength; i++) {
        const sourcePoints = [
          { x: 0, y: 0, m: grid[j * 2 * gridLength + i * 2] },
          { x: 1, y: 0, m: grid[j * 2 * gridLength + (i * 2 + 1)] },
          { x: 0, y: 1, m: grid[(j * 2 + 1) * gridLength + i * 2] },
          { x: 1, y: 1, m: grid[(j * 2 + 1) * gridLength + (i * 2 + 1)] },
        ];
        // @ts-expect-error: RGBA and number handling is abstract
        target[(j + y) * gridLength + (i + x)] = interpolation(halfPoint, sourcePoints, getValue);
      }
    }
  }

  /**
   * Get the point data as a grid of a tile
   * @param id - the cell id
   * @returns - a tile grid
   */
  async getTile(id: S2CellId): Promise<undefined | TileGrid> {
    const { layerName, gridSize, bufferSize } = this;
    const data = await this.gridTileStore.get(id);
    if (data === undefined) return;

    return {
      name: layerName,
      size: gridSize + bufferSize * 2,
      data,
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
