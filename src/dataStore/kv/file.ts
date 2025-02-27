import { S2FileStore } from '../file';

import type { KVStore } from '.';
import type { S2CellId } from '../..';
import type { Properties, Value } from '../..';

/**
 * # Key-Value File Store
 *
 * ## Description
 * A filesystem key-value store
 *
 * ## Usage
 * ```ts
 * import { FileKV } from 'gis-tools-ts/file';
 *
 * interface Data { name: string };
 *
 * const kv = new FileKV<Data>('./test.kv');
 * // set a key
 * kv.set(1n, { name: 'test' });
 * // get a key
 * const { name } = kv.get(1n); // { name: 'test' }
 * // check if a key exists
 * kv.has(1n); // true
 * // get length of the store
 * console.log(kv.length); // 1
 *
 * // iterate over the store
 * for await (const value of kv) console.log(value);
 *
 * // close the store
 * kv.close();
 * ```
 */
export class FileKV<V = Properties | Value> implements KVStore<V> {
  #store: S2FileStore<V>;

  /**
   * Builds a new MultiMap file store
   * @param fileName - the path + file name without the extension
   */
  constructor(fileName?: string) {
    this.#store = new S2FileStore<V>(fileName);
  }

  /** @returns - the length of the map */
  get length(): number {
    return this.#store.length;
  }

  /**
   * Adds a value to the list of values associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: number | S2CellId, value: V): void {
    this.#store.set(BigInt(key), value);
  }

  /**
   * Check if the key exists
   * @param key - the key
   * @returns true if the key exists
   */
  async has(key: number | S2CellId): Promise<boolean> {
    return await this.#store.has(BigInt(key));
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  async get(key: number | S2CellId): Promise<V | undefined> {
    const value = await this.#store.get(BigInt(key), 0);
    if (value === undefined) return undefined;
    return value[0] as V;
  }

  /**
   * iterate through the values
   * @yields an iterator
   */
  async *values(): AsyncGenerator<V> {
    for await (const { value } of this.#store.entries()) yield value;
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.asyncIterator](): AsyncGenerator<V> {
    return this.values();
  }

  /** Closes the store */
  close(): void {
    this.#store.close(true);
  }
}
