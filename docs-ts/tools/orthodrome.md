<h1 style="text-align: center;">
  <div align="center">Orthodrome</div>
</h1>

<p align="center">
  <img src="../../assets/badges/orthodrome-file.svg" alt="orthodrome-file-ts">
  <img src="../../assets/badges/orthodrome-gzip.svg" alt="orthodrome-gzip-ts">
  <img src="../../assets/badges/orthodrome-brotli.svg" alt="orthodrome-brotli-ts">
</p>

## Description

Represents an orthodrome, which is the shortest path between two points on a sphere. you can [Learn more here](http://www.movable-type.co.uk/scripts/latlong.html).

## Usage

```ts
import { Orthodrome } from 'gis-tools-ts'

// starting at lon-lat (-60, -40) and ending at (20, 10)
const orthodrome = new Orthodrome(-60, -40, 20, 10);
// OR create from VectorPoints
const orthodrome = Orthodrome.fromPoints({ x: -60, y: -40 }, { x: 20, y: 10 });
// { x: -39.13793657428956, y: -33.72852197561652 }
const intermediatePoint = orthodrome.intermediatePoint(0.2);
// Distance in KM: 1.5514126949321814
const distance = orthodrome.distanceTo();
// get the bearing of the first point to the second in degrees
const bearing = orthodrome.bearing();
```

## Useful links

- <http://www.movable-type.co.uk/scripts/latlong.html>
