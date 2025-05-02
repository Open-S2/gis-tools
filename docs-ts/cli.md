# CLI Tool

This tool is a command line interface for [GIS Tools](/docs-ts/index.md).

## Install

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
deno install -A gis-tools-ts
```

## Usage

### Convert

Example of converting a shapefile to GeoJSON

```bash
# Convert a shapefile to GeoJSON
gis-tools-ts convert -i input.shp -o output.geojson
```

Input Types are:

- CSV (`.csv`)
- GeoTIFF (`.tif`, `.tiff`, `.geotif`, `.geotiff`)
- GPX (`.gpx`)
- GRIB2 (`.grib`, `.grib2`)
- (Geo|S2)JSON (`.json`, `.geojson`, `.s2json`)
- LineDelimited (Geo|S2)JSON (`.jsonld`, `.geojsonld`, `.s2jsonld`, `.json-ld`, `.geojson-ld`, `.s2json-ld`)
- Sequence (Geo|S2)JSON (`.jsonsq`, `.geojsonsq`, `.s2jsonsq`, `.json-sq`, `.geojson-sq`, `.s2json-sq`)
- LAS (`.las`)
- LASzipped (`.laz`)
- NetCDF (`.nc4`, `.cdf`, `.nc`, `.netcdf`)
- OSM (`.osm`)
- RasterTiles (`.raster`)
- Shapefile (`.shp`)
- WellKnownText (`.wkt`)

Currently the only available Output Types are:

- GeoJSON (`.geojson`)
- GeoJSON-LD (`.geojsonld`)
- S2JSON (`.json`, `.s2json`)
- S2JSON-LD (`.jsonld`, `.s2jsonld`)
