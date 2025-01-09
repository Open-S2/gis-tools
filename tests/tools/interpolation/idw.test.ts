import { idwInterpolation } from '../../../src';
import { expect, test } from 'bun:test';

import type { Properties, VectorPoint } from '../../../src';

/** Temp data */
interface Temperature extends Properties {
  temp: number;
}

test('idw interpolation', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = idwInterpolation<Temperature>(point, data, (p) => p.m?.temp ?? 0);
  expect(interpolatedValue).toBe(2.4999999999999996);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = idwInterpolation<Temperature>(point2, data, (p) => p.m?.temp ?? 0);
  expect(interp2).toBe(2.088659617630171);
});
