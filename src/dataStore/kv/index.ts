import type { S2CellId } from '../..';
import type { Properties, Value } from '../..';

/** Represents a key-value store */
export interface KVStore<V = Properties | Value> {
  length: number;
  get:
    | ((key: number | S2CellId) => V | undefined)
    | ((key: number | S2CellId) => Promise<V | undefined>);
  set: (key: number | S2CellId, value: V) => void;
  has: (key: number | S2CellId) => boolean | Promise<boolean>;
  values: () => AsyncGenerator<V>;
  [Symbol.asyncIterator]: () => AsyncGenerator<V>;
  close: () => void;
}

/** A constructor for a vector store */
export type KVStoreConstructor<V = Properties | Value> = new (fileName?: string) => KVStore<V>;

/**
 * # Key-Value Store
 *
 * ## Description
 * A local key-value store
 *
 * ## Usage
 * ```ts
 * import { KV } from 'gis-tools-ts';
 *
 * interface Data { name: string };
 *
 * const kv = new KV<Data>();
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
 * // clear the store
 * kv.close();
 * ```
 */
export class KV<V = Properties | Value> implements KVStore<V> {
  #store = new Map<S2CellId, V>();
  /** @returns - the length of the map */
  get length(): number {
    return this.#store.size;
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  get(key: number | S2CellId): V | undefined {
    return this.#store.get(BigInt(key));
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
   * Adds a value to the list of values associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: number | S2CellId, value: V): void {
    this.#store.set(BigInt(key), value);
  }

  /**
   * iterate through the values
   * @yields an iterator
   */
  async *values(): AsyncGenerator<V> {
    for await (const [, value] of this.#store.entries()) yield value;
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.asyncIterator](): AsyncGenerator<V> {
    return this.values();
  }

  /**
   * Closes the store
   */
  close(): void {
    this.#store.clear();
  }
}
