import externalSort from './externalSort';
import { mmap } from 'bun';
import { closeSync, fstatSync, openSync, writeSync } from 'fs';

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
export class S2MMapStore<V = Stringifiable> {
  #state: 'read' | 'write' = 'write';
  #size = 0;
  #sorted: boolean;
  // options
  #indexIsValues = false;
  // write params
  #valueOffset = 0;
  #keyFd: number;
  #valueFd: number;
  // readers
  #keyReader!: Uint8Array;
  #valueReader!: Uint8Array;

  /**
   * Builds a new File based KV
   * @param fileName - the file name without the extension
   * @param isSorted - set to true if the keys are already sorted
   * @param valuesAreIndex - set to true if the values are stored in the index section of the keys file
   */
  constructor(
    public readonly fileName: string,
    isSorted = false,
    valuesAreIndex = false,
  ) {
    this.#sorted = isSorted;
    this.#indexIsValues = valuesAreIndex;
    this.#keyFd = openSync(`${fileName}.keys`, 'a');
    if (!this.#indexIsValues) this.#valueFd = openSync(`${fileName}.values`, 'a');
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
    if (this.#state !== 'write') throw new Error('Can no longer write to KVFile store.');
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
    if (!this.#sorted) throw new Error('Not sorted, please call "switchToReadState" first');
    let lowerIndex = this.#lowerBound(key);
    if (lowerIndex >= this.#size) return undefined;
    const { low: lowID, high: highID } = this.#getLowHigh(key);
    const res: V[] = [];
    while (true) {
      const keySlice = this.#keyReader.subarray(lowerIndex * 16, lowerIndex * 16 + 16);
      const buffer = Buffer.from(keySlice);
      if (buffer.readUInt32LE(0) !== lowID || buffer.readUInt32LE(4) !== highID) break;
      const valueOffset = buffer.readUInt32LE(8);
      const valueLength = buffer.readUInt32LE(12);
      if (this.#indexIsValues) {
        if (bigint) res.push((BigInt(valueOffset) + (BigInt(valueLength) << 32n)) as unknown as V);
        else res.push((valueOffset + (valueLength << 32)) as unknown as V);
      } else {
        const valSlice = this.#valueReader.subarray(valueOffset, valueOffset + valueLength);
        const valueBuf = Buffer.from(valSlice);
        res.push(JSON.parse(valueBuf.toString()) as V);
      }
      if (max !== undefined && res.length >= max) break;
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
    if (!this.#sorted) throw new Error('Not sorted, please call "switchToReadState" first');
    const lowerIndex = this.#lowerBound(key);
    if (lowerIndex >= this.#size) return false;
    const { low: lowID, high: highID } = this.#getLowHigh(key);
    const keySlice = this.#keyReader.subarray(lowerIndex * 16, lowerIndex * 16 + 8);
    const buf = Buffer.from(keySlice);
    const low = buf.readUint32LE(0);
    const high = buf.readUint32LE(4);
    return low === lowID && high === highID;
  }

  /** Switches to read state if in write. Also sort the keys. */
  async switchToReadState(): Promise<void> {
    if (this.#state === 'read') return;
    this.#state = 'read';
    closeSync(this.#keyFd);
    if (!this.#indexIsValues) closeSync(this.#valueFd);
    await this.#sort();
    this.#keyReader = mmap(`${this.fileName}.sortedkeys`);
    if (!this.#indexIsValues) this.#valueReader = mmap(`${this.fileName}.values`);
  }

  /** Sort the data */
  async #sort(): Promise<void> {
    if (this.#sorted) return;
    await externalSort([`${this.fileName}.keys`], `${this.fileName}.sortedKeys`);
    this.#sorted = true;
  }

  /**
   * @param id - the id to search for
   * @returns the starting index from the lower bound of the id
   */
  #lowerBound(id: Key): number {
    const loHiID = this.#getLowHigh(id);
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
    const key = this.#keyReader.subarray(index * 16, index * 16 + 8);
    const buf = Buffer.from(key);
    return {
      low: buf.readUint32LE(0),
      high: buf.readUint32LE(4),
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
   * @param key - the key used by the S2MMapStore
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
