import type { Key, Stringifiable } from '..';

/** Represents a key-value store */
export interface KVStore<K = Key, V = Stringifiable> {
  get: (key: K) => V | undefined;
  set: (key: K, value: V) => void;
  has: (key: K) => boolean;
}

/** Just a placeholder to explain what a local key-value store essentially is */
export class KV<K = Key, V = Stringifiable> extends Map<K, V> implements KVStore<K, V> {}
