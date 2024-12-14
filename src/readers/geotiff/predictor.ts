/**
 * Decode a block using the specified predictor
 * @param row - the row to decode
 * @param stride - the number of bytes per row
 */
function decodeRowAcc(row: Uint8Array | Uint16Array | Uint32Array, stride: number): void {
  let length = row.length - stride;
  let offset = 0;
  do {
    for (let i = stride; i > 0; i--) {
      row[offset + stride] += row[offset];
      offset++;
    }

    length -= stride;
  } while (length > 0);
}

/**
 * Decode a floating point block using the specified predictor
 * @param row - the row to decode
 * @param stride - the number of bytes per row
 * @param bytesPerSample - the number of bytes per sample
 */
function decodeRowFloatingPoint(
  row: Uint8Array | Uint16Array | Uint32Array,
  stride: number,
  bytesPerSample: number,
): void {
  let index = 0;
  let count = row.length;
  const wc = count / bytesPerSample;

  while (count > stride) {
    for (let i = stride; i > 0; --i) {
      row[index + stride] += row[index];
      ++index;
    }
    count -= stride;
  }

  const copy = row.slice();
  for (let i = 0; i < wc; ++i) {
    for (let b = 0; b < bytesPerSample; ++b) {
      row[bytesPerSample * i + b] = copy[(bytesPerSample - b - 1) * wc + i];
    }
  }
}

/**
 * Apply the specified predictor to a block
 * @param block - the block to modify
 * @param predictor - the predictor
 * @param width - the block width
 * @param height - the block height
 * @param bitsPerSample - the number of bits per sample
 * @param planarConfiguration - the planar configuration
 * @returns - the modified block
 */
export function applyPredictor(
  block: ArrayBuffer,
  predictor: number,
  width: number,
  height: number,
  bitsPerSample: number[],
  planarConfiguration: number,
): ArrayBuffer {
  if (predictor === 0 || predictor === 1) {
    return block;
  }

  for (let i = 0; i < bitsPerSample.length; ++i) {
    if (bitsPerSample[i] % 8 !== 0) {
      throw new Error('When decoding with predictor, only multiple of 8 bits are supported.');
    }
    if (bitsPerSample[i] !== bitsPerSample[0]) {
      throw new Error('When decoding with predictor, all samples must have the same size.');
    }
  }

  const bytesPerSample = bitsPerSample[0] / 8;
  const stride = planarConfiguration === 2 ? 1 : bitsPerSample.length;

  for (let i = 0; i < height; ++i) {
    // Last strip will be truncated if height % stripHeight != 0
    if (i * stride * width * bytesPerSample >= block.byteLength) {
      break;
    }
    let row;
    if (predictor === 2) {
      // horizontal prediction
      switch (bitsPerSample[0]) {
        case 8:
          row = new Uint8Array(
            block,
            i * stride * width * bytesPerSample,
            stride * width * bytesPerSample,
          );
          break;
        case 16:
          row = new Uint16Array(
            block,
            i * stride * width * bytesPerSample,
            (stride * width * bytesPerSample) / 2,
          );
          break;
        case 32:
          row = new Uint32Array(
            block,
            i * stride * width * bytesPerSample,
            (stride * width * bytesPerSample) / 4,
          );
          break;
        default:
          throw new Error(`Predictor 2 not allowed with ${bitsPerSample[0]} bits per sample.`);
      }
      decodeRowAcc(row, stride);
    } else if (predictor === 3) {
      // horizontal floating point
      row = new Uint8Array(
        block,
        i * stride * width * bytesPerSample,
        stride * width * bytesPerSample,
      );
      decodeRowFloatingPoint(row, stride, bytesPerSample);
    }
  }
  return block;
}
