import { compareIDs } from '../..';

import type { S2CellId } from '../..';

/** The kind of input required to store a vector for proper indexing */
export interface VectorKey {
  cell: S2CellId;
}

/** Represents a vector store or an array */
export interface VectorStore<V> {
  push: (value: V) => void;
  get: (index: number | S2CellId) => Promise<V>;
  has: (key: number | S2CellId) => boolean | Promise<boolean>;
  length: number;
  values: () => AsyncGenerator<V>;
  sort: (() => void) | (() => Promise<void>);
  [Symbol.asyncIterator]: () => AsyncGenerator<V>;
  close: () => void;
}

/** A constructor for a vector store */
export type VectorStoreConstructor<V extends VectorKey> = new () => VectorStore<V>;

/**
 * # Vector Store
 *
 * ## Description
 * A local vector store
 *
 * ## Usage
 * ```ts
 * import { Vector } from 'gis-tools-ts';
 * import type { VectorKey } from 'gis-tools-ts';
 *
 * interface Data extends VectorKey { name: string };
 *
 * const vec = new Vector<Data>();
 * // push an entry
 * vec.push({ cell: 1n, name: 'test' });
 * vec.push({ cell: 1n, name: 'test2' });
 * // check if a key exists
 * vec.has(1n); // true
 * // get length of the store
 * console.log(vec.length); // 2
 *
 * // iterate over the store
 * for await (const entry of vec) console.log(entry);
 *
 * // close the store
 * vec.close();
 * ```
 */
export class Vector<V extends VectorKey> implements VectorStore<V> {
  #store: V[] = [];

  /**
   * Push a value into the store
   * @param value - the value to store
   */
  push(value: V): void {
    this.#store.push(value);
  }

  /**
   * @param index - the position in the store to get the value from
   * @returns the value
   */
  async get(index: number | S2CellId): Promise<V> {
    return await this.#store[Number(index)];
  }

  /**
   * Check if the key exists
   * @param key - the key
   * @returns true if the key exists
   */
  has(key: number | S2CellId): boolean {
    return this.#store[Number(key)] !== undefined;
  }

  /** @returns the length of the store */
  get length(): number {
    return this.#store.length;
  }

  /**
   * iterate through the values
   * @yields an iterator
   */
  async *values(): AsyncGenerator<V> {
    for (const value of this.#store) yield value;
  }

  /** Sort the store in place */
  sort(): void {
    this.#store.sort((a, b): -1 | 0 | 1 => {
      return compareIDs(a.cell, b.cell);
    });
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.asyncIterator](): AsyncGenerator<V> {
    return this.values();
  }

  /** Closes the store */
  close(): void {
    this.#store = [];
  }
}
