import { expect, test } from 'bun:test';
import { fromS2Point, getDistance, normalize, toAngles, toS2Point } from '../../../src/geometry/ll';

test('fromS2Point', () => {
  expect(fromS2Point([0, 0, 0])).toEqual([0, 0]);
  expect(fromS2Point([1, 0, 0])).toEqual([0, 0]);
  expect(fromS2Point([0, 1, 0])).toEqual([90, 0]);
  expect(fromS2Point([0, 0, 1])).toEqual([0, 90]);
});

test('getDistance', () => {
  expect(getDistance([0, 0], [0, 0])).toBe(0);
  expect(getDistance([0.017453292519943295, 0], [0, 0])).toBe(0.0003046174197867086);
});

test('normalize', () => {
  expect(normalize([0, 0])).toEqual([0, 0]);
  expect(normalize([0.01745329251994, 0.111111])).toEqual([0.01745329251991734, 0.111111]);
  expect(normalize([640, 100])).toEqual([-80, 90]);
  expect(normalize([-640, -100])).toEqual([80, -90]);
});

test('toAngles', () => {
  expect(toAngles([0, 0])).toEqual([0, 0]);
  expect(toAngles([0.017453292519943295, 0.111111])).toEqual([
    0.0003046174197867086, 0.0019392527851834194,
  ]);
  expect(toAngles([90, 180])).toEqual([1.5707963267948966, 3.141592653589793]);
});

test('toS2Point', () => {
  expect(toS2Point([0, 0])).toEqual([1, 0, 0]);
  expect(toS2Point([90, 0])).toEqual([6.123233995736766e-17, 1, 0]);
  expect(toS2Point([0, 90])).toEqual([6.123233995736766e-17, 0, 1]);
  expect(toS2Point([0, 180])).toEqual([-1, -0, 1.2246467991473532e-16]);
});
