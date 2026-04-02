// import type { VectorPoint } from '../../index.js';

// // https://github.com/urbanspr1nter/marching-squares/blob/main/Teaching-Marching-Squares.pdf

// // Each index 0-15 returns pairs of edges to connect.
// // Example: Case 3 (binary 0011) connects edge 3 to edge 1.
// const MS_LUT = [
//   [], // 0: All points outside
//   [3, 2], // 1
//   [2, 1], // 2
//   [3, 1], // 3
//   [1, 0], // 4
//   [3, 0, 1, 2], // 5: Saddle case (two segments)
//   [2, 0], // 6
//   [3, 0], // 7
//   [3, 0], // 8
//   [2, 0], // 9
//   [3, 2, 1, 0], // 10: Saddle case (two segments)
//   [1, 0], // 11
//   [3, 1], // 12
//   [2, 1], // 13
//   [3, 2], // 14
//   [], // 15: All points inside
// ];

// export interface IsolineSegment {
//   from: VectorPoint;
//   to: VectorPoint;
//   visited: boolean;
// }

// export function isolineThresholds(min: number, max: number, step: number): number[] {
//   const thresholds: number[] = [];
//   let current = Math.ceil(min / step) * step;

//   while (current <= max) {
//     thresholds.push(current);
//     current += step;
//   }

//   return thresholds;
// }

// export type MarchingSquaresResult = Map<number, IsolineSegment[]>;

// /**
//  * Create isolines from a flat 2D heightmap
//  * @param heightmap - the heightmap data
//  * @param width - the width of the heightmap
//  * @param height - the height of the heightmap
//  * @param padding - how many pixels in the heightmap are padding
//  * @param thresholds - the thresholds to use
//  * @returns The isolines
//  */
// export function marchingSquares(
//   heightmap: number[],
//   width: number,
//   height: number,
//   padding: number,
//   thresholds: number[],
// ): MarchingSquaresResult {
//   const allSegmentsByLevel = new Map<number, IsolineSegment[]>();

//   for (let y = 0; y < height - 1; y++) {
//     for (let x = 0; x < width - 1; x++) {
//       // Get the 4 corners
//       const h0 = heightmap[y * width + x];
//       const h1 = heightmap[y * width + (x + 1)];
//       const h2 = heightmap[(y + 1) * width + (x + 1)];
//       const h3 = heightmap[(y + 1) * width + x];
//       const corners = [h0, h1, h2, h3];
//       const [min, max] = [Math.min(...corners), Math.max(...corners)];

//       // Only check thresholds that actually pass through this specific cell
//       for (const t of thresholds) {
//         if (t >= min && t <= max) {
//           const segments = marchCell(corners, width, height, padding, x, y, t);
//           if (segments.length > 0) {
//             // Push to a flat array for now (The "Soup")
//             if (!allSegmentsByLevel.has(t)) allSegmentsByLevel.set(t, []);
//             allSegmentsByLevel.get(t)!.push(...segments);
//           }
//         }
//       }
//     }
//   }

//   return allSegmentsByLevel;
// }

// /**
//  * Processes a cell and returns interpolated segments
//  * @param corners - Array of 4 corner values [top-left, top-right, bottom-right, bottom-left]
//  * @param width - Grid width
//  * @param height - Grid height
//  * @param padding - The number of pixels that extend around the main data
//  * @param x - Grid X coordinate
//  * @param y - Grid Y coordinate
//  * @param threshold - The elevation value we are looking for
//  * @returns Array of segments. If there are no segments, an empty array is returned
//  */
// function marchCell(
//   corners: number[],
//   width: number,
//   height: number,
//   padding: number,
//   x: number,
//   y: number,
//   threshold: number,
// ): IsolineSegment[] {
//   // 1. Determine the 4-bit case index
//   let caseIndex = 0;
//   if (corners[0] >= threshold) caseIndex |= 8; // Top-Left
//   if (corners[1] >= threshold) caseIndex |= 4; // Top-Right
//   if (corners[2] >= threshold) caseIndex |= 2; // Bottom-Right
//   if (corners[3] >= threshold) caseIndex |= 1; // Bottom-Left

//   const edges = MS_LUT[caseIndex];
//   const segments: IsolineSegment[] = [];
//   if (edges.length === 0) return segments;

//   // 2. Process edges in pairs (usually 1 segment, 2 for saddle cases)
//   for (let i = 0; i < edges.length; i += 2) {
//     const from = interpolate(edges[i], corners, width, height, padding, x, y, threshold);
//     const to = interpolate(edges[i + 1], corners, width, height, padding, x, y, threshold);
//     segments.push({ from, to, visited: false });
//   }

//   return segments;
// }

// // Interpolate an edge
// function interpolate(
//   edge: number,
//   corners: number[],
//   width: number,
//   height: number,
//   padding: number,
//   x: number,
//   y: number,
//   t: number,
// ): VectorPoint {
//   let point: VectorPoint;
//   // interpolate edge
//   if (edge === 0) {
//     point = { x: x + safeInterp(corners[0], corners[1], t), y }; // Top
//   } else if (edge === 1) {
//     point = { x: x + 1, y: y + safeInterp(corners[1], corners[2], t) }; // Right
//   } else if (edge === 2) {
//     point = { x: x + safeInterp(corners[3], corners[2], t), y: y + 1 }; // Bottom
//   } else if (edge === 3) {
//     point = { x, y: y + safeInterp(corners[0], corners[3], t) }; // Left
//   } else {
//     point = { x, y }; // Center
//   }
//   // remap to 0->32_768
//   remap(point, width, height, padding);

//   return point;
// }

// // epsilon check to avoid division by zero
// function safeInterp(v1: number, v2: number, t: number): number {
//   if (Math.abs(v1 - v2) < 1e-10) return 0.5;
//   return (t - v1) / (v2 - v1);
// }

// /**
//  * Convert the x-y values from width-height scale to 0->32_768
//  * @param point - VectorPoint to mutate
//  * @param width - width of the grid
//  * @param height - height of the grid
//  * @param padding - The number of pixels that extend around the main data
//  */
// function remap(point: VectorPoint, width: number, height: number, padding: number): void {
//   const { round } = Math;
//   const activeWidth = width - 1 - 2 * padding;
//   const activeHeight = height - 1 - 2 * padding;
//   point.x = round(((point.x + 0.5 - padding) * 32_768) / activeWidth);
//   point.y = round(((point.y + 0.5 - padding) * 32_768) / activeHeight);
// }
