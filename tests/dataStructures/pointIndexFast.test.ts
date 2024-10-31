import { PointIndexFast, sphericalDistance } from '../../src';
import { expect, test } from 'bun:test';

// HELPER TOOLS
import cities from 'all-the-cities';
import { fromLonLat as pointFromLonLat } from '../../src/geometry/s2/point';
import { fromS2Points, toMeters } from '../../src/geometry/s1/chordAngle';

test('point index', () => {
  const pointIndex = new PointIndexFast<{ a: number }>();

  pointIndex.insertLonLat(0, 0, { a: 0 });
  pointIndex.insertLonLat(0, 1, { a: 1 });
  pointIndex.insertLonLat(-20, 20, { a: 2 });
  pointIndex.insertLonLat(-22, 22, { a: 3 });
  pointIndex.insertFaceST(0, 0, 0, { a: 4 });

  pointIndex.sort();

  const radiusRes = pointIndex.searchRadius(0, 0, 5);
  expect(radiusRes).toEqual([
    { data: { a: 0 }, x: 0, y: 0 },
    { data: { a: 1 }, x: 0, y: 1 },
  ]);

  const rangeRes = pointIndex.searchRange(-24, 18, -18, 24);
  expect(rangeRes).toEqual([
    { data: { a: 2 }, x: -20, y: 20 },
    { data: { a: 3 }, x: -22, y: 22 },
  ]);

  const radiusResSphere = pointIndex.searchRadiusSphere(
    0,
    0,
    toMeters(fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(2, 2))),
  );
  expect(radiusResSphere).toEqual([
    { data: { a: 0 }, x: 0, y: 0 },
    { data: { a: 1 }, x: 0, y: 1 },
  ]);

  const allRes = Array.from(pointIndex);
  expect(allRes).toEqual([
    { data: { a: 0 }, x: 0, y: 0 },
    { data: { a: 1 }, x: 0, y: 1 },
    { data: { a: 2 }, x: -20, y: 20 },
    { data: { a: 3 }, x: -22, y: 22 },
    { data: { a: 4 }, x: -45, y: -35.264389682754654 },
  ]);
});

test('point index spherical test across the -180/180 boundary', () => {
  const pointIndex = new PointIndexFast<{ a: number }>();

  pointIndex.insertLonLat(0, 0, { a: 0 });
  pointIndex.insertLonLat(0, 1, { a: 1 });
  pointIndex.insertLonLat(-179, 20, { a: 2 });
  pointIndex.insertLonLat(-178, 22, { a: 3 });
  pointIndex.insertLonLat(179, 21, { a: 4 });
  pointIndex.insertLonLat(178, 20.5, { a: 5 });

  pointIndex.sort();

  const radiusResSphere = pointIndex.searchRadiusSphere(
    -180,
    21,
    toMeters(fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(5, 5))),
  );
  expect(radiusResSphere).toEqual([
    { data: { a: 4 }, x: 179, y: 21 },
    { data: { a: 2 }, x: -179, y: 20 },
    { data: { a: 5 }, x: 178, y: 20.5 },
    { data: { a: 3 }, x: -178, y: 22 },
  ]);
});

test('point index fast - cities', () => {
  const pointIndex = new PointIndexFast<{ name: string }>();

  for (const city of cities) {
    const {
      name,
      loc: {
        coordinates: [lon, lat],
      },
    } = city;
    pointIndex.insertLonLat(lon, lat, { name });
  }

  pointIndex.sort();

  const radiusResSphere = pointIndex.searchRadiusSphere(
    -110,
    39,
    toMeters(fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(2, 2))),
  );
  expect(radiusResSphere).toEqual([
    { data: { name: 'Heber City' }, x: -111.41324, y: 40.5069 },
    { data: { name: 'Francis' }, x: -111.28074, y: 40.61051 },
    { data: { name: 'Midway' }, x: -111.47435, y: 40.51218 },
    { data: { name: 'Kamas' }, x: -111.28074, y: 40.64301 },
    { data: { name: 'Oakley' }, x: -111.30074, y: 40.71467 },
    { data: { name: 'Park City' }, x: -111.49797, y: 40.64606 },
    { data: { name: 'Snyderville' }, x: -111.54381, y: 40.69439 },
    { data: { name: 'Draper' }, x: -111.86382, y: 40.52467 },
    { data: { name: 'Granite' }, x: -111.80604, y: 40.573 },
    { data: { name: 'White City' }, x: -111.86438, y: 40.56578 },
    { data: { name: 'Sandy Hills' }, x: -111.85077, y: 40.58106 },
    { data: { name: 'Riverton' }, x: -111.9391, y: 40.52189 },
    { data: { name: 'Little Cottonwood Creek Valley' }, x: -111.82938, y: 40.60439 },
    { data: { name: 'Cottonwood Heights' }, x: -111.81021, y: 40.61967 },
    { data: { name: 'Sandy' }, x: -111.8841, y: 40.59161 },
    { data: { name: 'South Jordan' }, x: -111.92966, y: 40.56217 },
    { data: { name: 'South Jordan Heights' }, x: -111.94938, y: 40.56384 },
    { data: { name: 'Midvale' }, x: -111.89994, y: 40.61106 },
    { data: { name: 'Mount Olympus' }, x: -111.78854, y: 40.6855 },
    { data: { name: 'Herriman' }, x: -112.03299, y: 40.51411 },
    { data: { name: 'Holladay' }, x: -111.82466, y: 40.66884 },
    { data: { name: 'West Jordan' }, x: -111.9391, y: 40.60967 },
    { data: { name: 'East Millcreek' }, x: -111.81049, y: 40.69995 },
    { data: { name: 'Canyon Rim' }, x: -111.82188, y: 40.70661 },
    { data: { name: 'Murray' }, x: -111.88799, y: 40.66689 },
    { data: { name: 'Willard' }, x: -111.83743, y: 40.70634 },
    { data: { name: 'Millcreek' }, x: -111.87549, y: 40.68689 },
    { data: { name: 'Centerfield' }, x: -111.89799, y: 40.69134 },
    { data: { name: 'Taylorsville' }, x: -111.93883, y: 40.66772 },
    { data: { name: 'South Salt Lake' }, x: -111.88827, y: 40.71884 },
    { data: { name: 'Oquirrh' }, x: -112.03383, y: 40.6305 },
    { data: { name: 'Kearns' }, x: -111.99633, y: 40.65995 },
    { data: { name: 'West Valley City' }, x: -112.00105, y: 40.69161 },
    { data: { name: 'Tooele' }, x: -112.29828, y: 40.53078 },
    { data: { name: 'Magna' }, x: -112.10161, y: 40.70911 },
    { data: { name: 'Erda' }, x: -112.30439, y: 40.61272 },
    { data: { name: 'Stansbury park' }, x: -112.29606, y: 40.63772 },
    { data: { name: 'Craig' }, x: -107.54645, y: 40.51525 },
    { data: { name: 'Grantsville' }, x: -112.4644, y: 40.59994 },
    { data: { name: 'Manila' }, x: -109.72265, y: 40.98801 },
    { data: { name: 'Silver Summit' }, x: -111.48775, y: 40.74144 },
    { data: { name: 'Summit Park' }, x: -111.61159, y: 40.74578 },
    { data: { name: 'Coalville' }, x: -111.39936, y: 40.91773 },
    { data: { name: 'Salt Lake City' }, x: -111.89105, y: 40.76078 },
    { data: { name: 'North Salt Lake' }, x: -111.90688, y: 40.84856 },
    { data: { name: 'Woods Cross' }, x: -111.89216, y: 40.87161 },
    { data: { name: 'Bountiful' }, x: -111.88077, y: 40.88939 },
    { data: { name: 'West Bountiful' }, x: -111.90188, y: 40.89383 },
    { data: { name: 'Centerville' }, x: -111.87216, y: 40.918 },
    { data: { name: 'Morgan' }, x: -111.67688, y: 41.03606 },
    { data: { name: 'Farmington' }, x: -111.88744, y: 40.9805 },
    { data: { name: 'Fruit Heights' }, x: -111.90216, y: 41.03217 },
    { data: { name: 'Kaysville' }, x: -111.93855, y: 41.03522 },
  ]);
});

test('sphericalDistance', () => {
  expect(sphericalDistance(0, 0, 0, 0)).toBe(0);
  expect(sphericalDistance(-110, 32, -125, 40)).toBe(1_612_725.6676401352);
});
