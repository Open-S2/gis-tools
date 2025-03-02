import { bilinearInterpolation, getBilinearPoints, rgbaBilinearInterpolation } from '../../../src';
import { expect, test } from 'bun:test';

import type { Properties, RGBA, VectorPoint } from '../../../src';

/** Temp data */
interface Temperature extends Properties {
  temp: number;
}

test('bilinear interpolation', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const corners = getBilinearPoints(point, data);
  const interpolatedValue = bilinearInterpolation<Temperature>(
    { x: 0.5, y: 0.5 },
    data,
    (p) => p.m?.temp ?? 0,
    corners,
  );
  expect(interpolatedValue).toBe(2.5);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = bilinearInterpolation<Temperature>(point2, corners, (p) => p.m?.temp ?? 0);
  expect(interp2).toBe(1.95);
});

test('bilinear interpolation RGBA', () => {
  const data: VectorPoint<RGBA>[] = [
    { x: 0, y: 0, m: { r: 20, g: 20, b: 60, a: 255 } },
    { x: 1, y: 0, m: { r: 30, g: 100, b: 60, a: 255 } },
    { x: 0, y: 1, m: { r: 127, g: 127, b: 60, a: 255 } },
    { x: 1, y: 1, m: { r: 255, g: 255, b: 60, a: 255 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = rgbaBilinearInterpolation(point, data);
  expect(interpolatedValue).toEqual({
    g: 107.07682192685724,
    r: 84.47960893525715,
    b: 60,
    a: 255,
  });

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = rgbaBilinearInterpolation(point2, data);
  expect(interp2).toEqual({
    a: 255,
    b: 60,
    g: 79.9674775634677,
    r: 41.50520518622988,
  });
});
