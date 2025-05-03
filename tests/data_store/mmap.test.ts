import { S2MMapStore } from '../../src/mmap';
import { expect, test } from 'bun:test';

import tmp from 'tmp';
tmp.setGracefulCleanup();

test('S2MMapStore', async () => {
  const dir = tmp.dirSync({ prefix: 'mmap_test' });
  const store = new S2MMapStore<{ a: number }>(`${dir.name}/testA`);
  expect(store.length).toBe(0);
  store.set(0, { a: 1 });
  expect(store.length).toBe(1);
  store.set(1, { a: 2 });
  store.set(5_005, { a: 3 });
  store.set(22, { a: 4 });
  expect(await store.has(0)).toBeTrue();
  expect(await store.get(0)).toEqual([{ a: 1 }]);

  const entries = await Array.fromAsync(store.entries());
  expect(entries).toStrictEqual([
    { key: 0n, value: { a: 1 } },
    { key: 1n, value: { a: 2 } },
    { key: 22n, value: { a: 4 } },
    { key: 5_005n, value: { a: 3 } },
  ]);

  store.close(true);
});

test('S2MMapStore - valuesAreIndex', async () => {
  const store = new S2MMapStore<number>(undefined, { valuesAreIndex: true });
  expect(store.length).toBe(0);

  store.set(0, 1);
  store.set(1, 2);
  store.set(5_005, 3);
  store.set(22, 4);
  await store.sort();

  const entries = await Array.fromAsync(store.entries());
  expect(entries).toStrictEqual([
    { key: 0n, value: 1 },
    { key: 1n, value: 2 },
    { key: 22n, value: 4 },
    { key: 5_005n, value: 3 },
  ]);

  store.close(true);
});
