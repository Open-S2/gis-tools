<h1 style="text-align: center;">
  <div align="center">Polylabel</div>
</h1>

<p align="center">
  <img src="../../assets/badges/polylabel-file.svg" alt="polylabel-file-ts">
  <img src="../../assets/badges/polylabel-gzip.svg" alt="polylabel-gzip-ts">
  <img src="../../assets/badges/polylabel-brotli.svg" alt="polylabel-brotli-ts">
</p>

## Description

A fast algorithm for finding polygon pole of inaccessibility, the most distant internal point from the polygon outline (not to be confused with centroid), implemented as a JavaScript library. Useful for optimal placement of a text label on a polygon.

## Usage

```ts
import { polylabels, polylabel } from 'gis-tools-ts'
import type { VectorMultiPolygon } from 'gis-tools-ts'

// for vector multipolygons:
const vectorGeometry: VectorMultiPolygon = [];
const polylabelHighPrecision = polylabels(vectorGeometry, 1);

// for a single polygon:
const vectorGeometry: VectorPolygon = [];
const polylabelHighPrecision = polylabel(vectorGeometry, 1);
```

## Useful links

- <https://sites.google.com/site/polesofinaccessibility/>
