<h1 style="text-align: center;">
  <div align="center">Data Store - Multi Map</div>
</h1>

<p align="center">
  <img src="../../assets/badges/multiMap-file.svg" alt="multiMap-file-ts">
  <img src="../../assets/badges/multiMap-gzip.svg" alt="multiMap-gzip-ts">
  <img src="../../assets/badges/multiMap-brotli.svg" alt="multiMap-brotli-ts">
</p>

## Description

Store data in a key-value data structure where the key is not unique. Works much like an Map in JavaScript but the key may return multiple values instead of one. If the key is unique, be sure to use the kv data store instead.

The key is always an S2CellId for sorting and geospatial queries.

## Usage

```ts
import { MMap } from 'gis-tools-ts';
// Or use the filesystem:
import { FileMultiMap } from 'gis-tools-ts/file';
// Or use the MMap store if using Bun:
import { MMapMultiMap } from 'gis-tools-ts/mmap';

interface Data { name: string };

// NOTE: Add an input string to the constructor to create a FileKV or MMapKV. Otherwise a tmp folder/file is used.
const mm = new MMap<Data>();
// set a key
mm.set(1n, { name: 'test' });
// get a key
const { name } = mm.get(1n); // { name: 'test' }
// check if a key exists
mm.has(1n); // true
// get length of the store
console.log(mm.length); // 2

// iterate over the store
for await (const value of mm) console.log(value);

// clear the store
mm.close();
```
