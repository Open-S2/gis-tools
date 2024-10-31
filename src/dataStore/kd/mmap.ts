// TODO: Add support for multi-threading the sorting
// * merge with other stores
import { mmap } from 'bun';
import { tmpdir } from 'os';
import { closeSync, fstatSync, openSync, unlinkSync, writeSync } from 'fs';

import type { Stringifiable } from '..';
import type { KDKV, KDStore } from '.';

/** Options to create a S2FileStore */
export interface MMapOptions {
  /** If true, then the values are stored in the index section of the keys file */
  valuesAreIndex?: boolean;
  /** The maximum heap size in bytes for each grouping of data. */
  maxHeap?: number;
  // /** The number of threads to use for sorting */
  // threadCount?: number;
  /** If desired, a temporary directory to use */
  tmpDir?: string;
}

const KEY_LENGTH = 24;

/** A local KD key-value store */
export class KDMMapSpatialIndex<T = Stringifiable> implements KDStore<T> {
  readonly fileName: string;
  #size = 0;
  // #threadCount?: number;
  // options
  #indexIsValues = false;
  // write params
  #valueOffset = 0;
  // read write fd
  #keyFd: number = -1;
  #valueFd: number = -1;
  #keyMap!: Uint8Array;
  #valueMap!: Uint8Array;

  /**
   * @param nodeSize - the size of each kd-tree node
   * @param fileName - the path + file name without the extension
   * @param options - the options of how the store should be created and used
   */
  constructor(
    private readonly nodeSize = 64,
    fileName?: string,
    options?: MMapOptions,
  ) {
    this.fileName = fileName ?? buildTmpFileName(options?.tmpDir);
    this.#indexIsValues = options?.valuesAreIndex ?? false;
    // this.#threadCount = options?.threadCount;
    // setup reads
    this.#keyFd = openSync(`${this.fileName}.keys`, 'a');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${this.fileName}.values`, 'a');
    // Update the size if the file already existed
    const stat = fstatSync(this.#keyFd);
    if (stat.size >= KEY_LENGTH) this.#size = stat.size / KEY_LENGTH;
  }

  /** @returns - the length of the store */
  get length(): number {
    return this.#size;
  }

  /**
   * Adds a value to be associated with a key
   * @param value - the value to store
   */
  push(value: KDKV<T>): void {
    // @ts-expect-error - we want to reset our map since it will be out of sync
    this.#keyMap = undefined;
    const { x, y, data } = value;
    // prepare value
    const valueBuf = Buffer.from(JSON.stringify(data));
    // write key offset as a uint64
    const buffer = Buffer.alloc(KEY_LENGTH);
    buffer.writeDoubleLE(x, 0);
    buffer.writeDoubleLE(y, 8);
    // write value offset to point to the value position in the `${path}.values`
    if (this.#indexIsValues) {
      if (typeof value !== 'number' && typeof value !== 'bigint')
        throw new Error('value must be a number.');
      if (typeof value === 'number') {
        buffer.writeDoubleLE(x, 16);
      } else {
        buffer.writeBigInt64LE(value, 16);
      }
    } else {
      buffer.writeUInt32LE(this.#valueOffset, 16);
      buffer.writeUInt32LE(valueBuf.byteLength, 20);
    }
    writeSync(this.#keyFd, buffer);
    // write value and update value offset
    if (!this.#indexIsValues) writeSync(this.#valueFd, valueBuf);
    this.#valueOffset += valueBuf.byteLength;
    // update size
    this.#size++;
  }

  /**
   * Gets a value from the store at index
   * @param index - the position in the store
   * @returns - the value
   */
  get(index: number): KDKV<T> {
    this.#setupMMap();
    const keySlice = this.#keyMap.subarray(index * KEY_LENGTH, index * KEY_LENGTH + KEY_LENGTH);
    const buffer = Buffer.from(keySlice);
    const x = buffer.readDoubleLE(0);
    const y = buffer.readDoubleLE(8);
    if (this.#indexIsValues) {
      const data = buffer.readBigInt64LE(16) as T;
      return { x, y, data };
    } else {
      const valueOffset = buffer.readUInt32LE(16);
      const valueLength = buffer.readUInt32LE(20);
      const valueSlice = this.#valueMap.subarray(valueOffset, valueOffset + valueLength);
      const valueBuf = Buffer.from(valueSlice);
      const data = JSON.parse(valueBuf.toString()) as T;
      return { x, y, data };
    }
  }

  /**
   * Get a range of values
   * @param indexStart - the start index
   * @param indexEnd - the end index
   * @returns - the values
   */
  getRange(indexStart: number, indexEnd: number): KDKV<T>[] {
    this.#setupMMap();
    const res: KDKV<T>[] = [];

    const length = indexEnd - indexStart;
    const keySlice = this.#keyMap.subarray(indexStart * KEY_LENGTH, indexEnd * KEY_LENGTH);
    const buffer = Buffer.from(keySlice);

    for (let i = 0; i < length; i++) {
      const x = buffer.readDoubleLE(i * KEY_LENGTH + 0);
      const y = buffer.readDoubleLE(i * KEY_LENGTH + 8);
      if (this.#indexIsValues) {
        const data = buffer.readBigInt64LE(i * KEY_LENGTH + 16) as T;
        res.push({ x, y, data });
      } else {
        const valueOffset = buffer.readUInt32LE(i * KEY_LENGTH + 16);
        const valueLength = buffer.readUInt32LE(i * KEY_LENGTH + 20);
        const valueSlice = this.#valueMap.subarray(valueOffset, valueLength + valueOffset);
        const valueBuf = Buffer.from(valueSlice);
        const data = JSON.parse(valueBuf.toString()) as T;
        res.push({ x, y, data });
      }
    }

    return res;
  }

  /**
   * Iterates over all values in the store
   * @param bigint - set to true if the value is a bigint stored in the index
   * @yields an iterator
   */
  *values(bigint = false): Generator<KDKV<T>> {
    this.#setupMMap();
    for (let i = 0; i < this.#size; i++) {
      const keySlice = this.#keyMap.subarray(i * KEY_LENGTH, i * KEY_LENGTH + KEY_LENGTH);
      const buffer = Buffer.from(keySlice);
      const x = buffer.readDoubleLE(0);
      const y = buffer.readDoubleLE(8);
      if (this.#indexIsValues) {
        const data = (bigint ? buffer.readBigInt64LE(16) : buffer.readDoubleLE(16)) as T;
        yield { x, y, data };
      } else {
        const valueOffset = buffer.readUInt32LE(16);
        const valueLength = buffer.readUInt32LE(20);
        const valueSlice = this.#valueMap.subarray(valueOffset, valueOffset + valueLength);
        const valueBuffer = Buffer.from(valueSlice);
        const data = JSON.parse(valueBuffer.toString()) as T;
        yield { x, y, data };
      }
    }
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.iterator](): Generator<KDKV<T>> {
    return this.values();
  }

  /**
   * Closes the store
   * @param cleanup - set to true if you want to remove the .keys and .values files upon closing
   */
  close(cleanup = false): void {
    this.#clearMMap();
    if (this.#keyFd >= 0) closeSync(this.#keyFd);
    if (!this.#indexIsValues && this.#valueFd >= 0) closeSync(this.#valueFd);
    if (cleanup) {
      unlinkSync(`${this.fileName}.keys`);
      if (!this.#indexIsValues) unlinkSync(`${this.fileName}.values`);
    }
  }

  /** Sort the store using the Floyd-Rivest selection algorithm */
  sort(): void {
    this.#sort(0, this.#size - 1, 0);
  }

  /**
   * Recursively kd-sort the store
   * @param left - the leftmost index
   * @param right - the rightmost index
   * @param axis - 0 for x, 1 for y
   */
  #sort(left: number, right: number, axis: number): void {
    this.#setupMMap();
    if (right - left <= this.nodeSize) return;

    const m = (left + right) >> 1; // middle index
    // sort ids and coords around the middle index so that the halves lie
    // either left/right or top/bottom correspondingly (taking turns)
    this.#select(m, left, right, axis);
    // recursively kd-sort first half and second half on the opposite axis
    this.#sort(left, m - 1, 1 - axis);
    this.#sort(m + 1, right, 1 - axis);
  }

  /**
   * Custom Floyd-Rivest selection algorithm: sort coords so that
   * [left..k-1] items are smaller than k-th item (on either x or y axis)
   * @param k - the sorting anchor index between left and right
   * @param left - the leftmost index
   * @param right - the rightmost index
   * @param axis - 0 for x, 1 for y
   */
  #select(k: number, left: number, right: number, axis: number): void {
    while (right > left) {
      if (right - left > 600) {
        const n = right - left + 1;
        const m = k - left + 1;
        const z = Math.log(n);
        const s = 0.5 * Math.exp((2 * z) / 3);
        const sd = 0.5 * Math.sqrt((z * s * (n - s)) / n) * (m - n / 2 < 0 ? -1 : 1);
        const newLeft = Math.max(left, Math.floor(k - (m * s) / n + sd));
        const newRight = Math.min(right, Math.floor(k + ((n - m) * s) / n + sd));
        this.#select(k, newLeft, newRight, axis);
      }

      const t = axis === 0 ? this.#getX(k) : this.#getY(k);
      let i = left;
      let j = right;

      this.#swap(left, k);
      if ((axis === 0 ? this.#getX(right) : this.#getY(right)) > t) this.#swap(left, right);

      while (i < j) {
        this.#swap(i, j);
        i++;
        j--;
        while (i < this.length && (axis === 0 ? this.#getX(i) : this.#getY(i)) < t) i++;
        while (j >= 0 && (axis === 0 ? this.#getX(j) : this.#getY(j)) > t) j--;
      }

      if ((axis === 0 ? this.#getX(left) : this.#getY(left)) === t) {
        this.#swap(left, j);
      } else {
        j++;
        this.#swap(j, right);
      }

      if (j <= k) left = j + 1;
      if (k <= j) right = j - 1;
    }
  }

  /**
   * Get the x coordinate from the buffer
   * @param i - the index
   * @returns - the x coordinate
   */
  #getX(i: number): number {
    const buf = Buffer.from(this.#keyMap.slice(i * KEY_LENGTH, i * KEY_LENGTH + 8));
    return buf.readDoubleLE();
  }

  /**
   * Get the y coordinate from the buffer
   * @param i - the index
   * @returns - the y coordinate
   */
  #getY(i: number): number {
    const buf = Buffer.from(this.#keyMap.slice(i * KEY_LENGTH + 8, i * KEY_LENGTH + 16));
    return buf.readDoubleLE();
  }

  /**
   * swaps two values of KEY_LENGTH (24bytes) size
   * @param i - the first index
   * @param j - the second index
   */
  #swap(i: number, j: number): void {
    const bufI = this.#keyMap.slice(i * KEY_LENGTH, i * KEY_LENGTH + KEY_LENGTH); // Create a copy of bufI
    const bufJ = this.#keyMap.subarray(j * KEY_LENGTH, j * KEY_LENGTH + KEY_LENGTH);
    this.#keyMap.set(bufJ, i * KEY_LENGTH); // Write bufJ to index i
    this.#keyMap.set(bufI, j * KEY_LENGTH); // Write bufI copy to index j
  }

  /** Sets up a mmap of the key file */
  #setupMMap(): void {
    if (this.#keyMap === undefined) this.#keyMap = mmap(`${this.fileName}.keys`);
    if (this.#valueMap === undefined) this.#valueMap = mmap(`${this.fileName}.values`);
  }

  /** Clears the mmaps (something renders them unusable) */
  #clearMMap(): void {
    // @ts-expect-error - we want to clear the mmap
    this.#keyMap = undefined;
    // @ts-expect-error - we want to clear the mmap
    this.#valueMap = undefined;
  }
}

/**
 * @param tmpDir - the temporary directory to use if provided otherwise default os tmpdir
 * @returns - a temporary file name based on a random number.
 */
function buildTmpFileName(tmpDir?: string): string {
  const tmpd = tmpDir ?? tmpdir();
  const randomName = Math.random().toString(36).slice(2);
  return `${tmpd}/${randomName}`;
}
