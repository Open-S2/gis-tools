<h1 style="text-align: center;">
  <div align="center">(S2) PM Tiles Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/pmtiles-file.svg" alt="pmtiles-file-ts">
  <img src="../../assets/badges/pmtiles-gzip.svg" alt="pmtiles-gzip-ts">
  <img src="../../assets/badges/pmtiles-brotli.svg" alt="pmtiles-brotli-ts">
</p>

## Description

A V3.0 PMTiles reader for reading standard WebMercator Tile data and V1.0 S2 Tile data.

A Modified implementation of the PMTiles library. It is backwards compatible but offers support for the S2 Projection.

You can learn more about the [S2PMTiles Specification here](https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md).

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the S2PMTilesReader.

```ts
import { S2PMTilesReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new S2PMTilesReader(new FileReader('./data.pmtiles'));

// pull out the header
const header = reader.getHeader();

// get the metadata
const metadata = await reader.getMetadata();

// S2 specific functions
const hasTile = await reader.hasTileS2(0, 0, 0, 0);
const tile = await reader.getTileS2(0, 0, 0, 0);

// WM functions
const hasTile = await reader.hasTile(0, 0, 0);
const tile = await reader.getTile(0, 0, 0);
```

## Polyfills

PMTiles may require the use of `gzip` or `zstd` compression. [This does not have great coverage across browsers](https://caniuse.com/mdn-api_decompressionstream). To alleviate this issue you can take advantage of the polyfills provided:

```ts
import 'gis-tools-ts/polyfills';
// OR specifically use the correct polyfills to reduce bundle size:
import 'gis-tools-ts/polyfills/decompression';
```

## Useful links

- <https://github.com/Open-S2/s2-pmtiles>
- <https://github.com/Open-S2/s2-pmtiles/blob/master/s2-pmtiles-spec/1.0.0/README.md>
- <https://github.com/protomaps/PMTiles>
- <https://github.com/protomaps/PMTiles/blob/main/spec/v3/spec.md>
