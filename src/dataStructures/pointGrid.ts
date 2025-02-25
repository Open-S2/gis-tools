import { fromST as fromSTPoint } from '../geometry/s2/point';
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
  face as cellFace,
  childrenIJ,
  convert,
  fromST,
  parent,
  toFaceIJ,
} from '../geometry';

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
import type {
  FeatureIterator,
  GetInterpolateValue,
  InterpolationFunction,
  InterpolationMethod,
  RGBA,
  RGBAInterpolationFunction,
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
  /**
   * Used by cell search to specify the type of interpolation to use.
   * The recommendation is IDW as you want to prioritize closest data points. [default: 'idw']
   */
  maxzoomInterpolation?: InterpolationMethod;
  /**
   * Used by cell search to specify the type of interpolation to use.
   * From experimentation, lanczos is a fast algorithm that maintains the quality of the data
   * [default: 'lanczos']
   */
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
export interface GridValueOptions<M extends MValue = Properties> extends BaseGridOptions<M> {
  /** Used by cell search to specify the interpolation function to use [default: 'z' value of the point] */
  getInterpolationValue: GetInterpolateValue<M>;
  /** Set a null value for grid cells that are empty */
  nullValue?: number;
}

/** Options for raster clustering */
export interface GridRasterOptions<M extends MValue = RGBA> extends BaseGridOptions<M> {
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
 * Useful for building raster tiles or other grid like data (temperature, precipitation, wind, etc).
 *
 * ## Usage
 * ```ts
 * import { PointGrid } from 'gis-tools-ts';
 * const PointGrid = new PointGrid();
 *
 * // add a lon-lat
 * PointGrid.insertLonLat(lon, lat, data);
 * // add an STPoint
 * PointGrid.insertFaceST(face, s, t, data);
 *
 * // after adding data build the clusters
 * await PointGrid.buildClusters();
 *
 * // get the clusters for a tile
 * const tile = await PointGrid.getTile(id);
 * ```
 */
export class PointGrid<M extends MValue = Properties | RGBA> {
  projection: Projection;
  layerName: string;
  minzoom: number;
  maxzoom: number;
  bufferSize: number;
  maxzoomInterpolation: InterpolationFunction<M> | RGBAInterpolationFunction;
  interpolation: InterpolationFunction<M> | RGBAInterpolationFunction;
  getValue: GetInterpolateValue<M>;
  gridSize: number; // a default is a 512x512 pixel tile
  pointIndex: PointIndex<M>;
  gridTileStore: KVStore<number[] | RGBA[]>;
  nullValue: number | RGBA;
  isRGBA: boolean;

  /**
   * @param options - cluster options on how to build the cluster
   * @param store - the store to use for storing all the grid tiles
   */
  constructor(
    options?: BaseGridOptions<M> | GridRasterOptions<M>,
    store: KVStoreConstructor<number[] | RGBA[]> = KV<number[] | RGBA[]>,
  ) {
    this.gridTileStore = new store();
    this.projection = options?.projection ?? 'S2';
    this.layerName = options?.layerName ?? 'default';
    this.minzoom = Math.max(options?.minzoom ?? 0, 0);
    this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
    this.bufferSize = options?.bufferSize ?? 0;
    this.gridSize = options?.gridSize ?? 512;
    const isRGBA = (this.isRGBA = options?.getInterpolationValue === 'rgba');
    this.nullValue = options?.nullValue ?? (isRGBA ? { r: 0, g: 0, b: 0, a: 255 } : 0);
    const interpolation = options?.interpolation ?? 'lanczos';
    const maxzoomInterpolation = options?.maxzoomInterpolation ?? 'idw';
    this.interpolation = isRGBA
      ? getRGBAInterpolation(interpolation)
      : getInterpolation<M>(interpolation);
    this.maxzoomInterpolation = isRGBA
      ? getRGBAInterpolation(maxzoomInterpolation)
      : getInterpolation<M>(maxzoomInterpolation);
    this.getValue =
      options?.getInterpolationValue === 'rgba'
        ? () => 1
        : (options?.getInterpolationValue ?? defaultGetInterpolateCurrentValue);
    // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
    this.pointIndex = new PointIndex<M>(options?.store, this.projection);
  }

  /**
   * Add a point to the maxzoom index. The point is a Point3D
   * @param point - the point to add
   */
  insert(point: VectorPointM<M>): void {
    this.pointIndex?.insert(point);
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
    this.insert(fromSTPoint(face, s, t, data) as VectorPointM<M>);
  }

  /** Build the grid cluster tiles */
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
    const { projection, nullValue, maxzoom, pointIndex, isRGBA } = this;
    const { maxzoomInterpolation, getValue, gridSize, bufferSize, gridTileStore } = this;
    const { min, floor, log2 } = Math;
    const parents = new Set<S2CellId>();
    const gridLength = gridSize + bufferSize * 2;
    // if the grid is 512 x 512, log2 is 9, meaning the quadtree must split 9 times to analyze
    // each individual pixel. Make sure we don't dive past 30 levels as that's the limit of the spec.
    const zoomGridLevel = min(maxzoom + floor(log2(gridSize)) - 1, 30);

    for await (const { cell } of pointIndex) {
      const maxzoomID = parent(cell, maxzoom);
      // if maxzoomID grid tile already exists, skip
      if (await gridTileStore.has(maxzoomID)) continue;
      // prep variables and grid result
      const face = cellFace(cell);
      const [sMin, tMin, sMax, tMax] = boundsST(maxzoomID, maxzoom);
      const sPixel = (sMax - sMin) / gridSize;
      const tPixel = (tMax - tMin) / gridSize;
      const sStart = sMin - sPixel * bufferSize;
      const tStart = tMin - tPixel * bufferSize;
      const grid: number[] | RGBA[] = new Array(gridLength * gridLength).fill(nullValue);
      // iterate through the grid and do searches for each position. Interpolate the data to the
      // position and store the result in the grid.
      for (let y = 0; y < gridLength; y++) {
        for (let x = 0; x < gridLength; x++) {
          const t = tStart + y * tPixel;
          let s = sStart + x * sPixel;
          if (projection === 'WG') s = (s + 1) % 1; // ensure within 0-1 range via wrapping to the other side
          // search for points within a reasonable cell size
          let gridLevelSearch = zoomGridLevel;
          let pointShapes: PointShape<M>[];
          let stCell = fromST(face, s, t, zoomGridLevel);
          do {
            pointShapes = await pointIndex.searchRange(stCell);
            stCell = parent(stCell, --gridLevelSearch);
          } while (
            pointShapes.length === 0 &&
            gridLevelSearch > 0 &&
            gridLevelSearch > zoomGridLevel - 3
          );
          if (pointShapes.length === 0) continue;
          const cluster = pointShapes.map(({ point }) => point);
          grid[y * gridLength + x] = maxzoomInterpolation(
            fromSTPoint(face, s, t),
            // @ts-expect-error - RGBA is already accounted for, typescript is being lame
            cluster,
            isRGBA ? undefined : getValue,
          );
        }
      }
      // store the grid and add the parent cell for future upscaling
      gridTileStore.set(maxzoomID, grid);
      if (maxzoom !== 0) parents.add(parent(maxzoomID, maxzoom - 1));
    }

    return parents;
  }

  /**
   * Build the parent cells. We simply search for the children of the cell and merge/downsample.
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
      // for each child, downsample into the result grid
      await this.#downsampleGrid(blID, grid, 0, 0);
      await this.#downsampleGrid(brID, grid, halfGridLength, 0);
      await this.#downsampleGrid(tlID, grid, 0, halfGridLength);
      await this.#downsampleGrid(trID, grid, halfGridLength, halfGridLength);
      // store the grid and add the parent cell for future upscaling
      gridTileStore.set(cell, grid);
      if (zoom !== 0) parents.add(parent(cell, zoom - 1));
    }

    return parents;
  }

  /**
   * Upscale a grid into the target grid at x,y position
   * @param cellID - the cell id for the grid to downsample
   * @param target - the target grid
   * @param x - the x offset
   * @param y - the y offset
   */
  async #downsampleGrid(
    cellID: S2CellId,
    target: number[] | RGBA[],
    x: number,
    y: number,
  ): Promise<void> {
    const grid = await this.gridTileStore.get(cellID);
    if (grid === undefined) return;

    const { gridSize, bufferSize, interpolation, getValue, isRGBA } = this;
    const gridLength = gridSize + bufferSize * 2;
    const halfGridLength = gridLength / 2;
    const halfPoint = { x: 0.5, y: 0.5 };

    for (let j = 0; j < halfGridLength; j++) {
      for (let i = 0; i < halfGridLength; i++) {
        // Filter "dead/null" pixels from sourcePoints
        const sourcePoints = [
          { x: 0, y: 0, m: grid[j * 2 * gridLength + i * 2] },
          { x: 1, y: 0, m: grid[j * 2 * gridLength + (i * 2 + 1)] },
          { x: 0, y: 1, m: grid[(j * 2 + 1) * gridLength + i * 2] },
          { x: 1, y: 1, m: grid[(j * 2 + 1) * gridLength + (i * 2 + 1)] },
        ].filter((p) => !this.#isNullValue(p.m));
        if (sourcePoints.length === 0) continue;
        target[(j + y) * gridLength + (i + x)] = interpolation(
          halfPoint,
          // @ts-expect-error: RGBA and number handling is abstract
          sourcePoints,
          isRGBA ? undefined : getValue,
        );
      }
    }
  }

  /**
   * Check if a value is null
   * @param value - the value to check
   * @returns - true if the value is equal to the null
   */
  #isNullValue(value: number | RGBA): boolean {
    const { nullValue } = this;
    if (typeof value === 'number' && typeof nullValue === 'number') return value === nullValue;
    else if (typeof value === 'object' && typeof nullValue === 'object')
      return (
        value.r === nullValue.r &&
        value.g === nullValue.g &&
        value.b === nullValue.b &&
        value.a === nullValue.a
      );
    return false;
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
}
