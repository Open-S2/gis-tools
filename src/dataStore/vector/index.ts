import type { Stringifiable } from '../';

/** Represents a vector store or an array */
export interface VectorStore<V = Stringifiable> {
  push: (value: V) => void;
  get: (index: number) => V;
  sort: (compareFn?: (a: V, b: V) => number) => void;
  length: () => number;
  values: () => IterableIterator<V>;
  [Symbol.iterator]: () => IterableIterator<V>;
}

/** Just a placeholder to explain what a local key-value store essentially is */
export class Vector<V = Stringifiable> implements VectorStore<V> {
  #store: V[] = [];

  /** @param value - the value to store */
  push(value: V): void {
    this.#store.push(value);
  }

  /**
   * @param index - the position in the store to get the value from
   * @returns the value
   */
  get(index: number): V {
    return this.#store[index];
  }

  /**
   * Sort the store
   * @param compareFn - the compare function explaining how the data should be sorted
   */
  sort(compareFn?: (a: V, b: V) => number): void {
    this.#store.sort(compareFn);
  }

  /** @returns the length of the store */
  length(): number {
    return this.#store.length;
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  values(): IterableIterator<V> {
    return this.#store.values();
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.iterator](): IterableIterator<V> {
    return this.values();
  }
}
