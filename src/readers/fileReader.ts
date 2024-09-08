import { closeSync, openSync, readSync, statSync } from 'fs'; // for random access

import type { Reader } from '.';

/** Reads data from a file */
export default class FileReader implements Reader {
  #fileHandle: number;
  byteOffset: number = 0;
  byteLength: number;

  /**
   * @param file - The path to the file
   */
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
    return new DataView(slice.buffer).getBigInt64(0, littleEndian);
  }

  /**
   * Reads a 64-bit unsigned integer (biguint64) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 64-bit unsigned integer as a bigint
   */
  getBigUint64(byteOffset: number, littleEndian: boolean = false): bigint {
    const slice = this.slice(byteOffset, byteOffset + 8);
    return new DataView(slice.buffer).getBigUint64(0, littleEndian);
  }

  /**
   * Reads a 32-bit floating-point number (float32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit floating-point number as a number
   */
  getFloat32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return new DataView(slice.buffer).getFloat32(0, littleEndian);
  }

  /**
   * Reads a 64-bit floating-point number (float64) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 64-bit floating-point number as a number
   */
  getFloat64(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 8);
    return new DataView(slice.buffer).getFloat64(0, littleEndian);
  }

  /**
   * Reads a signed 16-bit integer (int16) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 16-bit signed integer value as a number
   */
  getInt16(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 2);
    return new DataView(slice.buffer).getInt16(0, littleEndian);
  }

  /**
   * Reads a signed 32-bit integer (int32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit signed integer value as a number
   */
  getInt32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return new DataView(slice.buffer).getInt32(0, littleEndian);
  }

  /**
   * Reads a signed byte (int8) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @returns The byte value as a signed number
   */
  getInt8(byteOffset: number): number {
    const slice = this.slice(byteOffset, byteOffset + 1);
    return new DataView(slice.buffer).getInt8(0);
  }

  /**
   * Reads an unsigned 16-bit integer (uint16) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 16-bit unsigned integer value as a number
   */
  getUint16(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 2);
    return new DataView(slice.buffer).getUint16(0, littleEndian);
  }

  /**
   * Reads an unsigned 32-bit integer (uint32) at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @param littleEndian - Optional, specifies if the value is stored in little-endian format. Defaults to false (big-endian).
   * @returns The 32-bit unsigned integer value as a number
   */
  getUint32(byteOffset: number, littleEndian: boolean = false): number {
    const slice = this.slice(byteOffset, byteOffset + 4);
    return new DataView(slice.buffer).getUint32(0, littleEndian);
  }

  /**
   * Reads a single byte at the given byteOffset
   * @param byteOffset - The position in the file to read from
   * @returns The byte value as a number
   */
  getUint8(byteOffset: number): number {
    const slice = this.slice(byteOffset, byteOffset + 1);
    return slice[0];
  }

  /**
   * Get a slice of the file data as Uint8Array
   * @param begin - Beginning of the slice
   * @param end - End of the slice. If not provided, the end of the data is used
   * @returns - The data as a Uint8Array
   */
  slice(begin: number, end: number): Uint8Array {
    if (begin < 0 || end > this.byteLength || begin >= end) {
      throw new RangeError('Invalid slice range');
    }
    const sliceLength = end - begin;
    const buffer = Buffer.alloc(sliceLength);
    readSync(this.#fileHandle, buffer, 0, sliceLength, begin);

    // Return the data as a Uint8Array
    return new Uint8Array(buffer);
  }

  /**
   * Closes the file
   */
  close() {
    closeSync(this.#fileHandle);
  }
}
