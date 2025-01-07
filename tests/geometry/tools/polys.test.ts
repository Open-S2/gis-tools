import { expect, test } from 'bun:test';
import {
  pointInPolygon,
  pointInPolygons,
  polygonArea,
  polygonsArea,
} from '../../../src/geometry/tools/polys';

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
