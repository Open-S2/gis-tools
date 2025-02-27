<h1 style="text-align: center;">
  <div align="center">Data Store - External Sort</div>
</h1>

<p align="center">
  <img src="../../assets/badges/externalSort-file.svg" alt="externalSort-file-ts">
  <img src="../../assets/badges/externalSort-gzip.svg" alt="externalSort-gzip-ts">
  <img src="../../assets/badges/externalSort-brotli.svg" alt="externalSort-brotli-ts">
</p>

## Description

External Sort is a methodology of sorting key-value pairs where the key is a uint64. It can handle a single key-value store or a collection of key-value stores that are merged into a single key-value sorted store.

The whole project revolves around the idea that points and by extension lines/polygons can be represented as a singular uint64. What's more is that the S2 Geometry system has a "Locality of Reference" mechanic built in, that if you sort the points by their S2 CellId, you can utilize a lot of powerful search/find/range/cluster queries that are very fast and efficient.

If you checkout the `benchmarks/stores.ts` file, you can see how necessary this concept is. Just storing 100,000 points, you see normal DBs like SQL take close to a minute on my M2 Macbook Pro, whereas this External Sort mechanic takes a fraction of a second to sort the same data. Whats worse is that the SQL DB is CPU expensive and also has a logarithmic performance on this sort of data, so at 1,000,000 points, the SQL DB is taking over 7 minutes to store them. Compare this to the External Sort method which takes 4 seconds on 1,000,000 to store and sort the same data.

Now the argument can be that there are SQL DBs that incorporate geospatial indexing and it does improve performance sure, but these kinds of DBs lock you into non-flexible data models and aren't easy to port from language to language. Yet at the end of the day, Geospatial data is always just a collection of points and attributes that are associated with them.

It cannot be understated how important this section of code is. Sitting at less than 400 lines of code, including comments and imports, is for this toolkit and geospatial data management.

What makes this whole project so amazing is this `S2CellId` + `ExternalSort` trick works for both WGS84/Mercator and S2 Geometry data sets equally.

In practice, you will never use this tool directly. It is used internally by some of the data structures provided in this library. But it's important to understand how it works and document the process/purpose of it.

## Usage

This is more trivial example, whereas in practice you assume millions of points.

```ts
import { S2FileStore } from '../../../src/file';
import { S2MMapStore, externalSort } from '../../../src/mmap';

// Lets create a temp folder
import tmp from 'tmp';
tmp.setGracefulCleanup();
const dir = tmp.dirSync({ prefix: 'externalSort_single' });

// let's use a mmap store for convenience and track the name of the file
const name = `${dir.name}/sort-multi-threaded`;
const store = new S2MMapStore<{ a: number }>(name);
store.set(0, { a: 1 });
store.set(987_678_987_330, { a: 2 });
store.set(1, { a: 3 });
store.close();

// lets create a second store
const name2 = `${dir.name}/sort-multi-threaded-2`;
const store2 = new S2MMapStore<{ a: number }>(name2);
store2.set(5_005, { a: 4 });
store2.set(9_807, { a: 5 });
store2.set(456, { a: 6 });
store2.close();

// now let's sort all the data:
const output = `${dir.name}/output`;
const maxHeap = 100_000; // Size of a chunk of data to parse at a time
const threadCount = 8;
await externalSort([name, name2], output, maxHeap, threadCount);

// we can now use this data:
const storeSorted = new S2FileStore<{ a: number }>(name, { isSorted: true });
const data = await Array.fromAsync(storeSorted.entries());

// we now know this data is sorted:
expect(data).toStrictEqual([
  { key: 0n, value: { a: 1 } },
  { key: 1n, value: { a: 3 } },
  { key: 456n, value: { a: 6 } },
  { key: 5_005n, value: { a: 4 } },
  { key: 9_807n, value: { a: 5 } },
  { key: 987_678_987_330n, value: { a: 2 } },
]);
```
