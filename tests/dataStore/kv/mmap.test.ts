import MMapKV from '../../../src/dataStore/kv/mmap';
import { expect, test } from 'bun:test';

test('KV - MMap', async () => {
  const store = new MMapKV<number>();
  expect(store.length).toBe(0);
  store.set(0, 1);
  expect(store.length).toBe(1);
  store.set(1, 2);
  store.set(5_005, 3);
  store.set(22, 4);
  expect(store.length).toBe(4);
  expect(await store.get(0)).toBe(1);
  expect(await store.get(1)).toBe(2);

  const values = await Array.fromAsync(store.values());
  const values2 = await Array.fromAsync(store);
  expect(values).toStrictEqual([1, 2, 4, 3]);
  expect(values2).toStrictEqual([1, 2, 4, 3]);
  store.close();
});
