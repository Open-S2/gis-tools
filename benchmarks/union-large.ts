import { union } from '@turf/turf';
import { cleanPolygons, convert, polygonsUnion } from '../src/index.js';

import type { FeatureCollection, VectorMultiPolygon } from 's2json-spec';
import type {
  FeatureCollection as TurfFeatureCollection,
  MultiPolygon as TurfMultiPolygon,
  Polygon as TurfPolygon,
} from 'geojson';

console.info('union: ');

const featureCollection = (await Bun.file(
  `${__dirname}/../tests/geometry/tools/fixtures/chunks-water/args.geojson`,
).json()) as TurfFeatureCollection<TurfPolygon | TurfMultiPolygon>;
const vectorFeatures = convert('WG', featureCollection as FeatureCollection);

const vectorPolys: VectorMultiPolygon = [];

for (const feature of vectorFeatures) {
  const { type, coordinates } = feature.geometry;
  if (type === 'Polygon') {
    vectorPolys.push(coordinates);
  } else if (type === 'MultiPolygon') {
    vectorPolys.push(...coordinates);
  }
}
const cleanedPolys = cleanPolygons(vectorPolys, true)!;

// GIS-TOOLS

const startUnion = Bun.nanoseconds();

const _res = polygonsUnion(cleanedPolys);

const endUnion = Bun.nanoseconds();
const secondsUnion = (endUnion - startUnion) / 1_000_000_000;
console.info('GIS-TOOLS Union time: ', secondsUnion);

// TURF

const startTurf = Bun.nanoseconds();

const _turfRes = union(featureCollection);

const endTurf = Bun.nanoseconds();
const secondsTurf = (endTurf - startTurf) / 1_000_000_000;
console.info('TURF Union time: ', secondsTurf);

//  bun run ./benchmarks/union.ts
// union:
// GIS-TOOLS Union time:  6.474554583
// TURF Union time:  333.604353166

// GIS-TOOLS IS ~51.53x faster

//  cargo bench --bench union
// polygons_union_test/union
//                         time:   [4.3458 s 4.7888 s 5.2385 s]
