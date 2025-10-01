import { describe, expect, it, test } from 'bun:test';
import {
  intersectionOfSegments,
  intersectionOfSegmentsRobust,
  lineArea,
  toLines,
} from '../../../src';

import type { VectorPoint } from '../../../src';

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

describe('intersectionOfSegments', () => {
  it('returns intersection for crossing segments', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 2 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 0, y: 2 },
      { x: 2, y: 0 },
    ];
    const result = intersectionOfSegments(a, b);
    expect(result).toEqual({ x: 1, y: 1 });
  });

  it('returns undefined when segments are parallel', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 0, y: 1 },
      { x: 2, y: 1 },
    ];
    expect(intersectionOfSegments(a, b)).toBeUndefined();
  });

  it('returns undefined when intersection lies outside the segment bounds', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 2, y: 2 },
      { x: 3, y: 3 },
    ];
    expect(intersectionOfSegments(a, b)).toBeUndefined();
  });

  it('returns endpoint when only endpoints touch', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 1 },
      { x: 2, y: 0 },
    ];
    expect(intersectionOfSegments(a, b)).toEqual({ x: 1, y: 1 });
  });

  it('returns nothing when parallel overlap', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 0 },
      { x: 3, y: 0 },
    ];
    expect(intersectionOfSegments(a, b)).toBeUndefined();
  });

  it('returns correct intersection inside segment ranges', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 4, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 2, y: -1 },
      { x: 2, y: 1 },
    ];
    expect(intersectionOfSegments(a, b)).toEqual({ x: 2, y: 0 });
  });
});

describe('intersectionOfSegmentsRobust', () => {
  it('returns intersection for crossing segments', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 2 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 0, y: 2 },
      { x: 2, y: 0 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toEqual({ x: 1, y: 1 });
  });

  it('returns undefined for parallel non-intersecting segments', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 0, y: 1 },
      { x: 2, y: 1 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toBeUndefined();
  });

  it('returns undefined for collinear overlapping segments', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 0 },
      { x: 3, y: 0 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toBeUndefined();
  });

  it('returns endpoint intersection if segments touch and ringIDs differ', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 1 },
      { x: 2, y: 0 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, 1, 2)).toEqual({ x: 1, y: 1 });
  });

  it('returns undefined if segments touch at endpoints and ringIDs are the same', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 1 },
      { x: 2, y: 0 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, 1, 1)).toBeUndefined();
  });

  it('returns intersection inside segment ranges', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 4, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 2, y: -1 },
      { x: 2, y: 1 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toEqual({ x: 2, y: 0 });
  });

  it('returns undefined when intersection point is outside of segment ranges', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 2, y: -1 },
      { x: 2, y: 1 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toBeUndefined();
  });

  it('returns nothing when parallel overlap', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 1, y: 0 },
      { x: 3, y: 0 },
    ];
    expect(intersectionOfSegmentsRobust(a, b)).toBeUndefined();
  });
});
