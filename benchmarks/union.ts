import { polygonsUnion } from '../src/index.js';
import { polygon, union } from '@turf/turf';

import type { VectorPolygon } from 's2json-spec';
import type {
  FeatureCollection,
  MultiPolygon as TurfMultiPolygon,
  Polygon as TurfPolygon,
} from 'geojson';

console.info('union: ');

const polygonFeatureA: VectorPolygon = [
  [
    { x: -36.843736697711705, y: 26.902507073493283 },
    { x: -38.77733044771159, y: -10.660574687279677 },
    { x: -9.246080447711194, y: 4.565507293900282 },
    { x: -24.890611697711705, y: 10.141965007796856 },
    { x: -8.36717419771125, y: 17.476464485265197 },
    { x: -36.843736697711705, y: 26.902507073493283 },
  ],
];
const turfPolygonA = polygon(
  polygonFeatureA.map((ring) => ring.map((coord) => [coord.x, coord.y])),
);
const polygonFeatureB: VectorPolygon = [
  [
    { x: 25.91016955228889, y: 25.48298173273801 },
    { x: -19.617174197711336, y: 17.81148831664035 },
    { x: -8.191392947711279, y: 12.382961401589753 },
    { x: -17.33201794771142, y: 4.3902626777368 },
    { x: 30.304700802288608, y: -11.523054338551972 },
    { x: 25.91016955228889, y: 25.48298173273801 },
  ],
];
const turfPolygonB = polygon(
  polygonFeatureB.map((ring) => ring.map((coord) => [coord.x, coord.y])),
);

const turfCollection: FeatureCollection<TurfPolygon | TurfMultiPolygon> = {
  type: 'FeatureCollection',
  features: [turfPolygonA, turfPolygonB],
};

// GIS-TOOLS

const startUnion = Bun.nanoseconds();

for (let i = 0; i < 1_000_000; i++) {
  const _res = polygonsUnion([polygonFeatureA, polygonFeatureB]);
}

const endUnion = Bun.nanoseconds();
const secondsUnion = (endUnion - startUnion) / 1_000_000_000;
console.info('GIS-TOOLS Union time: ', secondsUnion);

// TURF

const startTurf = Bun.nanoseconds();

for (let i = 0; i < 1_000_000; i++) {
  const _res = union(turfCollection);
}

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
