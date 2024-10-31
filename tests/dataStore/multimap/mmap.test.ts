import { MMapMultiMap } from '../../../src/mmap';
import { expect, test } from 'bun:test';

import tmp from 'tmp';
tmp.setGracefulCleanup();

test('MMapMultiMap', async () => {
  const dir = tmp.dirSync({ prefix: 'multimap_mmap' });
  const store = new MMapMultiMap<number>(dir.name);
  expect(store.length).toBe(0);
  store.set(0, 1);
  expect(store.length).toBe(1);
  store.set(1, 2);
  store.set(5_005, 3);
  store.set(22, 4);
  store.set(22, 5);
  expect(store.length).toBe(5);
  expect(await store.get(0)).toStrictEqual([1]);
  expect(await store.get(1)).toStrictEqual([2]);
  expect(await store.get(22)).toStrictEqual([4, 5]);

  const values = await Array.fromAsync(store);
  expect(values).toStrictEqual([
    { key: { high: 0, low: 0 }, value: [1] },
    { key: { high: 0, low: 1 }, value: [2] },
    { key: { high: 0, low: 22 }, value: [4, 5] },
    { key: { high: 0, low: 5005 }, value: [3] },
  ]);

  store.close();
});
