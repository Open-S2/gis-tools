<h1 style="text-align: center;">
  <div align="center">ShapeFile Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/shapefile-file.svg" alt="shapefile-file-ts">
  <img src="../../assets/badges/shapefile-gzip.svg" alt="shapefile-gzip-ts">
  <img src="../../assets/badges/shapefile-brotli.svg" alt="shapefile-brotli-ts">
</p>

## Description

Reads data from an esri Shapefile.

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the ShapefileReader.

## Not Recommended

Just to show how the ShapefileReader works, here is a basic example:

```ts
import { ShapeFileReader, DataBaseFile, Transformer } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const transform = new Transformer();
const dbf = new DataBaseFile(new FileReader('./data.dbf'), 'utf-8');
const reader = new ShapeFileReader(new FileReader('./data.shp'), dbf, transform);

// read all the features
for await (const feature of reader) {
  console.log(feature);
}
```

## Browser or Locally

```ts
import { shapefileFromGzip, shapefileFromURL, LambertConformalConic, EPSG_9974 } from 'gis-tools-ts';

// From a URL:
const reader = await shapefileFromURL('https://example.com/data.zip', [LambertConformalConic], { EPSG_9974 });
// OR from raw ArrayBuffer from a gzip file
const reader = await shapefileFromGzip(arrayBufferInput, [LambertConformalConic], { EPSG_9974 });

for await (const feature of reader) {
  console.log(feature);
}
```

## Filesystem specific usage

Given an input path to all the relavant files, build a Shapefile

```ts
import { LambertConformalConic, EPSG_9974 } from 'gis-tools-ts';
import { shapefileFromPath } from 'gis-tools-ts/file'; // or 'gis-tools-ts/mmap' if you are using Bun

const reader = await shapefileFromPath('path/to/files', [LambertConformalConic], { EPSG_9974 });

for await (const feature of reader) {
  console.log(feature);
}
```

Or build from a Definition object

```ts
import { LambertConformalConic, EPSG_9974 } from 'gis-tools-ts';
import { shapefileFromDefinition } from 'gis-tools-ts/file'; // Or 'gis-tools-ts/mmap' if you are using Bun
import type { Definition } from 'gis-tools-ts';

const def: Definition = {
  shp: 'path/to/file.shp',
  dbf: 'path/to/file.dbf',
  prj: 'path/to/file.prj',
  cpg: 'path/to/file.cpg'
};

const reader = await shapefileFromDefinition(def, [LambertConformalConic], { EPSG_9974 });

for await (const feature of reader) {
  console.log(feature);
}
```

## Useful links

- <https://en.wikipedia.org/wiki/Shapefile>
