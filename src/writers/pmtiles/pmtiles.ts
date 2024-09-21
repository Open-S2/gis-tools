import { writeVarint } from './varint';

import type { Point } from 's2-tools/geometry';

/** PMTiles v3 directory entry. */
export interface Entry {
  tileID: number;
  offset: number;
  length: number;
  runLength: number;
}

/**
 * Enum representing a compression algorithm used.
 * 0 = unknown compression, for if you must use a different or unspecified algorithm.
 * 1 = no compression.
 * 2 = gzip
 * 3 = brotli
 * 4 = zstd
 */
export enum Compression {
  /** unknown compression, for if you must use a different or unspecified algorithm. */
  Unknown = 0,
  /** no compression. */
  None = 1,
  /** gzip. */
  Gzip = 2,
  /** brotli. */
  Brotli = 3,
  /** zstd. */
  Zstd = 4,
}

/**
 * Describe the type of tiles stored in the archive.
 * 0 is unknown/other, 1 is "MVT" vector tiles.
 */
export enum TileType {
  /** unknown/other. */
  Unknown = 0,
  /** Vector tiles. */
  Pbf = 1,
  /** Image tiles. */
  Png = 2,
  /** Image tiles. */
  Jpeg = 3,
  /** Image tiles. */
  Webp = 4,
  /** Image tiles. */
  Avif = 5,
}

/**
 * PMTiles v3 header storing basic archive-level information.
 */
export interface Header {
  specVersion: number;
  rootDirectoryOffset: number;
  rootDirectoryLength: number;
  jsonMetadataOffset: number;
  jsonMetadataLength: number;
  leafDirectoryOffset: number;
  leafDirectoryLength?: number;
  tileDataOffset: number;
  tileDataLength?: number;
  numAddressedTiles: number;
  numTileEntries: number;
  numTileContents: number;
  clustered: boolean;
  internalCompression: Compression;
  tileCompression: Compression;
  tileType: TileType;
  minZoom: number;
  maxZoom: number;
  etag?: string;
}

export const HEADER_SIZE_BYTES = 127;

export const ROOT_SIZE = 16_384;

/**
 * @param n - the rotation size
 * @param xy - the point
 * @param rx - the x rotation
 * @param ry - the y rotation
 */
function rotate(n: number, xy: Point, rx: number, ry: number): void {
  if (ry === 0) {
    if (rx === 1) {
      xy[0] = n - 1 - xy[0];
      xy[1] = n - 1 - xy[1];
    }
    const t = xy[0];
    xy[0] = xy[1];
    xy[1] = t;
  }
}

const tzValues: number[] = [
  0, 1, 5, 21, 85, 341, 1365, 5461, 21845, 87381, 349525, 1398101, 5592405, 22369621, 89478485,
  357913941, 1431655765, 5726623061, 22906492245, 91625968981, 366503875925, 1466015503701,
  5864062014805, 23456248059221, 93824992236885, 375299968947541, 1501199875790165,
];

/**
 * Convert Z,X,Y to a Hilbert TileID.
 * @param zoom - the zoom level
 * @param x - the x coordinate
 * @param y - the y coordinate
 * @returns - the Hilbert encoded TileID
 */
export function zxyToTileID(zoom: number, x: number, y: number): number {
  if (zoom > 26) {
    throw Error('Tile zoom level exceeds max safe number limit (26)');
  }
  if (x > 2 ** zoom - 1 || y > 2 ** zoom - 1) {
    throw Error('tile x/y outside zoom level bounds');
  }

  const acc = tzValues[zoom];
  const n = 2 ** zoom;
  let rx = 0;
  let ry = 0;
  let d = 0;
  const xy: [x: number, y: number] = [x, y];
  let s = n / 2;
  while (true) {
    rx = (xy[0] & s) > 0 ? 1 : 0;
    ry = (xy[1] & s) > 0 ? 1 : 0;
    d += s * s * ((3 * rx) ^ ry);
    rotate(s, xy, rx, ry);
    if (s <= 1) break;
    s = s / 2;
  }
  return acc + d;
}

/**
 * @param header - the header object
 * @returns the raw header bytes
 */
export function headerToBytes(header: Header): Uint8Array {
  const dv = new DataView(new ArrayBuffer(HEADER_SIZE_BYTES));
  dv.setUint16(0, 0x4d50, true);
  dv.setUint8(7, header.specVersion);
  setUint64(dv, 8, header.rootDirectoryOffset);
  setUint64(dv, 16, header.rootDirectoryLength);
  setUint64(dv, 24, header.jsonMetadataOffset);
  setUint64(dv, 32, header.jsonMetadataLength);
  setUint64(dv, 40, header.leafDirectoryOffset);
  setUint64(dv, 48, header.leafDirectoryLength ?? 0);
  setUint64(dv, 56, header.tileDataOffset);
  setUint64(dv, 64, header.tileDataLength ?? 0);
  setUint64(dv, 72, header.numAddressedTiles);
  setUint64(dv, 80, header.numTileEntries);
  setUint64(dv, 88, header.numTileContents);
  dv.setUint8(96, header.clustered ? 1 : 0);
  dv.setUint8(97, header.internalCompression);
  dv.setUint8(98, header.tileCompression);
  dv.setUint8(99, header.tileType);
  dv.setUint8(100, header.minZoom);
  dv.setUint8(101, header.maxZoom);

  return new Uint8Array(dv.buffer, dv.byteOffset, dv.byteLength);
}

/**
 * @param entries - the directory entries
 * @returns - the serialized directory
 */
export function serializeDir(entries: Entry[]): Uint8Array {
  const data = { buf: new Uint8Array(0), pos: 0 };

  writeVarint(entries.length, data);

  let lastID = 0;
  for (let i = 0; i < entries.length; i++) {
    const diff = entries[i].tileID - lastID;
    writeVarint(diff, data);
    lastID = entries[i].tileID;
  }

  for (let i = 0; i < entries.length; i++) writeVarint(entries[i].runLength, data);
  for (let i = 0; i < entries.length; i++) writeVarint(entries[i].length, data);
  for (let i = 0; i < entries.length; i++) {
    if (i > 0 && entries[i].offset == entries[i - 1].offset + entries[i - 1].length) {
      writeVarint(0, data);
    } else {
      writeVarint(entries[i].offset + 1, data);
    }
  }

  return new Uint8Array(data.buf.buffer, data.buf.byteOffset, data.pos);
}

/**
 * Take a large 64-bit number and encode it into a DataView
 * @param dv - a DataView
 * @param offset - the offset in the DataView
 * @param value - the encoded 64-bit number
 */
export function setUint64(dv: DataView, offset: number, value: number): void {
  // dv.setUint32(offset + 4, value % 2 ** 32, true);
  // dv.setUint32(offset, Math.floor(value / 2 ** 32), true);
  const wh = Math.floor(value / 2 ** 32);
  const wl = value >>> 0; // Keep the lower 32 bits as an unsigned 32-bit integer

  dv.setUint32(offset, wl, true); // Set the lower 32 bits
  dv.setUint32(offset + 4, wh, true); // Set the upper 32 bits
}
