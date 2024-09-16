import { expect, test } from 'bun:test';
import { orient2d, orient2dfast } from '../../../src/geometry/predicates';

import robustOrientation from 'robust-orientation';

test('orient2d', async () => {
  expect(orient2d(0, 0, 1, 1, 0, 1) < 0, 'clockwise');
  expect(orient2d(0, 0, 0, 1, 1, 1) > 0, 'counterclockwise');
  expect(orient2d(0, 0, 0.5, 0.5, 1, 1) === 0, 'collinear');

  const r = 0.95;
  const q = 18;
  const p = 16.8;
  const w = Math.pow(2, -43);

  for (let i = 0; i < 128; i++) {
    for (let j = 0; j < 128; j++) {
      const x = r + (w * i) / 128;
      const y = r + (w * j) / 128;

      const o = orient2d(x, y, q, q, p, p);
      const o2 = robustOrientation[3]([x, y], [q, q], [p, p]);

      expect(Math.sign(o) === Math.sign(o2), `${x},${y}, ${q},${q}, ${p},${p}: ${o} vs ${o2}`);
    }
  }
  // 512x512 near-collinear

  const lines = (await Bun.file(`${__dirname}/fixtures/orient2d.txt`).text()).trim().split(/\r?\n/);
  for (const line of lines) {
    const [, ax, ay, bx, by, cx, cy, sign] = line.split(' ').map(Number);
    const result = orient2d(ax, ay, bx, by, cx, cy);
    expect(Math.sign(result) === -sign, `${line}: ${result} vs ${-sign}`);
  }
  // 1000 hard fixtures
});

test('orient2dfast', () => {
  expect(orient2dfast(0, 0, 1, 1, 0, 1) < 0, 'counterclockwise');
  expect(orient2dfast(0, 0, 0, 1, 1, 1) > 0, 'clockwise');
  expect(orient2dfast(0, 0, 0.5, 0.5, 1, 1) === 0, 'collinear');
});
