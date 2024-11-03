import { createImage, lanczos } from '../../../src';
import { expect, test } from 'bun:test';

import sharp from 'sharp';

const patternPng = await sharp(`${__dirname}/fixtures/pattern.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const patternBorderPng = await sharp(`${__dirname}/fixtures/pattern-border.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternHalfPng = await sharp(`${__dirname}/fixtures/pattern-half.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternHalf2Png = await sharp(`${__dirname}/fixtures/pattern-half-2.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternDoublePng = await sharp(`${__dirname}/fixtures/pattern-double.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternHalfRegionPng = await sharp(`${__dirname}/fixtures/pattern-half-region.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternHalfRegion2Png = await sharp(`${__dirname}/fixtures/pattern-half-region-2.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternDoubleRegionPng = await sharp(`${__dirname}/fixtures/pattern-double-region.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });
const expectPatternOutOfBoundsPng = await sharp(`${__dirname}/fixtures/pattern-out-of-bounds.png`)
  .raw()
  .toBuffer({ resolveWithObject: true });

const pattern = {
  data: new Uint8ClampedArray(patternPng.data.buffer),
  width: patternPng.info.width,
  height: patternPng.info.height,
} as ImageData;
const patternBorder = {
  data: new Uint8ClampedArray(patternBorderPng.data.buffer),
  width: patternBorderPng.info.width,
  height: patternBorderPng.info.height,
} as ImageData;
const expectPatternHalf = {
  data: new Uint8ClampedArray(expectPatternHalfPng.data.buffer),
  width: expectPatternHalfPng.info.width,
  height: expectPatternHalfPng.info.height,
} as ImageData;
const expectPatternHalf2 = {
  data: new Uint8ClampedArray(expectPatternHalf2Png.data.buffer),
  width: expectPatternHalf2Png.info.width,
  height: expectPatternHalf2Png.info.height,
} as ImageData;
const expectPatternDouble = {
  data: new Uint8ClampedArray(expectPatternDoublePng.data.buffer),
  width: expectPatternDoublePng.info.width,
  height: expectPatternDoublePng.info.height,
} as ImageData;
const expectPatternHalfRegion = {
  data: new Uint8ClampedArray(expectPatternHalfRegionPng.data.buffer),
  width: expectPatternHalfRegionPng.info.width,
  height: expectPatternHalfRegionPng.info.height,
} as ImageData;
const expectPatternHalfRegion2 = {
  data: new Uint8ClampedArray(expectPatternHalfRegion2Png.data.buffer),
  width: expectPatternHalfRegion2Png.info.width,
  height: expectPatternHalfRegion2Png.info.height,
} as ImageData;
const expectPatternDoubleRegion = {
  data: new Uint8ClampedArray(expectPatternDoubleRegionPng.data.buffer),
  width: expectPatternDoubleRegionPng.info.width,
  height: expectPatternDoubleRegionPng.info.height,
} as ImageData;
const expectPatternOutOfBounds = {
  data: new Uint8ClampedArray(expectPatternOutOfBoundsPng.data.buffer),
  width: expectPatternOutOfBoundsPng.info.width,
  height: expectPatternOutOfBoundsPng.info.height,
} as ImageData;

test('lanczos - resizes down', () => {
  const patternHalf = createImage(4, 4);

  lanczos(pattern, patternHalf);

  expect(patternHalf).toEqual(expectPatternHalf);
});

test('resizes down with lanczos2', () => {
  const patternHalf = createImage(4, 4);

  lanczos(
    pattern,
    patternHalf,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    true,
  );

  expect(patternHalf).toEqual(expectPatternHalf2);
});

test('resizes up', () => {
  const patternDouble = createImage(16, 16);

  lanczos(pattern, patternDouble);

  expect(patternDouble).toEqual(expectPatternDouble);
});

test('resizes region down', () => {
  const patternHalfRegion = createImage(6, 6);

  lanczos(patternBorder, patternHalfRegion, 2, 2, 8, 8, 1, 1, 4, 4);

  expect(patternHalfRegion).toEqual(expectPatternHalfRegion);
});

test('resizes region down with lanczos2', () => {
  const patternHalfRegion = createImage(6, 6);

  lanczos(patternBorder, patternHalfRegion, 2, 2, 8, 8, 1, 1, 4, 4, true);

  expect(patternHalfRegion).toEqual(expectPatternHalfRegion2);
});

test('resizes region up', () => {
  const patternDoubleRegion = createImage(18, 18);

  lanczos(patternBorder, patternDoubleRegion, 2, 2, 8, 8, 1, 1, 16, 16);

  expect(patternDoubleRegion).toEqual(expectPatternDoubleRegion);
});

test('early return when any dimension is 0', () => {
  const empty = createImage(8, 8);
  const destSw = createImage(8, 8);
  const destSh = createImage(8, 8);
  const destDw = createImage(8, 8);
  const destDh = createImage(8, 8);

  lanczos(destSw, pattern, 0, 0, 0, 8);
  lanczos(destSh, pattern, 0, 0, 8, 0);
  lanczos(destDw, pattern, 0, 0, 8, 8, 0, 0, 0, 8);
  lanczos(destDh, pattern, 0, 0, 8, 8, 0, 0, 8, 0);

  expect(destSw).toEqual(empty);
  expect(destSh).toEqual(empty);
  expect(destDw).toEqual(empty);
  expect(destDh).toEqual(empty);

  lanczos(destSw, pattern, 0, 0, 0, 8, undefined, undefined, undefined, undefined, true);
  lanczos(destSh, pattern, 0, 0, 8, 0, undefined, undefined, undefined, undefined, true);
  lanczos(destDw, pattern, 0, 0, 8, 8, 0, 0, 0, 8, true);
  lanczos(destDh, pattern, 0, 0, 8, 8, 0, 0, 8, 0, true);

  expect(destSw).toEqual(empty);
  expect(destSh).toEqual(empty);
  expect(destDw).toEqual(empty);
  expect(destDh).toEqual(empty);
});

test('does not sample outside bounds', () => {
  const patternOutOfBounds = createImage(8, 8);

  lanczos(pattern, patternOutOfBounds, 0, 0, 16, 16, 0, 0, 32, 32);

  expect(patternOutOfBounds).toEqual(expectPatternOutOfBounds);
});

test('createImage errors', () => {
  expect(() => {
    createImage(0, 0);
  }).toThrow('Index or size is negative or greater than the allowed amount');

  expect(() => {
    createImage(10, 10, new Uint8ClampedArray(0));
  }).toThrow('Index or size is negative or greater than the allowed amount');
});
