import { S2MMapStore } from '../mmap';
import { compare } from '../../dataStructures/uint64';

import type { MMapEntry } from '../mmap';
import type { Stringifiable } from '..';
import type { Uint64 } from '../../dataStructures/uint64';
import type { MMEntry, MultiMapStore } from '.';

/** MMap based multimap store */
export class MMapMultiMap<V = Stringifiable> implements MultiMapStore<V> {
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
  set(key: Uint64, value: V): void {
    this.#store.set(key, value);
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  async get(key: Uint64): Promise<V[] | undefined> {
    return await this.#store.get(key);
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
        if (compare(last.key, entry.key) === 0) {
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
