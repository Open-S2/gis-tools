import { cleanPolygons, convert, polygonsUnion, vectorToFlat } from '../../../src';
import { describe, expect, test } from 'bun:test';

import type {
  BBox,
  FeatureCollection,
  MValue,
  Properties,
  VectorMultiPolygon,
  VectorMultiPolygonGeometry,
  VectorPolygon,
} from 's2json-spec';

/**
 * Reads a geojson file into a VectorMultiPolygon
 * @param folder - the name of the geojson folder to read from
 * @param cleanWGS84 - if the input needs to be cleaned for WGS84
 * @param clean - clean the input
 * @returns - the VectorMultiPolygon
 */
async function getInput(
  folder: string,
  cleanWGS84 = false,
  clean = true,
): Promise<VectorMultiPolygonGeometry> {
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

  if (!clean) return { type: 'MultiPolygon', coordinates: res, is3D: false };
  return (
    cleanPolygons(res, true, cleanWGS84) ?? { type: 'MultiPolygon', coordinates: [], is3D: false }
  );
}

/**
 * @param folder - the name of the geojson folder
 * @param data - the VectorMultiPolygonGeometry union result
 * @param writeResultFirst - whether to write the result to a file first
 */
async function checkResult<D extends MValue = Properties>(
  folder: string,
  data?: VectorMultiPolygonGeometry<D>,
  writeResultFirst: boolean = false,
): Promise<void> {
  if (data === undefined) {
    data = { type: 'MultiPolygon', coordinates: [], is3D: false };
  }
  const geometry = vectorToFlat(data);
  const feature = { type: 'Feature', properties: {}, geometry };
  const featureCollection = { type: 'FeatureCollection', features: [feature] };

  if (writeResultFirst) {
    await Bun.write(
      `${__dirname}/fixtures/${folder}/union.geojson`,
      JSON.stringify(featureCollection, null, 2),
    );
  }

  const storedResult = await Bun.file(`${__dirname}/fixtures/${folder}/union.geojson`).json();
  expect(featureCollection).toEqual(storedResult);
}

describe('polygonsUnion', () => {
  [
    'almost-colinear-segments-but-not',
    'almost-colinear-segments-but-not-2',
    'almost-parrallel-segments',
    'almost-parrallel-segments-2',
    'almost-parrallel-segments-3',
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
    // 'infinitely-thin-polygon',
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
    // 'issue-60-6',
    'issue-60-7',
    'issue-60-8',
    // 'issue-61',
    'issue-61-2',
    'issue-62',
    'issue-62-2',
    // 'issue-66',
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
    // 'issue-94',
    'issue-105',
    'issue-115',
    'issue-118',
    'issue-118-2',
    'issue-124',
    'issue-139',
    'issue-140',
    'issue-141',
    'issue-142',
    'issue-142-outers',
    'issue-142-simple',
    'issue-turf-1094',
    'maybe-colinear-sides',
    'multipoly-and-square',
    'multipoly-with-hole-and-square',
    'multipolys-with-disjoint-polys',
    'nearly-vertical-far-right',
    'no-bbox-overlap',
    // 'no-self-intersecting-rings-output',
    'non-zero-rule-not-even-odd',
    'overlap-edges',
    // 'overlap-loop',
    // 'overlapping-clippings',
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
    test(`polygonsUnion - ${folder}`, async () => {
      const inputPolys = await getInput(folder);
      const union = polygonsUnion(inputPolys)!;
      await checkResult(folder, union, false);
    });
  });

  // test('polygonsUnion - experiments', async () => {
  //   // const folder = 'split-prev-segment';
  //   // const folder = 'chunks-water-4';
  //   const folder = 'issue-118-2';
  //   const inputPolys = await getInput(folder, false, true);
  //   const union = polygonsUnion(inputPolys);
  //   await checkResult(folder, union, false);
  // });

  // TODO: leftover polys
  // - issue-60-6
  // - issue-61
  // - issue-66
  // - issue-94
  // - overlapping-clippings
  // - infinitely-thin-polygon
  // - no-self-intersecting-rings-output

  // TODO: Last bug -> what ever is plauging chunks-water

  test('polygonsUnion - fully empty', (): void => {
    const polyA: VectorPolygon = [[]];
    const polyB: VectorPolygon = [[]];

    const res = polygonsUnion([polyA, polyB]);
    expect(res).toBeUndefined();
  });

  test('polygonsUnion - one empty', (): void => {
    const polyA: VectorPolygon = [[]];
    const polyB: VectorPolygon = [
      [
        { x: 2, y: 2 },
        { x: 4, y: 2 },
        { x: 4, y: 3 },
        { x: 2, y: 3 },
        { x: 2, y: 2 },
      ],
    ];

    const polys = cleanPolygons([polyA, polyB], true)!;
    const res = polygonsUnion(polys);
    const bbox: BBox = [2, 2, 4, 3];
    expect(res).toEqual({ bbox, coordinates: [polyB], is3D: false, type: 'MultiPolygon' });
  });
});
