<h1 style="text-align: center;">
  <div align="center">Tiles Writer</div>
</h1>

<p align="center">
  <img src="../../assets/badges/tileWriter-file.svg" alt="tileWriter-file-ts">
  <img src="../../assets/badges/tileWriter-gzip.svg" alt="tileWriter-gzip-ts">
  <img src="../../assets/badges/tileWriter-brotli.svg" alt="tileWriter-brotli-ts">
</p>

## Description

Write Vector/Raster/Gridded data tiles to a Buffer or Filesystem.

The fundamental concept of storing tiles follows the [TileWriter](https://open-s2.github.io/gis-tools/interfaces/index.TileWriter.html) interface but also supports it's parent interface [TemporalTileWriter](https://open-s2.github.io/gis-tools/interfaces/index.TemporalTileWriter.html).

This design is implemented for the [BufferTileWriter](https://open-s2.github.io/gis-tools/classes/index.BufferTileWriter.html) and [FileTileWriter](https://open-s2.github.io/gis-tools/classes/index.FileTileWriter.html).

## Usage

### Tile File Writer

This is a filesystem Tile writer that organizes data via folders.

```ts
import { FileTileWriter } from 'gis-tools-ts/file';

const tileWriter = new FileTileWriter('./store', 'png');

// store WM tiles
await tileWriter.writeTileWM(0, 0, 0, data);
// store S2 tiles
await tileWriter.writeTileS2(0, 0, 0, 0, data);
// store temportal WM tiles
await tileWriter.writeTemporalTileWM(new Date(), 0, 0, 0, data);
// store temportal S2 tiles
await tileWriter.writeTemporalTileS2(new Date(), 0, 0, 0, 0, data);

// after writing all the tiles, store the metadata
await tileWriter.commit(metadata);
```

### Buffer Tile Writer (For use in the Browser or Testing Suite)

```ts
import { BufferTileWriter } from 'gis-tools-ts/file';

const tileWriter = new BufferTileWriter();

// store WM tiles
await tileWriter.writeTileWM(0, 0, 0, data);
// store S2 tiles
await tileWriter.writeTileS2(0, 0, 0, 0, data);
// store temportal WM tiles
await tileWriter.writeTemporalTileWM(new Date(), 0, 0, 0, data);
// store temportal S2 tiles
await tileWriter.writeTemporalTileS2(new Date(), 0, 0, 0, 0, data);

// after writing all the tiles, store the metadata
await tileWriter.commit(metadata);
```

## Useful links

- <https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html>
- <https://cesium.com/blog/2015/04/07/quadtree-cheatseet/>
