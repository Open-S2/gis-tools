<h1 style="text-align: center;">
  <div align="center">(GEO|S2)JSON Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/json-file.svg" alt="json-file-ts">
  <img src="../../assets/badges/json-gzip.svg" alt="json-gzip-ts">
  <img src="../../assets/badges/json-brotli.svg" alt="json-brotli-ts">
</p>

## Description

Parsed various (Geo|S2)JSON file formats

All JSON Readers here implement the `FeatureIterator` interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the appropriate json reader.

### Buffer JSON Reader

This is the default case where you already have a (S2|Geo)JSON object or string

```ts
import { BufferJSONReader } from 'gis-tools-ts';

// parse a string
const reader = new BufferJSONReader(`{ type: 'FeatureCollection', features: [...] }`);
// OR parse a JSON object
const reader = new BufferJSONReader({ type: 'FeatureCollection', features: [...] });
// OR fetch a JSON file as a string
const reader = new BufferJSONReader(
  await fetch('example.com/data.json').then(async (res) => await res.text())
);

// read the features
for await (const feature of reader) {
  console.log(feature);
}
```

### JSON Reader

This is for large files where you may want to parse JSON individually from a file to save on memory

```ts
import { JSONReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new JSONReader(new FileReader('./data.geojsonld'));

// read the features
for await (const feature of reader) {
  console.log(feature);
}
```

### New Line Delimited JSON Reader

Often extremely large JSON is stored in a newline-delimited format. It looks much like:

```json
{"type":"Feature","properties":{"name":"Melbourne"},"geometry":{"type":"Point","coordinates":[144.9584,-37.8173]}}
{"type":"Feature","properties":{"name":"Canberra"},"geometry":{"type":"Point","coordinates":[149.1009,-35.3039]}}
{"type":"Feature","properties":{"name":"Sydney"},"geometry":{"type":"Point","coordinates":[151.2144,-33.8766]}}
```

There is no `FeatureCollection` but rather each feature is separated by a newline. It is often compressed with gzip or brotli.

```ts
import { NewLineDelimitedJSONReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new NewLineDelimitedJSONReader(new FileReader('./data.geojsonld'));

// read the features
for await (const feature of reader) {
  console.log(feature);
}
```

### Text Sequence JSON Reader

Much like the New Line Delimited JSON Reader, but instead of using a newline as a seperator it uses `␞`. The example data is:

```json
␞{ "type": "Feature", "geometry": {"type": "Point",
"coordinates": [102.0, 0.5]}, "properties": {"prop0": "value0"}}
␞{ "type": "Feature", "geometry": {"type": "LineString",
"coordinates": [[102.0, 0.0], [103.0, 1.0], [104.0, 0.0],
[105.0, 1.0]]}, "properties": {"prop0": "value0", "prop1": 0.0}}
␞{ "type": "Feature", "geometry": {"type": "Polygon",
"coordinates": [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0],
[100.0, 1.0], [100.0, 0.0]]]}, "properties":
{"prop0": "value0", "prop1": {"this": "that"}}}
```

It's nice in how it ensures the JSON is more legible and doesn't suffer from the same issues as New Line Delimited JSON Reader like terminated strings.

```ts
import { SequenceJSONReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new SequenceJSONReader(new FileReader('./data.geojsonseq'));

// read the features
for await (const feature of reader) {
  console.log(feature);
}
```

## Useful links

- <https://datatracker.ietf.org/doc/html/rfc8259>
- <https://en.wikipedia.org/wiki/JSON>
- <https://database.guide/what-is-ndjson/>
- <https://datatracker.ietf.org/doc/html/rfc7464>
- <https://datatracker.ietf.org/doc/html/rfc8142>
- <https://github.com/geojson/geojson-text-sequences?tab=readme-ov-file>
