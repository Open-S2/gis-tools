import { KDSpatialIndex } from '../../../src';
import { expect, test } from 'bun:test';

// Tools test
import cities from 'all-the-cities';

test('KDTree - Local', (): void => {
  const store = new KDSpatialIndex<{ a: number }>();
  expect(store.length).toBe(0);
  store.push({ x: 0, y: 1, data: { a: 1 } });
  expect(store.length).toBe(1);
  store.push({ x: 1, y: 2, data: { a: 2 } });
  store.push({ x: 72, y: 72, data: { a: 3 } });
  store.push({ x: 22, y: 4, data: { a: 4 } });
  expect(store.length).toBe(4);
  expect(store.get(0)).toEqual({ x: 0, y: 1, data: { a: 1 } });
  expect(store.get(1)).toEqual({ x: 1, y: 2, data: { a: 2 } });

  expect(store.getRange(0, 2)).toStrictEqual([
    { x: 0, y: 1, data: { a: 1 } },
    { x: 1, y: 2, data: { a: 2 } },
  ]);

  expect(Array.from(store)).toStrictEqual([
    { data: { a: 1 }, x: 0, y: 1 },
    { data: { a: 2 }, x: 1, y: 2 },
    { data: { a: 3 }, x: 72, y: 72 },
    { data: { a: 4 }, x: 22, y: 4 },
  ]);

  store.sort();

  expect(Array.from(store)).toStrictEqual([
    { data: { a: 1 }, x: 0, y: 1 },
    { data: { a: 2 }, x: 1, y: 2 },
    { data: { a: 3 }, x: 72, y: 72 },
    { data: { a: 4 }, x: 22, y: 4 },
  ]);

  store.close();
});

test('KD - Local [cities]', (): void => {
  const store = new KDSpatialIndex<{ name: string }>();

  for (const city of cities) {
    const {
      name,
      loc: {
        coordinates: [lon, lat],
      },
    } = city;
    store.push({ x: lon, y: lat, data: { name } });
  }

  expect(store.getRange(5_000, 5_010)).toStrictEqual([
    { data: { name: 'Phillip' }, x: 150.85, y: -31.13333 },
    { data: { name: 'Petrie' }, x: 152.98333, y: -27.26667 },
    { data: { name: 'Perth' }, x: 147.17096, y: -41.57231 },
    { data: { name: 'Penshurst' }, x: 151.08333, y: -33.96667 },
    { data: { name: 'Penrith' }, x: 150.7, y: -33.75 },
    { data: { name: 'Penola' }, x: 140.83678, y: -37.3752 },
    { data: { name: 'Pennant Hills' }, x: 151.07216, y: -33.73783 },
    { data: { name: 'Penguin' }, x: 146.07318, y: -41.12258 },
    { data: { name: 'Pendle Hill' }, x: 150.95543, y: -33.80402 },
    { data: { name: 'Pearcedale' }, x: 145.23488, y: -38.203 },
  ]);

  store.sort();

  expect(store.getRange(5_000, 5_010)).toStrictEqual([
    { data: { name: 'San Lucas Camotlán' }, x: -95.71382, y: 16.94473 },
    { data: { name: 'Santo Domingo Tepuxtepec' }, x: -96.05823, y: 16.95584 },
    { data: { name: 'La Candelaria' }, x: -95.93306, y: 17.21833 },
    { data: { name: 'Zarzal' }, x: -95.03738, y: 17.01419 },
    { data: { name: 'Río Pachiñe' }, x: -95.06533, y: 16.9687 },
    { data: { name: 'Paso Real de Sarabia' }, x: -95.04667, y: 17.07389 },
    { data: { name: 'Palomares' }, x: -95.06266, y: 17.1383 },
    { data: { name: 'San Juan Guichicovi' }, x: -95.09418, y: 16.96231 },
    { data: { name: 'Santiago Ixcuintepec' }, x: -95.62282, y: 16.9354 },
    { data: { name: 'Cuauhtémoc' }, x: -94.88532, y: 17.10149 },
  ]);

  store.close();
});
