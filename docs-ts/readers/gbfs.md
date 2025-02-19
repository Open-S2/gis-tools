<h1 style="text-align: center;">
  <div align="center">General Bikeshare Feed Specification (GBFS) Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/gbfs-file.svg" alt="gbfs-file-ts">
  <img src="../../assets/badges/gbfs-gzip.svg" alt="gbfs-gzip-ts">
  <img src="../../assets/badges/gbfs-brotli.svg" alt="gbfs-brotli-ts">
</p>

## Description

Given a link to a GBFS feed, build the appropriate reader for the feed. The versions of GBFS reader classes this data could be (1, 2, or 3).

Implements the `FeatureIterator` interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

### Fetch the list of available services

Fetches the list of GBFS systems from the github CSV file

```ts
import { fetchGTFSSystems } from 'gis-tools-ts';
import type { GBFSSystem } from 'gis-tools-ts';

const systems: GBFSSystem[] = await fetchGTFSSystems();

console.log(systems);
```

### Fetch a specific service

```ts
import { buildGBFSReader } from 'gis-tools-ts';
import type { GBFSReader } from 'gis-tools-ts';

const reader: GBFSReader = await buildGBFSReader('https://gbfs.urbansharing.com/gbfs/gbfs.json');
// read the features
for await (const feature of reader) {
  // do something with the feature
}
```

## Useful links

- <https://github.com/MobilityData/gbfs>
- <https://github.com/MobilityData/gbfs-json-schema/tree/master/v3.0>
