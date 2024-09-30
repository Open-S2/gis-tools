import type { Stringifiable } from '..';
import type { Uint64Cell } from '../../dataStructures/uint64';

/** Represents a key-value store */
export interface KVStore<V = Stringifiable> {
  length: number;
  get: ((key: Uint64Cell) => V | undefined) | ((key: Uint64Cell) => Promise<V | undefined>);
  set: (key: Uint64Cell, value: V) => void;
}

/** Just a placeholder to explain what a local key-value store essentially is */
export class KV<V = Stringifiable> extends Map<Uint64Cell, V> implements KVStore<V> {
  /** @returns - the length of the map */
  get length(): number {
    return this.size;
  }
}
