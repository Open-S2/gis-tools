import { existsSync } from 'fs';
import { mkdir, writeFile } from 'fs/promises';

import type { Metadata } from 's2-tilejson';
import type { TileWriter } from '.';

/** This is a filesystem Tile writer that organizes data via folders. */
export class FileTileWriter implements TileWriter {
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
   * Write a tile to the PMTiles file given its (z, x, y) coordinates.
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileXYZ(zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    // if folders don't exist, create it
    const folders = `${this.path}/${zoom}/${x}`;
    if (!existsSync(folders)) await mkdir(folders, { recursive: true });

    await writeFile(`${folders}/${y}.${this.fileType}`, data);
  }

  /**
   * Write a tile to the PMTiles file given its (face, zoom, x, y) coordinates.
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
   * Finish writing by building the header with root and leaf directories.
   * @param metadata - the metadata about all the tiles to store
   */
  async commit(metadata: Metadata): Promise<void> {
    await writeFile(`${this.path}/metadata.json`, JSON.stringify(metadata));
  }
}
