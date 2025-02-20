<h1 style="text-align: center;">
  <div align="center">Tile Readers</div>
</h1>

<p align="center">
  <img src="../../assets/badges/tileReader-file.svg" alt="tileReader-file-ts">
  <img src="../../assets/badges/tileReader-gzip.svg" alt="tileReader-gzip-ts">
  <img src="../../assets/badges/tileReader-brotli.svg" alt="tileReader-brotli-ts">
</p>

## Description

Tiles are a quadtree storage structure for vector, raster, and grid GIS data.

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

### Raster

Read an entire archive of raster tiles, where the max zoom data is iterated upon

Supports reading either RGB(A) data and/or RGB(A) encoded elevation data.

```ts
import { RasterTilesReader, convertTerrariumElevationData } from 'gis-tools-ts';
// can also import convertMapboxElevationData

// creates a reader for a tile set treating the max zoom as 3 instead of the metadata's max zoom
const reader = new RasterTilesReader('https://example.com/satellite-data', 3);
// example of reading in an elevation dataset
const reader2 = new RasterTilesReader(
  'https://example.com/terrariumData',
  -1, // if -1 is set you want to access the max zoom data
  convertTerrariumElevationData
);

// grab the metadata
const metadata = await reader.getMetadata();

// grab a WM tile
const tile1 = await reader.getTile(0, 0, 0);
// or if it's an S2 tile spec
const tile2 = await reader.getTileS2(0, 0, 0, 0);

// grab all the max zoom tiles:
for await (const tile of reader) {
  console.log(tile);
}

// grab the features from tile1:
for await (const feature of tile1) {
  console.log(feature);
}
```

## Useful links

- <https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html>
- <https://cesium.com/blog/2015/04/07/quadtree-cheatseet/>
