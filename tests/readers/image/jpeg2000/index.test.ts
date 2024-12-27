import { FileReader } from '../../../../src/file';
import { JpxImage } from '../../../../src';
import { expect, test } from 'bun:test';

test('Basic JPEG2000 case', async () => {
  const reader = new FileReader(`${__dirname}/fixtures/input.j2k`);
  const jpxImage = new JpxImage(reader);
  expect(jpxImage.width).toEqual(63677);
  expect(jpxImage.height).toEqual(1);
  expect(jpxImage.componentsCount).toEqual(1);
  expect(jpxImage.tiles.length).toEqual(1);

  const tile = jpxImage.tiles[0];
  const expected = await Bun.file(`${__dirname}/fixtures/expected.raw`).arrayBuffer();
  expect(tile).toEqual({
    height: 1,
    width: 63677,
    top: 0,
    left: 0,
    items: new Uint8ClampedArray(expected),
  });
});
