import { expect, test } from 'bun:test';
import {
  getInterpolation,
  getRGBAInterpolation,
  lanczosInterpolation,
  rgbaLanczosInterpolation,
} from '../../../src';

import type { Properties, RGBA, VectorPoint } from '../../../src';

/** Temp data */
interface Temperature extends Properties {
  temp: number;
}

test('lanczos interpolation', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = getInterpolation<Temperature>('lanczos')(
    point,
    data,
    (p) => p.m?.temp ?? 0,
  );
  expect(interpolatedValue).toBe(2.5);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = lanczosInterpolation<Temperature>(point2, data, (p) => p.m?.temp ?? 0);
  expect(interp2).toBe(1.7622380738712637);
});

test('lanczos interpolation RGBA', () => {
  const data: VectorPoint<RGBA>[] = [
    { x: 0, y: 0, m: { r: 20, g: 20, b: 60, a: 255 } },
    { x: 1, y: 0, m: { r: 30, g: 100, b: 60, a: 255 } },
    { x: 0, y: 1, m: { r: 127, g: 127, b: 60, a: 255 } },
    { x: 1, y: 1, m: { r: 255, g: 255, b: 60, a: 255 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = getRGBAInterpolation('lanczos')(point, data);
  expect(interpolatedValue).toEqual({
    r: 143.57402271998927,
    g: 151.28945766311676,
    b: 59.99999999999999,
    a: 255,
  });

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = rgbaLanczosInterpolation(point2, data);
  expect(interp2).toEqual({
    a: 255,
    b: 60.00000000000001,
    g: 98.95711759121838,
    r: 60.88101077653883,
  });
});
