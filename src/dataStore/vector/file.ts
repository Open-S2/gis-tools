import { S2FileStore } from '../file';

import type { S2CellId } from '../..';
import type { VectorKey, VectorStore } from '.';

/**
 * # Vector File Store
 *
 * ## Description
 * A filesystem vector store
 *
 * ## Usage
 * ```ts
 * import { FileVector } from 'gis-tools-ts/file';
 * import type { VectorKey } from 'gis-tools-ts';
 *
 * interface Data extends VectorKey { name: string };
 *
 * const vec = new FileVector<Data>();
 * // push an entry
 * vec.push({ cell: 1n, name: 'test' });
 * vec.push({ cell: 1n, name: 'test2' });
 * // check if a key exists
 * vec.has(1n); // true
 * // get length of the store
 * console.log(vec.length); // 2
 *
 * // iterate over the store
 * for await (const entry of vec) console.log(entry);
 *
 * // close the store
 * vec.close();
 * ```
 */
export class FileVector<V extends VectorKey> implements VectorStore<V> {
  #store: S2FileStore;
  /** @param fileName - the path + file name without the extension */
  constructor(fileName?: string) {
    this.#store = new S2FileStore<V>(fileName);
  }

  /** @returns the length of the store */
  get length(): number {
    return this.#store.length;
  }

  /**
   * Push a value into the store
   * @param value - the value to store
   */
  push(value: V): void {
    this.#store.set(value.cell, value);
  }

  /**
   * @param index - the position in the store to get the value from
   * @returns the value
   */
  async get(index: number | S2CellId): Promise<V> {
    const value = await this.#store.get(BigInt(index));
    if (value === undefined) throw new Error('Value not found');
    return value[0] as V;
  }

  /**
   * Check if the key exists
   * @param key - the key
   * @returns true if the key exists
   */
  async has(key: number | S2CellId): Promise<boolean> {
    return await this.#store.has(BigInt(key));
  }

  /** Sort the store */
  async sort(): Promise<void> {
    await this.#store.sort();
  }

  /**
   * iterate through the values
   * @yields an iterator
   */
  async *values(): AsyncGenerator<V> {
    for await (const { value } of this.#store.entries()) yield value as V;
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
