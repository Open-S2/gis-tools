import { averageOfPoints, centerOfPoints, equalPoints, toPoints } from '../../../src';
import { expect, test } from 'bun:test';

test('equalPoints', () => {
  expect(equalPoints({ x: 0, y: 0 }, { x: 0, y: 0 })).toEqual(true);
  expect(equalPoints({ x: 0, y: 0 }, { x: 0, y: 1 })).toEqual(false);
  expect(equalPoints({ x: 1, y: 2, z: 3 }, { x: 1, y: 2, z: 3 })).toEqual(true);
  expect(equalPoints({ x: 1, y: 2, z: 3 }, { x: 1, y: 2, z: 4 })).toEqual(false);
});

test('averageOfPoints', () => {
  expect(
    averageOfPoints([
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5 });
  expect(
    averageOfPoints([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 0.5 });
  expect(averageOfPoints([])).toEqual({ x: 0, y: 0 });
  expect(
    averageOfPoints([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 0 });
  expect(
    averageOfPoints([
      { x: 0, y: 0 },
      { x: 1, y: 1, z: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 0.5 });
});

test('centerOfPoints', () => {
  expect(
    centerOfPoints([
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5 });
  expect(centerOfPoints([{ x: 0, y: 0, z: 0 }])).toEqual({ x: 0, y: 0, z: 0 });
  expect(
    centerOfPoints([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 0.5 });
  expect(
    centerOfPoints([
      { x: 0, y: 0 },
      { x: 1, y: 1, z: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 1 });
  expect(
    centerOfPoints([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1 },
    ]),
  ).toEqual({ x: 0.5, y: 0.5, z: 0 });
  expect(centerOfPoints([{ x: 0, y: 0 }])).toEqual({ x: 0, y: 0 });
});

test('toPoints', () => {
  expect(
    toPoints({
      type: 'VectorFeature',
      properties: {},
      geometry: { type: 'Point', coordinates: { x: 0, y: 0, z: 0 }, is3D: true },
    }),
  ).toEqual({
    type: 'MultiPoint',
    is3D: true,
    coordinates: [{ x: 0, y: 0, z: 0 }],
  });

  expect(
    toPoints({
      type: 'VectorFeature',
      properties: {},
      geometry: { type: 'MultiPoint', coordinates: [{ x: 0, y: 0, z: 0 }], is3D: true },
    }),
  ).toEqual({
    type: 'MultiPoint',
    is3D: true,
    coordinates: [{ x: 0, y: 0, z: 0 }],
  });

  expect(
    toPoints({
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
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
    ],
  });

  expect(
    toPoints({
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
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
      { x: 1, y: 1, z: 1 },
      { x: 2, y: 2, z: 2 },
    ],
  });

  expect(
    toPoints({
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
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
      { x: 1, y: 1, z: 1 },
      { x: 2, y: 2, z: 2 },
    ],
  });

  expect(
    toPoints({
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
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
      { x: 1, y: 1, z: 1 },
      { x: 2, y: 2, z: 2 },
      { x: 3, y: 3, z: 3 },
      { x: 4, y: 4, z: 4 },
    ],
  });
});
