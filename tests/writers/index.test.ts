import { BufferTileWriter } from '../../src';
import { expect, test } from 'bun:test';

test('writers - BufferTileWriter', async () => {
  const writer = new BufferTileWriter();
  expect(writer).toBeInstanceOf(BufferTileWriter);

  const date1 = new Date('2025-01-01T12:00:00Z');
  const date1Str = date1.getTime();
  const data1 = new Uint8Array([0, 1, 2, 3]);
  const date2 = new Date('2020-05-11T12:00:00Z');
  const date2Str = date2.getTime();
  const data2 = new Uint8Array([4, 3, 2, 1]);

  await writer.writeTemporalTileS2(date1, 0, 0, 0, 0, data1);
  await writer.writeTemporalTileWM(date2, 0, 0, 0, data2);

  const s2Tile = writer.tiles.get(`${date1Str}/0/0/0/0`);
  expect(s2Tile).toEqual(data1);

  const wmTile = writer.tiles.get(`${date2Str}/0/0/0`);
  expect(wmTile).toEqual(data2);
});
