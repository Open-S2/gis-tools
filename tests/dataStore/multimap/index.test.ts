import { MultiMap } from '../../../src';
import { expect, test } from 'bun:test';

test('MultiMap', async () => {
  const store = new MultiMap<number>();
  expect(store.length).toBe(0);
  store.set(0, 1);
  expect(store.length).toBe(1);
  store.set(1, 2);
  store.set(5_005, 3);
  store.set(22, 4);
  store.set(22, 5);
  expect(store.length).toBe(5);
  expect(store.has(0)).toBeTrue();
  expect(store.get(0)).toStrictEqual([1]);
  expect(store.get(1)).toStrictEqual([2]);
  expect(store.get(22)).toStrictEqual([4, 5]);

  const values = await Array.fromAsync(store);
  expect(values).toStrictEqual([
    { key: 0n, value: [1] },
    { key: 1n, value: [2] },
    { key: 22n, value: [4, 5] },
    { key: 5_005n, value: [3] },
  ]);

  store.close();
});
