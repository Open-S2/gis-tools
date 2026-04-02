import { expect, test } from 'bun:test';
import { orient3d, orient3dfast, orient3dfastVector } from '../../../src/index.js';

// import robustOrientation from 'robust-orientation';

test('orient3d', async () => {
  expect(orient3d(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1) > 0, 'above');
  expect(orient3d(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, -1) < 0, 'below');
  expect(orient3d(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0) === 0, 'coplanar');

  const a = -0.9999999999999999;
  const b = -1.0000000000000002;

  expect(orient3d(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, a) > 0, 'near above');
  expect(orient3d(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, b) < 0, 'near below');

  const lines = (await Bun.file(`${__dirname}/fixtures/orient3d.txt`).text()).trim().split(/\r?\n/);
  for (const line of lines) {
    const [, ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, sign] = line.split(' ').map(Number);
    const result = orient3d(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz);
    expect(Math.sign(result) === sign, `${line}: ${result} vs ${sign}`);
    expect(
      Math.sign(result) === Math.sign(orient3d(dx, dy, dz, bx, by, bz, ax, ay, az, cx, cy, cz)),
      'symmetry',
    );
  }
  // 1000 hard fixtures

  const tol = 5.0e-14;

  for (let i = 0; i < 1000; i++) {
    const ax = 0.5 + tol * Math.random();
    const ay = 0.5 + tol * Math.random();
    const az = 0.5 + tol * Math.random();
    const b = 12,
      c = 24,
      d = 48;
    expect(orient3d(b, b, b, c, c, c, d, d, d, ax, ay, az) === 0, 'degenerate');
    expect(orient3d(c, c, c, d, d, d, ax, ay, az, b, b, b) === 0, 'degenerate');
  }
  // 1000 degenerate cases
});

test('orient3dfast', () => {
  expect(orient3dfast(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1) > 0, 'above');
  expect(orient3dfast(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, -1) < 0, 'below');
  expect(orient3dfast(0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0) === 0, 'coplanar');
});

test('orient3dfastVector', () => {
  expect(
    orient3dfastVector({ x: 0, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 1, y: 0, z: 0 }) > 0,
    'above',
  );
  expect(
    orient3dfastVector({ x: 0, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 1, y: 0, z: 0 }) < 0,
    'below',
  );
  expect(
    orient3dfastVector({ x: 0, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 1, y: 0, z: 0 }) === 0,
    'coplanar',
  );
});
