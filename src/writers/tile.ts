import { existsSync } from 'fs';
import { mkdir, writeFile } from 'fs/promises';

import type { Metadata } from 's2-tilejson';
import type { TemporalTileWriter } from '.';

/**
 * # Tile File Writer
 *
 * ## Description
 * This is a filesystem Tile writer that organizes data via folders.
 *
 * ## Usage
 *
 * ```ts
 * import { FileTileWriter } from 'gis-tools-ts/file';
 *
 * const tileWriter = new FileTileWriter('./store', 'png');
 *
 * // store WM tiles
 * await tileWriter.writeTileWM(0, 0, 0, data);
 * // store S2 tiles
 * await tileWriter.writeTileS2(0, 0, 0, 0, data);
 * // store temportal WM tiles
 * await tileWriter.writeTemporalTileWM(new Date(), 0, 0, 0, data);
 * // store temportal S2 tiles
 * await tileWriter.writeTemporalTileS2(new Date(), 0, 0, 0, 0, data);
 *
 * // after writing all the tiles, store the metadata
 * await tileWriter.commit(metadata);
 * ```
 *
 * ## Links
 * - https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html
 * - https://cesium.com/blog/2015/04/07/quadtree-cheatseet/
 */
export class FileTileWriter implements TemporalTileWriter {
  /**
   * @param path - the location to write the data
   * @param fileType - the file ending to write
   */
  constructor(
    readonly path: string,
    readonly fileType: string = 'vector.pbf',
  ) {
    // check that the folder exists
    const folderExists = existsSync(this.path);
    if (!folderExists) throw new Error(`Folder ${this.path} does not exist.`);
  }

  /**
   * Write a tile to the folder location given its (z, x, y) coordinates.
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileWM(zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    // if folders don't exist, create it
    const folders = `${this.path}/${zoom}/${x}`;
    if (!existsSync(folders)) await mkdir(folders, { recursive: true });

    await writeFile(`${folders}/${y}.${this.fileType}`, data);
  }

  /**
   * Write a tile to the folder location given its (face, zoom, x, y) coordinates.
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileS2(
    face: number,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void> {
    // if folders don't exist, create it
    const folders = `${this.path}/${face}/${zoom}/${x}`;
    if (!existsSync(folders)) await mkdir(folders, { recursive: true });

    await writeFile(`${folders}/${y}.${this.fileType}`, data);
  }

  /**
   * Write a time series tile to the folder location given its (t, z, x, y) coordinates.
   * @param time - the date of the data
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTemporalTileWM(
    time: Date,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void> {
    // if folders don't exist, create it
    const folders = `${this.path}/${time.toISOString()}/${zoom}/${x}`;
    if (!existsSync(folders)) await mkdir(folders, { recursive: true });

    await writeFile(`${folders}/${y}.${this.fileType}`, data);
  }

  /**
   * Write a time series tile to the folder location given its (face, zoom, x, y) coordinates.
   * @param time - the date of the data
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTemporalTileS2(
    time: Date,
    face: number,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void> {
    // if folders don't exist, create it
    const folders = `${this.path}/${time.toISOString()}/${face}/${zoom}/${x}`;
    if (!existsSync(folders)) await mkdir(folders, { recursive: true });

    await writeFile(`${folders}/${y}.${this.fileType}`, data);
  }

  /**
   * Finish writing by building the header with root and leaf directories.
   * @param metadata - the metadata about all the tiles to store
   */
  async commit(metadata: Metadata): Promise<void> {
    await writeFile(`${this.path}/metadata.json`, JSON.stringify(metadata));
  }
}
