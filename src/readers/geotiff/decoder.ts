import { decompressStream, lzwDecoder } from '../../util';
import { imageDecoderBuffer, jpegDecoder } from '../image';

/** What to expect from the decoder */
export type Decoder =
  | ((buffer: ArrayBufferLike) => Promise<ArrayBufferLike>)
  | ((buffer: ArrayBufferLike) => ArrayBufferLike)
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
  else if ([6, 50001].includes(compression)) return imageDecoderBuffer;
  else if ([8, 32946].includes(compression)) return deflateDecoder;
  else if (compression === 32773) return packbitsDecoder;
  throw new Error(`Unsupported compression: ${compression}`);
}

/**
 * Deflate and decode the input buffer
 * @param buffer - inflated data
 * @returns - the decoded buffer
 */
async function deflateDecoder(buffer: ArrayBufferLike): Promise<ArrayBufferLike> {
  return (await decompressStream(new Uint8Array(buffer))).buffer;
}

/**
 * Raw decoder
 * @param buffer - the input buffer
 * @returns - the decoded buffer
 */
function rawDecoder(buffer: ArrayBufferLike): ArrayBufferLike {
  return buffer;
}

/**
 * Packbits decoder
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
