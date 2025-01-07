import {
  e5,
  e6,
  e7,
  fromDegrees,
  fromKM,
  fromLonLat,
  fromMeters,
  fromS2Points,
  normalize,
  toDegrees,
  toE5,
  toE6,
  toE7,
  toKM,
  toMeters,
} from '../../../src/geometry/s1/angle';
import { expect, test } from 'bun:test';

import { fromLonLat as pointFromLonLat } from '../../../src/geometry/s2/point';

test('e5', () => {
  expect(e5(0)).toBe(0);
  expect(e5(1)).toBe(5729577.951308233);
  expect(e5(-1)).toBe(-5729577.951308233);

  expect(e5(toE5(0))).toBe(0);
});

test('e6', () => {
  expect(e6(0)).toBe(0);
  expect(e6(1)).toBe(57295779.513082325);
  expect(e6(-1)).toBe(-57295779.513082325);
  expect(e6(toE6(0))).toBe(0);
});

test('e7', () => {
  expect(e7(0)).toBe(0);
  expect(e7(1)).toBe(572957795.1308233);
  expect(e7(-1)).toBe(-572957795.1308233);
  expect(e7(toE7(0))).toBe(0);
});

test('fromDegrees', () => {
  expect(fromDegrees(0)).toBe(0);
  expect(fromDegrees(1)).toBe(0.017453292519943295);
  expect(fromDegrees(90)).toBe(1.5707963267948966);
  expect(fromDegrees(180)).toBe(3.141592653589793);
  expect(fromDegrees(360)).toBe(6.283185307179586);
});

test('fromKM', () => {
  expect(fromKM(0)).toBe(0);
  expect(fromKM(1)).toBe(0.00015696101377226163);
  expect(fromKM(10)).toBe(0.0015696101377226162);
  expect(fromKM(100)).toBe(0.015696101377226164);
  expect(fromKM(1000)).toBe(0.15696101377226163);
});

test('fromLonLat', () => {
  expect(fromLonLat([0, 0], [0, 0])).toBe(0);
  expect(fromLonLat([1, 0], [0, 0])).toBe(0.017453292519943295);
  expect(fromLonLat([90, 0], [0, 0])).toBe(1.5707963267948963);
  expect(fromLonLat([45, 20], [60, 40])).toBe(0.4148806056779849);
});

test('fromMeters', () => {
  expect(fromMeters(0)).toBe(0);
  expect(fromMeters(1)).toBe(1.5696101377226164e-7);
  expect(fromMeters(10)).toBe(0.0000015696101377226163);
  expect(fromMeters(100)).toBe(0.000015696101377226163);
  expect(fromMeters(1000)).toBe(0.00015696101377226163);
});

test('fromS2Points', () => {
  expect(fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(0, 0))).toBe(0);
  expect(fromS2Points(pointFromLonLat(1, 0), pointFromLonLat(0, 0))).toBe(0.017453292519943295);
  expect(fromS2Points(pointFromLonLat(90, 0), pointFromLonLat(0, 0))).toBe(1.5707963267948966);
  expect(fromS2Points(pointFromLonLat(45, 20), pointFromLonLat(60, 40))).toBe(0.41488060567798485);
});

test('normalize', () => {
  expect(normalize(0)).toBe(0);
  expect(normalize(fromDegrees(1))).toBe(0.017453292519943295);
  expect(normalize(fromDegrees(180))).toBe(3.141592653589793);
  expect(normalize(fromDegrees(360))).toBe(0);
});

test('toDegrees', () => {
  expect(toDegrees(0)).toBe(0);
  expect(toDegrees(0.017453292519943295)).toBe(1);
  expect(toDegrees(1.5707963267948966)).toBe(90);
  expect(toDegrees(3.141592653589793)).toBe(180);
  expect(toDegrees(6.283185307179586)).toBe(360);
});

test('toKM', () => {
  expect(toKM(0)).toBe(0);
  expect(toKM(fromDegrees(180))).toBe(20015.114442035923);
});

test('toMeters', () => {
  expect(toMeters(0)).toBe(0);
  expect(toMeters(fromDegrees(180))).toBe(20015114.442035925);
});
