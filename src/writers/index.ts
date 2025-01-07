import type { Face, Metadata } from 's2-tilejson';

export * from './pmtiles';

/** The defacto interface for all writers. */
export interface Writer {
  write(data: Uint8Array, offset: number): Promise<void>;
  append(data: Uint8Array): Promise<void>;
  appendSync(data: Uint8Array): void;
  appendString(string: string): Promise<void>;
  appendStringSync(string: string): void;
}

/** A base interface for all tile stores. */
export interface TileWriter {
  writeTileWM(zoom: number, x: number, y: number, data: Uint8Array): Promise<void>;
  writeTileS2(face: Face, zoom: number, x: number, y: number, data: Uint8Array): Promise<void>;
  commit(metadata: Metadata): Promise<void>;
}

/** Buffer writer is used on smaller datasets that are easy to write in memory. Faster then the Filesystem */
export class BufferWriter implements Writer {
  #buffer: number[] = [];
  #textEncoder = new TextEncoder();

  /**
   * Append data to the buffer
   * @param data - the data to append
   */
  async append(data: Uint8Array): Promise<void> {
    for (let i = 0; i < data.byteLength; i++) this.#buffer.push(data[i]);
    await true;
  }

  /**
   * Append string to the buffer
   * @param string - the string to append
   */
  async appendString(string: string): Promise<void> {
    await this.append(this.#textEncoder.encode(string) as Uint8Array);
  }

  /**
   * Append data to the buffer synchronously
   * @param data - the data to append
   */
  appendSync(data: Uint8Array): void {
    for (let i = 0; i < data.byteLength; i++) this.#buffer.push(data[i]);
  }

  /**
   * Append string to the buffer synchronously
   * @param string - the string to append
   */
  appendStringSync(string: string): void {
    this.appendSync(this.#textEncoder.encode(string) as Uint8Array);
  }

  /**
   * Write data to the buffer
   * @param data - the data to write
   * @param offset - where in the buffer to start
   */
  async write(data: Uint8Array, offset: number): Promise<void> {
    for (let i = 0; i < data.byteLength; i++) {
      this.#buffer[offset + i] = data[i];
    }
    await true;
  }

  /** @returns - the buffer */
  commit(): Uint8Array {
    return new Uint8Array(this.#buffer);
  }
}
