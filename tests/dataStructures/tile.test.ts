import { Tile, TileStore, transformPoint } from '../../src/dataStructures/tile';
import { expect, test } from 'bun:test';

import { childrenIJ, fromFace } from '../../src/geometry/id';

import type { FeatureCollection } from '../../src/geometry';

const SIMPLIFY_MAXZOOM = 16;

test('transformPoint', () => {
  const p = { x: 0, y: 0 };
  transformPoint(p, 1, 0, 0);
  expect(p).toEqual({ x: 0, y: 0 });
  transformPoint(p, 1, 1, 0);
  expect(p).toEqual({ x: -1, y: 0 });
});

test('Tile', () => {
  const tile = new Tile(0n);
  expect(tile).toEqual({
    id: 0n,
    layers: {},
    transformed: false,
  } as Tile);

  expect(tile.isEmpty()).toBe(true);

  tile.addFeature(
    {
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'Point',
        is3D: false,
        coordinates: { x: 0, y: 0 },
      },
    },
    'default',
  );

  expect(tile.isEmpty()).toBe(false);

  tile.transform(3, SIMPLIFY_MAXZOOM);

  expect(tile).toEqual({
    id: 0n,
    transformed: true,
    layers: {
      default: {
        name: 'default',
        features: [
          {
            type: 'VectorFeature',
            properties: {},
            geometry: {
              type: 'Point',
              is3D: false,
              coordinates: { x: 0, y: 0 },
            },
          },
        ],
      },
    },
  } as unknown as Tile);
});

test('TileStore - points', () => {
  const featureCollection: FeatureCollection = {
    type: 'FeatureCollection',
    features: [
      {
        type: 'Feature',
        properties: { a: 1 },
        geometry: {
          type: 'Point',
          coordinates: [0, 0],
        },
      },
      {
        type: 'Feature',
        properties: { b: 2 },
        geometry: {
          type: 'Point3D',
          coordinates: [45, 45, 1],
        },
      },
      {
        type: 'Feature',
        properties: { c: 3 },
        geometry: {
          type: 'MultiPoint',
          coordinates: [
            [-45, -45],
            [-45, 45],
          ],
        },
      },
      {
        type: 'Feature',
        properties: { d: 4 },
        geometry: {
          type: 'MultiPoint3D',
          coordinates: [
            [45, -45, 1],
            [-180, 20, 2],
          ],
        },
      },
    ],
  };

  const store = new TileStore(featureCollection, { projection: 'WM' });

  const faceID = fromFace(0);
  const faceTile = store.getTile(faceID);

  expect(faceTile).toEqual({
    id: 1152921504606846976n,
    layers: {
      default: {
        features: [
          {
            geometry: {
              bbox: undefined,
              coordinates: {
                m: undefined,
                x: 0.5,
                y: 0.5,
                z: undefined,
              },
              type: 'Point',
              is3D: false,
              vecBBox: [0.5, 0.5, 0.5, 0.5],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              a: 1,
            },
            type: 'VectorFeature',
          },
          {
            geometry: {
              bbox: undefined,
              coordinates: {
                m: undefined,
                x: 0.625,
                y: 0.35972503691520497,
                z: 1,
              },
              type: 'Point',
              is3D: true,
              vecBBox: [0.625, 0.35972503691520497, 0.625, 0.35972503691520497, 1, 1],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              b: 2,
            },
            type: 'VectorFeature',
          },
          {
            geometry: {
              bbox: undefined,
              coordinates: [
                {
                  m: undefined,
                  x: 0.375,
                  y: 0.640274963084795,
                  z: undefined,
                },
                {
                  m: undefined,
                  x: 0.375,
                  y: 0.35972503691520497,
                  z: undefined,
                },
              ],
              type: 'MultiPoint',
              is3D: false,
              vecBBox: [0.375, 0.35972503691520497, 0.375, 0.640274963084795],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              c: 3,
            },
            type: 'VectorFeature',
          },
          {
            geometry: {
              bbox: undefined,
              coordinates: [
                {
                  m: undefined,
                  x: 0.625,
                  y: 0.640274963084795,
                  z: 1,
                },
                {
                  m: undefined,
                  x: 0,
                  y: 0.4432805993614054,
                  z: 2,
                },
              ],
              type: 'MultiPoint',
              is3D: true,
              vecBBox: [0, 0.4432805993614054, 0.625, 0.640274963084795, 1, 2],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              d: 4,
            },
            type: 'VectorFeature',
          },
        ],
        name: 'default',
      },
    },
    transformed: true,
  } as unknown as Tile);

  const [childID] = childrenIJ(0, 0, 0, 0);
  const childTile = store.getTile(childID);
  expect(childTile).toEqual({
    id: 288230376151711744n,
    layers: {
      default: {
        features: [
          {
            geometry: {
              bbox: undefined,
              coordinates: [
                {
                  m: undefined,
                  x: 0.75,
                  y: 0.7194500738304099,
                  z: undefined,
                },
              ],
              type: 'MultiPoint',
              is3D: false,
              vecBBox: [0.375, 0.35972503691520497, 0.375, 0.35972503691520497],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              c: 3,
            },
            type: 'VectorFeature',
          },
          {
            geometry: {
              bbox: undefined,
              coordinates: [
                {
                  m: undefined,
                  x: 0,
                  y: 0.8865611987228108,
                  z: 2,
                },
              ],
              type: 'MultiPoint',
              is3D: true,
              vecBBox: [0, 0.4432805993614054, 0, 0.4432805993614054, 2, 2],
            },
            id: undefined,
            metadata: undefined,
            properties: {
              d: 4,
            },
            type: 'VectorFeature',
          },
        ],
        name: 'default',
      },
    },
    transformed: true,
  } as unknown as Tile);
});

// test('TileStore - lines', () => {
//   const featureCollection: FeatureCollection = {
//     type: 'FeatureCollection',
//     features: [
//       {
//         type: 'Feature',
//         properties: { a: 1 },
//         geometry: {
//           type: 'Point',
//           coordinates: [0, 0],
//         },
//       },
//       {
//         type: 'Feature',
//         properties: { b: 2 },
//         geometry: {
//           type: 'Point3D',
//           coordinates: [45, 45, 1],
//         },
//       },
//       {
//         type: 'Feature',
//         properties: { c: 3 },
//         geometry: {
//           type: 'MultiPoint',
//           coordinates: [
//             [-45, -45],
//             [-45, 45],
//           ],
//         },
//       },
//       {
//         type: 'Feature',
//         properties: { d: 4 },
//         geometry: {
//           type: 'MultiPoint3D',
//           coordinates: [
//             [45, -45, 1],
//             [-180, 20, 2],
//           ],
//         },
//       },
//     ],
//   };

//   const store = new TileStore(featureCollection, { projection: 'WM' });

//   const faceID = fromFace('WM', 0);
//   const faceTile = store.getTile(faceID);

//   expect(faceTile).toEqual({} as unknown as Tile);
// });
