/**
 * @param array
 * @param width
 * @param height
 * @param samplesPerPixel
 */
function copyNewSize(
  array: number[],
  width: number,
  height: number,
  samplesPerPixel = 1,
): number[] {
  return new (Object.getPrototypeOf(array).constructor)(width * height * samplesPerPixel);
}

/**
 * Resample the input arrays using nearest neighbor value selection.
 * @param valueArrays The input arrays to resample
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @returns The resampled rasters
 */
export function resampleNearest(
  valueArrays: number[][],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
): number[][] {
  const relX = inWidth / outWidth;
  const relY = inHeight / outHeight;
  return valueArrays.map((array) => {
    const newArray = copyNewSize(array, outWidth, outHeight);
    for (let y = 0; y < outHeight; ++y) {
      const cy = Math.min(Math.round(relY * y), inHeight - 1);
      for (let x = 0; x < outWidth; ++x) {
        const cx = Math.min(Math.round(relX * x), inWidth - 1);
        const value = array[cy * inWidth + cx];
        newArray[y * outWidth + x] = value;
      }
    }
    return newArray;
  });
}

/**
 * simple linear interpolation, code from:
 * https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support
 * @param v0
 * @param v1
 * @param t
 */
function lerp(v0: number, v1: number, t: number): number {
  return (1 - t) * v0 + t * v1;
}

/**
 * Resample the input arrays using bilinear interpolation.
 * @param valueArrays The input arrays to resample
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @returns The resampled rasters
 */
export function resampleBilinear(
  valueArrays: number[][],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
): number[][] {
  const relX = inWidth / outWidth;
  const relY = inHeight / outHeight;

  return valueArrays.map((array) => {
    const newArray = copyNewSize(array, outWidth, outHeight);
    for (let y = 0; y < outHeight; ++y) {
      const rawY = relY * y;

      const yl = Math.floor(rawY);
      const yh = Math.min(Math.ceil(rawY), inHeight - 1);

      for (let x = 0; x < outWidth; ++x) {
        const rawX = relX * x;
        const tx = rawX % 1;

        const xl = Math.floor(rawX);
        const xh = Math.min(Math.ceil(rawX), inWidth - 1);

        const ll = array[yl * inWidth + xl];
        const hl = array[yl * inWidth + xh];
        const lh = array[yh * inWidth + xl];
        const hh = array[yh * inWidth + xh];

        const value = lerp(lerp(ll, hl, tx), lerp(lh, hh, tx), rawY % 1);
        newArray[y * outWidth + x] = value;
      }
    }
    return newArray;
  });
}

/**
 *
 */
export type Method = 'nearest' | 'bilinear' | 'linear';

/**
 * Resample the input arrays using the selected resampling method.
 * @param valueArrays The input arrays to resample
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @param [method] The desired resampling method
 * @returns The resampled rasters
 */
export function resample(
  valueArrays: number[][],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
  method: Method = 'nearest',
): number[][] {
  switch (method) {
    case 'nearest':
      return resampleNearest(valueArrays, inWidth, inHeight, outWidth, outHeight);
    case 'bilinear':
    case 'linear':
      return resampleBilinear(valueArrays, inWidth, inHeight, outWidth, outHeight);
    default:
      throw new Error(`Unsupported resampling method: '${method}'`);
  }
}

/**
 * Resample the pixel interleaved input array using nearest neighbor value selection.
 * @param valueArrays The input arrays to resample
 * @param valueArray
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @param samples The number of samples per pixel for pixel
 *                         interleaved data
 * @returns The resampled raster
 */
export function resampleNearestInterleaved(
  valueArray: number[],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
  samples: number,
): number[] {
  const relX = inWidth / outWidth;
  const relY = inHeight / outHeight;

  const newArray = copyNewSize(valueArray, outWidth, outHeight, samples);
  for (let y = 0; y < outHeight; ++y) {
    const cy = Math.min(Math.round(relY * y), inHeight - 1);
    for (let x = 0; x < outWidth; ++x) {
      const cx = Math.min(Math.round(relX * x), inWidth - 1);
      for (let i = 0; i < samples; ++i) {
        const value = valueArray[cy * inWidth * samples + cx * samples + i];
        newArray[y * outWidth * samples + x * samples + i] = value;
      }
    }
  }
  return newArray;
}

/**
 * Resample the pixel interleaved input array using bilinear interpolation.
 * @param valueArrays The input arrays to resample
 * @param valueArray
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @param samples The number of samples per pixel for pixel
 *                         interleaved data
 * @returns The resampled raster
 */
export function resampleBilinearInterleaved(
  valueArray: number[],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
  samples: number,
): number[] {
  const relX = inWidth / outWidth;
  const relY = inHeight / outHeight;
  const newArray = copyNewSize(valueArray, outWidth, outHeight, samples);
  for (let y = 0; y < outHeight; ++y) {
    const rawY = relY * y;

    const yl = Math.floor(rawY);
    const yh = Math.min(Math.ceil(rawY), inHeight - 1);

    for (let x = 0; x < outWidth; ++x) {
      const rawX = relX * x;
      const tx = rawX % 1;

      const xl = Math.floor(rawX);
      const xh = Math.min(Math.ceil(rawX), inWidth - 1);

      for (let i = 0; i < samples; ++i) {
        const ll = valueArray[yl * inWidth * samples + xl * samples + i];
        const hl = valueArray[yl * inWidth * samples + xh * samples + i];
        const lh = valueArray[yh * inWidth * samples + xl * samples + i];
        const hh = valueArray[yh * inWidth * samples + xh * samples + i];

        const value = lerp(lerp(ll, hl, tx), lerp(lh, hh, tx), rawY % 1);
        newArray[y * outWidth * samples + x * samples + i] = value;
      }
    }
  }
  return newArray;
}

/**
 * Resample the pixel interleaved input array using the selected resampling method.
 * @param valueArray The input array to resample
 * @param inWidth The width of the input rasters
 * @param inHeight The height of the input rasters
 * @param outWidth The desired width of the output rasters
 * @param outHeight The desired height of the output rasters
 * @param samples The number of samples per pixel for pixel
 *                                 interleaved data
 * @param [method] The desired resampling method
 * @returns The resampled rasters
 */
export function resampleInterleaved(
  valueArray: number[],
  inWidth: number,
  inHeight: number,
  outWidth: number,
  outHeight: number,
  samples: number,
  method: Method = 'nearest',
): number[] {
  switch (method) {
    case 'nearest':
      return resampleNearestInterleaved(
        valueArray,
        inWidth,
        inHeight,
        outWidth,
        outHeight,
        samples,
      );
    case 'bilinear':
    case 'linear':
      return resampleBilinearInterleaved(
        valueArray,
        inWidth,
        inHeight,
        outWidth,
        outHeight,
        samples,
      );
    default:
      throw new Error(`Unsupported resampling method: '${method}'`);
  }
}
