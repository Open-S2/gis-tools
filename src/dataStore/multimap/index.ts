import type { Stringifiable } from '..';
import type { Uint64Cell } from '../../dataStructures/uint64';

/** Represents a key-value store */
export interface MultiMapStore<V = Stringifiable> {
  length: number;
  get: ((key: Uint64Cell) => V[] | undefined) | ((key: Uint64Cell) => Promise<V[] | undefined>);
  set: (key: Uint64Cell, value: V) => void;
}

/** A local multimap key-value store */
export class MultiMap<V = Stringifiable> implements MultiMapStore<V> {
  private map: Map<Uint64Cell, V[]>;

  /** Builds a new MultiMap */
  constructor() {
    this.map = new Map();
  }

  /** @returns - the length of the map */
  get length(): number {
    return this.map.size;
  }

  /**
   * Adds a value to the list of values associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: Uint64Cell, value: V): void {
    const list = this.get(key);
    if (list === undefined) {
      this.map.set(key, [value]);
    } else {
      list.push(value);
    }
  }

  /**
   * Gets the list of values associated with a key
   * @param key - the key
   * @returns the list of values if the map contains values for the key
   */
  get(key: Uint64Cell): V[] | undefined {
    return this.map.get(key);
  }
}
