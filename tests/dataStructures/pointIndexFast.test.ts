import { FileReader } from '../../src/file';
import { JSONReader, PointIndexFast, sphericalDistance } from '../../src';
import { expect, test } from 'bun:test';

// HELPER TOOLS
import cities from 'all-the-cities';
import { fromLonLat as pointFromLonLat } from '../../src/geometry/s2/point';
import { fromS2Points, toMeters } from '../../src/geometry/s1/chordAngle';

test('point index fast', () => {
  const pointIndex = new PointIndexFast<{ a: number }>();

  pointIndex.insertLonLat(0, 0, { a: 0 });
  pointIndex.insertLonLat(0, 1, { a: 1 });
  pointIndex.insertLonLat(-20, 20, { a: 2 });
  pointIndex.insertLonLat(-22, 22, { a: 3 });
  pointIndex.insertFaceST(0, 0, 0, { a: 4 });

  pointIndex.sort();

  const radiusRes = pointIndex.searchRadius(0, 0, 5);
  expect(radiusRes).toEqual([
    { m: { a: 0 }, x: 0, y: 0 },
    { m: { a: 1 }, x: 0, y: 1 },
  ]);

  const rangeRes = pointIndex.searchRange(-24, 18, -18, 24);
  expect(rangeRes).toEqual([
    { m: { a: 2 }, x: -20, y: 20 },
    { m: { a: 3 }, x: -22, y: 22 },
  ]);

  const radiusResSphere = pointIndex.searchRadiusSphere(
    0,
    0,
    toMeters(fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(2, 2))),
  );
  expect(radiusResSphere).toEqual([
    { m: { a: 0 }, x: 0, y: 0 },
    { m: { a: 1 }, x: 0, y: 1 },
  ]);

  const allRes = Array.from(pointIndex);
  expect(allRes).toEqual([
    { m: { a: 0 }, x: 0, y: 0 },
    { m: { a: 1 }, x: 0, y: 1 },
    { m: { a: 2 }, x: -20, y: 20 },
    { m: { a: 3 }, x: -22, y: 22 },
    { m: { a: 4 }, x: -45, y: -35.264389682754654 },
  ]);
});

test('point index fast - from reader', async () => {
  const fileReaderPoints = new FileReader(`${__dirname}/../readers/json/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReaderPoints);
  const pointIndex = new PointIndexFast();
  await pointIndex.insertReader(jsonReader);

  const radiusRes = pointIndex.searchRadius(144.9584, -37.8173, 1);
  expect(radiusRes).toEqual([{ m: { name: 'Melbourne' }, x: 144.9584, y: -37.8173 }]);
});

test('point index fast spherical test across the -180/180 boundary', () => {
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
    { m: { a: 4 }, x: 179, y: 21 },
    { m: { a: 2 }, x: -179, y: 20 },
    { m: { a: 5 }, x: 178, y: 20.5 },
    { m: { a: 3 }, x: -178, y: 22 },
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
    { m: { name: 'Heber City' }, x: -111.41324, y: 40.5069 },
    { m: { name: 'Francis' }, x: -111.28074, y: 40.61051 },
    { m: { name: 'Midway' }, x: -111.47435, y: 40.51218 },
    { m: { name: 'Kamas' }, x: -111.28074, y: 40.64301 },
    { m: { name: 'Oakley' }, x: -111.30074, y: 40.71467 },
    { m: { name: 'Park City' }, x: -111.49797, y: 40.64606 },
    { m: { name: 'Snyderville' }, x: -111.54381, y: 40.69439 },
    { m: { name: 'Draper' }, x: -111.86382, y: 40.52467 },
    { m: { name: 'Granite' }, x: -111.80604, y: 40.573 },
    { m: { name: 'White City' }, x: -111.86438, y: 40.56578 },
    { m: { name: 'Sandy Hills' }, x: -111.85077, y: 40.58106 },
    { m: { name: 'Riverton' }, x: -111.9391, y: 40.52189 },
    { m: { name: 'Little Cottonwood Creek Valley' }, x: -111.82938, y: 40.60439 },
    { m: { name: 'Cottonwood Heights' }, x: -111.81021, y: 40.61967 },
    { m: { name: 'Sandy' }, x: -111.8841, y: 40.59161 },
    { m: { name: 'South Jordan' }, x: -111.92966, y: 40.56217 },
    { m: { name: 'South Jordan Heights' }, x: -111.94938, y: 40.56384 },
    { m: { name: 'Midvale' }, x: -111.89994, y: 40.61106 },
    { m: { name: 'Mount Olympus' }, x: -111.78854, y: 40.6855 },
    { m: { name: 'Herriman' }, x: -112.03299, y: 40.51411 },
    { m: { name: 'Holladay' }, x: -111.82466, y: 40.66884 },
    { m: { name: 'West Jordan' }, x: -111.9391, y: 40.60967 },
    { m: { name: 'East Millcreek' }, x: -111.81049, y: 40.69995 },
    { m: { name: 'Canyon Rim' }, x: -111.82188, y: 40.70661 },
    { m: { name: 'Murray' }, x: -111.88799, y: 40.66689 },
    { m: { name: 'Willard' }, x: -111.83743, y: 40.70634 },
    { m: { name: 'Millcreek' }, x: -111.87549, y: 40.68689 },
    { m: { name: 'Centerfield' }, x: -111.89799, y: 40.69134 },
    { m: { name: 'Taylorsville' }, x: -111.93883, y: 40.66772 },
    { m: { name: 'South Salt Lake' }, x: -111.88827, y: 40.71884 },
    { m: { name: 'Oquirrh' }, x: -112.03383, y: 40.6305 },
    { m: { name: 'Kearns' }, x: -111.99633, y: 40.65995 },
    { m: { name: 'West Valley City' }, x: -112.00105, y: 40.69161 },
    { m: { name: 'Tooele' }, x: -112.29828, y: 40.53078 },
    { m: { name: 'Magna' }, x: -112.10161, y: 40.70911 },
    { m: { name: 'Erda' }, x: -112.30439, y: 40.61272 },
    { m: { name: 'Stansbury park' }, x: -112.29606, y: 40.63772 },
    { m: { name: 'Craig' }, x: -107.54645, y: 40.51525 },
    { m: { name: 'Grantsville' }, x: -112.4644, y: 40.59994 },
    { m: { name: 'Manila' }, x: -109.72265, y: 40.98801 },
    { m: { name: 'Silver Summit' }, x: -111.48775, y: 40.74144 },
    { m: { name: 'Summit Park' }, x: -111.61159, y: 40.74578 },
    { m: { name: 'Coalville' }, x: -111.39936, y: 40.91773 },
    { m: { name: 'Salt Lake City' }, x: -111.89105, y: 40.76078 },
    { m: { name: 'North Salt Lake' }, x: -111.90688, y: 40.84856 },
    { m: { name: 'Woods Cross' }, x: -111.89216, y: 40.87161 },
    { m: { name: 'Bountiful' }, x: -111.88077, y: 40.88939 },
    { m: { name: 'West Bountiful' }, x: -111.90188, y: 40.89383 },
    { m: { name: 'Centerville' }, x: -111.87216, y: 40.918 },
    { m: { name: 'Morgan' }, x: -111.67688, y: 41.03606 },
    { m: { name: 'Farmington' }, x: -111.88744, y: 40.9805 },
    { m: { name: 'Fruit Heights' }, x: -111.90216, y: 41.03217 },
    { m: { name: 'Kaysville' }, x: -111.93855, y: 41.03522 },
  ]);
});

test('sphericalDistance', () => {
  expect(sphericalDistance(0, 0, 0, 0)).toBe(0);
  expect(sphericalDistance(-110, 32, -125, 40)).toBe(1_612_725.6676401352);
});
