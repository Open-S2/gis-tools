<h1 style="text-align: center;">
  <div align="center">Data Store - Vector</div>
</h1>

<p align="center">
  <img src="../../assets/badges/vector-file.svg" alt="vector-file-ts">
  <img src="../../assets/badges/vector-gzip.svg" alt="vector-gzip-ts">
  <img src="../../assets/badges/vector-brotli.svg" alt="vector-brotli-ts">
</p>

## Description

Store data in an array like data structure. Works much like an Array in JavaScript.

The key is always an S2CellId for sorting and geospatial queries.

## Usage

```ts
import { Vector } from 'gis-tools-ts';
// Or use the filesystem:
import { FileVector } from 'gis-tools-ts/file';
// Or use the MMap store if using Bun:
import { MMapVector } from 'gis-tools-ts/mmap';

import type { VectorKey } from 'gis-tools-ts';

interface Data extends VectorKey { name: string };

// NOTE: Add an input string to the constructor to create a FileKV or MMapKV. Otherwise a tmp folder/file is used.
const vec = new Vector<Data>();
// push an entry
vec.push({ cell: 1n, name: 'test' });
vec.push({ cell: 1n, name: 'test2' });
// check if a key exists
vec.has(1n); // true
// get length of the store
console.log(vec.length); // 2

// iterate over the store
for await (const entry of vec) console.log(entry);

// close the store
vec.close();
```
