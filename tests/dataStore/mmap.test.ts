import { S2MMapStore } from '../../src/dataStore/mmap';
import { expect, test } from 'bun:test';

test('S2MMapStore', async () => {
  const store = new S2MMapStore<{ a: number }>();
  expect(store.length).toBe(0);
  store.set(0, { a: 1 });
  expect(store.length).toBe(1);
  store.set(1, { a: 2 });
  store.set(5_005, { a: 3 });
  store.set(22, { a: 4 });
  expect(await store.get(0)).toEqual([{ a: 1 }]);

  const entries = await Array.fromAsync(store.entries());
  expect(entries).toStrictEqual([
    { key: { low: 0, high: 0 }, value: { a: 1 } },
    { key: { low: 1, high: 0 }, value: { a: 2 } },
    { key: { low: 22, high: 0 }, value: { a: 4 } },
    { key: { low: 5_005, high: 0 }, value: { a: 3 } },
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
    { key: { low: 0, high: 0 }, value: 1 },
    { key: { low: 1, high: 0 }, value: 2 },
    { key: { low: 22, high: 0 }, value: 4 },
    { key: { low: 5_005, high: 0 }, value: 3 },
  ]);

  store.close(true);
});
