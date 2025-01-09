import { compare, toCell } from '../../dataStructures/uint64';

import type { Properties, Value } from '../..';
import type { Uint64, Uint64Cell } from '../../dataStructures/uint64';

/** A key-value entry in the multimap */
export interface MMEntry<V> {
  key: Uint64;
  value: V[];
}

/** Represents a key-value store */
export interface MultiMapStore<V = Properties | Value> {
  length: number;
  get: ((key: Uint64) => V[] | undefined) | ((key: Uint64) => Promise<V[] | undefined>);
  set: (key: Uint64, value: V) => void;
  entries: () => AsyncGenerator<MMEntry<V>>;
  [Symbol.asyncIterator]: () => AsyncGenerator<MMEntry<V>>;
  close: () => void;
}

/** A constructor for a vector store */
export type MultiMapStoreConstructor<V = Properties | Value> = new () => MultiMapStore<V>;

/** A local multimap key-value store */
export class MultiMap<V = Properties | Value> implements MultiMapStore<V> {
  #store: Map<Uint64, V[]>;
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
  set(key: Uint64, value: V): void {
    const list = this.get(key);
    if (list === undefined) {
      this.#store.set(key, [value]);
    } else {
      list.push(value);
    }
    this.#count++;
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  get(key: Uint64): V[] | undefined {
    return this.#store.get(key);
  }

  /**
   * iterate through the values
   * @yields - The values in the store
   */
  async *entries(): AsyncGenerator<MMEntry<V>> {
    const entries = Array.from(this.#store.entries()).map(([id, value]) => [toCell(id), value]) as [
      Uint64Cell,
      V[],
    ][];
    entries.sort((a, b) => compare(a[0], b[0]));
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
