<h1 style="text-align: center;">
  <div align="center">LAZ Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/laz-file.svg" alt="laz-file-ts">
  <img src="../../assets/badges/laz-gzip.svg" alt="laz-gzip-ts">
  <img src="../../assets/badges/laz-brotli.svg" alt="laz-brotli-ts">
</p>

## Description

Reads LAZ data. Supports up to the LAS 1.4 specification. [See specification](https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf) for more details.

Implements the [FeatureIterator](https://open-s2.github.io/gis-tools/interfaces/index.FeatureIterator.html) interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the LAZReader.

```ts
import { LASZipReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new LASZipReader(new FileReader('./data.laz'));

// read the features
for (const feature of reader) {
  console.log(feature);
}
```

## Useful links

- <https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf>
- <https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online>
- <https://liblas.org/development/index.html>
- <https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf>
- <https://github.com/PDAL/PDAL>
- <https://github.com/libLAS/libLAS> (deprecated for PDAL)
- <https://github.com/LASzip>
