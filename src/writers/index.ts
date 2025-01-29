import type { Compression } from '..';
import type { Face, Metadata } from 's2-tilejson';

export * from './pmtiles';

/** The defacto interface for all writers. */
export interface Writer {
  write(data: Uint8Array, offset: number): Promise<void>;
  append(data: Uint8Array): Promise<void>;
  appendSync(data: Uint8Array): void;
  appendString(string: string): Promise<void>;
  appendStringSync(string: string): void;
}

/** A base interface for all tile stores. */
export interface TileWriter {
  writeTileWM(zoom: number, x: number, y: number, data: Uint8Array): Promise<void>;
  writeTileS2(face: Face, zoom: number, x: number, y: number, data: Uint8Array): Promise<void>;
  commit(metadata: Metadata, tileCompression?: Compression): Promise<void>;
}

/** A base interface for all tile stores. */
export interface TemporalTileWriter extends TileWriter {
  writeTemporalTileWM(
    time: Date,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void>;
  writeTemporalTileS2(
    time: Date,
    face: number,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void>;
}

/** A Tile writer designed for testing. */
export class BufferTileWriter implements TemporalTileWriter {
  metadata: Metadata | null = null;
  tiles: Map<string, Uint8Array> = new Map();

  /**
   * Write a tile to the folder location given its (z, x, y) coordinates.
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileWM(zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    const key = `${zoom}/${x}/${y}`;
    await this.tiles.set(key, data);
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
    face: Face,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array,
  ): Promise<void> {
    const key = `${face}/${zoom}/${x}/${y}`;
    await this.tiles.set(key, data);
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
    const key = `${time.getTime()}/${zoom}/${x}/${y}`;
    await this.tiles.set(key, data);
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
    const key = `${time.getTime()}/${face}/${zoom}/${x}/${y}`;
    await this.tiles.set(key, data);
  }

  /**
   * Finish writing by building the header with root and leaf directories.
   * @param metadata - the metadata about all the tiles to store
   */
  async commit(metadata: Metadata): Promise<void> {
    this.metadata = metadata;
    await Promise.resolve();
  }
}

/** Buffer writer is used on smaller datasets that are easy to write in memory. Faster then the Filesystem */
export class BufferWriter implements Writer {
  #buffer: number[] = [];
  #textEncoder = new TextEncoder();

  /**
   * Append data to the buffer
   * @param data - the data to append
   */
  async append(data: Uint8Array): Promise<void> {
    for (let i = 0; i < data.byteLength; i++) this.#buffer.push(data[i]);
    await true;
  }

  /**
   * Append string to the buffer
   * @param string - the string to append
   */
  async appendString(string: string): Promise<void> {
    await this.append(this.#textEncoder.encode(string) as Uint8Array);
  }

  /**
   * Append data to the buffer synchronously
   * @param data - the data to append
   */
  appendSync(data: Uint8Array): void {
    for (let i = 0; i < data.byteLength; i++) this.#buffer.push(data[i]);
  }

  /**
   * Append string to the buffer synchronously
   * @param string - the string to append
   */
  appendStringSync(string: string): void {
    this.appendSync(this.#textEncoder.encode(string) as Uint8Array);
  }

  /**
   * Write data to the buffer
   * @param data - the data to write
   * @param offset - where in the buffer to start
   */
  async write(data: Uint8Array, offset: number): Promise<void> {
    for (let i = 0; i < data.byteLength; i++) {
      this.#buffer[offset + i] = data[i];
    }
    await true;
  }

  /** @returns - the buffer */
  commit(): Uint8Array {
    return new Uint8Array(this.#buffer);
  }
}
