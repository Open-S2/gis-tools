/** Always a uint64. Sometimes a number is enough though */
export type Key = number | DataView;

/** Any type of value that can be stored as a string */
export type Stringifiable = string | object | number | boolean | null;

/**
 * Represents a key-value store
 */
export interface KVStore<K = Key, V = Stringifiable> {
  get: (key: K) => V | undefined;
  set: (key: K, value: V) => void;
  has: (key: K) => boolean;
}

/** Just a placeholder to explain what a local key-value store essentially is */
export class KV<K = Key, V = Stringifiable> extends Map<K, V> implements KVStore<K, V> {}
