import { createWriteStream } from 'fs';
import { open } from 'fs/promises';

import type { Writable } from 'stream'; // Needed for type annotation if desired
import type { Writer } from '.';

/** The File writer is to be used by bun/node/deno on the local filesystem. */
export class FileWriter implements Writer {
  #stream: Writable;
  #textEncoder = new TextEncoder();

  /** @param file - the location of the PMTiles data in the FS */
  constructor(readonly file: string) {
    this.#stream = createWriteStream(file, { flags: 'a+' }); // Open with append mode and create stream
  }

  /**
   * @param data - the data to write
   * @param offset - where in the buffer to start
   */
  async write(data: Uint8Array, offset: number): Promise<void> {
    const fd = await open(this.file, 'r+'); // Open file for reading and writing
    try {
      await fd.write(data, 0, data.length, offset); // Write at the specified offset
    } finally {
      await fd.close(); // Close the file after writing
    }
  }

  /**
   * @param data - the data to append
   * @returns - a promise that resolves when the data is appended
   */
  async append(data: Uint8Array): Promise<void> {
    return await new Promise((resolve, reject) => {
      this.#stream.write(data, (err) => {
        if (err instanceof Error) reject(err);
        else resolve();
      });
    });
  }

  /** @param string - the string to append */
  async appendString(string: string): Promise<void> {
    await this.append(this.#textEncoder.encode(string));
  }

  /** @param data - the data to append */
  appendSync(data: Uint8Array): void {
    this.#stream.write(data); // Write data synchronously
  }

  /** @param string - the string to append */
  appendStringSync(string: string): void {
    this.appendSync(this.#textEncoder.encode(string));
  }
}
