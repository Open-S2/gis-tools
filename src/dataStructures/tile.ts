import { convert } from '../geometry/tools/convert';
import { simplify } from '../geometry';
import { splitTile } from '../geometry/tools/clip';
import {
  contains,
  fromFace,
  face as getFace,
  isFace,
  level,
  parent as parentID,
  toFaceIJ,
} from '../geometry/id';

import type { FeatureIterator } from '..';
import type {
  Face,
  JSONCollection,
  MValue,
  Projection,
  Properties,
  VectorFeatures,
  VectorGeometry,
  VectorPoint,
} from '../geometry';

/**
 * # Tile Class
 *
 * ## Description
 * Tile Class to contain the tile information for splitting or simplifying
 *
 * ## Fields
 *
 * - `extent` - the extent of the tile
 * - `face` - the tile's face
 * - `zoom` - the tile's zoom
 * - `i` - the tile's x position
 * - `j` - the tile's y position
 * - `layers` - the tile's layers
 * - `transformed` - whether the tile feature geometry has been transformed to tile coordinates
 *
 * ## Usage
 *
 * ```ts
 * import { Tile } from 'gis-tools-ts';
 *  // create a tile
 * const tile = new Tile(id);
 * // add a feature
 * tile.addFeature(feature);
 *  // transform the geometry to be relative to the tile
 * tile.transform();
 * ```
 *
 * If you have some kind reader you can use the `addReader` method
 * ```ts
 * import { Tile, JSONReader } from 'gis-tools-ts';
 * // create a tile
 * const tile = new Tile(id);
 * // add a reader
 * const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
 * const jsonReader = new JSONReader(fileReader);
 * await tile.addReader(jsonReader);
 * // then transform
 * tile.transform();
 * ```
 */
export class Tile<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> {
  extent = 1;
  face: Face;
  zoom: number;
  i: number;
  j: number;
  /**
   * @param id - the tile id
   * @param layers - the tile's layers
   * @param transformed - whether the tile feature geometry has been transformed to tile coordinates
   */
  constructor(
    id: bigint,
    public layers: Record<string, Layer<M, D, P>> = {},
    public transformed = false,
  ) {
    const [face, zoom, i, j] = toFaceIJ(id);
    this.face = face;
    this.zoom = zoom;
    this.i = i;
    this.j = j;
  }

  /** @returns true if the tile is empty of features */
  isEmpty(): boolean {
    for (const layer of Object.values(this.layers)) {
      if (layer.features.length > 0) return false;
    }
    return true;
  }

  /**
   * Add features from a reader to the tile, optionally to a specific layer to store it in. Defaults to "default".
   * @param reader - the reader containing the input data
   * @param layer - layer to store the feature to
   */
  async addReader(reader: FeatureIterator<M, D, P>, layer?: string): Promise<void> {
    for await (const feature of reader) {
      const vectorFeatures = convert(feature.type === 'S2Feature' ? 'S2' : 'WM', feature);
      for (const vf of vectorFeatures) this.addFeature(vf, layer);
    }
  }

  /**
   * Add a vector feature to the tile, optionally to a specific layer to store it in. Defaults to "default".
   * @param feature - Vector Feature
   * @param layer - layer to store the feature to
   */
  addFeature(feature: VectorFeatures<M, D, P>, layer?: string): void {
    const { metadata = {} } = feature;
    // @ts-expect-error - ignore if meta doesn't have layer. its ok
    const layerName = (metadata.layer as string) ?? layer ?? 'default';
    if (this.layers[layerName] === undefined) {
      this.layers[layerName] = new Layer(layerName, []);
    }
    this.layers[layerName].features.push(feature);
  }

  /**
   * Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
   * NOTE: This should be called after the tile has been split into children if that functionality
   * is needed.
   * @param tolerance - tolerance
   * @param maxzoom - max zoom at which to simplify
   */
  transform(tolerance: number, maxzoom?: number): void {
    const { transformed, zoom, i, j, layers } = this;
    if (transformed) return;

    for (const layer of Object.values(layers)) {
      for (const feature of layer.features) {
        if (tolerance > 0) simplify(feature.geometry, tolerance, zoom, maxzoom);
        _transform(feature.geometry, zoom, i, j);
      }
    }

    this.transformed = true;
  }
}

/**
 * @param geometry - input vector geometry to be mutated in place
 * @param zoom - tile zoom
 * @param ti - tile i
 * @param tj - tile j
 */
function _transform<M extends MValue = Properties>(
  geometry: VectorGeometry<M>,
  zoom: number,
  ti: number,
  tj: number,
): void {
  const { type, coordinates } = geometry;
  zoom = 1 << zoom;

  if (type === 'Point') transformPoint(coordinates, zoom, ti, tj);
  else if (type === 'MultiPoint' || type === 'LineString')
    coordinates.forEach((p) => transformPoint(p, zoom, ti, tj));
  else if (type === 'MultiLineString' || type === 'Polygon')
    coordinates.forEach((l) => l.forEach((p) => transformPoint(p, zoom, ti, tj)));
  else if (type === 'MultiPolygon')
    coordinates.forEach((p) => p.forEach((l) => l.forEach((p) => transformPoint(p, zoom, ti, tj))));
}

/**
 * Mutates the point in place to a tile coordinate
 * @param vp - input vector point that we are mutating in place
 * @param zoom - current zoom
 * @param ti - x translation
 * @param tj - y translation
 */
export function transformPoint<M extends MValue = Properties>(
  vp: VectorPoint<M>,
  zoom: number,
  ti: number,
  tj: number,
): void {
  vp.x = vp.x * zoom - ti;
  vp.y = vp.y * zoom - tj;
}

/** Layer Class to contain the layer information for splitting or simplifying */
export class Layer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> {
  extent = 1;
  /**
   * @param name - the layer name
   * @param features - the layer's features
   */
  constructor(
    public name: string,
    public features: VectorFeatures<M, D, P>[] = [],
  ) {}
}

/** Options for creating a TileStore */
export interface TileStoreOptions {
  /** manually set the projection, otherwise it defaults to whatever the data type is */
  projection?: Projection;
  /** min zoom to generate data on */
  minzoom?: number;
  /** max zoom level to cluster the points on */
  maxzoom?: number;
  /** max zoom to index data on construction */
  indexMaxzoom?: number;
  /** simplification tolerance (higher means simpler) */
  tolerance?: number;
  /** tile buffer on each side so lines and polygons don't get clipped */
  buffer?: number;
  /** whether to build the bounding box for each tile feature */
  buildBBox?: boolean;
}

/**
 * # Tile Store
 *
 * ## Description
 * TileStore Class is a tile-lookup system that splits and simplifies as needed for each tile request
 *
 * ## Usage
 * ```ts
 * const tileStore = new TileStore(data, {
 *  projection: 'WM',
 *  minzoom: 0,
 *  maxzoom: 9,
 *  indexMaxzoom: 4,
 *  tolerance: 3,
 *  buffer: 0.0625
 *  buildBBox: false
 * });
 *
 * // get a tile
 * const tile = tileStore.getTile(id);
 * ```
 */
export class TileStore<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> {
  minzoom = 0; // min zoom to preserve detail on
  maxzoom = 18; // max zoom to preserve detail on
  faces = new Set<Face>(); // store which faces are active. 0 face could be entire WM projection
  indexMaxzoom = 4; // max zoom in the tile index
  tolerance = 3; // simplification tolerance (higher means simpler)
  buffer = 0.0625; // tile buffer for lines and polygons
  tiles: Map<bigint, Tile<M, D, P>> = new Map(); // stores both WM and S2 tiles
  projection: Projection; // projection to build tiles for
  buildBBox = false; // whether to build the bounding box for each tile feature
  /**
   * @param data - input data may be WM or S2 as a Feature or a Collection of Features
   * @param options - options to define how to store the data
   */
  constructor(data: JSONCollection<M, D, P>, options?: TileStoreOptions) {
    // set options should they exist
    this.minzoom = options?.minzoom ?? 0;
    this.maxzoom = options?.maxzoom ?? 20;
    this.indexMaxzoom = options?.indexMaxzoom ?? 4;
    this.tolerance = options?.tolerance ?? 3;
    this.buffer = options?.buffer ?? 0.0625;
    this.buildBBox = options?.buildBBox ?? false;
    // update projection
    if (options?.projection !== undefined) this.projection = options.projection;
    else if (data.type === 'Feature' || data.type === 'FeatureCollection') this.projection = 'WM';
    else this.projection = 'S2';
    // sanity check
    if (this.maxzoom < 0 || this.maxzoom > 20)
      throw new Error('maxzoom should be in the 0-20 range');
    // convert features
    const features = convert(
      this.projection,
      data,
      this.buildBBox,
      this.tolerance,
      this.maxzoom,
      true,
    );
    for (const feature of features) this.#addFeature(feature);
    for (let face = 0; face < 6; face++) {
      const id = fromFace(face as Face);
      this.#splitTile(id);
    }
  }

  /**
   * @param id - the tile id to acquire
   * @returns - the tile if it exists
   */
  getTile(id: bigint): undefined | Tile<M, D, P> {
    const { tiles, faces } = this;
    const zoom = level(id);
    const face = getFace(id);
    // If the zoom is out of bounds, return nothing
    if (zoom < 0 || zoom > 20 || !faces.has(face) || zoom < this.minzoom || zoom > this.maxzoom)
      return;

    // we want to find the closest tile to the data.
    let pID = id;
    while (!tiles.has(pID) && !isFace(pID)) pID = parentID(pID);
    // split as necessary, the algorithm will know if the tile is already split
    this.#splitTile(pID, id, zoom);

    return tiles.get(id);
  }

  /**
   * Stores a feature to a tile, creating the tile if it doesn't exist and tracking the faces we use
   * @param feature - the feature to store to a face tile. Creates the tile if it doesn't exist
   */
  #addFeature(feature: VectorFeatures<M, D, P>): void {
    const { faces, tiles } = this;
    const face = feature.face ?? 0;
    const id = fromFace(face);
    let tile = tiles.get(id);
    if (tile === undefined) {
      faces.add(face);
      tile = new Tile(id);
      tiles.set(id, tile);
    }
    tile?.addFeature(feature);
  }

  /**
   * Splits a tile into smaller tiles given a start and end range, stopping at maxzoom
   * @param startID - where to start tiling
   * @param endID - where to stop tiling
   * @param endZoom - stop tiling at this zoom
   */
  #splitTile(startID: bigint, endID?: bigint, endZoom: number = this.maxzoom): void {
    const { buffer, tiles, tolerance, maxzoom, indexMaxzoom } = this;
    const stack: bigint[] = [startID];
    // avoid recursion by using a processing queue
    while (stack.length > 0) {
      // find our next tile to split
      const stackID = stack.pop();
      if (stackID === undefined) break;
      const tile = tiles.get(stackID);
      // if the tile we need does not exist, is empty, or already transformed, skip it
      if (tile === undefined || tile.isEmpty() || tile.transformed) continue;
      const tileZoom = tile.zoom;
      // 1: stop tiling if we reached a defined end
      // 2: stop tiling if it's the first-pass tiling, and we reached max zoom for indexing
      // 3: stop at currently needed maxzoom OR current tile does not include child
      if (
        tileZoom >= maxzoom || // 1
        (endID === undefined && tileZoom >= indexMaxzoom) || // 2
        (endID !== undefined && (tileZoom > endZoom || !contains(stackID, endID))) // 3
      )
        continue;

      // split the tile and store the children
      const [
        { id: blID, tile: blTile },
        { id: brID, tile: brTile },
        { id: tlID, tile: tlTile },
        { id: trID, tile: trTile },
      ] = splitTile(tile, buffer);
      tiles.set(blID, blTile);
      tiles.set(brID, brTile);
      tiles.set(tlID, tlTile);
      tiles.set(trID, trTile);
      // now that the tile has been split, we can transform it
      tile.transform(tolerance, maxzoom);
      // push the new features to the stack
      stack.push(blID, brID, tlID, trID);
    }
  }
}
