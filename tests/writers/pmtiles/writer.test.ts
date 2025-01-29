import tmp from 'tmp';
import {
  BufferReader,
  BufferWriter,
  S2PMTilesReader,
  S2PMTilesWriter,
  TileType,
} from '../../../src';
import { FileReader, FileWriter } from '../../../src/file';
import { expect, test } from 'bun:test';

import { stat } from 'node:fs/promises';

import type { Metadata } from 's2-tilejson';
import type { S2Header } from '../../../src/readers/pmtiles';

tmp.setGracefulCleanup();

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

test('File Writer WM', async () => {
  const bufWriter = new BufferWriter();
  const writer = new S2PMTilesWriter(bufWriter, TileType.Pbf);
  // setup data
  const str = 'hello world';
  const buf = Buffer.from(str, 'utf8');
  const uint8 = new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
  const str2 = 'hello world 2';
  const buf2 = Buffer.from(str2, 'utf8');
  const uint8_2 = new Uint8Array(buf2.buffer, buf2.byteOffset, buf2.byteLength);
  // write data in tile
  await writer.writeTileWM(0, 0, 0, uint8);
  await writer.writeTileWM(1, 0, 1, uint8);
  await writer.writeTileWM(5, 2, 9, uint8_2);
  // finish
  await writer.commit({ metadata: true } as unknown as Metadata);

  const bufReader = new BufferReader(bufWriter.commit().buffer);
  const reader = new S2PMTilesReader(bufReader);
  const metadata = await reader.getMetadata();
  const header = await reader.getHeader();
  expect(bufReader.buffer.byteLength).toEqual(98_339);
  expect(header).toEqual({
    clustered: true,
    internalCompression: 2,
    jsonMetadataLength: 37,
    jsonMetadataOffset: 296,
    leafDirectoryLength: 0,
    leafDirectoryOffset: 98_339,
    maxZoom: 5,
    minZoom: 0,
    numAddressedTiles: 3,
    numTileContents: 3,
    numTileEntries: 3,
    rootDirectoryLength: 34,
    rootDirectoryOffset: 262,
    specVersion: 3,
    tileCompression: 1,
    tileDataLength: 35,
    tileDataOffset: 98_304,
    tileType: 1,
  });
  expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

  const tile = await reader.getTile(0, 0, 0);
  expect(tile).toEqual(uint8);

  const tile2 = await reader.getTile(1, 0, 1);
  expect(tile2).toEqual(uint8);

  const tile3 = await reader.getTile(5, 2, 9);
  expect(tile3).toEqual(uint8_2);
});

test('File Writer S2', async () => {
  const tmpFile1 = tmp.tmpNameSync({ prefix: 'S2' });
  const writer = new S2PMTilesWriter(new FileWriter(tmpFile1), TileType.Pbf);
  // setup data
  const txtEncoder = new TextEncoder();
  const str = 'hello world';
  const uint8 = txtEncoder.encode(str);
  const str2 = 'hello world 2';
  const uint8_2 = txtEncoder.encode(str2);
  // write data in tile
  await writer.writeTileS2(0, 0, 0, 0, uint8);
  await writer.writeTileS2(1, 0, 0, 0, uint8);
  await writer.writeTileS2(2, 8, 1, 1, uint8_2);
  await writer.writeTileS2(3, 2, 1, 1, uint8_2);
  await writer.writeTileS2(4, 5, 5, 5, uint8_2);
  await writer.writeTileS2(5, 5, 5, 5, uint8);
  // finish
  await writer.commit({ metadata: true } as unknown as Metadata);

  const reader = new S2PMTilesReader(new FileReader(tmpFile1));
  const metadata = await reader.getMetadata();
  const header = await reader.getHeader();

  expect((await stat(tmpFile1)).size).toEqual(98_376);
  expect(header).toEqual({
    clustered: true,
    internalCompression: 2,
    jsonMetadataLength: 37,
    jsonMetadataOffset: 416,
    leafDirectoryLength: 0,
    leafDirectoryLength1: 0,
    leafDirectoryLength2: 0,
    leafDirectoryLength3: 0,
    leafDirectoryLength4: 0,
    leafDirectoryLength5: 0,
    leafDirectoryOffset: 98_376,
    leafDirectoryOffset1: 98_376,
    leafDirectoryOffset2: 98_376,
    leafDirectoryOffset3: 98_376,
    leafDirectoryOffset4: 98_376,
    leafDirectoryOffset5: 98_376,
    maxZoom: 8,
    minZoom: 0,
    numAddressedTiles: 6,
    numTileContents: 1,
    numTileEntries: 1,
    rootDirectoryLength: 25,
    rootDirectoryLength1: 25,
    rootDirectoryLength2: 27,
    rootDirectoryLength3: 25,
    rootDirectoryLength4: 26,
    rootDirectoryLength5: 26,
    rootDirectoryOffset: 262,
    rootDirectoryOffset1: 287,
    rootDirectoryOffset2: 312,
    rootDirectoryOffset3: 339,
    rootDirectoryOffset4: 364,
    rootDirectoryOffset5: 390,
    specVersion: 1,
    tileCompression: 1,
    tileDataLength: 72,
    tileDataOffset: 98304,
    tileType: 1,
  } as S2Header);
  expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

  const tile = await reader.getTileS2(0, 0, 0, 0);
  expect(tile).toEqual(uint8);

  const tile2 = await reader.getTileS2(1, 0, 0, 0);
  expect(tile2).toEqual(uint8);

  const tile3 = await reader.getTileS2(3, 2, 1, 1);
  expect(tile3).toEqual(uint8_2);

  const tile4 = await reader.getTileS2(4, 5, 5, 5);
  expect(tile4).toEqual(uint8_2);

  const tile5 = await reader.getTileS2(5, 5, 5, 5);
  expect(tile5).toEqual(uint8);

  const tile6 = await reader.getTileS2(2, 8, 1, 1);
  expect(tile6).toEqual(uint8_2);
});

testFunc(
  'File Writer WM Large',
  async () => {
    const tmpFile2 = tmp.tmpNameSync({ prefix: 'S2-big-2' });
    const writer = new S2PMTilesWriter(new FileWriter(tmpFile2), TileType.Pbf);
    // write lots of tiles
    for (let zoom = 0; zoom < 8; zoom++) {
      const size = 1 << zoom;
      for (let x = 0; x < size; x++) {
        for (let y = 0; y < size; y++) {
          const str = `${zoom}-${x}-${y}`;
          const buf = Buffer.from(str, 'utf8');
          const uint8 = new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
          await writer.writeTileWM(zoom, x, y, uint8);
        }
      }
    }
    // finish
    await writer.commit({ metadata: true } as unknown as Metadata);

    const reader = new S2PMTilesReader(new FileReader(tmpFile2));
    // const header = await reader.getHeader();
    // expect((await stat(tmpFile2)).size).toEqual(736_752);
    // expect(header).toEqual({
    //   clustered: false,
    //   internalCompression: 2,
    //   jsonMetadataLength: 37,
    //   jsonMetadataOffset: 305,
    //   leafDirectoryLength: 46_519,
    //   leafDirectoryOffset: 690_233,
    //   maxZoom: 7,
    //   minZoom: 0,
    //   numAddressedTiles: 21845,
    //   numTileContents: 21_845,
    //   numTileEntries: 21_845,
    //   rootDirectoryLength: 43,
    //   rootDirectoryOffset: 262,
    //   specVersion: 3,
    //   tileCompression: 2,
    //   tileDataLength: 638_448,
    //   tileDataOffset: 98_304,
    //   tileType: 1,
    // });
    const metadata = await reader.getMetadata();
    expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

    // get a random tile
    const tile = await reader.getTile(6, 22, 45);
    const str = `6-22-45`;
    const buf = Buffer.from(str, 'utf8');
    const uint8 = new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
    expect(tile).toEqual(uint8);
  },
  { timeout: 10_000 },
);
