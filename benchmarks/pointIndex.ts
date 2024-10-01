import KDBush from 'kdbush';
import PointIndex from '../dist/dataStructures/pointIndex';
import { fromLonLat } from '../dist/geometry/s2/point';

const TOTAL_SIZE = 1_000_000;

// ---------------------------------------------- setup

const lls: { lon: number; lat: number; data: { a: number } }[] = [];

for (let i = 0; i < TOTAL_SIZE; i++) {
  const lon = getRandomInt(-180, 180);
  const lat = getRandomInt(-90, 90);
  lls.push({ lon, lat, data: { a: i } });
}

// ---------------------------------------------- BUSH

// const bushBuild = Bun.nanoseconds();
// const bush = new KDBush(TOTAL_SIZE);
// for (let i = 0; i < TOTAL_SIZE; i++) bush.add(lls[i].lon, lls[i].lat);
// bush.finish();
// const bushBuildEnd = Bun.nanoseconds();
// const bushBuildSeconds = (bushBuildEnd - bushBuild) / 1_000_000_000;
// console.info('bush Build time: ', bushBuildSeconds);

// const bushSearchTime = Bun.nanoseconds();
// const _withinSearch = bush.within(lls[0].lon, lls[0].lat, 1);
// const withinSearchEnd = Bun.nanoseconds();
// const withinSearchSeconds = (withinSearchEnd - bushSearchTime) / 1_000_000_000;
// console.info('bush Search time: ', withinSearchSeconds);

// console.info('bush total time: ', bushBuildSeconds + withinSearchSeconds);

// ---------------------------------------------- POINT INDEX
console.info('\n\n');

// const indexBuild = Bun.nanoseconds();
const index = new PointIndex<{ a: number }>();
for (let i = 0; i < TOTAL_SIZE; i++) {
  index.insertLonLat(lls[i].lon, lls[i].lat, { a: i });
}
// const indexSortTime = Bun.nanoseconds();
await index.sort();
// const indexBuildEnd = Bun.nanoseconds();
// const indexSortSeconds = (indexBuildEnd - indexSortTime) / 1_000_000_000;
// const indexBuildSeconds = (indexBuildEnd - indexBuild) / 1_000_000_000;
// console.info('index Build time: ', indexBuildSeconds);
// console.info('index Sort time: ', indexSortSeconds);

// const indexSearchTime = Bun.nanoseconds();
const _withinSearch2 = index.searchRadius(fromLonLat(lls[0].lon, lls[0].lat), 1);
// const withinSearchEnd2 = Bun.nanoseconds();
// const withinSearchSeconds2 = (withinSearchEnd2 - indexSearchTime) / 1_000_000_000;
// console.info('index Search time: ', withinSearchSeconds2);

// console.info('index total time: ', indexBuildSeconds + withinSearchSeconds2);

/**
 * Generate a random whole number between two given values.
 * @param a - The lower bound (inclusive).
 * @param b - The upper bound (inclusive).
 * @returns A random whole number between a and b.
 */
function getRandomInt(a: number, b: number): number {
  if (a > b) {
    throw new Error('The first argument must be less than or equal to the second argument.');
  }

  const min = Math.ceil(a);
  const max = Math.floor(b);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}
