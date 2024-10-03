// import { Uint64Cell } from '../src/wasm/uint64';
import { fromLonLat } from '../src/geometry/s2/point';
import { fromS2Point } from '../src/geometry';
// import { fromS2Point } from '../src/geometry/idBig';

// // create two numbers and add them 1_000_000 times
// const startAdd = Bun.nanoseconds();
// for (let i = 0; i < 1_000_000; i++) {
//   const id1 = Uint64Cell.fromNumber(0);
//   const id2 = Uint64Cell.fromNumber(10_000);
//   const _addRes = id1.add(id2);
//   const _subRes = id1.sub(id2);
//   const _mulRes = id1.mul(id2);
//   const _divRes = id1.div(id2);
//   const _toLoHiRes1 = id1.toLoHi();
//   const _toLoHiRes2 = id2.toLoHi();
// }
// const endAdd = Bun.nanoseconds();
// const secondsAdd = (endAdd - startAdd) / 1_000_000_000;
// console.info('Add time: ', secondsAdd);

const start = Bun.nanoseconds();
for (let i = 0; i < 100_000; i++) {
  const lon = getRandomInt(-180, 180);
  const lat = getRandomInt(-90, 90);
  const point = fromLonLat(lon, lat);
  const _cell = fromS2Point(point);
}
const end = Bun.nanoseconds();
const seconds = (end - start) / 1_000_000_000;
console.info('Point time: ', seconds);

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
