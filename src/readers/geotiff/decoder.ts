import { jpegDecoder } from './jpeg';
import { decompressStream, lzwDecoder } from 's2-tools/util';

/** What to expect from the decoder */
export type Decoder =
  | ((buffer: ArrayBufferLike, tables?: number[]) => Promise<ArrayBufferLike>)
  | ((buffer: ArrayBufferLike, tables?: number[]) => ArrayBufferLike);

/**
 * @param compression - the encoded compression value
 * @returns the decoder function matching the given compression value
 */
export function getDecoder(compression = 1): Decoder {
  if (compression === 1) return rawDecoder;
  else if (compression === 5) return lzwDecoder;
  else if (compression === 7) return jpegDecoder;
  else if ([6, 7, 50001].includes(compression)) return imageDecoder;
  else if ([8, 32946].includes(compression)) return deflateDecoder;
  else if (compression === 32773) return packbitsDecoder;
  throw new Error(`Unsupported compression: ${compression}`);
}

/**
 * @param buffer
 */
async function deflateDecoder(buffer: ArrayBufferLike): Promise<ArrayBufferLike> {
  return (await decompressStream(new Uint8Array(buffer))).buffer;
}

/**
 * @param buffer
 */
function rawDecoder(buffer: ArrayBufferLike): ArrayBufferLike {
  return buffer;
}
/**
 * @param fileDirectory
 * @param buffer
 * @param options
 * @param jpegTables
 */
async function imageDecoder(buffer: ArrayBufferLike): Promise<ArrayBufferLike> {
  const blob = new Blob([buffer as ArrayBuffer]); // e.g. { type: 'image/png' }
  const imageBitmap = await createImageBitmap(blob);
  // Create OffscreenCanvas and draw
  const canvas: OffscreenCanvas = new OffscreenCanvas(imageBitmap.width, imageBitmap.height);
  const ctx = canvas.getContext('2d');
  if (ctx === null) throw new Error('Could not get 2d context');
  ctx.drawImage(imageBitmap, 0, 0);

  const imageData = ctx.getImageData(0, 0, imageBitmap.width, imageBitmap.height);
  return imageData.data.buffer;
}

/**
 * @param buffer - an array of packed bits in a block
 * @returns the decoded array
 */
function packbitsDecoder(buffer: ArrayBufferLike): ArrayBufferLike {
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
