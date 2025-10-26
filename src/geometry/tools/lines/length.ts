import { llGetDistance } from '../../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineString,
  VectorLineStringGeometry,
} from '../../../index.js';

/**
 * Find the length of a linestring. No projection is assumed
 * @param input - the linestring as either a VectorFeature, VectorLineStringGeometry, or raw VectorLineString
 * @param haversine - if set to true, uses the Haversine formula on lon-lat points. Defaults to false.
 * @returns - the raw length of the linestring, but if haversine is set to true the result is in radians.
 */
export function lineLength<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  input:
    | VectorLineString<D>
    | VectorLineStringGeometry<D>
    | VectorFeature<M, D, P, VectorLineStringGeometry<D>>,
  haversine: boolean = false,
): number {
  const { sqrt, pow } = Math;
  const vectorLines: VectorLineString<D> =
    'geometry' in input
      ? input.geometry.coordinates
      : 'coordinates' in input
        ? input.coordinates
        : input;

  let length = 0;

  for (let i = 0; i < vectorLines.length - 1; i++) {
    if (haversine) {
      length += llGetDistance(vectorLines[i], vectorLines[i + 1]);
    } else {
      length += sqrt(
        pow(vectorLines[i + 1].x - vectorLines[i].x, 2) +
          pow(vectorLines[i + 1].y - vectorLines[i].y, 2),
      );
    }
  }
  return length;
}
