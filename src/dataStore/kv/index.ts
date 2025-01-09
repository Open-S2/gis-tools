import type { Uint64 } from '../../dataStructures/uint64';
import type { Properties, Value } from '../..';

/** Represents a key-value store */
export interface KVStore<V = Properties | Value> {
  length: number;
  get: ((key: Uint64) => V | undefined) | ((key: Uint64) => Promise<V | undefined>);
  set: (key: Uint64, value: V) => void;
  values: () => AsyncGenerator<V>;
  [Symbol.asyncIterator]: () => AsyncGenerator<V>;
  close: () => void;
}

/** A constructor for a vector store */
export type KVStoreConstructor<V = Properties | Value> = new (fileName?: string) => KVStore<V>;

/** Just a placeholder to explain what a local key-value store essentially is */
export class KV<V = Properties | Value> implements KVStore<V> {
  #store = new Map<Uint64, V>();
  /** @returns - the length of the map */
  get length(): number {
    return this.#store.size;
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  get(key: Uint64): V | undefined {
    return this.#store.get(key);
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
