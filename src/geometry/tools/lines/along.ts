import { pointBearing, pointDestination } from '../../../index.js';

import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineString,
  VectorLineStringGeometry,
  VectorPoint,
} from '../../../index.js';

/**
 * Given a linestring in degrees and a distance, create a [`VectorPoint`] along the line
 * @param input - The linestring to interpolate along
 * @param distance - The distance to interpolate in meters
 * @param radius - The radius of the sphere. Defaults to the Earth's radius in meters
 * @returns - the interpolated point
 */
export function alongLine<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(
  input:
    | VectorLineString<D>
    | VectorLineStringGeometry<D>
    | VectorFeature<M, D, P, VectorLineStringGeometry<D>>,
  distance: number,
  radius?: number,
): VectorPoint {
  const { sqrt, pow } = Math;
  const vectorLines: VectorLineString<D> =
    'geometry' in input
      ? input.geometry.coordinates
      : 'coordinates' in input
        ? input.coordinates
        : input;
  let travelled = 0;

  for (let i = 0; i < vectorLines.length; i++) {
    if (distance >= travelled && i === vectorLines.length - 1) {
      break;
    } else if (travelled >= distance) {
      const overshot = distance - travelled;
      if (overshot === 0) {
        return { x: vectorLines[i].x, y: vectorLines[i].y };
      } else {
        const direction = pointBearing(vectorLines[i], vectorLines[i - 1]) - 180;
        const interpolated = pointDestination(vectorLines[i], overshot, direction, radius);
        return interpolated;
      }
    } else {
      travelled += sqrt(
        pow(vectorLines[i].x - vectorLines[i + 1].x, 2) +
          pow(vectorLines[i].y - vectorLines[i + 1].y, 2),
      );
    }
  }

  const last = vectorLines.length - 1;
  return { x: vectorLines[last].x, y: vectorLines[last].y };
}
