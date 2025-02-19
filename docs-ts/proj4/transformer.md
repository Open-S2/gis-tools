<h1 style="text-align: center;">
  <div align="center">PROJ Transformer</div>
</h1>

<p align="center">
  <img src="../../assets/badges/transformer-file.svg" alt="transformer-file-ts">
  <img src="../../assets/badges/transformer-gzip.svg" alt="transformer-gzip-ts">
  <img src="../../assets/badges/transformer-brotli.svg" alt="transformer-brotli-ts">
</p>

## Description

The [Transformer](https://open-s2.github.io/gis-tools/classes/index.Transformer.html) class contains all projections necessary for converting coordinates from one projection to another. This is a modular class that can be extended to add new projections as needed to reduce code size and improve performance. Both forward and inverse projections are default set to wgs84.

The Transformer class extends the [NadGridStore](../readers//nadgrid.md) class to support grid lookups for more precise transformations.

## Usage

### Full Example

```ts
import { Transformer, injectAllDefinitions, injectAllEPSGCodes, HotineObliqueMercator, EPSG_8803 } from 'gis-tools-ts';
// Create a transform using a source and destination projection
const transform = new Transformer();
// inject all default definition projections. This is not memory efficient but ensures all
// projections are available
injectAllDefinitions(transform);
// Or add a specific projection
transform.insertDefinition(HotineObliqueMercator);
// inject all common EPSG codes. This is not memory efficient but ensures all EPSG codes are available
injectAllEPSGCodes(transform);
// Or add a specific EPSG code
transform.insertEPSGCode('EPSG_4326', EPSG_4326);
// If the transform requires a grid, this is how you add it.
transform.addGridFromReader(
  'BETA2007.gsb',
  new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
);
// Set the source and destination projections
transform.setSource('EPSG_31466');
transform.setDestination('EPSG_25832');
// example forward projection
const forward = transform.forward({ x: 2559552, y: 5670982 });
// example inverse projection
const inverse = transform.inverse({ x: 349757.381712518, y: 5671004.06504954 });
```

### Minimal Example only adding the Hotine Oblique Mercator

It's often recommended to prioritize this style of Transformer creation as it dramatically reduces the size of the bundle.

```ts
import { Transformer, HotineObliqueMercator, EPSG_8803 } from 'gis-tools-ts';
 *
const transform = new Transformer(undefined, 'EPSG_8803', [HotineObliqueMercator], { EPSG_8803 });
// example moving from WGS84 to EPSG:8803
const forward = transform.forward({ x: 60.8, y: -132.2 });
// example moving from EPSG:8803 to WGS84
const inverse = transform.inverse(forward);
```

## Useful Links

When using EPSG codes, you can study up on them using the [TS documentation](https://open-s2.github.io/gis-tools/modules/index.EPSG_CODES.html). When an error is thrown because of a missing definition, the EPSG code should be provided. Use this link with the search feature to find the correct definition to import.

Often missing projection errors will provide the short name of the projection. So instead of `HotineObliqueMercator` it will probably report `omerc` is missing. Example:

```sh
Error: "omerc" invalid, unsupported, or not loaded
```

Refer to [projections docs](./projections.md) for more guidance.
