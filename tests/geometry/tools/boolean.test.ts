import { cleanPolygons, convert, polygonsUnion, vectorToFlat } from '../../../src';
import { describe, expect, test } from 'bun:test';

import type {
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
    // 'simple-kink',
    // 'simple-kink-2',
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

  test('polygonsUnion - experiments', async () => {
    const folder = 'simple-kink-2';
    const inputPolys = await getInput(folder, false, true);
    const union = polygonsUnion(inputPolys);
    await checkResult(folder, union, false);
  });

  // TODO: leftover polys
  // - issue-60-6
  // - issue-61
  // - issue-66
  // - issue-94
  // - overlapping-clippings
  // - infinitely-thin-polygon
  // - no-self-intersecting-rings-output

  // TODO: chunks-water => After fixing existing bugs this should be perfect

  // TODO: issue-142 => Hole is not building properly. Use vectors to check angles not segments
  // 54.57080496898934, 24.441673716356398

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
    expect(res).toEqual({
      bbox: [2, 2, 4, 3],
      coordinates: [polyB],
      is3D: false,
      type: 'MultiPolygon',
    });
  });
});

// issue-142 logs:

// [ 54.569778932416476, 24.441366817541834 ] 2.0134769880365415e-13 0.10811948670049906 {
//   x: 54.569778932416476,
//   y: 24.441366817541834,
//   z: undefined,
//   m: undefined,
// } {
//   x: 54.56977894449294,
//   y: 24.441074136738756,
//   z: undefined,
//   m: undefined,
// } {
//   x: 54.57000000000001,
//   y: 24.441250624330703,
//   z: undefined,
//   m: undefined,
// } {
//   x: 54.56795534005685,
//   y: 24.442325298422087,
//   z: undefined,
//   m: undefined,
// }

// ------------------------------------------------------------------------

// INTERSECTION 0 0 0 1 2.0134769880365415e-13 [ 54.569778932416476, 24.441366817541834 ]
// SKIP
// INTERSECTION 0 0 6 7 0.22638921520691385 [ 54.571070122708974, 24.441813081258456 ]
// INTERSECTION 0 0 7 8 4.8727279044351385e-12 [ 54.57080496898934, 24.441673716356398 ]
// INTERSECTION 0 0 9 10 0.9999999999949839 [ 54.57074509421786, 24.44164224615234 ]
// INTERSECTION 0 0 10 11 1 [ 54.57000000000001, 24.441250624330703 ]
// INTERSECTION 0 0 11 12 0 [ 54.57000000000001, 24.441250624330703 ]
// SKIP
// INTERSECTION 1 1 3 4 0.4766257516779802 [ 54.571070122708974, 24.441813081258456 ]
// INTERSECTION 1 1 3 4 0.6063066662838654 [ 54.57080496898934, 24.441673716356398 ]
// INTERSECTION 1 1 3 4 0.6355901526068212 [ 54.57074509421786, 24.44164224615234 ]
// INTERSECTION 1 1 3 4 1 [ 54.57000000000001, 24.441250624330703 ]
// INTERSECTION 1 1 4 5 0 [ 54.57000000000001, 24.441250624330703 ]
// SKIP
// INTERSECTION 1 1 4 5 0.10811948670049906 [ 54.569778932416476, 24.441366817541834 ]

// PAIR 0 0 {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } {
//   x: 54.57080502276297,
//   y: 24.441026402022757,
// } 3.1416757254802707 0 0

// PAIR 0 0 {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } {
//   x: 54.57074509421786,
//   y: 24.44164224615234,
// } 2.0547138861302745 1 1

// PAIR 1 1 {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } {
//   x: 54.57080502276297,
//   y: 24.441026402022757,
// } 4.228554492947358 0 0

// PAIR 1 1 {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } {
//   x: 54.57074509421786,
//   y: 24.44164224615234,
// } 3.141592653597362 1 1

// CONNECT! [ 54.571070122708974, 24.441813081258456 ] [
//   [ 54.57080496898934, 24.4416737163564 ]
// ] [ 54.57080496898934, 24.441673716356398 ] [] [ 54.57074509421786, 24.44164224615234 ]

// CONNECT! [ 54.571070122708974, 24.441813081258456 ] [] [ 54.57080496898934, 24.441673716356398 ] [
//   [ 54.57080502276297, 24.441026402022757 ], [ 54.57074511559248, 24.441057889217532 ]
// ] [ 54.57074509421786, 24.44164224615234 ]

// ------------------------------------------------------------------------

// 54.571070122708974, 24.441813081258456

// PAIR 0 0 {
//   x: 54.57114771720956,
//   y: 24.441853864959285,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.1415926535640484 0 0

// PAIR 0 0 {
//   x: 54.57114771720956,
//   y: 24.441853864959285,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } 3.141592653574547 1 1

// PAIR 1 1 {
//   x: 54.57204465994316,
//   y: 24.442325298422087,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.1415926535730576 0 0

// PAIR 1 1 {
//   x: 54.57204465994316,
//   y: 24.442325298422087,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } 3.1415926535835563 1 1

// CONNECT! [ 54.569778932416476, 24.441366817541834 ] [
//   [ 54.56977894449294, 24.441074136738756 ], [ 54.57000000000001, 24.441190327160086 ], [ 54.57084694057397, 24.440745161222193 ], [ 54.57084693745136, 24.44028034081218 ], [ 54.571147760242575, 24.44043845608456 ], [ 54.57114771720956, 24.441853864959285 ]
// ] [ 54.571070122708974, 24.441813081258456 ] [
//   [ 54.57080496898934, 24.4416737163564 ]
// ] [ 54.57080496898934, 24.441673716356398 ]

// CONNECT! [ 54.56795530519258, 24.44447467409078 ] [
//   [ 54.57000000000001, 24.4455493756693 ], [ 54.57204469480743, 24.44447467409078 ], [ 54.57204465994316, 24.442325298422087 ]
// ] [ 54.571070122708974, 24.441813081258456 ] [] [ 54.57080496898934, 24.441673716356398 ]

// ------------------------------------------------------------------------

// START [ 54.569778932416476, 24.441366817541834 ]
// ADD MID [
//   [ 54.56795534005685, 24.442325298422087 ]
// ]
// ADD INT [ 54.56795530519258, 24.44447467409078 ]
// ADD MID [
//   [ 54.57000000000001, 24.4455493756693 ], [ 54.57204469480743, 24.44447467409078 ], [ 54.57204465994316, 24.442325298422087 ]
// ]
// ADD INT [ 54.571070122708974, 24.441813081258456 ]
// ADD MID []
// ADD INT [ 54.57080496898934, 24.441673716356398 ]
// ADD MID [
//   [ 54.57080502276297, 24.441026402022757 ], [ 54.57074511559248, 24.441057889217532 ]
// ]
// ADD INT [ 54.57074509421786, 24.44164224615234 ]
// ADD MID []
// ADD INT [ 54.57000000000001, 24.441250624330703 ]
// ADD MID []
// ADD INT [ 54.569778932416476, 24.441366817541834 ]
// START [ 54.569778932416476, 24.441366817541834 ]
// ADD MID [
//   [ 54.56977894449294, 24.441074136738756 ], [ 54.57000000000001, 24.441190327160086 ], [ 54.57084694057397, 24.440745161222193 ], [ 54.57084693745136, 24.44028034081218 ], [ 54.571147760242575, 24.44043845608456 ], [ 54.57114771720956, 24.441853864959285 ]
// ]
// ADD INT [ 54.571070122708974, 24.441813081258456 ]
// ADD MID [
//   [ 54.57080496898934, 24.4416737163564 ]
// ]
// ADD INT [ 54.57080496898934, 24.441673716356398 ]
// ADD MID []
// ADD INT [ 54.57074509421786, 24.44164224615234 ]
// ADD MID [
//   [ 54.57074509421786, 24.441642246152345 ]
// ]
// ADD INT [ 54.57000000000001, 24.441250624330703 ]
// ADD MID []
// ADD INT [ 54.569778932416476, 24.441366817541834 ]

// PAIR 0 0 {
//   x: 54.57114771720956,
//   y: 24.441853864959285,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.141592653564049 0 0

// PAIR 0 0 {
//   x: 54.57114771720956,
//   y: 24.441853864959285,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } 3.141592653574547 1 1

// PAIR 1 1 {
//   x: 54.57204465994316,
//   y: 24.442325298422087,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.4416737163564,
// } 3.141592653573058 0 0

// PAIR 1 1 {
//   x: 54.57204465994316,
//   y: 24.442325298422087,
// } {
//   x: 54.571070122708974,
//   y: 24.441813081258456,
// } {
//   x: 54.57080496898934,
//   y: 24.441673716356398,
// } 3.1415926535835563 1 1
