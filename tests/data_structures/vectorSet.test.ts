import { VectorSet } from '../../src/index.js';
import { expect, test } from 'bun:test';

test('vector set', () => {
  const vecSet = new VectorSet();

  expect(vecSet.length).toEqual(0);
  expect(vecSet.has(3)).toEqual(false);

  vecSet.add(1);
  vecSet.add(2);
  vecSet.add(3);
  vecSet.add(4);
  vecSet.add(5);
  vecSet.add(6);

  expect(vecSet.length).toEqual(6);
  expect(vecSet.has(3)).toEqual(true);

  vecSet.delete(3);

  expect(vecSet.length).toEqual(5);
  expect(vecSet.has(3)).toEqual(false);

  expect(vecSet.first()).toEqual(1);
  expect(vecSet.last()).toEqual(6);

  expect(vecSet.lastBefore(3)).toEqual(2);
  expect(vecSet.lastBefore(1)).toEqual(undefined);

  expect(vecSet.firstAfter(3)).toEqual(4);
  expect(vecSet.firstAfter(6)).toEqual(undefined);
});

test('vector set - unordered with duplicates', () => {
  const vecSet = new VectorSet();

  expect(vecSet.length).toEqual(0);
  expect(vecSet.has(3)).toEqual(false);

  vecSet.add(20);
  vecSet.add(1);
  vecSet.add(-2);
  vecSet.add(3);
  vecSet.add(44);
  vecSet.add(5);
  vecSet.add(4);
  vecSet.add(4);
  vecSet.add(4);
  vecSet.add(4);
  vecSet.add(4);
  vecSet.add(4);
  vecSet.add(4);

  expect(vecSet.length).toEqual(7);
  expect(vecSet.has(3)).toEqual(true);

  vecSet.delete(3);
  vecSet.delete(3);
  vecSet.delete(3);
  vecSet.delete(3);
  vecSet.delete(3);

  expect(vecSet.length).toEqual(6);
  expect(vecSet.has(3)).toEqual(false);

  expect(vecSet.first()).toEqual(-2);
  expect(vecSet.last()).toEqual(44);

  expect(vecSet.lastBefore(3)).toEqual(1);
  expect(vecSet.lastBefore(-2)).toEqual(undefined);

  expect(vecSet.firstAfter(3)).toEqual(4);
  expect(vecSet.firstAfter(6)).toEqual(20);
  expect(vecSet.firstAfter(44)).toEqual(undefined);
});
