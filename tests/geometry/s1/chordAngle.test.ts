import {
  chordAngleCos,
  chordAngleSin,
  chordAngleSin2,
  chordAngleTan,
  fastUpperBoundFrom,
  fromAngle,
  fromLength2,
  fromS2Points,
  isSpecial,
  negativeAngle,
  rightAngle,
  straightAngle,
  toAngle,
  toKM,
  toMeters,
} from '../../../src/geometry/s1/chordAngle';
import { expect, test } from 'bun:test';

test('chordAngleCos', () => {
  expect(chordAngleCos(0)).toEqual(1);
  expect(chordAngleCos(1)).toEqual(0.5);
  expect(chordAngleCos(2)).toEqual(0);
  expect(chordAngleCos(3)).toEqual(-0.5);
  expect(chordAngleCos(4)).toEqual(-1);
  expect(chordAngleCos(5)).toEqual(-1.5);
});

test('chordAngleSin', () => {
  expect(chordAngleSin(0)).toEqual(0);
  expect(chordAngleSin(1)).toEqual(0.8660254037844386);
  expect(chordAngleSin(2)).toEqual(1);
  expect(chordAngleSin(3)).toEqual(0.8660254037844386);
  expect(chordAngleSin(4)).toEqual(0);
  expect(chordAngleSin(5)).toEqual(NaN);
});

test('chordAngleSin2', () => {
  expect(chordAngleSin2(0)).toEqual(0);
  expect(chordAngleSin2(1)).toEqual(0.75);
  expect(chordAngleSin2(2)).toEqual(1);
  expect(chordAngleSin2(3)).toEqual(0.75);
  expect(chordAngleSin2(4)).toEqual(0);
  expect(chordAngleSin2(5)).toEqual(-1.25);
});

test('chordAngleTan', () => {
  expect(chordAngleTan(0)).toEqual(0);
  expect(chordAngleTan(1)).toEqual(1.7320508075688772);
  expect(chordAngleTan(2)).toEqual(Infinity);
  expect(chordAngleTan(3)).toEqual(-1.7320508075688772);
  expect(chordAngleTan(4)).toEqual(-0);
  expect(chordAngleTan(5)).toEqual(NaN);
});

test('fastUpperBoundFrom', () => {
  expect(fastUpperBoundFrom(0)).toEqual(0);
  expect(fastUpperBoundFrom(1)).toEqual(1);
  expect(fastUpperBoundFrom(2)).toEqual(4);
  expect(fastUpperBoundFrom(3)).toEqual(4);
});

test('fromAngle', () => {
  expect(fromAngle(0)).toEqual(0);
  expect(fromAngle(1)).toEqual(0.9193953882637206);
  expect(fromAngle(2)).toEqual(2.8322936730942847);
  expect(fromAngle(3)).toEqual(3.979984993200891);
  expect(fromAngle(Math.PI)).toEqual(4);
  expect(fromAngle(-2)).toEqual(-1);
  expect(fromAngle(Infinity)).toEqual(Infinity);
});

test('fromLength2', () => {
  expect(fromLength2(0)).toEqual(0);
  expect(fromLength2(1)).toEqual(1);
  expect(fromLength2(2)).toEqual(2);
  expect(fromLength2(3)).toEqual(3);
  expect(fromLength2(4)).toEqual(4);
  expect(fromLength2(5)).toEqual(4);
});

test('fromS2Points', () => {
  expect(fromS2Points([0, 0, 0], [0, 0, 0])).toEqual(0);
  expect(fromS2Points([0, 0, 0], [1, 0, 0])).toEqual(1);
  expect(fromS2Points([0, 0, 0], [0, 1, 0])).toEqual(1);
});

test('isSpecial', () => {
  expect(isSpecial(0)).toEqual(false);
  expect(isSpecial(-2)).toEqual(true);
  expect(isSpecial(Infinity)).toEqual(true);
});

test('negativeAngle', () => {
  expect(negativeAngle()).toEqual(-1);
});

test('rightAngle', () => {
  expect(rightAngle()).toEqual(2);
});

test('straightAngle', () => {
  expect(straightAngle()).toEqual(4);
});

test('toAngle', () => {
  expect(toAngle(0)).toEqual(0);
  expect(toAngle(0.9193953882637206)).toEqual(1);
  expect(toAngle(2.8322936730942847)).toEqual(2);
  expect(toAngle(3.979984993200891)).toEqual(3.0000000000000004);
  expect(toAngle(4)).toEqual(Math.PI);
});

test('toKM', () => {
  expect(toKM(0)).toEqual(0);
  expect(toKM(0.9193953882637206)).toEqual(6371.0088);
});

test('toMeters', () => {
  expect(toMeters(0)).toEqual(0);
  expect(toMeters(0.9193953882637206)).toEqual(6371008.8);
});
