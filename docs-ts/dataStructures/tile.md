<h1 style="text-align: center;">
  <div align="center">Data Structure - Tile</div>
</h1>

<p align="center">
  <img src="../../assets/badges/dataTile-file.svg" alt="dataTile-file-ts">
  <img src="../../assets/badges/dataTile-gzip.svg" alt="dataTile-gzip-ts">
  <img src="../../assets/badges/dataTile-brotli.svg" alt="dataTile-brotli-ts">
</p>

## Description

Tile Class to contain the tile information for splitting or simplifying.

## Usage

### Tile Class

```ts
import { Tile } from 'gis-tools-ts';
 // create a tile
const tile = new Tile(id);
// add a feature
tile.addFeature(feature);
 // transform the geometry to be relative to the tile
tile.transform();
```

If you have some kind reader you can use the `addReader` method to build the tile

```ts
import { Tile, JSONReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// create a tile
const tile = new Tile(id);
// add a reader
const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
const jsonReader = new JSONReader(fileReader);
await tile.addReader(jsonReader);
// then transform
tile.transform();
```

### Tile Store

TileStore Class is a tile-lookup system that splits and simplifies a collection of features/vector-features as needed for each tile request

```ts
import { TileStore } from 'gis-tools-ts';

const tileStore = new TileStore(data, {
 projection: 'WG', // projection (WG or S2)
 minzoom: 0, // minzoom
 maxzoom: 9, // maxzoom
 indexMaxzoom: 4, // max zoom to index data on creation
 tolerance: 3, // tolerance to simplify data. 3 Seems to be a good balance
 buffer: 0.0625 // buffer around the tile before cutting and simplifying geometry. Necessary to properly render lines and polygons.
 buildBBox: false // whether to build the bounding box for each tile feature
});

// get a tile
const tile = tileStore.getTile(id);
```

## Useful links

- <https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html>
- <https://cesium.com/blog/2015/04/07/quadtree-cheatseet/>
