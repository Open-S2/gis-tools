import { Vector } from '../../../src';
import { expect, test } from 'bun:test';

import type { VectorKey } from '../../../src';

/** Test key */
interface TestKey extends VectorKey {
  a: number;
}

test('Vector', async () => {
  const store = new Vector<TestKey>();
  expect(store.length).toBe(0);
  store.push({ a: 1, cell: 0n });
  expect(store.length).toBe(1);
  store.push({ a: 7, cell: 5000001n });
  store.push({ a: 2, cell: 1n });
  store.push({ a: 4, cell: 12345678900001n });
  store.push({ a: 5, cell: 12345678900000n });
  store.push({ a: 3, cell: 5000001n });
  store.sort();
  expect(store.length).toBe(6);
  expect(await store.get(0)).toEqual({ a: 1, cell: 0n });
  expect(await store.get(1)).toEqual({ a: 2, cell: 1n });

  const values = await Array.fromAsync(store.values());
  const values2 = await Array.fromAsync(store);
  expect(values).toStrictEqual([
    { a: 1, cell: 0n },
    { a: 2, cell: 1n },
    { a: 7, cell: 5000001n },
    { a: 3, cell: 5000001n },
    { a: 5, cell: 12345678900000n },
    { a: 4, cell: 12345678900001n },
  ]);
  expect(values2).toStrictEqual([
    { a: 1, cell: 0n },
    { a: 2, cell: 1n },
    { a: 7, cell: 5000001n },
    { a: 3, cell: 5000001n },
    { a: 5, cell: 12345678900000n },
    { a: 4, cell: 12345678900001n },
  ]);

  store.close();
});
