<h1 style="text-align: center;">
  <div align="center">Readers</div>
</h1>

## Description

The readers are the main entry points for reading data from different file formats.

Most readers require input data that can be both large and small. Because of that fact, this project has a `Reader` interface to ensure consistency across browser and local environments.

The `Reader` interface works much like a DataView:

```ts
/** Reader interface. Implemented to read data from either a buffer or a filesystem */
export interface Reader {
  // Properties
  byteLength: number;
  byteOffset: number;
  // Getters
  getBigInt64: (byteOffset?: number, littleEndian?: boolean) => bigint;
  getBigUint64: (byteOffset?: number, littleEndian?: boolean) => bigint;
  getFloat32: (byteOffset?: number, littleEndian?: boolean) => number;
  getFloat64: (byteOffset?: number, littleEndian?: boolean) => number;
  getInt16: (byteOffset?: number, littleEndian?: boolean) => number;
  getInt32: (byteOffset?: number, littleEndian?: boolean) => number;
  getInt8: (byteOffset?: number) => number;
  getUint16: (byteOffset?: number, littleEndian?: boolean) => number;
  getUint32: (byteOffset?: number, littleEndian?: boolean) => number;
  getUint8: (byteOffset?: number) => number;
  // Methods
  tell(): number; // Returns the current position of the cursor
  seek(pos?: number): void; // Sets the current position of the cursor
  slice: (begin?: number, end?: number) => DataView;
  seekSlice: (size: number) => DataView;
  setStringEncoding: (encoding: string) => void;
  parseString: (byteOffset?: number, byteLength?: number) => string;
  getRange: (offset: number, length: number) => Promise<Uint8Array>;
}
```

The existing readers are:

- [BufferReader](https://open-s2.github.io/gis-tools/classes/index.BufferReader.html)
- [FileReader](https://open-s2.github.io/gis-tools/classes/index.FileReader.html)
- [MMapReader](https://open-s2.github.io/gis-tools/classes/index.MMapReader.html)
- [FetchReader](https://open-s2.github.io/gis-tools/classes/index.FetchReader.html)

The MMapReader is a [Bun](https://bun.sh/) exclusive feature.

To make the use of each reader as simple as possible, every GIS reader class takes in a [ReaderInputs](https://open-s2.github.io/gis-tools/types/index.ReaderInputs.html) to reduce the pain of needing a specific data type. The allowed inputs are as follows:

```ts
/** All input types that can be placed into a reader */
export type ReaderInputs =
  | Reader
  | BufferReader
  | Buffer
  | ArrayBufferLike
  | Uint8Array
  | Uint8ClampedArray
  | Uint16Array
  | Uint32Array
  | Int8Array
  | Int16Array
  | Int32Array
  | Float32Array
  | Float64Array
  | DataView;
```

## Examples

Assuming we are in the browser, we can parse a GeoTIFF for instance as follows:

```ts
import { GeoTIFFReader } from 'gis-tools-ts';

const data = await fetch('https://example.com/example.tiff').then(async (res) => await res.arrayBuffer());
const reader = new GeoTIFFReader(data);
```

If we are in a local system like node/bun/deno/etc., we can parse a GeoTIFF as follows:

```ts
import { GeoTIFFReader } from 'gis-tools-ts';
import { FileReader } from 'gis-tools-ts/file';

const reader = new GeoTIFFReader(new FileReader('example.tiff'));

// OR use the filesystem API if the file is simple/small enough
import { readFile } from 'fs/promises';
const reader = new GeoTIFFReader(await readFile('example.tiff'));
```

Bun supports a MMap system which can enable a significant performance boost:

```ts
import { GeoTIFFReader } from 'gis-tools-ts';
import { MMapReader } from 'gis-tools-ts/mmap';

const reader = new GeoTIFFReader(new MMapReader('example.tiff'));

// OR use the bun fs API if the file is simple/small enough
const reader = new GeoTIFFReader(await Bun.file('example.tiff').arrayBuffer());
```

Sometimes there are readers that simplify working with the filesystem like OSM data. For example, we can parse an OSM PBF file using the [OSMFileReader](https://open-s2.github.io/gis-tools/classes/file.OSMFileReader.html):

```ts
import { OSMFileReader } from 'gis-tools-ts/file';

const reader = new OSMFileReader('example.osm.pbf');
```

Or again using the MMAP system available in Bun we can use the [OSMMMapReader](https://open-s2.github.io/gis-tools/classes/mmap.OSMMMapReader.html):

```ts
import { OSMMMapReader } from 'gis-tools-ts/mmap';

const reader = new OSMMMapReader('example.osm.pbf');
```

Be sure to read the documentation for the reader you are using to see what kind of optimizations are available to you.
