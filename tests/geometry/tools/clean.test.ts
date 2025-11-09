import { cleanPolygons, convert, vectorToFlat } from '../../../src';
import { describe, expect, test } from 'bun:test';

import type {
  Feature,
  FeatureCollection,
  MValue,
  Properties,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
} from 's2json-spec';

/**
 * Reads a geojson file into a VectorMultiPolygon
 * @param folder - the name of the geojson folder to read in
 * @returns - the VectorMultiPolygon
 */
async function getInput(folder: string): Promise<VectorMultiPolygon> {
  const featureCollection = (await Bun.file(
    `${__dirname}/fixtures/${folder}/args.geojson`,
  ).json()) as FeatureCollection;
  const vectorFeatures = convert('WG', featureCollection);

  const res: VectorMultiPolygon = [];

  for (const feature of vectorFeatures) {
    const { type, coordinates } = feature.geometry;
    if (type === 'Polygon') {
      res.push(coordinates);
    } else if (type === 'MultiPolygon') {
      res.push(...coordinates);
    }
  }

  return res;
}

/**
 * Reads a geojson file into a VectorMultiPolygon
 * @param folder - the name of the geojson folder to read in
 * @param geo - the resultant VectorMultiPolygonGeometry
 * @param writeResult - whether to write the result to a file first
 */
async function checkResult<D extends MValue = Properties>(
  folder: string,
  geo: VectorMultiPolygonGeometry<D>,
  writeResult: boolean,
): Promise<void> {
  const geometry = vectorToFlat(geo);
  const feature: Feature = { type: 'Feature', properties: {}, geometry };
  const featureCollection: FeatureCollection = { type: 'FeatureCollection', features: [feature] };

  if (writeResult) {
    await Bun.write(
      `${__dirname}/fixtures/${folder}/cleaned.geojson`,
      JSON.stringify(featureCollection, null, 2),
    );
  }

  const storedResult = await Bun.file(`${__dirname}/fixtures/${folder}/cleaned.geojson`).json();
  expect(featureCollection).toEqual(storedResult);
}

describe('cleanPolygons', () => {
  [
    'almost-colinear-segments-but-not',
    'almost-colinear-segments-but-not-2',
    'almost-parrallel-segments',
    'almost-parrallel-segments-2',
    'almost-parrallel-segments-3',
    'bathymetry_2000',
    'chunks-water-2',
    'chunks-water-3',
    'clean-multipoly-with-polys-overlapping',
    'clean-multipoly-with-polys-touching',
    'clean-poly-with-backward-ring-winding-order',
    'clean-poly-with-repeated-and-extra-points',
    'collapsed-edges-removed',
    'disjoint-union',
    'dont-consume-prev-segment-1',
    'dont-consume-prev-segment-2',
    'dont-consume-prev-segment-3',
    'double-overlap',
    'empty-multipoly',
    'high-coincidence',
    'hole-from-outers-bug',
    'hole-interacts-outer',
    'infinitely-thin-polygon',
    'intersection-after-remove-1',
    'intersection-after-remove-2',
    'island-in-hole-4x',
    'issue-1',
    'issue-36',
    'issue-37',
    'issue-38',
    'issue-44',
    'issue-60',
    'issue-60-2',
    'issue-60-3',
    'issue-60-4',
    'issue-60-5',
    'issue-60-6',
    'issue-60-7',
    'issue-60-8',
    'issue-61',
    'issue-61-2',
    'issue-62',
    'issue-62-2',
    'issue-66',
    'issue-68',
    'issue-68-1',
    'issue-75',
    'issue-78',
    'issue-79',
    'issue-83',
    'issue-85',
    'issue-86',
    'issue-90',
    'issue-91',
    'issue-93',
    'issue-94',
    'issue-105',
    'issue-115',
    'issue-118',
    'issue-124',
    'issue-139',
    'issue-140',
    'issue-141',
    'issue-142',
    'issue-turf-1094',
    'maybe-colinear-sides',
    'multipoly-and-square',
    'multipoly-with-hole-and-square',
    'multipolys-with-disjoint-polys',
    'nearly-vertical-far-right',
    'no-bbox-overlap',
    'no-self-intersecting-rings-output',
    'non-zero-rule-not-even-odd',
    'overlap-edges',
    'overlap-loop',
    'overlapping-clippings',
    'poly-and-square',
    'poly-with-hole-and-square',
    'polygon-and-trapezoid',
    'right-sweep-events-change-ordering',
    'rings-with-no-area',
    'saw-and-cheese',
    'self-intersects-but-doesnt-cross-1',
    'self-intersects-but-doesnt-cross-2',
    'simple-kink',
    'simple-kink-2',
    'split-almost-vertical-segment',
    'split-prev-segment',
    'three-triangles',
    'touching-boxes',
    'triple-coincident-segments',
    'two-disjoint-polygons',
    'two-overlapping-triangles',
    'two-overlapping-triangles-start-inside',
    'union-same-shape-multiple-times',
    'vertical-intersection-rounding-error',
    'vertical-segment-upon-split',
    'windmill-3-polys',
    'windmill-3-polys-2',
    'windmill-3-polys-3',
    'windmill-4-blades',
  ].forEach((folder) => {
    test(`cleanPolygons - ${folder}`, async () => {
      const input = await getInput(folder);
      const cleaned = cleanPolygons(input)!;

      await checkResult(folder, cleaned, false);
    });
  });

  test('cleanPolygons - experiments', async () => {
    const folder = 'hole-interacts-outer';
    const input = await getInput(folder);
    const cleaned = cleanPolygons(input)!;

    await checkResult(folder, cleaned, false);
  });
  // 'chunks-water-simple'

  // start: Point(0.0, 1.0)
  // add mid: []
  // add int: Point(0.07466666666666667, 1.0213333333333334)
  // add mid: []
  // add int: Point(0.0, 1.04)
  // add mid: []
  // add int: Point(0.0, 1.0)
  // start: Point(0.08, 1.02)
  // add mid: []
  // add int: Point(0.07466666666666667, 1.0213333333333334)
  // add mid: []
  // add int: Point(0.14, 1.04)
  // add mid: []
  // add int: Point(0.08, 1.02)
});

// TODO: Hole interacts with outer ring
// - clean-poly-with-interior-ring-splitting-exterior
// - clean-poly-with-interior-ring-touching-exterior
// - clean-poly-with-interior-rings-overlapping
// - coincident-with-invalid-segments
// - multipoly-with-self-crossing-rings
