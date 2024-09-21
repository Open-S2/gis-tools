// DataView Utility functions for float32 to float16 conversion and vice versa
export {};

declare global {
  /** Extend the DataView interface to include getFloat16 and setFloat16 */
  interface DataView {
    getFloat16(byteOffset: number, littleEndian?: boolean): number;
    setFloat16(byteOffset: number, value: number, littleEndian?: boolean): void;
  }
}

/**
 * @param value
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
    return sign | 0x7c00 | (fraction ? 1 : 0);
  }

  // Normalized number
  return sign | (exponent << 10) | fraction;
}

/**
 * @param hbits
 */
function float16ToFloat32(hbits: number): number {
  const s = (hbits & 0x8000) >> 15;
  const e = (hbits & 0x7c00) >> 10;
  const f = hbits & 0x03ff;

  if (e === 0) {
    // Subnormal number
    return (s ? -1 : 1) * Math.pow(2, -14) * (f / Math.pow(2, 10));
  } else if (e === 31) {
    // NaN or Infinity
    return f ? NaN : (s ? -1 : 1) * Infinity;
  }

  // Normalized number
  return (s ? -1 : 1) * Math.pow(2, e - 15) * (1 + f / Math.pow(2, 10));
}

// Polyfill for DataView.getFloat16 and DataView.setFloat16
if (!('getFloat16' in DataView.prototype)) {
  /**
   * @param byteOffset
   * @param littleEndian
   */
  DataView.prototype.getFloat16 = function (byteOffset: number, littleEndian = false): number {
    const value = this.getUint16(byteOffset, littleEndian);
    return float16ToFloat32(value);
  };
}

if (!('setFloat16' in DataView.prototype)) {
  /**
   * @param byteOffset
   * @param value
   * @param littleEndian
   */
  DataView.prototype.setFloat16 = function (
    byteOffset: number,
    value: number,
    littleEndian = false,
  ): void {
    const float16Value = float32ToFloat16(value);
    this.setUint16(byteOffset, float16Value, littleEndian);
  };
}
