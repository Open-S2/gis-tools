const FIXED_FRAC_BITS = 14;

/**
 * Filter input value given a filter window.
 * @param x - input
 * @param a - filter window
 * @returns - filtered value
 */
function filterValue(x: number, a: 2 | 3): number {
  if (x <= -a || x >= a) return 0;
  if (x === 0) return 0;
  // appears to do nothing?
  // if ( x > -1.19209290e-07 && x < 1.19209290e-07 ) return 1
  const xPi = x * Math.PI;

  return ((Math.sin(xPi) / xPi) * Math.sin(xPi / a)) / (xPi / a);
}

/**
 * Convert value to fixed point
 * @param value - input
 * @returns - fixed point
 */
function toFixedPoint(value: number): number {
  return Math.round(value * ((1 << FIXED_FRAC_BITS) - 1));
}

/**
 * Create a Lanczos filter
 * @param srcSize - source image size
 * @param destSize - destination image size
 * @param scale - scale factor
 * @param offset - offset to apply
 * @param use2 - use 2nd lanczos filter instead of 3rd
 * @returns - filter
 */
function filters(
  srcSize: number,
  destSize: number,
  scale: number,
  offset: number,
  use2: boolean,
): Int16Array {
  const a = use2 ? 2 : 3;
  const scaleInverted = 1 / scale;
  const scaleClamped = Math.min(1, scale); // For upscale

  // Filter window (averaging interval), scaled to src image
  const srcWindow = a / scaleClamped;

  const maxFilterElementSize = Math.floor((srcWindow + 1) * 2);
  const packedFilter = new Int16Array((maxFilterElementSize + 2) * destSize);

  let packedFilterPtr = 0;

  // For each destination pixel calculate source range and built filter values
  for (let destPixel = 0; destPixel < destSize; destPixel++) {
    // Scaling should be done relative to central pixel point
    const sourcePixel = (destPixel + 0.5) * scaleInverted + offset;
    const sourceFirst = Math.max(0, Math.floor(sourcePixel - srcWindow));
    const sourceLast = Math.min(srcSize - 1, Math.ceil(sourcePixel + srcWindow));

    const filterElementSize = sourceLast - sourceFirst + 1;
    const floatFilter = new Float32Array(filterElementSize);
    const fxpFilter = new Int16Array(filterElementSize);

    let total = 0;

    // Fill filter values for calculated range
    let index = 0;
    for (let pixel = sourceFirst; pixel <= sourceLast; pixel++) {
      const floatValue = filterValue((pixel + 0.5 - sourcePixel) * scaleClamped, a);

      total += floatValue;
      floatFilter[index] = floatValue;

      index++;
    }

    // Normalize filter, convert to fixed point and accumulate conversion error
    let filterTotal = 0;

    for (let index = 0; index < floatFilter.length; index++) {
      const filterValue = floatFilter[index] / total;

      filterTotal += filterValue;
      fxpFilter[index] = toFixedPoint(filterValue);
    }

    // Compensate normalization error, to minimize brightness drift
    fxpFilter[destSize >> 1] += toFixedPoint(1 - filterTotal);

    //
    // Now pack filter to useable form
    //
    // 1. Trim heading and tailing zero values, and compensate shitf/length
    // 2. Put all to single array in this format:
    //
    //    [ pos shift, data length, value1, value2, value3, ... ]
    //
    let leftNotEmpty = 0;
    while (leftNotEmpty < fxpFilter.length && fxpFilter[leftNotEmpty] === 0) {
      leftNotEmpty++;
    }

    let rightNotEmpty = fxpFilter.length - 1;
    while (rightNotEmpty > 0 && fxpFilter[rightNotEmpty] === 0) {
      rightNotEmpty--;
    }

    const filterShift = sourceFirst + leftNotEmpty;
    const filterSize = rightNotEmpty - leftNotEmpty + 1;

    packedFilter[packedFilterPtr++] = filterShift; // shift
    packedFilter[packedFilterPtr++] = filterSize; // size

    packedFilter.set(fxpFilter.subarray(leftNotEmpty, rightNotEmpty + 1), packedFilterPtr);
    packedFilterPtr += filterSize;
  }

  return packedFilter;
}

/**
 * Copy the contents of the source image to the destination image
 * @param source - the source image
 * @param dest - the destination image
 * @param sx - source starting x point [Default: 0]
 * @param sy - source starting y point [Default: 0]
 * @param sw - source width to use [Default: source width - sx]
 * @param sh - source height to use [Default: source height - sy]
 * @param dx - destination starting x point [Default: 0]
 * @param dy - destination starting y point [Default: 0]
 */
export function copyImage(
  source: ImageData,
  dest: ImageData,
  sx = 0,
  sy = 0,
  sw = source.width - sx,
  sh = source.height - sy,
  dx = 0,
  dy = 0,
): void {
  sx = sx | 0;
  sy = sy | 0;
  sw = sw | 0;
  sh = sh | 0;
  dx = dx | 0;
  dy = dy | 0;

  if (sw <= 0 || sh <= 0) return;

  const sourceData = new Uint32Array(source.data.buffer);
  const destData = new Uint32Array(dest.data.buffer);

  for (let y = 0; y < sh; y++) {
    const sourceY = sy + y;
    if (sourceY < 0 || sourceY >= source.height) continue;

    const destY = dy + y;
    if (destY < 0 || destY >= dest.height) continue;

    for (let x = 0; x < sw; x++) {
      const sourceX = sx + x;
      if (sourceX < 0 || sourceX >= source.width) continue;

      const destX = dx + x;
      if (destX < 0 || destX >= dest.width) continue;

      const sourceIndex = sourceY * source.width + sourceX;
      const destIndex = destY * dest.width + destX;

      destData[destIndex] = sourceData[sourceIndex];
    }
  }
}

/**
 * Create an image given the size, fill color and number of channels
 * @param width - the image width
 * @param height - the image height
 * @param data - the image data [Default: creates a new array]
 * @param fill - the fill color [Default: [0, 0, 0, 0]]
 * @param channels - the number of channels [Default: 4]
 * @returns - the created image
 */
export function createImage(
  width: number,
  height: number,
  data?: Uint8ClampedArray<ArrayBuffer>,
  fill: number[] | Uint8ClampedArray<ArrayBuffer> = [0, 0, 0, 0],
  channels = 4,
): ImageData {
  width = Math.floor(width);
  height = Math.floor(height);

  if (width < 1 || height < 1) {
    throw TypeError('Index or size is negative or greater than the allowed amount');
  }

  const length = width * height * channels;

  if (data === undefined) {
    data = new Uint8ClampedArray(length);
  }

  if (data.length !== length) {
    throw TypeError('Index or size is negative or greater than the allowed amount');
  }

  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const index = (y * width + x) * channels;
      for (let c = 0; c < channels; c++) {
        data[index + c] = fill[c];
      }
    }
  }

  return { data, width, height };
}

/**
 * Lanczos resize function
 * @param source - the source image
 * @param dest - the destination image
 * @param use2 - use 2nd lanczos filter instead of 3rd
 */
export function resizeImage(source: ImageData, dest: ImageData, use2 = false): void {
  const xRatio = dest.width / source.width;
  const yRatio = dest.height / source.height;

  const filtersX = filters(source.width, dest.width, xRatio, 0, use2);
  const filtersY = filters(source.height, dest.height, yRatio, 0, use2);

  const tmp = new Uint8ClampedArray(dest.width * source.height * 4);

  convolveImage(source.data, tmp, source.width, source.height, dest.width, filtersX);
  convolveImage(tmp, dest.data, source.height, dest.width, dest.height, filtersY);
}

/**
 * Convolve an image with a filter
 * @param source - the source image
 * @param dest - the destination image
 * @param sw - source width
 * @param sh - source height
 * @param dw - destination width
 * @param filters - image filter
 */
export function convolveImage(
  source: Uint8ClampedArray,
  dest: Uint8ClampedArray,
  sw: number,
  sh: number,
  dw: number,
  filters: Int16Array,
): void {
  let srcOffset = 0;
  let destOffset = 0;

  // For each row
  for (let sourceY = 0; sourceY < sh; sourceY++) {
    let filterPtr = 0;

    // Apply precomputed filters to each destination row point
    for (let destX = 0; destX < dw; destX++) {
      // Get the filter that determines the current output pixel.
      const filterShift = filters[filterPtr++];

      let srcPtr = (srcOffset + filterShift * 4) | 0;

      let r = 0;
      let g = 0;
      let b = 0;
      let a = 0;

      // Apply the filter to the row to get the destination pixel r, g, b, a
      for (let filterSize = filters[filterPtr++]; filterSize > 0; filterSize--) {
        const filterValue = filters[filterPtr++];

        r = (r + filterValue * source[srcPtr]) | 0;
        g = (g + filterValue * source[srcPtr + 1]) | 0;
        b = (b + filterValue * source[srcPtr + 2]) | 0;
        a = (a + filterValue * source[srcPtr + 3]) | 0;

        srcPtr = (srcPtr + 4) | 0;
      }

      // Bring this value back in range. All of the filter scaling factors
      // are in fixed point with FIXED_FRAC_BITS bits of fractional part.
      //
      // (!) Add 1/2 of value before clamping to get proper rounding. In other
      // case brightness loss will be noticeable if you resize image with white
      // border and place it on white background.
      //
      dest[destOffset] = (r + (1 << 13)) >> FIXED_FRAC_BITS;
      dest[destOffset + 1] = (g + (1 << 13)) >> FIXED_FRAC_BITS;
      dest[destOffset + 2] = (b + (1 << 13)) >> FIXED_FRAC_BITS;
      dest[destOffset + 3] = (a + (1 << 13)) >> FIXED_FRAC_BITS;

      destOffset = (destOffset + sh * 4) | 0;
    }

    destOffset = ((sourceY + 1) * 4) | 0;
    srcOffset = ((sourceY + 1) * sw * 4) | 0;
  }
}
