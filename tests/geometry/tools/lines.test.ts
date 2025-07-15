import { lineArea, toLines } from '../../../src';

import { expect, test } from 'bun:test';

test('toLines', () => {
  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: { type: 'Point', coordinates: { x: 0, y: 0, z: 0 }, is3D: true },
    }),
  ).toBeUndefined();

  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: { type: 'MultiPoint', coordinates: [{ x: 0, y: 0, z: 0 }], is3D: true },
    }),
  ).toBeUndefined();

  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'LineString',
        coordinates: [
          { x: 0, y: 0, z: 0 },
          { x: 1, y: 1, z: 1 },
        ],
        is3D: true,
      },
    }),
  ).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
    ],
  });

  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'MultiLineString',
        coordinates: [
          [
            { x: 0, y: 0, z: 0 },
            { x: 1, y: 1, z: 1 },
          ],
          [
            { x: 1, y: 1, z: 1 },
            { x: 2, y: 2, z: 2 },
          ],
        ],
        is3D: true,
      },
    }),
  ).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
    ],
  });

  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'Polygon',
        coordinates: [
          [
            { x: 0, y: 0, z: 0 },
            { x: 1, y: 1, z: 1 },
          ],
          [
            { x: 1, y: 1, z: 1 },
            { x: 2, y: 2, z: 2 },
          ],
        ],
        is3D: true,
      },
    }),
  ).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
    ],
  });

  expect(
    toLines({
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'MultiPolygon',
        coordinates: [
          [
            [
              { x: 0, y: 0, z: 0 },
              { x: 1, y: 1, z: 1 },
            ],
            [
              { x: 1, y: 1, z: 1 },
              { x: 2, y: 2, z: 2 },
            ],
          ],
          [
            [
              { x: 3, y: 3, z: 3 },
              { x: 4, y: 4, z: 4 },
            ],
          ],
        ],
        is3D: true,
      },
    }),
  ).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
      [
        { x: 3, y: 3, z: 3 },
        { x: 4, y: 4, z: 4 },
      ],
    ],
  });
});

test('lineArea', () => {
  const line = {
    type: 'LineString' as const,
    is3D: true,
    coordinates: [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 1, y: 1 },
      { x: 0, y: 1 },
      { x: 0, y: 0 },
    ],
  };
  const area = lineArea(line);
  expect(area).toEqual(-1);

  const line2 = {
    type: 'VectorFeature' as const,
    properties: {},
    geometry: {
      type: 'LineString' as const,
      is3D: true,
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 0 },
        { x: 1, y: 1 },
        { x: 0, y: 1 },
        { x: 0, y: 0 },
      ],
    },
  };
  const area2 = lineArea(line2);
  expect(area2).toEqual(-1);
});
