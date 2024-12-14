const MIN_BITS = 9;
const CLEAR_CODE = 256; // clear code
const EOI_CODE = 257; // end of information
const MAX_BYTELENGTH = 12;

/**
 * Get a byte from an array
 * @param array - The array to read the byte from
 * @param position - The position to read the byte from
 * @param length - The length of the byte
 * @returns - The byte
 */
function getByte(array: number[] | Uint8Array, position: number, length: number): number {
  const d = position % 8;
  const a = Math.floor(position / 8);
  const de = 8 - d;
  const ef = position + length - (a + 1) * 8;
  let fg = 8 * (a + 2) - (position + length);
  const dg = (a + 2) * 8 - position;
  fg = Math.max(0, fg);
  if (a >= array.length) {
    console.warn('ran off the end of the buffer before finding EOI_CODE (end on input code)');
    return EOI_CODE;
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
function decompress(input: ArrayBuffer): Uint8Array<ArrayBuffer> {
  const dictionaryIndex = new Uint16Array(4093);
  const dictionaryChar = new Uint8Array(4093);
  for (let i = 0; i <= 257; i++) {
    dictionaryIndex[i] = 4096;
    dictionaryChar[i] = i;
  }
  let dictionaryLength = 258;
  let byteLength = MIN_BITS;
  let position = 0;

  /**
   * Initializes the dictionary
   */
  function initDictionary() {
    dictionaryLength = 258;
    byteLength = MIN_BITS;
  }
  /**
   * Go next
   * @param array - The array
   * @returns - The next byte
   */
  function getNext(array: number[] | Uint8Array): number {
    const byte = getByte(array, position, byteLength);
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
  while (code !== EOI_CODE) {
    if (code === CLEAR_CODE) {
      initDictionary();
      code = getNext(array);
      while (code === CLEAR_CODE) {
        code = getNext(array);
      }

      if (code === EOI_CODE) {
        break;
      } else if (code > CLEAR_CODE) {
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
      if (byteLength === MAX_BYTELENGTH) {
        oldCode = undefined;
      } else {
        byteLength++;
      }
    }
    code = getNext(array);
  }
  return new Uint8Array(result);
}

/**
 * Decompress the LZW data
 * @param buffer - The LZW data
 * @returns - The decompressed data
 */
export function lzwDecoder(buffer: ArrayBuffer): ArrayBuffer {
  return decompress(buffer).buffer;
}
