<h1 style="text-align: center;">
  <div align="center">JPEG Reader</div>
</h1>

<p align="center">
  <img src="../../../assets/badges/jpeg-file.svg" alt="jpeg-file-ts">
  <img src="../../../assets/badges/jpeg-gzip.svg" alt="jpeg-gzip-ts">
  <img src="../../../assets/badges/jpeg-brotli.svg" alt="jpeg-brotli-ts">
</p>

## Description

The Joint Photographic Experts Group (JPEG) is a popular image compression format. It's used by the GeoTIFF format for compression.

## Usage

```ts
import { readFileSync } from 'fs';
import { jpegDecoder, decodeJpegData } from 'gis-tools-ts';
import type { JPEGOptions } from 'gis-tools-ts';

const options: JPEGOptions = { ... };
const inputData = readFileSync('./grumpycat.jpg');

// take a JPEG input and decode it into RGB(A)
const data = decodeJpegData(inputData, options); // Image
// if you're only worried about using the JPEG as raw data use the jpegDecoder function
const decodedData = jpegDecoder(inputData); // ArrayBuffer
```

## Useful links

- <https://en.wikipedia.org/wiki/JPEG>
- <https://en.wikipedia.org/wiki/JPEG_File_Interchange_Format>
- The JPEG specification can be found in the ITU CCITT Recommendation T.81 (<www.w3.org/Graphics/JPEG/itu-t81.pdf>)
- The JFIF specification can be found in the JPEG File Interchange Format (<www.w3.org/Graphics/JPEG/jfif3.pdf>)
- The Adobe Application-Specific JPEG markers in the Supporting the DCT Filters in PostScript Level 2, Technical Note #5116 (partners.adobe.com/public/developer/en/ps/sdk/5116.DCT_Filter.pdf)
