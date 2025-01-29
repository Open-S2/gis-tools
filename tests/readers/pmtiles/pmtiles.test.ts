import {
  Compression,
  HEADER_SIZE_BYTES,
  ROOT_SIZE,
  TileType,
  bytesToHeader,
  deserializeDir,
  findTile,
  getUint64,
  tileIDToZxy,
  zxyToTileID,
} from '../../..';
import { headerToBytes, serializeDir, setUint64 } from '../../../src/writers/pmtiles';

import { describe, expect, test } from 'bun:test';

import type { Entry, Header } from '../../..';

test('HEADER_SIZE_BYTES', () => {
  expect(HEADER_SIZE_BYTES).toBe(127);
});

test('ROOT_SIZE', () => {
  expect(ROOT_SIZE).toBe(16_384);
});

describe('zxyToTileID & tileIDToZxy', () => {
  test('zxyToTileID', () => {
    expect(zxyToTileID(0, 0, 0)).toBe(0);
    expect(zxyToTileID(1, 0, 1)).toBe(2);
    expect(zxyToTileID(1, 1, 0)).toBe(4);
    expect(zxyToTileID(1, 1, 1)).toBe(3);
    expect(zxyToTileID(2, 0, 0)).toBe(5);
    expect(zxyToTileID(2, 0, 1)).toBe(8);
    expect(zxyToTileID(2, 1, 0)).toBe(6);
    expect(zxyToTileID(2, 1, 1)).toBe(7);
    expect(zxyToTileID(20, 1_002, 6_969)).toBe(366567509724);
    expect(() => zxyToTileID(30, 0, 0)).toThrowError(
      'Tile zoom level exceeds max safe number limit (26)',
    );
    expect(() => zxyToTileID(0, 10, 0)).toThrowError('tile x/y outside zoom level bounds');
  });

  test('tileIDToZxy', () => {
    expect(tileIDToZxy(0)).toEqual([0, 0, 0]);
    expect(tileIDToZxy(2)).toEqual([1, 0, 1]);
    expect(tileIDToZxy(4)).toEqual([1, 1, 0]);
    expect(tileIDToZxy(3)).toEqual([1, 1, 1]);
    expect(tileIDToZxy(5)).toEqual([2, 0, 0]);
    expect(tileIDToZxy(8)).toEqual([2, 0, 1]);
    expect(tileIDToZxy(6)).toEqual([2, 1, 0]);
    expect(tileIDToZxy(7)).toEqual([2, 1, 1]);
    expect(tileIDToZxy(366567509724)).toEqual([20, 1_002, 6_969]);
    expect(() => tileIDToZxy(8501199875890165)).toThrowError(
      'Tile zoom level exceeds max safe number limit (26)',
    );
  });
});

// bytesToHeader,
//   headerToBytes,

test('bytesToHeader & headerToBytes', () => {
  const header: Header = {
    specVersion: 1,
    rootDirectoryOffset: 20,
    rootDirectoryLength: 634,
    jsonMetadataOffset: 720,
    jsonMetadataLength: 7,
    leafDirectoryOffset: 6,
    leafDirectoryLength: 100,
    tileDataOffset: 5,
    tileDataLength: 4,
    numAddressedTiles: 3,
    numTileEntries: 2,
    numTileContents: 1,
    clustered: true,
    internalCompression: Compression.None,
    tileCompression: Compression.Zstd,
    tileType: TileType.Pbf,
    minZoom: 1,
    maxZoom: 10,
  };
  const bytes = headerToBytes(header);
  expect(bytes).toEqual(
    new Uint8Array([
      80, 77, 0, 0, 0, 0, 0, 1, 20, 0, 0, 0, 0, 0, 0, 0, 122, 2, 0, 0, 0, 0, 0, 0, 208, 2, 0, 0, 0,
      0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
      0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 1, 1, 4, 1, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0,
    ]),
  );
  const header2 = bytesToHeader(bytes);
  expect(header2).toEqual(header);
});

test('getUint64 and setUint64', () => {
  const dv = new DataView(new ArrayBuffer(8));
  setUint64(dv, 0, 1234567890);
  expect(getUint64(dv, 0)).toBe(1234567890);
});

test('serializeDir, deserializeDir, & findTile', () => {
  const entries: Entry[] = [];
  for (let i = 0; i < 10; i++) {
    entries.push({
      tileID: i,
      offset: i * 2,
      length: 100,
      runLength: 1,
    });
  }
  const bytes = serializeDir(entries);
  const entries2 = deserializeDir(bytes);
  expect(entries2).toEqual(entries);

  // findTile
  let tile = findTile(entries, 0);
  expect(tile).toEqual({ tileID: 0, offset: 0, length: 100, runLength: 1 });
  tile = findTile(entries, 1);
  expect(tile).toEqual({ tileID: 1, offset: 2, length: 100, runLength: 1 });
  tile = findTile(entries, 2);
  expect(tile).toEqual({ tileID: 2, offset: 4, length: 100, runLength: 1 });

  expect(findTile(entries, 11)).toBeNull();
});

// let tile = Tile { x: 1_002, y: 6_969, zoom: 20 };
//         let id = tile.to_id();
