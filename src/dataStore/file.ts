import { compareIDs } from '..';
import { externalSort } from './externalSort';
import { tmpdir } from 'os';
import { closeSync, fstatSync, openSync, readSync, unlinkSync, writeSync } from 'fs';

import type { S2CellId } from '..';
import type { Properties, Value, VectorKey } from '..';

/** Options to create a S2FileStore */
export interface FileOptions {
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
export interface FileEntry<V> {
  key: S2CellId;
  value: V;
}

const KEY_LENGTH = 16;

/**
 * NOTE: The File KVStore is designed to be used in states:
 * - write-only. The initial state is write-only. Write all you need to before reading
 * - read-only. Once you have written everything, the first read will lock the file to be static
 * and read-only.
 */
export class S2FileStore<V = Properties | Value | VectorKey> {
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
  // read write fd
  #keyFd: number = -1;
  #valueFd: number = -1;

  /**
   * Builds a new File based KV
   * @param fileName - the path + file name without the extension
   * @param options - the options of how the store should be created and used
   */
  constructor(fileName?: string, options?: FileOptions) {
    this.fileName = fileName ?? buildTmpFileName(options?.tmpDir);
    this.#sorted = options?.isSorted ?? false;
    this.#indexIsValues = options?.valuesAreIndex ?? false;
    this.#maxHeap = options?.maxHeap;
    this.#threadCount = options?.threadCount;
    this.#tmpDir = options?.tmpDir;
    if (!this.#sorted) this.#switchToWriteState();
    else {
      this.#keyFd = openSync(`${this.fileName}.sortedKeys`, 'r');
      if (!this.#indexIsValues) this.#valueFd = openSync(`${this.fileName}.values`, 'r');
    }
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
   * @param key - the uint64 id
   * @param value - the value to store
   */
  set(key: number | S2CellId, value: V): void {
    this.#switchToWriteState();
    // prepare value
    // @ts-expect-error - we know its an object
    if (typeof value === 'object' && 'cell' in value && typeof value.cell === 'bigint')
      value.cell = value.cell.toString();
    const valueBuf = Buffer.from(JSON.stringify(value));
    // write key offset as a uint64
    const buffer = Buffer.alloc(KEY_LENGTH);
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
    const buffer = Buffer.alloc(KEY_LENGTH);
    while (true) {
      readSync(this.#keyFd, buffer, 0, KEY_LENGTH, lowerIndex * KEY_LENGTH);
      if (buffer.readBigUInt64LE(0) !== key) break;
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        if (bigint) res.push((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V);
        else res.push(((valueOffset >>> 0) + (valueLength >>> 0) * 0x100000000) as unknown as V);
      } else {
        const valueBuf = Buffer.alloc(valueLength);
        readSync(this.#valueFd, valueBuf, 0, valueLength, valueOffset);
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
   * Iterates over all values in the store
   * @param bigint - set to true if the value is a bigint stored in the index
   * @yields an iterator
   */
  async *entries(bigint = false): AsyncIterableIterator<FileEntry<V>> {
    await this.#switchToReadState();
    for (let i = 0; i < this.#size; i++) {
      const buffer = Buffer.alloc(KEY_LENGTH);
      readSync(this.#keyFd, buffer, 0, KEY_LENGTH, i * KEY_LENGTH);
      const key = buffer.readBigUInt64LE(0);
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        const value = bigint
          ? ((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V)
          : ((valueOffset + valueLength * 0x100000000) as unknown as V);
        yield { key, value };
      } else {
        const valueBuf = Buffer.alloc(valueLength);
        readSync(this.#valueFd, valueBuf, 0, valueLength, valueOffset);
        const value = JSON.parse(valueBuf.toString()) as V;
        // @ts-expect-error - we know its an object
        if (typeof value === 'object' && 'cell' in value) value.cell = BigInt(value.cell);
        yield { key, value };
      }
    }
  }

  /**
   * Closes the store
   * @param cleanup - set to true if you want to remove the .keys and .values files upon closing
   */
  close(cleanup = false): void {
    if (this.#keyFd >= 0) {
      closeSync(this.#keyFd);
      this.#keyFd = -1;
    }
    if (!this.#indexIsValues && this.#valueFd >= 0) {
      closeSync(this.#valueFd);
      this.#valueFd = -1;
    }
    if (cleanup) {
      unlinkSync(`${this.fileName}.keys`);
      if (!this.#indexIsValues) unlinkSync(`${this.fileName}.values`);
      if (this.#sorted) unlinkSync(`${this.fileName}.sortedKeys`);
    }
  }

  /** Switches to write state if in read. */
  #switchToWriteState(): void {
    if (this.#state === 'write') return;
    this.#state = 'write';
    this.close();
    this.#keyFd = openSync(`${this.fileName}.keys`, 'a');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${this.fileName}.values`, 'a');
  }

  /** Switches to read state if in write. Also sort the keys. */
  async #switchToReadState(): Promise<void> {
    if (this.#state === 'read') return;
    this.#state = 'read';
    this.close();
    if (this.#size === 0) return;
    await this.#sort();
    this.#keyFd = openSync(`${this.fileName}.sortedKeys`, 'r');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${this.fileName}.values`, 'r');
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
    const buf = Buffer.alloc(8);
    readSync(this.#keyFd, buf, 0, 8, index * KEY_LENGTH);
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
