/** Types of outputs */
export type ArrayTypes =
  | Uint8ClampedArray
  | Uint8Array
  | Uint16Array
  | Uint32Array
  | Int8Array
  | Int16Array
  | Int32Array
  | Float32Array
  | Float64Array;
/** Types of output constructors */
export type ArrayTypesConstructors =
  | Uint8ClampedArrayConstructor
  | Uint8ArrayConstructor
  | Uint16ArrayConstructor
  | Uint32ArrayConstructor
  | Int8ArrayConstructor
  | Int16ArrayConstructor
  | Int32ArrayConstructor
  | Float32ArrayConstructor
  | Float64ArrayConstructor;

/**
 * Convert the data format and bits per sample to an array type
 * @param format - the data format
 * @param bitsPerSample - the bits per sample
 * @returns the array type constructor
 */
function arrayType(format: number, bitsPerSample: number): ArrayTypesConstructors {
  switch (format) {
    case 1: // unsigned integer data
      if (bitsPerSample <= 8) {
        return Uint8Array;
      } else if (bitsPerSample <= 16) {
        return Uint16Array;
      } else if (bitsPerSample <= 32) {
        return Uint32Array;
      }
      break;
    case 2: // twos complement signed integer data
      if (bitsPerSample === 8) {
        return Int8Array;
      } else if (bitsPerSample === 16) {
        return Int16Array;
      } else if (bitsPerSample === 32) {
        return Int32Array;
      }
      break;
    case 3: // floating point data
      switch (bitsPerSample) {
        case 16:
        case 32:
          return Float32Array;
        case 64:
          return Float64Array;
        default:
          break;
      }
      break;
    default:
      break;
  }
  throw Error('Unsupported data format/bitsPerSample');
}

/**
 * Convert the data format and bits per sample to an array type
 * @param raster - the data
 * @param format - the data format
 * @param bitsPerSample - the bits per sample
 * @returns - the array
 */
export function toArrayType(raster: number[], format: number, bitsPerSample: number): ArrayTypes {
  const constructor = arrayType(format, bitsPerSample);
  return new constructor(raster);
}

/**
 * Create an array of the right type
 * @param format - the data format
 * @param bitsPerSample - the bits per sample
 * @param size - the size
 * @returns - the array
 */
export function arrayForType(format: number, bitsPerSample: number, size: number): ArrayTypes {
  const constructor = arrayType(format, bitsPerSample);
  return new constructor(size);
}

/**
 * @param array - An array of numbers
 * @param start - Start index
 * @param end - End index
 * @returns The sum
 */
export function sampleSum(array: number[], start: number, end: number): number {
  let s = 0;
  for (let i = start; i < end; ++i) s += array[i];
  return s;
}

/**
 * Check if the data needs normalization
 * @param format - the data format
 * @param bitsPerSample - the bits per sample
 * @returns - true if the data needs normalization
 */
export function needsNormalization(format: number, bitsPerSample: number): boolean {
  if ((format === 1 || format === 2) && bitsPerSample <= 32 && bitsPerSample % 8 === 0) {
    return false;
  } else if (
    format === 3 &&
    (bitsPerSample === 16 || bitsPerSample === 32 || bitsPerSample === 64)
  ) {
    return false;
  }
  return true;
}

/**
 * Normalize the array
 * @param inBuffer - the input buffer
 * @param format - the data format
 * @param planarConfiguration - the planar configuration
 * @param samplesPerPixel - the number of samples per pixel
 * @param bitsPerSample - the bits per sample
 * @param tileWidth - the tile width
 * @param tileHeight - the tile height
 * @returns - the normalized array
 */
export function normalizeArray(
  inBuffer: ArrayBufferLike,
  format: number,
  planarConfiguration: number,
  samplesPerPixel: number,
  bitsPerSample: number,
  tileWidth: number,
  tileHeight: number,
): ArrayBufferLike {
  // const inByteArray = new Uint8Array(inBuffer);
  const view = new DataView(inBuffer);
  const outSize =
    planarConfiguration === 2 ? tileHeight * tileWidth : tileHeight * tileWidth * samplesPerPixel;
  const samplesToTransfer = planarConfiguration === 2 ? 1 : samplesPerPixel;
  const outArray = arrayForType(format, bitsPerSample, outSize);
  // let pixel = 0;

  const bitMask = parseInt('1'.repeat(bitsPerSample), 2);

  if (format === 1) {
    // unsigned integer
    // translation of https://github.com/OSGeo/gdal/blob/master/gdal/frmts/gtiff/geotiff.cpp#L7337
    let pixelBitSkip;
    // let sampleBitOffset = 0;
    if (planarConfiguration === 1) {
      pixelBitSkip = samplesPerPixel * bitsPerSample;
      // sampleBitOffset = (samplesPerPixel - 1) * bitsPerSample;
    } else {
      pixelBitSkip = bitsPerSample;
    }

    // Bits per line rounds up to next byte boundary.
    let bitsPerLine = tileWidth * pixelBitSkip;
    if ((bitsPerLine & 7) !== 0) {
      bitsPerLine = (bitsPerLine + 7) & ~7;
    }

    for (let y = 0; y < tileHeight; ++y) {
      const lineBitOffset = y * bitsPerLine;
      for (let x = 0; x < tileWidth; ++x) {
        const pixelBitOffset = lineBitOffset + x * samplesToTransfer * bitsPerSample;
        for (let i = 0; i < samplesToTransfer; ++i) {
          const bitOffset = pixelBitOffset + i * bitsPerSample;
          const outIndex = (y * tileWidth + x) * samplesToTransfer + i;

          const byteOffset = Math.floor(bitOffset / 8);
          const innerBitOffset = bitOffset % 8;
          if (innerBitOffset + bitsPerSample <= 8) {
            outArray[outIndex] =
              (view.getUint8(byteOffset) >> (8 - bitsPerSample - innerBitOffset)) & bitMask;
          } else if (innerBitOffset + bitsPerSample <= 16) {
            outArray[outIndex] =
              (view.getUint16(byteOffset) >> (16 - bitsPerSample - innerBitOffset)) & bitMask;
          } else if (innerBitOffset + bitsPerSample <= 24) {
            const raw = (view.getUint16(byteOffset) << 8) | view.getUint8(byteOffset + 2);
            outArray[outIndex] = (raw >> (24 - bitsPerSample - innerBitOffset)) & bitMask;
          } else {
            outArray[outIndex] =
              (view.getUint32(byteOffset) >> (32 - bitsPerSample - innerBitOffset)) & bitMask;
          }

          // let outWord = 0;
          // for (let bit = 0; bit < bitsPerSample; ++bit) {
          //   if (inByteArray[bitOffset >> 3]
          //     & (0x80 >> (bitOffset & 7))) {
          //     outWord |= (1 << (bitsPerSample - 1 - bit));
          //   }
          //   ++bitOffset;
          // }

          // outArray[outIndex] = outWord;
          // outArray[pixel] = outWord;
          // pixel += 1;
        }
        // bitOffset = bitOffset + pixelBitSkip - bitsPerSample;
      }
    }
  } else if (format === 3) {
    // floating point
    // Float16 is handled elsewhere
    // normalize 16/24 bit floats to 32 bit floats in the array
    // console.time();
    // if (bitsPerSample === 16) {
    //   for (let byte = 0, outIndex = 0; byte < inBuffer.byteLength; byte += 2, ++outIndex) {
    //     outArray[outIndex] = getFloat16(view, byte);
    //   }
    // }
    // console.timeEnd()
  }

  return outArray.buffer;
}
