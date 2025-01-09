import { bilinearInterpolation, getBilinearPoints } from '../../../src';
import { expect, test } from 'bun:test';

import type { Properties, VectorPoint } from '../../../src';

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
    corners,
    (p) => p.m?.temp ?? 0,
  );
  expect(interpolatedValue).toBe(2.5);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = bilinearInterpolation<Temperature>(point2, corners, (p) => p.m?.temp ?? 0);
  expect(interp2).toBe(1.95);
});
