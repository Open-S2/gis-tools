import { Database } from 'bun:sqlite';
// import { S2FileStore } from '../src/dataStore/file';
import { S2MMapStore } from '../src/dataStore/mmap';
// import { open } from 'lmdb';
import tmp from 'tmp';

tmp.setGracefulCleanup();

const dir = tmp.dirSync({ prefix: 'store_benchmarks' });
// const TEST_SIZE = 1_000_000;
const TEST_SIZE = 100_000;

const mmapStore = new S2MMapStore<{ a: number }>(`${dir.name}/mmap`, false, false);
// const fileStore = new S2FileStore<{ a: number }>(`${dir.name}/file`, false, false);

const mmapAddStart = Bun.nanoseconds();
for (let i = 0; i < TEST_SIZE; i++) {
  const rand = getRandomInt(0, TEST_SIZE);
  mmapStore.set(rand, { a: rand });
}
mmapStore.set(22, { a: 22 });
const mmapAddEnd = Bun.nanoseconds();
const mmapAddSeconds = (mmapAddEnd - mmapAddStart) / 1_000_000_000;
console.info('mmap Add time: ', mmapAddSeconds);

// lets sort:
const mmapSortStart = Bun.nanoseconds();
await mmapStore.switchToReadState();
const mmapSortEnd = Bun.nanoseconds();
const mmapSortSeconds = (mmapSortEnd - mmapSortStart) / 1_000_000_000;
console.info('mmap Sort time: ', mmapSortSeconds);

// query
const mmapQueryStart = Bun.nanoseconds();
const mmapRes = mmapStore.get(22, 1);
const mmapQueryEnd = Bun.nanoseconds();
const mmapQuerySeconds = (mmapQueryEnd - mmapQueryStart) / 1_000_000_000;
console.info('mmap Query time: ', mmapQuerySeconds, mmapRes);

console.info('mmap total time: ', mmapAddSeconds + mmapSortSeconds + mmapQuerySeconds);

/// ----------------------------------------------

// const fileAddStart = Bun.nanoseconds();
// for (let i = 0; i < 10_000; i++) fileStore.set(i, { a: i });
// const fileAddEnd = Bun.nanoseconds();
// const fileAddSeconds = (fileAddEnd - fileAddStart) / 1_000_000_000;
// console.info('file Add time: ', fileAddSeconds);

// // lets sort:
// const fileSortStart = Bun.nanoseconds();
// fileStore.has(0);
// const fileSortEnd = Bun.nanoseconds();
// const fileSortSeconds = (fileSortEnd - fileSortStart) / 1_000_000_000;
// console.info('file Sort time: ', fileSortSeconds);

// console.info('file total time: ', fileAddSeconds + fileSortSeconds);

/// ----------------------------------------------

// const myDB = open({ path: `${dir.name}/lmdb` });
// const lmdbAddStart = Bun.nanoseconds();
// for (let i = 0; i < TEST_SIZE; i++) {
//   const rand = getRandomInt(0, TEST_SIZE);
//   await myDB.put(String(rand), { a: rand });
// }
// const lmdbAddEnd = Bun.nanoseconds();
// const lmdbAddSeconds = (lmdbAddEnd - lmdbAddStart) / 1_000_000_000;
// console.info('lmdb Add time: ', lmdbAddSeconds);

// // Let's perform a simple lookup to simulate a query
// const lmdbQueryStart = Bun.nanoseconds();
// await myDB.get('22');
// const lmdbQueryEnd = Bun.nanoseconds();
// const lmdbQuerySeconds = (lmdbQueryEnd - lmdbQueryStart) / 1_000_000_000;
// console.info('lmdb Query time: ', lmdbQuerySeconds);

// console.info('lmdb total time: ', lmdbAddSeconds + lmdbQuerySeconds);

// await myDB.close();

/// ----------------------------------------------

const db = new Database(`${dir.name}/sqlite.db`);
db.exec(`
  CREATE TABLE IF NOT EXISTS data (
    hi INTEGER NOT NULL,
    lo INTEGER NOT NULL,
    value BLOB NOT NULL
  );
  CREATE INDEX IF NOT EXISTS idx_hi_lo ON data (hi, lo);
`);

// Adding data as BLOB to SQLite
const sqliteAddStart = Bun.nanoseconds();
const insert = db.prepare('INSERT INTO data (hi, lo, value) VALUES (?, ?, ?)');
for (let i = 0; i < TEST_SIZE; i++) {
  const rand = getRandomInt(0, TEST_SIZE);
  insert.run(0, rand, Buffer.from(JSON.stringify({ a: rand }))); // Storing Buffer as BLOB
}
const sqliteAddEnd = Bun.nanoseconds();
const sqliteAddSeconds = (sqliteAddEnd - sqliteAddStart) / 1_000_000_000;
console.info('SQLite Add time: ', sqliteAddSeconds);

// Let's perform a simple lookup to simulate a query
const sqliteQueryStart = Bun.nanoseconds();
const res = db.prepare('SELECT * FROM data WHERE hi = ? AND lo = ?').get(0, 22); // Query the first entry
const sqliteQueryEnd = Bun.nanoseconds();
const sqliteQuerySeconds = (sqliteQueryEnd - sqliteQueryStart) / 1_000_000_000;
console.info('SQLite Query time: ', sqliteQuerySeconds, res);

console.info('SQLite total time: ', sqliteAddSeconds + sqliteQuerySeconds);

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
