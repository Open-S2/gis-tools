import { S2FileStore } from '../../../src/file';
import { S2MMapStore, externalSort } from '../../../src/mmap';
import { expect, test } from 'bun:test';

import tmp from 'tmp';
tmp.setGracefulCleanup();

test('sort - single threaded', async () => {
  const dir = tmp.dirSync({ prefix: 'externalSort_single' });
  const name = `${dir.name}/sort-single-threaded`;
  const store = new S2FileStore<{ a: number }>(name);

  store.set(0, { a: 1 });
  store.set(1, { a: 2 });
  store.set(5_005, { a: 3 });
  store.set(22, { a: 4 });
  store.set(22, { a: 5 });
  store.set(22, { a: 6 });

  store.close();

  await externalSort([name], name);

  const storeSorted = new S2FileStore<{ a: number }>(name, { isSorted: true });
  const data = await Array.fromAsync(storeSorted.entries());

  expect(data).toStrictEqual([
    { key: 0n, value: { a: 1 } },
    { key: 1n, value: { a: 2 } },
    { key: 22n, value: { a: 4 } },
    { key: 22n, value: { a: 5 } },
    { key: 22n, value: { a: 6 } },
    { key: 5_005n, value: { a: 3 } },
  ]);
});

test('sort multi-file - single threaded', async () => {
  const dir = tmp.dirSync({ prefix: 'externalSort_single' });

  const storeA = new S2FileStore<{ a: number }>(`${dir.name}/a`);
  storeA.set(0, { a: 1 });
  storeA.set(1, { a: 2 });
  storeA.set(22, { a: 6 });
  storeA.close();

  const storeB = new S2FileStore<{ a: number }>(`${dir.name}/b`);
  storeB.set(5_005, { a: 3 });
  storeB.set(22, { a: 4 });
  storeB.set(22, { a: 5 });
  storeB.close();

  const storeC = new S2FileStore<{ a: number }>(`${dir.name}/c`);
  storeC.set(9807, { a: 7 });
  storeC.set(456, { a: 8 });
  storeC.set(55, { a: 9 });
  storeC.set(12, { a: 10 });
  storeC.close();

  await externalSort([`${dir.name}/a`, `${dir.name}/b`, `${dir.name}/c`], `${dir.name}/a`);

  const storeSorted = new S2FileStore<{ a: number }>(`${dir.name}/a`, { isSorted: true });
  const data = await Array.fromAsync(storeSorted.entries());

  expect(data).toStrictEqual([
    { key: 0n, value: { a: 1 } },
    { key: 1n, value: { a: 2 } },
    { key: 12n, value: { a: 10 } },
    { key: 22n, value: { a: 6 } },
    { key: 22n, value: { a: 4 } },
    { key: 22n, value: { a: 5 } },
    { key: 55n, value: { a: 9 } },
    { key: 456n, value: { a: 8 } },
    { key: 5_005n, value: { a: 3 } },
    { key: 9_807n, value: { a: 7 } },
  ]);
});

test('sort - multi threaded', async () => {
  const dir = tmp.dirSync({ prefix: 'externalSort_single' });
  const name = `${dir.name}/sort-multi-threaded`;
  const store = new S2MMapStore<{ a: number }>(name);

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

  await externalSort([name], name, 2, 2);

  const storeSorted = new S2FileStore<{ a: number }>(name, { isSorted: true });
  const data = await Array.fromAsync(storeSorted.entries());

  // We cant strict equal because threading, so just check that all key-vaue pairs exist
  expect(data.sort((a, b) => a.value.a - b.value.a)).toEqual(
    [
      { key: 0n, value: { a: 1 } },
      { key: 1n, value: { a: 2 } },
      { key: 5005n, value: { a: 3 } },
      { key: 22n, value: { a: 4 } },
      { key: 22n, value: { a: 5 } },
      { key: 22n, value: { a: 6 } },
      { key: 9807n, value: { a: 7 } },
      { key: 456n, value: { a: 8 } },
      { key: 55n, value: { a: 9 } },
      { key: 12n, value: { a: 10 } },
      { key: 7n, value: { a: 11 } },
      { key: 100n, value: { a: 12 } },
      { key: 9807n, value: { a: 13 } },
      { key: 456n, value: { a: 14 } },
      { key: 55n, value: { a: 15 } },
      { key: 12n, value: { a: 16 } },
      { key: 7n, value: { a: 17 } },
      { key: 100n, value: { a: 18 } },
      { key: 93n, value: { a: 19 } },
      { key: 66n, value: { a: 20 } },
      { key: 11n, value: { a: 21 } },
      { key: 901n, value: { a: 22 } },
      { key: 98081n, value: { a: 23 } },
      { key: 987678987330n, value: { a: 24 } },
    ].sort((a, b) => a.value.a - b.value.a),
  );
});
