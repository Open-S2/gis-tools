import { expect, test } from 'bun:test';
import { insphere, inspherefast } from '../../../src';

test('insphere', async () => {
  expect(insphere(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0) < 0, 'inside');

  expect(insphere(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 2) > 0, 'outside');

  expect(insphere(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, -1) === 0, 'cospherical');

  const a = -0.9999999999999999;
  const b = -1.0000000000000002;

  expect(insphere(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, a) < 0, 'near inside');

  expect(insphere(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, b) > 0, 'near outside');

  const lines = (await Bun.file(`${__dirname}/fixtures/insphere.txt`).text()).trim().split(/\r?\n/);
  for (const line of lines) {
    const [, ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ex, ey, ez, sign] = line
      .split(' ')
      .map(Number);
    const result = insphere(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, ex, ey, ez);
    expect(Math.sign(result) === -sign, `${line}: ${result} vs ${-sign}`);
  }
  // 1000 hard fixtures
});

test('inspherefast', () => {
  expect(inspherefast(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0) < 0, 'inside');
  expect(inspherefast(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 2) > 0, 'outside');
  expect(inspherefast(1, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 1, 0, 0, -1) === 0, 'cospherical');
});
