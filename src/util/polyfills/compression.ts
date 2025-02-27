/*! MIT License. Jimmy Wärting <https://jimmy.warting.se/opensource> */
import zlib from 'node:zlib';

import type { Duplex } from 'node:stream';
import type { Format } from '..';

// fyi, Byte streams aren't really implemented anywhere yet
// It only exist as a issue: https://github.com/WICG/compression/issues/31

// TRACKER: https://github.com/oven-sh/bun/issues/1723

/**
 * @param ctx - the context
 * @param handle - the handle
 * @returns - the transform
 */
const make = (ctx: CompressionStream | DecompressionStream, handle: Duplex) =>
  Object.assign(ctx, {
    writable: new WritableStream({
      /**
       * @param chunk - input data
       * @returns - `true` if more data can be written
       */
      write: (chunk: Uint8Array): void | Promise<void> => {
        const canWrite = handle.write(chunk);
        if (!canWrite) {
          // If the internal buffer is full, wait for 'drain' before continuing.
          return new Promise<void>((resolve) => handle.once('drain', resolve));
        }
      },
      /** closes the stream */
      close: (): void | Promise<void> => {
        handle.end();
      },
    }),
    readable: new ReadableStream({
      type: 'bytes',
      /**
       * @param ctrl - the controller
       */
      start(ctrl: ReadableByteStreamController): void {
        handle.on('data', (chunk: Uint8Array): void => ctrl.enqueue(chunk));
        handle.once('end', () => ctrl.close());
      },
    }),
  });

globalThis.CompressionStream ??= class CompressionStream {
  writable!: WritableStream;
  readable!: ReadableStream;
  /** @param format - the format to use */
  constructor(format: Format) {
    if (format === 'br') make(this, zlib.createBrotliCompress());
    else if (format === 'deflate') make(this, zlib.createDeflate());
    else if (format === 'gzip') make(this, zlib.createGzip());
    else if (format === 'deflate-raw') make(this, zlib.createDeflateRaw());
    else throw new Error(`Unsupported format: ${format}`);
  }
};

globalThis.DecompressionStream ??= class DecompressionStream {
  writable!: WritableStream;
  readable!: ReadableStream;
  /** @param format - the format to use */
  constructor(format: Format) {
    if (format === 'br') make(this, zlib.createBrotliDecompress());
    else if (format === 'deflate') make(this, zlib.createInflate());
    else if (format === 'gzip') make(this, zlib.createGunzip());
    else if (format === 'deflate-raw') make(this, zlib.createInflateRaw());
    else throw new Error(`Unsupported format: ${format}`);
  }
};
