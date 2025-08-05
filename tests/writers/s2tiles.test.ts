import { stat } from 'node:fs/promises';
import tmp from 'tmp';
import { BufferReader, BufferWriter, Compression, S2TilesReader, S2TilesWriter } from '../../src';
import { FileReader, FileWriter } from '../../src/file';
import { expect, test } from 'bun:test';

import type { Metadata } from 's2-tilejson';

tmp.setGracefulCleanup();

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

test('S2Tiles - Buffer Writer - WM', async () => {
  const bufWriter = new BufferWriter();
  const writer = new S2TilesWriter(bufWriter, 9, Compression.None);
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
  await writer.writeTileWM(9, 22, 9, uint8_2);
  // finish
  await writer.commit({ metadata: true } as unknown as Metadata);

  const bufData = bufWriter.commit();
  const bufReader = new BufferReader(bufData.buffer);
  const reader = new S2TilesReader(bufReader);
  const metadata = await reader.getMetadata();
  expect(bufReader.buffer.byteLength).toEqual(216_417);
  expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

  expect(await reader.hasTileWM(0, 0, 0)).toBeTrue();
  const tile = await reader.getTileWM(0, 0, 0);
  expect(tile).toEqual(uint8);

  const tile2 = await reader.getTileWM(1, 0, 1);
  expect(tile2).toEqual(uint8);

  const tile3 = await reader.getTileWM(9, 22, 9);
  expect(tile3).toEqual(uint8_2);

  expect(await reader.hasTileWM(1, 1, 1)).toBeFalse();
});

test('S2Tiles - File Writer - S2', async () => {
  const tmpFile1 = tmp.tmpNameSync({ prefix: 'S2' });
  const fileWriter = new FileWriter(tmpFile1);
  const writer = new S2TilesWriter(fileWriter, 8, Compression.None);
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
  fileWriter.close();

  const reader = new S2TilesReader(new FileReader(tmpFile1));
  const metadata = await reader.getMetadata();

  expect((await stat(tmpFile1)).size).toEqual(215_594);
  expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

  expect(await reader.hasTileS2(0, 0, 0, 0)).toBeTrue();
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

  expect(await reader.hasTileS2(1, 1, 1, 1)).toBeFalse();
});

testFunc(
  'S2Tiles - File Writer - WM Large',
  async () => {
    const tmpFile2 = tmp.tmpNameSync({ prefix: 'S2-big-2' });
    const writer = new S2TilesWriter(new FileWriter(tmpFile2), 8, Compression.None);
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

    const reader = new S2TilesReader(new FileReader(tmpFile2));
    const metadata = await reader.getMetadata();
    expect(metadata).toEqual({ metadata: true } as unknown as Metadata);

    // get a random tile
    const tile = await reader.getTileWM(6, 22, 45);
    const str = `6-22-45`;
    const buf = Buffer.from(str, 'utf8');
    const uint8 = new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
    expect(tile).toEqual(uint8);

    // get another random tile
    const tile2 = await reader.getTileWM(5, 12, 30);
    const str2 = `5-12-30`;
    const buf2 = Buffer.from(str2, 'utf8');
    const uint8_2 = new Uint8Array(buf2.buffer, buf2.byteOffset, buf2.byteLength);
    expect(tile2).toEqual(uint8_2);
  },
  { timeout: 10_000 },
);
