import { concatUint8Arrays } from '.';

import type { Format } from '..';

/**
 * A Browser compatible Gzip compression function
 * @param bytes - the data to decompress
 * @param format - the format of the data. Defaults to 'gzip'
 * @returns - the decompressed data
 */
export async function compressStream(bytes: Uint8Array, format?: Format): Promise<Uint8Array> {
  // Convert the string to a byte stream.
  const stream = new Blob([bytes]).stream();
  // Create a compressed stream.
  const compressedStream = stream.pipeThrough(new CompressionStream((format ?? 'gzip') as 'gzip'));
  // Read all the bytes from this stream.
  const chunks = [];
  for await (const chunk of compressedStream) chunks.push(chunk);

  return await concatUint8Arrays(chunks);
}

/**
 * A Browser compatible Gzip decompression function
 * @param bytes - the data to decompress
 * @param format - the format of the data. Defaults to 'gzip'
 * @returns - the decompressed data
 */
export async function decompressStream(bytes: Uint8Array, format?: Format): Promise<Uint8Array> {
  if (format === undefined) format = bytes[0] === 0x1f && bytes[1] === 0x8b ? 'gzip' : 'deflate';
  // Convert the bytes to a stream.
  const stream = new Blob([bytes]).stream();

  // Create a decompressed stream.
  const decompressedStream = stream.pipeThrough(new DecompressionStream(format as 'gzip'));
  // Read all the bytes from this stream.
  const chunks = [];
  for await (const chunk of decompressedStream) chunks.push(chunk);

  return await concatUint8Arrays(chunks);
}

/**
 * Find the end of the central directory
 * @param raw - the raw data
 * @returns - the end of the central directory
 */
function findEndCentralDirectory(raw: Uint8Array): number {
  let search = raw.length - 20;
  const bounds = Math.max(search - 65516, 2); // sub 2**256 - 20 (max comment length)

  while (
    (search = raw.lastIndexOf(0x50, search - 1)) !== -1 &&
    !(raw[search + 1] === 0x4b && raw[search + 2] === 0x05 && raw[search + 3] === 0x06) &&
    search > bounds
  ) {
    // no-op
  }

  return search;
}

/**
 * Each {@link ZipItem} can have its decompressed bytes read by calling its `read()` method,
 * which'll give either `Uint8Array` _or_ `Promise<Uint8Array>` (as it's possible to be sync).
 */
export interface ZipItem {
  filename: string;
  comment: string;
  read: () => Promise<Uint8Array> | Uint8Array;
}

/**
 * Iterate over the items in a zip file
 * @param raw - the raw data to read
 * @yields - {@link ZipItem}
 */
export function* iterItems(raw: Uint8Array): Generator<ZipItem, void, void> {
  const d = new TextDecoder();
  let at = findEndCentralDirectory(raw);
  if (at === -1) {
    throwCode(2); // bad zip format
  }

  /**
   * @param startBy - starting index
   * @param endBy - ending index
   * @returns - the subarray
   */
  const subarrayMove = (startBy: number, endBy: number): Uint8Array =>
    raw.subarray((at += startBy), (at += endBy));

  const dataView = new DataView(raw.buffer, raw.byteOffset); // we don't need byteLength, could be a longer buffer :shrug:

  /**
   * @param offset - the offset to read from
   * @returns - the u16 value
   */
  const u16 = (offset: number) => dataView.getUint16(offset + at, true);
  /**
   * @param offset - the offset to read from
   * @returns - the u32 value
   */
  const u32 = (offset: number) => dataView.getUint32(offset + at, true);

  // read end central directory
  let fileCount = u16(10);
  if (fileCount !== u16(8)) {
    throwCode(3); // no multi-disk support
  }
  const centralDirectoryStart = u32(16);
  at = centralDirectoryStart;

  // read central directory
  while (fileCount-- > 0) {
    const compressionMethod = u16(10);
    const filenameLength = u16(28);
    const extraFieldsLength = u16(30);
    const commentLength = u16(32);
    const compressedSize = u32(20);

    // find local entry location
    const localEntryAt = u32(42);

    // read buffers, move at to after entry, and store where we were
    const filename = d.decode(subarrayMove(46, filenameLength));
    // we skip extraFields here
    const comment = d.decode(subarrayMove(extraFieldsLength, commentLength));
    const nextCentralDirectoryEntry = at;

    // >> start reading entry
    at = localEntryAt;

    // this is the local entry (filename + extra fields) length, which we skip
    const bytes = subarrayMove(30 + u16(26) + u16(28), compressedSize);

    yield {
      filename,
      comment,
      /**
       * @returns - the decompressed bytes
       */
      read: async () => {
        return (compressionMethod & 8) > 0
          ? await decompressStream(bytes, 'deflate-raw')
          : compressionMethod > 0
            ? throwCode(1)
            : bytes;
      },
    };

    at = nextCentralDirectoryEntry;
  }
}

/**
 * @param code - error code
 * @throws - an error
 */
function throwCode(code: number): never {
  throw new Error(`Error code: ${code}`);
}
