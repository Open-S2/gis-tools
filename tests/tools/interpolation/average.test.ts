import {
  averageInterpolation,
  getInterpolation,
  getRGBAInterpolation,
  rgbaAverageInterpolation,
} from '../../../src';
import { expect, test } from 'bun:test';

import type { Properties, RGBA, VectorPoint } from '../../../src';

/** Temp data */
interface Temperature extends Properties {
  temp: number;
}

test('average interpolation', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = getInterpolation<Temperature>('average')(
    point,
    data,
    (p) => p.m?.temp ?? 0,
  );
  expect(interpolatedValue).toEqual(2.5);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = averageInterpolation<Temperature>(point2, data, (p) => p.m?.temp ?? 0);
  expect(interp2).toEqual(2.5);
});

test('average interpolation RGBA', () => {
  const data: VectorPoint<RGBA>[] = [
    { x: 0, y: 0, m: { r: 20, g: 20, b: 60, a: 255 } },
    { x: 1, y: 0, m: { r: 30, g: 100, b: 60, a: 255 } },
    { x: 0, y: 1, m: { r: 127, g: 127, b: 60, a: 255 } },
    { x: 1, y: 1, m: { r: 255, g: 255, b: 60, a: 255 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = getRGBAInterpolation('average')(point, data);
  expect(interpolatedValue).toEqual({
    r: 143.57402271998927,
    g: 151.28945766311676,
    b: 60,
    a: 255,
  });

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = rgbaAverageInterpolation(point2, data);
  expect(interp2).toEqual({
    r: 143.57402271998927,
    g: 151.28945766311676,
    b: 60,
    a: 255,
  });
});
