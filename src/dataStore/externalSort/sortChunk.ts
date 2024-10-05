import { open } from 'fs/promises';

import type { SortChunk } from '.';

/**
 * @param chunk - chunk information needed to start sorting
 * @returns - output file that it created
 */
export async function sortChunk(chunk: SortChunk): Promise<string> {
  const { name, input, outDir, start, end } = chunk;
  const outFile = `${outDir}/es_${name}_${start}_${end}.tmp`;
  await _sortChunk(input, outFile, start, end);
  return outFile;
}

/**
 * @param input - input file
 * @param output - output file
 * @param start - start index
 * @param end - end index
 */
async function _sortChunk(
  input: string,
  output: string,
  start: number,
  end: number,
): Promise<void> {
  // prepare the input and output files
  const inputHandle = await open(input, 'r').catch((err) => {
    throw new Error(err);
  });
  const outputHandle = await open(output, 'a').catch((err) => {
    throw new Error(err);
  });
  // read in the chunk
  const inputBuffer = Buffer.alloc(end - start);
  await inputHandle.read(inputBuffer, 0, inputBuffer.length, start);
  // sort the chunk
  const keys = bufferToKeys(inputBuffer);
  keys.sort(keySort);
  // write out the sorted chunk
  const sortedBuffer = keysToBuffer(keys);
  await outputHandle.appendFile(sortedBuffer);
  // close the files
  await inputHandle.close();
  await outputHandle.close();
}

/**
 * A Key is a 16 byte buffer that contains a uint64 id split into lo and hi.
 * The last 8 bytes contains the u32 offset and u32 length
 */
export interface Key {
  lo: number;
  hi: number;
  offset: number;
  length: number;
}

/**
 * @param buffer - buffer containing an encoded list of keys
 * @returns - decoded list of keys
 */
export function bufferToKeys(buffer: Buffer): Key[] {
  // for each 16 bytes in the buffer, create a key
  const keys: Key[] = [];
  for (let i = 0; i < buffer.length; i += 16) {
    keys.push({
      lo: buffer.readUInt32LE(i),
      hi: buffer.readUInt32LE(i + 4),
      offset: buffer.readUInt32LE(i + 8),
      length: buffer.readUInt32LE(i + 12),
    });
  }
  return keys;
}

/**
 * @param keys - list of keys
 * @returns - buffer containing an encoded list of keys
 */
export function keysToBuffer(keys: Key[]): Buffer {
  const buffer = Buffer.alloc(keys.length * 16);
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    buffer.writeUInt32LE(key.lo, i * 16);
    buffer.writeUInt32LE(key.hi, i * 16 + 4);
    buffer.writeUInt32LE(key.offset, i * 16 + 8);
    buffer.writeUInt32LE(key.length, i * 16 + 12);
  }
  return buffer;
}

/**
 * @param a - key A
 * @param b - key B
 * @returns - ordered answer between A and B
 */
export function keySort(a: Key, b: Key): -1 | 0 | 1 {
  if (a.hi < b.hi) return -1;
  if (a.hi > b.hi) return 1;
  if (a.lo < b.lo) return -1;
  if (a.lo > b.lo) return 1;
  return 0;
}
