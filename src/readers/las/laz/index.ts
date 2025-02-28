import type { Reader } from '../..';

export * from './arithmeticDecoder';
export * from './constants';
export * from './integerCompressor';
export * from './items';
export * from './v1';
export * from './v2';
export * from './v3';
// export * from './v4';

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
