// import {
//   buildSqDists,
//   convertMapboxElevationData,
//   equalPoints,
//   getElevationGrid,
//   isolineThresholds,
//   marchingSquares,
//   simplify,
//   stitchMarchingSquareSegments,
// } from '../../index.js';

// import type {
//   BBox,
//   ElevationConverter,
//   ElevationProperties,
//   MValue,
//   VectorFeatureCollection,
//   VectorMultiLineString,
//   VectorMultiLineStringGeometry,
//   VectorMultiPolygon,
//   VectorMultiPolygonFeature,
//   VectorMultiPolygonGeometry,
//   VectorPoint,
// } from '../../index.js';

// export type IsobandFeature = VectorMultiPolygonFeature<
//   Record<string, unknown>,
//   MValue,
//   ElevationProperties
// >;

// export type ContourFeatureCollection = VectorFeatureCollection<
//   Record<string, unknown>,
//   MValue,
//   ElevationProperties,
//   VectorMultiLineStringGeometry | VectorMultiPolygonGeometry
// >;

// /**
//  * # Create Isolines
//  *
//  * ## Description
//  * Creates isolines from an image.
//  *
//  * NOTE: Defaults to the Mapbox elevation data converter `convertMapboxElevationData`. However,
//  * to use the Terrarium elevation data converter, use `convertTerrariumElevationData`.
//  *
//  * NOTE: Using a buffer with a small padding works, but it only extends the line ends of the isolines.
//  * A better method would be to pull in grid data with large padding (second example).
//  * If you have a large padding like 16px then you don't need a buffer, so you should set it to 0.
//  *
//  * ## Examples
//  *
//  * ### Create Isolines
//  * ```ts
//  * import 'gis-tools-ts/polyfills/local'; // You may need this to handle image data if you are using the file system.
//  * import { buildContours } from 'gis-tools-ts';
//  *
//  * // Pull in the image data. We are using a local image file that is 514x514 with prebuilt padding
//  * const elevationImage = await Bun.file(
//  *   './tests/tools/isobands/fixtures/13_1556_3084.webp',
//  * ).arrayBuffer();
//  *
//  * // Create the isolines.
//  * const isolines = await buildContours(elevationImage);
//  * ```
//  *
//  * ### High quality with large padding
//  * ```ts
//  * import { RasterTilesFileReader } from 'gis-tools-ts/file'; // because we use file.js local polyfills already added
//  * import { buildContours } from 'gis-tools-ts';
//  *
//  * // Setup a tile reader
//  * const reader = new RasterTilesFileReader(
//  *   `${__dirname}/../../readers/tile/fixtures/wm/terrain-v2`,
//  * );
//  * const metadata = await reader.getMetadata();
//  * const isTMS = metadata.scheme === 'tms';
//  * // read in the elevation data with a big bigger padding
//  * // we are pulling in Canada/Greenland area at zoom 3. Padding of 16 so a resulting 544x544 image.
//  * const padding = 16;
//  * const tile = await reader.getTileWithPaddingWM(3, 3, 1, padding, 512, 512);
//  * if (tile === undefined) throw new Error('Tile not found');
//  * // build the isolines
//  * const isolines = await buildContours(tile!.image, undefined, 1_000, false, isTMS, padding, 0);
//  * ```
//  *
//  * @param imageData - the raw RGB(A) image data
//  * @param elevationConverter - the conversion function to convert the pixels to elevation
//  * @param step - the step size for the heightmap. Defaults to 100 meters for the Mapbox elevation data.
//  * @param close - if true, the contours will be closed (instead of isolines we create isobands)
//  * @param tmsStyle - if true, the y position will be inverted
//  * @param padding - The number of pixels that extend around the main data
//  * @param buffer - The number of pixels to extend the lines by to ensure rendering looks smooth. Defaults to `16` pixels assuming a 512x512 grid.
//  * @param tolerance - The Douglas-Peucker tolerance. Defaults to `1 / 2_096`
//  * @returns The isolines stored in a FeatureCollection
//  */
// export async function buildContours(
//   imageData:
//     | ArrayBufferLike
//     | Uint8Array<ArrayBufferLike>
//     | Uint8ClampedArray<ArrayBufferLike>
//     | Buffer<ArrayBufferLike>
//     | ImageData,
//   elevationConverter: ElevationConverter = convertMapboxElevationData,
//   step = 100,
//   close = false,
//   tmsStyle = false,
//   padding = 1,
//   buffer = 16,
//   tolerance = 1 / 2_096,
// ): Promise<ContourFeatureCollection> {
//   // create the elevation grid
//   const elevationGrid = await getElevationGrid(imageData, elevationConverter, tmsStyle);
//   const { width, height, min, max, elevations } = elevationGrid;
//   // setup the thresholds
//   const thresholds = isolineThresholds(min, max, step);
//   // apply marching squares to find all segments
//   const segments = marchingSquares(elevations, width, height, padding, thresholds);
//   // Now that we have our segments, stitch them up
//   const stichedFeatures = stitchMarchingSquareSegments(segments);
//   // simplify lines and extend by buffer
//   if (tolerance !== 0)
//     simplifyFeaturesWithBuffer(stichedFeatures, tolerance, padding, width, height, buffer);

//   if (close) {
//     const features = closeIsolines(stichedFeatures, padding, width, height, buffer);
//     return { type: 'FeatureCollection', features };
//   }
//   return { type: 'FeatureCollection', features: stichedFeatures };
// }

// export function simplifyFeaturesWithBuffer(
//   features: IsobandFeature[],
//   tolerance: number,
//   padding: number,
//   width: number,
//   height: number,
//   buffer: number,
// ): void {
//   for (const feature of features) {
//     const geometry = feature.geometry;
//     const lines: VectorMultiLineString = geometry.coordinates.flat();
//     // Prep Douglas-Peucker simplification by setting t-values.
//     buildSqDists(feature.geometry, tolerance, 0);
//     // Apply Douglas-Peucker
//     simplify(feature.geometry, tolerance, 0);
//     // Extend lines by buffer if they touch the edges
//     if (buffer !== 0) extendByBuffer(lines, padding, width, height, buffer);
//     // cleanup all lines of t values
//     for (const line of lines) {
//       for (const point of line) point.t = undefined;
//     }
//   }
// }

// // extend lines that touch the edges by the buffer
// function extendByBuffer(
//   lines: VectorMultiLineString,
//   padding: number,
//   width: number,
//   height: number,
//   buffer: number,
// ): void {
//   const [currBoundaries, wantedBoundaries] = getBounds(padding, width, height, buffer);

//   for (const line of lines) {
//     if (line.length < 2) continue;
//     // Check Start of line and end of line
//     extendPoint(line[0], line[1], currBoundaries, wantedBoundaries);
//     extendPoint(line[line.length - 1], line[line.length - 2], currBoundaries, wantedBoundaries);
//   }
// }

// function getBounds(
//   padding: number,
//   width: number,
//   height: number,
//   buffer: number,
// ): [currBoundaries: BBox, wantedBoundaries: BBox] {
//   const actualWidth = width - 2 * padding;
//   const actualHeight = height - 2 * padding;

//   // Calculate the raw boundaries (normalized 0-1)
//   const minX = (0.5 - padding) / actualWidth;
//   const maxX = (width - 0.5 - padding) / actualWidth;
//   const minY = (0.5 - padding) / actualHeight;
//   const maxY = (height - 0.5 - padding) / actualHeight;
//   const currBoundaries: BBox = [minX, minY, maxX, maxY];

//   // Calculate the wanted boundaries with buffer
//   const bufferX = buffer / actualWidth;
//   const bufferY = buffer / actualHeight;
//   const wantedBoundaries: BBox = [minX - bufferX, minY - bufferY, maxX + bufferX, maxY + bufferY];

//   return [currBoundaries, wantedBoundaries];
// }

// function extendPoint(
//   p: VectorPoint,
//   n: VectorPoint,
//   currBoundaries: BBox,
//   wantedBoundaries: BBox,
// ): void {
//   const { min } = Math;
//   const [cminX, cminY, cmaxX, cmaxY] = currBoundaries;
//   const [minX, minY, maxX, maxY] = wantedBoundaries;
//   // Only extend if the point is on (or very near) the current physical edge
//   const onEdge = p.x <= cminX || p.x >= cmaxX || p.y <= cminY || p.y >= cmaxY;
//   if (!onEdge) return;
//   // Vector from neighbor to edge point
//   const dx = p.x - n.x;
//   const dy = p.y - n.y;
//   // We want to find which boundary the vector hits first
//   let k = Infinity;

//   if (dx < 0) k = min(k, (minX - p.x) / dx);
//   if (dx > 0) k = min(k, (maxX - p.x) / dx);
//   if (dy < 0) k = min(k, (minY - p.y) / dy);
//   if (dy > 0) k = min(k, (maxY - p.y) / dy);

//   // If we found a valid intersection, move the point out
//   if (k !== Infinity && k > 0) {
//     p.x = p.x + k * dx;
//     p.y = p.y + k * dy;
//   }
// }

// // close the isolines if needed
// export function closeIsolines(
//   features: ElevationFeature[],
//   padding: number,
//   width: number,
//   height: number,
//   buffer: number,
// ): IsobandFeature[] {
//   const [_, [minX, minY, maxX, maxY]] = getBounds(padding, width, height, buffer);
//   const result: IsobandFeature[] = [];

//   for (const { properties, geometry } of features) {
//     const coordinates: VectorMultiPolygon = [];
//     // close
//     for (const line of geometry.coordinates) {
//       const first = line.at(0)!;
//       const last = line.at(-1)!;
//       // 1) if the line is already self closing, just store it
//       if (equalPoints(first, last)) {
//         coordinates.push([line]);
//         continue;
//       }
//       // 2) If both of the endpoints are not outside the grid, just close the line
//       const firstInside = first.x >= 0 && first.x <= 1 && first.y >= 0 && first.y <= 1;
//       const lastInside = last.x >= 0 && last.x <= 1 && last.y >= 0 && last.y <= 1;
//       if (firstInside && lastInside) {
//         line.push(first);
//         coordinates.push([line]);
//         continue;
//       }
//       // 3) Otherwise, extend line by wrapping around the shape of the grid from last to first
//       // Calculate the corners
//       const corners: VectorPoint[] = [
//         { x: maxX, y: minY }, // Dist 1: Top-Right
//         { x: maxX, y: maxY }, // Dist 2: Bottom-Right
//         { x: minX, y: maxY }, // Dist 3: Bottom-Left
//         { x: minX, y: minY }, // Dist 4: Top-Left
//       ];
//       const dLast = getTrackDist(last);
//       const dFirst = getTrackDist(first);
//       // We want to visit corners in order: 1 -> 2 -> 3 -> 4 -> 1...
//       for (let i = 1; i <= 4; i++) {
//         // Check if corner 'i' is between dLast and dFirst clockwise
//         let isBetween = false;
//         // Standard case: last is at 0.5, first is at 2.5. Corners 1 and 2 are between.
//         if (dLast < dFirst) isBetween = i > dLast && i < dFirst;
//         // Wrap case: last is at 3.5, first is at 0.5. Corner 4 and 1 are between.
//         else isBetween = i > dLast || i < dFirst;

//         if (isBetween) line.push({ ...corners[i - 1] });
//       }
//       // Finally, close the loop and store
//       line.push(first);
//       coordinates.push([line]);
//     }
//     result.push({
//       type: 'VectorFeature',
//       properties,
//       geometry: { type: 'MultiPolygon', is3D: false, coordinates },
//     });
//   }

//   return result;
// }

// function getTrackDist(p: VectorPoint): number {
//   if (p.y <= 0) return p.x; // Top: 0.0 -> 1.0
//   if (p.x >= 1) return 1 + p.y; // Right: 1.0 -> 2.0
//   if (p.y >= 1) return 2 + (1 - p.x); // Bottom: 2.0 -> 3.0
//   return 3 + (1 - p.y); // Left: 3.0 -> 4.0
// }
