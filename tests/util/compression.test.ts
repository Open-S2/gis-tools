import {
  compressStream,
  decompressStream,
  iterZipFolder,
  zipFolder,
} from '../../src/util/compression.js';
import { describe, expect, it, test } from 'bun:test';

import type { WriteZipItem } from '../../src/util/compression.js';

test('iter', async () => {
  const zipFile = new Uint8Array(await Bun.file(`${__dirname}/fixtures/utf.zip`).arrayBuffer());

  const items = [...iterZipFolder(zipFile)];
  expect(items.map((i) => i.filename)).toEqual([
    'utf.cpg',
    'utf.dbf',
    'utf.prj',
    'utf.qpj',
    'utf.shp',
    'utf.shx',
  ]);
  const firstItem = await items[0].read();
  expect(new TextDecoder().decode(firstItem)).toEqual('UTF-8');
});

test('decompressStream', async () => {
  const zipFile = new Uint8Array(
    await Bun.file(`${__dirname}/fixtures/testfile.zip`).arrayBuffer(),
  );

  const items = [...iterZipFolder(zipFile)];
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

describe('ZIP Encoder & Decoder Integration Tests', () => {
  it('should successfully zip and unzip basic text files', async () => {
    // 1. Arrange: Setup some dummy data to compress
    const encoder = new TextEncoder();
    const filesToZip: WriteZipItem[] = [
      {
        name: 'hello.txt',
        comment: 'Greet the world',
        data: encoder.encode('Hello World! This is a simple text payload.'),
      },
      {
        name: 'nested/folder/log.csv',
        comment: 'Some metrics data',
        data: encoder.encode('id,value\n1,100\n2,200\n3,300'),
      },
    ];

    // 2. Act: Encode the files into a single ZIP binary
    const zipBuffer = await zipFolder(filesToZip);

    // Verify we actually generated something substantial
    expect(zipBuffer).toBeInstanceOf(Uint8Array);
    expect(zipBuffer.length).toBeGreaterThan(0);

    // 3. Act: Pass the generated binary straight into your original decoder
    const extractedFiles = [];
    for (const item of iterZipFolder(zipBuffer)) {
      extractedFiles.push({
        filename: item.filename,
        comment: item.comment,
        // Resolve the async read() method from your ZipItem interface
        bytes: await item.read(),
      });
    }

    // 4. Assert: Validate everything came out exactly how it went in
    expect(extractedFiles).toHaveLength(filesToZip.length);

    // File 1 Assertions
    expect(extractedFiles[0].filename).toBe(filesToZip[0].name);
    expect(extractedFiles[0].comment).toBe(filesToZip[0].comment);
    expect(extractedFiles[0].bytes).toEqual(filesToZip[0].data);

    // File 2 Assertions
    expect(extractedFiles[1].filename).toBe(filesToZip[1].name);
    expect(extractedFiles[1].comment).toBe(filesToZip[1].comment);
    expect(extractedFiles[1].bytes).toEqual(filesToZip[1].data);
  });

  it('should handle empty files properly without breaking pointers', async () => {
    const emptyFile: WriteZipItem[] = [
      {
        name: 'empty.txt',
        comment: 'Nothing to see here',
        data: new Uint8Array(0),
      },
    ];

    const zipBuffer = await zipFolder(emptyFile);
    const iterator = iterZipFolder(zipBuffer);

    const result = iterator.next();
    expect(result.done).toBe(false);

    const item = result.value;
    expect(item?.filename).toBe('empty.txt');
    expect(item?.comment).toBe('Nothing to see here');

    const data = await item?.read();
    expect(data?.length).toBe(0);

    expect(iterator.next().done).toBe(true);
  });
});
