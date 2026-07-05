<h1 style="text-align: center;">
  <div align="center">S2 Tiles Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/s2tiles-file.svg" alt="s2tiles-file-ts">
  <img src="../../assets/badges/s2tiles-gzip.svg" alt="s2tiles-gzip-ts">
  <img src="../../assets/badges/s2tiles-brotli.svg" alt="s2tiles-brotli-ts">
</p>

## Description

S2Tiles is a single-file archive format for tiled data that works for both WM and S2 projections. The goal is to be a "cloud optimized tile store" for vector/raster/grid data.

A V1.0 S2Tiles reader for reading standard WebMercator Tile data and S2 Tile data at the same time.

You can learn more about the [S2Tiles Specification here](https://github.com/Open-S2/s2tiles/blob/master/s2tiles-spec/1.0.0/README.md).

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the S2PMTilesReader.

TODO

## Polyfills

S2Tiles may require the use of `gzip` or `zstd` compression. [This does not have great coverage across browsers](https://caniuse.com/mdn-api_decompressionstream). To alleviate this issue you can take advantage of the polyfills provided:

```ts
import 'gis-tools-ts/polyfills';
// OR specifically use the correct polyfills to reduce bundle size:
import 'gis-tools-ts/polyfills/decompression';
```

## Useful links

- <https://github.com/Open-S2/s2tiles/tree/master/s2tiles-spec>
