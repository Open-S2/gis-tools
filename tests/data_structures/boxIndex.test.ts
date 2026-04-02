import { BoxIndex } from '../../src/index.js';
import { expect, test } from 'bun:test';

import type { BoxIndexAccessor } from '../../src/index.js';

const data = [
  8, 62, 11, 66, 57, 17, 57, 19, 76, 26, 79, 29, 36, 56, 38, 56, 92, 77, 96, 80, 87, 70, 90, 74, 43,
  41, 47, 43, 0, 58, 2, 62, 76, 86, 80, 89, 27, 13, 27, 15, 71, 63, 75, 67, 25, 2, 27, 2, 87, 6, 88,
  6, 22, 90, 23, 93, 22, 89, 22, 93, 57, 11, 61, 13, 61, 55, 63, 56, 17, 85, 21, 87, 33, 43, 37, 43,
  6, 1, 7, 3, 80, 87, 80, 87, 23, 50, 26, 52, 58, 89, 58, 89, 12, 30, 15, 34, 32, 58, 36, 61, 41,
  84, 44, 87, 44, 18, 44, 19, 13, 63, 15, 67, 52, 70, 54, 74, 57, 59, 58, 59, 17, 90, 20, 92, 48,
  53, 52, 56, 92, 68, 92, 72, 26, 52, 30, 52, 56, 23, 57, 26, 88, 48, 88, 48, 66, 13, 67, 15, 7, 82,
  8, 86, 46, 68, 50, 68, 37, 33, 38, 36, 6, 15, 8, 18, 85, 36, 89, 38, 82, 45, 84, 48, 12, 2, 16, 3,
  26, 15, 26, 16, 55, 23, 59, 26, 76, 37, 79, 39, 86, 74, 90, 77, 16, 75, 18, 78, 44, 18, 45, 21,
  52, 67, 54, 71, 59, 78, 62, 78, 24, 5, 24, 8, 64, 80, 64, 83, 66, 55, 70, 55, 0, 17, 2, 19, 15,
  71, 18, 74, 87, 57, 87, 59, 6, 34, 7, 37, 34, 30, 37, 32, 51, 19, 53, 19, 72, 51, 73, 55, 29, 45,
  30, 45, 94, 94, 96, 95, 7, 22, 11, 24, 86, 45, 87, 48, 33, 62, 34, 65, 18, 10, 21, 14, 64, 66, 67,
  67, 64, 25, 65, 28, 27, 4, 31, 6, 84, 4, 85, 5, 48, 80, 50, 81, 1, 61, 3, 61, 71, 89, 74, 92, 40,
  42, 43, 43, 27, 64, 28, 66, 46, 26, 50, 26, 53, 83, 57, 87, 14, 75, 15, 79, 31, 45, 34, 45, 89,
  84, 92, 88, 84, 51, 85, 53, 67, 87, 67, 89, 39, 26, 43, 27, 47, 61, 47, 63, 23, 49, 25, 53, 12, 3,
  14, 5, 16, 50, 19, 53, 63, 80, 64, 84, 22, 63, 22, 64, 26, 66, 29, 66, 2, 15, 3, 15, 74, 77, 77,
  79, 64, 11, 68, 11, 38, 4, 39, 8, 83, 73, 87, 77, 85, 52, 89, 56, 74, 60, 76, 63, 62, 66, 65, 67,
];

/** Test item */
interface Item {
  id: number;
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

/**
 * A function for accessing the minX, minY, maxX, and maxY properties of the items.
 * @param item - The item to access.
 * @returns An array of the minX, minY, maxX, and maxY properties of the item.
 */
const accessor: BoxIndexAccessor<Item> = (item: Item) => [
  item.minX,
  item.minY,
  item.maxX,
  item.maxY,
];

/**
 * @returns A new BoxIndex instance using data
 */
function createIndex(): BoxIndex<Item> {
  const items: Item[] = [];

  for (let i = 0; i < data.length; i += 4) {
    items.push({
      id: i,
      minX: data[i],
      minY: data[i + 1],
      maxX: data[i + 2],
      maxY: data[i + 3],
    });
  }

  return new BoxIndex<Item>(items, accessor);
}

test('box index basic test', () => {
  const index = new BoxIndex<Item>([], accessor);

  expect(index).toBeDefined();
});

test('indexes a bunch of rectangles', () => {
  const index = createIndex();

  expect(index.minX).toEqual(0);
  expect(index.minY).toEqual(1);
  expect(index.maxX).toEqual(96);
  expect(index.maxY).toEqual(95);
});

test('performs bbox search', () => {
  const index = createIndex();

  const items = index.search(40, 40, 60, 60);

  expect(items).toEqual([
    {
      id: 124,
      maxX: 52,
      maxY: 56,
      minX: 48,
      minY: 53,
    },
    {
      id: 116,
      maxX: 58,
      maxY: 59,
      minX: 57,
      minY: 59,
    },
    {
      id: 24,
      maxX: 47,
      maxY: 43,
      minX: 43,
      minY: 41,
    },
    {
      id: 300,
      maxX: 43,
      maxY: 43,
      minX: 40,
      minY: 42,
    },
  ]);
});

test('performs a k-nearest-neighbors query', () => {
  const index = createIndex();
  const found = index.neighbors(50, 50, 3);

  expect(found).toEqual([
    {
      id: 124,
      maxX: 52,
      maxY: 56,
      minX: 48,
      minY: 53,
    },
    {
      id: 24,
      maxX: 47,
      maxY: 43,
      minX: 43,
      minY: 41,
    },
    {
      id: 300,
      maxX: 43,
      maxY: 43,
      minX: 40,
      minY: 42,
    },
  ]);
});

test('k-nearest-neighbors query accepts maxDistance', () => {
  const index = createIndex();
  const items = index.neighbors(50, 50, Infinity, 12);

  expect(items).toEqual([
    {
      id: 124,
      maxX: 52,
      maxY: 56,
      minX: 48,
      minY: 53,
    },
    {
      id: 24,
      maxX: 47,
      maxY: 43,
      minX: 43,
      minY: 41,
    },
    {
      id: 300,
      maxX: 43,
      maxY: 43,
      minX: 40,
      minY: 42,
    },
    {
      id: 340,
      maxX: 47,
      maxY: 63,
      minX: 47,
      minY: 61,
    },
    {
      id: 116,
      maxX: 58,
      maxY: 59,
      minX: 57,
      minY: 59,
    },
  ]);
});

test('k-nearest-neighbors query accepts filterFn', () => {
  const index = createIndex();
  const items = index.neighbors(50, 50, 6, Infinity, (item) => item.minX > 50);

  expect(items).toEqual([
    {
      id: 116,
      maxX: 58,
      maxY: 59,
      minX: 57,
      minY: 59,
    },
    {
      id: 64,
      maxX: 63,
      maxY: 56,
      minX: 61,
      minY: 55,
    },
    {
      id: 216,
      maxX: 70,
      maxY: 55,
      minX: 66,
      minY: 55,
    },
    {
      id: 200,
      maxX: 54,
      maxY: 71,
      minX: 52,
      minY: 67,
    },
    {
      id: 396,
      maxX: 65,
      maxY: 67,
      minX: 62,
      minY: 66,
    },
    {
      id: 112,
      maxX: 54,
      maxY: 74,
      minX: 52,
      minY: 70,
    },
  ]);
});
