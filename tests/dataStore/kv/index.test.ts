import { KV } from '../../../src/dataStore/kv';
import { expect, test } from 'bun:test';

test('KV', async () => {
  const store = new KV<number>();
  expect(store.length).toBe(0);
  store.set(0, 1);
  expect(store.length).toBe(1);
  store.set(1, 2);
  store.set(5_005, 3);
  store.set(22, 4);
  expect(store.length).toBe(4);
  expect(store.get(0)).toBe(1);
  expect(store.get(1)).toBe(2);

  const values = await Array.fromAsync(store.values());
  const values2 = await Array.fromAsync(store);
  expect(values).toStrictEqual([1, 2, 3, 4]);
  expect(values2).toStrictEqual([1, 2, 3, 4]);
  store.close();
});
