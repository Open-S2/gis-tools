import { imageDecoder } from '../image';
import { toMetadata } from 's2-tilejson';
import { RasterS2TileReader, RasterTileReader } from '.';
import { readFile, readdir, stat } from 'fs/promises';

import type { ElevationConverter, ElevationPoint, S2TileMetadata, TileMetadata } from '.';
import type {
  Face,
  FeatureIterator,
  Properties,
  RGBA,
  S2Feature,
  S2PMTilesReader,
  VectorFeature,
} from '../..';
import type { Metadata, Metadatas } from 's2-tilejson';

// TODO: Support timestamps

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
 * import { convertTerrariumElevationData } from 's2-tools';
 * import { RasterTilesFileReader } from 's2-tools/file';
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
 * // grab all the max zoom tiles:
 * for await (const tile of reader) {
 *   console.log(tile);
 * }
 * ```
 */
export class RasterTilesFileReader<T extends Properties = RGBA | ElevationPoint>
  implements FeatureIterator<S2TileMetadata | TileMetadata, T, Properties>
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
  async getTile(zoom: number, x: number, y: number): Promise<RasterTileReader<T> | undefined> {
    const { extension, scheme } = await this.getMetadata();
    const isTMS = scheme === 'tms';
    const data =
      typeof this.input === 'string'
        ? await readFile(`${this.input}/${zoom}/${x}/${y}.${extension}`)
        : await this.input.getTile(zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterTileReader(zoom, x, y, imageData, isTMS, this.converter);
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
  ): Promise<RasterS2TileReader<T> | undefined> {
    const { extension } = await this.getMetadata();
    const data =
      typeof this.input === 'string'
        ? await readFile(`${this.input}/${face}/${zoom}/${x}/${y}.${extension}`)
        : await this.input.getTileS2(face, zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterS2TileReader(face, zoom, x, y, imageData, this.converter);
  }

  /**
   * Return true if the tile exists
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @returns - true if the tile exists
   */
  async hasTile(zoom: number, x: number, y: number): Promise<boolean> {
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
   * Iterate over all tiles in the archive
   * @yields - the each of the tile's pixel RGBA data as lon-lat or S2 s-t coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    S2Feature<S2TileMetadata, T, Properties> | VectorFeature<TileMetadata, T, Properties>
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
            : await this.getTile(zoom, xNumber, yNumber);
          if (tile === undefined) continue;
          yield* tile;
        }
      }
    }
  }
}
