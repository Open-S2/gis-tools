import { FileReader } from '../../src/file';
import { JSONReader, PointIndex } from '../../src';
import { expect, test } from 'bun:test';

// HELPER TOOLS
import { fromS2Points } from '../../src/geometry/s1/chordAngle';
import { fromLonLat as pointFromLonLat } from '../../src/geometry/s2/point';

test('point index', async () => {
  const pointIndex = new PointIndex<{ a: number }>();

  pointIndex.insertLonLat(0, 0, { a: 0 });
  pointIndex.insertLonLat(0, 1, { a: 1 });
  pointIndex.insertLonLat(-20, 20, { a: 2 });
  pointIndex.insertLonLat(-22, 22, { a: 3 });
  pointIndex.insertFaceST(0, 0, 0, { a: 4 });

  await pointIndex.sort();

  const radiusRes = await pointIndex.searchRadius(
    pointFromLonLat(0, 0),
    fromS2Points(pointFromLonLat(0, 0), pointFromLonLat(2, 2)),
  );
  expect(radiusRes).toEqual([
    {
      cell: { high: 268435456, low: 1 },
      data: { a: 0 },
      point: [1, 0, 0],
    },
    {
      cell: { high: 268558858, low: 2684384641 },
      data: { a: 1 },
      point: [0.9998476951563913, 0, 0.01745240643728351],
    },
  ]);

  const allRes = await Array.fromAsync(pointIndex);
  expect(allRes).toEqual([
    {
      cell: { high: 201390817, low: 3620709829 },
      data: { a: 3 },
      point: [0.8596699001693257, -0.3473291852294986, 0.374606593415912],
    },
    {
      cell: { high: 245943244, low: 4103347791 },
      data: { a: 2 },
      point: [0.8830222215594891, -0.3213938048432697, 0.3420201433256687],
    },
    {
      cell: { high: 268435456, low: 1 },
      data: { a: 0 },
      point: [1, 0, 0],
    },
    {
      cell: { high: 268558858, low: 2684384641 },
      data: { a: 1 },
      point: [0.9998476951563913, 0, 0.01745240643728351],
    },
    {
      cell: { high: 3221225471, low: 4294967295 },
      data: { a: 4 },
      point: [1, -1, -1],
    },
  ]);
});

test('point index - from reader', async () => {
  const fileReaderPoints = new FileReader(`${__dirname}/../readers/json/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReaderPoints);
  const pointIndex = new PointIndex();
  await pointIndex.insertReader(jsonReader);

  const radiusRes = await pointIndex.searchRadius(pointFromLonLat(144.9584, -37.8173), 0.0001);
  expect(radiusRes).toEqual([
    {
      cell: { high: 1792433484, low: 1484416611 },
      data: { name: 'Melbourne' },
      point: [-0.6467763171329769, 0.453577844062781, -0.6131456066639167],
    },
  ]);
});
