import {
  buildIsoBands,
  convertMapboxElevationData,
  getElevationGrid,
  tileToCenterLonLat,
} from '../../index.js';

import type {
  ElevationConverter,
  MValue,
  Properties,
  TileID,
  VectorFeatureCollection,
  VectorMultiPolygonFeature,
  VectorMultiPolygonGeometry,
} from '../../index.js';

export interface HillshadeProperties extends Properties {
  hillshade: string;
}

export type HillshadeFeature = VectorMultiPolygonFeature<
  Record<string, unknown>,
  MValue,
  HillshadeProperties
>;

export type HillshadeFeatureCollection = VectorFeatureCollection<
  Record<string, unknown>,
  MValue,
  HillshadeProperties,
  VectorMultiPolygonGeometry
>;

// https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-hillshade-works.htm

export interface HillshadeResult {
  width: number;
  height: number;
  hillshade: number[];
}

/**
 * # Vectorize hillshade data
 *
 * ## Description
 * Generate vectorized hillshade data with lights and darks
 *
 * ## Example
 * ```ts
 * import { generateHillshade } from 'gis-tools-ts';
 * import sharp from 'sharp';
 *
 * const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1544_3085.webp`).arrayBuffer();
 * const vectorHillshade = await vectorizeHillshade(elevationImage, [13, 1544, 3085]);
 * ```
 *
 * ## Links
 * - <https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-hillshade-works.htm>
 *
 * @param imageData - the raw RGB(A) image data
 * @param tile - The zoom, x, and y of the tile
 * @param elevationConverter - the conversion function to convert the pixels to elevation
 * @param tmsStyle - if true, the y position will be inverted
 * @param azimuth - The azimuth of the sun
 * @param altitude - The altitude of the sun
 * @param zFactor - The zFactor effects the weight of the sun's light ouput
 * @param thresholds - The thresholds for the lights and darks to generate
 * @param weights - The weights of the azimuths. The first azimuth is the sun's azimuth value parameter. Each subsequent azimuth is the angle away from the sun.
 * @param padding - The padding to add to the image. This is needed to account for the edge of the tile.
 * @param tolerance - The tolerance of the simplification of the lines using the Ramer-Douglas-Peucker algorithm
 * @returns An image/array of grayscale values
 */
export async function vectorizeHillshade(
  imageData:
    | ArrayBufferLike
    | Uint8Array<ArrayBufferLike>
    | Uint8ClampedArray<ArrayBufferLike>
    | Buffer<ArrayBufferLike>,
  tile: TileID,
  elevationConverter: ElevationConverter = convertMapboxElevationData,
  tmsStyle = false,
  azimuth = 315,
  altitude = 45,
  zFactor = 1.5,
  thresholds: Map<number, string> = new Map([
    [220, 'dark'],
    [235, 'darker'],
    [60, 'light'],
    [40, 'lighter'],
    [15, 'lightest'],
  ]),
  weights: [primary: number, neg45: number, pos45: number, pos90: number] = [
    0.65, 0.15, 0.15, 0.05,
  ],
  padding = 0,
  tolerance = 1 / 2_096,
): Promise<HillshadeFeatureCollection> {
  const { width, height, hillshade } = await generateHillshade(
    imageData,
    tile,
    elevationConverter,
    tmsStyle,
    azimuth,
    altitude,
    zFactor,
    weights,
    true,
  );
  const hillshadeInvert = hillshade.map((x) => 255 - x);

  const features: HillshadeFeature[] = [];
  for (const [elev, name] of thresholds.entries()) {
    const hill = elev >= 127.5 ? hillshade : hillshadeInvert;
    const threshold = elev >= 127.5 ? elev : 255 - elev;
    const polygons = buildIsoBands(hill, threshold, width, height, padding, tolerance);
    features.push({
      type: 'VectorFeature',
      geometry: { type: 'MultiPolygon', is3D: false, coordinates: polygons },
      properties: { hillshade: name },
    });
  }

  return { type: 'FeatureCollection', features };
}

/**
 * # Build greyscale hillshade data
 *
 * ## Description
 * Builds an array of grayscale values for a given tile.
 *
 * ## Example
 * ```ts
 * import { generateHillshade } from 'gis-tools-ts';
 * import sharp from 'sharp';
 *
 * import type { SharpOptions } from 'sharp';
 *
 * const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1544_3085.webp`).arrayBuffer();
 * const { width, height, hillshade } = await generateHillshade(elevationImage, [13, 1544, 3085]);
 *
 * const sharpOptions: SharpOptions = { raw: { width, height, channels: 1 } };
 * const pngData = await sharp(new Uint8ClampedArray(hillshade), sharpOptions).png().toBuffer();
 * ```
 *
 * ## Links
 * - <https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-hillshade-works.htm>
 *
 * @param imageData - the raw RGB(A) image data
 * @param tile - The zoom, x, and y of the tile
 * @param elevationConverter - the conversion function to convert the pixels to elevation
 * @param tmsStyle - if true, the y position will be inverted
 * @param azimuth - The azimuth of the sun
 * @param altitude - The altitude of the sun
 * @param zFactor - The zFactor effects the weight of the sun's light ouput
 * @param weights - The weights of the azimuths. The first azimuth is the sun's azimuth value parameter. Each subsequent azimuth is the angle away from the sun.
 * @param smooth - if true, the hillshade will be smoothed
 * @returns An image/array of grayscale values
 */
export async function generateHillshade(
  imageData:
    | ArrayBufferLike
    | Uint8Array<ArrayBufferLike>
    | Uint8ClampedArray<ArrayBufferLike>
    | Buffer<ArrayBufferLike>,
  tile: TileID,
  elevationConverter: ElevationConverter = convertMapboxElevationData,
  tmsStyle = false,
  azimuth = 315,
  altitude = 45,
  zFactor = 1,
  weights: [primary: number, neg45: number, pos45: number, pos90: number] = [
    0.65, 0.15, 0.15, 0.05,
  ],
  smooth = false,
): Promise<HillshadeResult> {
  const { cos, sin, atan, atan2, sqrt, max, pow, PI } = Math;
  // create the elevation grid
  const elevationGrid = await getElevationGrid(imageData, elevationConverter, tmsStyle);
  const { width, height, elevations } = elevationGrid;
  // remove padding pixels for future calculations
  const correctedWidth = width - (width % 256);
  let hillshade: number[] = new Array(width * height);

  const zenithRad = ((90 - altitude) * PI) / 180;
  // const azimuthRad = (azimuthMath * PI) / 180;
  const azimuthSources = [
    azimuth, // Primary
    azimuth - 45, // Secondary 1
    azimuth + 45, // Secondary 2
    azimuth + 90, // Secondary 3
  ].map((a) => {
    const mathA = (360 - a + 90) % 360;
    return (mathA * PI) / 180;
  });

  const { y: lat } = tileToCenterLonLat(tile, tmsStyle);
  const latRad = (lat * PI) / 180;
  const cellSize = (40075016.686 * cos(latRad)) / (pow(2, tile.zoom) * correctedWidth);

  for (let y = 1; y < height - 1; y++) {
    for (let x = 1; x < width - 1; x++) {
      // 1. Get 3x3 window
      const a = elevations[(y - 1) * width + (x - 1)];
      const b = elevations[(y - 1) * width + x];
      const c = elevations[(y - 1) * width + (x + 1)];
      const d = elevations[y * width + (x - 1)];
      // const e = elevations[y * width + x];
      const f = elevations[y * width + (x + 1)];
      const g = elevations[(y + 1) * width + (x - 1)];
      const h = elevations[(y + 1) * width + x];
      const i = elevations[(y + 1) * width + (x + 1)];

      // 2. Calculate dz/dx and dz/dy (Horn's Method)
      // [dz/dx] = ((c + 2f + i) - (a + 2d + g)) / (8 * cellsize)
      const dzdx = (zFactor * (c + 2 * f + i - (a + 2 * d + g))) / (8 * cellSize);
      // [dz/dy] = ((g + 2h + i) - (a + 2b + c)) / (8 * cellsize)
      const dzdy = (zFactor * (g + 2 * h + i - (a + 2 * b + c))) / (8 * cellSize);

      // 3. Calculate Aspect and Slope
      const riseRun = sqrt(dzdx * dzdx + dzdy * dzdy);
      const slopeRad = atan(riseRun);

      let aspectRad = 0;
      // Only calculate aspect if the slope is not perfectly flat
      if (riseRun > 0.0001) {
        aspectRad = atan2(dzdy, -dzdx);
        if (aspectRad < 0) aspectRad += 2 * PI;
      }

      // 4. Calculate Multi-directional Illumination
      let totalIllumination = 0;

      for (let i = 0; i < azimuthSources.length; i++) {
        const sourceAzimuthRad = azimuthSources[i];
        const weight = weights[i];

        const illumination =
          cos(zenithRad) * cos(slopeRad) +
          sin(zenithRad) * sin(slopeRad) * cos(sourceAzimuthRad - aspectRad);

        // Weighted sum
        totalIllumination += max(0, illumination) * weight;
      }

      const val = totalIllumination * 255;
      hillshade[y * width + x] = val;
    }
  }

  // Pad top and bottom rows
  for (let i = 0; i < width; i++) {
    hillshade[i] = hillshade[width + i];
    hillshade[(height - 1) * width + i] = hillshade[(height - 2) * width + i];
  }
  // Pad left and right columns
  for (let j = 0; j < height; j++) {
    hillshade[j * width] = hillshade[j * width + 1];
    hillshade[j * width + (width - 1)] = hillshade[j * width + (width - 2)];
  }

  if (smooth) hillshade = smoothHillshade(hillshade, width, height);

  return { width, height, hillshade };
}

// Simple 3x3 Mean Filter to "clean up" the hillshade a bit. works a lot like a gaussian blur
function smoothHillshade(data: number[], width: number, height: number): number[] {
  const smoothed = new Array(width * height).fill(0);
  // only smooth inner cells
  for (let y = 1; y < height - 1; y++) {
    for (let x = 1; x < width - 1; x++) {
      let sum = 0;
      for (let ky = -1; ky <= 1; ky++) {
        for (let kx = -1; kx <= 1; kx++) {
          sum += data[(y + ky) * width + (x + kx)];
        }
      }
      smoothed[y * width + x] = sum / 9;
    }
  }
  // refill top and bottom rows:
  for (let i = 0; i < width; i++) {
    smoothed[i] = smoothed[width + i];
    smoothed[(height - 1) * width + i] = smoothed[(height - 2) * width + i];
  }
  // refill left and right columns:
  for (let j = 0; j < height; j++) {
    smoothed[j * width] = smoothed[j * width + 1];
    smoothed[j * width + (width - 1)] = smoothed[j * width + (width - 2)];
  }
  return smoothed;
}
