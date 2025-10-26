import { deKinkPolygon } from '../src/index.js';
import { polygon, unkinkPolygon } from '@turf/turf';

import type { VectorPolygon } from 's2json-spec';

console.info('dekink: ');

const polygonFeature: VectorPolygon = [
  [
    { x: 8.094854051549703, y: 44.067038922182604 },
    { x: 27.45169791493106, y: 34.31013538862004 },
    { x: 31.238906496896703, y: 25.572928139998595 },
    { x: 26.610096007827508, y: 22.88716015007573 },
    { x: 25.978894577499233, y: 18.957601207155236 },
    { x: 32.08050840400031, y: 17.157354229920827 },
    { x: 38.8133236608289, y: 20.541732106259843 },
    { x: 40.496527475035236, y: 28.199781765371043 },
    { x: 7.463652621221485, y: 25.00221485407819 },
    { x: 25.347693147171753, y: 4.999693002409302 },
    { x: -7.4747812298659255, y: -36.777396059815665 },
    { x: 27.662098391706394, y: -40.233822107102995 },
    { x: 28.92450125236215, y: -14.406933337995738 },
    { x: 4.097244992807987, y: -34.38206769619466 },
    { x: 62.79897801327945, y: -31.19907851930298 },
    { x: 86.57423188895399, y: 16.55327251195662 },
    { x: 54.38295894224376, y: 12.685928855764459 },
    { x: 73.73980280562509, y: -3.197906810124664 },
    { x: 81.52462044633336, y: 36.369487623534425 },
    { x: 54.80375989579596, y: 56.70904723358515 },
    { x: 8.094854051549703, y: 44.067038922182604 },
  ],
];
const turfPolygon = polygon(polygonFeature.map((ring) => ring.map((coord) => [coord.x, coord.y])));

// GIS-TOOLS

const startDekink = Bun.nanoseconds();

for (let i = 0; i < 1_000_000; i++) {
  deKinkPolygon(polygonFeature);
}

const endDekink = Bun.nanoseconds();
const secondsDekink = (endDekink - startDekink) / 1_000_000_000;
console.info('GIS-TOOLS Dekink time: ', secondsDekink);

// TURF

const startTurf = Bun.nanoseconds();

for (let i = 0; i < 1_000_000; i++) {
  unkinkPolygon(turfPolygon);
}

const endTurf = Bun.nanoseconds();
const secondsTurf = (endTurf - startTurf) / 1_000_000_000;
console.info('TURF Dekink time: ', secondsTurf);

//  bun run ./benchmarks/dekink.ts
// dekink:
// GIS-TOOLS Dekink time:  4.632328
// TURF Dekink time:      15.094146583

//  cargo bench --bench dekink
// dekink_polygon_test/dekink
//                         time:   [2.5963 s 2.6372 s 2.6844 s]
