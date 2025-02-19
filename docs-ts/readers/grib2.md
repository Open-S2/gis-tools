<h1 style="text-align: center;">
  <div align="center">GRIB 2 Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/grib2-file.svg" alt="grib2-file-ts">
  <img src="../../assets/badges/grib2-gzip.svg" alt="grib2-gzip-ts">
  <img src="../../assets/badges/grib2-brotli.svg" alt="grib2-brotli-ts">
</p>

## Description

This reads a GRIB2 file and returns a list of GRIB2 products.

Implements the `FeatureIterator` interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the GRIB2Reader.

### The recommended way to parse grib files is to filter out what you want

```ts
// pull .idx file FIRST and filter the ones you want
const filters = [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:'];
// fetch the .idx file
const idxs = await parsedIDXFromURL(`${link}.idx`, filters);
// now bulid the reader
const gribReader =  await GRIB2Reader.fromIDX(link, idxs);

for await (const feature of gribReader) {
  console.log(feature);
}
```

### Parsing the entire grib file (not recommended)

```ts
const gribReader = new GRIB2Reader(link);
```

### GFS Atmosphere data

To get started it's helpful to find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/). You want to start scrolling through the `gfs.*` indexes.

An example of a file tree I walked:

```txt
/pub/data/nccf/com/gfs/prod/gfs.20250209/12/atmos
                [year][month][day] -^    |    |
                                   hour -`    |
                                      domain -`
```

A dataset that I see used in weather products is:

```sh
gfs.t12z.pgrb2.0p25.f000                     09-Feb-2025 15:31  471M  
gfs.t12z.pgrb2.0p25.f000.idx                 09-Feb-2025 15:31   31K 
```

A lot of that 471MB is useless to your application, so the idx is important to help filter it.

Now learning about what products are inside the GRIB2 file can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p25.f000.shtml).

Let's say we want the `1mb TMP Temperature [K]`, `1 mb RH analysis Relative Humidity [%]`, `1 mb UGRD analysis U-Component of Wind [m/s]`, and `1 mb VGRD analysis V-Component of Wind [m/s]` fields. We now have all the constants we need to filter the idx file.

Let's use the AWS database for this example.

```ts
import { fetchGFSWave } from '@turf/grib2';

const data = await fetchGFSAtmos(
  'aws', // source (`aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url)
  'pgrb2.0p25', // product
  'atmos', // domain
  '2025', // year
  '02', // month
  '09', // day
  '12', // hour
  '000', // forecast (3 digits often in increments of 3 up to 384, e.g. '000' or '003')
  [ // FILTERS
    ':TMP:1 mb:',
    ':RH:1 mb:',
    ':UGRD:1 mb:',
    ':VGRD:1 mb:',
  ]
)
```

### GFS WAVE data

To get started it's helpful to find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/). You want to start scrolling through the `gfs.*` indexes then on into `wave/gridded` category.

An example of a file tree I walked:

```txt
/pub/data/nccf/com/gfs/prod/gfs.20250209/12/wave/gridded
                [year][month][day] -^    |    |
                                   hour -`    |
                                      domain -`
```

A dataset that I see used in weather products is:

```sh
gfswave.t12z.arctic.9km.f000.grib2                09-Feb-2025 15:29  5.7M  
gfswave.t12z.arctic.9km.f000.grib2.idx            09-Feb-2025 15:29  851 
```

A lot of that 5.7M is useless to your application, so the idx is important to help filter it.

Now learning about what products are inside the GRIB2 file can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml).

Let's say we want the `surface UGRD 3 hour fcst U-Component of Wind [m/s]` and `surface VGRD 3 hour fcst V-Component of Wind [m/s]` fields. We now have all the constants we need to filter the idx file.

Let's use the AWS database for this example.

```ts
import { fetchGFSWave } from '@turf/grib2';

const data = await fetchGFSWave(
    `aws`,
    'arctic.9km',
    '2025',
    '02',
    '09',
    '12',
    '000',
    ['UGRD:surface', 'VGRD:surface'],
  );
```

## Useful links

- <https://en.wikipedia.org/wiki/GRIB>
- <https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/>
