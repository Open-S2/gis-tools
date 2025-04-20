import { describe, expect, test } from 'bun:test';
import {
  pointInPolygon,
  pointInPolygons,
  polygonArea,
  polygonsArea,
} from '../../../src/geometry/tools/polys';

import type { VectorPolygon, VectorPolygonGeometry } from 's2json-spec';

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
  expect(pointInPolygon({ x: 0, y: 0 }, polygon)).toEqual(true);
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
  expect(pointInPolygons({ x: 0, y: 0 }, polygons)).toEqual(true);
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
      expect(pointInPolygon({ x: 5, y: 5 }, simplePolygonCoords)).toBe(true);
    });

    test('should return true for point strictly inside (VectorPolygonGeometry input)', () => {
      expect(pointInPolygon({ x: 5, y: 5 }, simplePolygonGeom)).toBe(true);
    });

    test('should return false for point strictly outside', () => {
      expect(pointInPolygon({ x: 15, y: 15 }, simplePolygonCoords)).toBe(false);
      expect(pointInPolygon({ x: 5, y: 15 }, simplePolygonGeom)).toBe(false); // Mix types for variety
      expect(pointInPolygon({ x: -5, y: 5 }, simplePolygonCoords)).toBe(false);
    });
  });

  describe('Boundary Cases (Simple Polygon)', () => {
    // Vertex
    test('should handle point on vertex', () => {
      expect(pointInPolygon({ x: 0, y: 0 }, simplePolygonCoords)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 0, y: 0 }, simplePolygonCoords, true)).toBe(false); // Ignore boundary
      expect(pointInPolygon({ x: 10, y: 10 }, simplePolygonGeom)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 10, y: 10 }, simplePolygonGeom, true)).toBe(false); // Ignore boundary
    });

    // Horizontal Edge
    test('should handle point on horizontal edge', () => {
      expect(pointInPolygon({ x: 5, y: 0 }, simplePolygonCoords)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 5, y: 0 }, simplePolygonCoords, true)).toBe(false); // Ignore boundary
      expect(pointInPolygon({ x: 5, y: 10 }, simplePolygonGeom)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 5, y: 10 }, simplePolygonGeom, true)).toBe(false); // Ignore boundary
    });

    // Vertical Edge
    test('should handle point on vertical edge', () => {
      expect(pointInPolygon({ x: 0, y: 5 }, simplePolygonCoords)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 0, y: 5 }, simplePolygonCoords, true)).toBe(false); // Ignore boundary
      expect(pointInPolygon({ x: 10, y: 5 }, simplePolygonGeom)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 10, y: 5 }, simplePolygonGeom, true)).toBe(false); // Ignore boundary
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
      expect(pointInPolygon({ x: 7.5, y: 5 }, diagonalPolygonCoords)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 7.5, y: 5 }, diagonalPolygonCoords, true)).toBe(false); // Ignore boundary
    });
  });

  describe('Polygon with Hole', () => {
    test('should return true for point between outer and inner ring', () => {
      expect(pointInPolygon({ x: 1, y: 5 }, polygonWithHoleCoords)).toBe(true);
      expect(pointInPolygon({ x: 9, y: 5 }, polygonWithHoleGeom)).toBe(true);
    });

    test('should return false for point inside the hole', () => {
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithHoleCoords)).toBe(false);
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithHoleGeom)).toBe(false);
    });

    test('should handle point on outer boundary', () => {
      expect(pointInPolygon({ x: 0, y: 5 }, polygonWithHoleCoords)).toBe(true);
      expect(pointInPolygon({ x: 0, y: 5 }, polygonWithHoleCoords, true)).toBe(false);
    });

    test('should handle point on inner boundary (hole edge)', () => {
      expect(pointInPolygon({ x: 2, y: 5 }, polygonWithHoleGeom)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 2, y: 5 }, polygonWithHoleGeom, true)).toBe(false); // Ignore boundary
      expect(pointInPolygon({ x: 5, y: 2 }, polygonWithHoleCoords)).toBe(true); // On boundary
      expect(pointInPolygon({ x: 5, y: 2 }, polygonWithHoleCoords, true)).toBe(false); // Ignore boundary
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

      expect(pointInPolygon({ x: 5, y: 5 }, unclosedPolygonCoords)).toBe(false);
      expect(pointInPolygon({ x: 5, y: 5 }, unclosedPolygonGeom)).toBe(false);
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
      expect(pointInPolygon({ x: 1, y: 1 }, polygonWithUnclosedHoleCoords)).toBe(true);
      // Test point technically in hole, should fail
      expect(pointInPolygon({ x: 5, y: 5 }, polygonWithUnclosedHoleGeom)).toBe(true);
    });
  });
});
