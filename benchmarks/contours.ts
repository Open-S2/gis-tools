import '../src/util/polyfills/local.js';
import * as d3 from 'd3-contour';
import { buildContours, getElevationGrid, isolineThresholds } from '../src/index.js';

console.info('isolines: ');

const elevationImage = await Bun.file(
  './tests/tools/isobands/fixtures/13_1556_3084.webp',
).arrayBuffer();

// D3 //

const startD3 = Bun.nanoseconds();
const { elevations: grid, min, max, width, height } = await getElevationGrid(elevationImage);
const thresholds = isolineThresholds(min, max, 100);

const _contours = d3
  .contours()
  .size([width, height])
  .thresholds(thresholds)(grid)
  .map(({ type, value, coordinates }) => {
    return {
      type: 'VectorFeature',
      geometry: {
        type,
        is3D: false,
        coordinates: coordinates.map((rings) => {
          return rings.map((points) => {
            return points.map(([x, y]) => ({ x: x / width, y: y / height }));
          });
        }),
      },
      properties: { elevation: value, elevationFt: value * 3.28084 },
    };
  });

const endD3 = Bun.nanoseconds();
console.info('D3: ', (endD3 - startD3) / 1e6);

// GIS TOOLS //

const startGISTools = Bun.nanoseconds();

const _isolines = await buildContours(elevationImage);

const endGISTools = Bun.nanoseconds();
console.info('GISTools: ', (endGISTools - startGISTools) / 1e6);
