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
  entries: () => AsyncGenerator<MMEntry<V>>;
  [Symbol.asyncIterator]: () => AsyncGenerator<MMEntry<V>>;
  close: () => void;
}

/** A constructor for a vector store */
export type MultiMapStoreConstructor<V = Properties | Value> = new () => MultiMapStore<V>;

/** A local multimap key-value store */
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
