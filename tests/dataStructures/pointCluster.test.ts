import { PointCluster } from '../../src/dataStructures/pointCluster';
import { expect, test } from 'bun:test';

// helper functions
import { fromFace } from '../../src/geometry';

import type { Tile } from '../../src/dataStructures/tile';

test('PointCluster', async () => {
  const cluster = new PointCluster(
    {
      type: 'Feature',
      properties: { a: 22 },
      geometry: { type: 'Point', coordinates: [0, 0] },
    },
    {
      maxzoom: 4,
    },
  );

  cluster.insertLonLat(2, -2, { a: 0 });
  cluster.insertLonLat(1, -1, { a: 1 });
  cluster.insertLonLat(-160, 60, { a: 2 });
  cluster.insertFaceST(0, 0.25, 0.25, { a: 3 });

  await cluster.buildClusters();

  const tileFace = await cluster.getTile(fromFace(0));
  expect(tileFace).toEqual({
    id: 1152921504606846976n,
    layers: {
      default: {
        features: [
          {
            face: 0,
            geometry: {
              coordinates: { m: { sum: 1 }, x: 0.25, y: 0.25 },
              is3D: false,
              type: 'Point',
            },
            properties: { a: 3 },
            type: 'VectorFeature',
          },
          {
            face: 0,
            geometry: {
              coordinates: { m: { sum: 3 }, x: 0.5129213455463228, y: 0.48707282292783416 },
              is3D: false,
              type: 'Point',
            },
            properties: { a: 22 },
            type: 'VectorFeature',
          },
        ],
        name: 'default',
      },
    },
    transformed: true,
  } as unknown as Tile);
});
