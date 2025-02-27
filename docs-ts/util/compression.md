<h1 style="text-align: center;">
  <div align="center">Compression</div>
</h1>

<p align="center">
  <img src="../../assets/badges/compression-file.svg" alt="compression-file-ts">
  <img src="../../assets/badges/compression-gzip.svg" alt="compression-gzip-ts">
  <img src="../../assets/badges/compression-brotli.svg" alt="compression-brotli-ts">
</p>

## Description

Compression and Decompression tools

## Usage

### compressStream

A Browser compatible Gzip compression function

```ts
import { compressStream } from 'gis-tools-ts';

const inputData: Uint8Array = new Uint8Array(100);

// supports gzip, deflate, deflate-raw
// also supports brotli and zstd if using locally
const compressedData = await compressStream(inputData, 'gzip');
```

### decompressStream

A Browser compatible Gzip decompression function

```ts
import { decompressStream } from 'gis-tools-ts';

const inputData: Uint8Array = new Uint8Array(100);

// supports gzip, deflate, deflate-raw
// also supports brotli and zstd if using locally
const decompressedData = await decompressStream(inputData, 'gzip');
```

### Zipped Folder Reader

Iterate over the items in a zip file

```ts
import { iterZipFolder } from 'gis-tools-ts';

const inputData: Uint8Array = new Uint8Array(100);

for await (const item of iterZipFolder(inputData)) {
  const { filename, comment } = item;
  const file = await item.read();
}
```

### LZW Decoder

Decompress the LZW data

```ts
import { lzwDecoder } from 'gis-tools-ts';

const inputData: Uint8Array = new Uint8Array(100);

const decompressedData = lzwDecoder(inputData);
```

## Polyfills

If you are running the code locally like Bun/Deno/NodeJS/etc., you may need to use the "local" polyfill to get access to all compression and decompression tools:

```ts
import 'gis-tools-ts/polyfills/local';
```

On the browser, if you want to keep the bundle size smaller but need support for older browsers:

```ts
import 'gis-tools-ts/polyfills/decompression';
```
