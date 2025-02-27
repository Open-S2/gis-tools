import { compareIDs } from '../..';

import type { S2CellId } from '../..';
import type { Properties, Value } from '../..';

/** A key-value entry in the multimap */
export interface MMEntry<V> {
  key: S2CellId;
  value: V[];
}

/** Represents a key-value store */
export interface MultiMapStore<V = Properties | Value> {
  length: number;
  get:
    | ((key: number | S2CellId) => V[] | undefined)
    | ((key: number | S2CellId) => Promise<V[] | undefined>);
  set: (key: number | S2CellId, value: V) => void;
  has: (key: number | S2CellId) => boolean | Promise<boolean>;
  entries: () => AsyncGenerator<MMEntry<V>>;
  [Symbol.asyncIterator]: () => AsyncGenerator<MMEntry<V>>;
  close: () => void;
}

/** A constructor for a vector store */
export type MultiMapStoreConstructor<V = Properties | Value> = new () => MultiMapStore<V>;

/**
 * # MultiMap Store
 *
 * ## Description
 * A local multimap store
 *
 * ## Usage
 * ```ts
 * import { MultiMap } from 'gis-tools-ts';
 *
 * interface Data { name: string };
 *
 * const mm = new MultiMap<Data>();
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
export class MultiMap<V = Properties | Value> implements MultiMapStore<V> {
  #store: Map<S2CellId, V[]>;
  #count = 0;

  /** Builds a new MultiMap */
  constructor() {
    this.#store = new Map();
  }

  /** @returns - the length of the map */
  get length(): number {
    return this.#count;
  }

  /**
   * Adds a value to the list of values associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: number | S2CellId, value: V): void {
    const list = this.get(key);
    if (list === undefined) {
      this.#store.set(BigInt(key), [value]);
    } else {
      list.push(value);
    }
    this.#count++;
  }

  /**
   * Check if the key exists
   * @param key - the key
   * @returns true if the key exists
   */
  has(key: number | S2CellId): boolean {
    return this.#store.has(BigInt(key));
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  get(key: number | S2CellId): V[] | undefined {
    return this.#store.get(BigInt(key));
  }

  /**
   * iterate through the values
   * @yields - The values in the store
   */
  async *entries(): AsyncGenerator<MMEntry<V>> {
    const entries = Array.from(this.#store.entries()).map(([id, value]) => [id, value]) as [
      S2CellId,
      V[],
    ][];
    entries.sort((a, b) => compareIDs(a[0], b[0]));
    for (const [key, value] of entries) {
      yield { key, value };
    }
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
    this.#store.clear();
  }
}
