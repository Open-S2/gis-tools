export * from './contours.js';
export * from './grid.js';
export * from './hillshade.js';
export * from './mesh.js';

/** Elevation converter */
export type ElevationConverter = (r: number, g: number, b: number, a?: number) => number;

/**
 * Conver a Terrarium tile encoded elevation data into a float precision elevation
 * Terrarium formula: (red * 256 + green + blue / 256) - 32768
 * @param r - red
 * @param g - green
 * @param b - blue
 * @returns - elevation
 */
export function convertTerrariumElevationData(r: number, g: number, b: number): number {
  return r * 256.0 + g + b / 256.0 - 32768.0;
}

/**
 * Conver a Mapbox tile encoded elevation data into a float precision elevation
 * Mapbox formula: -10000 + (red * 256 * 256 + green * 256 + blue) * 0.1
 * @param r - red
 * @param g - green
 * @param b - blue
 * @returns - elevation
 */
export function convertMapboxElevationData(r: number, g: number, b: number): number {
  return -10000 + (r * 256 * 256 + g * 256 + b) * 0.1;
}

/**
 * Encode a float precision elevation into Terrarium tile encoded elevation data.
 * Terrarium formula: (red * 256 + green + blue / 256) - 32768
 * @param elevation - The elevation value to encode
 * @returns - the encoded elevation as RGB
 */
export function encodeTerrariumElevationData(
  elevation: number,
): [r: number, g: number, b: number, a?: number] {
  const scaled_elevation = Math.round((elevation + 32768.0) * 256.0);
  return [(scaled_elevation >> 16) & 0xff, (scaled_elevation >> 8) & 0xff, scaled_elevation & 0xff];
}

/**
 * Encode a float precision elevation into Mapbox tile encoded elevation data
 * Mapbox formula: -10000 + (red * 256 * 256 + green * 256 + blue) * 0.1
 * @param elevation - The elevation value to encode
 * @returns - the encoded elevation as RGB
 */
export function encodeMapboxElevationData(
  elevation: number,
): [r: number, g: number, b: number, a?: number] {
  const scaled_elevation = Math.round((elevation + 10000.0) * 10.0);
  return [(scaled_elevation >> 16) & 0xff, (scaled_elevation >> 8) & 0xff, scaled_elevation & 0xff];
}
