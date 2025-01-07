import { decodeJpegData, jpegDecoder } from '../../../../src';
import { expect, test } from 'bun:test';

const SUPER_LARGE_JPEG_BASE64 = '/9j/wfFRBf//BdgC/9p/2P/E4d4=';
const SUPER_LARGE_RESOLUTION_JPEG_BASE64 = '/9j/wfFR2PDh3g==';
const SUPER_LARGE_JPEG_BUFFER = Buffer.from(SUPER_LARGE_JPEG_BASE64, 'base64');
const SUPER_LARGE_RESOLUTION_JPEG_BUFFER = Buffer.from(
  SUPER_LARGE_RESOLUTION_JPEG_BASE64,
  'base64',
);

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

/**
 * @param name - the name of the fixture
 * @returns the contents of the fixture as an array buffer
 */
async function fixture(name: string): Promise<ArrayBufferLike> {
  return await Bun.file(`${__dirname}/fixtures/${name}`).arrayBuffer();
}

test('reads image with a bad e1 marker not preceeded by ff', async () => {
  const jpegData = await fixture('table-with-bad-e1.jpg');
  const rawImageData = new Uint8Array(jpegDecoder(jpegData));
  const expected = await fixture('table-with-good-e1.jpg');
  const rawExpectedImageData = new Uint8Array(jpegDecoder(expected));
  expect(rawImageData).toEqual(rawExpectedImageData);
});

test('decodes a JPEG', async () => {
  const jpegData = await fixture('grumpycat.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(320);
  expect(rawImageData.height).toEqual(180);
  const expected = new Uint8Array(await fixture('grumpycat.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes a JPEG with fill bytes', async () => {
  const jpegData = await fixture('fillbytes.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(704);
  expect(rawImageData.height).toEqual(576);
});

test('decodes a JPEG with RST intervals', async () => {
  const jpegData = await fixture('redbox-with-rst.jpg');
  const rawImageData = decodeJpegData(jpegData);
  const expected = await fixture('redbox.jpg');
  const rawExpectedImageData = decodeJpegData(expected);
  expect(rawImageData.data).toEqual(rawExpectedImageData.data);
});

test('decodes a JPEG with trailing bytes', async () => {
  const jpegData = await fixture('redbox-with-trailing-bytes.jpg');
  const rawImageData = decodeJpegData(jpegData);
  const expected = await fixture('redbox.jpg');
  const rawExpectedImageData = decodeJpegData(expected);
  expect(rawImageData.data).toEqual(rawExpectedImageData.data);
});

test('decodes a grayscale JPEG', async () => {
  const jpegData = await fixture('apsara.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(580);
  expect(rawImageData.height).toEqual(599);
  expect(rawImageData.comments).toEqual([
    'File source: http://commons.wikimedia.org/wiki/File:Apsara-mit-Sitar.jpg',
  ]);
  const expected = new Uint8Array(await fixture('apsara.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

testFunc('decodes a 32-bit TrueColor RGB image', async () => {
  const jpegData = await fixture('truecolor.jpg');
  const rawImageData = decodeJpegData(jpegData, { colorTransform: false });
  expect(rawImageData.width).toEqual(1280);
  expect(rawImageData.height).toEqual(2000);
  const expected = new Uint8Array(await fixture('truecolor.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes a CMYK jpeg with correct colors', async () => {
  const jpegData = await fixture('tree-cmyk.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(400);
  expect(rawImageData.height).toEqual(250);
  const expected = new Uint8Array(await fixture('tree-cmyk.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes a CMYK jpeg with correct colors without transform', async () => {
  const jpegData = await fixture('tree-cmyk-notransform.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(400);
  expect(rawImageData.height).toEqual(250);
  const expected = new Uint8Array(await fixture('tree-cmyk-notransform.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes an RGB jpeg with correct colors', async () => {
  const jpegData = await fixture('tree-rgb.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(400);
  expect(rawImageData.height).toEqual(250);
  const expected = new Uint8Array(await fixture('tree-rgb.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes an progressive RGB jpeg with correct colors', async () => {
  const jpegData = await fixture('rgb.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(350);
  expect(rawImageData.height).toEqual(262);
  const expected = new Uint8Array(await fixture('rgb.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes a greyscale CMYK jpeg with correct colors', async () => {
  const jpegData = await fixture('cmyk-grey.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(300);
  expect(rawImageData.height).toEqual(389);
  const expected = new Uint8Array(await fixture('cmyk-grey.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes an adobe CMYK jpeg with correct colors', async () => {
  const jpegData = await fixture('cmyktest.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(300);
  expect(rawImageData.height).toEqual(111);
  const expected = new Uint8Array(await fixture('cmyktest.rgba'));
  expect(rawImageData.data).toEqual(expected);

  const jpegData2 = await fixture('plusshelf-drawing.jpg');
  const rawImageData2 = decodeJpegData(jpegData2);
  expect(rawImageData2.width).toEqual(350);
  expect(rawImageData2.height).toEqual(233);
  const expected2 = new Uint8Array(await fixture('plusshelf-drawing.rgba'));
  expect(rawImageData2.data).toEqual(expected2);
});

testFunc('decodes a unconventional table JPEG', async () => {
  const jpegData = await fixture('unconventional-table.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(1920);
  expect(rawImageData.height).toEqual(1200);
});

test('decodes a progressive JPEG', async () => {
  const jpegData = await fixture('skater-progressive.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(256);
  expect(rawImageData.height).toEqual(256);
  const expected = new Uint8Array(await fixture('skater-progressive.rgba'));
  expect(rawImageData.data).toEqual(expected);
});

test('decodes a progressive JPEG the same as non-progressive', async () => {
  const jpegData = await fixture('skater.jpg');
  const rawImageData = decodeJpegData(jpegData);

  const otherJpegData = await fixture('skater-progressive.jpg');
  const otherRawImageData = decodeJpegData(otherJpegData);

  expect(rawImageData.width).toEqual(otherRawImageData.width);
  expect(rawImageData.height).toEqual(otherRawImageData.height);
  expect(rawImageData.data).toEqual(otherRawImageData.data);
});

test('decodes a JPEG into a typed array', async () => {
  const jpegData = await fixture('grumpycat.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(320);
  expect(rawImageData.height).toEqual(180);
  const expected = await fixture('grumpycat.rgba');
  expect(rawImageData.data).toEqual(new Uint8Array(expected));
  expect(rawImageData.data instanceof Uint8Array, 'data is a typed array').toBeTruthy();
});

test('decodes a JPEG from a typed array into a typed array', async () => {
  const jpegData = await fixture('grumpycat.jpg');
  const rawImageData = decodeJpegData(jpegData);
  expect(rawImageData.width).toEqual(320);
  expect(rawImageData.height).toEqual(180);
  const expected = await fixture('grumpycat.rgba');
  expect(rawImageData.data).toEqual(new Uint8Array(expected));
  expect(rawImageData.data instanceof Uint8Array, 'data is a typed array').toBeTruthy();
});

test('decodes a JPEG with options', async () => {
  const jpegData = await fixture('grumpycat.jpg');
  const rawImageData = decodeJpegData(jpegData, { colorTransform: false });
  expect(rawImageData.width).toEqual(320);
  expect(rawImageData.height).toEqual(180);
  const expected = await fixture('grumpycat-nocolortrans.rgba');
  expect(rawImageData.data).toEqual(new Uint8Array(expected));
  expect(rawImageData.data instanceof Uint8Array, 'data is a typed array').toBeTruthy();
});

test('decodes a JPEG into RGB', async () => {
  const jpegData = await fixture('grumpycat.jpg');
  const rawImageData = decodeJpegData(jpegData, { formatAsRGBA: false });
  expect(rawImageData.width).toEqual(320);
  expect(rawImageData.height).toEqual(180);
  const expected = await fixture('grumpycat.rgb');
  expect(rawImageData.data).toEqual(new Uint8Array(expected));
  expect(rawImageData.data instanceof Uint8Array, 'data is a typed array').toBeTruthy();
});

test('decodes image with ffdc marker', async () => {
  const jpegData = await fixture('marker-ffdc.jpg');
  const imageData = decodeJpegData(jpegData);
  expect(imageData.height).toEqual(200);
  expect(imageData.width).toEqual(200);
});

testFunc(
  'decodes large images within memory limits',
  async () => {
    const jpegData = await fixture('black-6000x6000.jpg');
    const rawImageData = decodeJpegData(jpegData);
    expect(rawImageData.width).toEqual(6000);
    expect(rawImageData.height).toEqual(6000);
  },
  30000,
);

// See https://github.com/eugeneware/jpeg-js/issues/53
test('limits resolution exposure', () => {
  expect(() => decodeJpegData(SUPER_LARGE_RESOLUTION_JPEG_BUFFER.buffer)).toThrow(
    'maxResolutionInMP limit exceeded by 3405MP',
  );
});

test('limits memory exposure', async () => {
  expect(() => decodeJpegData(SUPER_LARGE_JPEG_BUFFER.buffer, { maxResolutionInMP: 500 })).toThrow(
    /maxMemoryUsageInMB limit exceeded by at least \d+MB/,
  );

  // Make sure the limit resets each decode.
  const jpegData = await fixture('grumpycat.jpg');
  expect(() => decodeJpegData(jpegData)).not.toThrow();
}, 30000);

// See https://github.com/jpeg-js/jpeg-js/issues/105
test('errors out invalid sampling factors', () => {
  expect(() => decodeJpegData(Buffer.from('/9j/wfFR2AD/UdgA/9r/3g==', 'base64').buffer)).toThrow(
    'marker was not found',
  );
});
