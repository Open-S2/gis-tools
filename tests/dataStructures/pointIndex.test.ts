import { FileReader } from '../../src/file';
import { JSONReader, PointIndex } from '../../src';
import { expect, test } from 'bun:test';

// HELPER TOOLS
import { fromS2Points } from '../../src/geometry/s1/chordAngle';
import { fromLonLat as pointFromLonLat } from '../../src/geometry/s2/point';

test('point index', async () => {
  const pointIndex = new PointIndex<{ a: number }>();

  pointIndex.insertLonLat({ x: 0, y: 0, m: { a: 0 } });
  pointIndex.insertLonLat({ x: 0, y: 1, m: { a: 1 } });
  pointIndex.insertLonLat({ x: -20, y: 20, m: { a: 2 } });
  pointIndex.insertLonLat({ x: -22, y: 22, m: { a: 3 } });
  pointIndex.insertFaceST(0, 0, 0, { a: 4 });

  await pointIndex.sort();

  const radiusRes = await pointIndex.searchRadius(
    pointFromLonLat({ x: 0, y: 0 }),
    fromS2Points(pointFromLonLat({ x: 0, y: 0 }), pointFromLonLat({ x: 2, y: 2 })),
  );
  expect(radiusRes).toEqual([
    {
      cell: 1152921504606846977n,
      point: { x: 1, y: 0, z: 0, m: { a: 0 } },
    },
    {
      cell: 1153451514845492609n,
      point: { x: 0.9998476951563913, y: 0, z: 0.01745240643728351, m: { a: 1 } },
    },
  ]);

  const allRes = await Array.fromAsync(pointIndex);
  expect(allRes).toEqual([
    {
      cell: 864966976350430661n,
      point: { x: 0.8596699001693257, y: -0.3473291852294986, z: 0.374606593415912, m: { a: 3 } },
    },
    {
      cell: 1056318193755496015n,
      point: { x: 0.8830222215594891, y: -0.3213938048432697, z: 0.3420201433256687, m: { a: 2 } },
    },
    {
      cell: 1152921504606846977n,
      point: { x: 1, y: 0, z: 0, m: { a: 0 } },
    },
    {
      cell: 1153451514845492609n,
      point: { x: 0.9998476951563913, y: 0, z: 0.01745240643728351, m: { a: 1 } },
    },
    {
      cell: 13835058055282163711n,
      point: { x: 1, y: -1, z: -1, m: { a: 4 } },
    },
  ]);
});

test('point index - from reader', async () => {
  const fileReaderPoints = new FileReader(`${__dirname}/../readers/json/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReaderPoints);
  const pointIndex = new PointIndex();
  await pointIndex.insertReader(jsonReader);

  const radiusRes = await pointIndex.searchRadius(
    pointFromLonLat({ x: 144.9584, y: -37.8173 }),
    0.0001,
  );
  expect(radiusRes).toEqual([
    {
      cell: 7698443195519755875n,
      point: {
        x: -0.6467763171329769,
        y: 0.453577844062781,
        z: -0.6131456066639167,
        m: { name: 'Melbourne' },
      },
    },
  ]);
});
