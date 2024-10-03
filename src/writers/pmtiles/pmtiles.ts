import { HEADER_SIZE_BYTES } from '../../readers/pmtiles';
import { writeVarint } from './varint';

import type { Entry, Header } from '../../readers/pmtiles';

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
    if (i > 0 && entries[i].offset === entries[i - 1].offset + entries[i - 1].length) {
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
