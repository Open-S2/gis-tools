import { TileFileWriter } from '../../src/file';
import { expect, test } from 'bun:test';

import tmp from 'tmp';
tmp.setGracefulCleanup();

test('writers - Tile File Writer', async () => {
  const dir = tmp.dirSync({ prefix: 'file_test' });
  const textEncoder = new TextEncoder();
  const writer = new TileFileWriter(`${dir.name}`, 'png');

  const testDate = new Date('2025-01-01T12:00:00Z');
  const testDateStr = testDate.toISOString();
  const testDate2 = new Date('2020-05-11T12:00:00Z');
  const testDateStr2 = testDate2.toISOString();

  // writes
  await writer.writeTileWM(0, 0, 0, textEncoder.encode('test'));
  await writer.writeTileS2(1, 1, 1, 1, textEncoder.encode('test2'));
  await writer.writeTemporalTileWM(testDate, 5, 3, 7, textEncoder.encode('test3'));
  await writer.writeTemporalTileS2(testDate2, 4, 10, 10001, 22, textEncoder.encode('test4'));
  // @ts-expect-error - invalid interface its ok
  await writer.commit({ test: 'a', test2: 2 });

  const wmTile = await Bun.file(`${dir.name}/0/0/0.png`).text();
  expect(wmTile).toEqual('test');
  const s2Tile = await Bun.file(`${dir.name}/1/1/1/1.png`).text();
  expect(s2Tile).toEqual('test2');
  const wmTimeTile = await Bun.file(`${dir.name}/${testDateStr}/5/3/7.png`).text();
  expect(wmTimeTile).toEqual('test3');
  const s2TimeTile = await Bun.file(`${dir.name}/${testDateStr2}/4/10/10001/22.png`).text();
  expect(s2TimeTile).toEqual('test4');
  const metadata = await Bun.file(`${dir.name}/metadata.json`).json();
  expect(metadata).toEqual({ test: 'a', test2: 2 });
});
