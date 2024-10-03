import FileMultiMap from '../../../src/dataStore/multimap/file';
import { expect, test } from 'bun:test';

test('FileMultiMap', async () => {
  const store = new FileMultiMap<number>();
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

  store.close();
});
