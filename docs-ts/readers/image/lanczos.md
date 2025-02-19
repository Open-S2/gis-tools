<h1 style="text-align: center;">
  <div align="center">Lanczos Resampling</div>
</h1>

<p align="center">
  <img src="../../../assets/badges/lanczos-file.svg" alt="lanczos-file-ts">
  <img src="../../../assets/badges/lanczos-gzip.svg" alt="lanczos-gzip-ts">
  <img src="../../../assets/badges/lanczos-brotli.svg" alt="lanczos-brotli-ts">
</p>

## Description

Perform a 2D Lanczos filter on an image. the Lanczos resampler creates some regions that are darker than any in the original and others that are lighter than any in the original. This Fourier transform method is useful in spatial/GIS applications, especially downscaling raster data.

## Usage

```ts
import sharp from 'sharp';
import { lanczos, createImage } from 'gis-tools-ts';

// pull in the image you want to downscale
const pattern = await sharp('./pattern.png') // 8x8 source image
  .raw()
  .toBuffer({ resolveWithObject: true });

// setup the destianation image
const patternHalf = createImage(4, 4);
// resize down into patternHalf
lanczos(pattern, patternHalf);
```

## Useful links

- <https://en.wikipedia.org/wiki/Lanczos_algorithm>
- <https://en.wikipedia.org/wiki/Lanczos_resampling>
- <https://gis.stackexchange.com/questions/10931/what-is-lanczos-resampling-useful-for-in-a-spatial-context>
- <https://icess.eri.ucsb.edu/gem/Duchon_1979_JAM_Lanczos.pdf>
