# GIS Tools Typescript Documentation

## Purpose

## Components

> 💡 **NOTE:** The sizes are estimates and can change based on how you use them. Click the module link for documentation and more precise guides on file cost.

### Converters

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [toJSON]                 | ![To JSON Badge][toJSONBadge] | Convert any Reader to JSON data.                                 |
| [toTiles]                | ![FT Badge][toTilesBadge]     | Convert any Reader to vector and/or raster tiles.                |

[toJSON]: /converters/toJSON.md
[toJSONBadge]: /assets/badges/toJSON-gzip-cover.svg
[toTiles]: /converters/toTiles.md
[toTilesBadge]: /assets/badges/toTiles-gzip-cover.svg

### Data Stores

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [externalSort]           | ![ES Badge][esBadge]          | Sort large files with uint64 keys                                |
| [kd]                     | ![KD Badge][kdBadge]          | KD Spatial index that works in the browser and the filesystem.   |
| [kv]                     | ![KV Badge][kvBadge]          | Key-Value store that works in the browser and the filesystem.    |
| [multiMap]               | ![MM Badge][mmBadge]          | Multi-map that works in the browser and the filesystem.          |
| [vector]                 | ![Vec Badge][vecBadge]        | Vector store that works in the browser and the filesystem.       |

[externalSort]: /dataStore/externalSort.md
[esBadge]: /assets/badges/externalSort-gzip-cover.svg
[kd]: /dataStore/kd.md
[kdBadge]: /assets/badges/kd-gzip-cover.svg
[kv]: /dataStore/kv.md
[kvBadge]: /assets/badges/kv-gzip-cover.svg
[multiMap]: /dataStore/multimap.md
[mmBadge]: /assets/badges/multimap-gzip-cover.svg
[vector]: /dataStore/vector.md
[vecBadge]: /assets/badges/vector-gzip-cover.svg

### Data Structures

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [cache]                  | ![Cache Badge][cacheBadge]    | A KV cache for values with a max size. Least used dropped first. |
| [pointGrid]              | ![PG Badge][pgBadge]          | Point grid tiling for number or raster data.                     |
| [pointCluster]           | ![PC Badge][pcBadge]          | Point cluster tool with indexing.                                |
| [pointIndex]             | ![PI Badge][piBadge]          | Point indexing with range/radius queries.                        |
| [pointIndexFast]         | ![PIF Badge][pifBadge]        | Faster point indexing with range/radius queries.                 |
| [priorityQueue]          | ![PQ Badge][pqBadge]          | A priority queue.                                                |
| [tile]                   | ![Tile Badge][tileBadge]      | A tile/layer management tool for features.                       |

[cache]: /dataStructures/cache.md
[cacheBadge]: /assets/badges/cache-gzip-cover.svg
[pointGrid]: /dataStructures/pointGrid.md
[pgBadge]: /assets/badges/pointGrid-gzip-cover.svg
[pointCluster]: /dataStructures/pointCluster.md
[pcBadge]: /assets/badges/pointCluster-gzip-cover.svg
[pointIndex]: /dataStructures/pointIndex.md
[piBadge]: /assets/badges/pointIndex-gzip-cover.svg
[pointIndexFast]: /dataStructures/pointIndexFast.md
[pifBadge]: /assets/badges/pointIndexFast-gzip-cover.svg
[priorityQueue]: /dataStructures/priorityQueue.md
[pqBadge]: /assets/badges/priorityQueue-gzip-cover.svg
[tile]: /dataStructures/tile.md
[tileBadge]: /assets/badges/dataTile-gzip-cover.svg

### Geometry

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [angles]                 | ![Angle Badge][anglesBadge]   | Spherical geodetic angle methods.                                |
| [bbox]                   | ![BBOX Badge][bboxBadge]      | Bounding box creation/manipulation.                              |
| [id]                     | ![ID Badge][idBadge]          | ID tools for S2 and WM.                                          |
| [lonlat]                 | ![LonLat Badge][lonlanBadge]  | Longitude/Latitude convienience methods.                         |
| [planets]                | ![Planet Badge][planetBadge]  | Collection of planet constants with observation tools.           |
| [predicates]             | ![Pred Badge][predBadge]      | Reliability predicates for 2D and 3D orientation geometry.       |
| [s2]                     | ![S2 Badge][s2Badge]          | S2 geometry convienience methods.                                |
| [tools]                  | ![Tools Badge][toolsBadge]    | Geometry manipulation tools.                                     |
| [wm]                     | ![WM Badge][wmBadge]          | Web Mercator (WM) geometry convienience methods.                 |

[angles]: /dataStructures/angles.md
[anglesBadge]: /assets/badges/angles-gzip-cover.svg
[bbox]: /dataStructures/bbox.md
[bboxBadge]: /assets/badges/bbox-gzip-cover.svg
[id]: /dataStructures/id.md
[idBadge]: /assets/badges/id-gzip-cover.svg
[lonlat]: /dataStructures/lonlat.md
[lonlanBadge]: /assets/badges/lonlat-gzip-cover.svg
[planets]: /dataStructures/planets.md
[planetBadge]: /assets/badges/planets-gzip-cover.svg
[predicates]: /dataStructures/predicates.md
[predBadge]: /assets/badges/predicates-gzip-cover.svg
[s2]: /dataStructures/s2.md
[s2Badge]: /assets/badges/s2-gzip-cover.svg
[tools]: /dataStructures/tools.md
[toolsBadge]: /assets/badges/tools-gzip-cover.svg
[wm]: /dataStructures/wm.md
[wmBadge]: /assets/badges/wm-gzip-cover.svg

### PROJ4

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [mgrs]                   | ![MGRS Badge][mgrsBadge]      | Military Grid Reference System (MGRS) converter.                 |
| [projections]            | ![Proj Badge][projBadge]      | Supports a large list of projections to be used by transformers. |
| [transformers]           | ![Trans Badge][transBadge]    | Tool for transforming coordinates from one projection to another.|

[mgrs]: /proj4/mgrs.md
[mgrsBadge]: /assets/badges/mgrs-gzip-cover.svg
[projections]: /proj4/projections.md
[projBadge]: /assets/badges/projections-gzip-cover.svg
[transformers]: /proj4/transformers.md
[transBadge]: /assets/badges/transformer-gzip-cover.svg

### Readers

Most readers are parsers that take `ReaderInputs` as an input. This is to ensure both browser and file inputs are supported. You can learn more about [readers here](/readers/reader.md).

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [jpeg]                   | ![JPEG Badge][jpegBadge]      | Read/parse JPEG data.                                            |
| [jpeg2000]               | ![JPEG2 Badge][jpeg2Badge]    | Read/parse JPEG 2000 data.                                       |
| [lanczos]                | ![Lanc Badge][lancBadge]      | Apply a Lanczos filter that downsamples an image.                |
| [csv]                    | ![CSV Badge][csvBadge]        | CSV data reader with options on parsing.                         |
| [gbfs]                   | ![GBFS Badge][gbfsBadge]      | General Bikeshare Feed Specification reader.                     |
| [geotiff]                | ![GTiff Badge][gtiffBadge]    | Geotiff image reader with projection support.                    |
| [gpx]                    | ![GPX Badge][gpxBadge]        | GPX (xml based) data reader.                                     |
| [grib2]                  | ![grib2 Badge][grib2Badge]    | GRIB 2 data reader.                                              |
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

[jpeg]: /readers/image/jpeg.md
[jpegBadge]: /assets/badges/jpeg-gzip-cover.svg
[jpeg2000]: /readers/image/jpeg2000.md
[jpeg2Badge]: /assets/badges/jpeg2000-gzip-cover.svg
[lanczos]: /readers/image/lanczos.md
[lancBadge]: /assets/badges/lanczos-gzip-cover.svg
[csv]: /readers/csv.md
[csvBadge]: /assets/badges/csv-gzip-cover.svg
[gbfs]: /readers/gbfs.md
[gbfsBadge]: /assets/badges/gbfs-gzip-cover.svg
[geotiff]: /readers/geotiff.md
[gtiffBadge]: /assets/badges/geotiff-gzip-cover.svg
[gpx]: /readers/gpx.md
[gpxBadge]: /assets/badges/gpx-gzip-cover.svg
[grib2]: /readers/grib2.md
[grib2Badge]: /assets/badges/grib2-gzip-cover.svg
[json]: /readers/json.md
[jsonBadge]: /assets/badges/json-gzip-cover.svg
[las]: /readers/las.md
[lasBadge]: /assets/badges/las-gzip-cover.svg
[laz]: /readers/laz.md
[lazBadge]: /assets/badges/laz-gzip-cover.svg
[nadgrid]: /readers/nadgrid.md
[ngridBadge]: /assets/badges/nadgrid-gzip-cover.svg
[netcdf]: /readers/netcdf.md
[netcdfBadge]: /assets/badges/netcdf-gzip-cover.svg
[osm]: /readers/osm.md
[osmBadge]: /assets/badges/osm-gzip-cover.svg
[pmtiles]: /readers/pmtiles.md
[pmtBadge]: /assets/badges/pmtilesReader-gzip-cover.svg
[protobuf]: /readers/protobuf.md
[protoBadge]: /assets/badges/protobuf-gzip-cover.svg
[shapefile]: /readers/shapefile.md
[shapeBadge]: /assets/badges/shapefile-gzip-cover.svg
[tileReader]: /readers/tileReader.md
[trBadge]: /assets/badges/tileReader-gzip-cover.svg
[wkt]: /readers/wkt.md
[wktBadge]: /assets/badges/wkt-gzip-cover.svg
[xml]: /readers/xml.md
[xmlBadge]: /assets/badges/xml-gzip-cover.svg

### Space

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [satellite]              | ![SAT Badge][satBadge]        | Satellite Orbit Class from TLE data                              |

[satellite]: /space/satellite.md
[satBadge]: /assets/badges/sat-gzip-cover.svg

### Tools

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [delaunator]             | ![DEL Badge][delBadge]        | Delaunay triangulation of 2D points.                             |
| [interpolators]          | ![INT Badge][intBadge]        | Interpolate values from points and weights.                      |
| [orthodrome]             | ![ORT Badge][orthBadge]       | Find shortest path between two points or point on path.          |
| [polylabel]              | ![POL Badge][polBadge]        | Find the labels for vector polygons                              |

[delaunator]: /tools/delaunator.md
[delBadge]: /assets/badges/delaunator-gzip-cover.svg
[interpolators]: /tools/interpolators.md
[intBadge]: /assets/badges/interpolators-gzip-cover.svg
[orthodrome]: /tools/orthodrome.md
[orthBadge]: /assets/badges/orthodrome-gzip-cover.svg
[polylabel]: /tools/polylabel.md
[polBadge]: /assets/badges/polylabel-gzip-cover.svg

### Writers

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [pmtilesWriter]          | ![PMTW Badge][pmtwBadge]      | Write (S2)PMTiles data.                                          |
| [tileWriter]             | ![TW Badge][twBadge]          | Write (S2)Tiles data. Supports time series as well.              |

[pmtilesWriter]: /writers/pmtilesWriter.md
[pmtwBadge]: /assets/badges/pmtilesWriter-gzip-cover.svg
[tileWriter]: /writers/tileWriter.md
[twBadge]: /assets/badges/tileWriter-gzip-cover.svg

### Utils

| Main Modules             | Size                          | Description                                                      |
| ------------------------ | ----------------------------- | ---------------------------------------------------------------- |
| [polyfills]              | ![PF Badge][pfBadge]          | Collection of polyfills that might add value for the browser.    |
| [gzip]                   | ![GZ Badge][gzBadge]          | gzip compression/decompression convenience methods.              |
| [lzw]                    | ![LZW Badge][lzwBadge]        | lzw decompression methods.                                       |

[polyfills]: /utils/polyfills.md
[pfBadge]: /assets/badges/polyfills-gzip-cover.svg
[gzip]: /utils/gzip.md
[gzBadge]: /assets/badges/gzip-gzip-cover.svg
[lzw]: /utils/lzw.md
[lzwBadge]: /assets/badges/lzw-gzip-cover.svg
