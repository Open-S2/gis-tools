import { decompressSync } from '../../../src/util/polyfills/fflate';
import { afterAll, beforeAll, expect, test } from 'bun:test';
import { decompressStream, iterItems } from '../../../src';

import zlib from 'zlib';

// /**
//  * @param name
//  * @param compressed
//  * @param expected
//  * @param dictionary
//  */
// async function storeCompressionFixtures(
//   name: string,
//   compressed: Uint8Array,
//   expected: Uint8Array,
//   dictionary?: Uint8Array,
// ) {
//   await Bun.write(`${__dirname}/../fixtures/${name}_expected.bin`, expected);
//   await Bun.write(`${__dirname}/../fixtures/${name}_compressed.bin`, compressed);
//   if (dictionary !== undefined) {
//     await Bun.write(`${__dirname}/../fixtures/${name}_dictionary.bin`, dictionary);
//   }
// }

// inject polyfill for this test case
beforeAll(() => {
  void import('../../../src/util/polyfills/decompression');
});

// fix the polyfilling when im done
afterAll(() => {
  globalThis.decompressionPolyfill = undefined;
});

test('deflateSync - dictionary', async () => {
  const dictionary = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/spdyDict.txt`).arrayBuffer(),
  );
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateSync(expected, { dictionary });

  const res: Uint8Array = decompressSync(data, dictionary);

  // await storeCompressionFixtures('deflateSync_dictionary', data, expected);

  expect(expected).toEqual(res);
});

test('deflateSync - level: 9', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = new Uint8Array(zlib.deflateSync(expected, { level: 9 }).buffer);

  const res: Uint8Array = decompressSync(data);

  // await storeCompressionFixtures('deflateSync_level_9', data, expected);

  expect(expected).toEqual(res);
});

test('deflateSync - memLevel: 9', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateSync(expected, { memLevel: 9 });

  const res: Uint8Array = decompressSync(data);

  // await storeCompressionFixtures('deflateSync_memLevel_9', data, expected);

  expect(expected).toEqual(res);
});

test('deflateSync - strategy: 0', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateSync(expected, { strategy: 0 });

  const res: Uint8Array = decompressSync(data);

  // await storeCompressionFixtures('deflateSync_strategy_0', data, expected);

  expect(expected).toEqual(res);
});

test('deflateRawSync - windowBits: 15', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateRawSync(expected, { windowBits: 15 });

  const res: Uint8Array = decompressSync(data);

  // await storeCompressionFixtures('deflateRawSync_windowBits_15', data, expected);

  expect(expected).toEqual(res);
});

test('deflateRawSync - level: 0', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateRawSync(expected, { level: 0 });

  const res: Uint8Array = decompressSync(data);

  // await storeCompressionFixtures('deflateRawSync_level_0', data, expected);

  expect(expected).toEqual(res);
});

test('deflateRawSync - Error', async () => {
  const expected: Uint8Array = new Uint8Array(
    await Bun.file(`${__dirname}/../fixtures/lorem_en_100k.txt`).arrayBuffer(),
  );
  const data = zlib.deflateRawSync(expected, { windowBits: 15 });

  expect(() => decompressSync(data.slice(20))).toThrowError('invalid block type');
});

test('dictionary', () => {
  const dict = new Uint8Array(Buffer.from('abcd').buffer); // [97, 98, 99, 100]

  // const deflate = new pako.Deflate({ dictionary: dict });
  // deflate.push(Buffer.from('hello'), false);
  // deflate.push(Buffer.from('hello'), false);
  // deflate.push(Buffer.from(' world'), true);
  // const uncompressed = pako.inflate(Buffer.from(deflateResult), { dictionary: dict });

  const deflateResult = new Uint8Array([
    120, 187, 3, 216, 1, 139, 203, 72, 205, 201, 201, 7, 19, 10, 229, 249, 69, 57, 41, 0, 55, 19, 6,
    113,
  ]);

  const uncompressedInternal: Uint8Array = decompressSync(deflateResult, dict);
  const expected: Uint8Array = new Uint8Array(Buffer.from('hellohello world'));

  expect(expected).toEqual(uncompressedInternal);
});

test('iterItems - polyfill', async () => {
  const zipFile = new Uint8Array(await Bun.file(`${__dirname}/../fixtures/utf.zip`).arrayBuffer());

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

test('gzip - decompressStream (polyfill)', async () => {
  const expected = await Bun.file(`${__dirname}/../fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/../fixtures/expected.txt.gz`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'gzip');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});

test('deflate - decompressStream (polyfill)', async () => {
  const expected = await Bun.file(`${__dirname}/../fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/../fixtures/expected.txt.deflate`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'deflate');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});

test('deflate - raw - decompressStream (polyfill)', async () => {
  const expected = await Bun.file(`${__dirname}/../fixtures/expected.txt`).text();
  const data = await Bun.file(`${__dirname}/../fixtures/expected.txt.deflate-raw`).arrayBuffer();

  const result = await decompressStream(new Uint8Array(data), 'deflate-raw');
  const actual = new TextDecoder().decode(result);
  expect(actual).toEqual(expected);
});
