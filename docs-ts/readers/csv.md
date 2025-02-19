<h1 style="text-align: center;">
  <div align="center">CSV Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/csv-file.svg" alt="csv-file-ts">
  <img src="../../assets/badges/csv-gzip.svg" alt="csv-gzip-ts">
  <img src="../../assets/badges/csv-brotli.svg" alt="csv-brotli-ts">
</p>

## Description

Parse (Geo|S2)JSON from a file that is in the CSV format.

Implements the `FeatureIterator` interface which means you can use it in a `for await` loop for all the resulting Vector Features.

## Usage

### GIS Reader

Be sure to checkout the [Reader](reader.md) page for more knowledge on how to input data into the CSVReader.

```ts
import { CSVReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';
// Or use the MMapReader if using Bun:
// import { MMapReader } from 'gis-tools-ts/mmap';
import type { CSVReaderOptions } from 'gis-tools-ts';

// set options, completely optional, but the values you see are the default.
const options: CSVReaderOptions = {
  delimiter: ',', // how the CSV document separates values
  lineDelimiter: '\n', // the line delimiter, sometimes "\r\n" or some specializ unicode character
  lonKey: 'Longitude', // the key to use to store the longitude
  latKey: 'Latitude', // the key to use to store the latitude
  heightKey: 'height', // the key to use for the height or "z-value"
};

// pull in the data into a reader.
const fileReader = new FileReader('./basic3D.csv');
const csvReader = new CSVReader(fileReader, options);

for await (const feature of csvReader) {
  // do something with the feature
}
```

### Record Reader

If you need to parse just basic CSV records, you can use the `parseCSVAsRecord` function.
This implementation assumes the records are simple enough to parse the entire string at once.

```ts
import { parseCSVAsRecord } from 'gis-tools-ts';

const csv: string = '...';
const records = parseCSVAsRecord<{ a: string, b: number }>(csv);
```

## Useful links

- <https://en.wikipedia.org/wiki/Comma-separated_values>
