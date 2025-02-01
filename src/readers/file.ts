import { promisify } from 'util';
import { closeSync, openSync, read, readSync, statSync } from 'fs';

import type { Reader } from '.';

export * from './tile/file';
export * from './osm/file';

const readAsync = promisify(read);

/**
 * # File Reader
 *
 * ## Description
 * Reads data from a file implementing the {@link Reader} interface
 *
 * ## Usage
 * ```ts
 * import { FileReader } from 'gis-tools/file-ts';
 *
 * const reader = new FileReader('./BETA2007.gsb');
 *
 * const data = await reader.getRange(0, 100);
 * ```
 */
export class FileReader implements Reader {
  #fileHandle: number;
  byteOffset: number = 0;
  byteLength: number;
  textDecoder = new TextDecoder('utf-8');

  /** @param file - The path to the file */
  constructor(file: string) {
    const stats = statSync(file);
    this.byteLength = stats.size;
    this.#fileHandle = openSync(file, 'r');
  }

  /**
   * Reads a 64-bit unsigned integer (biguint64) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 64-bit unsigned integer as a bigint
   */
  getBigInt64(byteOffset: number, littleEndian: boolean = false): bigint {
    const slice = this.slice(byteOffset, byteOffset + 8);
    return slice.getBigInt64(0, littleEndian);
  }

  /**
   * Reads a 64-bit unsigned integer (biguint64) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 64-bit unsigned integer as a bigint
   */
  getBigUint64(byteOffset: number, littleEndian: boolean = false): bigint {
    const slice = this.slice(byteOffset, byteOffset + 8);
    return slice.getBigUint64(0, littleEndian);
  }

  /**
   * Reads a 32-bit floating-point number (float32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit floating-point number as a number
   */
  getFloat32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return slice.getFloat32(0, littleEndian);
  }

  /**
   * Reads a 64-bit floating-point number (float64) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 64-bit floating-point number as a number
   */
  getFloat64(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 8);
    return slice.getFloat64(0, littleEndian);
  }

  /**
   * Reads a signed 16-bit integer (int16) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 16-bit signed integer value as a number
   */
  getInt16(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 2);
    return slice.getInt16(0, littleEndian);
  }

  /**
   * Reads a signed 32-bit integer (int32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit signed integer value as a number
   */
  getInt32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return slice.getInt32(0, littleEndian);
  }

  /**
   * Reads a signed byte (int8) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @returns The byte value as a signed number
   */
  getInt8(byteOffset: number): number {
    const slice = this.slice(byteOffset, byteOffset + 1);
    return slice.getInt8(0);
  }

  /**
   * Reads an unsigned 16-bit integer (uint16) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 16-bit unsigned integer value as a number
   */
  getUint16(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 2);
    return slice.getUint16(0, littleEndian);
  }

  /**
   * Reads an unsigned 32-bit integer (uint32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit unsigned integer value as a number
   */
  getUint32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return slice.getUint32(0, littleEndian);
  }

  /**
   * Reads a single byte at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @returns The byte value as a number
   */
  getUint8(byteOffset: number): number {
    const slice = this.slice(byteOffset, byteOffset + 1);
    return slice.getUint8(0);
  }

  /**
   * Get a slice of the file data as DataView
   * @param begin - Beginning of the slice
   * @param end - End of the slice. If not provided, the end of the data is used
   * @returns - The data as a DataView
   */
  slice(begin?: number, end?: number): DataView {
    if (begin === undefined) begin = 0;
    if (end === undefined) end = this.byteLength;
    if (begin < 0 || end > this.byteLength || begin >= end) {
      throw new RangeError('Invalid slice range');
    }
    const sliceLength = end - begin;
    const buffer = Buffer.alloc(sliceLength);
    readSync(this.#fileHandle, buffer, 0, sliceLength, begin);

    return new DataView(buffer.buffer, buffer.byteOffset, buffer.byteLength);
  }

  /**
   * Set the text decoder's encoding
   * @param encoding - update the text decoder's encoding
   */
  setStringEncoding(encoding: string) {
    this.textDecoder = new TextDecoder(encoding);
  }

  /**
   * Reads a string from the file
   * @param byteOffset - Start of the string
   * @param byteLength - Length of the string
   * @returns - The string
   */
  parseString(byteOffset: number = 0, byteLength: number = this.byteLength): string {
    const { textDecoder } = this;
    const data = this.slice(byteOffset, byteOffset + byteLength).buffer;
    const out = textDecoder.decode(data as ArrayBuffer, { stream: true }) + textDecoder.decode();
    return out.replace(/\0/g, '').trim();
  }

  /**
   * Reads a range from the file
   * @param offset - the offset of the range
   * @param length - the length of the range
   * @returns - the ranged buffer
   */
  async getRange(offset: number, length: number): Promise<Uint8Array> {
    const buffer = Buffer.alloc(length);
    await readAsync(this.#fileHandle, buffer, 0, length, offset);
    return new Uint8Array(buffer.buffer, 0, length);
  }

  /** Closes the file */
  close() {
    closeSync(this.#fileHandle);
  }
}
