import {
  complement,
  containsS2Cell,
  containsS2Point,
  emptyCap,
  fromS1Angle,
  fromS1ChordAngle,
  fromS2Point,
  fullCap,
  getArea,
  getIntersectingCells,
  height,
  intersectsS2CellFast,
  isEmpty,
  isFull,
  radius,
} from '../../../src/geometry/s2/cap';
import { expect, test } from 'bun:test';

import { fromFace, fromIJ, toS2Point } from '../../../src/geometry/id';

import type { S2Cap } from '../../../src/geometry/s2/cap';

test('complement', () => {
  const cap: S2Cap<{ a: number }> = { center: [1, 0, 0], radius: 1, data: { a: 1 } };
  expect(complement(cap)).toEqual({ center: [-1, -0, -0], radius: 3, data: { a: 1 } });
});

test('containsS2Cell', () => {
  const face = fromFace(0);
  const subPoint = fromIJ(0, 10, 10, 5);
  const subPoint2 = fromIJ(3, 10, 10, 6);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(containsS2Cell(cap, subPoint)).toEqual(true);
  expect(containsS2Cell(cap, subPoint2)).toEqual(false);

  const empty = emptyCap({ a: 1 });
  expect(containsS2Cell(empty, subPoint)).toEqual(false);

  const full = fullCap({ a: 1 });
  expect(containsS2Cell(full, subPoint)).toEqual(true);
});

test('containsS2Point', () => {
  const face = fromFace(0);
  const subPoint = fromIJ(0, 10, 10, 5);
  const subPoint2 = fromIJ(3, 10, 10, 6);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(containsS2Point(cap, toS2Point(subPoint))).toEqual(true);
  expect(containsS2Point(cap, toS2Point(subPoint2))).toEqual(false);
});

test('emptyCap', () => {
  const cap = emptyCap({ a: 1 });
  expect(cap.center).toEqual([1, 0, 0]);
  expect(cap.radius).toEqual(-1);
  expect(cap.data).toEqual({ a: 1 });
});

test('fromS1Angle', () => {
  const face = fromFace(0);
  const cap = fromS1Angle(toS2Point(face), 1, { a: 1 });
  expect(cap.center).toEqual([1, 0, 0]);
  expect(cap.radius).toEqual(0.9193953882637206);
  expect(cap.data).toEqual({ a: 1 });
});

test('fromS1ChordAngle', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(cap.center).toEqual([1, 0, 0]);
  expect(cap.radius).toEqual(1);
  expect(cap.data).toEqual({ a: 1 });
});

test('fromS2Point', () => {
  const cap = fromS2Point([1, 0, 0], { a: 1 });
  expect(cap.center).toEqual([1, 0, 0]);
  expect(cap.radius).toEqual(0);
  expect(cap.data).toEqual({ a: 1 });
});

test('fullCap', () => {
  const cap = fullCap({ a: 1 });
  expect(cap.center).toEqual([1, 0, 0]);
  expect(cap.radius).toEqual(4);
  expect(cap.data).toEqual({ a: 1 });
});

test('getArea', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(getArea(cap)).toEqual(3.141592653589793);
});

test('height', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(height(cap)).toEqual(0.5);
});

test('isEmpty', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(isEmpty(cap)).toEqual(false);

  const empty = emptyCap({ a: 1 });
  expect(isEmpty(empty)).toEqual(true);
});

test('isFull', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(isFull(cap)).toEqual(false);

  const full = fullCap({ a: 1 });
  expect(isFull(full)).toEqual(true);
});

test('radius', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  expect(radius(cap)).toEqual(1.0471975511965976);
});

test('getIntersectingCells', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 1, { a: 1 });
  const cells = getIntersectingCells(cap);
  expect(cells).toEqual([
    13546827679130451968n,
    12970366926827028480n,
    10664523917613334528n,
    10088063165309911040n,
    5476377146882523136n,
    4899916394579099648n,
    4323455642275676160n,
    2594073385365405696n,
    1152921504606846976n,
  ]);
});

test('intersectsS2CellFast', () => {
  const face = fromFace(0);
  const cap = fromS1ChordAngle(toS2Point(face), 0.95, { a: 1 });
  expect(intersectsS2CellFast(cap, 13546827679130451968n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 12970366926827028480n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 10664523917613334528n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 10088063165309911040n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 5476377146882523136n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 4899916394579099648n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 4323455642275676160n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 2594073385365405696n)).toEqual(true);
  expect(intersectsS2CellFast(cap, 3746994889972252672n)).toEqual(false);
});
