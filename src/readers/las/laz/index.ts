import type { Reader } from '../../index.js';

export * from './arithmeticDecoder.js';
export * from './constants.js';
export * from './integerCompressor.js';
export * from './items.js';
export * from './v1.js';
export * from './v2.js';
export * from './v3.js';
// export * from './v4.js';

/** A context of decompression */
export interface LAZContext {
  value: number;
}

/** Template for reading data */
export interface ItemReader {
  init: (item: DataView, context: LAZContext) => void;
  read: (item: DataView, context: LAZContext) => void;
  chunkSizes: (reader: Reader) => void;
}
