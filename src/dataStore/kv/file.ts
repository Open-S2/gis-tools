import { closeSync, openSync, writeSync } from 'fs';

import type { KVStore } from '.';
import type { Key, Stringifiable } from '..';

/**
 * NOTE: The File KVStore is designed to be used in states:
 * - write-only. The initial state is write-only. Write all you need to before reading
 * - read-only. Once you have written everything, the first read will lock the file to be static
 * and read-only.
 */
export class KVFile<K = Key, V = Stringifiable> implements KVStore<K, V> {
  state: 'read' | 'write' = 'read';
  // write params
  #valueOffset = 0;
  #keyWriteFd: number;
  #valueWriteFd: number;
  // read params

  /**
   * Builds a new File based KV
   * @param file - the path to the file
   */
  constructor(file: string) {
    this.#keyWriteFd = openSync(`${file}.keys`, 'a');
    this.#valueWriteFd = openSync(`${file}.values`, 'a');
  }

  /**
   * Adds a value to be associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: K, value: V): void {
    if (this.state !== 'read') throw new Error('Can no longer write to KVFile store.');
    // write key offset as a uint64
    const buffer = Buffer.alloc(8);
    if (typeof key === 'number') {
      buffer.writeUInt32LE(key & 0xffffffff, 0);
      buffer.writeUInt32LE((key >>> 32) & 0xffffffff, 4);
    } else {
      const keyBuffer = Buffer.from(
        (key as DataView).buffer,
        (key as DataView).byteOffset,
        (key as DataView).byteLength,
      );
      keyBuffer.copy(buffer, 0, 0, 8);
    }
    buffer.writeUInt32LE(this.#valueOffset & 0xffffffff, 8);
    buffer.writeUInt32LE((this.#valueOffset >>> 32) & 0xffffffff, 12);
    writeSync(this.#keyWriteFd, buffer);
    // write value and update value offset
    const valueStr = JSON.stringify(value);
    const valueBuf = Buffer.from(valueStr);
    writeSync(this.#valueWriteFd, valueBuf);
    this.#valueOffset += valueBuf.byteLength;
  }

  /**
   * Gets the value associated with a key
   * @param key - the key
   * @returns the value if the map contains values for the key
   */
  get(key: K): V | undefined {
    this.#switchToWriteState();
    return undefined;
  }

  /**
   * Check if the map contains the key
   * @param key - the key
   * @returns true if the map contains value(s) for the key
   */
  has(key: K): boolean {
    this.#switchToWriteState();
    return false;
  }

  /** Switches to write state if in read. Also sort the keys. */
  #switchToWriteState(): void {
    if (this.state === 'write') return;
    if (this.state === 'read') {
      this.state = 'write';
      closeSync(this.#keyWriteFd);
      closeSync(this.#valueWriteFd);
      // TODO: SORT KEYS
    }
  }
}
