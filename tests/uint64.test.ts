// import { Uint64, Uint64LoHi } from '../src/wasm/uint64';

// import { expect, test } from 'bun:test';

// test('S2Cell creation', () => {
//   const pointA = Uint64(0);
//   const pointB = Uint64(10_000);
//   const pointC = Uint64(20_000);
//   const pointD = Uint64LoHi(4_000, 0);
//   expect(pointA.compare(pointB)).toEqual(-1);
//   expect(pointC.compare(pointB)).toEqual(1);
//   expect(pointA.compare(pointA)).toEqual(0);
//   expect(pointC.compare(pointD)).toEqual(1);

//   expect(pointA.toLoHi()).toEqual({ lo: 0, hi: 0 });
//   expect(pointB.toLoHi()).toEqual({ lo: 10_000, hi: 0 });
//   expect(pointC.toLoHi()).toEqual({ lo: 20_000, hi: 0 });
//   expect(pointD.toLoHi()).toEqual({ lo: 4_000, hi: 0 });

//   const big = 13835058055282163712n;
//   const lo = Number(big & 0xffffffffn);
//   const hi = Number((big >> 32n) & 0xffffffffn);
//   expect(lo).toEqual(0);
//   expect(hi).toEqual(3221225472);
//   const cell = Uint64LoHi(lo, hi);
//   expect(cell.toBigInt()).toEqual(big);

//   const big2 = 2305843009213693951n;
//   const lo2 = Number(big2 & 0xffffffffn);
//   const hi2 = Number((big2 >> 32n) & 0xffffffffn);
//   expect(lo2).toEqual(4294967295);
//   expect(hi2).toEqual(536870911);
//   const cell2 = Uint64LoHi(lo2, hi2);
//   expect(cell2.toBigInt()).toEqual(big2);

//   const big3 = 1229782938247303424n;
//   const lo3 = Number(big3 & 0xffffffffn);
//   const hi3 = Number((big3 >> 32n) & 0xffffffffn);
//   expect(lo3).toEqual(286331136);
//   expect(hi3).toEqual(286331153);
//   const cell3 = Uint64LoHi(lo3, hi3);
//   expect(cell3.toBigInt()).toEqual(big3);
// });
