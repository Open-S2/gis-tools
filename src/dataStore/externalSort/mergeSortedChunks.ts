import { promisify } from 'util';
import { bufferToKeys, keySort, keysToBuffer } from './sortChunk';
import { close as closeFS, createWriteStream, open as openFS, read as readFS, statSync } from 'fs';

import type { Key } from './sortChunk';

const open = promisify(openFS);
const read = promisify(readFS);
const close = promisify(closeFS);

/** A wrapper of a readable stream to handle reading in the sorted data */
class SortedFile {
  keys: Key[] = [];
  #offset = 0;
  #isDone = false;
  /**
   * @param input - a readable stream of a sorted file
   * @param size - the size of the file
   */
  constructor(
    readonly input: number,
    readonly size: number,
  ) {}

  /**
   * Check the current key in the buffer
   * @returns - the current key if there is one
   */
  current(): Key | undefined {
    return this.keys.length > 0 ? this.keys[0] : undefined;
  }

  /** Update the current key store if necessary */
  async prepare(): Promise<void> {
    if (this.#isDone) return;
    // if there are no keys in the buffer, read in the next chunk
    if (this.keys.length === 0) {
      const length = Math.min(16 * 1_024, this.size - this.#offset);
      const buffer = Buffer.alloc(length);
      await read(this.input, { buffer, offset: 0, length, position: this.#offset });
      // await this.input.read({ buffer, offset: 0, length, position: this.#offset });
      this.keys = bufferToKeys(buffer);
      this.#offset += length;
      if (this.#offset >= this.size) this.#isDone = true;
    }
  }

  /**
   * @returns - the next key in the buffer. Assumes the user has called current first to
   * validate that there is a current key
   */
  take(): Key | undefined {
    return this.keys.shift();
  }

  /** Closes the input file */
  async close(): Promise<void> {
    await close(this.input);
  }
}

/**
 * @param inputs - a list of sorted files
 * @param output - output file
 */
export default async function mergeSortedChunks(inputs: string[], output: string): Promise<void> {
  const inputFiles: SortedFile[] = [];
  for (const input of inputs) {
    inputFiles.push(new SortedFile(await open(input, 'r'), statSync(input).size));
  }
  const outputStream = createWriteStream(output);

  // loop through all the input files and grab the next key in order
  let keyWrites: Key[] = [];
  while (true) {
    const nextKey = await getNextLowestKey(inputFiles);
    if (nextKey === undefined) break;
    keyWrites.push(nextKey);
    if (keyWrites.length > 1_024) {
      outputStream.write(keysToBuffer(keyWrites));
      keyWrites = [];
    }
  }
  if (keyWrites.length > 0) outputStream.write(keysToBuffer(keyWrites));

  // finally close all the input files
  for (const input of inputFiles) await input.close();
  outputStream.close();
}

/**
 * @param sortedFiles - a list of sorted files
 * @returns - the next lowest key
 */
async function getNextLowestKey(sortedFiles: SortedFile[]): Promise<Key | undefined> {
  // make sure all files are up to date on their current key
  for (const file of sortedFiles) await file.prepare();
  // 1) sort the files by their current key
  sortedFiles.sort((a, b) => {
    const aKey = a.current();
    const bKey = b.current();
    if (aKey === undefined) return 1;
    else if (bKey === undefined) return -1;
    else return keySort(aKey, bKey);
  });
  if (sortedFiles.length === 0) return undefined;
  return sortedFiles[0].take();
}
