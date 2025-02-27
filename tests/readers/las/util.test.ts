import { U32I32F32 } from '../../../src/readers/las/util';
import { expect, test } from 'bun:test';

test('U32I32F32', () => {
  const u32i32f32 = new U32I32F32(42);
  expect(u32i32f32.u32).toBe(42);
  u32i32f32.i32 = 42;
  expect(u32i32f32.i32).toBe(42);
  u32i32f32.f32 = 42.2;
  expect(u32i32f32.f32).toBe(42.20000076293945);
});
