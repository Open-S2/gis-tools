import { FlatQueue } from '../../src';
import { beforeAll, expect, test } from 'bun:test';

const data: number[] = [];
let sorted: number[] = [];

beforeAll(() => {
  for (let i = 0; i < 100; i++) {
    data.push(Math.floor(100 * Math.random()));
  }
  sorted = [...data].sort((a, b) => a - b);
});

test('maintains a priority queue', () => {
  const queue = new FlatQueue();
  for (let i = 0; i < data.length; i++) queue.push(i, data[i]);

  expect(queue.peekValue()).toEqual(sorted[0]);
  expect(data[queue.peek() ?? 0]).toEqual(sorted[0]);

  const result: number[] = [];
  while (queue.length !== 0) result.push(data[queue.pop() ?? 0]);

  expect(result).toEqual(sorted);
});

test('handles edge cases with few elements', () => {
  const queue = new FlatQueue();

  queue.push(0, 2);
  queue.push(1, 1);
  queue.pop();
  queue.pop();
  queue.pop();
  queue.push(2, 2);
  queue.push(3, 1);
  expect(queue.pop()).toEqual(3);
  expect(queue.pop()).toEqual(2);
  expect(queue.pop()).toBeUndefined();
  expect(queue.peek()).toBeUndefined();
  expect(queue.peekValue()).toBeUndefined();
});

test('shrinks internal arrays when calling shrink', () => {
  const queue = new FlatQueue();

  for (let i = 0; i < 10; i++) queue.push(i, i);

  while (queue.length !== 0) queue.pop();

  // expect(queue.ids.length, 10);
  // expect(queue.values.length, 10);
  expect(queue.length).toEqual(0);

  queue.shrink();

  // expect(queue.ids.length, 0);
  // expect(queue.values.length, 0);
});
