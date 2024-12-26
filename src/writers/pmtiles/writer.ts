import { concatUint8Arrays } from '../../util';
import { s2HeaderToBytes } from './s2pmtiles';
import {
  Compression,
  ROOT_SIZE,
  S2_HEADER_SIZE_BYTES,
  S2_ROOT_SIZE,
  zxyToTileID,
} from '../../readers/pmtiles';
import { headerToBytes, serializeDir } from './pmtiles';

import type { Entry, Header, S2Entries, S2Header, TileType } from '../../readers/pmtiles';
import type { Face, Metadata } from 's2-tilejson';
import type { TileWriter, Writer } from '..';

/** Write a PMTiles file. */
export class S2PMTilesWriter implements TileWriter {
  #tileEntries: Entry[] = [];
  #s2tileEntries: S2Entries = { 0: [], 1: [], 2: [], 3: [], 4: [], 5: [] };
  #offset = 0;
  #addressedTiles = 0;
  #clustered = true;
  #minZoom = 30;
  #maxZoom = 0;
  /**
   * @param writer - the writer to append to
   * @param type - the tile type
   * @param compression - the compression algorithm
   */
  constructor(
    readonly writer: Writer,
    readonly type: TileType,
    readonly compression: Compression = Compression.Gzip,
  ) {
    this.writer.appendSync(new Uint8Array(S2_ROOT_SIZE));
  }

  /**
   * Write a tile to the PMTiles file given its (z, x, y) coordinates.
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileXYZ(
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array<ArrayBuffer>,
  ): Promise<void> {
    this.#minZoom = Math.min(this.#minZoom, zoom);
    this.#maxZoom = Math.max(this.#maxZoom, zoom);
    const tileID = zxyToTileID(zoom, x, y);
    await this.writeTile(tileID, data);
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
    face: Face,
    zoom: number,
    x: number,
    y: number,
    data: Uint8Array<ArrayBuffer>,
  ): Promise<void> {
    this.#minZoom = Math.min(this.#minZoom, zoom);
    this.#maxZoom = Math.max(this.#maxZoom, zoom);
    const tileID = zxyToTileID(zoom, x, y);
    await this.writeTile(tileID, data, face);
  }

  /**
   * Write a tile to the PMTiles file given its tile ID.
   * @param tileID - the tile ID
   * @param data - the tile data
   * @param face - If it exists, then we are storing S2 data
   */
  async writeTile(tileID: number, data: Uint8Array<ArrayBuffer>, face?: Face): Promise<void> {
    data = await compress(data, this.compression);
    const length = data.length;
    const tileEntries = face !== undefined ? this.#s2tileEntries[face] : this.#tileEntries;
    if (tileEntries.length > 0 && tileID < (tileEntries.at(-1) as Entry).tileID) {
      this.#clustered = false;
    }

    const offset = this.#offset;
    await this.writer.append(data);
    tileEntries.push({ tileID, offset, length, runLength: 1 });
    this.#offset += length;

    this.#addressedTiles++;
  }

  /**
   * Finish writing by building the header with root and leaf directories
   * @param metadata - the metadata to store
   */
  async commit(metadata: Metadata): Promise<void> {
    if (this.#tileEntries.length === 0) await this.#commitS2(metadata);
    else await this.#commit(metadata);
  }

  /**
   * Finish writing by building the header with root and leaf directories
   * @param metadata - the metadata to store
   */
  async #commit(metadata: Metadata): Promise<void> {
    const tileEntries = this.#tileEntries;
    // keep tile entries sorted
    tileEntries.sort((a, b) => a.tileID - b.tileID);
    // build metadata
    const metaBuffer = Buffer.from(JSON.stringify(metadata));
    let metauint8 = new Uint8Array(metaBuffer.buffer, metaBuffer.byteOffset, metaBuffer.byteLength);
    metauint8 = await compress(metauint8, this.compression);

    // optimize directories
    const { rootBytes, leavesBytes } = await optimizeDirectories(
      tileEntries,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      this.compression,
    );

    // build header data
    const rootDirectoryOffset = S2_HEADER_SIZE_BYTES;
    const rootDirectoryLength = rootBytes.byteLength;
    const jsonMetadataOffset = rootDirectoryOffset + rootDirectoryLength;
    const jsonMetadataLength = metauint8.byteLength;
    const leafDirectoryOffset = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength = leavesBytes.byteLength;
    this.#offset += leavesBytes.byteLength;
    await this.writer.append(leavesBytes);

    // build header
    const header: Header = {
      specVersion: 3,
      rootDirectoryOffset,
      rootDirectoryLength,
      jsonMetadataOffset,
      jsonMetadataLength,
      leafDirectoryOffset,
      leafDirectoryLength,
      tileDataOffset: S2_ROOT_SIZE,
      tileDataLength: this.#offset,
      numAddressedTiles: this.#addressedTiles,
      numTileEntries: tileEntries.length,
      numTileContents: tileEntries.length,
      clustered: this.#clustered,
      internalCompression: this.compression,
      tileCompression: this.compression,
      tileType: this.type,
      minZoom: this.#minZoom,
      maxZoom: this.#maxZoom,
    };
    const serialzedHeader = headerToBytes(header);

    // write header
    await this.writer.write(serialzedHeader, 0);
    await this.writer.write(rootBytes, rootDirectoryOffset);
    await this.writer.write(metauint8, jsonMetadataOffset);
  }

  /**
   * Finish writing by building the header with root and leaf directories
   * @param metadata - the metadata to store
   */
  async #commitS2(metadata: Metadata): Promise<void> {
    const { compression } = this;
    const tileEntries = this.#s2tileEntries[0];
    const tileEntries1 = this.#s2tileEntries[1];
    const tileEntries2 = this.#s2tileEntries[2];
    const tileEntries3 = this.#s2tileEntries[3];
    const tileEntries4 = this.#s2tileEntries[4];
    const tileEntries5 = this.#s2tileEntries[5];
    // keep tile entries sorted
    tileEntries.sort((a, b) => a.tileID - b.tileID);
    tileEntries1.sort((a, b) => a.tileID - b.tileID);
    tileEntries2.sort((a, b) => a.tileID - b.tileID);
    tileEntries3.sort((a, b) => a.tileID - b.tileID);
    tileEntries4.sort((a, b) => a.tileID - b.tileID);
    tileEntries5.sort((a, b) => a.tileID - b.tileID);
    // build metadata
    const metaBuffer = Buffer.from(JSON.stringify(metadata));
    let metauint8 = new Uint8Array(metaBuffer.buffer, metaBuffer.byteOffset, metaBuffer.byteLength);
    metauint8 = await compress(metauint8, this.compression);

    // optimize directories
    const { rootBytes, leavesBytes } = await optimizeDirectories(
      tileEntries,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );
    const { rootBytes: rootBytes1, leavesBytes: leavesBytes1 } = await optimizeDirectories(
      tileEntries1,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );
    const { rootBytes: rootBytes2, leavesBytes: leavesBytes2 } = await optimizeDirectories(
      tileEntries2,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );
    const { rootBytes: rootBytes3, leavesBytes: leavesBytes3 } = await optimizeDirectories(
      tileEntries3,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );
    const { rootBytes: rootBytes4, leavesBytes: leavesBytes4 } = await optimizeDirectories(
      tileEntries4,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );
    const { rootBytes: rootBytes5, leavesBytes: leavesBytes5 } = await optimizeDirectories(
      tileEntries5,
      ROOT_SIZE - S2_HEADER_SIZE_BYTES - metauint8.byteLength,
      compression,
    );

    // build header data
    const rootDirectoryOffset = S2_HEADER_SIZE_BYTES;
    const rootDirectoryLength = rootBytes.byteLength;
    const rootDirectoryOffset1 = rootDirectoryOffset + rootDirectoryLength;
    const rootDirectoryLength1 = rootBytes1.byteLength;
    const rootDirectoryOffset2 = rootDirectoryOffset1 + rootDirectoryLength1;
    const rootDirectoryLength2 = rootBytes2.byteLength;
    const rootDirectoryOffset3 = rootDirectoryOffset2 + rootDirectoryLength2;
    const rootDirectoryLength3 = rootBytes3.byteLength;
    const rootDirectoryOffset4 = rootDirectoryOffset3 + rootDirectoryLength3;
    const rootDirectoryLength4 = rootBytes4.byteLength;
    const rootDirectoryOffset5 = rootDirectoryOffset4 + rootDirectoryLength4;
    const rootDirectoryLength5 = rootBytes5.byteLength;
    // metadata
    const jsonMetadataOffset = rootDirectoryOffset5 + rootDirectoryLength5;
    const jsonMetadataLength = metauint8.byteLength;
    // leafs
    const leafDirectoryOffset = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength = leavesBytes.byteLength;
    this.#offset += leafDirectoryLength;
    await this.writer.append(leavesBytes);
    const leafDirectoryOffset1 = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength1 = leavesBytes1.byteLength;
    this.#offset += leafDirectoryLength1;
    await this.writer.append(leavesBytes1);
    const leafDirectoryOffset2 = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength2 = leavesBytes2.byteLength;
    this.#offset += leafDirectoryLength2;
    await this.writer.append(leavesBytes2);
    const leafDirectoryOffset3 = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength3 = leavesBytes3.byteLength;
    this.#offset += leafDirectoryLength3;
    await this.writer.append(leavesBytes3);
    const leafDirectoryOffset4 = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength4 = leavesBytes4.byteLength;
    this.#offset += leafDirectoryLength4;
    await this.writer.append(leavesBytes4);
    const leafDirectoryOffset5 = this.#offset + S2_ROOT_SIZE;
    const leafDirectoryLength5 = leavesBytes5.byteLength;
    this.#offset += leafDirectoryLength5;
    await this.writer.append(leavesBytes5);
    // build header
    const header: S2Header = {
      specVersion: 3,
      rootDirectoryOffset,
      rootDirectoryLength,
      rootDirectoryOffset1,
      rootDirectoryLength1,
      rootDirectoryOffset2,
      rootDirectoryLength2,
      rootDirectoryOffset3,
      rootDirectoryLength3,
      rootDirectoryOffset4,
      rootDirectoryLength4,
      rootDirectoryOffset5,
      rootDirectoryLength5,
      jsonMetadataOffset,
      jsonMetadataLength,
      leafDirectoryOffset,
      leafDirectoryLength,
      leafDirectoryOffset1,
      leafDirectoryLength1,
      leafDirectoryOffset2,
      leafDirectoryLength2,
      leafDirectoryOffset3,
      leafDirectoryLength3,
      leafDirectoryOffset4,
      leafDirectoryLength4,
      leafDirectoryOffset5,
      leafDirectoryLength5,
      tileDataOffset: S2_ROOT_SIZE,
      tileDataLength: this.#offset,
      numAddressedTiles: this.#addressedTiles,
      numTileEntries: tileEntries.length,
      numTileContents: tileEntries.length,
      clustered: this.#clustered,
      internalCompression: this.compression,
      tileCompression: this.compression,
      tileType: this.type,
      minZoom: this.#minZoom,
      maxZoom: this.#maxZoom,
    };
    const serialzedHeader = s2HeaderToBytes(header);

    // write header
    await this.writer.write(serialzedHeader, 0);
    await this.writer.write(rootBytes, rootDirectoryOffset);
    await this.writer.write(rootBytes1, rootDirectoryOffset1);
    await this.writer.write(rootBytes2, rootDirectoryOffset2);
    await this.writer.write(rootBytes3, rootDirectoryOffset3);
    await this.writer.write(rootBytes4, rootDirectoryOffset4);
    await this.writer.write(rootBytes5, rootDirectoryOffset5);
    await this.writer.write(metauint8, jsonMetadataOffset);
  }
}

/** The result of an optimized directory computation */
interface OptimizedDirectory {
  /** The root directory bytes */
  rootBytes: Uint8Array<ArrayBuffer>;
  /** The leaf directories bytes */
  leavesBytes: Uint8Array<ArrayBuffer>;
  /** The number of leaf directories */
  numLeaves: number;
}

/**
 * @param entries - the tile entries
 * @param leafSize - the max leaf size
 * @param compression - the compression
 * @returns - the optimized directories
 */
async function buildRootsLeaves(
  entries: Entry[],
  leafSize: number,
  compression: Compression,
): Promise<OptimizedDirectory> {
  const rootEntries: Entry[] = [];
  let leavesBytes = new Uint8Array(0);
  let numLeaves = 0;

  let i = 0;
  while (i < entries.length) {
    numLeaves += 1;
    const serialized = await compress(serializeDir(entries.slice(i, i + leafSize)), compression);
    rootEntries.push({
      tileID: entries[i].tileID,
      offset: leavesBytes.length,
      length: serialized.length,
      runLength: 0,
    });
    leavesBytes = await concatUint8Arrays([leavesBytes, serialized]);
    i += leafSize;
  }

  return {
    rootBytes: await compress(serializeDir(rootEntries), compression),
    leavesBytes,
    numLeaves,
  };
}

/**
 * @param entries - the tile entries
 * @param targetRootLength - the max leaf size
 * @param compression - the compression
 * @returns - the optimized directories
 */
async function optimizeDirectories(
  entries: Entry[],
  targetRootLength: number,
  compression: Compression,
): Promise<OptimizedDirectory> {
  const testBytes = await compress(serializeDir(entries), compression);
  if (testBytes.length < targetRootLength)
    return { rootBytes: testBytes, leavesBytes: new Uint8Array(0), numLeaves: 0 };

  let leafSize = 4096;
  while (true) {
    const build = await buildRootsLeaves(entries, leafSize, compression);
    if (build.rootBytes.length < targetRootLength) return build;
    leafSize *= 2;
  }
}

// /**
//  * @param a - the first array
//  * @param b - the second array
//  * @returns - the combined array of the two starting with "a"
//  */
// function concatUint8Arrays(a: Uint8Array, b: Uint8Array): Uint8Array {
//   const result = new Uint8Array(a.length + b.length);
//   result.set(a, 0);
//   result.set(b, a.length);
//   return result;
// }

/**
 * @param input - the input Uint8Array
 * @param compression - the compression
 * @returns - the compressed Uint8Array or the original if compression is None
 */
async function compress(
  input: Uint8Array<ArrayBuffer>,
  compression: Compression,
): Promise<Uint8Array<ArrayBuffer>> {
  if (compression === Compression.None) return input;
  else if (compression === Compression.Gzip) return await compressGzip(input);
  else throw new Error(`Unsupported compression: ${compression}`);
}

/**
 * @param input - the input Uint8Array
 * @returns - the compressed Uint8Array
 */
async function compressGzip(input: Uint8Array<ArrayBuffer>): Promise<Uint8Array<ArrayBuffer>> {
  // Convert the string to a byte stream.
  const stream = new Blob([input]).stream();

  // Create a compressed stream.
  const compressedStream = stream.pipeThrough(new CompressionStream('gzip'));

  // Read all the bytes from this stream.
  const chunks = [];
  for await (const chunk of compressedStream) chunks.push(chunk);

  return await concatUint8Arrays(chunks);
}
