import { PriorityQueue } from '../dataStructures/priorityQueue';
import { Properties, VectorMultiPolygon, VectorPoint, VectorPolygon } from '../geometry';

/** The metadata inserted into the Vector Feature */
export interface PolyLabelMetadata extends Properties {
  distance: number;
}

/**
 * # Polylabels
 *
 * ## Description
 * Find the labels for a collection of vector polygons
 *
 * ## Usage
 * ```ts
 * import { polylabels } from 'gis-tools-ts'
 * import type { VectorMultiPolygon } from 'gis-tools-ts'
 *
 * const vectorGeometry: VectorMultiPolygon = [];
 * const polylabelHighPrecision = polylabels(vectorGeometry, 1);
 * ```
 *
 * ## Links
 * - https://sites.google.com/site/polesofinaccessibility/
 * @param polygons - A collection of vector polygons to find the labels for
 * @param precision - the precision of the label [default: 1.0]
 * @returns - the labels
 */
export function polylabels(
  polygons: VectorMultiPolygon,
  precision = 1.0,
): VectorPoint<PolyLabelMetadata>[] {
  return polygons.map((polygon) => polylabel(polygon, precision));
}

/**
 * # Polylabel
 *
 * ## Description
 * Find the label for a vector polygon
 *
 * ## Usage
 * ```ts
 * import { polylabel } from 'gis-tools-ts'
 * import type { VectorPolygon } from 'gis-tools-ts'
 *
 * const vectorGeometry: VectorPolygon = [];
 * const polylabelHighPrecision = polylabel(vectorGeometry, 1);
 * ```
 *
 * ## Links
 * - https://sites.google.com/site/polesofinaccessibility/
 * @param polygon - the vector polygon to find the label for
 * @param precision - the precision of the label [default: 1.0]
 * @returns - the label
 */
export function polylabel(polygon: VectorPolygon, precision = 1.0): VectorPoint<PolyLabelMetadata> {
  // find the bounding box of the outer ring
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;

  for (const { x, y } of polygon[0]) {
    if (x < minX) minX = x;
    if (y < minY) minY = y;
    if (x > maxX) maxX = x;
    if (y > maxY) maxY = y;
  }

  const width = maxX - minX;
  const height = maxY - minY;
  const cellSize = Math.max(precision, Math.min(width, height));

  if (cellSize === precision) return { x: minX, y: minY, m: { distance: 0 } };

  // a priority queue of cells in order of their "potential" (max distance to polygon)
  const cellQueue = new PriorityQueue<PolyLabelCell>(
    [],
    (a: PolyLabelCell, b: PolyLabelCell): number => b.max - a.max,
  );

  // take centroid as the first best guess
  let bestCell = getCentroidCell(polygon);

  // second guess: bounding box centroid
  const bboxCell = buildCell(minX + width / 2, minY + height / 2, 0, polygon);
  if (bboxCell.d > bestCell.d) bestCell = bboxCell;

  /**
   * add a cell to the queue
   * @param x - the cell x coordinate
   * @param y - the cell y coordinate
   * @param h - the cell height
   */
  const potentiallyQueue = (x: number, y: number, h: number): void => {
    const cell = buildCell(x, y, h, polygon);
    if (cell.max > bestCell.d + precision) cellQueue.push(cell);

    // update the best cell if we found a better one
    if (cell.d > bestCell.d) bestCell = cell;
  };

  // cover polygon with initial cells
  let h = cellSize / 2;
  for (let x = minX; x < maxX; x += cellSize) {
    for (let y = minY; y < maxY; y += cellSize) {
      potentiallyQueue(x + h, y + h, h);
    }
  }

  while (true) {
    // pick the most promising cell from the queue
    const cell = cellQueue.pop();
    if (cell === undefined) break;
    const { max, x, y, h: ch } = cell;

    // do not drill down further if there's no chance of a better solution
    if (max - bestCell.d <= precision) break;

    // split the cell into four cells
    h = ch / 2;
    potentiallyQueue(x - h, y - h, h);
    potentiallyQueue(x + h, y - h, h);
    potentiallyQueue(x - h, y + h, h);
    potentiallyQueue(x + h, y + h, h);
  }

  const result = { x: bestCell.x, y: bestCell.y, m: { distance: bestCell.d } };
  return result;
}

/** A cell in the polygon label algorithm */
interface PolyLabelCell {
  /** cell center x */
  x: number;
  /** cell center y */
  y: number;
  /** half the cell size */
  h: number;
  /** distance from cell center to polygon */
  d: number;
  /** max distance to polygon within a cell */
  max: number;
}

/**
 * build a cell
 * @param x - the cell x coordinate
 * @param y - the cell y coordinate
 * @param h - half the cell size
 * @param polygon - the vector polygon
 * @returns - the cell
 */
function buildCell(x: number, y: number, h: number, polygon: VectorPolygon): PolyLabelCell {
  const d = pointToPolygonDist(x, y, polygon);
  return { x, y, h, d, max: d + h * Math.SQRT2 };
}

/**
 * signed distance from point to polygon outline (negative if point is outside)
 * @param x - the point x coordinate
 * @param y - the point y coordinate
 * @param polygon - the vector polygon to check
 * @returns - the signed distance
 */
function pointToPolygonDist(x: number, y: number, polygon: VectorPolygon): number {
  let inside = false;
  let minDistSq = Infinity;

  for (const ring of polygon) {
    for (let i = 0, len = ring.length, j = len - 1; i < len; j = i++) {
      const a = ring[i];
      const b = ring[j];

      if (a.y > y !== b.y > y && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x)
        inside = !inside;

      minDistSq = Math.min(minDistSq, getSegDistSq(x, y, a, b));
    }
  }

  return minDistSq === 0 ? 0 : (inside ? 1 : -1) * Math.sqrt(minDistSq);
}

/**
 * get polygon centroid
 * @param polygon - the vector polygon
 * @returns - the centroid as a cell
 */
function getCentroidCell(polygon: VectorPolygon): PolyLabelCell {
  let area = 0;
  let x = 0;
  let y = 0;
  const points = polygon[0];

  for (let i = 0, len = points.length, j = len - 1; i < len; j = i++) {
    const a = points[i];
    const b = points[j];
    const f = a.x * b.y - b.x * a.y;
    x += (a.x + b.x) * f;
    y += (a.y + b.y) * f;
    area += f * 3;
  }
  const centroid = buildCell(x / area, y / area, 0, polygon);
  if (area === 0 || centroid.d < 0) return buildCell(points[0].x, points[0].y, 0, polygon);
  return centroid;
}

/**
 * get squared distance from a point to a segment AB
 * @param px - the segment start point
 * @param py - the segment end point
 * @param a - the reference point A
 * @param b - the reference point B
 * @returns - the squared distance
 */
function getSegDistSq(px: number, py: number, a: VectorPoint, b: VectorPoint): number {
  let { x, y } = a;
  let dx = b.x - x;
  let dy = b.y - y;

  if (dx !== 0 || dy !== 0) {
    const t = ((px - x) * dx + (py - y) * dy) / (dx * dx + dy * dy);

    if (t > 1) {
      x = b.x;
      y = b.y;
    } else if (t > 0) {
      x += dx * t;
      y += dy * t;
    }
  }

  dx = px - x;
  dy = py - y;

  return dx * dx + dy * dy;
}
