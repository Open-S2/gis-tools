import type { Reader } from '.';

/** A buffer reader is an extension of a DataView with some extra methods */
export class BufferReader extends DataView implements Reader {
  /**
   * @param buffer - the input buffer
   * @param byteOffset - offset in the buffer
   * @param byteLength - length of the buffer
   */
  constructor(
    buffer: ArrayBufferLike & {
      BYTES_PER_ELEMENT?: never;
    },
    byteOffset?: number,
    byteLength?: number,
  ) {
    super(buffer, byteOffset, byteLength);
  }

  /**
   * @param begin - beginning of the slice
   * @param end - end of the slice. If not provided, the end of the data is used
   * @returns - a DataView of the slice
   */
  slice(begin: number, end: number): DataView {
    return new DataView(
      this.buffer.slice(this.byteOffset + begin, this.byteOffset + (end ?? this.byteLength)),
    );
  }
}
