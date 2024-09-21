/**
 *
 */
export default class DataSlice {
  #dataView: DataView;
  #sliceOffset: number;
  #littleEndian: boolean;
  #bigTiff: boolean;

  /**
   * @param arrayBuffer
   * @param sliceOffset
   * @param littleEndian
   * @param bigTiff
   */
  constructor(
    arrayBuffer: ArrayBufferLike,
    sliceOffset: number,
    littleEndian: boolean,
    bigTiff: boolean,
  ) {
    this.#dataView = new DataView(arrayBuffer);
    this.#sliceOffset = sliceOffset;
    this.#littleEndian = littleEndian;
    this.#bigTiff = bigTiff;
  }

  /**
   *
   */
  get sliceOffset() {
    return this.#sliceOffset;
  }

  /**
   *
   */
  get sliceTop() {
    return this.#sliceOffset + this.buffer.byteLength;
  }

  /**
   *
   */
  get littleEndian() {
    return this.#littleEndian;
  }

  /**
   *
   */
  get bigTiff() {
    return this.#bigTiff;
  }

  /**
   *
   */
  get buffer() {
    return this.#dataView.buffer;
  }

  /**
   * @param offset
   * @param length
   */
  covers(offset: number, length: number): boolean {
    return this.sliceOffset <= offset && this.sliceTop >= offset + length;
  }

  /**
   * @param offset
   */
  readUint8(offset: number): number {
    return this.#dataView.getUint8(offset - this.#sliceOffset);
  }

  /**
   * @param offset
   */
  readInt8(offset: number): number {
    return this.#dataView.getInt8(offset - this.#sliceOffset);
  }

  /**
   * @param offset
   */
  readUint16(offset: number) {
    return this.#dataView.getUint16(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readInt16(offset: number) {
    return this.#dataView.getInt16(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readUint32(offset: number) {
    return this.#dataView.getUint32(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readInt32(offset: number) {
    return this.#dataView.getInt32(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readFloat32(offset: number) {
    return this.#dataView.getFloat32(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readFloat64(offset: number) {
    return this.#dataView.getFloat64(offset - this.#sliceOffset, this.#littleEndian);
  }

  /**
   * @param offset
   */
  readUint64(offset: number) {
    const left = this.readUint32(offset);
    const right = this.readUint32(offset + 4);
    let combined;
    if (this.#littleEndian) {
      combined = left + 2 ** 32 * right;
      if (!Number.isSafeInteger(combined)) {
        throw new Error(
          `${combined} exceeds MAX_SAFE_INTEGER. ` +
            'Precision may be lost. Please report if you get this message to https://github.com/geotiffjs/geotiff.js/issues',
        );
      }
      return combined;
    }
    combined = 2 ** 32 * left + right;
    if (!Number.isSafeInteger(combined)) {
      throw new Error(
        `${combined} exceeds MAX_SAFE_INTEGER. ` +
          'Precision may be lost. Please report if you get this message to https://github.com/geotiffjs/geotiff.js/issues',
      );
    }

    return combined;
  }

  /**
   * adapted from https://stackoverflow.com/a/55338384/8060591
   * @param offset
   */
  readInt64(offset: number) {
    let value = 0;
    const isNegative = (this.#dataView.getUint8(offset + (this.#littleEndian ? 7 : 0)) & 0x80) > 0;
    let carrying = true;
    for (let i = 0; i < 8; i++) {
      let byte = this.#dataView.getUint8(offset + (this.#littleEndian ? i : 7 - i));
      if (isNegative) {
        if (carrying) {
          if (byte !== 0x00) {
            byte = ~(byte - 1) & 0xff;
            carrying = false;
          }
        } else {
          byte = ~byte & 0xff;
        }
      }
      value += byte * 256 ** i;
    }
    if (isNegative) {
      value = -value;
    }
    return value;
  }

  /**
   * @param offset
   */
  readOffset(offset: number) {
    if (this.#bigTiff) {
      return this.readUint64(offset);
    }
    return this.readUint32(offset);
  }
}
