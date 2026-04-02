import { createWriteStream } from 'fs';
import { finished } from 'stream/promises';
import { open } from 'fs/promises';

import type { Writable } from 'stream'; // Needed for type annotation if desired
import type { Writer } from './index.js';

/** The File writer is to be used by bun/node/deno on the local filesystem. */
export class FileWriter implements Writer {
  #stream: Writable;
  #size = 0;
  #textEncoder = new TextEncoder();

  /** @param file - the location of the PMTiles data in the FS */
  constructor(readonly file: string) {
    this.#stream = createWriteStream(file, { flags: 'a+' }); // Open with append mode and create stream
  }

  /** @returns The current length of the file */
  tell(): number {
    return this.#size;
  }

  /**
   * Write data to the buffer
   * @param data - the data to write
   * @param offset - where in the buffer to start
   */
  async write(data: Uint8Array, offset: number): Promise<void> {
    const fd = await open(this.file, 'r+'); // Open file for reading and writing
    try {
      await fd.write(data, 0, data.length, offset); // Write at the specified offset
    } finally {
      this.#size += data.length;
      await fd.close(); // Close the file after writing
    }
  }

  /**
   * Append data to the buffer
   * @param data - the data to append
   * @returns - a promise that resolves when the data is appended
   */
  async append(data: Uint8Array): Promise<void> {
    return await new Promise((resolve, reject) => {
      this.#stream.write(data, (err) => {
        if (err instanceof Error) reject(err);
        else {
          this.#size += data.length;
          resolve();
        }
      });
    });
  }

  /**
   * Append string to the buffer synchronously
   * @param string - the string to append
   */
  async appendString(string: string): Promise<void> {
    await this.append(this.#textEncoder.encode(string));
  }

  /**
   * Append data to the buffer synchronously
   * @param data - the data to append
   */
  appendSync(data: Uint8Array): void {
    this.#stream.write(data); // Write data synchronously
    this.#size += data.length;
  }

  /**
   * Append string to the buffer synchronously
   * @param string - the string to append
   */
  appendStringSync(string: string): void {
    this.appendSync(this.#textEncoder.encode(string));
  }

  /**
   * Slice the buffer
   * @param start - the start of the slice
   * @param end - the end of the slice
   * @returns - the sliced buffer
   */
  async slice(start: number, end: number): Promise<Uint8Array> {
    const length = end - start;
    const buffer = Buffer.alloc(length);
    const fd = await open(this.file, 'r');
    try {
      await fd.read(buffer, 0, length, start);
      return new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
    } finally {
      await fd.close();
    }
  }

  /** Close the file */
  async close(): Promise<void> {
    this.#stream.end();
    await finished(this.#stream);
  }
}
