import { S2MMapStore } from '../mmap';
import { compareIDs } from '../..';

import type { MMapEntry } from '../mmap';
import type { S2CellId } from '../..';
import type { MMEntry, MultiMapStore } from '.';
import type { Properties, Value } from '../..';

/**
 * # MultiMap MMap Store
 *
 * ## Description
 * A mmap multimap store
 *
 * ## Usage
 * ```ts
 * import { MMapMultiMap } from 'gis-tools-ts/mmap';
 *
 * interface Data { name: string };
 *
 * const mm = new FileMultiMap<Data>();
 * // set a key
 * mm.set(1n, { name: 'test' });
 * mm.set(1n, { name: 'test2' });
 * // get a key
 * const { name } = mm.get(1n); // [{ name: 'test' }, { name: 'test2' }]
 * // check if a key exists
 * mm.has(1n); // true
 * // get length of the store
 * console.log(mm.length); // 2
 *
 * // iterate over the store
 * for await (const entry of mm) console.log(entry);
 *
 * // close the store
 * mm.close();
 * ```
 */
export class MMapMultiMap<V = Properties | Value> implements MultiMapStore<V> {
  #store: S2MMapStore<V>;

  /**
   * Builds a new MultiMap file store
   * @param fileName - the path + file name without the extension
   */
  constructor(fileName?: string) {
    this.#store = new S2MMapStore<V>(fileName);
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
  async get(key: number | S2CellId): Promise<V[] | undefined> {
    return await this.#store.get(BigInt(key));
  }

  /**
   * iterate through the values
   * @yields - The values in the store
   */
  async *entries(): AsyncGenerator<MMEntry<V>> {
    let entries: MMapEntry<V>[] = [];
    for await (const entry of this.#store.entries()) {
      if (entries.length > 0) {
        const last = entries.at(-1)!;
        if (compareIDs(last.key, entry.key) === 0) {
          entries.push(entry);
        } else {
          yield { key: last.key, value: entries.map((e) => e.value) };
          entries = [entry];
        }
      } else {
        entries.push(entry);
      }
    }
    if (entries.length > 0) yield { key: entries[0].key, value: entries.map((e) => e.value) };
  }

  /**
   * iterate through the values
   * @returns - an iterator
   */
  [Symbol.asyncIterator](): AsyncGenerator<MMEntry<V>> {
    return this.entries();
  }

  /** Closes the store */
  close(): void {
    this.#store.close(true);
  }
}
