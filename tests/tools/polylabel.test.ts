import polylabel from '../../src/tools/polylabel';
import { expect, test } from 'bun:test';

import type { Polygon, VectorPolygon } from '../../src/geometry';

test('finds pole of inaccessibility for water1 and precision 1', async () => {
  const water1: Polygon = await Bun.file(`${__dirname}/fixtures/water1.json`).json();
  const vectorWater1 = convertGeometry(water1);
  const polylabelHighPrecision = polylabel(vectorWater1, 1);
  expect(polylabelHighPrecision).toEqual({
    x: 3865.85009765625,
    y: 2124.87841796875,
    m: { distance: 288.8493574779127 },
  });

  const polylabelLowPrecision = polylabel(vectorWater1, 50);
  expect(polylabelLowPrecision).toEqual({
    x: 3854.296875,
    y: 2123.828125,
    m: { distance: 278.5795872381558 },
  });
});

test('finds pole of inaccessibility for water2 and default precision 1', async () => {
  const water2: Polygon = await Bun.file(`${__dirname}/fixtures/water2.json`).json();
  const vectorWater2 = convertGeometry(water2);
  const p = polylabel(vectorWater2);
  expect(p).toEqual({
    x: 3263.5,
    y: 3263.5,
    m: { distance: 960.5 },
  });
});

test('works on degenerate polygons', () => {
  const p1 = polylabel(
    convertGeometry([
      [
        [0, 0],
        [1, 0],
        [2, 0],
        [0, 0],
      ],
    ]),
  );
  expect(p1).toEqual({ x: 0, y: 0, m: { distance: 0 } });

  const p2 = polylabel(
    convertGeometry([
      [
        [0, 0],
        [1, 0],
        [1, 1],
        [1, 0],
        [0, 0],
      ],
    ]),
  );
  expect(p2).toEqual({ x: 0, y: 0, m: { distance: 0 } });
});

/**
 * @param polygon - the polygon to convert
 * @returns - the vector polygon
 */
function convertGeometry(polygon: Polygon): VectorPolygon {
  return polygon.map((line) => line.map((point) => ({ x: point[0], y: point[1] })));
}
