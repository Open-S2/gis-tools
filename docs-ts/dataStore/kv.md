<h1 style="text-align: center;">
  <div align="center">Data Store - Key-Value (KV)</div>
</h1>

<p align="center">
  <img src="../../assets/badges/kv-file.svg" alt="kv-file-ts">
  <img src="../../assets/badges/kv-gzip.svg" alt="kv-gzip-ts">
  <img src="../../assets/badges/kv-brotli.svg" alt="kv-brotli-ts">
</p>

## Description

Store data in a key-value data structure. Works much like an Map in JavaScript. If the key is not unique, be sure to use the multiMap data store instead.

The key is always an S2CellId for sorting and geospatial queries.

## Usage

```ts
import { KV } from 'gis-tools-ts';
// Or use the filesystem:
import { FileKV } from 'gis-tools-ts/file';
// Or use the MMapKV if using Bun:
import { MMapKV } from 'gis-tools-ts/mmap';

interface Data { name: string };

// NOTE: Add an input string to the constructor to create a FileKV or MMapKV. Otherwise a tmp folder/file is used.
const kv = new KV<Data>();
// set a key
kv.set(1n, { name: 'test' });
// get a key
const { name } = kv.get(1n); // { name: 'test' }
// check if a key exists
kv.has(1n); // true
// get length of the store
console.log(kv.length); // 1

// iterate over the store
for await (const value of kv) console.log(value);

// clear the store
kv.close();
```
