export * from './gzip';
export * from './lzw';

/** The formats available to DecompressionStream */
export type Format = 'deflate' | 'deflate-raw' | 'gzip';

/**
 * pollyfill for string to array buffer
 * @param base64 - base64 encoded string
 * @returns converted ArrayBuffer of the string data
 */
export function base64ToArrayBuffer(base64: string): ArrayBufferLike {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

  return bytes.buffer;
}

/**
 * @param uint8arrays - the Uint8Arrays to concatenate
 * @returns - the concatenated Uint8Array
 */
export async function concatUint8Arrays(uint8arrays: Uint8Array[]): Promise<Uint8Array> {
  const blob = new Blob(uint8arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}
