<h1 style="text-align: center;">
  <div align="center">GPS Exchange Format (GPX) Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/gpx-file.svg" alt="gpx-file-ts">
  <img src="../../assets/badges/gpx-gzip.svg" alt="gpx-gzip-ts">
  <img src="../../assets/badges/gpx-brotli.svg" alt="gpx-brotli-ts">
</p>

## Description

The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.

GPX (the GPS Exchange Format) is a light-weight XML data format for the interchange of GPS data (waypoints, routes, and tracks) between applications and Web services on the Internet.

This implementation is designed to parse the complete [GPX 1.1 Schema](https://www.topografix.com/GPX/1/1/gpx.xsd).

Implements the `FeatureIterator` interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the GPXReader.

```ts
import { GPXReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';

const gpxReader = new GPXReader(new FileReader('./example.gpx'));

for await (const feature of gpxReader) {
  console.log(feature);
}
```

## Useful links

- <https://www.topografix.com/gpx.asp>
