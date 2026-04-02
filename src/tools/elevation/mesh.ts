import { getElevationGrid } from './grid.js';

import type { ElevationConverter } from '../../index.js';

/** The Resultant mesh created from elevation data */
export interface TerrainMesh {
  /** The size of the elevation grid */
  gridSize: number;
  /** The complete elevation grid */
  terrain: number[];
  /** The vertices of the mesh to be rendered */
  vertices: number[];
  /** The triangles of the mesh to be rendered */
  triangles: number[];
}

/**
 * # Build Terrain Mesh
 *
 * ## Description
 *
 * Builds a triangular mesh from elevation data. Useful for rendering elevation data as a 3D model.
 *
 * This is a port of the [martini](https://github.com/mapbox/martini) codebase to be compatible with this library.
 *
 * NOTE: Defaults to the Mapbox elevation data converter `convertMapboxElevationData`. However,
 * to use the Terrarium elevation data converter, use `convertTerrariumElevationData`.
 *
 * NOTE: This algorithm is limited to a GRID, meaning both the width and height must be equal and a power of 2.
 *
 * ## Examples
 *
 * ```ts
 * import { buildTerrainMesh } from 'gis-tools-ts';
 * // If using Terrarium elevation data:
 * // import { convertTerrariumElevationData } from 'gis-tools-ts';
 *
 * const elevationImage = await Bun.file(`${__dirname}/fixtures/fuji.png`).arrayBuffer();
 * const { terrain, vertices, triangles } = await buildTerrainMesh(elevationImage, 500, undefined, false);
 * ```
 *
 * ## Links
 * - <https://github.com/mapbox/martini/tree/main>
 *
 * @param image - the raw RGB(A) image data
 * @param maxError - The maximum error allowed in the mesh in meters
 * @param elevationConverter - the conversion function to convert the pixels to elevation
 * @param tmsStyle - if true, the y position will be inverted
 * @returns the terrain mesh. The elevation values.
 */
export async function buildTerrainMesh(
  image:
    | ArrayBufferLike
    | Uint8Array<ArrayBufferLike>
    | Uint8ClampedArray<ArrayBufferLike>
    | Buffer<ArrayBufferLike>
    | ImageData,
  maxError = 0,
  elevationConverter?: ElevationConverter,
  tmsStyle?: boolean,
): Promise<TerrainMesh> {
  const { max, abs } = Math;
  const grid = await getElevationGrid(image, elevationConverter, tmsStyle);

  // PREPARE MESH //
  const gridSize = grid.width + 1;
  const tileSize = grid.width;
  if ((tileSize & (tileSize - 1)) !== 0) {
    throw new Error(`Expected grid size to be 2^n+1, got ${gridSize}.`);
  }
  const numTriangles = tileSize * tileSize * 2 - 2;
  const numParentTriangles = numTriangles - tileSize * tileSize;

  // clone the elevation grid over to the terrain grid without the backfill
  const terrain = new Array(gridSize * gridSize);
  for (let y = 0; y < tileSize; y++) {
    for (let x = 0; x < tileSize; x++) {
      terrain[y * gridSize + x] = grid.elevations[y * tileSize + x];
    }
  }
  // backfill right and bottom borders
  for (let x = 0; x < gridSize - 1; x++) {
    terrain[gridSize * (gridSize - 1) + x] = terrain[gridSize * (gridSize - 2) + x];
  }
  for (let y = 0; y < gridSize; y++) {
    terrain[gridSize * y + gridSize - 1] = terrain[gridSize * y + gridSize - 2];
  }

  // coordinates for all possible triangles in an RTIN tile
  const errors = new Float32Array(terrain.length);
  const coords = new Uint16Array(numTriangles * 4);
  const indices = new Uint32Array(gridSize * gridSize).fill(0);

  // get triangle coordinates from its index in an implicit binary tree
  for (let i = 0; i < numTriangles; i++) {
    let id = i + 2;
    let ax = 0,
      ay = 0,
      bx = 0,
      by = 0,
      cx = 0,
      cy = 0;
    if ((id & 1) !== 0) {
      bx = by = cx = tileSize; // bottom-left triangle
    } else {
      ax = ay = cy = tileSize; // top-right triangle
    }
    while ((id >>= 1) > 1) {
      const mx = (ax + bx) >> 1;
      const my = (ay + by) >> 1;

      if ((id & 1) !== 0) {
        // left half
        bx = ax;
        by = ay;
        ax = cx;
        ay = cy;
      } else {
        // right half
        ax = bx;
        ay = by;
        bx = cx;
        by = cy;
      }
      cx = mx;
      cy = my;
    }
    const k = i * 4;
    coords[k + 0] = ax;
    coords[k + 1] = ay;
    coords[k + 2] = bx;
    coords[k + 3] = by;
  }

  // UPDATE //

  // iterate over all possible triangles, starting from the smallest level
  for (let i = numTriangles - 1; i >= 0; i--) {
    const k = i * 4;
    const ax = coords[k + 0];
    const ay = coords[k + 1];
    const bx = coords[k + 2];
    const by = coords[k + 3];
    const mx = (ax + bx) >> 1;
    const my = (ay + by) >> 1;
    const cx = mx + my - ay;
    const cy = my + ax - mx;

    // calculate error in the middle of the long edge of the triangle
    const interpolatedHeight = (terrain[ay * gridSize + ax] + terrain[by * gridSize + bx]) / 2;
    const middleIndex = my * gridSize + mx;
    const middleError = abs(interpolatedHeight - terrain[middleIndex]);

    errors[middleIndex] = max(errors[middleIndex], middleError);

    if (i < numParentTriangles) {
      // bigger triangles; accumulate error with children
      const leftChildIndex = ((ay + cy) >> 1) * gridSize + ((ax + cx) >> 1);
      const rightChildIndex = ((by + cy) >> 1) * gridSize + ((bx + cx) >> 1);
      errors[middleIndex] = max(
        errors[middleIndex],
        errors[leftChildIndex],
        errors[rightChildIndex],
      );
    }
  }

  // CREATE MESH //

  let numVertices = 0;
  let resTriangles = 0;
  const maxSize = gridSize - 1;

  // retrieve mesh in two stages that both traverse the error map:
  // - countElements: find used vertices (and assign each an index), and count triangles (for minimum allocation)
  // - processTriangle: fill the allocated vertices & triangles typed arrays

  function countElements(
    ax: number,
    ay: number,
    bx: number,
    by: number,
    cx: number,
    cy: number,
  ): void {
    const mx = (ax + bx) >> 1;
    const my = (ay + by) >> 1;

    if (abs(ax - cx) + abs(ay - cy) > 1 && errors[my * gridSize + mx] > maxError) {
      countElements(cx, cy, ax, ay, mx, my);
      countElements(bx, by, cx, cy, mx, my);
    } else {
      indices[ay * gridSize + ax] =
        indices[ay * gridSize + ax] !== 0 ? indices[ay * gridSize + ax] : ++numVertices;
      indices[by * gridSize + bx] =
        indices[by * gridSize + bx] !== 0 ? indices[by * gridSize + bx] : ++numVertices;
      indices[cy * gridSize + cx] =
        indices[cy * gridSize + cx] !== 0 ? indices[cy * gridSize + cx] : ++numVertices;
      resTriangles++;
    }
  }
  countElements(0, 0, maxSize, maxSize, maxSize, 0);
  countElements(maxSize, maxSize, 0, 0, 0, maxSize);

  const vertices = new Array(numVertices * 2); // Uint16Array
  const triangles = new Array(resTriangles * 3); // Uint32Array
  let triIndex = 0;

  function processTriangle(
    ax: number,
    ay: number,
    bx: number,
    by: number,
    cx: number,
    cy: number,
  ): void {
    const mx = (ax + bx) >> 1;
    const my = (ay + by) >> 1;

    if (abs(ax - cx) + abs(ay - cy) > 1 && errors[my * gridSize + mx] > maxError) {
      // triangle doesn't approximate the surface well enough; drill down further
      processTriangle(cx, cy, ax, ay, mx, my);
      processTriangle(bx, by, cx, cy, mx, my);
    } else {
      // add a triangle
      const a = indices[ay * gridSize + ax] - 1;
      const b = indices[by * gridSize + bx] - 1;
      const c = indices[cy * gridSize + cx] - 1;

      vertices[2 * a] = ax;
      vertices[2 * a + 1] = ay;

      vertices[2 * b] = bx;
      vertices[2 * b + 1] = by;

      vertices[2 * c] = cx;
      vertices[2 * c + 1] = cy;

      triangles[triIndex++] = a;
      triangles[triIndex++] = b;
      triangles[triIndex++] = c;
    }
  }

  processTriangle(0, 0, maxSize, maxSize, maxSize, 0);
  processTriangle(maxSize, maxSize, 0, 0, 0, maxSize);

  return { gridSize, terrain, vertices, triangles };
}
