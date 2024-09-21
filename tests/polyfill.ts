// @bun

/*! MIT License. Jimmy Wärting <https://jimmy.warting.se/opensource> */
import zlib from 'node:zlib';

// fyi, Byte streams aren't really implemented anywhere yet
// It only exist as a issue: https://github.com/WICG/compression/issues/31

// TRACKER: https://github.com/oven-sh/bun/issues/1723

/**
 * @param ctx - the context
 * @param handle - the handle
 * @returns - the transform
 */
const make = (ctx, handle) =>
  Object.assign(ctx, {
    writable: new WritableStream({
      /**
       * @param chunk - input data
       * @returns - `true` if more data can be written
       */
      write: (chunk) => handle.write(chunk),
      /**
       * @returns - close the stream
       */
      close: () => handle.end(),
    }),
    readable: new ReadableStream({
      type: 'bytes',
      /**
       * @param ctrl - the controller
       */
      start(ctrl) {
        handle.on('data', (chunk) => ctrl.enqueue(chunk));
        handle.once('end', () => ctrl.close());
      },
    }),
  });

// @ts-expect-error - polyfill exception
globalThis.CompressionStream ??= class CompressionStream {
  /**
   * @param format - the format to use
   */
  constructor(format) {
    make(
      this,
      format === 'deflate'
        ? zlib.createDeflate()
        : format === 'gzip'
          ? zlib.createGzip()
          : zlib.createDeflateRaw(),
    );
  }
};

// @ts-expect-error - polyfill exception
globalThis.DecompressionStream ??= class DecompressionStream {
  /**
   * @param format - the format to use
   */
  constructor(format) {
    make(
      this,
      format === 'deflate'
        ? zlib.createInflate()
        : format === 'gzip'
          ? zlib.createGunzip()
          : zlib.createInflateRaw(),
    );
  }
};
