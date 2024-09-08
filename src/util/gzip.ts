/** The formats available to DecompressionStream */
export type Format = 'deflate' | 'deflate-raw' | 'gzip';

/**
 * A Browser compatible Gzip decompression function
 * @param uint8Array - the data to decompress
 * @param format - the format of the data. Defaults to 'gzip'
 * @returns - the decompressed data
 */
export async function decompressGzip(
  uint8Array: Uint8Array,
  format: Format = 'gzip',
): Promise<Uint8Array> {
  // Create a DecompressionStream for 'gzip'
  const decompressionStream = new DecompressionStream(format);
  // Convert the Uint8Array to a readable stream
  const uint8ArrayStream = new ReadableStream({
    /**  @param controller - the controller for the stream */
    start(controller) {
      controller.enqueue(uint8Array);
      controller.close();
    },
  });
  // Pipe the readable stream through the decompression stream
  const decompressedStream = uint8ArrayStream.pipeThrough(decompressionStream);
  // Create a new Response object from the decompressed stream to easily retrieve the data
  const response = new Response(decompressedStream);
  // Get the decompressed data as an ArrayBuffer
  const decompressedAB = await response.arrayBuffer();

  // Convert the ArrayBuffer to a Uint8Array and return
  return new Uint8Array(decompressedAB, 0, decompressedAB.byteLength);
}
