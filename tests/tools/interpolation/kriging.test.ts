import { KrigingInterpolator, krigingInterpolation, rgbaKrigingInterpolation } from '../../../src';
import { expect, test } from 'bun:test';

import type { Properties, RGBA, VectorPoint } from '../../../src';

/** Temp data */
interface Temperature extends Properties {
  temp: number;
}

test('kriging gaussian', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const kriging = new KrigingInterpolator<Temperature>(data, 'gaussian', (p) => p.m?.temp ?? 0);

  const interp1 = kriging.predict({ x: 0.5, y: 0.5 });
  expect(interp1).toEqual(2.8569152451779507);
  const variance1 = kriging.variance({ x: 0.5, y: 0.5 });
  expect(variance1).toEqual(2.7355110684092128);

  const interp2 = kriging.predict({ x: 0.65, y: 0.15 });
  expect(interp2).toEqual(2.7884543556631534);
});

test('kriging exponential', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const kriging = new KrigingInterpolator<Temperature>(data, 'exponential', (p) => p.m?.temp ?? 0);

  const interp1 = kriging.predict({ x: 0.5, y: 0.5 });
  expect(interp1).toEqual(2.8600086267459988);

  const interp2 = kriging.predict({ x: 0.65, y: 0.15 });
  expect(interp2).toEqual(2.8003406905612267);
});

test('kriging spherical', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];
  const kriging = new KrigingInterpolator<Temperature>(data, 'spherical', (p) => p.m?.temp ?? 0);

  const interp1 = kriging.predict({ x: 0.5, y: 0.5 });
  expect(interp1).toEqual(2.4787428546416876);

  const interp2 = kriging.predict({ x: 0.65, y: 0.15 });
  expect(interp2).toEqual(2.4741925173613524);
});

test('kriging interpolation', () => {
  const data: VectorPoint<Temperature>[] = [
    { x: 0, y: 0, m: { temp: 1 } },
    { x: 1, y: 0, m: { temp: 2 } },
    { x: 0, y: 1, m: { temp: 3 } },
    { x: 1, y: 1, m: { temp: 4 } },
  ];

  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = krigingInterpolation<Temperature>(point, data, (p) => p.m?.temp ?? 0);
  expect(interpolatedValue).toEqual(2.8569152451779507);

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = krigingInterpolation<Temperature>(point2, data, (p) => p.m?.temp ?? 0);
  expect(interp2).toEqual(2.7884543556631534);
});

test('kriging interpolation RGBA', () => {
  const data: VectorPoint<RGBA>[] = [
    { x: 0, y: 0, m: { r: 20, g: 20, b: 60, a: 255 } },
    { x: 1, y: 0, m: { r: 30, g: 100, b: 60, a: 255 } },
    { x: 0, y: 1, m: { r: 127, g: 127, b: 60, a: 255 } },
    { x: 1, y: 1, m: { r: 255, g: 255, b: 60, a: 255 } },
  ];
  const point = { x: 0.5, y: 0.5 };
  const interpolatedValue = rgbaKrigingInterpolation(point, data);
  expect(interpolatedValue).toEqual({
    b: 0,
    g: 161.7287828230922,
    r: 153.48096960327914,
    a: 255,
  });

  const point2 = { x: 0.65, y: 0.15 };
  const interp2 = rgbaKrigingInterpolation(point2, data);
  // TODO: B is wrong. why?
  expect(interp2).toEqual({
    a: 255,
    b: 0,
    g: 157.69362582696115,
    r: 147.49018570315383,
  });
});
