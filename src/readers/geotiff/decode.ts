import { applyPredictor } from './predictor';

/**
 * @param fileDirectory
 * @param buffer
 */
export async function decode(fileDirectory, buffer) {
  const decoded = await this.decodeBlock(buffer);
  const predictor = fileDirectory.Predictor || 1;
  if (predictor !== 1) {
    const isTiled = !fileDirectory.StripOffsets;
    const tileWidth = isTiled ? fileDirectory.TileWidth : fileDirectory.ImageWidth;
    const tileHeight = isTiled
      ? fileDirectory.TileLength
      : fileDirectory.RowsPerStrip || fileDirectory.ImageLength;
    return applyPredictor(
      decoded,
      predictor,
      tileWidth,
      tileHeight,
      fileDirectory.BitsPerSample,
      fileDirectory.PlanarConfiguration,
    );
  }
  return decoded;
}

/** Assumes inputs like 'image/png' */
export interface BlobOptions {
  type?: string;
}

/**
 * @param fileDirectory
 * @param buffer
 * @param options
 */
export async function decodeImage(
  buffer: ArrayBufferLike,
  options: BlobOptions = {},
): Promise<ImageData> {
  const blob = new Blob([buffer as ArrayBuffer], options); // e.g. { type: 'image/png' }
  const imageBitmap = await createImageBitmap(blob);
  // Create OffscreenCanvas and draw
  const canvas: OffscreenCanvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
  const ctx = canvas.getContext('2d');
  if (!ctx) throw new Error('Could not get 2d context');
  ctx.drawImage(imageBitmap, 0, 0);

  return ctx.getImageData(0, 0, canvas.width, canvas.height);
}

/**
 * @param buffer - an array of packed bits in a block
 * @returns the decoded array
 */
export function decodePackedBitsBlock(buffer: ArrayBufferLike): ArrayBufferLike {
  const dataView = new DataView(buffer);
  const out = [];

  for (let i = 0; i < buffer.byteLength; ++i) {
    let header = dataView.getInt8(i);
    if (header < 0) {
      const next = dataView.getUint8(i + 1);
      header = -header;
      for (let j = 0; j <= header; ++j) out.push(next);
      i += 1;
    } else {
      for (let j = 0; j <= header; ++j) out.push(dataView.getUint8(i + j + 1));
      i += header + 1;
    }
  }
  return new Uint8Array(out).buffer;
}
