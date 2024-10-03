import { S2FileStore } from '../../../src/dataStore/file';
import { S2MMapStore } from '../../../src/dataStore/mmap';
import { externalSort } from '../../../src/dataStore/externalSort';
import { expect, test } from 'bun:test';

import tmp from 'tmp';

tmp.setGracefulCleanup();

test('sort - single threaded', async () => {
  const dir = tmp.dirSync({ prefix: 'externalSort_single' });
  const store = new S2FileStore<{ a: number }>(dir.name);

  store.set(0, { a: 1 });
  store.set(1, { a: 2 });
  store.set(5_005, { a: 3 });
  store.set(22, { a: 4 });
  store.set(22, { a: 5 });
  store.set(22, { a: 6 });

  store.close();

  await externalSort([`${dir.name}.keys`], `${dir.name}.sortedkeys`);

  const storeSorted = new S2FileStore<{ a: number }>(dir.name, { isSorted: true });
  const data = await Array.fromAsync(storeSorted.entries());

  expect(data).toStrictEqual([
    { key: { high: 0, low: 0 }, value: { a: 1 } },
    { key: { high: 0, low: 1 }, value: { a: 2 } },
    { key: { high: 0, low: 22 }, value: { a: 4 } },
    { key: { high: 0, low: 22 }, value: { a: 5 } },
    { key: { high: 0, low: 22 }, value: { a: 6 } },
    { key: { high: 0, low: 5005 }, value: { a: 3 } },
  ]);
});

test('sort - multi threaded', async () => {
  const dir = tmp.dirSync({ prefix: 'externalSort_single' });
  const store = new S2MMapStore<{ a: number }>(dir.name);

  store.set(0, { a: 1 });
  store.set(1, { a: 2 });
  store.set(5_005, { a: 3 });
  store.set(22, { a: 4 });
  store.set(22, { a: 5 });
  store.set(22, { a: 6 });

  store.set(9807, { a: 7 });
  store.set(456, { a: 8 });
  store.set(55, { a: 9 });
  store.set(12, { a: 10 });
  store.set(7, { a: 11 });
  store.set(100, { a: 12 });

  store.set(9807, { a: 13 });
  store.set(456, { a: 14 });
  store.set(55, { a: 15 });
  store.set(12, { a: 16 });
  store.set(7, { a: 17 });
  store.set(100, { a: 18 });

  store.set(93, { a: 19 });
  store.set(66, { a: 20 });
  store.set(11, { a: 21 });
  store.set(901, { a: 22 });
  store.set(98081, { a: 23 });
  store.set(987678987330, { a: 24 });

  store.close();

  await externalSort([`${dir.name}.keys`], `${dir.name}.sortedkeys`, 2, 2);

  const storeSorted = new S2FileStore<{ a: number }>(dir.name, { isSorted: true });
  const data = await Array.fromAsync(storeSorted.entries());

  // We cant strict equal because threading
  expect(data).toEqual([
    { key: { high: 0, low: 0 }, value: { a: 1 } },
    { key: { high: 0, low: 1 }, value: { a: 2 } },
    { key: { high: 0, low: 7 }, value: { a: 11 } },
    { key: { high: 0, low: 7 }, value: { a: 17 } },
    { key: { high: 0, low: 11 }, value: { a: 21 } },
    { key: { high: 0, low: 12 }, value: { a: 10 } },
    { key: { high: 0, low: 12 }, value: { a: 16 } },
    { key: { high: 0, low: 22 }, value: { a: 4 } },
    { key: { high: 0, low: 22 }, value: { a: 5 } },
    { key: { high: 0, low: 22 }, value: { a: 6 } },
    { key: { high: 0, low: 55 }, value: { a: 15 } },
    { key: { high: 0, low: 55 }, value: { a: 9 } },
    { key: { high: 0, low: 66 }, value: { a: 20 } },
    { key: { high: 0, low: 93 }, value: { a: 19 } },
    { key: { high: 0, low: 100 }, value: { a: 18 } },
    { key: { high: 0, low: 100 }, value: { a: 12 } },
    { key: { high: 0, low: 456 }, value: { a: 8 } },
    { key: { high: 0, low: 456 }, value: { a: 14 } },
    { key: { high: 0, low: 901 }, value: { a: 22 } },
    { key: { high: 0, low: 5005 }, value: { a: 3 } },
    { key: { high: 0, low: 9807 }, value: { a: 13 } },
    { key: { high: 0, low: 9807 }, value: { a: 7 } },
    { key: { high: 0, low: 98081 }, value: { a: 23 } },
    { key: { high: 229, low: 4131476546 }, value: { a: 24 } },
  ]);
});
