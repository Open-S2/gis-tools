import { iterItems } from '../../src/util/gzip';
import { expect, test } from 'bun:test';

test('iter', async () => {
  const zipFile = new Uint8Array(await Bun.file(`${__dirname}/fixtures/utf.zip`).arrayBuffer());

  const items = [...iterItems(zipFile)];
  expect(items.map((i) => i.filename)).toEqual([
    'utf.cpg',
    'utf.dbf',
    'utf.prj',
    'utf.qpj',
    'utf.shp',
    'utf.shx',
  ]);
});

test('decompressStream', async () => {
  const zipFile = new Uint8Array(
    await Bun.file(`${__dirname}/fixtures/testfile.zip`).arrayBuffer(),
  );

  const items = [...iterItems(zipFile)];
  const file = await items[0].read();
  expect(new TextDecoder().decode(file)).toEqual('Hello!\n');
});
