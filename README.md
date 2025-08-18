<h1 style="text-align: center;">
  <div align="center">gis-tools</div>
</h1>

<p align="center">
  <a href="https://img.shields.io/github/actions/workflow/status/Open-S2/gis-tools/test.yml?logo=github">
    <img src="https://img.shields.io/github/actions/workflow/status/Open-S2/gis-tools/test.yml?logo=github" alt="GitHub Actions Workflow Status">
  </a>
  <a href="https://npmjs.org/package/gis-tools-ts">
    <img src="https://img.shields.io/npm/v/gis-tools-ts.svg?logo=npm&logoColor=white" alt="npm">
  </a>
  <a href="https://crates.io/crates/gis-tools">
    <img src="https://img.shields.io/crates/v/gis-tools.svg?logo=rust&logoColor=white" alt="crate">
  </a>
  <a href="https://www.npmjs.com/package/gis-tools-ts">
    <img src="https://img.shields.io/npm/dm/gis-tools-ts.svg" alt="downloads">
  </a>
  <a href="https://open-s2.github.io/gis-tools/">
    <img src="https://img.shields.io/badge/docs-typescript-yellow.svg" alt="docs-ts">
  </a>
  <a href="https://docs.rs/gis-tools">
    <img src="https://img.shields.io/badge/docs-rust-yellow.svg" alt="docs-rust">
  </a>
  <a href="https://coveralls.io/github/Open-S2/gis-tools?branch=master">
    <img src="https://coveralls.io/repos/github/Open-S2/gis-tools/badge.svg?branch=master" alt="code-coverage">
  </a>
  <a href="https://discord.opens2.com">
    <img src="https://img.shields.io/discord/953563031701426206?logo=discord&logoColor=white" alt="Discord">
  </a>
</p>

## About

A collection of geospatial tools primarily designed for WGS84, Web Mercator, and S2.

## Features

Notable features of GIS-Tools are:

* 🔗 Lightweight, fast, and memory efficient. `no_std` builds for Rust. Tree-shaking for Typescript.
* 🗺️ Full toolkit support for WGS84, Web Mercator, and S2 projections.
* 🌱 A large list of projections can be converted to/from one of the above 3 via Transformers.
* 📦 Build Vector Tiles, Raster Tiles, and Gridded Data Tiles. Vector supports 3 output formats (Mapbox Vector Tile, Open S2 Tiles, and Flat Open S2 Tiles).
* ✅ Most data structures support all projections, but primarily focus the big 3 states above. Also handle large data sets through working with the filesystem and mmap buffers.
* 📖 Contains **23** native GIS readers. The list of readers are: CSV, GBFS, GeoTIFF, GPX, GRIB2, GTFS, JPEG and JPEG2000, (Geo|S2)JSON, LineDelimted GeoJSON, GeoJSON Text Sequences, LAS, LAZ, NadGrids, NetCDF, OSM, (S2)PMTiles, Shapefiles, raster and vector tiles, WKT, and XML.
* 🦺 Secure code where the only external dependency is [sharp](https://github.com/lovell/sharp) for local image processing, otherwise all code is written internally by Open S2 in Typescript. Rust requires a little more: [libm](https://github.com/rust-lang/libm), [half](https://docs.rs/half/latest/half/), [regex](https://github.com/rust-lang/regex), [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), and some optional std [memmap2](https://github.com/RazrFalcon/memmap2-rs), [image](https://github.com/image-rs/image), and [flate2](https://github.com/rust-lang/flate2-rs).
* 🧲 Full suite of tools for points, lines, polygons, greater-circle-arcs, predicates, and more.
* 🌌 Space specific tools for planets and satellite orbits.

## Goals

Making GIS data easy to parse and work with. One of the biggest issues in GIS right now is how segmented various niche tools are. The other issue is how most solutions to read GIS data are half baked, deprecated, or partially parse results that need to be transformed one more time to use them.

So this tool exists to make GIS tools simple to use, various data fast to parse, and transformed to usable projections without having to worry about the details.

Lastly the goal is for all code to be accessible to both the browser and locally. An example was the shapefile reader where it can pull from online data or handle extremely large data as well.

## Install

```bash
# NPM
npm install gis-tools-ts
# PNPM
pnpm add gis-tools-ts
# Yarn
yarn add gis-tools-ts
# Bun
bun add gis-tools-ts
# Deno
deno install gis-tools-ts

# Cargo
cargo add gis-tools
```

### CLI Install

You can read the docs on the [CLI here](/docs-ts/cli.md).

```bash
# NPM
npm install -g gis-tools-ts
# PNPM
pnpm add -g gis-tools-ts
# Yarn
yarn global add gis-tools-ts
# Bun
bun add -g gis-tools-ts
# Deno
deno install -g gis-tools-ts
```

## Components

> 💡 **NOTE:** The sizes are estimates and can change based on how you use them. Click the module link for documentation and more precise guides on file cost.

### Data Stores

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [externalSort]           | ![ES Badge][esBadge]          | Sort large files with uint64 keys                                |
| [kv]                     | ![KV Badge][kvBadge]          | Key-Value store that works in the browser and the filesystem.    |
| [multiMap]               | ![MM Badge][mmBadge]          | Multi-map that works in the browser and the filesystem.          |
| [vector]                 | ![Vec Badge][vecBadge]        | Vector store that works in the browser and the filesystem.       |

[externalSort]: /docs-ts/dataStore/externalSort.md
[esBadge]: /assets/badges/externalSort-gzip-cover.svg
[kv]: /docs-ts/dataStore/kv.md
[kvBadge]: /assets/badges/kv-gzip-cover.svg
[multiMap]: /docs-ts/dataStore/multiMap.md
[mmBadge]: /assets/badges/multimap-gzip-cover.svg
[vector]: /docs-ts/dataStore/vector.md
[vecBadge]: /assets/badges/vector-gzip-cover.svg

### Data Structures

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [cache]                  | ![Cache Badge][cacheBadge]    | A KV cache for values with a max size. Least used dropped first. |
| [pointGrid]              | ![PG Badge][pgBadge]          | Point grid tiling for number or raster data.                     |
| [pointCluster]           | ![PC Badge][pcBadge]          | Point cluster tool with indexing.                                |
| [pointIndex]             | ![PI Badge][piBadge]          | Point indexing with range/radius queries.                        |
| [pointIndexFast]         | ![PIF Badge][pifBadge]        | Faster point indexing with range/radius queries.                 |
| [priorityQueue]          | ![PQ Badge][pqBadge]          | A priority queue.                                                |
| [splayTree]              | ![ST Badge][stBadge]          | Self balancing binary tree with O(log n) access                  |
| [tile]                   | ![Tile Badge][tileBadge]      | A tile/layer management tool for features.                       |

[cache]: /docs-ts/dataStructures/cache.md
[cacheBadge]: /assets/badges/cache-gzip-cover.svg
[pointGrid]: /docs-ts/dataStructures/pointGrid.md
[pgBadge]: /assets/badges/pointGrid-gzip-cover.svg
[pointCluster]: /docs-ts/dataStructures/pointCluster.md
[pcBadge]: /assets/badges/pointCluster-gzip-cover.svg
[pointIndex]: /docs-ts/dataStructures/pointIndex.md
[piBadge]: /assets/badges/pointIndex-gzip-cover.svg
[pointIndexFast]: /docs-ts/dataStructures/pointIndexFast.md
[pifBadge]: /assets/badges/pointIndexFast-gzip-cover.svg
[priorityQueue]: /docs-ts/dataStructures/priorityQueue.md
[pqBadge]: /assets/badges/priorityQueue-gzip-cover.svg
[splayTree]: /docs-ts/dataStructures/splayTree.md
[stBadge]: /assets/badges/splayTree-gzip-cover.svg
[tile]: /docs-ts/dataStructures/tile.md
[tileBadge]: /assets/badges/dataTile-gzip-cover.svg

### Geometry

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [angles]                 | ![Angle Badge][anglesBadge]   | Spherical geodetic angle methods.                                |
| [bbox]                   | ![BBOX Badge][bboxBadge]      | Bounding box creation/manipulation.                              |
| [id]                     | ![ID Badge][idBadge]          | ID tools for S2 and WM.                                          |
| [lonlat]                 | ![LonLat Badge][lonlanBadge]  | Longitude/Latitude convienience methods.                         |
| [predicates]             | ![Pred Badge][predBadge]      | Reliability predicates for 2D and 3D orientation geometry.       |
| [s2]                     | ![S2 Badge][s2Badge]          | S2 geometry convienience methods.                                |
| [tools]                  | ![Tools Badge][toolsBadge]    | Geometry manipulation tools.                                     |
| [wm]                     | ![WM Badge][wmBadge]          | Web Mercator (WM) geometry convienience methods.                 |

[angles]: /docs-ts/geometry/angles.md
[anglesBadge]: /assets/badges/angles-gzip-cover.svg
[bbox]: /docs-ts/geometry/bbox.md
[bboxBadge]: /assets/badges/bbox-gzip-cover.svg
[id]: /docs-ts/geometry/id.md
[idBadge]: /assets/badges/id-gzip-cover.svg
[lonlat]: /docs-ts/geometry/lonlat.md
[lonlanBadge]: /assets/badges/lonlat-gzip-cover.svg
[predicates]: /docs-ts/geometry/predicates.md
[predBadge]: /assets/badges/predicates-gzip-cover.svg
[s2]: /docs-ts/geometry/s2.md
[s2Badge]: /assets/badges/s2-gzip-cover.svg
[tools]: /docs-ts/geometry/tools.md
[toolsBadge]: /assets/badges/tools-gzip-cover.svg
[wm]: /docs-ts/geometry/wm.md
[wmBadge]: /assets/badges/wm-gzip-cover.svg

### PROJ4

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [mgrs]                   | ![MGRS Badge][mgrsBadge]      | Military Grid Reference System (MGRS) converter.                 |
| [projections]            | ![Proj Badge][projBadge]      | Supports a large list of projections to be used by transformers. |
| [transformer]            | ![Trans Badge][transBadge]    | Tool for transforming coordinates from one projection to another.|

[mgrs]: /docs-ts/proj4/mgrs.md
[mgrsBadge]: /assets/badges/mgrs-gzip-cover.svg
[projections]: /docs-ts/proj4/projections.md
[projBadge]: /assets/badges/projections-gzip-cover.svg
[transformer]: /docs-ts/proj4/transformer.md
[transBadge]: /assets/badges/transformer-gzip-cover.svg

### Readers

Most readers are parsers that take `ReaderInputs` as an input. This is to ensure both browser and file inputs are supported. You can learn more about [readers here](/docs-ts/readers/reader.md).

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [jpeg]                   | ![JPEG Badge][jpegBadge]      | Read/parse JPEG data.                                            |
| [jpeg2000]               | ![JPEG2 Badge][jpeg2Badge]    | Read/parse JPEG 2000 data.                                       |
| [lanczos]                | ![Lanc Badge][lancBadge]      | Apply a Lanczos filter that downsamples an image.                |
| [csv]                    | ![CSV Badge][csvBadge]        | CSV data reader with options on parsing.                         |
| [gbfs]                   | ![GBFS Badge][gbfsBadge]      | General Bikeshare Feed Specification reader.                     |
| [geotiff]                | ![GTiff Badge][gtiffBadge]    | Geotiff image reader with projection support.                    |
| [gpx]                    | ![GPX Badge][gpxBadge]        | GPX (xml based) data reader.                                     |
| [grib2]                  | ![grib2 Badge][grib2Badge]    | GRIB 2 data reader.                                              |
| [gtfs]                   | ![gtfs Badge][gtfsBadge]      | General Transit Feed Specification. Both Static and Realtime.    |
| [json]                   | ![JSON Badge][jsonBadge]      | JSON data reader with line delimiter support.                    |
| [las]                    | ![LAS Badge][lasBadge]        | LAS data reader.                                                 |
| [laz]                    | ![LAZ Badge][lazBadge]        | LASzipped data reader.                                           |
| [nadgrid]                | ![NGrid Badge][ngridBadge]    | NAD Grid data reader.                                            |
| [netcdf]                 | ![NetCDF Badge][netcdfBadge]  | NetCDF data reader.                                              |
| [osm]                    | ![OSM Badge][osmBadge]        | OpenStreetMap PBF data reader                                    |
| [pmtiles]                | ![PMT Badge][pmtBadge]        | (S2)PMTiles data reader.                                         |
| [protobuf]               | ![Proto Badge][protoBadge]    | Protobuf data reader/writer.                                     |
| [shapefile]              | ![Shape Badge][shapeBadge]    | Shapefile data reader supporting DBF and projections (PRJ).      |
| [tileReader]             | ![TR Badge][trBadge]          | Tile data reader, usually from a local input folder.             |
| [wkt]                    | ![wkt Badge][wktBadge]        | Well Known Text data reader.                                     |
| [xml]                    | ![XML Badge][xmlBadge]        | XML data reader.                                                 |

[jpeg]: /docs-ts/readers/image/jpeg.md
[jpegBadge]: /assets/badges/jpeg-gzip-cover.svg
[jpeg2000]: /docs-ts/readers/image/jpeg2000.md
[jpeg2Badge]: /assets/badges/jpeg2000-gzip-cover.svg
[lanczos]: /docs-ts/readers/image/lanczos.md
[lancBadge]: /assets/badges/lanczos-gzip-cover.svg
[csv]: /docs-ts/readers/csv.md
[csvBadge]: /assets/badges/csv-gzip-cover.svg
[gbfs]: /docs-ts/readers/gbfs.md
[gbfsBadge]: /assets/badges/gbfs-gzip-cover.svg
[geotiff]: /docs-ts/readers/geotiff.md
[gtiffBadge]: /assets/badges/geotiff-gzip-cover.svg
[gpx]: /docs-ts/readers/gpx.md
[gpxBadge]: /assets/badges/gpx-gzip-cover.svg
[grib2]: /docs-ts/readers/grib2.md
[grib2Badge]: /assets/badges/grib2-gzip-cover.svg
[gtfs]: /docs-ts/readers/gtfs.md
[gtfsBadge]: /assets/badges/gtfs-gzip-cover.svg
[json]: /docs-ts/readers/json.md
[jsonBadge]: /assets/badges/json-gzip-cover.svg
[las]: /docs-ts/readers/las.md
[lasBadge]: /assets/badges/las-gzip-cover.svg
[laz]: /docs-ts/readers/laz.md
[lazBadge]: /assets/badges/laz-gzip-cover.svg
[nadgrid]: /docs-ts/readers/nadgrid.md
[ngridBadge]: /assets/badges/nadgrid-gzip-cover.svg
[netcdf]: /docs-ts/readers/netcdf.md
[netcdfBadge]: /assets/badges/netcdf-gzip-cover.svg
[osm]: /docs-ts/readers/osm.md
[osmBadge]: /assets/badges/osm-gzip-cover.svg
[pmtiles]: /docs-ts/readers/pmtiles.md
[pmtBadge]: /assets/badges/pmtilesReader-gzip-cover.svg
[protobuf]: /docs-ts/readers/protobuf.md
[protoBadge]: /assets/badges/protobuf-gzip-cover.svg
[shapefile]: /docs-ts/readers/shapefile.md
[shapeBadge]: /assets/badges/shapefile-gzip-cover.svg
[tileReader]: /docs-ts/readers/tile.md
[trBadge]: /assets/badges/tileReader-gzip-cover.svg
[wkt]: /docs-ts/readers/wkt.md
[wktBadge]: /assets/badges/wkt-gzip-cover.svg
[xml]: /docs-ts/readers/xml.md
[xmlBadge]: /assets/badges/xml-gzip-cover.svg

### Space

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [planets]                | ![Planet Badge][planetBadge]  | Collection of planet constants with observation tools.           |
| [satellite]              | ![SAT Badge][satBadge]        | Satellite Orbit Class from TLE data                              |

[planets]: /docs-ts/space/planets.md
[planetBadge]: /assets/badges/planets-gzip-cover.svg
[satellite]: /docs-ts/space/satellite.md
[satBadge]: /assets/badges/sat-gzip-cover.svg

### Tools

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [delaunator]             | ![DEL Badge][delBadge]        | Delaunay triangulation of 2D points.                             |
| [interpolators]          | ![INT Badge][intBadge]        | Interpolate values from points and weights.                      |
| [orthodrome]             | ![ORT Badge][orthBadge]       | Find shortest path between two points or point on path.          |
| [polylabel]              | ![POL Badge][polBadge]        | Find the labels for vector polygons                              |

[delaunator]: /docs-ts/tools/delaunator.md
[delBadge]: /assets/badges/delaunator-gzip-cover.svg
[interpolators]: /docs-ts/tools/interpolators.md
[intBadge]: /assets/badges/interpolators-gzip-cover.svg
[orthodrome]: /docs-ts/tools/orthodrome.md
[orthBadge]: /assets/badges/orthodrome-gzip-cover.svg
[polylabel]: /docs-ts/tools/polylabel.md
[polBadge]: /assets/badges/polylabel-gzip-cover.svg

### Writers

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [pmtilesWriter]          | ![PMTW Badge][pmtwBadge]      | Write (S2)PMTiles data.                                          |
| [tileWriter]             | ![TW Badge][twBadge]          | Write (S2)Tiles data. Supports time series as well.              |
| [toJSON]                 | ![To JSON Badge][toJSONBadge] | Convert any Reader to JSON data.                                 |
| [toTiles]                | ![FT Badge][toTilesBadge]     | Convert any Reader to vector and/or raster tiles.                |

[pmtilesWriter]: /docs-ts/writers/pmtiles.md
[pmtwBadge]: /assets/badges/pmtilesWriter-gzip-cover.svg
[tileWriter]: /docs-ts/writers/tile.md
[twBadge]: /assets/badges/tileWriter-gzip-cover.svg
[toJSON]: /docs-ts/writers/toJSON.md
[toJSONBadge]: /assets/badges/toJSON-gzip-cover.svg
[toTiles]: /docs-ts/writers/toTiles.md
[toTilesBadge]: /assets/badges/toTiles-gzip-cover.svg

### Utils

| Module                   | Size                          | <img width="550" height="0"> Description                         |
| :----------------------- | :---------------------------: | ---------------------------------------------------------------: |
| [polyfills]              | ![PF Badge][pfBadge]          | Collection of polyfills that might add value for the browser.    |
| [compression]            | ![CMP Badge][cmpBadge]        | compression/decompression convenience methods.                   |

[polyfills]: /docs-ts/util/polyfills.md
[pfBadge]: /assets/badges/polyfills-gzip-cover.svg
[compression]: /docs-ts/util/compression.md
[cmpBadge]: /assets/badges/compression-gzip-cover.svg

---

## Development

### Requirements

You need the tool `tarpaulin` to generate the coverage report. Install it using the following command:

```bash
cargo install cargo-tarpaulin
```

The `bacon coverage` tool is used to generate the coverage report. To utilize the [pycobertura](https://pypi.org/project/pycobertura/) package for a prettier coverage report, install it using the following command:

```bash
pip install pycobertura
```

### Running Tests

To run the tests, use the following command:

```bash
# TYPESCRIPT
## basic test
bun run test
## live testing
bun run test:dev

# RUST
## basic test
cargo test
# live testing
bacon test
```

### Running Linters and Coverage

```bash
# TYPESCRIPT LINT
bun run lint
#TYPESCRIPT COVERAGE
bun run test:coverage

# RUST LINT
# bun run rust:test:coverage
RUSTC_BOOTSTRAP=1 cargo +nightly clippy -- -D warnings
# RUST COVERAGE
# bun run rust:test:coverage
RUSTC_BOOTSTRAP=1 cargo +nightly llvm-cov --lcov --output-path coverage/lcov.info --workspace --features nightly
```

### Generating Coverage Report

To generate the coverage report, use the following command:

```bash
# install
cargo +stable install cargo-llvm-cov --locked
# run test
cargo llvm-cov
# bacon
bacon coverage # or type `l` inside the tool

# output lcov
cargo llvm-cov --lcov --output-path coverage/lcov.info
```

### Using Tokei

```bash
cargo install tokei
```
