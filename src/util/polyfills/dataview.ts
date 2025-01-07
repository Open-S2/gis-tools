/* eslint-disable @typescript-eslint/no-explicit-any */
// DataView Utility functions for float16
export {};

declare global {
  /** Extend the DataView interface */
  interface DataView {
    // byteLength: number;
    // byteOffset: number;
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
    /**
     * Retrieves a 16-bit floating point number (Float16) at the specified byte offset from the start of the view.
     * This method reads two bytes from the buffer, converts them into a 16-bit floating-point number,
     * and returns the corresponding 32-bit floating-point representation.
     * @param byteOffset - The offset, in bytes, from the start of the DataView to read the value from.
     * @param littleEndian - If true, the value is read as little-endian. Otherwise, it's read as big-endian.
     * @returns The converted 32-bit floating-point number (Float32) corresponding to the 16-bit value.
     */
    getFloat16(byteOffset: number, littleEndian?: boolean): number;
    /**
     * Stores a 16-bit floating point number (Float16) at the specified byte offset in the DataView.
     * This method converts a 32-bit floating-point number (Float32) to a 16-bit floating-point representation,
     * then writes the resulting 16-bit value into the buffer at the specified offset.
     * @param byteOffset - The offset, in bytes, at which to store the value.
     * @param value - The 32-bit floating-point number (Float32) to be converted and stored as Float16.
     * @param littleEndian - If true, the value is stored as little-endian. Otherwise, it's stored as big-endian.
     */
    setFloat16(byteOffset: number, value: number, littleEndian?: boolean): void;
  }
}

/**
 * Converts a 32-bit floating point number to a 16-bit floating point number
 * @param value - the float 32 value
 * @returns - the float 16 value
 */
function float32ToFloat16(value: number): number {
  const floatView = new Float32Array(1);
  const int32View = new Uint32Array(floatView.buffer);
  floatView[0] = value;

  const sign = (int32View[0] >> 16) & 0x8000;
  const exponent = ((int32View[0] >> 23) & 0xff) - 112;
  let fraction = (int32View[0] >> 13) & 0x03ff;

  if (exponent <= 0) {
    // Subnormal numbers or zero
    if (exponent < -10) return sign; // Underflow to zero
    fraction = (fraction | 0x0400) >> (1 - exponent);
    return sign | fraction;
  } else if (exponent === 143) {
    // NaN or Infinity
    return sign | 0x7c00 | (fraction > 0 ? 1 : 0);
  }

  // Normalized number
  return sign | (exponent << 10) | fraction;
}

/**
 * Converts a 16-bit floating point number to a 32-bit floating point number
 * @param hbits - float 16 bits
 * @returns - float32
 */
function float16ToFloat32(hbits: number): number {
  const s = (hbits & 0x8000) >> 15;
  const e = (hbits & 0x7c00) >> 10;
  const f = hbits & 0x03ff;

  if (e === 0) {
    // Subnormal number
    return (s > 0 ? -1 : 1) * Math.pow(2, -14) * (f / Math.pow(2, 10));
  } else if (e === 31) {
    // NaN or Infinity
    return f > 0 ? NaN : (s > 0 ? -1 : 1) * Infinity;
  }

  // Normalized number
  return (s > 0 ? -1 : 1) * Math.pow(2, e - 15) * (1 + f / Math.pow(2, 10));
}

// Polyfill for DataView.getFloat16
if (!('getFloat16' in DataView.prototype) || process.env.FORCE_POLYFILL !== undefined) {
  /**
   * Retrieves a 16-bit floating point number (Float16) at the specified byte offset from the start of the view.
   * This method reads two bytes from the buffer, converts them into a 16-bit floating-point number,
   * and returns the corresponding 32-bit floating-point representation.
   * @param byteOffset - The offset, in bytes, from the start of the DataView to read the value from.
   * @param littleEndian - If true, the value is read as little-endian. Otherwise, it's read as big-endian.
   * @returns The converted 32-bit floating-point number (Float32) corresponding to the 16-bit value.
   */
  (DataView.prototype as any).getFloat16 = function (
    byteOffset: number,
    littleEndian = false,
  ): number {
    const value = this.getUint16(byteOffset, littleEndian);
    return float16ToFloat32(value);
  };
}

// Polyfill for DataView.setFloat16
if (!('setFloat16' in DataView.prototype) || process.env.FORCE_POLYFILL !== undefined) {
  /**
   * Stores a 16-bit floating point number (Float16) at the specified byte offset in the DataView.
   * This method converts a 32-bit floating-point number (Float32) to a 16-bit floating-point representation,
   * then writes the resulting 16-bit value into the buffer at the specified offset.
   * @param byteOffset - The offset, in bytes, at which to store the value.
   * @param value - The 32-bit floating-point number (Float32) to be converted and stored as Float16.
   * @param littleEndian - If true, the value is stored as little-endian. Otherwise, it's stored as big-endian.
   */
  (DataView.prototype as any).setFloat16 = function (
    byteOffset: number,
    value: number,
    littleEndian = false,
  ): void {
    const float16Value = float32ToFloat16(value);
    this.setUint16(byteOffset, float16Value, littleEndian);
  };
}
