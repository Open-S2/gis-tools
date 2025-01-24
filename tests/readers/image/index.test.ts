import { imageDecoderBuffer } from '../../../src';
import { expect, test } from 'bun:test';

/**
 * @param name - the name of the fixture
 * @returns the contents of the fixture as an array buffer
 */
async function fixture(name: string): Promise<ArrayBufferLike> {
  return await Bun.file(`${__dirname}/jpeg/fixtures/${name}`).arrayBuffer();
}

test('imageDecoderBuffer', async () => {
  const buffer = await fixture('rgb.jpg');
  const decoded = await imageDecoderBuffer(buffer);
  expect(decoded).toBeInstanceOf(ArrayBuffer);
});
