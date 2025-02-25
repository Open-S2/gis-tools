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
  // first test if polyfill was already added
  if (globalThis.decompressionPolyfill !== undefined) {
    return globalThis.decompressionPolyfill(bytes, format);
  }

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
 * @param code - error code
 * @throws - an error
 */
function throwCode(code: number): never {
  throw new Error(`Error code: ${code}`);
}

/**
 * Decompress the LZW data
 * @param buffer - The LZW data
 * @returns - The decompressed data
 */
export function lzwDecoder(buffer: ArrayBufferLike): ArrayBufferLike {
  return decompressLZW(buffer).buffer;
}

const LZW_MIN_BITS = 9;
const LZW_CLEAR_CODE = 256; // clear code
const LZW_EOI_CODE = 257; // end of information
const LZW_MAX_BYTELENGTH = 12;

/**
 * Get a byte from an array
 * @param array - The array to read the byte from
 * @param position - The position to read the byte from
 * @param length - The length of the byte
 * @returns - The byte
 */
function lzwGetByte(array: number[] | Uint8Array, position: number, length: number): number {
  const d = position % 8;
  const a = Math.floor(position / 8);
  const de = 8 - d;
  const ef = position + length - (a + 1) * 8;
  let fg = 8 * (a + 2) - (position + length);
  const dg = (a + 2) * 8 - position;
  fg = Math.max(0, fg);
  if (a >= array.length) {
    console.warn('ran off the end of the buffer before finding LZW_EOI_CODE (end on input code)');
    return LZW_EOI_CODE;
  }
  let chunk1 = array[a] & (2 ** (8 - d) - 1);
  chunk1 <<= length - de;
  let chunks = chunk1;
  if (a + 1 < array.length) {
    let chunk2 = array[a + 1] >>> fg;
    chunk2 <<= Math.max(0, length - dg);
    chunks += chunk2;
  }
  if (ef > 8 && a + 2 < array.length) {
    const hi = (a + 3) * 8 - (position + length);
    const chunk3 = array[a + 2] >>> hi;
    chunks += chunk3;
  }
  return chunks;
}

/**
 * Append an array in reverse
 * @param dest - The array to append to
 * @param source - The array to append
 * @returns - The dest array
 */
function appendReversed(dest: number[], source: number[]): number[] {
  for (let i = source.length - 1; i >= 0; i--) dest.push(source[i]);
  return dest;
}

/**
 * Decompress the LZW data
 * @param input - The LZW data
 * @returns - The decompressed data
 */
function decompressLZW(input: ArrayBufferLike): Uint8Array {
  const dictionaryIndex = new Uint16Array(4093);
  const dictionaryChar = new Uint8Array(4093);
  for (let i = 0; i <= 257; i++) {
    dictionaryIndex[i] = 4096;
    dictionaryChar[i] = i;
  }
  let dictionaryLength = 258;
  let byteLength = LZW_MIN_BITS;
  let position = 0;

  /**
   * Initializes the dictionary
   */
  function initDictionary(): void {
    dictionaryLength = 258;
    byteLength = LZW_MIN_BITS;
  }
  /**
   * Go next
   * @param array - The array
   * @returns - The next byte
   */
  function getNext(array: number[] | Uint8Array): number {
    const byte = lzwGetByte(array, position, byteLength);
    position += byteLength;
    return byte;
  }
  /**
   * Add to the dictionary
   * @param i - The index
   * @param c - The character
   * @returns - The new length of the dictionary
   */
  function addToDictionary(i: number, c: number): number {
    dictionaryChar[dictionaryLength] = c;
    dictionaryIndex[dictionaryLength] = i;
    dictionaryLength++;
    return dictionaryLength - 1;
  }
  /**
   * Get the dictionary reversed
   * @param n - The index
   * @returns - The reversed dictionary
   */
  function getDictionaryReversed(n: number): number[] {
    const rev = [];
    for (let i = n; i !== 4096; i = dictionaryIndex[i]) {
      rev.push(dictionaryChar[i]);
    }
    return rev;
  }

  const result = [];
  initDictionary();
  const array = new Uint8Array(input);
  let code = getNext(array);
  let oldCode;
  while (code !== LZW_EOI_CODE) {
    if (code === LZW_CLEAR_CODE) {
      initDictionary();
      code = getNext(array);
      while (code === LZW_CLEAR_CODE) {
        code = getNext(array);
      }

      if (code === LZW_EOI_CODE) {
        break;
      } else if (code > LZW_CLEAR_CODE) {
        throw new Error(`corrupted code at scanline ${code}`);
      } else {
        const val = getDictionaryReversed(code);
        appendReversed(result, val);
        oldCode = code;
      }
    } else if (code < dictionaryLength) {
      const val = getDictionaryReversed(code);
      appendReversed(result, val);
      addToDictionary(oldCode ?? code, val[val.length - 1]);
      oldCode = code;
    } else {
      const oldVal = getDictionaryReversed(oldCode ?? code);
      if (oldVal.length === 0) {
        throw new Error(
          `Bogus entry. Not in dictionary, ${oldCode} / ${dictionaryLength}, position: ${position}`,
        );
      }
      appendReversed(result, oldVal);
      result.push(oldVal[oldVal.length - 1]);
      addToDictionary(oldCode ?? code, oldVal[oldVal.length - 1]);
      oldCode = code;
    }

    if (dictionaryLength + 1 >= 2 ** byteLength) {
      if (byteLength === LZW_MAX_BYTELENGTH) {
        oldCode = undefined;
      } else {
        byteLength++;
      }
    }
    code = getNext(array);
  }
  return new Uint8Array(result);
}
