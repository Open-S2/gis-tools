<h1 style="text-align: center;">
  <div align="center">S2 Tiles Writer</div>
</h1>

<p align="center">
  <img src="../../assets/badges/s2tilesWriter-file.svg" alt="s2tilesWriter-file-ts">
  <img src="../../assets/badges/s2tilesWriter-gzip.svg" alt="s2tilesWriter-gzip-ts">
  <img src="../../assets/badges/s2tilesWriter-brotli.svg" alt="s2tilesWriter-brotli-ts">
</p>

## Description

S2Tiles is a single-file archive format for tiled data that works for both WM and S2 projections. The goal is to be a "cloud optimized tile store" for vector/raster/grid data.

A V1.0 S2Tiles reader for reading standard WebMercator Tile data and S2 Tile data at the same time.

You can learn more about the [S2Tiles Specification here](https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md).

## Usage

### Browser Compatible

```ts
import { BufferWriter, S2TilesWriter, Compression } from 'gis-tools-ts';

import type { Metadata } from 'gis-tools-ts';

// Setup the writers
const bufWriter = new BufferWriter();
const writer = new S2TilesWriter(bufWriter, 12, Compression.Gzip);
// example data
const txtEncoder = new TextEncoder();
const str = 'hello world';
const uint8 = txtEncoder.encode(str);
const str2 = 'hello world 2';
const uint8_2 = txtEncoder.encode(str2);
// write data in tile
await writer.writeTileWM(0, 0, 0, uint8);
await writer.writeTileWM(1, 0, 1, uint8);
await writer.writeTileWM(5, 2, 9, uint8_2);
// can also write S2 tiles in the same writer
await writer.writeTileS2(0, 0, 0, 0, uint8);
// finish
await writer.commit({ metadata: true } as unknown as Metadata);
// Get the result Uint8Array
const resultData = bufWriter.commit();
```

### Node/Deno/Bun using the filesystem

```ts
import { S2TilesWriter, Compression } from 'gis-tools-ts';
import { FileWriter } from 'gis-tools-ts/file';

const writer = new S2TilesWriter(new FileWriter('./output.s2tiles'), 12, Compression.Gzip);
// SAME AS ABOVE
```

## Useful links

- <https://github.com/Open-S2/s2tiles/tree/master/s2tiles-spec>
