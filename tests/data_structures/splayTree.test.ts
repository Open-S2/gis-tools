import { SplayTreeSet } from '../../src/index.js';
import { expect, test } from 'bun:test';

test('splay tree', () => {
  const tree = new SplayTreeSet();

  expect(tree.length).toEqual(0);
  expect(tree.has(3)).toEqual(false);

  tree.add(1);
  tree.add(2);
  tree.add(3);
  tree.add(4);
  tree.add(5);
  tree.add(6);

  expect(tree.length).toEqual(6);
  expect(tree.has(3)).toEqual(true);

  tree.delete(3);

  expect(tree.length).toEqual(5);
  expect(tree.has(3)).toEqual(false);

  expect(tree.first()).toEqual(1);
  expect(tree.last()).toEqual(6);

  expect(tree.lastBefore(3)).toEqual(2);
  expect(tree.lastBefore(1)).toEqual(undefined);

  expect(tree.firstAfter(3)).toEqual(4);
  expect(tree.firstAfter(6)).toEqual(undefined);
});

test('splay tree - unordered with duplicates', () => {
  const tree = new SplayTreeSet();

  expect(tree.length).toEqual(0);
  expect(tree.has(3)).toEqual(false);

  tree.add(20);
  tree.add(1);
  tree.add(-2);
  tree.add(3);
  tree.add(44);
  tree.add(5);
  tree.add(4);
  tree.add(4);
  tree.add(4);
  tree.add(4);
  tree.add(4);
  tree.add(4);
  tree.add(4);

  expect(tree.length).toEqual(7);
  expect(tree.has(3)).toEqual(true);

  tree.delete(3);
  tree.delete(3);
  tree.delete(3);
  tree.delete(3);
  tree.delete(3);

  expect(tree.length).toEqual(6);
  expect(tree.has(3)).toEqual(false);

  expect(tree.first()).toEqual(-2);
  expect(tree.last()).toEqual(44);

  expect(tree.lastBefore(3)).toEqual(1);
  expect(tree.lastBefore(-2)).toEqual(undefined);

  expect(tree.firstAfter(3)).toEqual(4);
  expect(tree.firstAfter(6)).toEqual(20);
  expect(tree.firstAfter(44)).toEqual(undefined);
});
