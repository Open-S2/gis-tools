import { toMetadata } from 's2-tilejson';
import {
  RasterS2TileReader,
  RasterTileReader,
  imageDecoder,
  llToPX,
  lonLatToXYZ,
  pointToST,
  pxToTile,
  tileXYFromSTZoom,
} from '../../index.js';
import { readFile, readdir, stat } from 'fs/promises';

import type {
  ElevationConverter,
  ElevationPoint,
  S2TileMetadata,
  TileMetadata,
  TileReader,
} from './index.js';
import type {
  Face,
  FeatureIterator,
  MValue,
  Properties,
  RGBA,
  S2Feature,
  S2PMTilesReader,
  VectorFeature,
} from '../../index.js';
import type { Metadata, Metadatas } from 's2-tilejson';

// TODO: Get encoding from the metadata and decode the data if necessary

/**
 * # Raster Tiles File Reader
 *
 * ## Description
 * Read an entire archive of raster tiles, where the max zoom data is iterated upon
 *
 * Supports reading either RGB(A) data and/or RGB(A) encoded elevation data.
 *
 * ## Usage
 * ```ts
 * import { convertTerrariumElevationData } from 'gis-tools-ts';
 * import { RasterTilesFileReader } from 'gis-tools-ts/file';
 *
 * // creates a reader for a tile set treating the max zoom as 3 instead of the metadata's max zoom
 * const reader = new RasterTilesFileReader('./raster-tiles-top-level-folder', 3);
 * // example of reading in an elevation dataset
 * const reader2 = new RasterTilesFileReader('./terrariumData', -1, convertTerrariumElevationData);
 *
 * // grab the metadata
 * const metadata = await reader.getMetadata();
 *
 * // grab a WM tile
 * const tile1 = await reader.getTile(0, 0, 0);
 * // or if it's an S2 tile spec
 * const tile2 = await reader.getTileS2(0, 0, 0, 0);
 *
 * // get a specfic WM value given a longitude and latitude
 * const value = await reader.getLonLatValuesWM(0, 0, 0);
 * // get a specfic S2 value given a longitude and latitude
 * const value2 = await reader.getLonLatValuesS2(0, 0, 0);
 *
 * // grab all the max zoom tiles:
 * for await (const tile of reader) {
 *   console.log(tile);
 * }
 * ```
 *
 * ## Links
 * - https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html
 * - https://cesium.com/blog/2015/04/07/quadtree-cheatseet/
 */
export class RasterTilesFileReader<
  T extends MValue = RGBA | ElevationPoint,
  P extends Properties = T,
>
  implements
    FeatureIterator<S2TileMetadata | TileMetadata, T, P>,
    TileReader<S2TileMetadata | TileMetadata, T, P>
{
  metadata?: Metadata;
  /**
   * @param input - the file path or S2PMTilesReader to read from
   * @param threshold - if non-zero its the max zoom to read all tiles in the FeatureIterator
   * @param converter - the elevation converter
   */
  constructor(
    readonly input: string | S2PMTilesReader,
    readonly threshold = -1,
    readonly converter?: ElevationConverter,
  ) {}

  /**
   * Get the metadata of the archive
   * @returns - the metadata
   */
  async getMetadata(): Promise<Metadata> {
    if (this.metadata !== undefined) return this.metadata;
    if (typeof this.input === 'string') {
      const file = await readFile(`${this.input}/metadata.json`, { encoding: 'utf-8' });
      this.metadata = toMetadata(JSON.parse(file) as Metadatas);
    } else {
      this.metadata = await this.input.getMetadata();
    }
    return this.metadata;
  }

  /**
   * Grab the tile at the given zoom-x-y coordinates
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @returns - the tile
   */
  async getTileWM(zoom: number, x: number, y: number): Promise<RasterTileReader<T, P> | undefined> {
    const { extension, scheme } = await this.getMetadata();
    const isTMS = scheme === 'tms';
    const data =
      typeof this.input === 'string'
        ? await readFile(`${this.input}/${zoom}/${x}/${y}.${extension}`)
        : await this.input.getTile(zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterTileReader<T, P>(zoom, x, y, imageData, isTMS, this.converter);
  }

  /**
   * Grab the tile at the given (face, zoom, x, y) coordinates
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @returns - the tile
   */
  async getTileS2(
    face: Face,
    zoom: number,
    x: number,
    y: number,
  ): Promise<RasterS2TileReader<T, P> | undefined> {
    const { extension } = await this.getMetadata();
    const data =
      typeof this.input === 'string'
        ? await readFile(`${this.input}/${face}/${zoom}/${x}/${y}.${extension}`)
        : await this.input.getTileS2(face, zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterS2TileReader<T, P>(face, zoom, x, y, imageData, this.converter);
  }

  /**
   * Return true if the tile exists
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @returns - true if the tile exists
   */
  async hasTileWM(zoom: number, x: number, y: number): Promise<boolean> {
    const { extension } = await this.getMetadata();
    if (typeof this.input === 'string') {
      const stats = await stat(`${this.input}/${zoom}/${x}/${y}.${extension}`);
      return stats.isFile();
    } else {
      return await this.input.hasTile(zoom, x, y);
    }
  }

  /**
   * Return true if the tile exists
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @returns - true if the tile exists
   */
  async hasTileS2(face: Face, zoom: number, x: number, y: number): Promise<boolean> {
    const { extension } = await this.getMetadata();
    if (typeof this.input === 'string') {
      const stats = await stat(`${this.input}/${face}/${zoom}/${x}/${y}.${extension}`);
      return stats.isFile();
    } else {
      return await this.input.hasTileS2(face, zoom, x, y);
    }
  }

  /**
   * Get the value of the given longitude and latitude
   * @param zoom - the zoom level
   * @param lon - the longitude
   * @param lat - the latitude
   * @param tileSize - in pixels
   * @returns - the value at the given longitude and latitude
   */
  async getLonLatValuesWM(
    zoom: number,
    lon: number,
    lat: number,
    tileSize = 512,
  ): Promise<RGBA | ElevationPoint | undefined> {
    const { floor } = Math;
    const mod = (n: number, m: number) => ((n % m) + m) % m;
    // get the tile coordinates
    const { x, y } = llToPX({ x: lon, y: lat }, zoom, false, tileSize);
    const { x: tileX, y: tileY } = pxToTile({ x, y }, tileSize);
    // get the tile
    const tile = await this.getTileWM(zoom, tileX, tileY);
    if (tile === undefined) return undefined;
    // get the pixel
    const localX = mod(x, tileSize);
    const localY = mod(y, tileSize);
    const pixelX = floor(localX);
    // If TMS style, invert the y position
    const pixelY = tile.tmsStyle ? floor(tileSize - 1 - localY) : floor(localY);
    const channels = tile.image.data.length / (tileSize * tileSize);
    const position = (pixelY * tileSize + pixelX) * channels;
    const r = tile.image.data[position];
    const g = tile.image.data[position + 1];
    const b = tile.image.data[position + 2];
    const a = channels >= 4 ? tile.image.data[position + 3] : 255;
    // set to the elevation or RGBA
    if (this.converter !== undefined) {
      return { elev: this.converter(r, g, b, a) };
    } else {
      return { r, g, b, a };
    }
  }

  /**
   * Get the value of the given longitude and latitude
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param lon - the longitude
   * @param lat - the latitude
   * @param tileSize - in pixels
   * @returns - the value at the given longitude and latitude
   */
  async getLonLatValuesS2(
    zoom: number,
    lon: number,
    lat: number,
    tileSize = 512,
  ): Promise<RGBA | ElevationPoint | undefined> {
    const { floor } = Math;
    const mod = (n: number, m: number) => ((n % m) + m) % m;
    // get the tile coordinates
    const xyz = lonLatToXYZ({ x: lon, y: lat });
    const [face, s, t] = pointToST(xyz);
    const [tileX, tileY] = tileXYFromSTZoom(s, t, zoom);
    // get the tile
    const tile = await this.getTileS2(face, zoom, tileX, tileY);
    if (tile === undefined) return undefined;
    // get the pixel
    const zoomSize = tileSize * (1 << zoom);
    const pixelX = floor(mod(zoomSize * s, tileSize));
    const pixelY = floor(mod(zoomSize * t, tileSize));
    const channels = tile.image.data.length / (tileSize * tileSize);
    const position = (pixelY * tileSize + pixelX) * channels;
    const r = tile.image.data[position];
    const g = tile.image.data[position + 1];
    const b = tile.image.data[position + 2];
    const a = channels >= 4 ? tile.image.data[position + 3] : 255;

    if (this.converter !== undefined) {
      return { elev: this.converter(r, g, b, a) };
    } else {
      return { r, g, b, a };
    }
  }

  /**
   * Iterate over all tiles in the archive
   * @yields {S2Feature<S2TileMetadata, T, P> | VectorFeature<TileMetadata, T, P>} the each of the
   * tile's pixel RGBA data as lon-lat or S2 s-t coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    S2Feature<S2TileMetadata, T, P> | VectorFeature<TileMetadata, T, P>
  > {
    // iterate down from min zoom. Upon reaching maxzoom store all pixels
    const { scheme, maxzoom } = await this.getMetadata();
    const zoom = this.threshold >= 0 ? this.threshold : maxzoom;
    const isS2 = scheme === 'fzxy' || scheme === 'tfzxy';
    for (const face of (isS2 ? [0, 1, 2, 3, 4, 5] : [0]) as Face[]) {
      const xPath = isS2 ? `${this.input}/${face}/${zoom}` : `${this.input}/${zoom}`;
      for (const x of await readdir(xPath)) {
        const yPath = `${xPath}/${x}`;
        const xNumber = Number(x);
        for (const y of await readdir(yPath)) {
          const yNumber = Number(y.split('.')[0]);
          const tile = isS2
            ? await this.getTileS2(face, zoom, xNumber, yNumber)
            : await this.getTileWM(zoom, xNumber, yNumber);
          if (tile === undefined) continue;
          yield* tile;
        }
      }
    }
  }
}
