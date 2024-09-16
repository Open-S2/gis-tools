export * from './gzip';

/**
 * @param uint8arrays - the Uint8Arrays to concatenate
 * @returns - the concatenated Uint8Array
 */
export async function concatUint8Arrays(uint8arrays: Uint8Array[]): Promise<Uint8Array> {
  const blob = new Blob(uint8arrays);
  const buffer = await blob.arrayBuffer();
  return new Uint8Array(buffer);
}
