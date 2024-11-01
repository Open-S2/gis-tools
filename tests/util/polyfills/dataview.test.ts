import '../../../src/util/polyfills/dataview';
import { expect, test } from 'bun:test';

test('dataview', () => {
  expect(new DataView(new ArrayBuffer(2))).toBeInstanceOf(DataView);
});
test('getFloat16', () => {
  const dv = new DataView(new ArrayBuffer(20));
  dv.setFloat16(0, 0x7c00);
  dv.setFloat16(2, 0, true);
  dv.setFloat16(4, 0.1);
  dv.setFloat16(6, -0.01, true);
  dv.setFloat16(8, Infinity);
  expect(dv.getFloat16(0)).toEqual(0x7c00);
  expect(dv.getFloat16(2, true)).toEqual(0);
  expect(dv.getFloat16(4)).toEqual(0.0999755859375);
  expect(dv.getFloat16(6, true)).toEqual(-0.0099945068359375);
  expect(dv.getFloat16(8)).toEqual(Infinity);
});
