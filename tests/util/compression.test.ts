import { compressStream, decompressStream, iterItems } from '../../src/util/compression';
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
  const decompressed = await decompressStream(compressed);
  expect(new TextDecoder().decode(decompressed)).toEqual('Hello!\n');
});

test('gzip - decompressStream', async () => {
  const expected = await Bun.file(`${__dirname}/fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/fixtures/expected.txt.gz`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'gzip');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});

test('deflate - decompressStream', async () => {
  const expected = await Bun.file(`${__dirname}/fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/fixtures/expected.txt.deflate`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'deflate');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});

test('deflate - raw - decompressStream', async () => {
  const expected = await Bun.file(`${__dirname}/fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/fixtures/expected.txt.deflate-raw`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'deflate-raw');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});
