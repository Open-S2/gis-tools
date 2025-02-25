<h1 style="text-align: center;">
  <div align="center">Satellite Orbit Class Reader</div>
</h1>

<p align="center">
  <img src="../../assets/badges/sat-file.svg" alt="sat-file-ts">
  <img src="../../assets/badges/sat-gzip.svg" alt="sat-gzip-ts">
  <img src="../../assets/badges/sat-brotli.svg" alt="sat-brotli-ts">
</p>

## Description

The **Satellite Orbit Class Reader** is a lightweight TypeScript library for parsing and propagating satellite orbits using Two Line Element (TLE) sets. It provides a simple API to compute satellite positions and velocities at any given time using orbital mechanics.

## Usage

### Input TLE example

```txt
STARLINK-1007
1 44713C 19074A   23048.53451389 -.00009219  00000+0 -61811-3 0   482
2 44713  53.0512 157.2379 0001140  81.3827  74.7980 15.06382459    15
```

### Run example

```ts
import { Satellite } from 'gis-tools-ts';

const sat = new Satellite(tleString);
// get propagation at time
const { position, velocity } = sat.propagate(new Date());
```

## Useful links

- <https://en.wikipedia.org/wiki/Two-line_element_set>
- <https://celestrak.org/NORAD/documentation/tle-fmt.php>
- <https://www.space-track.org/documentation#tle-basic>
