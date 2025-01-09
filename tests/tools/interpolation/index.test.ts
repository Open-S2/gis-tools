import { defaultGetInterpolateCurrentValue } from '../../../src';
import { expect, test } from 'bun:test';

test('defaultGetInterpolateCurrentValue', () => {
  expect(defaultGetInterpolateCurrentValue({ x: 1, y: 1 })).toEqual(0);
  expect(defaultGetInterpolateCurrentValue({ x: 1, y: 1, z: 0 })).toEqual(0);
  expect(defaultGetInterpolateCurrentValue({ x: 1, y: 1, z: 102.2 })).toEqual(102.2);
});
