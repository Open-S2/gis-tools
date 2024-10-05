import { Delaunator } from '../../src';
import { beforeAll, expect, test } from 'bun:test';

import { Point } from '../../src/geometry';

let points: Point[];

beforeAll(async () => {
  points = await Bun.file(`${__dirname}/fixtures/ukraine.json`).json();
});

test('triangulates typed array', () => {
  // @ts-expect-error - testing purposes only
  const d = new Delaunator(Float64Array.from([].concat(...points)));
  expect(d.triangles).toEqual(Delaunator.fromPoints(points).triangles);
});

test('produces correct triangulation', () => {
  validate(points);
});

test('produces correct triangulation after modifying coords in place', () => {
  const d = Delaunator.fromPoints(points);

  validate(points, d);
  expect(d.trianglesLen).toEqual(5133);

  const p: Point = [80, 220];
  d.coords[0] = p[0];
  d.coords[1] = p[1];
  const newPoints: Point[] = [p].concat(points.slice(1));

  d.update();
  validate(newPoints, d);
  expect(d.trianglesLen).toEqual(5139);
});

test('issue #11', () => {
  validate([
    [516, 661],
    [369, 793],
    [426, 539],
    [273, 525],
    [204, 694],
    [747, 750],
    [454, 390],
  ]);
});

test('issue #13', async () => {
  const issue13 = await Bun.file(`${__dirname}/fixtures/issue13.json`).json();
  validate(issue13);
});

test('issue #24', () => {
  validate([
    [382, 302],
    [382, 328],
    [382, 205],
    [623, 175],
    [382, 188],
    [382, 284],
    [623, 87],
    [623, 341],
    [141, 227],
  ]);
});

test('issue #43', async () => {
  const issue43 = await Bun.file(`${__dirname}/fixtures/issue43.json`).json();
  validate(issue43);
});

test('issue #44', async () => {
  const issue44 = await Bun.file(`${__dirname}/fixtures/issue44.json`).json();
  validate(issue44);
});

test('robustness', async () => {
  const robustness1 = await Bun.file(`${__dirname}/fixtures/robustness1.json`).json();
  const robustness2 = await Bun.file(`${__dirname}/fixtures/robustness2.json`).json();
  const robustness3 = await Bun.file(`${__dirname}/fixtures/robustness3.json`).json();
  const robustness4 = await Bun.file(`${__dirname}/fixtures/robustness4.json`).json();
  validate(robustness1);
  validate(robustness1.map((p) => [p[0] / 1e9, p[1] / 1e9]));
  validate(robustness1.map((p) => [p[0] / 100, p[1] / 100]));
  validate(robustness1.map((p) => [p[0] * 100, p[1] * 100]));
  validate(robustness1.map((p) => [p[0] * 1e9, p[1] * 1e9]));
  validate(robustness2.slice(0, 100));
  validate(robustness2);
  validate(robustness3);
  validate(robustness4);
});

test('returns empty triangulation for small number of points', () => {
  let d = Delaunator.fromPoints([]);
  expect(d.triangles).toEqual([]);
  expect(d.hull).toEqual([]);
  d = Delaunator.fromPoints(points.slice(0, 1));
  expect(d.triangles).toEqual([]);
  expect(d.hull).toEqual([0]);
  d = Delaunator.fromPoints(points.slice(0, 2));
  expect(d.triangles).toEqual([]);
  expect(d.hull).toEqual([1, 0]); // [0, 1] is also correct
});

test('returns empty triangulation for all-collinear input', () => {
  const d = Delaunator.fromPoints([
    [0, 0],
    [1, 0],
    [3, 0],
    [2, 0],
  ]);
  expect(d.triangles).toEqual([]);
  expect(d.hull).toEqual([0, 1, 3, 2]); // [2, 3, 0, 1] is also correct
});

test('supports custom point format', () => {
  const d = Delaunator.fromVectorPoints([
    { x: 5, y: 5 },
    { x: 7, y: 5 },
    { x: 7, y: 6 },
  ]);
  expect(d.triangles).toEqual([0, 2, 1]);
});

/**
 * @param root0 - first point
 * @param root0."0" - x coordinate
 * @param root0."1" - y coordinate
 * @param root1 - second point
 * @param root1."0" - x coordinate
 * @param root1."1" - y coordinate
 * @param root2 - third point
 * @param root2."0" - x coordinate
 * @param root2."1" - y coordinate
 * @returns - orientation
 */
function orient([px, py]: Point, [rx, ry]: Point, [qx, qy]: Point): number {
  const l = (ry - py) * (qx - px);
  const r = (rx - px) * (qy - py);
  return Math.abs(l - r) >= 3.3306690738754716e-16 * Math.abs(l + r) ? l - r : 0;
}
/**
 * @param r - first point
 * @param q - second point
 * @param p - third point
 * @returns - orientation
 */
function convex(r: Point, q: Point, p: Point): boolean {
  return orient(p, r, q) >= 0 || orient(r, q, p) >= 0 || orient(q, p, r) >= 0;
}

/**
 * @param points - array of points
 * @param d - optional Delaunator instance
 */
function validate(points: Point[], d = Delaunator.fromPoints(points)) {
  // validate halfedges
  for (let i = 0; i < d.halfedges.length; i++) {
    expect(d.halfedges[i] === -1 || d.halfedges[d.halfedges[i]] === i, 'valid halfedge connection');
  }

  // validate triangulation
  const hullAreas: number[] = [];
  for (let i = 0, len = d.hull.length, j = len - 1; i < len; j = i++) {
    const [x0, y0] = points[d.hull[j]];
    const [x, y] = points[d.hull[i]];
    hullAreas.push((x - x0) * (y + y0));
    expect(
      convex(
        points[d.hull[j]],
        points[d.hull[(j + 1) % d.hull.length]],
        points[d.hull[(j + 3) % d.hull.length]],
      ),
      `hull should be convex at ${j}`,
    );
  }
  const hullArea = sum(hullAreas);

  const triangleAreas: number[] = [];
  for (let i = 0; i < d.triangles.length; i += 3) {
    const [ax, ay] = points[d.triangles[i]];
    const [bx, by] = points[d.triangles[i + 1]];
    const [cx, cy] = points[d.triangles[i + 2]];
    triangleAreas.push(Math.abs((by - ay) * (cx - bx) - (bx - ax) * (cy - by)));
  }
  const trianglesArea = sum(triangleAreas);

  const err = Math.abs((hullArea - trianglesArea) / hullArea);
  expect(err <= Math.pow(2, -51), `triangulation should be valid; ${err} error`);
}

/**
 * Kahan and Babuska summation, Neumaier variant; accumulates less FP error
 * @param x - array of numbers
 * @returns - sum
 */
function sum(x: number[]): number {
  let sum = x[0];
  let err = 0;
  for (let i = 1; i < x.length; i++) {
    const k = x[i];
    const m = sum + k;
    err += Math.abs(sum) >= Math.abs(k) ? sum - m + k : k - m + sum;
    sum = m;
  }
  return sum + err;
}
