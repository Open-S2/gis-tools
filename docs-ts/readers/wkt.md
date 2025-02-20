<h1 style="text-align: center;">
  <div align="center">WKT Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/wkt-file.svg" alt="wkt-file-ts">
  <img src="../../assets/badges/wkt-gzip.svg" alt="wkt-gzip-ts">
  <img src="../../assets/badges/wkt-brotli.svg" alt="wkt-brotli-ts">
</p>

## Description

Parse a collection of WKT geometries from string(s).

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the WKTReader.

### WKT Geometry Reader

```ts
import { WKTGeometryReader } from 'gis-tools-ts';

const reader = new WKTGeometryReader('POINT(4 6) GEOMETRYCOLLECTION(POINT(1 2), LINESTRING(3 4,5 6))');

// read the features
for await (const feature of reader) {
  console.log(feature);
}
```

### WKT Geometry Parser

Parse individual geometries from a WKT string into a VectorGeometry

```ts
import { parseWKTGeometry } from 'gis-tools-ts';

const geometry = parseWKTGeometry('POINT (1 2)');
```

### WKT Object

If you just want to parse a WKT Object, use the `parseWKTObject` function

```ts
import { parseWKTObject } from 'gis-tools-ts';

const object = parseWKTObject('PROJCS["NZGD49 / New Zealand Map...');
```

### WKT Projection Parser

If you're playing with a PROJ WKT string, feel free to use the `parseWKTProjection` function

```ts
import { isWKTProjection, parseWKTProjection } from 'gis-tools-ts';

const projWKTStr = 'PROJCS["NZGD49 / New Zealand Map...'

if (isWKTProjection(projWKTStr)) {
  const projectionObject = parseWKTProjection(projWKTStr);
}
```

## Useful links

- <https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry>
