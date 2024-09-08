export * from './bufferReader';

/** Reader interface. Used to read data from either a buffer or a filesystem */
export interface Reader {
  // Properties
  byteLength: number;
  byteOffset: number;
  // Getters
  getBigInt64: (byteOffset: number, littleEndian?: boolean) => bigint;
  getBigUint64: (byteOffset: number, littleEndian?: boolean) => bigint;
  getFloat32: (byteOffset: number, littleEndian?: boolean) => number;
  getFloat64: (byteOffset: number, littleEndian?: boolean) => number;
  getInt16: (byteOffset: number, littleEndian?: boolean) => number;
  getInt32: (byteOffset: number, littleEndian?: boolean) => number;
  getInt8: (byteOffset: number) => number;
  getUint16: (byteOffset: number, littleEndian?: boolean) => number;
  getUint32: (byteOffset: number, littleEndian?: boolean) => number;
  getUint8: (byteOffset: number) => number;
  // Methods
  slice: (begin: number, end: number) => Uint8Array;
}
