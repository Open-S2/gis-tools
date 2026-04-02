import { toMetadata } from 's2-tilejson';
import {
  bboxST,
  imageDecoder,
  llToPX,
  lonLatToXYZ,
  mercToLL,
  pointToST,
  pxToTile,
  tileXYFromSTZoom,
  xyzToBBOX,
} from '../../index.js';
import { buildTileGridWM, mergeTileGridWM } from './grid.js';

import type {
  ElevationConverter,
  Face,
  FeatureIterator,
  MValue,
  Properties,
  RGBA,
  S2Feature,
  S2PMTilesReader,
  S2TileID,
  S2TilesReader,
  TileID,
  VectorFeature,
  VectorPoint,
  WMTileID,
} from '../../index.js';
import type { Metadata, Metadatas } from 's2-tilejson';

/** Elevation point used by elevation readers */
export interface ElevationPoint extends Properties {
  elev: number;
}

/** Tile Reader Interface */
export interface TileReader<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> extends FeatureIterator<M, D, P> {
  getMetadata: () => Promise<Metadata>;
  hasTileWM: (zoom: number, x: number, y: number) => Promise<boolean>;
  hasTileS2: (face: Face, zoom: number, x: number, y: number) => Promise<boolean>;
  getTileWM: (zoom: number, x: number, y: number) => Promise<RasterTileReader<D, P> | undefined>;
  getTileS2: (
    face: Face,
    zoom: number,
    x: number,
    y: number,
  ) => Promise<RasterS2TileReader<D, P> | undefined>;
  getLonLatValuesWM: (
    zoom: number,
    lon: number,
    lat: number,
    tileSize?: number,
  ) => Promise<RGBA | ElevationPoint | undefined>;
  getLonLatValuesS2: (
    zoom: number,
    lon: number,
    lat: number,
    tileSize?: number,
  ) => Promise<RGBA | ElevationPoint | undefined>;
  iterate(): AsyncGenerator<TileID>;
}

/**
 * # Raster Tiles Reader
 *
 * ## Description
 * Read an entire archive of raster tiles, where the max zoom data is iterated upon
 *
 * Supports reading either RGB(A) data and/or RGB(A) encoded elevation data.
 *
 * NOTE: Consider using the `RasterTilesFileReader` from `gis-tools-ts/file` instead for local access.
 *
 * ## Usage
 * ```ts
 * import { RasterTilesReader, convertTerrariumElevationData } from 'gis-tools-ts';
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
export class RasterTilesReader<T extends MValue = RGBA | ElevationPoint>
  implements FeatureIterator<TileID, T, Properties>, TileReader<TileID, T, Properties>
{
  metadata?: Metadata;
  /**
   * @param input - the URL path, S2PMTilesReader, or S2TilesReader to read from
   * @param threshold - if non-zero its the max zoom to read all tiles in the FeatureIterator
   * @param converter - the elevation converter
   */
  constructor(
    readonly input: string | S2PMTilesReader | S2TilesReader,
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
  async getTileWM(zoom: number, x: number, y: number): Promise<RasterTileReader<T> | undefined> {
    const { extension, scheme } = await this.getMetadata();
    const isTMS = scheme === 'tms';
    const data =
      typeof this.input === 'string'
        ? await fetch(`${this.input}/${zoom}/${x}/${y}.${extension}`).then(
            async (res) => await res.arrayBuffer(),
          )
        : await this.input.getTileWM(zoom, x, y);
    if (data === undefined) return undefined;
    const imageData = await imageDecoder(data, { modulo: 256 });
    return new RasterTileReader<T>(zoom, x, y, imageData, isTMS, this.converter);
  }

  /**
   * Grab the tile at the given zoom-x-y coordinates.
   *
   * This function adds the ability to pull from surrounding images and add them as padding
   *
   * This function is also useful for just expanding the zoom level up. So if the image is 256x256,
   * you can use this function to get a 512x512 image
   * @param zoom - the zoom level of the tile
   * @param x - the x coordinate of the tile
   * @param y - the y coordinate of the tile
   * @param padding - the amount of padding to add to each side of the tile
   * @param size - the size of each tile width and height.
   * @param wantedSize - the size of the rendered center tile. For example if you want a 512x512 tile, but the source is 256x256, you can set this to 512.
   * @returns - the tile
   */
  async getTileWithPaddingWM(
    zoom: number,
    x: number,
    y: number,
    padding: number,
    size = 512,
    wantedSize = size,
  ): Promise<RasterTileReader<T> | undefined> {
    const { scheme } = await this.getMetadata();
    const isTMS = scheme === 'tms';
    // Setup a grid
    const grid = buildTileGridWM({ zoom, x, y }, padding, size, wantedSize, isTMS);
    // track the tiles we've already fetched so we don't keep fetching
    const fetchMap = new Map<string, ImageData>();
    for (const tileGuide of grid) {
      const { zoom, x, y } = tileGuide.tile;
      const key = `${zoom}/${x}/${y}`;
      const image = fetchMap.get(key);
      if (image !== undefined) {
        tileGuide.image = image;
        continue;
      }
      const tile = await this.getTileWM(zoom, x, y);
      if (tile === undefined) continue;
      fetchMap.set(key, tile.image);
      tileGuide.image = tile.image;
    }
    // Now merge the images into a single image
    const merged: ImageData = mergeTileGridWM(grid, wantedSize, padding);

    return new RasterTileReader<T>(zoom, x, y, merged, isTMS, this.converter);
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
  async hasTileWM(zoom: number, x: number, y: number): Promise<boolean> {
    const { extension } = await this.getMetadata();
    if (typeof this.input === 'string') {
      const response = await fetch(`${this.input}/${zoom}/${x}/${y}.${extension}`, {
        method: 'HEAD',
      });
      return response.ok;
    } else {
      return await this.input.hasTileWM(zoom, x, y);
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
   * Iterate over all the tiles in the tileset and yield their positions
   * @returns - an iterator over all the tiles in the tileset
   * @yields - the metadata of each tile
   */
  async *iterate(): AsyncGenerator<TileID> {
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
          : await this.hasTileWM(zoom, x, y);
        if (hasTile) yield isS2 ? { face, zoom, x, y } : { zoom, x, y };
        if (zoom < minzoom || (zoom !== threshold && hasTile)) {
          stack.push(
            [zoom + 1, x * 2, y * 2],
            [zoom + 1, x * 2 + 1, y * 2],
            [zoom + 1, x * 2, y * 2 + 1],
            [zoom + 1, x * 2 + 1, y * 2 + 1],
          );
          continue;
        }
      }
    }
  }

  /**
   * Iterate over all tiles in the archive
   * @yields {S2Feature<S2TileID, T, Properties> | VectorFeature<WMTileID, T, Properties>}
   * the each of the tile's pixel RGBA data as lon-lat or S2 s-t coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    S2Feature<S2TileID, T, Properties> | VectorFeature<WMTileID, T, Properties>
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
          : await this.hasTileWM(zoom, x, y);
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
            : await this.getTileWM(zoom, x, y);
          if (tile === undefined) continue;
          yield* tile;
        }
      }
    }
  }
}

/** WM Raster Tile Reader */
export class RasterTileReader<
  T extends MValue = RGBA | ElevationPoint,
  P extends Properties = T,
> implements FeatureIterator<WMTileID, T, P> {
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
   * @yields {VectorFeature<TileID, T, P>} the each of the tile's pixel RGBA data as lon-lat
   * coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<WMTileID, T, P>> {
    const { zoom, x, y, image, tmsStyle } = this;
    const { width: tileSize, data } = image;
    const channels = data.length / (tileSize * tileSize);
    // Get the bounding box of the tile in lon-lat
    const [west, south, east, north] = xyzToBBOX(x, y, zoom, tmsStyle, '900913');
    const xStep = (east - west) / tileSize;
    const yStep = (north - south) / tileSize;
    const coordinates: VectorPoint<T>[] = [];

    for (let py = 0; py < tileSize; py++) {
      const yPos = north - (py + 0.5) * yStep; // Center of the row
      for (let px = 0; px < tileSize; px++) {
        const xPos = west + (px + 0.5) * xStep; // Center of the column
        const index = (py * tileSize + px) * channels;
        const { x: lon, y: lat } = mercToLL({ x: xPos, y: yPos });
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
      properties: {} as P,
      metadata: { zoom, x, y },
    };
  }
}

/** S2 Raster Tile Reader */
export class RasterS2TileReader<
  T extends MValue = RGBA | ElevationPoint,
  P extends Properties = T,
> implements FeatureIterator<S2TileID, T, P> {
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
   * @yields {S2Feature<S2TileID, T, P>} The each of the tile's pixel RGBA data as S2 s-t
   * coordinates with the RGBA as m-values
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<S2Feature<S2TileID, T, P>> {
    const { face, zoom, x, y, image } = this;
    const { width: tileSize, data } = image;
    const channels = data.length / (tileSize * tileSize);
    // Get the bounding box of the tile in s-t space
    const [minS, minT, maxS, maxT] = bboxST(x, y, zoom);
    const sStep = (maxS - minS) / tileSize;
    const tStep = (maxT - minT) / tileSize;
    const coordinates: VectorPoint<T>[] = [];

    for (let py = 0; py < tileSize; py++) {
      const y = minS + (py + 0.5) * tStep; // Center of the row
      for (let px = 0; px < tileSize; px++) {
        const x = minT + (px + 0.5) * sStep; // Center of the column
        const index = (py * tileSize + px) * channels;
        const m: RGBA | ElevationPoint =
          this.converter !== undefined
            ? { elev: this.converter(data[index], data[index + 1], data[index + 2]) }
            : {
                r: data[index],
                g: data[index + 1],
                b: data[index + 2],
                a: channels === 4 ? data[index + 3] : 255,
              };
        coordinates.push({ x, y, m: m as unknown as T });
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
      properties: {} as P,
      metadata: { face, zoom, x, y },
    };
  }
}
