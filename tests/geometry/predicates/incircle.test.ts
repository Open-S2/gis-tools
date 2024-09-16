import { expect, test } from 'bun:test';
import { incircle, incirclefast } from '../../../src/geometry/predicates';

import nextafter from 'nextafter';

test('incircle', async () => {
  expect(incircle(0, -1, 0, 1, 1, 0, -0.5, 0) < 0, 'inside');
  expect(incircle(0, -1, 1, 0, 0, 1, -1, 0) === 0, 'on circle');
  expect(incircle(0, -1, 0, 1, 1, 0, -1.5, 0) > 0, 'outside');

  const a = nextafter(-1, 0);
  const b = nextafter(-1, -2);

  expect(incircle(1, 0, -1, 0, 0, 1, 0, a) < 0, 'near inside');
  expect(incircle(1, 0, -1, 0, 0, 1, 0, b) > 0, 'near outside');

  let x = 1e-64;
  for (let i = 0; i < 128; i++) {
    expect(incircle(0, x, -x, -x, x, -x, 0, 0) > 0, `incircle test ${x}, outside`);
    expect(incircle(0, x, -x, -x, x, -x, 0, 2 * x) < 0, `incircle test ${x}, inside`);
    expect(incircle(0, x, -x, -x, x, -x, 0, x) === 0, `incircle test ${x}, cocircular`);
    x *= 10;
  }
  // 384 incircle tests

  const lines = (await Bun.file(`${__dirname}/fixtures/incircle.txt`).text()).trim().split(/\r?\n/);
  for (const line of lines) {
    const [, ax, ay, bx, by, cx, cy, dx, dy, sign] = line.split(' ').map(Number);
    const result = incircle(ax, ay, bx, by, cx, cy, dx, dy);
    expect(Math.sign(result) === sign, `${line}: ${result} vs ${sign}`);
  }
  // 1000 hard fixtures
});

test('incirclefast', () => {
  expect(incirclefast(0, -1, 0, 1, 1, 0, -0.5, 0) < 0, 'inside');
  expect(incirclefast(0, -1, 0, 1, 1, 0, -1, 0) === 0, 'on circle');
  expect(incirclefast(0, -1, 0, 1, 1, 0, -1.5, 0) > 0, 'outside');
});
