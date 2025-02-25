<h1 style="text-align: center;">
  <div align="center">Polyfills</div>
</h1>

<p align="center">
  <img src="../../assets/badges/polyfills-file.svg" alt="polyfills-file-ts">
  <img src="../../assets/badges/polyfills-gzip.svg" alt="polyfills-gzip-ts">
  <img src="../../assets/badges/polyfills-brotli.svg" alt="polyfills-brotli-ts">
</p>

## Description

## Usage

To take advantage of all polyfills, simply add the `polyfills.ts` file to your project.

```ts
import 'gis-tools-ts/polyfills';
```

If you'd like to include only a subset of the polyfills, you can import them individually.

### Dataview

![Dataview File Badge][dataviewFileBadge] ![Dataview Gzip Badge][dataviewGzipBadge] ![Dataview Brotli Badge][dataviewBrotliBadge]

[dataviewFileBadge]: ../../assets/badges/dataview-file.svg
[dataviewGzipBadge]: ../../assets/badges/dataview-gzip.svg
[dataviewBrotliBadge]: ../../assets/badges/dataview-brotli.svg

Some projects including the geotiff class require the use of [getFloat16](https://caniuse.com/mdn-javascript_builtins_dataview_getfloat16), which requires the `dataview` polyfill for most browsers.

### Decompression

![Dec File Badge][decFileBadge] ![Dec Gzip Badge][decGzipBadge] ![Dec Brotli Badge][decBrotliBadge]

[decFileBadge]: ../../assets/badges/decompression-file.svg
[decGzipBadge]: ../../assets/badges/decompression-gzip.svg
[decBrotliBadge]: ../../assets/badges/decompression-brotli.svg

Some projects may need to support older browsers that by default do not support the [DecompressionStream](https://caniuse.com/mdn-api_decompressionstream). This polyfill provides support for those browsers but also in the future will ensure access to `zstd` and `brotli` decompression.
