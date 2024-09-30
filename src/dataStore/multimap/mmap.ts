import { S2MMapStore } from '../mmap';

import type { MultiMapStore } from '.';
import type { Stringifiable } from '..';
import type { Uint64Cell } from '../../dataStructures/uint64';

/** File based multimap store */
export class FileMultiMap<V = Stringifiable> implements MultiMapStore<V> {
  #store: S2MMapStore<V>;

  /**
   * Builds a new MultiMap file store
   * @param fileName - the path + file name without the extension
   */
  constructor(fileName: string) {
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
  set(key: Uint64Cell, value: V): void {
    this.#store.set(key, value);
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  async get(key: Uint64Cell): Promise<V[] | undefined> {
    return await this.#store.get(key);
  }
}
