import { Compression, compressStream } from '../../util/index.js';

import type { Face } from 'gis-tools/index.js';
import type { Metadata } from 's2-tilejson';
import type { TileWriter, Writer } from '../index.js';

/**
 * A Node consists of an offset and a length pointing to a node
 * Offset: 6 bytes
 * Length: 4 bytes
 */
type Node = Directory;
/**
 * A directory consists of an offset and a length pointing to a node or a leaf.
 * Offset: 6 bytes
 * Length: 4 bytes
 */
type Directory = [offset: number, length: number];

const NODE_SIZE = 10; // [offset, length] => [6 bytes, 4 bytes]
const DIR_SIZE = 1_365 * NODE_SIZE; // (13_650) -> 6 levels, the 6th level has both node and leaf (1+4+16+64+256+1024)*2 => (1365)+1365 => 2_730
const METADATA_SIZE = 131_072; // 131,072 bytes is 128kB. It is assumed the map metadata AND the S2Tile format metadata is less than 128kB
const ROOT_DIR_SIZE = DIR_SIZE * 6; // 27_300 * 6 = 163_800
const ROOT_SIZE = METADATA_SIZE + ROOT_DIR_SIZE;
// assuming all tiles exist for every face from 0->30 the max leafs to reach depth of 30 is 5
// root: 6sides * 27_300bytes/dir = (163_800 bytes)
// all leafs at 6: 1024 * 6sides * 27_300bytes/dir (0.167731 GB)
// al leafs at 12: 524_288 * 6sides * 27_300bytes/dir (85.8783744 GB) - obviously most of this is water

/**
 * # S2 Tiles Writer
 *
 * ## Description
 *
 * An S2 Tile Writer to store tile and metadata in a cloud optimized format. Similar to PMTiles
 * but simplified to have as few features as possible.
 *
 * Reads either a Web Mercator tile or an S2 tile to the folder location given its (zoom, x, y) or (face, zoom, x, y) coordinates.
 *
 * Writes data via the [S2Tiles specification](https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md).
 *
 * ## Usage
 *
 * ### Browser Compatible
 * ```ts
 * import { BufferWriter, S2TilesWriter, Compression } from 'gis-tools-ts';
 * import type { Metadata } from 'gis-tools-ts';
 *
 * // Setup the writers
 * const bufWriter = new BufferWriter();
 * const writer = new S2TilesWriter(bufWriter, 12, Compression.Gzip);
 * // example data
 * const txtEncoder = new TextEncoder();
 * const str = 'hello world';
 * const uint8 = txtEncoder.encode(str);
 * const str2 = 'hello world 2';
 * const uint8_2 = txtEncoder.encode(str2);
 * // write data in tile
 * await writer.writeTileWM(0, 0, 0, uint8);
 * await writer.writeTileWM(1, 0, 1, uint8);
 * await writer.writeTileWM(5, 2, 9, uint8_2);
 * // If S2 tiles:
 * await writer.writeTileS2(1, 0, 0, 0, uint8);
 * // finish
 * await writer.commit({ metadata: true } as unknown as Metadata);
 * // Get the result Uint8Array
 * const resultData = bufWriter.commit();
 * ```
 *
 * ### Node/Deno/Bun using the filesystem
 * ```ts
 * import { S2TilesWriter, Compression } from 'gis-tools-ts';
 * import { FileWriter } from 'gis-tools-ts/file';
 *
 * const writer = new S2TilesWriter(new FileWriter('./output.s2tiles'), 10, Compression.Gzip);
 * // SAME AS ABOVE
 * ```
 *
 * ## Links
 * - https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md
 */
export class S2TilesWriter implements TileWriter {
  offset = ROOT_SIZE;
  version = 1;
  encoder = new TextEncoder();
  /**
   * @param writer - the writer to append to
   * @param maxzoom - the maximum zoom level to write to
   * @param compression - the compression algorithm to use for internal data like the
   */
  constructor(
    readonly writer: Writer,
    readonly maxzoom: number,
    readonly compression: Compression = Compression.Gzip,
  ) {
    this.writer.appendSync(new Uint8Array(ROOT_SIZE));
  }

  /**
   * Write a tile to the S2Tiles file given its (z, x, y) coordinates.
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async writeTileWM(zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    await this.putTileIJ(0, zoom, x, y, data);
  }

  /**
   * Write a tile to the S2Tiles file given its (face, zoom, x, y) coordinates.
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
    await this.putTileIJ(face, zoom, x, y, data);
  }

  /**
   * Finish writing by building the header with root and leaf directories
   * @param metadata - the metadata to store
   * @param tileCompression - the compression algorithm that was used on the tiles [Default: None]
   */
  async commit(metadata: Metadata, tileCompression?: Compression): Promise<void> {
    // set the ID, version, and compression type
    const data = new DataView(new ArrayBuffer(10));
    // Store format metadata
    data.setUint8(0, 83); // S
    data.setUint8(1, 50); // 2
    data.setUint16(2, this.version, true);
    data.setUint8(4, this.maxzoom);
    data.setUint8(5, tileCompression ?? this.compression);
    // store the metadata's length then actual data
    let metaBuffer = this.encoder.encode(JSON.stringify(metadata));
    metaBuffer = await compress(metaBuffer, this.compression);
    if (metaBuffer.byteLength > METADATA_SIZE - 10) {
      throw new Error('Metadata too large for S2Tiles');
    }
    data.setUint32(6, metaBuffer.byteLength, true);
    // store the format metadata and lengthen the writer to fill METADATA_SIZE. Then store the map metadata
    await this.writer.write(new Uint8Array(data.buffer, 0, 10), 0);
    await this.writer.write(metaBuffer, 10);
  }

  /**
   * Write a tile to the S2Tiles file given its (face, zoom, x, y) coordinates.
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data to store
   */
  async putTileIJ(face: Face, zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    await this.putTile(face, zoom, x, y, data);
  }

  /**
   * Inserts a tile into the S2Tiles store.
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param data - the tile data
   */
  async putTile(face: Face, zoom: number, x: number, y: number, data: Uint8Array): Promise<void> {
    const length = data.byteLength;
    // first create node, setting offset
    const node: Node = [this.offset, length];
    await this.writer.append(await compress(data, this.compression));
    this.offset += length;
    // store node in the correct directory
    await this.#putNodeInDir(face, zoom, x, y, node);
  }

  /**
   * Work our way towards the correct parent directory.
   * If parent directory does not exists, we create it.
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @param node - the node
   */
  async #putNodeInDir(face: Face, zoom: number, x: number, y: number, node: Node): Promise<void> {
    // use the s2cellID and move the cursor
    const cursor = await this.#walk(face, zoom, x, y);
    // finally store
    await this.#writeNode(cursor, node);
  }

  /**
   * given position and level, explain where to adust the cursor to file
   * @param face - the Open S2 projection face
   * @param zoom - the zoom level
   * @param x - the tile X coordinate
   * @param y - the tile Y coordinate
   * @returns - the new cursor position
   */
  async #walk(face: Face, zoom: number, x: number, y: number): Promise<number> {
    const { maxzoom } = this;

    let leafNode: DataView = new DataView(new ArrayBuffer(NODE_SIZE));
    let cursor: number = METADATA_SIZE + DIR_SIZE * face;
    let leaf: number;
    let depth = 0;
    const path = getPath(zoom, x, y);

    while (path.length !== 0) {
      // grab movement
      const shift = path.shift() ?? 0;
      depth++;
      // update cursor position
      cursor += shift * NODE_SIZE;

      if (path.length !== 0) {
        // if we hit a leaf, adjust nodePos position and move cursor to new directory
        // if we are at the max zoom, we are already in the correct position (the "leaf" is actually a node instead)
        if (maxzoom % 5 === 0 && path.length === 1 && zoom === maxzoom && path[0] === 0)
          return cursor;
        // grab the leaf from the file
        leafNode = new DataView((await this.writer.slice(cursor, cursor + NODE_SIZE)).buffer);
        leaf = _readUInt48LE(leafNode);
        // if the leaf doesn't exist we create a new directory to host it
        if (leaf === 0) {
          cursor = await this.#createLeaf(cursor, depth * 5);
        } else {
          cursor = leaf;
        } // move to where leaf is pointing
      }
    }

    return cursor;
  }

  /**
   * Create a new leaf directory
   * @param cursor - the cursor
   * @param depth - the depth
   * @returns - the offset of the new leaf
   */
  async #createLeaf(cursor: number, depth: number): Promise<number> {
    // build directory size according to maxzoom
    const dirSize = _buildDirSize(depth, this.maxzoom);
    // create offset & node
    const offset = this.offset;
    const node: Node = [offset, dirSize];
    // create a dir of said size and update to new offset
    await this.writer.write(new Uint8Array(dirSize), offset);
    this.offset += dirSize;
    // store our newly created directory as a leaf directory in our current directory
    await this.#writeNode(cursor, node);

    // return the offset of the leaf directory
    return offset;
  }

  /**
   * Writes a node to the file
   * @param cursor - the cursor
   * @param node - the node
   */
  async #writeNode(cursor: number, node: Node): Promise<void> {
    const [offset, length] = node;
    // write offset and length to buffer
    const nodeBuf = new DataView(new ArrayBuffer(NODE_SIZE));
    _writeUInt48LE(nodeBuf, offset);
    nodeBuf.setUint32(6, length, true);
    // write buffer to file at directory offset
    await this.writer.write(new Uint8Array(nodeBuf.buffer), cursor);
  }
}

/**
 * write a 32 bit and a 16 bit
 * @param data - the data to write to
 * @param num - the number
 * @param offset - the offset to write at
 */
function _writeUInt48LE(data: DataView, num: number, offset = 0): void {
  const lower = num & 0xffff;
  const upper = num / (1 << 16);

  data.setUint16(offset, lower, true);
  data.setUint32(offset + 2, upper, true);
}

/**
 * read a 48 bit number
 * @param buffer - the buffer
 * @param offset - the offset
 * @returns - the number
 */
function _readUInt48LE(buffer: DataView, offset = 0): number {
  return buffer.getUint32(offset + 2, true) * (1 << 16) + buffer.getUint16(offset, true);
}

/**
 * Build a directory size relative to maxzoom
 * @param depth - the depth
 * @param maxzoom - the maxzoom
 * @returns - the directory size
 */
function _buildDirSize(depth: number, maxzoom: number): number {
  const { min, pow } = Math;
  let dirSize = 0;
  // grab the remainder
  let remainder = min(maxzoom - depth, 5); // must be increments of 5, so if level 4 then inc is 0 but if 5, inc is 5
  // for each remainder (including 0), we add a quadrant
  do {
    dirSize += pow(1 << remainder, 2);
  } while (remainder-- !== 0);

  return dirSize * NODE_SIZE;
}

/**
 * Get the path to a tile
 * @param zoom - the zoom
 * @param x - the x
 * @param y - the y
 * @returns - the path
 */
function getPath(zoom: number, x: number, y: number): number[] {
  const { max, pow } = Math;
  const path: Array<[number, number, number]> = [];
  while (zoom >= 5) {
    path.push([5, x & 31, y & 31]);
    x >>= 5;
    y >>= 5;
    zoom = max(zoom - 5, 0);
  }
  path.push([zoom, x, y]);
  return path.map(([zoom, x, y]) => {
    let val = 0;
    val += y * (1 << zoom) + x;
    while (zoom-- !== 0) val += pow(1 << zoom, 2);
    return val;
  });
}

/**
 * Compresses a Uint8Array if a compression method is specified
 * @param input - the input Uint8Array
 * @param compression - the compression
 * @returns - the compressed Uint8Array or the original if compression is None
 */
async function compress(input: Uint8Array, compression: Compression): Promise<Uint8Array> {
  if (compression === Compression.None) return input;
  else if (compression === Compression.Gzip) return await compressStream(input);
  else throw new Error(`Unsupported compression: ${compression}`);
}
