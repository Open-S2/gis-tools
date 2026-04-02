import { convertMapboxElevationData, imageDecoder } from '../../index.js';

import type { ElevationConverter } from '../../index.js';

export interface IsoGrid {
  width: number;
  height: number;
  min: number;
  max: number;
  elevations: number[];
}

/**
 * Build a grid from image data and a conversion tool
 *
 * NOTE: Defaults to the Mapbox elevation data converter `convertMapboxElevationData`. However,
 * to use the Terrarium elevation data converter, use `convertTerrariumElevationData`.
 *
 * @param imageData - image data
 * @param elevationConverter - the conversion tool
 * @param tmsStyle - if true, the y position will be inverted
 * @returns the elevation grid
 */
export async function getElevationGrid(
  imageData:
    | ArrayBufferLike
    | Uint8Array<ArrayBufferLike>
    | Uint8ClampedArray<ArrayBufferLike>
    | Buffer<ArrayBufferLike>
    | ImageData,
  elevationConverter: ElevationConverter = convertMapboxElevationData,
  tmsStyle = false,
): Promise<IsoGrid> {
  const { width, height, data } = 'data' in imageData ? imageData : await imageDecoder(imageData);
  const channels = data.length / (width * height);
  let min = Infinity;
  let max = -Infinity;

  const elevations = new Array(width * height);
  for (let j = 0; j < height; j++) {
    const actualJ = tmsStyle ? height - j - 1 : j;
    const rowOffset = actualJ * width * channels;
    const outputRowOffset = actualJ * width;
    for (let i = 0; i < width; i++) {
      const index = rowOffset + i * channels;
      const alpha = channels > 3 ? data[index + 3] : 255;
      const elevation = elevationConverter(data[index], data[index + 1], data[index + 2], alpha);
      min = Math.min(min, elevation);
      max = Math.max(max, elevation);
      elevations[outputRowOffset + i] = elevation;
    }
  }

  return { width, height, min, max, elevations };
}
