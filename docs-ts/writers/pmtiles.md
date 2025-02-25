<h1 style="text-align: center;">
  <div align="center">(S2) PMTiles Writer</div>
</h1>

<p align="center">
  <img src="../../assets/badges/pmtilesWriter-file.svg" alt="pmtilesWriter-file-ts">
  <img src="../../assets/badges/pmtilesWriter-gzip.svg" alt="pmtilesWriter-gzip-ts">
  <img src="../../assets/badges/pmtilesWriter-brotli.svg" alt="pmtilesWriter-brotli-ts">
</p>

## Description

Writes data via the [S2-PMTiles specification](https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md).

A Modified TypeScript implementation of the [PMTiles](https://github.com/protomaps/PMTiles) library. It is backwards compatible but offers support for the S2 Projection.

## Usage

### Browser Compatible

```ts
import { TileType, BufferWriter, S2PMTilesWriter, Compression } from 'gis-tools-ts';
import type { Metadata } from 'gis-tools-ts';

// Setup the writers
const bufWriter = new BufferWriter();
const writer = new S2PMTilesWriter(bufWriter, TileType.Unknown, Compression.Gzip);
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
// finish
await writer.commit({ metadata: true } as unknown as Metadata);
// Get the result Uint8Array
const resultData = bufWriter.commit();
```

### Node/Deno/Bun using the filesystem

```ts
import { S2PMTilesWriter, TileType } from 'gis-tools-ts';
import { FileWriter } from 'gis-tools-ts/file';

const writer = new S2PMTilesWriter(new FileWriter('./output.pmtiles'), TileType.Pbf);
// SAME AS ABOVE
```

## Useful links

- <https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md>
- <https://github.com/protomaps/PMTiles>
