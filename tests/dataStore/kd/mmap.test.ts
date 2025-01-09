import { KDMMapSpatialIndex } from '../../../src/dataStore/kd/mmap';
import { expect, test } from 'bun:test';

// Tools
import cities from 'all-the-cities';

import tmp from 'tmp';
tmp.setGracefulCleanup();

test('KDTree - MMAP', (): void => {
  const dir = tmp.dirSync({ prefix: 'file_test' });
  const store = new KDMMapSpatialIndex<{ a: number }>(64, dir.name);
  expect(store.length).toBe(0);
  store.push({ x: 0, y: 1, m: { a: 1 } });
  expect(store.length).toBe(1);
  store.push({ x: 1, y: 2, m: { a: 2 } });
  store.push({ x: 72, y: 72, m: { a: 3 } });
  store.push({ x: 22, y: 4, m: { a: 4 } });
  expect(store.length).toBe(4);
  expect(store.get(0)).toEqual({ x: 0, y: 1, m: { a: 1 } });
  expect(store.get(1)).toEqual({ x: 1, y: 2, m: { a: 2 } });

  expect(store.getRange(0, 2)).toStrictEqual([
    { x: 0, y: 1, m: { a: 1 } },
    { x: 1, y: 2, m: { a: 2 } },
  ]);

  expect(Array.from(store)).toStrictEqual([
    { m: { a: 1 }, x: 0, y: 1 },
    { m: { a: 2 }, x: 1, y: 2 },
    { m: { a: 3 }, x: 72, y: 72 },
    { m: { a: 4 }, x: 22, y: 4 },
  ]);

  store.sort();

  expect(Array.from(store)).toStrictEqual([
    { m: { a: 1 }, x: 0, y: 1 },
    { m: { a: 2 }, x: 1, y: 2 },
    { m: { a: 3 }, x: 72, y: 72 },
    { m: { a: 4 }, x: 22, y: 4 },
  ]);

  store.close();
});

test('KDTree - MMap [cities]', (): void => {
  const store = new KDMMapSpatialIndex<{ name: string }>();

  for (const city of cities) {
    const {
      name,
      loc: {
        coordinates: [lon, lat],
      },
    } = city;
    store.push({ x: lon, y: lat, m: { name } });
  }

  expect(store.getRange(5_000, 5_010)).toStrictEqual([
    { m: { name: 'Phillip' }, x: 150.85, y: -31.13333 },
    { m: { name: 'Petrie' }, x: 152.98333, y: -27.26667 },
    { m: { name: 'Perth' }, x: 147.17096, y: -41.57231 },
    { m: { name: 'Penshurst' }, x: 151.08333, y: -33.96667 },
    { m: { name: 'Penrith' }, x: 150.7, y: -33.75 },
    { m: { name: 'Penola' }, x: 140.83678, y: -37.3752 },
    { m: { name: 'Pennant Hills' }, x: 151.07216, y: -33.73783 },
    { m: { name: 'Penguin' }, x: 146.07318, y: -41.12258 },
    { m: { name: 'Pendle Hill' }, x: 150.95543, y: -33.80402 },
    { m: { name: 'Pearcedale' }, x: 145.23488, y: -38.203 },
  ]);

  store.sort();

  expect(store.getRange(5_000, 5_010)).toStrictEqual([
    { m: { name: 'San Lucas Camotlán' }, x: -95.71382, y: 16.94473 },
    { m: { name: 'Santo Domingo Tepuxtepec' }, x: -96.05823, y: 16.95584 },
    { m: { name: 'La Candelaria' }, x: -95.93306, y: 17.21833 },
    { m: { name: 'Zarzal' }, x: -95.03738, y: 17.01419 },
    { m: { name: 'Río Pachiñe' }, x: -95.06533, y: 16.9687 },
    { m: { name: 'Paso Real de Sarabia' }, x: -95.04667, y: 17.07389 },
    { m: { name: 'Palomares' }, x: -95.06266, y: 17.1383 },
    { m: { name: 'San Juan Guichicovi' }, x: -95.09418, y: 16.96231 },
    { m: { name: 'Santiago Ixcuintepec' }, x: -95.62282, y: 16.9354 },
    { m: { name: 'Cuauhtémoc' }, x: -94.88532, y: 17.10149 },
  ]);

  store.close();
});
