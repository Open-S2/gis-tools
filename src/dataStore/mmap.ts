import { compareIDs } from '..';
import { externalSort } from './externalSort';
import { mmap } from 'bun';
import { tmpdir } from 'os';
import { closeSync, openSync, unlinkSync, writeSync } from 'fs';

import type { S2CellId } from '..';
import type { Properties, Value, VectorKey } from '..';

/** Options to create a S2MMapStore */
export interface MMapOptions {
  /** If true, then the values are stored in the index section of the keys file */
  valuesAreIndex?: boolean;
  /** If true, then the data is already sorted and get calls can be immediately returned */
  isSorted?: boolean;
  /** The maximum heap size in bytes for each grouping of data. */
  maxHeap?: number;
  /** The number of threads to use for sorting */
  threadCount?: number;
  /** If desired, a temporary directory to use */
  tmpDir?: string;
}

/** An entry in a file */
export interface MMapEntry<V> {
  key: S2CellId;
  value: V;
}

/**
 * NOTE: The File KVStore is designed to be used in states:
 * - write-only. The initial state is write-only. Write all you need to before reading
 * - read-only. Once you have written everything, the first read will lock the file to be static
 * and read-only.
 */
export class S2MMapStore<V = Properties | Value | VectorKey> {
  readonly fileName: string;
  #state: 'read' | 'write' = 'read';
  #size = 0;
  #sorted: boolean;
  #maxHeap?: number;
  #threadCount?: number;
  #tmpDir?: string;
  // options
  #indexIsValues = false;
  // write params
  #valueOffset = 0;
  #keyFd: number = -1;
  #valueFd: number = -1;
  // readers
  #keyReader!: Uint8Array;
  #valueReader!: Uint8Array;

  /**
   * Builds a new File based KV
   * @param fileName - the path + file name without the extension
   * @param options - the options of how the store should be created and ued
   */
  constructor(fileName?: string, options?: MMapOptions) {
    this.fileName = fileName ?? buildTmpFileName(options?.tmpDir);
    this.#sorted = options?.isSorted ?? false;
    this.#indexIsValues = options?.valuesAreIndex ?? false;
    this.#maxHeap = options?.maxHeap;
    this.#threadCount = options?.threadCount;
    this.#tmpDir = options?.tmpDir;
    if (!this.#sorted) this.#switchToWriteState();
    else {
      this.#keyReader = mmap(`${this.fileName}.sortedKeys`) as Uint8Array;
      if (!this.#indexIsValues) this.#valueReader = mmap(`${this.fileName}.values`) as Uint8Array;
      this.#size = this.#keyReader.length / 16;
    }
  }

  /** @returns - the length of the store */
  get length(): number {
    return this.#size;
  }

  /**
   * Adds a value to be associated with a key
   * @param key - the uint64 id
   * @param value - the value to store
   */
  set(key: number | S2CellId, value: V): void {
    this.#switchToWriteState();
    // prepare value
    // @ts-expect-error - we know its an object
    if (typeof value === 'object' && 'cell' in value && typeof value.cell === 'bigint')
      value.cell = value.cell.toString();
    const valueStr = JSON.stringify(value);
    const valueBuf = Buffer.from(valueStr);
    // write key offset as a uint64
    const buffer = Buffer.alloc(16);
    buffer.writeBigUInt64LE(BigInt(key), 0);
    // write value offset to point to the value position in the `${path}.values`
    if (this.#indexIsValues) {
      if (typeof value !== 'number' && typeof value !== 'bigint')
        throw new Error('value must be a number.');
      if (typeof value === 'number') {
        buffer.writeUInt32LE(value >>> 0, 8);
        buffer.writeUInt32LE(Math.floor(value / 0x100000000), 12);
      } else {
        buffer.writeBigInt64LE(value, 8);
      }
    } else {
      buffer.writeUInt32LE(this.#valueOffset, 8);
      buffer.writeUInt32LE(valueBuf.byteLength, 12);
    }
    writeSync(this.#keyFd, buffer);
    // write value and update value offset
    if (!this.#indexIsValues) writeSync(this.#valueFd, valueBuf);
    this.#valueOffset += valueBuf.byteLength;
    // update size
    this.#size++;
  }

  /**
   * Gets the value associated with a key
   * @param key - the key
   * @param max - the max number of values to return
   * @param bigint - set to true if the key is a bigint
   * @returns the value if the map contains values for the key
   */
  async get(key: number | S2CellId, max?: number, bigint = false): Promise<V[] | undefined> {
    key = BigInt(key);
    await this.#switchToReadState();
    if (this.#size === 0) return;
    let lowerIndex = this.#lowerBound(key);
    if (lowerIndex >= this.#size) return undefined;
    const res: V[] = [];
    while (true) {
      const keySlice = this.#keyReader.subarray(lowerIndex * 16, lowerIndex * 16 + 16);
      const buffer = Buffer.from(keySlice);
      if (buffer.readBigUInt64LE(0) !== key) break;
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        if (bigint) res.push((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V);
        else res.push(((valueOffset >>> 0) + (valueLength >>> 0) * 0x100000000) as unknown as V);
      } else {
        const valSlice = this.#valueReader.subarray(valueOffset, valueOffset + valueLength);
        const valueBuf = Buffer.from(valSlice);
        const json = JSON.parse(valueBuf.toString()) as V;
        // @ts-expect-error - we know its an object
        if (typeof json === 'object' && 'cell' in json) json.cell = BigInt(json.cell);
        res.push(json);
      }
      if (max !== undefined && res.length >= max) break;
      lowerIndex++;
      if (lowerIndex >= this.#size) break;
    }

    if (res.length === 0) return undefined;
    return res;
  }

  /** Sort the data if not sorted */
  async sort(): Promise<void> {
    await this.#switchToReadState();
  }

  /**
   * Closes the store
   * @param cleanup - set to true if you want to remove the .keys and .values files upon closing
   */
  close(cleanup = false): void {
    if (this.#keyFd >= 0) closeSync(this.#keyFd);
    if (!this.#indexIsValues && this.#valueFd >= 0) closeSync(this.#valueFd);
    if (cleanup) {
      unlinkSync(`${this.fileName}.keys`);
      if (!this.#indexIsValues) unlinkSync(`${this.fileName}.values`);
      if (this.#sorted) unlinkSync(`${this.fileName}.sortedKeys`);
    }
  }

  /**
   * Iterates over all values in the store
   * @param bigint - set to true if the value is a bigint stored in the index
   * @yields an iterator
   */
  async *entries(bigint = false): AsyncIterableIterator<MMapEntry<V>> {
    await this.#switchToReadState();
    for (let i = 0; i < this.#size; i++) {
      const keySlice = this.#keyReader.subarray(i * 16, i * 16 + 16);
      const buffer = Buffer.from(keySlice);
      const key = buffer.readBigUInt64LE(0);
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        const value = bigint
          ? ((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V)
          : ((valueOffset + (valueLength << 32)) as unknown as V);
        yield { key, value };
      } else {
        const valSlice = this.#valueReader.subarray(valueOffset, valueOffset + valueLength);
        const valueBuf = Buffer.from(valSlice);
        const value = JSON.parse(valueBuf.toString()) as V;
        // @ts-expect-error - we know its an object
        if (typeof value === 'object' && 'cell' in value) value.cell = BigInt(value.cell);
        yield { key, value };
      }
    }
  }

  /** Switches to write state if in read. */
  #switchToWriteState(): void {
    if (this.#state === 'write') return;
    this.#state = 'write';
    this.#keyFd = openSync(`${this.fileName}.keys`, 'a');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${this.fileName}.values`, 'a');
  }

  /** Switches to read state if in write. Also sort the keys. */
  async #switchToReadState(): Promise<void> {
    if (this.#state === 'read') return;
    this.#state = 'read';
    if (this.#keyFd > 0) {
      closeSync(this.#keyFd);
      this.#keyFd = -1;
    }
    if (!this.#indexIsValues && this.#valueFd > 0) {
      closeSync(this.#valueFd);
      this.#valueFd = -1;
    }
    if (this.#size === 0) return;
    await this.#sort();
    this.#keyReader = mmap(`${this.fileName}.sortedKeys`);
    if (!this.#indexIsValues) this.#valueReader = mmap(`${this.fileName}.values`);
  }

  /** Sort the data */
  async #sort(): Promise<void> {
    if (this.#sorted) return;
    await externalSort(
      [this.fileName],
      this.fileName,
      this.#maxHeap,
      this.#threadCount,
      this.#tmpDir,
    );
    this.#sorted = true;
  }

  /**
   * @param id - the id to search for
   * @returns the starting index from the lower bound of the id
   */
  #lowerBound(id: S2CellId): number {
    // lower bound search
    let lo: number = 0;
    let hi: number = this.#size;
    let mid: number;

    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const loHi = this.#getKey(mid);
      if (compareIDs(loHi, id) === -1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    return lo;
  }

  /**
   * @param index - the index to get the key from
   * @returns the key
   */
  #getKey(index: number): S2CellId {
    const key = this.#keyReader.subarray(index * 16, index * 16 + 8);
    const buf = Buffer.from(key);
    return buf.readBigUInt64LE(0);
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
