import { expect, test } from 'bun:test';
import { fromS2Point, getDistance, normalize, toAngles, toS2Point } from '../../../src/geometry/ll';

test('fromS2Point', () => {
  expect(fromS2Point({ x: 0, y: 0, z: 0 })).toEqual({ x: 0, y: 0 });
  expect(fromS2Point({ x: 1, y: 0, z: 0 })).toEqual({ x: 0, y: 0 });
  expect(fromS2Point({ x: 0, y: 1, z: 0 })).toEqual({ x: 90, y: 0 });
  expect(fromS2Point({ x: 0, y: 0, z: 1 })).toEqual({ x: 0, y: 90 });
});

test('getDistance', () => {
  expect(getDistance({ x: 0, y: 0 }, { x: 0, y: 0 })).toBe(0);
  expect(getDistance({ x: 0.017453292519943295, y: 0 }, { x: 0, y: 0 })).toBe(
    0.0003046174197867086,
  );
});

test('normalize', () => {
  expect(normalize({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
  expect(normalize({ x: 0.01745329251994, y: 0.111111 })).toEqual({
    x: 0.01745329251991734,
    y: 0.111111,
  });
  expect(normalize({ x: 640, y: 100 })).toEqual({ x: -80, y: 90 });
  expect(normalize({ x: -640, y: -100 })).toEqual({ x: 80, y: -90 });
});

test('toAngles', () => {
  expect(toAngles({ x: 0, y: 0 })).toEqual([0, 0]);
  expect(toAngles({ x: 0.017453292519943295, y: 0.111111 })).toEqual([
    0.0003046174197867086, 0.0019392527851834194,
  ]);
  expect(toAngles({ x: 90, y: 180 })).toEqual([1.5707963267948966, 3.141592653589793]);
});

test('toS2Point', () => {
  expect(toS2Point({ x: 0, y: 0 })).toEqual({ x: 1, y: 0, z: 0 });
  expect(toS2Point({ x: 90, y: 0 })).toEqual({ x: 6.123233995736766e-17, y: 1, z: 0 });
  expect(toS2Point({ x: 0, y: 90 })).toEqual({ x: 6.123233995736766e-17, y: 0, z: 1 });
  expect(toS2Point({ x: 0, y: 180 })).toEqual({ x: -1, y: -0, z: 1.2246467991473532e-16 });
});
