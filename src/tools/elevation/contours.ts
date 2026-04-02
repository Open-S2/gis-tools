import {
  buildSqDist,
  convertMapboxElevationData,
  getElevationGrid,
  polygonRingArea,
  polylineInPolyline,
  simplifyLine,
} from '../../index.js';

import type {
  ElevationConverter,
  MValue,
  Properties,
  VectorFeatureCollection,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorMultiPolygonFeature,
  VectorMultiPolygonGeometry,
  VectorPoint,
} from '../../index.js';

export interface ElevationProperties extends Properties {
  elev: number;
  elevFt: number;
}

export type ContourFeature = VectorMultiPolygonFeature<
  Record<string, unknown>,
  MValue,
  ElevationProperties
>;

export type ContourFeatureCollection = VectorFeatureCollection<
  Record<string, unknown>,
  MValue,
  ElevationProperties,
  VectorMultiPolygonGeometry
>;

interface Fragment {
  start: number;
  end: number;
  ring: VectorLineString;
}

// prettier-ignore
const MS_LUT = [
  [], // 0
  [[{ x: 1.0, y: 1.5 }, { x: 0.5, y: 1.0 }]], // 1
  [[{ x: 1.5, y: 1.0 }, { x: 1.0, y: 1.5 }]], // 2
  [[{ x: 1.5, y: 1.0 }, { x: 0.5, y: 1.0 }]], // 3
  [[{ x: 1.0, y: 0.5 }, { x: 1.5, y: 1.0 }]], // 4
  [[{ x: 1.0, y: 1.5 }, { x: 0.5, y: 1.0 }],[{ x: 1.0, y: 0.5 }, { x: 1.5, y: 1.0 }]], // 5
  [[{ x: 1.0, y: 0.5 }, { x: 1.0, y: 1.5 }]], // 6
  [[{ x: 1.0, y: 0.5 }, { x: 0.5, y: 1.0 }]], // 7
  [[{ x: 0.5, y: 1.0 }, { x: 1.0, y: 0.5 }]], // 8
  [[{ x: 1.0, y: 1.5 }, { x: 1.0, y: 0.5 }]], // 9
  [[{ x: 0.5, y: 1.0 }, { x: 1.0, y: 0.5 }],[{ x: 1.5, y: 1.0 }, { x: 1.0, y: 1.5 }]], // 10
  [[{ x: 1.5, y: 1.0 }, { x: 1.0, y: 0.5 }]], // 11
  [[{ x: 0.5, y: 1.0 }, { x: 1.5, y: 1.0 }]], // 12
  [[{ x: 1.0, y: 1.5 }, { x: 1.5, y: 1.0 }]], // 13
  [[{ x: 0.5, y: 1.0 }, { x: 1.0, y: 1.5 }]], // 14
  [], // 15
];

/**
 * Generate isoline thresholds given a min, max, and step.
 * @param min - minimum value
 * @param max - maximum value
 * @param step - step size
 * @returns a collection of thresholds within the range provided all inclusive and sorted
 */
export function isolineThresholds(min: number, max: number, step: number): number[] {
  const thresholds: number[] = [];
  let current = Math.ceil(min / step) * step;

  while (current <= max) {
    thresholds.push(current === 0 ? 0 : current); // avoid -0
    current += step;
  }

  return thresholds;
}

/**
 * # Create Isolines
 *
 * ## Description
 * Creates isolines from an image.
 *
 * NOTE: Defaults to the Mapbox elevation data converter `convertMapboxElevationData`. However,
 * to use the Terrarium elevation data converter, use `convertTerrariumElevationData`.
 *
 * NOTE: Using a buffer with a small padding works, but it only extends the line ends of the isolines.
 * A better method would be to pull in grid data with large padding (second example).
 * If you have a large padding like 16px then you don't need a buffer, so you should set it to 0.
 *
 * ## Examples
 *
 * ### Create Isolines
 * ```ts
 * import 'gis-tools-ts/polyfills/local'; // You may need this to handle image data if you are using the file system.
 * import { buildContours } from 'gis-tools-ts';
 *
 * // Pull in the image data. We are using a local image file that is 514x514 with prebuilt padding
 * const elevationImage = await Bun.file(
 *   './tests/tools/isobands/fixtures/13_1556_3084.webp',
 * ).arrayBuffer();
 *
 * // Create the isolines.
 * const isolines = await buildContours(elevationImage);
 * ```
 *
 * ### High quality with large padding
 * ```ts
 * import { RasterTilesFileReader } from 'gis-tools-ts/file'; // because we use file.js local polyfills already added
 * import { buildContours } from 'gis-tools-ts';
 *
 * // Setup a tile reader
 * const reader = new RasterTilesFileReader(
 *   `${__dirname}/../../readers/tile/fixtures/wm/terrain-v2`,
 * );
 * const metadata = await reader.getMetadata();
 * const isTMS = metadata.scheme === 'tms';
 * // read in the elevation data with a big bigger padding
 * // we are pulling in Canada/Greenland area at zoom 3. Padding of 16 so a resulting 544x544 image.
 * const padding = 16;
 * const tile = await reader.getTileWithPaddingWM(3, 3, 1, padding, 512, 512);
 * if (tile === undefined) throw new Error('Tile not found');
 * // build the isolines
 * const isolines = await buildContours(tile!.image, undefined, 1_000, isTMS, padding, 0);
 * ```
 *
 * @param imageData - the raw RGB(A) image data
 * @param elevationConverter - the conversion function to convert the pixels to elevation
 * @param step - the step size for the heightmap. Defaults to 100 meters for the Mapbox elevation data.
 * @param tmsStyle - if true, the y position will be inverted
 * @param padding - The number of pixels that extend around the main data
 * @param tolerance - The Douglas-Peucker tolerance. Defaults to `1 / 2_096`
 * @returns The isolines stored in a FeatureCollection
 */
export async function buildContours(
  imageData:
    | ArrayBufferLike
    | Uint8Array<ArrayBufferLike>
    | Uint8ClampedArray<ArrayBufferLike>
    | Buffer<ArrayBufferLike>
    | ImageData,
  elevationConverter: ElevationConverter = convertMapboxElevationData,
  step = 100,
  tmsStyle = false,
  padding = 1,
  tolerance = 1 / 2_096,
): Promise<ContourFeatureCollection> {
  // create the elevation grid
  const elevationGrid = await getElevationGrid(imageData, elevationConverter, tmsStyle);
  const { width, height, min, max, elevations } = elevationGrid;
  // setup the thresholds
  const thresholds = isolineThresholds(min, max, step);

  const features: ContourFeature[] = [];
  for (const threshold of thresholds) {
    const coordinates = buildIsoBands(elevations, threshold, width, height, padding, tolerance);
    features.push({
      type: 'VectorFeature',
      properties: { elev: threshold, elevFt: threshold * 3.28084 },
      geometry: { type: 'MultiPolygon', is3D: false, coordinates },
    });
  }

  return { type: 'FeatureCollection', features };
}

// Accumulate, smooth contour rings, assign holes to exterior rings.
// Based on https://github.com/mbostock/shapefile/blob/v0.6.2/shp/polygon.js
// @param tolerance - The Douglas-Peucker tolerance. A good default is `1 / 2_096`
export function buildIsoBands(
  values: number[],
  threshold: number,
  width: number,
  height: number,
  padding: number,
  tolerance = 0,
): VectorMultiPolygon {
  const polygons: VectorMultiPolygon = [];
  const holes: VectorLineString[] = [];

  const isorings = buildIsorings(values, threshold, width, height, padding, tolerance);
  // Store rings in the correct groups. Skip rings with zero area.
  for (const ring of isorings) {
    const area = polygonRingArea(ring, 1);
    if (area === 0) continue;
    else if (area > 0) polygons.push([ring]);
    else holes.push(ring);
  }
  // sort the holes into their correct polygons
  for (const hole of holes) {
    for (const polygon of polygons) {
      if (polylineInPolyline(hole, polygon[0])) {
        polygon.push(hole);
        break;
      }
    }
  }

  return polygons;
}

// Marching squares with isolines stitched into rings.
// Based on https://github.com/topojson/topojson-client/blob/v3.0.0/src/stitch.js
// @param tolerance - The Douglas-Peucker tolerance. A good default is `1 / 2_096`
export function buildIsorings(
  values: number[],
  threshold: number,
  width: number,
  height: number,
  padding: number,
  tolerance = 0,
): VectorMultiLineString {
  const res: VectorMultiLineString = [];
  const fragByStart = new Map<number, Fragment>();
  const fragByEnd = new Map<number, Fragment>();
  let x: number;
  let y: number;
  let t0: number;
  let t1: number;
  let t2: number;
  let t3: number;
  // Convert a point to an index.
  const index = (point: VectorPoint, width: number): number => {
    return point.x * 2 + point.y * (width + 1) * 4;
  };
  // Convert to a number (0 or 1) whether the value is above the threshold.
  const above = (x: number | undefined, value: number): number => {
    return x === undefined ? 0 : Number(x >= value);
  };

  // Special case for the first row (y = -1, t2 = t3 = 0).
  x = y = -1;
  t1 = above(values[0], threshold);
  MS_LUT[t1 << 1].forEach(stitch);
  while (++x < width - 1) {
    t0 = t1;
    t1 = above(values[x + 1], threshold);
    MS_LUT[t0 | (t1 << 1)].forEach(stitch);
  }
  MS_LUT[t1 << 0].forEach(stitch);

  // General case for the intermediate rows.
  while (++y < height - 1) {
    x = -1;
    t1 = above(values[y * width + width], threshold);
    t2 = above(values[y * width], threshold);
    MS_LUT[(t1 << 1) | (t2 << 2)].forEach(stitch);
    while (++x < width - 1) {
      t0 = t1;
      t1 = above(values[y * width + width + x + 1], threshold);
      t3 = t2;
      t2 = above(values[y * width + x + 1], threshold);
      MS_LUT[t0 | (t1 << 1) | (t2 << 2) | (t3 << 3)].forEach(stitch);
    }
    MS_LUT[t1 | (t2 << 3)].forEach(stitch);
  }

  // Special case for the last row (y = height - 1, t0 = t1 = 0).
  x = -1;
  t2 = Number(values[y * width] >= threshold);
  MS_LUT[t2 << 2].forEach(stitch);
  while (++x < width - 1) {
    t3 = t2;
    t2 = above(values[y * width + x + 1], threshold);
    MS_LUT[(t2 << 2) | (t3 << 3)].forEach(stitch);
  }
  MS_LUT[t2 << 3].forEach(stitch);

  function stitch(line: VectorLineString): void {
    const start = { x: line[0].x + x, y: line[0].y + y };
    const end = { x: line[1].x + x, y: line[1].y + y };
    const startIndex = index(start, width);
    const endIndex = index(end, width);

    let f = fragByEnd.get(startIndex);
    let g = fragByStart.get(endIndex);

    if (f !== undefined) {
      if (g !== undefined) {
        fragByEnd.delete(f.end);
        fragByStart.delete(g.start);
        if (f === g) {
          f.ring.push(end);
          res.push(f.ring);
        } else {
          const frag = {
            start: f.start,
            end: g.end,
            ring: f.ring.concat(g.ring),
          };
          fragByStart.set(f.start, frag);
          fragByEnd.set(g.end, frag);
        }
      } else {
        fragByEnd.delete(f.end);
        f.ring.push(end);
        f.end = endIndex;
        fragByEnd.set(endIndex, f);
      }
    } else if ((f = fragByStart.get(endIndex)) !== undefined) {
      if ((g = fragByEnd.get(startIndex)) !== undefined) {
        fragByStart.delete(f.start);
        fragByEnd.delete(g.end);
        if (f === g) {
          f.ring.push(end);
          res.push(f.ring);
        } else {
          const frag = {
            start: g.start,
            end: f.end,
            ring: g.ring.concat(f.ring),
          };
          fragByStart.set(g.start, frag);
          fragByEnd.set(f.end, frag);
        }
      } else {
        fragByStart.delete(f.start);
        f.ring.unshift(start);
        f.start = startIndex;
        fragByStart.set(startIndex, f);
      }
    } else {
      const frag = { start: startIndex, end: endIndex, ring: [start, end] };
      fragByStart.set(startIndex, frag);
      fragByEnd.set(endIndex, frag);
    }
  }

  for (let i = 0; i < res.length; i++) {
    const ring = res[i];
    // reverse the ring
    ring.reverse();
    // smooth the edges
    smooth(ring, values, threshold, width, height);
    // convert points to 0->1
    for (const point of ring) remap(point, width, height, padding);
    // lastly simplify if tolerance is not 0
    if (tolerance !== 0) {
      // Prep Douglas-Peucker simplification by setting t-values.
      buildSqDist(ring, 0, ring.length - 1, tolerance * tolerance);
      // Apply Douglas-Peucker
      res[i] = simplifyLine(ring, tolerance, false, false);
    }
  }

  return res;
}

function smooth(
  ring: VectorLineString,
  values: number[],
  threshold: number,
  width: number,
  height: number,
): void {
  ring.forEach((point): void => {
    const { x, y } = point;
    const xt = x | 0;
    const yt = y | 0;
    const v1 = valid(values[yt * width + xt]);
    if (x > 0 && x < width && xt === x) {
      point.x = smooth1(x, valid(values[yt * width + xt - 1]), v1, threshold);
    }
    if (y > 0 && y < height && yt === y) {
      point.y = smooth1(y, valid(values[(yt - 1) * width + xt]), v1, threshold);
    }
  });
}

function valid(v: number | undefined): number {
  return v === undefined || isNaN(v) ? -Infinity : v;
}

function smooth1(x: number, v0: number, v1: number, value: number): number {
  const a = value - v0;
  const b = v1 - v0;
  const d = isFinite(a) || isFinite(b) ? a / b : Math.sign(a) / Math.sign(b);
  return isNaN(d) ? x : x + d - 0.5;
}

// Convert the x-y values from width-height scale to 0->1.
// Input data was already offest to represent cell centers ([0.5, 0.5] start)
function remap(point: VectorPoint, width: number, height: number, padding: number): void {
  const activeWidth = width - 2 * padding;
  const activeHeight = height - 2 * padding;
  point.x = (point.x - padding) / activeWidth;
  point.y = (point.y - padding) / activeHeight;
}
