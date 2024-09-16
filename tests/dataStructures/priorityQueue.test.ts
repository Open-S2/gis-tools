import PriorityQueue from '../../src/dataStructures/priorityQueue';
import { beforeAll, expect, test } from 'bun:test';

import type { CompareFunction } from '../../src/dataStructures/priorityQueue';

const data: number[] = [];
let sorted: number[] = [];

beforeAll(() => {
  for (let i = 0; i < 100; i++) {
    data.push(Math.floor(100 * Math.random()));
  }
  sorted = [...data].sort((a, b) => a - b);
});

test('maintains a priority queue', () => {
  const queue = new PriorityQueue();
  for (const n of data) queue.push(n);

  expect(queue.peek()).toEqual(sorted[0]);

  const result: number[] = [];
  while (queue.length) result.push(queue.pop() ?? 999_999);

  expect(result).toEqual(sorted);
});

test('accepts data in constructor', () => {
  const queue = new PriorityQueue(data.slice());

  const result: number[] = [];
  while (queue.length) result.push(queue.pop() ?? 999_999);

  expect(result).toEqual(sorted);
});

test('handles edge cases with few elements', () => {
  const queue = new PriorityQueue();

  queue.push(2);
  queue.push(1);
  queue.pop();
  queue.pop();
  queue.pop();
  queue.push(2);
  queue.push(1);
  expect(queue.pop()).toEqual(1);
  expect(queue.pop()).toEqual(2);
  expect(queue.pop()).toBeUndefined();
});

test('handles init with empty array', () => {
  const queue = new PriorityQueue([]);
  expect(queue.length).toEqual(0);
  // @ts-expect-error - access just for testing purposes
  expect(queue.data).toEqual([]);
});

test('handle object type', () => {
  /** Example structure */
  type StructTest = { x: number };
  /**
   * @param a - the first element
   * @param b - the second element
   * @returns - comparison result
   */
  const comparitor: CompareFunction<StructTest> = (a: StructTest, b: StructTest): 0 | 1 | -1 =>
    a.x < b.x ? -1 : a.x > b.x ? 1 : 0;
  const queue = new PriorityQueue<StructTest>([], comparitor);

  queue.push({ x: 8 });
  queue.push({ x: 2 });
  queue.push({ x: 1 });
  queue.push({ x: 4 });
  queue.push({ x: 22 });
  expect(queue.pop()?.x).toEqual(1);
  expect(queue.pop()?.x).toEqual(2);
  expect(queue.pop()?.x).toEqual(4);
  expect(queue.pop()?.x).toEqual(8);
  expect(queue.pop()?.x).toEqual(22);
  expect(queue.pop()).toBeUndefined();
});
