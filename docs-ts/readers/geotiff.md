<h1 style="text-align: center;">
  <div align="center">GeoTIFF Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/geotiff-file.svg" alt="geotiff-file-ts">
  <img src="../../assets/badges/geotiff-gzip.svg" alt="geotiff-gzip-ts">
  <img src="../../assets/badges/geotiff-brotli.svg" alt="geotiff-brotli-ts">
</p>

## Description

This class reads a GeoTIFF file and returns a list of GeoTIFF images.

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the GeoTIFFReader.

A basic example is as follows

```ts
import { GeoTIFFReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// Or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const geotiffReader = new GeoTIFFReader(new FileReader('example.tif'));
```

If you know the projection and EPSG code to use, you can pass them in as follows:

```ts
import { UniversalTransverseMercator, EPSG_32617, GeoTIFFReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';

// The gridfile is just an example, UTM will never need it.
const geotiffReader = new GeoTIFFReader(
  new FileReader('utf.tif'),
  [UniversalTransverseMercator],
  { EPSG_32617 },
  [{ key: 'BETA2007.gsb', reader: new FileReader('BETA2007.gsb') }]
);
```

If you are not sure what kind of projection to use, use the `ALL_DEFINITIONS` and `EPSG_CODES` constants.

```ts
import { ALL_DEFINITIONS, EPSG_CODES, GeoTIFFReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const fileReader = new FileReader('utf.tif');
const geotiffReader = new GeoTIFFReader(fileReader, ALL_DEFINITIONS, EPSG_CODES);
```

It's recommended though to reduce build size by passing in the needed definitions and EPSG codes. You can refer to [projections](../proj4/projections.md) and [transformer](../proj4/transformer.md)  docs for more information.

## Polyfills

Geotiff may require the use of DataView using `getFloat16`. [This does not have great coverage across browsers](https://caniuse.com/?search=getFloat16). To alleviate this issue you can take advantage of the polyfills provided:

```ts
import 'gis-tools-ts/polyfills';
// OR specifically use the correct polyfills to reduce bundle size:
import 'gis-tools-ts/polyfills/dataview';
```

## Useful links

- <https://www.ogc.org/publications/standard/geotiff/>
- <https://docs.ogc.org/is/19-008r4/19-008r4.html>
