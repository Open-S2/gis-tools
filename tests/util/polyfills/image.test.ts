import '../../../src/util/polyfills/image';
import { expect, test } from 'bun:test';

import sharp from 'sharp';

const rgbImage = [
  // red
  255, 0, 0, 255,
  // green
  0, 255, 0, 255,
  // blue
  0, 0, 255, 255,
  // white
  255, 255, 255, 255,
];
const pngBuffer = await sharp(Buffer.from(rgbImage), { raw: { width: 2, height: 2, channels: 4 } })
  .png()
  .toBuffer();

test('createImageBitmap', async () => {
  const blob = new Blob([pngBuffer.buffer as ArrayBuffer]); // e.g. { type: 'image/png' }
  const imageBitmap = await createImageBitmap(blob);
  // Create OffscreenCanvas and draw
  const canvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
  const ctx = canvas.getContext('2d');
  if (ctx === null) throw new Error('Could not get 2d context');
  ctx.drawImage(imageBitmap, 0, 0);

  const imageData = ctx.getImageData(0, 0, imageBitmap.width, imageBitmap.height);
  expect(imageData.data).toEqual(new Uint8ClampedArray(rgbImage));

  // just pull out red and green:
  const sliceData = ctx.getImageData(0, 0, 2, 1);
  expect(sliceData.data).toEqual(new Uint8ClampedArray([255, 0, 0, 255, 0, 255, 0, 255]));
});
