<h1 style="text-align: center;">
  <div align="center">Open Street Map (OSM) Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/osm-file.svg" alt="osm-file-ts">
  <img src="../../assets/badges/osm-gzip.svg" alt="osm-gzip-ts">
  <img src="../../assets/badges/osm-brotli.svg" alt="osm-brotli-ts">
</p>

## Description

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the OSMReader.

```ts
import { TagFilter, OSMReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';
import type { OsmReaderOptions } from 'gis-tools-ts';

// build options if desired.
const options: OsmReaderOptions = {
  /** if true, remove nodes that have no tags [Default = true] */
  removeEmptyNodes: true,
  /** If provided, filters of the  */
  tagFilter,
  /** If set to true, nodes will be skipped. [Default = false] */
  skipNodes: false,
  /** If set to true, ways will be skipped. [Default = false] */
  skipWays: false,
  /** If set to true, relations will be skipped. [Default = false] */
  skipRelations: false,
  /**
   * If set to true, ways will be converted to areas if they are closed.
   * NOTE: They are upgraded anyways if the tag "area" is set to "yes".
   * [Default = false]
   */
  upgradeWaysToAreas: true,
  /** If set to true, add a bbox property to each feature */
  addBBox: true,
};

const tagFilter = new TagFilter();
// add a node filter
tagFilter.addFilter('Node', 'foo', 'bar');
// add a way filter
tagFilter.addFilter('Way', 'foo', 'bar');
// add a relation filter
tagFilter.addFilter('Relation', 'foo', 'bar');
// add a filter that effects all types
tagFilter.addFilter('All', 'foo', 'bar');

// create the reader
const reader = new OSMReader(new FileReader('./data.osm.pbf'), options);

// pull out the header
const header = reader.getHeader();
// read the features
for (const feature of reader) {
  console.log(feature);
}

// close the reader when done
reader.close();
```

If the OSM file is large, you will probably run out of memory, so feel free to use the filesystem version or MMap version:

```ts
import { OSMFileReader } from 'gis-tools-ts/file';
const reader = new OSMFileReader('./data.osm.pbf');

// OR: 

import { OSMMMapReader } from 'gis-tools-ts/mmap';
const reader = new OSMMMapReader('./data.osm.pbf');
```

## Useful links

- <https://wiki.openstreetmap.org/wiki/PBF_Format>
- <https://github.com/openstreetmap/pbf/blob/master/OSM-binary.md>
