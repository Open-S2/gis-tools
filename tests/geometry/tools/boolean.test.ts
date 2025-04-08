// import { polyUnion } from '../../../src';
// import { describe, expect, test } from 'bun:test';

// import { lstatSync, readdirSync } from 'fs';

// import type { Feature, FeatureCollection, MultiPolygon, Polygon } from '../../../src';

// // test('polyUnion', () => {
// //   const geoms: MultiPolygon = [
// //     [
// //       [
// //         [0, 0],
// //         [1, 0],
// //         [1, 1],
// //         [0, 1],
// //         [0, 0],
// //       ],
// //     ],
// //     [
// //       [
// //         [0.25, 0.25],
// //         [2.75, 0.25],
// //         [2.75, 2.75],
// //         [0.25, 2.75],
// //         [0.25, 0.25],
// //       ],
// //     ],
// //   ];

// //   const res = polyUnion(geoms);
// //   expect(res).toEqual([
// //     [
// //       [
// //         [0, 0],
// //         [1, 0],
// //         [1, 0.25],
// //         [2.75, 0.25],
// //         [2.75, 2.75],
// //         [0.25, 2.75],
// //         [0.25, 1],
// //         [0, 1],
// //         [0, 0],
// //       ],
// //     ],
// //   ]);
// // });

// // test('polyUnion', () => {
// //   const geoms: MultiPolygon = [
// //     [
// //       [
// //         [-1.5250794556600624, -4.254232682994385],
// //         [-0.8777654404938744, -3.7722127395794445],
// //         [-1.461259764023481, -3.426446898836332],
// //         [-0.8048286500535369, -2.889356857184623],
// //         [-1.8988805066701389, -2.7982981578169017],
// //         [-1.5250794556600624, -4.254232682994385],
// //       ],
// //     ],
// //     [
// //       [
// //         [-0.13016333847261308, -2.57062096233345],
// //         [-1.1148100094281403, -2.9713034486426153],
// //         [-0.6407208715607737, -3.4173460550061208],
// //         [-1.2150980962843505, -3.817698277059037],
// //         [-0.5130814882887194, -4.290599736114771],
// //         [-0.13016333847261308, -2.57062096233345],
// //       ],
// //     ],
// //   ];

// //   const res = polyUnion(geoms);
// //   expect(res).toEqual([
// //     [
// //       [
// //         [-1.8988805066701389, -2.7982981578169017],
// //         [-1.5250794556600624, -4.254232682994385],
// //         [-1.070057564896894, -3.9154023457863882],
// //         [-0.5130814882887194, -4.290599736114771],
// //         [-0.13016333847261308, -2.57062096233345],
// //         [-0.8949912674362106, -2.88185255963163],
// //         [-1.8988805066701389, -2.7982981578169017],
// //       ],
// //       [
// //         [-1.461259764023481, -3.426446898836332],
// //         [-1.0172116469004515, -3.063128011834442],
// //         [-0.6407208715607737, -3.4173460550061208],
// //         [-1.024820586120268, -3.6850711102392735],
// //         [-1.461259764023481, -3.426446898836332],
// //       ],
// //     ],
// //   ]);
// // });

// // test('base case - union', async () => {
// //   const fc: FeatureCollection = await Bun.file(
// //     `${__dirname}/fixtures/multipoly-and-square/args.geojson`,
// //   ).json();
// //   const comparison = await Bun.file(
// //     `${__dirname}/fixtures/multipoly-and-square/union.geojson`,
// //   ).json();

// //   const featureCoords = fc.features
// //     .filter((f) => f.geometry.type === 'Polygon' || f.geometry.type === 'MultiPolygon')
// //     .map((f) => f.geometry.coordinates) as (MultiPolygon | Polygon)[];
// //   const unionRes = polyUnion(...featureCoords);

// //   expect(unionRes).toEqual(comparison.geometry.coordinates);
// // });

// test('almost-colinear-segments-but-not-2 - union', async () => {
//   const fc: FeatureCollection = await Bun.file(
//     `${__dirname}/fixtures/almost-colinear-segments-but-not-2/args.geojson`,
//   ).json();
//   const comparison = await Bun.file(
//     `${__dirname}/fixtures/almost-colinear-segments-but-not-2/union.geojson`,
//   ).json();

//   const featureCoords = fc.features
//     .filter((f) => f.geometry.type === 'Polygon' || f.geometry.type === 'MultiPolygon')
//     .map((f) => f.geometry.coordinates) as (MultiPolygon | Polygon)[];
//   const unionRes = polyUnion(...featureCoords);

//   expect(unionRes).toEqual(comparison.geometry.coordinates);
// });

// // describe('union - all', () => {
// //   const fixtures = readdirSync(`${__dirname}/fixtures`);

// //   // handle all fixtures with "union.geojson" or "all.geojson"
// //   for (const fixture of fixtures) {
// //     // if fixture is not a directory, skip
// //     const stats = lstatSync(`${__dirname}/fixtures/${fixture}`);
// //     if (!stats.isDirectory()) continue;
// //     // grab the args.geojson
// //     const files = readdirSync(`${__dirname}/fixtures/${fixture}`);
// //     if (!files.includes('args.geojson')) continue;
// //     // find the union file
// //     const unionFile = files.find((f) => f === 'union.geojson' || f === 'all.geojson');
// //     if (unionFile === undefined) continue;

// //     test(`${fixture}`, async () => {
// //       // grab the args
// //       const args: FeatureCollection = await Bun.file(
// //         `${__dirname}/fixtures/${fixture}/args.geojson`,
// //       ).json();
// //       const argsCoords = args.features
// //         .filter((f) => f.geometry.type === 'Polygon' || f.geometry.type === 'MultiPolygon')
// //         .map((f) => f.geometry.coordinates) as (MultiPolygon | Polygon)[];
// //       // grab the union
// //       const unionCmp: Feature = await Bun.file(
// //         `${__dirname}/fixtures/${fixture}/${unionFile}`,
// //       ).json();
// //       const unionCoords = unionCmp.geometry.coordinates as MultiPolygon;

// //       // run and compare
// //       const unionRes = polyUnion(...argsCoords);
// //       expect(unionRes).toEqual(unionCoords);
// //     });
// //   }
// // });
