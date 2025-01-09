import { bboxST } from '../../geometry/s2/coords';
import { imageDecoder } from '../image';
import { toMetadata } from 's2-tilejson';
import { xyzToBBOX } from '../../geometry/wm/coords';

import type {
  Face,
  FeatureIterator,
  Properties,
  RGBA,
  S2Feature,
  S2PMTilesReader,
  VectorFeature,
  VectorPoint,
} from '../..';
import type { Metadata, Metadatas } from 's2-tilejson';

// TODO: Support timestamps

/** Elevation point used by elevation readers */
export interface ElevationPoint extends Properties {
  elev: number;
}

/** Tile's metadata */
export interface TileMetadata {
  zoom: number;
  x: number;
  y: number;
}

/** S2 Tile's metadata */
export interface S2TileMetadata {
  face: Face;
  zoom: number;
  x: number;
  y: number;
}

/** Elevation converter */
export type ElevationConverter = (r: number, g: number, b: number, a?: number) => number;

/**
 * @param r - red
 * @param g - green
 * @param b - blue
 * @returns - elevation
 */
export function convertTerrariumElevationData(r: number, g: number, b: number): number {
  return r * 256.0 + g + b / 256.0 - 32768.0;
}

/**
 * @param r - red
 * @param g - green
 * @param b - blue
 * @returns - elevation
 */
export function convertMapboxElevationData(r: number, g: number, b: number): number {
  return -10000 + (r * 256 * 256 + g * 256 + b) * 0.1;
}

/**
 * # Raster Tiles Reader
 *
 * ## Description
 * Read an entire archive of raster tiles, where the max zoom data is iterated upon
 *
 * Supports reading either RGB(A) data and/or RGB(A) encoded elevation data.
 *
 * NOTE: Consider using the RasterTilesFileReader from `gis-tools/file` instead for local access.
 *
 * ## Usage
 * ```ts
 * import { RasterTilesReader, convertTerrariumElevationData } from 'gis-tools';
 *
 * // creates a reader for a tile set treating the max zoom as 3 instead of the metadata's max zoom
 * const reader = new RasterTilesReader('https://example.com/satellite-data', 3);
 * // example of reading in an elevation dataset
 * const reader2 = new RasterTilesReader('https://example.com/terrariumData', -1, convertTerrariumElevationData);
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
export class RasterTilesReader<T extends Properties = RGBA | ElevationPoint>
  implements FeatureIterator<S2TileMetadata | TileMetadata, T, Properties>
{
  metadata?: Metadata;
  /**
   * @param input - the URL path or S2PMTilesReader to read from
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
      const meta = await fetch(`${this.input}/metadata.json`).then(
        async (res) => (await res.json()) as Metadatas,
      );
      this.metadata = toMetadata(meta);
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
        ? await fetch(`${this.input}/${zoom}/${x}/${y}.${extension}`).then(
            async (res) => await res.arrayBuffer(),
          )
        : await this.input.getTile(zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterTileReader<T>(zoom, x, y, imageData, isTMS, this.converter);
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
        ? await fetch(`${this.input}/${face}/${zoom}/${x}/${y}.${extension}`).then(
            async (res) => await res.arrayBuffer(),
          )
        : await this.input.getTileS2(face, zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterS2TileReader<T>(face, zoom, x, y, imageData, this.converter);
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
      const response = await fetch(`${this.input}/${zoom}/${x}/${y}.${extension}`, {
        method: 'HEAD',
      });
      return response.ok;
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
      const response = await fetch(`${this.input}/${face}/${zoom}/${x}/${y}.${extension}`, {
        method: 'HEAD',
      });
      return response.ok;
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
    const { scheme, minzoom, maxzoom } = await this.getMetadata();
    const threshold = this.threshold >= 0 ? this.threshold : maxzoom;
    const isS2 = scheme === 'fzxy' || scheme === 'tfzxy';
    for (const face of (isS2 ? [0, 1, 2, 3, 4, 5] : [0]) as Face[]) {
      const stack: [zoom: number, x: number, y: number][] = [[0, 0, 0]];
      while (stack.length > 0) {
        const [zoom, x, y] = stack.pop()!;
        // if zoom not reached yet, push children and continue
        const hasTile = isS2
          ? await this.hasTileS2(face, zoom, x, y)
          : await this.hasTile(zoom, x, y);
        if (zoom < minzoom || (zoom !== threshold && hasTile)) {
          stack.push(
            [zoom + 1, x * 2, y * 2],
            [zoom + 1, x * 2 + 1, y * 2],
            [zoom + 1, x * 2, y * 2 + 1],
            [zoom + 1, x * 2 + 1, y * 2 + 1],
          );
          continue;
        } else if (zoom === threshold) {
          const tile = isS2
            ? await this.getTileS2(face, zoom, x, y)
            : await this.getTile(zoom, x, y);
          if (tile === undefined) continue;
          yield* tile;
        }
      }
    }
  }
}

/**
 * Raster Tile Reader
 */
export class RasterTileReader<T extends Properties = RGBA | ElevationPoint>
  implements FeatureIterator<TileMetadata, T, Properties>
{
  /**
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @param image - the raw RGB(A) image data
   * @param tmsStyle - if true, the y is inverted
   * @param converter - the elevation converter (if provided its not an RGBA image but rather elevation data)
   */
  constructor(
    readonly zoom: number,
    readonly x: number,
    readonly y: number,
    readonly image: ImageData,
    readonly tmsStyle = false,
    readonly converter?: ElevationConverter,
  ) {}

  /**
   * Iterate over all tiles in the archive
   * @yields - the each of the tile's pixel RGBA data as lon-lat coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<TileMetadata, T, Properties>> {
    const { zoom, x, y, image, tmsStyle } = this;
    const { width: tileSize, data } = image;
    const channels = data.length / (tileSize * tileSize);
    // Get the bounding box of the tile in lon-lat
    const [west, south, east, north] = xyzToBBOX(x, y, zoom, tmsStyle, 'WGS84', tileSize);
    const lonStep = (east - west) / tileSize;
    const latStep = (north - south) / tileSize;
    const coordinates: VectorPoint<T>[] = [];

    for (let py = 1; py <= tileSize; py++) {
      const lat = north - (py - 0.5) * latStep; // Center of the row
      for (let px = 1; px <= tileSize; px++) {
        const lon = west + (px - 0.5) * lonStep; // Center of the column
        const index = ((py - 1) * tileSize + (px - 1)) * channels;
        const m: RGBA | ElevationPoint =
          this.converter !== undefined
            ? { elev: this.converter(data[index], data[index + 1], data[index + 2]) }
            : {
                r: data[index],
                g: data[index + 1],
                b: data[index + 2],
                a: channels === 4 ? data[index + 3] : 255,
              };
        coordinates.push({ x: lon, y: lat, m: m as unknown as T });
      }
    }

    yield {
      type: 'VectorFeature',
      geometry: {
        type: 'MultiPoint',
        coordinates,
        is3D: false,
      },
      properties: {},
      metadata: { zoom, x, y },
    };
  }
}

/**
 * S2 Raster Tile Reader
 */
export class RasterS2TileReader<T extends Properties = RGBA | ElevationPoint>
  implements FeatureIterator<S2TileMetadata, T, Properties>
{
  /**
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @param image - the raw image RGB(A) data
   * @param converter - the elevation converter (if provided its not an RGBA image but rather elevation data)
   */
  constructor(
    readonly face: Face,
    readonly zoom: number,
    readonly x: number,
    readonly y: number,
    readonly image: ImageData,
    readonly converter?: ElevationConverter,
  ) {}

  /**
   * Iterate over all tiles in the archive
   * @yields - the each of the tile's pixel RGBA data as S2 s-t coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<S2Feature<S2TileMetadata, T, Properties>> {
    const { face, zoom, x, y, image } = this;
    const { width: tileSize, data } = image;
    const channels = data.length / (tileSize * tileSize);
    // Get the bounding box of the tile in s-t space
    const [west, south, east, north] = bboxST(x, y, zoom);
    const lonStep = (east - west) / tileSize;
    const latStep = (north - south) / tileSize;
    const coordinates: VectorPoint<T>[] = [];

    for (let py = 1; py <= tileSize; py++) {
      const lat = north - (py - 0.5) * latStep; // Center of the row
      for (let px = 1; px <= tileSize; px++) {
        const lon = west + (px - 0.5) * lonStep; // Center of the column
        const index = ((py - 1) * tileSize + (px - 1)) * channels;
        const m: RGBA | ElevationPoint =
          this.converter !== undefined
            ? { elev: this.converter(data[index], data[index + 1], data[index + 2]) }
            : {
                r: data[index],
                g: data[index + 1],
                b: data[index + 2],
                a: channels === 4 ? data[index + 3] : 255,
              };
        coordinates.push({ x: lon, y: lat, m: m as unknown as T });
      }
    }

    yield {
      type: 'S2Feature',
      face,
      geometry: {
        type: 'MultiPoint',
        coordinates,
        is3D: false,
      },
      properties: {},
      metadata: { face, zoom, x, y },
    };
  }
}
