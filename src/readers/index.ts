import type { Features } from '../geometry';

export * from './csv';
export * from './geotiff';
export * from './grib2';
export * from './json';
export * from './osm';
export * from './pmtiles';
export * from './protobuf';
export * from './shapefile';
export * from './wkt';
export * from './xml';
export * from './fetch';
export * from './nadgrid';

/** Reader interface. Implemented to read data from either a buffer or a filesystem */
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
  slice: (begin?: number, end?: number) => DataView<ArrayBuffer>;
  setStringEncoding: (encoding: string) => void;
  parseString: (byteOffset: number, byteLength: number) => string;
  getRange: (offset: number, length: number) => Promise<Uint8Array<ArrayBuffer>>;
}

/** Feature iteration interface. Implemented by readers to iterate over features */
export interface FeatureIterator<M = Record<string, unknown>> {
  [Symbol.asyncIterator]: () => AsyncGenerator<Features<M>>;
}

/** A buffer reader is an extension of a DataView with some extra methods */
export class BufferReader extends DataView<ArrayBuffer> implements Reader {
  textDecoder = new TextDecoder('utf-8');

  /**
   * @param buffer - the input buffer
   * @param byteOffset - offset in the buffer
   * @param byteLength - length of the buffer
   */
  constructor(buffer: ArrayBuffer, byteOffset?: number, byteLength?: number) {
    super(buffer, byteOffset, byteLength);
  }

  /**
   * @param begin - beginning of the slice
   * @param end - end of the slice. If not provided, the end of the data is used
   * @returns - a DataView of the slice
   */
  slice(begin?: number, end?: number): DataView<ArrayBuffer> {
    return new DataView(
      this.buffer.slice(this.byteOffset + (begin ?? 0), this.byteOffset + (end ?? this.byteLength)),
    );
  }

  /**
   * Set the text decoder's encoding
   * @param encoding - update the text decoder's encoding
   */
  setStringEncoding(encoding: string) {
    this.textDecoder = new TextDecoder(encoding);
  }

  /**
   * @param byteOffset - Start of the string
   * @param byteLength - Length of the string
   * @returns - The string
   */
  parseString(byteOffset: number, byteLength: number): string {
    const { textDecoder } = this;
    const data = this.slice(byteOffset, byteOffset + byteLength).buffer;
    const out = textDecoder.decode(data, { stream: true }) + textDecoder.decode();
    return out.replace(/\0/g, '').trim();
  }

  /**
   * @param offset - the offset of the range
   * @param length - the length of the range
   * @returns - the ranged buffer
   */
  async getRange(offset: number, length: number): Promise<Uint8Array<ArrayBuffer>> {
    return await new Uint8Array(this.buffer).slice(offset, offset + length);
  }
}
