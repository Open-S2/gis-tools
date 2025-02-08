# Build countries list

## Get data

Grab latest OSM PBF file from `https://planet.openstreetmap.org/`

## Filter countries

Until the Rust port is complete, use osmium for speeed to filter out countries:

```shell
# filter to admin_level=2 relations
osmium tags-filter planet-250127.osm.pbf r/admin_level=2 -o countries.osm.pbf
# filter out the ways, we don't need the boundary data.
osmosis --read-pbf countries.osm.pbf --tf reject-ways --used-node --write-pbf countries_no_ways.osm.pbf
```

## Convert to GeoJSON

`bun run tools/toCountriesJSON.ts`
