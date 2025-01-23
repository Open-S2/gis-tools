import { FileReader } from '../../src/file';
import { JSONReader, PointCluster } from '../../src';
import { expect, test } from 'bun:test';

// helper functions
import { fromFace } from '../../src';

import type { Properties, Tile } from '../../src';

test('PointCluster', async () => {
  const cluster = new PointCluster<{ a: number }>(
    {
      type: 'Feature',
      properties: { a: 22 },
      geometry: { type: 'Point', coordinates: [0, 0] },
    },
    {
      maxzoom: 4,
    },
  );

  cluster.insertLonLat({ x: 2, y: -2, m: { a: 0 } });
  cluster.insertLonLat({ x: 1, y: -1, m: { a: 1 } });
  cluster.insertLonLat({ x: -160, y: 60, m: { a: 2 } });
  cluster.insertFaceST(0, 0.25, 0.25, { a: 3 });

  await cluster.buildClusters();

  const tileFace = await cluster.getTile(fromFace(0));
  expect(tileFace).toEqual({
    face: 0,
    zoom: 0,
    i: 0,
    j: 0,
    extent: 1,
    layers: {
      default: {
        extent: 1,
        features: [
          {
            face: 0,
            geometry: {
              coordinates: { m: { value: 1 }, x: 0.25, y: 0.25 },
              is3D: false,
              type: 'Point',
            },
            properties: { a: 3 },
            type: 'VectorFeature',
          },
          {
            face: 0,
            geometry: {
              coordinates: { m: { value: 3 }, x: 0.5129213455463228, y: 0.48707282292783416 },
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
  } as unknown as Tile<Record<string, unknown>, { value: number }, { a: number }>);
});

test('PointCluster - from reader', async () => {
  const fileReaderPoints = new FileReader(`${__dirname}/../readers/json/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReaderPoints);
  const cluster = new PointCluster();

  await cluster.insertReader(jsonReader);

  await cluster.buildClusters();

  const tileFace = await cluster.getTile(fromFace(3));
  expect(tileFace).toEqual({
    extent: 1,
    face: 3,
    i: 0,
    j: 0,
    layers: {
      default: {
        extent: 1,
        features: [
          {
            face: 3,
            geometry: {
              coordinates: {
                m: {
                  value: 3,
                },
                x: 0.9391825486078325,
                y: 0.15739240199815419,
              },
              is3D: false,
              type: 'Point',
            },
            properties: {
              name: 'Melbourne',
            },
            type: 'VectorFeature',
          },
        ],
        name: 'default',
      },
    },
    transformed: true,
    zoom: 0,
  } as unknown as Tile<Record<string, unknown>, { value: number }, Properties>);
});
