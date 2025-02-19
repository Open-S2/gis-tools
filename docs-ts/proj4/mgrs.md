<h1 style="text-align: center;">
  <div align="center">Military Grid Reference System</div>
</h1>

<p align="center">
  <img src="../../assets/badges/mgrs-file.svg" alt="mgrs-file-ts">
  <img src="../../assets/badges/mgrs-gzip.svg" alt="mgrs-gzip-ts">
  <img src="../../assets/badges/mgrs-brotli.svg" alt="mgrs-brotli-ts">
</p>

## Description

The Military Grid Reference System (MGRS) is the geocoordinate standard used by NATO militaries for locating points on Earth.

## Usage

```ts
import { mgrsForward, mgrsToPoint } from 'gis-tools-ts';

// convert WGS84 ellipsoid longitude and latitude to MGRS
const mgrs = mgrsForward({ x: 60.8, y: -132.2 });
// convert MGRS to WGS84 ellipsoid longitude and latitude
const { x: lon, y: lat } = mgrsToPoint(mgrs);
```

## Useful links

- <https://en.wikipedia.org/wiki/Military_Grid_Reference_System>
- <https://earth-info.nga.mil/#geotrans>
