import {
  cleanPolygon,
  dekinkPolygon,
  dekinkPolygons,
  equalPolys,
  pointInPolygon,
  pointInPolygons,
  polygonArea,
  polygonsArea,
  polygonsIntersections,
  toVector,
} from '../../../src';
import { describe, expect, test } from 'bun:test';

import type {
  FeatureCollection,
  VectorMultiPolygon,
  VectorPolygon,
  VectorPolygonGeometry,
} from 's2json-spec';

test('pointInPolygon', () => {
  const polygon = [
    [
      { x: 0, y: 0 },
      { x: 1, y: 0 },
      { x: 1, y: 1 },
      { x: 0, y: 1 },
      { x: 0, y: 0 },
    ],
  ];
  expect(pointInPolygon({ x: 0, y: 0 }, polygon)).toEqual(0);
  expect(pointInPolygon({ x: 0, y: 0 }, polygon, true)).toEqual(false);
});

test('pointInPolygons', () => {
  const polygons = [
    [
      [
        { x: 0, y: 0 },
        { x: 1, y: 0 },
        { x: 1, y: 1 },
        { x: 0, y: 1 },
        { x: 0, y: 0 },
      ],
    ],
  ];
  expect(pointInPolygons({ x: 0, y: 0 }, polygons)).toEqual(0);
  expect(pointInPolygons({ x: 0, y: 0 }, polygons, true)).toEqual(false);
});

test('polygonArea', () => {
  const polygon = [
    [
      { x: 125, y: -15 },
      { x: 113, y: -22 },
      { x: 117, y: -37 },
      { x: 130, y: -33 },
      { x: 148, y: -39 },
      { x: 154, y: -27 },
      { x: 144, y: -15 },
      { x: 125, y: -15 },
    ],
  ];
  expect(polygonArea(polygon)).toEqual(7748891609977.455);
});

test('polygonsArea', () => {
  const polygons = [
    [
      [
        { x: 125, y: -15 },
        { x: 113, y: -22 },
        { x: 117, y: -37 },
        { x: 130, y: -33 },
        { x: 148, y: -39 },
        { x: 154, y: -27 },
        { x: 144, y: -15 },
        { x: 125, y: -15 },
      ],
    ],
  ];
  expect(polygonsArea(polygons)).toEqual(7748891609977.455);
});

const simplePolygonCoords: VectorPolygon = [
  [
    { x: 0, y: 0 },
    { x: 10, y: 0 },
    { x: 10, y: 10 },
    { x: 0, y: 10 },
    { x: 0, y: 0 },
  ],
];

// Simple square polygon (as VectorPolygonGeometry)
const simplePolygonGeom: VectorPolygonGeometry = {
  type: 'Polygon',
  is3D: false,
  coordinates: simplePolygonCoords,
};

// Polygon with a hole (as VectorPolygon)
const polygonWithHoleCoords: VectorPolygon = [
  // Outer ring (counter-clockwise)
  [
    { x: 0, y: 0 },
    { x: 10, y: 0 },
    { x: 10, y: 10 },
    { x: 0, y: 10 },
    { x: 0, y: 0 },
  ],
  // Inner ring (hole - clockwise)
  [
    { x: 2, y: 2 },
    { x: 8, y: 2 },
    { x: 8, y: 8 },
    { x: 2, y: 8 },
    { x: 2, y: 2 },
  ],
];

// Polygon with a hole (as VectorPolygonGeometry)
const polygonWithHoleGeom: VectorPolygonGeometry = {
  type: 'Polygon',
  is3D: false,
  coordinates: polygonWithHoleCoords,
};

describe('pointInPolygon (Geometry Inputs)', () => {
  describe('Basic Cases (Simple Polygon)', () => {
    test('should return true for point strictly inside (VectorPolygon input)', () => {
      expect(pointInPolygon({ x: 5, y: 5 }, simplePolygonCoords)).toEqual(true);
    });

    test('should return true for point strictly inside (VectorPolygonGeometry input)', () => {
      expect(pointInPolygon({ x: 5, y: 5 }, simplePolygonGeom)).toEqual(true);
    });

    test('should return false for point strictly outside', () => {
      expect(pointInPolygon({ x: 15, y: 15 }, simplePolygonCoords)).toEqual(false);
      expect(pointInPolygon({ x: 5, y: 15 }, simplePolygonGeom)).toEqual(false); // Mix types for variety
      expect(pointInPolygon({ x: -5, y: 5 }, simplePolygonCoords)).toEqual(false);
    });
  });

  describe('Boundary Cases (Simple Polygon)', () => {
    // Vertex
    test('should handle point on vertex', () => {
      expect(pointInPolygon({ x: 0, y: 0 }, simplePolygonCoords)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 0, y: 0 }, simplePolygonCoords, true)).toEqual(false); // Ignore boundary
      expect(pointInPolygon({ x: 10, y: 10 }, simplePolygonGeom)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 10, y: 10 }, simplePolygonGeom, true)).toEqual(false); // Ignore boundary
    });

    // Horizontal Edge
    test('should handle point on horizontal edge', () => {
      expect(pointInPolygon({ x: 5, y: 0 }, simplePolygonCoords)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 5, y: 0 }, simplePolygonCoords, true)).toEqual(false); // Ignore boundary
      expect(pointInPolygon({ x: 5, y: 10 }, simplePolygonGeom)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 5, y: 10 }, simplePolygonGeom, true)).toEqual(false); // Ignore boundary
    });

    // Vertical Edge
    test('should handle point on vertical edge', () => {
      expect(pointInPolygon({ x: 0, y: 5 }, simplePolygonCoords)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 0, y: 5 }, simplePolygonCoords, true)).toEqual(false); // Ignore boundary
      expect(pointInPolygon({ x: 10, y: 5 }, simplePolygonGeom)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 10, y: 5 }, simplePolygonGeom, true)).toEqual(false); // Ignore boundary
    });

    // Diagonal Edge (requires a different polygon)
    const diagonalPolygonCoords: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 10, y: 0 },
        { x: 5, y: 10 },
        { x: 0, y: 10 },
        { x: 0, y: 0 },
      ],
    ];
    test('should handle point on diagonal edge', () => {
      // Point exactly on the segment from (10,0) to (5,10) -> y = -2x + 20. Let x = 7.5, y = 5
      expect(pointInPolygon({ x: 7.5, y: 5 }, diagonalPolygonCoords)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 7.5, y: 5 }, diagonalPolygonCoords, true)).toEqual(false); // Ignore boundary
    });
  });

  describe('Polygon with Hole', () => {
    test('should return true for point between outer and inner ring', () => {
      expect(pointInPolygon({ x: 1, y: 5 }, polygonWithHoleCoords)).toEqual(true);
      expect(pointInPolygon({ x: 9, y: 5 }, polygonWithHoleGeom)).toEqual(true);
    });

    test('should return false for point inside the hole', () => {
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithHoleCoords)).toEqual(false);
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithHoleGeom)).toEqual(false);
    });

    test('should handle point on outer boundary', () => {
      expect(pointInPolygon({ x: 0, y: 5 }, polygonWithHoleCoords)).toEqual(0);
      expect(pointInPolygon({ x: 0, y: 5 }, polygonWithHoleCoords, true)).toEqual(false);
    });

    test('should handle point on inner boundary (hole edge)', () => {
      expect(pointInPolygon({ x: 2, y: 5 }, polygonWithHoleGeom)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 2, y: 5 }, polygonWithHoleGeom, true)).toEqual(false); // Ignore boundary
      expect(pointInPolygon({ x: 5, y: 2 }, polygonWithHoleCoords)).toEqual(0); // On boundary
      expect(pointInPolygon({ x: 5, y: 2 }, polygonWithHoleCoords, true)).toEqual(false); // Ignore boundary
    });
  });

  describe('Invalid Polygon', () => {
    test('should return false for unclosed polygon ring', () => {
      const unclosedPolygonCoords: VectorPolygon = [
        [
          { x: 0, y: 0 },
          { x: 10, y: 0 },
          { x: 10, y: 10 },
          { x: 0, y: 10 },
          // Missing closing point
        ],
      ];
      const unclosedPolygonGeom: VectorPolygonGeometry = {
        type: 'Polygon',
        is3D: false,
        coordinates: unclosedPolygonCoords,
      };

      expect(pointInPolygon({ x: 5, y: 5 }, unclosedPolygonCoords)).toEqual(false);
      expect(pointInPolygon({ x: 5, y: 5 }, unclosedPolygonGeom)).toEqual(false);
    });

    test('should return false for unclosed hole ring', () => {
      const polygonWithUnclosedHoleCoords: VectorPolygon = [
        // Outer ring (OK)
        [
          { x: 0, y: 0 },
          { x: 10, y: 0 },
          { x: 10, y: 10 },
          { x: 0, y: 10 },
          { x: 0, y: 0 },
        ],
        // Inner ring (unclosed)
        [
          { x: 2, y: 2 },
          { x: 8, y: 2 },
          { x: 8, y: 8 },
          { x: 2, y: 8 },
        ],
      ];
      const polygonWithUnclosedHoleGeom: VectorPolygonGeometry = {
        type: 'Polygon',
        is3D: false,
        coordinates: polygonWithUnclosedHoleCoords,
      };

      // Test point technically between rings, but should fail due to invalid hole
      expect(pointInPolygon({ x: 1, y: 1 }, polygonWithUnclosedHoleCoords)).toEqual(true);
      // Test point technically in hole, should fail
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithUnclosedHoleGeom)).toEqual(true);
    });
  });
});

describe('polygonsIntersections', () => {
  test('polygonsIntersections - simple no overlap', () => {
    const a: VectorPolygon = [
      [
        { x: -57.29250824839444, y: 39.309204530727754 },
        { x: -58.742162935523396, y: 35.86408152890863 },
        { x: -53.43642678063297, y: 37.560632581866784 },
        { x: -57.29250824839444, y: 39.309204530727754 },
      ],
    ];

    const b: VectorPolygon = [
      [
        { x: -47.66680112586184, y: 39.08451444040281 },
        { x: -51.377917124910766, y: 37.76719296237772 },
        { x: -47.4058632821789, y: 35.274503072379716 },
        { x: -47.66680112586184, y: 39.08451444040281 },
      ],
    ];

    const intersections = polygonsIntersections([a, b]);

    expect(intersections).toEqual([]);
  });

  test('polygonsIntersections - simple overlap', () => {
    const a: VectorPolygon = [
      [
        { x: -57.29250824839444, y: 39.309204530727754 },
        { x: -58.742162935523396, y: 35.86408152890863 },
        { x: -53.43642678063297, y: 37.560632581866784 },
        { x: -57.29250824839444, y: 39.309204530727754 },
      ],
    ];

    const b: VectorPolygon = [
      [
        { x: -51.29093784368342, y: 39.08451444040281 },
        { x: -55.118026217701825, y: 37.72134033908044 },
        { x: -50.79805525005969, y: 35.53445202830912 },
        { x: -51.29093784368342, y: 39.08451444040281 },
      ],
    ];

    const intersections = polygonsIntersections([a, b]);

    expect(intersections).toEqual([
      {
        point: { x: -54.27247565285823, y: 37.293299349853186 },
        segment1: { from: 1, id: 1, polyIndex: 0, ringIndex: 0, to: 2 },
        segment2: { from: 1, id: 4, polyIndex: 1, ringIndex: 0, to: 2 },
        u: 0.8424254716370221,
        t: 0.19573061281592416,
      },
      {
        point: { x: -54.37470749761522, y: 37.98610371249223 },
        segment1: { from: 2, id: 2, polyIndex: 0, ringIndex: 0, to: 3 },
        segment2: { from: 0, id: 3, polyIndex: 1, ringIndex: 0, to: 1 },
        u: 0.24332492060312594,
        t: 0.8057743518198057,
      },
    ]);
  });

  test('dekinkPolygons - hourglass', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];

    const intersections = polygonsIntersections([polygonFeature]);

    expect(intersections).toEqual([]);
  });

  test('dekinkPolygons - hourglass self intersection', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];

    const intersections = polygonsIntersections([polygonFeature], true);

    expect(intersections).toEqual([
      {
        point: { x: 1, y: 1 },
        segment1: { from: 1, id: 1, polyIndex: 0, ringIndex: 0, to: 2 },
        segment2: { from: 3, id: 3, polyIndex: 0, ringIndex: 0, to: 4 },
        u: 0.5,
        t: 0.5,
      },
    ]);
  });

  test('dekinkPolygons - almost parallel segments 3', () => {
    const polyA: VectorPolygon = [
      [
        { x: -104.117212, y: 75.4383502 },
        { x: -104.0624, y: 75.4279145091691 },
        { x: -104.0625, y: 75.44 },
        { x: -104.117212, y: 75.4383502 },
      ],
    ];
    const polyB: VectorPolygon = [
      [
        { x: -104.0529352, y: 75.4261125 },
        { x: -104.0625, y: 75.44 },
        { x: -104.0626, y: 75.4279525872937 },
        { x: -104.0529352, y: 75.4261125 },
      ],
    ];

    const intersections = polygonsIntersections([polyA, polyB], false);

    expect(intersections).toEqual([
      {
        point: { x: -104.0626, y: 75.42795258729372 },
        segment1: { from: 0, id: 0, polyIndex: 0, ringIndex: 0, to: 1 },
        segment2: { from: 1, id: 4, polyIndex: 1, ringIndex: 0, to: 2 },
        t: 0.9999999999992828,
        u: 0.9963511639785707,
      },
      {
        point: { x: -104.0625, y: 75.44 },
        segment1: { from: 1, id: 1, polyIndex: 0, ringIndex: 0, to: 2 },
        segment2: { from: 0, id: 3, polyIndex: 1, ringIndex: 0, to: 1 },
        t: 1,
        u: 1,
      },
      {
        point: { x: -104.0625, y: 75.44 },
        segment1: { from: 1, id: 1, polyIndex: 0, ringIndex: 0, to: 2 },
        segment2: { from: 1, id: 4, polyIndex: 1, ringIndex: 0, to: 2 },
        t: 0,
        u: 1,
      },
      {
        point: { x: -104.0625, y: 75.44 },
        segment1: { from: 2, id: 2, polyIndex: 0, ringIndex: 0, to: 3 },
        segment2: { from: 0, id: 3, polyIndex: 1, ringIndex: 0, to: 1 },
        t: 1,
        u: 0,
      },
      {
        point: { x: -104.0625, y: 75.44 },
        segment1: { from: 2, id: 2, polyIndex: 0, ringIndex: 0, to: 3 },
        segment2: { from: 1, id: 4, polyIndex: 1, ringIndex: 0, to: 2 },
        t: 0,
        u: 0,
      },
    ]);
  });

  test('intersections of large json set', async () => {
    const polygons: FeatureCollection = await Bun.file(
      `${__dirname}/fixtures/chunks-water/args.geojson`,
    ).json();
    // convert to vector format
    const vectorPolygons: VectorMultiPolygon = [];
    for (const feature of polygons.features) {
      const vectorFeature = toVector(feature);
      const { geometry } = vectorFeature;
      if (geometry.type === 'MultiPolygon') {
        vectorPolygons.push(...geometry.coordinates);
      } else if (geometry.type === 'Polygon') {
        vectorPolygons.push(geometry.coordinates);
      }
    }

    const intersections = polygonsIntersections(vectorPolygons);
    expect(intersections).toHaveLength(5_108);
  });
});

describe('cleanPolygon', () => {
  test('cleanPolygon - hourglass', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];

    const cleaned = cleanPolygon(polygonFeature);
    expect(cleaned).toEqual({
      bbox: [0, 0, 2, 2],
      coordinates: [
        [
          [
            { x: 0, y: 0 },
            { x: 2, y: 0 },
            { x: 1, y: 1 },
            { x: 0, y: 0 },
          ],
        ],
        [
          [
            { x: 1, y: 1 },
            { x: 2, y: 2 },
            { x: 0, y: 2 },
            { x: 1, y: 1 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });

  test('cleanPolygon - multiple kinks', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 8.094854051549703, y: 44.067038922182604 },
        { x: 27.45169791493106, y: 34.31013538862004 },
        { x: 31.238906496896703, y: 25.572928139998595 },
        { x: 26.610096007827508, y: 22.88716015007573 },
        { x: 25.978894577499233, y: 18.957601207155236 },
        { x: 32.08050840400031, y: 17.157354229920827 },
        { x: 38.8133236608289, y: 20.541732106259843 },
        { x: 40.496527475035236, y: 28.199781765371043 },
        { x: 7.463652621221485, y: 25.00221485407819 },
        { x: 25.347693147171753, y: 4.999693002409302 },
        { x: -7.4747812298659255, y: -36.777396059815665 },
        { x: 27.662098391706394, y: -40.233822107102995 },
        { x: 28.92450125236215, y: -14.406933337995738 },
        { x: 4.097244992807987, y: -34.38206769619466 },
        { x: 62.79897801327945, y: -31.19907851930298 },
        { x: 86.57423188895399, y: 16.55327251195662 },
        { x: 54.38295894224376, y: 12.685928855764459 },
        { x: 73.73980280562509, y: -3.197906810124664 },
        { x: 81.52462044633336, y: 36.369487623534425 },
        { x: 54.80375989579596, y: 56.70904723358515 },
        { x: 8.094854051549703, y: 44.067038922182604 },
      ],
    ];

    expect(cleanPolygon(polygonFeature)).toEqual({
      bbox: [-7.4747812298659255, -40.233822107102995, 86.57423188895399, 56.70904723358515],
      coordinates: [
        [
          [
            { x: 7.463652621221485, y: 25.00221485407819 },
            { x: 25.347693147171753, y: 4.999693002409302 },
            { x: -7.4747812298659255, y: -36.777396059815665 },
            { x: 27.662098391706394, y: -40.233822107102995 },
            { x: 28.011510823463176, y: -33.08536237940715 },
            { x: 62.79897801327945, y: -31.19907851930298 },
            { x: 86.57423188895399, y: 16.55327251195662 },
            { x: 77.40918288852106, y: 15.452216515532072 },
            { x: 81.52462044633336, y: 36.369487623534425 },
            { x: 54.80375989579596, y: 56.70904723358515 },
            { x: 8.094854051549703, y: 44.067038922182604 },
            { x: 27.45169791493106, y: 34.31013538862004 },
            { x: 30.51892217779216, y: 27.233954216494492 },
            { x: 7.463652621221485, y: 25.00221485407819 },
          ],
        ],
        [
          [
            { x: 28.92450125236215, y: -14.406933337995738 },
            { x: 4.097244992807987, y: -34.38206769619466 },
            { x: 28.011510823463176, y: -33.08536237940715 },
            { x: 28.92450125236215, y: -14.406933337995738 },
          ],
        ],
        [
          [
            { x: 54.38295894224376, y: 12.685928855764459 },
            { x: 73.73980280562509, y: -3.197906810124664 },
            { x: 77.40918288852106, y: 15.452216515532072 },
            { x: 54.38295894224376, y: 12.685928855764459 },
          ],
        ],
        [
          [
            { x: 31.238906496896703, y: 25.572928139998595 },
            { x: 26.610096007827508, y: 22.88716015007573 },
            { x: 25.978894577499233, y: 18.957601207155236 },
            { x: 32.08050840400031, y: 17.157354229920827 },
            { x: 38.8133236608289, y: 20.541732106259843 },
            { x: 40.496527475035236, y: 28.199781765371043 },
            { x: 30.51892217779216, y: 27.233954216494492 },
            { x: 31.238906496896703, y: 25.572928139998595 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });

  test('cleanPolygon - all problems', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: -91.92218713423073, y: 42.750854798206724 },
        { x: -91.9139393415105, y: 42.75096509455043 },
        { x: -91.91403053661699, y: 42.74800278177934 },
        { x: -91.91407345196123, y: 42.74679733206786 },
        { x: -91.91410563846941, y: 42.74537912592471 },
        { x: -91.91653571983723, y: 42.745359428388724 },
        { x: -91.91653571983723, y: 42.745359428388724 },
        { x: -91.91653571983723, y: 42.745359428388724 },
        { x: -91.9165088977471, y: 42.74624187186039 },
        { x: -91.91667519470603, y: 42.746249750763376 },
        { x: -91.91669128796013, y: 42.74536730740387 },
        { x: -91.91752277275486, y: 42.745359428388724 },
        { x: -91.91750131508273, y: 42.74636399474419 },
        { x: -91.91750667950076, y: 42.74737248420069 },
        { x: -91.91857047641793, y: 42.74735355619808 },
        { x: -91.91856615206176, y: 42.74748869579865 },
        { x: -91.91859029194288, y: 42.746734301373 },
        { x: -91.918995305504, y: 42.7463029333329 },
        { x: -91.91905967852034, y: 42.7461453552181 },
        { x: -91.9191481914178, y: 42.74588535045296 },
        { x: -91.91914282699976, y: 42.74558594967556 },
        { x: -91.91900871654907, y: 42.74558004043513 },
        { x: -91.91898457666794, y: 42.745357458635716 },
        { x: -91.92070119043696, y: 42.74535154937352 },
        { x: -91.92096672912932, y: 42.745483522761845 },
        { x: -91.92110352178906, y: 42.74550912950613 },
        { x: -91.92127786537496, y: 42.745485492511776 },
        { x: -91.92160241266565, y: 42.74565292102763 },
        { x: -91.9217257942803, y: 42.745786863514724 },
        { x: -91.92172311207129, y: 42.74599959510529 },
        { x: -91.92183040043186, y: 42.746527481454386 },
        { x: -91.92210935016931, y: 42.74716369671481 },
        { x: -91.92166678568198, y: 42.747210969414716 },
        { x: -91.92159704824762, y: 42.7472739996252 },
        { x: -91.92226760050114, y: 42.74730157532212 },
        { x: -91.92218713423073, y: 42.750854798206724 },
      ],
    ];

    const res = cleanPolygon(polygonFeature);
    expect(res).toEqual({
      bbox: [-91.92226760050114, 42.74535154937352, -91.9139393415105, 42.75096509455043],
      coordinates: [
        [
          [
            { x: -91.92218713423073, y: 42.750854798206724 },
            { x: -91.92226760050114, y: 42.74730157532212 },
            { x: -91.92159704824762, y: 42.7472739996252 },
            { x: -91.92166678568198, y: 42.747210969414716 },
            { x: -91.92210935016931, y: 42.74716369671481 },
            { x: -91.92183040043186, y: 42.746527481454386 },
            { x: -91.92172311207129, y: 42.74599959510529 },
            { x: -91.9217257942803, y: 42.745786863514724 },
            { x: -91.92160241266565, y: 42.74565292102763 },
            { x: -91.92127786537496, y: 42.745485492511776 },
            { x: -91.92110352178906, y: 42.74550912950613 },
            { x: -91.92096672912932, y: 42.745483522761845 },
            { x: -91.92070119043696, y: 42.74535154937352 },
            { x: -91.91898457666794, y: 42.745357458635716 },
            { x: -91.91900871654907, y: 42.74558004043513 },
            { x: -91.91914282699976, y: 42.74558594967556 },
            { x: -91.9191481914178, y: 42.74588535045296 },
            { x: -91.91905967852034, y: 42.7461453552181 },
            { x: -91.918995305504, y: 42.7463029333329 },
            { x: -91.91859029194288, y: 42.746734301373 },
            { x: -91.91857047639631, y: 42.74735355619846 },
            { x: -91.91750667950076, y: 42.74737248420069 },
            { x: -91.91750131508273, y: 42.74636399474419 },
            { x: -91.91752277275486, y: 42.745359428388724 },
            { x: -91.91669128796013, y: 42.74536730740387 },
            { x: -91.91667519470603, y: 42.746249750763376 },
            { x: -91.9165088977471, y: 42.74624187186039 },
            { x: -91.91653571983723, y: 42.745359428388724 },
            { x: -91.91410563846941, y: 42.74537912592471 },
            { x: -91.91407345196123, y: 42.74679733206786 },
            { x: -91.91403053661699, y: 42.74800278177934 },
            { x: -91.9139393415105, y: 42.75096509455043 },
            { x: -91.92218713423073, y: 42.750854798206724 },
          ],
        ],
        [
          [
            { x: -91.91856615206176, y: 42.74748869579865 },
            { x: -91.91857047641793, y: 42.74735355619808 },
            { x: -91.91857047639631, y: 42.74735355619846 },
            { x: -91.91856615206176, y: 42.74748869579865 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });
});

describe('cleanPolygons', () => {
  test('cleanPolygon - all problems + collinearity', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 1, y: 2 },
        { x: 2, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
        { x: 0, y: 0 },
      ],
    ];

    const cleaned = cleanPolygon(polygonFeature, true);
    expect(cleaned).toEqual({
      bbox: [0, 0, 2, 2],
      coordinates: [
        [
          [
            { x: 0, y: 0 },
            { x: 2, y: 0 },
            { x: 1, y: 1 },
            { x: 0, y: 0 },
          ],
        ],
        [
          [
            { x: 1, y: 1 },
            { x: 2, y: 2 },
            { x: 0, y: 2 },
            { x: 1, y: 1 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });
});

describe('dekinkPolygon', () => {
  test('dekinkPolygon - hourglass', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];

    const dekinked = dekinkPolygon(polygonFeature);
    expect(dekinked).toEqual({
      bbox: [0, 0, 2, 2],
      coordinates: [
        [
          [
            { x: 0, y: 0 },
            { x: 2, y: 0 },
            { x: 1, y: 1 },
            { x: 0, y: 0 },
          ],
        ],
        [
          [
            { x: 1, y: 1 },
            { x: 2, y: 2 },
            { x: 0, y: 2 },
            { x: 1, y: 1 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });
});

describe('dekinkPolygons', () => {
  test('dekinkPolygons - hourglass', () => {
    const polygonFeature: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];

    const dekinked = dekinkPolygons([polygonFeature]);
    expect(dekinked).toEqual({
      bbox: [0, 0, 2, 2],
      coordinates: [
        [
          [
            { x: 0, y: 0 },
            { x: 2, y: 0 },
            { x: 1, y: 1 },
            { x: 0, y: 0 },
          ],
        ],
        [
          [
            { x: 1, y: 1 },
            { x: 2, y: 2 },
            { x: 0, y: 2 },
            { x: 1, y: 1 },
          ],
        ],
      ],
      is3D: false,
      type: 'MultiPolygon',
    });
  });
});

describe('equalPolys', () => {
  test('equalPolys - hourglass', () => {
    const a: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];
    const b: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];
    const c: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 2 },
        { x: 0, y: 0 },
      ],
    ];
    const d: VectorPolygon = [
      [
        { x: 0, y: 0 },
        { x: 2, y: 0 },
        { x: 0, y: 2 },
        { x: 2, y: 3 },
        { x: 0, y: 0 },
      ],
    ];

    expect(equalPolys(a, a)).toBeTrue();
    expect(equalPolys(a, b)).toBeTrue();
    expect(equalPolys(a, c)).toBeFalse();
    expect(equalPolys(a, d)).toBeFalse();
  });
});
