import type { Key, Stringifiable } from '..';

/** Represents a key-value store */
export interface MultiMapStore<K = Key, V = Stringifiable> {
  get: (key: K) => V[] | undefined;
  set: (key: K, value: V) => void;
  has: (key: K) => boolean;
}

/** A local multimap key-value store */
export class MultiMap<K = Key, V = Stringifiable> implements MultiMapStore<K, V> {
  private map: Map<K, V[]>;

  /** Builds a new MultiMap */
  constructor() {
    this.map = new Map();
  }

  /**
   * Adds a value to the list of values associated with a key
   * @param key - the key
   * @param value - the value to store
   */
  set(key: K, value: V): void {
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
  get(key: K): V[] | undefined {
    return this.map.get(key);
  }

  /**
   * Check if the map contains the key
   * @param key - the key
   * @returns true if the map contains value(s) for the key
   */
  has(key: K): boolean {
    return this.map.has(key);
  }
}
