import { Database } from 'bun:sqlite';
import { S2FileStore } from '../src/dataStore/file';
import { S2MMapStore } from '../src/dataStore/mmap';
// import { open } from 'lmdb';
import tmp from 'tmp';

tmp.setGracefulCleanup();

const dir = tmp.dirSync({ prefix: 'store_benchmarks' });
const TEST_SIZE = 1_000_000;

/// ----------------------------------------------

const mmapStore = new S2MMapStore<{ a: number }>(`${dir.name}/mmap`);

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
await mmapStore.sort();
const mmapSortEnd = Bun.nanoseconds();
const mmapSortSeconds = (mmapSortEnd - mmapSortStart) / 1_000_000_000;
console.info('mmap Sort time: ', mmapSortSeconds);

// query
const mmapQueryStart = Bun.nanoseconds();
const mmapRes = await mmapStore.get(22, 1);
const mmapQueryEnd = Bun.nanoseconds();
const mmapQuerySeconds = (mmapQueryEnd - mmapQueryStart) / 1_000_000_000;
console.info('mmap Query time: ', mmapQuerySeconds, mmapRes);

console.info('mmap total time: ', mmapAddSeconds + mmapSortSeconds + mmapQuerySeconds);

/// ----------------------------------------------

const fileStore = new S2FileStore<{ a: number }>(`${dir.name}/file`);

const fileAddStart = Bun.nanoseconds();
for (let i = 0; i < TEST_SIZE; i++) {
  const rand = getRandomInt(0, TEST_SIZE);
  fileStore.set(rand, { a: rand });
}
fileStore.set(22, { a: 22 });
const fileAddEnd = Bun.nanoseconds();
const fileAddSeconds = (fileAddEnd - fileAddStart) / 1_000_000_000;
console.info('file Add time: ', fileAddSeconds);

// lets sort:
const fileSortStart = Bun.nanoseconds();
await fileStore.sort();
const fileSortEnd = Bun.nanoseconds();
const fileSortSeconds = (fileSortEnd - fileSortStart) / 1_000_000_000;
console.info('file Sort time: ', fileSortSeconds);

// query
const fileQueryStart = Bun.nanoseconds();
const fileRes = await fileStore.get(22, 1);
const fileQueryEnd = Bun.nanoseconds();
const fileQuerySeconds = (fileQueryEnd - fileQueryStart) / 1_000_000_000;
console.info('file Query time: ', fileQuerySeconds, fileRes);

console.info('file total time: ', fileAddSeconds + fileSortSeconds + fileQuerySeconds);

fileStore.close();

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
    id BIGINT NOT NULL,
    value BLOB NOT NULL
  );
  CREATE INDEX IF NOT EXISTS idx ON data (id);
`);

// Adding data as BLOB to SQLite
const sqliteAddStart = Bun.nanoseconds();
const alreadyUsed = new Set<number>();
const insert = db.prepare('INSERT INTO data (id, value) VALUES (?, ?)');
for (let i = 0; i < TEST_SIZE; i++) {
  let rand: number;
  do {
    rand = getRandomInt(0, 1_000_000_000);
  } while (alreadyUsed.has(rand));
  alreadyUsed.add(rand);
  insert.run(rand, Buffer.from(JSON.stringify({ a: rand }))); // Storing Buffer as BLOB
}
// add 22
if (!alreadyUsed.has(22)) insert.run(22, Buffer.from(JSON.stringify({ a: 22 }))); // Storing Buffer as BLOB
const sqliteAddEnd = Bun.nanoseconds();
const sqliteAddSeconds = (sqliteAddEnd - sqliteAddStart) / 1_000_000_000;
console.info('SQLite Add time: ', sqliteAddSeconds);

// Let's perform a simple lookup to simulate a query
const sqliteQueryStart = Bun.nanoseconds();
const res = db.prepare('SELECT * FROM data WHERE id = ?').get(22); // Query the first entry
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
