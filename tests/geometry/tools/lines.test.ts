import {
  alongLine,
  cleanLineString,
  cleanMultiLineString,
  intersectionOfSegments,
  intersectionOfSegmentsRobust,
  lineArea,
  lineLength,
  pointOnLine,
  pointToLineDistance,
  toLines,
} from '../../../src';
import { describe, expect, it, test } from 'bun:test';

import type {
  VectorLineStringGeometry,
  VectorMultiLineStringGeometry,
  VectorPoint,
} from '../../../src';

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
    expect(result).toEqual({
      point: {
        x: 1,
        y: 1,
      },
      t: 0.5,
      u: 0.5,
    });
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
    expect(intersectionOfSegments(a, b)).toEqual({
      point: { x: 1, y: 1 },
      t: -0,
      u: 1,
    });
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
    expect(intersectionOfSegments(a, b)).toEqual({
      point: { x: 2, y: 0 },
      t: 0.5,
      u: 0.5,
    });
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
    // expect(intersectionOfSegmentsRobust(a, b)).toEqual({ x: 1, y: 1, t: 0.5 });
    expect(intersectionOfSegmentsRobust(a, b)).toEqual({
      point: { x: 1, y: 1 },
      t: 0.5,
      u: 0.5,
    });
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
    expect(intersectionOfSegmentsRobust(a, b, 1, 2)).toEqual({
      point: { x: 1, y: 1 },
      t: 0,
      u: 1,
    });
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
    // expect(intersectionOfSegmentsRobust(a, b)).toEqual({ x: 2, y: 0, t: 0.5 });
    expect(intersectionOfSegmentsRobust(a, b)).toEqual({
      point: { x: 2, y: 0 },
      t: 0.5,
      u: 0.5,
    });
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

describe('lineLength', () => {
  it('simple line', () => {
    const length = lineLength({
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
    });

    expect(length).toEqual(1.4142135623730951);
  });

  it('simple line - haversine', () => {
    const length = lineLength(
      {
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
      },
      true,
    );

    expect(length).toEqual(0.024682056391766436);
  });

  it('real line', () => {
    const length = lineLength({
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'LineString',
        coordinates: [
          { x: -75.343, y: 39.984 },
          { x: -75.534, y: 39.123 },
        ],
        is3D: true,
      },
    });

    expect(length).toEqual(0.8819308362904702);
  });

  it('real line - haversine', () => {
    const length = lineLength(
      {
        type: 'VectorFeature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [
            { x: -75.343, y: 39.984 },
            { x: -75.534, y: 39.123 },
          ],
          is3D: true,
        },
      },
      true,
    );

    expect(length).toEqual(0.015245501024842196);
  });
});

describe('alongLine', () => {
  it('alongLine - with radius', () => {
    const point = alongLine(
      {
        type: 'VectorFeature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [
            { x: 0, y: 0 },
            { x: 1, y: 1 },
          ],
          is3D: false,
        },
      },
      0.5,
      1,
    );
    expect(point).toEqual({ x: -0.5391218665305646, y: 59.547812487066544 });
  });

  it('alongLine - no radius', () => {
    const point = alongLine(
      {
        type: 'VectorFeature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [
            { x: 0, y: 0 },
            { x: 1, y: 1 },
          ],
          is3D: false,
        },
      },
      0.5,
    );
    expect(point).toEqual({ x: 0.9999935413484524, y: 1.0004046818854357 });
  });
});

describe('pointOnLine', () => {
  test('returns true for point exactly on the segment', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 5 };
    expect(pointOnLine(line, point)).toBeTrue();
  });

  test('returns true for point exactly on a line point of the line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 10, y: 10 };
    expect(pointOnLine(line, point)).toBeTrue();
  });

  test('returns false for point not on the line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 6 };
    expect(pointOnLine(line, point)).toBeFalse();
  });

  test('returns true when within epsilon tolerance', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 5.00001 };
    expect(pointOnLine(line, point, 0.001)).toBeTrue();
  });

  test('returns false when outside bounding box even if collinear', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 15, y: 15 };
    expect(pointOnLine(line, point)).toBeFalse();
  });

  test('handles degenerate line (single coordinate)', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [{ x: 1, y: 1 }],
      is3D: false,
    };
    const point: VectorPoint = { x: 1, y: 1 };
    expect(pointOnLine(line, point)).toBeFalse(); // No segment exists
  });
});

describe('pointToLineDistance', () => {
  test('returns 0 when point exactly on line vertex', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 10 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 0, y: 0 };
    expect(pointToLineDistance(line, point)).toBe(0);
  });

  test('returns -1 for empty line', () => {
    const line: VectorLineStringGeometry = { type: 'LineString', coordinates: [], is3D: false };
    const point: VectorPoint = { x: 5, y: 5 };
    expect(pointToLineDistance(line, point)).toBe(-1);
  });

  test('returns correct Euclidean distance for midpoint perpendicular', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 5 };
    // perpendicular distance = 5
    expect(pointToLineDistance(line, point, 'euclidean')).toBeCloseTo(5, 6);
  });

  test('returns correct Haversine distance for midpoint perpendicular', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 5 };
    // perpendicular distance = 5
    expect(pointToLineDistance(line, point, 'haversine')).toBeCloseTo(5, 6);
  });

  test('handles degenerate line (one vertex only)', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [{ x: 2, y: 3 }],
      is3D: false,
    };
    const point: VectorPoint = { x: 5, y: 3 };
    expect(pointToLineDistance(line, point, 'euclidean')).toBeCloseTo(3, 6);
  });

  test('handles line with three vertices, point closest to middle segment', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 20, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 9, y: 3 };
    const dist = pointToLineDistance(line, point, 'euclidean');
    expect(dist).toBeCloseTo(3, 6);
  });

  test('returns 0 when point lies exactly on segment', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 3, y: 0 };
    expect(pointToLineDistance(line, point, 'euclidean')).toBe(0);
  });

  test('uses haversine method internally and calls llGetDistance', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 20, y: 20 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 10, y: 10 };
    const dist = pointToLineDistance(line, point, 'haversine');
    expect(dist).toBeCloseTo(0, 6);
  });

  test('handles closest vertex at start of line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 20, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: -5, y: 0 };
    const dist = pointToLineDistance(line, point, 'euclidean');
    expect(dist).toBeCloseTo(5, 6);
  });

  test('handles closest vertex at end of line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
      ],
      is3D: false,
    };
    const point: VectorPoint = { x: 15, y: 0 };
    const dist = pointToLineDistance(line, point, 'euclidean');
    expect(dist).toBeCloseTo(5, 6);
  });
});

describe('cleanLineString', () => {
  test('removes collinear points along a straight line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
        { x: 2, y: 2 }, // superfluous
        { x: 3, y: 3 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result).toEqual([
      { x: 0, y: 0 },
      { x: 3, y: 3 },
    ]);
  });

  test('removes collinear points and duplicates along a straight line', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
        { x: 1, y: 1 },
        { x: 1, y: 1 },
        { x: 1, y: 1 },
        { x: 1, y: 1 },
        { x: 1, y: 1 },
        { x: 2, y: 2 }, // superfluous
        { x: 3, y: 3 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result).toEqual([
      { x: 0, y: 0 },
      { x: 3, y: 3 },
    ]);
  });

  test('removes collinear points and duplicates along a straight line 2', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
        { x: 2, y: 2 }, // superfluous
        { x: 3, y: 3 },
        { x: 3, y: 3 },
        { x: 3, y: 3 },
        { x: 3, y: 3 },
        { x: 3, y: 3 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result).toEqual([
      { x: 0, y: 0 },
      { x: 3, y: 3 },
    ]);
  });

  test('removes collinear points and duplicates along a straight line 3', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 1, y: 2 },
        { x: 2, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
        { x: 0, y: 0 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result).toEqual([
      { x: 0, y: 0 },
      { x: 2, y: 0 },
      { x: 0, y: 2 },
      { x: 2, y: 2 },
      { x: 0, y: 0 },
    ]);
  });

  test('retains non-collinear points', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
        { x: 2, y: 0 }, // not collinear
        { x: 3, y: 1 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result.length).toBe(4);
  });

  test('returns original when too few points', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 2, y: 2 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line);
    expect(result).toEqual(line.coordinates);
  });

  test('returns original when too few points (poly)', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
      is3D: false,
    };
    const result = cleanLineString(line, true);
    expect(result).toEqual(line.coordinates);
  });

  test('respects tolerance (eps) for nearly collinear points', () => {
    const line: VectorLineStringGeometry = {
      type: 'LineString',
      coordinates: [
        { x: 0, y: 0 },
        { x: 1, y: 1.000000000001 }, // tiny deviation
        { x: 2, y: 2 },
      ],
      is3D: false,
    };
    const resultStrict = cleanLineString(line, false, 1e-15);
    const resultLoose = cleanLineString(line, false, 1e-3);
    expect(resultStrict.length).toBe(3); // strict keeps small deviations
    expect(resultLoose.length).toBe(2); // loose removes them
  });
});

describe('cleanLineStrings', () => {
  test('removes collinear points along a straight line', () => {
    const lines: VectorMultiLineStringGeometry = {
      type: 'MultiLineString',
      coordinates: [
        [
          { x: 0, y: 0 },
          { x: 1, y: 1 },
          { x: 2, y: 2 }, // superfluous
          { x: 3, y: 3 },
        ],
      ],
      is3D: false,
    };
    const result = cleanMultiLineString(lines);
    expect(result).toEqual([
      [
        { x: 0, y: 0 },
        { x: 3, y: 3 },
      ],
    ]);
  });
});
