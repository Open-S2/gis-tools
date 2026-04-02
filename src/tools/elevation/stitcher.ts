// import { equalPoints } from '../../index.js';

// import type { IsolineSegment, MarchingSquaresResult } from './marching_old.js';
// import type {
//   MValue,
//   Properties,
//   VectorLineString,
//   VectorMultiLineString,
//   VectorMultiLineStringFeature,
//   VectorPoint,
// } from '../../index.js';

// /** `${from.x}_${from.y}`: To Segment */
// export type SegmentLookup = Map<string, IsolineSegment[]>;

// export interface ElevationProperties extends Properties {
//   elev: number;
//   elevFt: number;
// }

// export type ElevationFeature = VectorMultiLineStringFeature<
//   Record<string, unknown>,
//   MValue,
//   ElevationProperties
// >;

// export function stitchMarchingSquareSegments(
//   marchingSquares: MarchingSquaresResult,
// ): ElevationFeature[] {
//   const result: ElevationFeature[] = [];

//   for (const [threshold, segments] of marchingSquares) {
//     // setup graph for threshold
//     const pointToSegments = new Map<string, IsolineSegment[]>();
//     for (const seg of segments) {
//       const p1 = `${seg.from.x}_${seg.from.y}`;
//       const p2 = `${seg.to.x}_${seg.to.y}`;
//       if (!pointToSegments.has(p1)) pointToSegments.set(p1, []);
//       if (!pointToSegments.has(p2)) pointToSegments.set(p2, []);
//       pointToSegments.get(p1)!.push(seg);
//       pointToSegments.get(p2)!.push(seg);
//     }

//     const coordinates: VectorMultiLineString = [];
//     // Stitching
//     for (const startSeg of segments) {
//       if (startSeg.visited) continue;

//       const line: VectorLineString = [startSeg.from, startSeg.to];
//       startSeg.visited = true;

//       // Grow forward from 'to'
//       growLine(line, startSeg.to, pointToSegments);
//       // Grow backward from 'from' (and reverse the result)
//       const backwardPoints: VectorLineString = [];
//       growLine(backwardPoints, startSeg.from, pointToSegments);
//       line.unshift(...backwardPoints.reverse());

//       if (line.length <= 2) continue;

//       // remap line to 0->1
//       for (const point of line) {
//         point.x = point.x / 32_768;
//         point.y = point.y / 32_768;
//       }
//       coordinates.push(line);
//     }

//     if (coordinates.length === 0) continue;
//     result.push({
//       type: 'VectorFeature',
//       properties: { elev: threshold, elevFt: threshold * 3.28084 },
//       geometry: { type: 'MultiLineString', coordinates, is3D: false },
//     });
//   }

//   return result;
// }

// function growLine(
//   line: VectorLineString,
//   currentPt: VectorPoint,
//   graph: Map<string, IsolineSegment[]>,
// ) {
//   while (true) {
//     const key = `${currentPt.x}_${currentPt.y}`;
//     const neighbors = graph.get(key) ?? [];
//     const nextSeg = neighbors.find((s) => !s.visited);
//     if (nextSeg === undefined) break;

//     nextSeg.visited = true;
//     // Figure out which end of the segment is the new point
//     const nextPt = equalPoints(nextSeg.from, currentPt) ? nextSeg.to : nextSeg.from;
//     line.push(nextPt);
//     currentPt = nextPt;
//   }
// }
