/** Image Decoder Options */
export interface DecodeOptions {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  /** The modulo to use. For example you may have a 514x514 image, but you want to use 512x512 [Default=1] */
  modulo?: number;
}

/**
 * Image decoder
 * @param buffer - the input buffer
 * @param options - user defined options
 * @returns - the decoded buffer
 */
export async function imageDecoder(
  buffer: ArrayBufferLike | Uint8Array | Buffer,
  options?: DecodeOptions,
): Promise<ImageData> {
  const blob = new Blob([buffer as ArrayBuffer]); // e.g. { type: 'image/png' }
  const imageBitmap = await createImageBitmap(blob);
  let { x = 0, y = 0, width = imageBitmap.width, height = imageBitmap.height } = options ?? {};
  const { modulo = 1 } = options ?? {};
  // If modulo exists, shift x and y by modulo's diff
  const diff = imageBitmap.width % modulo;
  if (diff !== 0) {
    x = y = Math.min(diff / 2);
    width = height = imageBitmap.width - diff;
  }
  // Create OffscreenCanvas and draw
  const canvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
  const ctx = canvas.getContext('2d');
  if (ctx === null) throw new Error('Could not get 2d context');
  ctx.drawImage(imageBitmap, 0, 0);

  return ctx.getImageData(x, y, width, height);
}

/**
 * Image decoder
 * @param buffer - the input buffer
 * @param options - user defined options
 * @returns - the decoded buffer
 */
export async function imageDecoderBuffer(
  buffer: ArrayBufferLike | Uint8Array | Buffer,
  options?: DecodeOptions,
): Promise<ArrayBufferLike> {
  const imageData = await imageDecoder(buffer, options);
  return imageData.data.buffer;
}
