<h1 style="text-align: center;">
  <div align="center">Shapefile Writer</div>
</h1>

<p align="center">
  <img src="../../assets/badges/shapefileWriter-file.svg" alt="shapefileWriter-file-ts">
  <img src="../../assets/badges/shapefileWriter-gzip.svg" alt="shapefileWriter-gzip-ts">
  <img src="../../assets/badges/shapefileWriter-brotli.svg" alt="shapefileWriter-brotli-ts">
</p>

## Description

Given a writer and an array of iterators, write the input features property data into a SHP file

NOTE: The correct way to store geometry in a shapefile is to only store one kind of geometry. However, this libraries writer and reader do not enforce this.

## Usage

#### Write to files

99% of the time you will want to write to files. While the `toSHP` works in the browser, this is a direct way to work with the filesystem.

```ts
import { toSHP, JSONReader } from 'gis-tools-ts';
import { FileReader, FileWriter } from 'gis-tools-ts/file';

// setup readers and writers
const jsonReader = new JSONReader(new FileReader('./points.geojson'));
const shpWriter = new FileWriter('./points.shp');
const dbfWriter = new FileWriter('./points.dbf');
const shxWriter = new FileWriter('./points.shx');
const prjWriter = new FileWriter('./points.prj');

// store to outputs
await toSHP(shpWriter, [jsonReader], dbfWriter, shxWriter, prjWriter);
```

#### Zip the files

More often than not, you will want to zip the files together for easy distribution.

```ts
import { zipFolder } from 'gis-tools-ts';

const shpFile = await Bun.file('./points.shp').arrayBuffer();
const dbfFile = await Bun.file('./points.dbf').arrayBuffer();
const shxFile = await Bun.file('./points.shx').arrayBuffer();
const prjFile = await Bun.file('./points.prj').arrayBuffer();

const zippedData = await zipFolder([
  { name: 'points.shp', comment: 'shapefile data', data: shpFile },
  { name: 'points.dbf', comment: 'properties data', data: dbfFile },
  { name: 'points.shx', comment: 'index data', data: shxFile },
  { name: 'points.prj', comment: 'projection', data: prjFile },
]);
```

## Useful links

- <https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf>
