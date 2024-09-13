import type { Reader } from '.';

/** A buffer reader is an extension of a DataView with some extra methods */
export class BufferReader extends DataView implements Reader {
  textDecoder = new TextDecoder('utf-8');

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

  /**  @param encoding - update the text decoder's encoding */
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
}
