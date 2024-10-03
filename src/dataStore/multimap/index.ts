import type { Stringifiable } from '..';
import type { Uint64 } from '../../dataStructures/uint64';

/** Represents a key-value store */
export interface MultiMapStore<V = Stringifiable> {
  length: number;
  get: ((key: Uint64) => V[] | undefined) | ((key: Uint64) => Promise<V[] | undefined>);
  set: (key: Uint64, value: V) => void;
  close: () => void;
}

/** A constructor for a vector store */
export type MultiMapStoreConstructor<V = Stringifiable> = new () => MultiMapStore<V>;

/** A local multimap key-value store */
export class MultiMap<V = Stringifiable> implements MultiMapStore<V> {
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

  /** Closes the store */
  close(): void {
    this.#store.clear();
  }
}
