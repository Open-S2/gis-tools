import { closeSync, fstatSync, openSync, readSync, writeSync } from 'fs';

import type { Uint64Cell } from '../wasm/uint64';
import type { Key, Stringifiable } from '..';

/** A low/high pair for faster comparisons */
interface LowHigh {
  low: number;
  high: number;
}

/**
 * NOTE: The File KVStore is designed to be used in states:
 * - write-only. The initial state is write-only. Write all you need to before reading
 * - read-only. Once you have written everything, the first read will lock the file to be static
 * and read-only.
 */
export class S2FileStore<V = Stringifiable> {
  #state: 'read' | 'write' = 'read';
  #size = 0;
  #sorted: boolean;
  // options
  #indexIsValues = false;
  // write params
  #valueOffset = 0;
  #keyFd: number;
  #valueFd: number;

  /**
   * Builds a new File based KV
   * @param path - the path to the file
   * @param isSorted - set to true if the keys are already sorted
   * @param valuesAreIndex - set to true if the values are stored in the index section of the keys file
   */
  constructor(
    public readonly path: string,
    isSorted = false,
    valuesAreIndex = false,
  ) {
    this.#sorted = isSorted;
    this.#indexIsValues = valuesAreIndex;
    this.#keyFd = openSync(`${path}.keys`, 'a');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${path}.values`, 'a');
    else this.#valueFd = -1;
    // Update the size if the file already existed
    const stat = fstatSync(this.#keyFd);
    if (stat.size >= 16) this.#size = stat.size / 16;
  }

  /**
   * Adds a value to be associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: Key, value: V): void {
    if (this.#state !== 'read') throw new Error('Can no longer write to KVFile store.');
    // prepare value
    const valueStr = JSON.stringify(value);
    const valueBuf = Buffer.from(valueStr);
    // write key offset as a uint64
    const buffer = Buffer.alloc(16);
    const { low, high } = this.#getLowHigh(key);
    buffer.writeUInt32LE(low, 0);
    buffer.writeUInt32LE(high, 4);
    // write value offset to point to the value position in the `${path}.values`
    if (this.#indexIsValues) {
      if (typeof value !== 'number' && typeof value !== 'bigint')
        throw new Error('value must be a number.');
      if (typeof value === 'number') {
        buffer.writeUInt32LE(value & 0xffffffff, 8);
        buffer.writeUInt32LE(value >>> 32, 12);
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
  get(key: Key, max?: number, bigint = false): V[] | undefined {
    this.#switchToWriteState();
    let lowerIndex = this.#lowerBound(key);
    if (lowerIndex >= this.#size) return undefined;
    const res: V[] = [];
    const buffer = Buffer.alloc(16);
    while (true) {
      readSync(this.#keyFd, buffer, 0, 16, lowerIndex * 16);
      const { low: lowID, high: highID } = this.#getLowHigh(key);
      if (buffer.readUInt32LE(0) !== lowID || buffer.readUInt32LE(4) !== highID) break;
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        if (bigint) res.push((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V);
        else res.push((valueOffset + (valueLength << 32)) as unknown as V);
      } else {
        const valueBuf = Buffer.alloc(valueLength);
        readSync(this.#valueFd, valueBuf, 0, valueLength, valueOffset);
        res.push(JSON.parse(valueBuf.toString()) as V);
      }
      if (max && res.length >= max) break;
      lowerIndex++;
      if (lowerIndex >= this.#size) break;
    }

    if (res.length === 0) return undefined;
    return res;
  }

  /**
   * Check if the map contains the key
   * @param key - the key
   * @returns true if the map contains value(s) for the key
   */
  has(key: Key): boolean {
    this.#switchToWriteState();
    const lowerIndex = this.#lowerBound(key);
    if (lowerIndex >= this.#size) return false;
    const { low: lowID, high: highID } = this.#getLowHigh(key);
    const buffer = Buffer.alloc(8);
    readSync(this.#keyFd, buffer, 0, 8, lowerIndex * 16);
    const low = buffer.readUInt32LE(0);
    const high = buffer.readUInt32LE(4);
    return low === lowID && high === highID;
  }

  /** Switches to write state if in read. Also sort the keys. */
  #switchToWriteState(): void {
    if (this.#state === 'write') return;
    if (this.#state === 'read') {
      this.#state = 'write';
      closeSync(this.#keyFd);
      if (!this.#indexIsValues) closeSync(this.#valueFd);
      this.#keyFd = openSync(`${this.path}.keys`, 'r+');
      if (!this.#indexIsValues) this.#valueFd = openSync(`${this.path}.values`, 'r+');
      this.#sort();
    }
  }

  /** Sort the data */
  #sort(): void {
    if (this.#sorted) return;
    this.#quickSort(0, this.#size - 1);
    this.#sorted = true;
  }

  /**
   * TODO: Running in parallel can be faster
   * @param low - low index inclusive
   * @param high - high index inclusive
   */
  #quickSort(low: number, high: number): void {
    if (low >= high) return;
    const pivot = this.#partition(low, high);
    this.#quickSort(low, pivot - 1);
    this.#quickSort(pivot + 1, high);
  }

  /**
   * @param low - low index inclusive
   * @param high - high index inclusive
   * @returns the index of the pivot
   */
  #partition(low: number, high: number): number {
    const pivot = this.#getKey(high);
    let i = low - 1;
    for (let j = low; j < high; j++) {
      const jLohi = this.#getKey(j);
      if (this.#compareLowHigh(jLohi, pivot) <= 0) {
        i++;
        this.#swapKeys(i, j);
      }
    }
    this.#swapKeys(i + 1, high);
    return i + 1;
  }

  /**
   * Swap the keys at i and j including their pointers to values
   * @param i - the index of the i-th key
   * @param j - the index of the j-th key
   */
  #swapKeys(i: number, j: number): void {
    if (i === j) return;
    const iKey = Buffer.alloc(16);
    const jKey = Buffer.alloc(16);
    readSync(this.#keyFd, iKey, 0, 16, i * 16);
    readSync(this.#keyFd, jKey, 0, 16, j * 16);
    writeSync(this.#keyFd, jKey, 0, 16, i * 16);
    writeSync(this.#keyFd, iKey, 0, 16, j * 16);
    readSync(this.#keyFd, iKey, 0, 16, i * 16);
    readSync(this.#keyFd, jKey, 0, 16, j * 16);
  }

  /**
   * @param id - the id to search for
   * @returns the starting index from the lower bound of the id
   */
  #lowerBound(id: Key): number {
    const loHiID = this.#getLowHigh(id);
    this.#sort();
    // lower bound search
    let lo: number = 0;
    let hi: number = this.#size;
    let mid: number;

    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const loHi = this.#getKey(mid);
      if (this.#compareLowHigh(loHi, loHiID) === -1) {
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
  #getKey(index: number): LowHigh {
    // read in the key
    const buffer = Buffer.alloc(8);
    readSync(this.#keyFd, buffer, 0, 4, index * 16);
    return {
      low: buffer.readUInt32LE(0),
      high: buffer.readUInt32LE(4),
    };
  }

  /**
   * @param lohiA - the first LowHigh
   * @param lohiB - the second LowHigh
   * @returns -1 | 0 | 1
   */
  #compareLowHigh(lohiA: LowHigh, lohiB: LowHigh): -1 | 0 | 1 {
    if (lohiA.high < lohiB.high) return -1;
    if (lohiA.high > lohiB.high) return 1;
    if (lohiA.low < lohiB.low) return -1;
    if (lohiA.low > lohiB.low) return 1;
    return 0;
  }

  /**
   * @param key - the key used by the S2FileStore
   * @returns the low and high parts of the key
   */
  #getLowHigh(key: Key): LowHigh {
    if (typeof key === 'number') {
      const keyBig = BigInt(key);
      return {
        low: Number(keyBig & 0xffffffffn),
        high: Number(keyBig >> 32n) & 0xffffffff,
      };
    } else if (typeof key === 'bigint') {
      return {
        low: Number(key & 0xffffffffn),
        high: Number(key >> 32n) & 0xffffffff,
      };
    } else {
      const { low, high } = key as Uint64Cell;
      return {
        low,
        high,
      };
    }
  }
}
