import { compressStream, decompressStream, iterItems } from '../../src/util/gzip';
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

test('compressStream', async () => {
  const data = new TextEncoder().encode('Hello!\n');
  const compressed = await compressStream(data);
  expect(compressed).toEqual(
    new Uint8Array([
      31, 139, 8, 0, 0, 0, 0, 0, 0, 19, 243, 72, 205, 201, 201, 87, 228, 2, 0, 158, 216, 66, 176, 7,
      0, 0, 0,
    ]),
  );
  const decompressed = await decompressStream(compressed);
  expect(new TextDecoder().decode(decompressed)).toEqual('Hello!\n');
});
