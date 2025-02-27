<h1 style="text-align: center;">
  <div align="center">Data Structure - Cache</div>
</h1>

<p align="center">
  <img src="../../assets/badges/cache-file.svg" alt="cache-file-ts">
  <img src="../../assets/badges/cache-gzip.svg" alt="cache-gzip-ts">
  <img src="../../assets/badges/cache-brotli.svg" alt="cache-brotli-ts">
</p>

## Description

A cache of values with a max size to ensure that too much old data is not stored. The deletion system uses the FIFO policy

## Usage

```ts
import { Cache } from 'gis-tools-ts';

const onDelete = (key: string, value: string) => {
  console.log(`Deleted key ${key} with value ${value}`);
};
const cache = new Cache<string, string>(10, onDelete);
cache.set('key', 'value');
console.log(cache.get('key')); // 'value'
cache.delete('key');
```
