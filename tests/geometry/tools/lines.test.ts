import {
  alongLine,
  cleanLineString,
  cleanMultiLineString,
  equalLines,
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
  VectorLineString,
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
  it('returns intersection for crossing segments robust', () => {
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
      tAngle: -0.7853981633974483,
      tVec: { x: 1, y: -1 },
      u: 0.5,
      uAngle: 0.7853981633974483,
      uVec: { x: 1, y: 1 },
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
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: { x: 1, y: 1 },
      t: 0,
      tAngle: -0.7853981633974483,
      tVec: { x: 0, y: 0 },
      u: 1,
      uAngle: 0.7853981633974483,
      uVec: { x: 1, y: 1 },
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
    expect(intersectionOfSegmentsRobust(a, b, true)).toBeUndefined();
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
    expect(intersectionOfSegmentsRobust(a, b)).toEqual({
      point: { x: 2, y: 0 },
      t: 0.5,
      tAngle: 1.5707963267948966,
      tVec: { x: 0, y: 1 },
      u: 0.5,
      uAngle: 0,
      uVec: { x: 2, y: 0 },
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

  it('edges are touching', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: -104.0624, y: 75.4279145091691 },
      { x: -104.0625, y: 75.44 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: -104.0529352, y: 75.4261125 },
      { x: -104.0625, y: 75.44 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: { x: -104.0625, y: 75.44 },
      t: 1,
      tAngle: 2.173921453053248,
      tVec: { x: -0.009564800000006812, y: 0.013887499999995612 },
      u: 1,
      uAngle: 1.5790705226314463,
      uVec: { x: -0.00010000000000331966, y: 0.012085490830898493 },
    });
  });

  it('such a small u value the points are equal', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 54.569778932416476, y: 24.441366817541834 },
      { x: 54.56977894449294, y: 24.441074136738756 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 54.57000000000001, y: 24.441250624330703 },
      { x: 54.56795534005685, y: 24.442325298422087 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: { x: 54.56977893241648, y: 24.44136681754183 },
      t: 0.10811948670049906,
      tAngle: 2.6576750942587646,
      tVec: {
        x: -0.00022106758353146463,
        y: 0.0001161932111308183,
      },
      u: 0.00000000000020134769880365415,
      uAngle: -1.5707550652486564,
      uVec: {
        x: 0.0000000000000000000024315679333950687,
        y: -0.0000000000000000589306061837349,
      },
    });
  });

  it('such a small u value the points are equal reversed', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 54.56977894449294, y: 24.441074136738756 },
      { x: 54.569778932416476, y: 24.441366817541834 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 54.57000000000001, y: 24.441250624330703 },
      { x: 54.56795534005685, y: 24.442325298422087 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: {
        x: 54.56977893241648,
        y: 24.44136681754183,
      },
      t: 0.10811948670049906,
      tAngle: 2.6576750942587646,
      tVec: {
        x: -0.00022106758353146463,
        y: 0.0001161932111308183,
      },
      u: 0.9999999999997986,
      uAngle: 1.570837588341137,
      uVec: {
        x: -0.000000012076462496677164,
        y: 0.00029268080307781266,
      },
    });
  });

  it('such a small u value the points are equal 2', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 54.57000000000001, y: 24.441250624330703 },
      { x: 54.57204465994316, y: 24.442325298422087 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 54.57114771720956, y: 24.441853864959285 },
      { x: 54.57080496898934, y: 24.4416737163564 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: { x: 54.57107012236083, y: 24.441813081075473 },
      t: 0.22638921520691385,
      tAngle: -2.6576750942666547,
      tVec: {
        x: -0.00007759450058913746,
        y: -0.00004078370082734485,
      },
      u: 0.5233742483220198,
      uAngle: 0.48391755933245995,
      uVec: {
        x: 0.0010701223608214699,
        y: 0.0005624567447695169,
      },
    });
  });

  it('such a small u value the points are equal 3', () => {
    const a: [VectorPoint, VectorPoint] = [
      { x: 54.57074509421786, y: 24.44164224615234 },
      { x: 54.57204465994316, y: 24.442325298422087 },
    ];
    const b: [VectorPoint, VectorPoint] = [
      { x: 54.57114771720956, y: 24.441853864959285 },
      { x: 54.57080496898934, y: 24.4416737163564 },
    ];
    expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({
      point: { x: 54.571107779107194, y: 24.44183287347656 },
      t: 0.1165233176297705,
      tAngle: -2.6576750942666547,
      tVec: {
        x: -0.000039938159731715664,
        y: -0.000020991512874316045,
      },
      u: 0.27908160570130575,
      uAngle: 0.48391755933283453,
      uVec: {
        x: 0.0003626848893303284,
        y: 0.00019062732421871832,
      },
    });
  });

  // it('such a small u value the points are equal 4', () => {
  //   const a: [VectorPoint, VectorPoint] = [
  //     { x: 54.57080496898934, y: 24.441673716356398 },
  //     { x: 54.57204465994316, y: 24.442325298422087 },
  //   ];
  //   const b: [VectorPoint, VectorPoint] = [
  //     { x: 54.57114771720956, y: 24.441853864959285 },
  //     { x: 54.57080496898934, y: 24.4416737163564 },
  //   ];
  //   expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({});
  // });

  // it('should intersect', () => {
  //   const a: [VectorPoint, VectorPoint] = [
  //     { x: 19.9284281536379, y: 50.05417333008677 },
  //     { x: 19.928265336875185, y: 50.05438343158938 },
  //   ];
  //   const b: [VectorPoint, VectorPoint] = [
  //     { x: 19.928383178550334, y: 50.05423136669995 },
  //     { x: 19.928383178550334, y: 50.05417333008677 },
  //   ];
  //   // 2.23007100305349 2.2300710030445616
  //   /**
  //    * @param a
  //    * @param b
  //    */
  //   function angle(a: VectorPoint, b: VectorPoint): number {
  //     return Math.atan2(a.y - b.y, a.x - b.x);
  //   }
  //   console.log(angle(b[0], a[0]), angle(a[1], a[0]));
  //   expect(intersectionOfSegmentsRobust(a, b, false)).toEqual({});
  // });
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
    const result = cleanLineString(line)!;
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

  test('0 area poly returns empty', () => {
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
    expect(result).toBeUndefined();
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
    const resultStrict = cleanLineString(line, false, 1e-15)!;
    const resultLoose = cleanLineString(line, false, 1e-3)!;
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

describe('equalLines', () => {
  test('returns true for equal lines', () => {
    const a: VectorLineString = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
      { x: 0, y: 2 },
      { x: 2, y: 2 },
      { x: 0, y: 0 },
    ];
    const b: VectorLineString = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
      { x: 0, y: 2 },
      { x: 2, y: 2 },
      { x: 0, y: 0 },
    ];
    const c: VectorLineString = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
      { x: 0, y: 2 },
      { x: 2, y: 2 },
    ];
    const d: VectorLineString = [
      { x: 0, y: 0 },
      { x: 2, y: 0 },
      { x: 0, y: 2 },
      { x: 2, y: 3 },
      { x: 0, y: 0 },
    ];
    expect(equalLines(a, a)).toBeTrue();
    expect(equalLines(a, b)).toBeTrue();
    expect(equalLines(a, c)).toBeFalse();
    expect(equalLines(a, d)).toBeFalse();
  });
});
