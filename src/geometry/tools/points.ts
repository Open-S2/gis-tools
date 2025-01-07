import type { Properties, VectorFeature, VectorMultiPointGeometry, VectorPoint } from '../..';

/**
 * Find the average of a collection of Vector points
 * @param vectorPoints - collection of Vector points, whether from a VectorFeature, geometry, or raw coordinates
 * @returns - the average of the vector points
 */
export function averageOfPoints(
  vectorPoints:
    | VectorPoint[]
    | VectorMultiPointGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorMultiPointGeometry>,
): VectorPoint {
  const coords =
    'geometry' in vectorPoints
      ? vectorPoints.geometry.coordinates
      : 'coordinates' in vectorPoints
        ? vectorPoints.coordinates
        : vectorPoints;
  if (coords.length === 0) return { x: 0, y: 0 };
  let xAvg = 0;
  let yAvg = 0;
  let zAvg = 0;
  let hasZ = false;
  for (const { x, y, z } of coords) {
    xAvg += x;
    yAvg += y;
    if (z !== undefined) {
      zAvg += z;
      hasZ = true;
    }
  }
  xAvg /= coords.length;
  yAvg /= coords.length;
  zAvg /= coords.length;
  if (hasZ) return { x: xAvg, y: yAvg, z: zAvg };
  return { x: xAvg, y: yAvg };
}

/**
 * Find the center of a collection of Vector points
 * @param vectorPoints - collection of Vector points, whether from a VectorFeature, geometry, or raw coordinates
 * @returns - the center of the vector points
 */
export function centerOfPoints(
  vectorPoints:
    | VectorPoint[]
    | VectorMultiPointGeometry
    | VectorFeature<Record<string, unknown>, Properties, Properties, VectorMultiPointGeometry>,
): VectorPoint {
  const { min, max } = Math;
  const coords =
    'geometry' in vectorPoints
      ? vectorPoints.geometry.coordinates
      : 'coordinates' in vectorPoints
        ? vectorPoints.coordinates
        : vectorPoints;
  let minX = Infinity;
  let maxX = -Infinity;
  let minY = Infinity;
  let maxY = -Infinity;
  let minZ = Infinity;
  let maxZ = -Infinity;
  for (const { x, y, z } of coords) {
    minX = min(minX, x);
    maxX = max(maxX, x);
    minY = min(minY, y);
    maxY = max(maxY, y);
    if (z !== undefined) {
      minZ = min(minZ, z);
      maxZ = max(maxZ, z);
    }
  }
  const x = (minX + maxX) / 2;
  const y = (minY + maxY) / 2;
  if (minZ !== Infinity && maxZ !== -Infinity) return { x, y, z: (minZ + maxZ) / 2 };
  return { x, y };
}
