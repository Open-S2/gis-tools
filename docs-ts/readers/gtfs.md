<h1 style="text-align: center;">
  <div align="center">The General Transit Feed Specification (GTFS) Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/gtfs-file.svg" alt="gtfs-file-ts">
  <img src="../../assets/badges/gtfs-gzip.svg" alt="gtfs-gzip-ts">
  <img src="../../assets/badges/gtfs-brotli.svg" alt="gtfs-brotli-ts">
</p>

## Description

The General Transit Feed Specification defines a common format for public transportation schedules and associated geographic information. GTFS "feeds" let public transit agencies publish their transit data and developers write applications that consume that data in an interoperable way.

## Usage

### Static

Schedule class that pulls in all of the GTFS schedule files and parses them into a single object

```ts
import { buildGTFSSchedule } from 'gis-tools-ts';

const schedule = await buildGTFSSchedule(gzipData);

for await (const feature of schedule) {
    console.log(feature);
}
```

### Realtime

The input is a Uint8Array that has encoded protobuffer messages.

```ts
import { GTFSRealtimeReader } from 'gis-tools-ts';
 *
const gtfsRealtimeReader = new GTFSRealtimeReader(data);
const { header, entities } = gtfsRealtimeReader;
for (const entity of entities) {
  console.log(entity);
}
```

## Useful links

- <https://mobilitydatabase.org>
- <https://developers.google.com/transit/gtfs/examples/overview>
- <https://gtfs.org/documentation/schedule/reference/#tripstxt>
- <https://mobilitydata.github.io/>
- <https://www.transit.land>
