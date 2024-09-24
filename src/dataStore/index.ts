import type { S2CellId } from '..';
import type { Uint64Cell } from '../wasm/uint64';

export * from './kv';
export * from './multimap';

/** Always a uint64. Sometimes a number is enough though */
export type Key = number | S2CellId | Uint64Cell;

/** Any type of value that can be stored as a string */
export type Stringifiable = string | object | number | boolean | null;
