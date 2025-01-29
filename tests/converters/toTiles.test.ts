import { VectorTile } from 'open-vector-tile';
import {
  BufferJSONReader,
  BufferTileWriter,
  JSONReader,
  convertMapboxElevationData,
  decompressStream,
  toTiles,
} from '../../src';
import { FileReader, RasterTilesFileReader } from '../../src/file';
import { expect, test } from 'bun:test';

import { DrawType } from 's2-tilejson';

// import sharp from 'sharp';

import type { ElevationPoint, RGBA, VectorPoint } from '../../src';

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

testFunc('toTiles - vector & cluster', async () => {
  const fileReaderPoints = new FileReader(`${__dirname}/../readers/json/fixtures/points.geojson`);
  const bufferJSON = new BufferJSONReader({
    type: 'FeatureCollection',
    features: [
      {
        type: 'Feature',
        properties: {
          name: 'polys',
          id: 100,
        },
        geometry: {
          coordinates: [
            [
              [-82.22210288919695, 43.20251833642999],
              [-101.93729667579208, 19.709540835851655],
              [-29.6482527916086, 6.24385238701565],
              [-82.22210288919695, 43.20251833642999],
            ],
          ],
          type: 'Polygon',
        },
      },
      {
        type: 'Feature',
        properties: {
          name: 'lines',
          id: 1336,
        },
        geometry: {
          coordinates: [
            [-13.292352825505162, 54.34883408204476],
            [36.83102287804303, 59.56941785818924],
            [50.34083898563978, 16.040052775278994],
            [76.38149901912357, 35.155968522292056],
          ],
          type: 'LineString',
        },
      },
      {
        type: 'Feature',
        properties: {
          name: 'points',
          id: 9,
        },
        geometry: {
          coordinates: [
            [4.037548985522477, 17.257322750025494],
            [160.037548985522477, -20.257322750025494],
          ],
          type: 'MultiPoint',
        },
      },
    ],
  });
  const jsonReader = new JSONReader(fileReaderPoints);
  const tileWriter = new BufferTileWriter();

  await toTiles({
    name: 'vector data',
    vectorSources: { clusters: jsonReader, geos: bufferJSON },
    format: 'open-s2',
    projection: 'S2',
    layerGuides: [
      {
        sourceName: 'geos',
        layerName: 'randoms',
        vectorGuide: {
          buildBBox: true,
          minzoom: 0,
          maxzoom: 5,
        },
        extent: 4_096,
        shape: {
          name: 'string',
          id: 'u64',
        },
        drawTypes: [DrawType.Points, DrawType.Lines, DrawType.Polys],
      },
      {
        sourceName: 'clusters',
        layerName: 'cities',
        clusterGuide: {
          minzoom: 0,
          maxzoom: 5,
        },
        extent: 4_096,
      },
    ],
    tileWriter,
  });

  const vectorTileFace3 = await tileWriter.tiles.get(`3/0/0/0`);
  if (vectorTileFace3 === undefined) throw Error('vectorTileFace3 is undefined');
  const tile = new VectorTile(vectorTileFace3);
  const { cities, randoms } = tile.layers;
  // cities
  expect(cities.length).toEqual(1);
  const feature0 = cities.feature(0);
  expect(feature0.properties).toEqual({ name: 'Melbourne' });
  expect(feature0.loadGeometry()).toEqual([{ x: 3847, y: 645 }]);
  // randoms
  expect(randoms.length).toEqual(1);
  const feature1 = randoms.feature(0);
  expect(feature1.properties).toEqual({ name: 'points', id: 9 });
  expect(feature1.loadGeometry()).toEqual([{ x: 3022, y: 1135 }]);
});

testFunc(
  'toTiles - Raster WM',
  async () => {
    const reader = new RasterTilesFileReader<RGBA>(
      `${__dirname}/../readers/tile/fixtures/wm/satellite`,
      1,
    );
    const tileWriter = new BufferTileWriter();

    await toTiles({
      name: 'Satellite Data',
      rasterSources: { satellite: reader },
      format: 'raster',
      projection: 'WM',
      extension: 'raw',
      attribution: {
        'Satellite Data': 'https://example.com',
      },
      layerGuides: [
        {
          sourceName: 'satellite',
          layerName: 'sat',
          outputType: 'raw',
          rasterGuide: {
            minzoom: 0,
            maxzoom: 1,
            bufferSize: 0,
            getInterpolationValue: 'rgba',
            nullValue: { r: 0, g: 0, b: 0, a: 255 },
          },
        },
      ],
      tileWriter,
    });

    const meta = await tileWriter.metadata;
    expect(meta).toEqual({
      attribution: {
        'Satellite Data': 'https://example.com',
      },
      bounds: {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
      },
      center: { lat: 0, lon: 0, zoom: 0 },
      description: 'Built by S2-Tools',
      encoding: 'none',
      extension: 'raw',
      faces: [0],
      facesbounds: { '0': {}, '1': {}, '2': {}, '3': {}, '4': {}, '5': {} },
      layers: {
        sat: {
          drawTypes: [DrawType.Raster],
          maxzoom: 1,
          minzoom: 0,
          shape: {},
        },
      },
      maxzoom: 1,
      minzoom: 0,
      name: 'Satellite Data',
      s2tilejson: '1.0.0',
      scheme: 'xyz',
      tilestats: { '0': 0, '1': 0, '2': 0, '3': 0, '4': 0, '5': 0, total: 5 },
      type: 'raster',
      vector_layers: [
        {
          fields: {},
          id: 'sat',
          maxzoom: 1,
          minzoom: 0,
        },
      ],
      version: '1.0.0',
    });

    const zeroTileRaster = tileWriter.tiles.get('0/0/0');
    if (zeroTileRaster === undefined) throw Error('zeroTileRaster is undefined');
    expect(new Uint8ClampedArray(zeroTileRaster)).toMatchSnapshot(
      `${__dirname}/fixtures/WMRasterTile.png`,
    );
    // await sharp(zeroTileRaster, {
    //   raw: {
    //     width: 512,
    //     height: 512,
    //     channels: 4,
    //   },
    // })
    //   .png()
    //   .toFile(`${__dirname}/fixtures/WMRasterTile.png`);
  },
  40_000,
);

testFunc(
  'toTiles - grid S2',
  async () => {
    const reader = new RasterTilesFileReader<ElevationPoint>(
      `${__dirname}/../readers/tile/fixtures/wm/terrain-v2`,
      1,
      convertMapboxElevationData,
    );
    const tileWriter = new BufferTileWriter();

    await toTiles({
      name: 'Elevation Data',
      gridSources: { elevation: reader },
      format: 'open-s2',
      projection: 'S2',
      extension: 'pbf',
      encoding: 'gz',
      layerGuides: [
        {
          sourceName: 'elevation',
          layerName: 'elev',
          extent: 8_192,
          gridGuide: {
            minzoom: 0,
            maxzoom: 0,
            bufferSize: 0,
            /**
             * Get the elevation value from the point
             * @param point - the input point
             * @returns - the elevation
             */
            getInterpolationValue: (point: VectorPoint<ElevationPoint>): number =>
              point.m?.elev ?? 0,
            nullValue: 0,
          },
        },
      ],
      tileWriter,
    });

    const meta = await tileWriter.metadata;
    expect(meta).toEqual({
      attribution: {},
      bounds: {},
      center: { lat: 0, lon: 0, zoom: 0 },
      description: 'Built by S2-Tools',
      encoding: 'gz',
      extension: 'pbf',
      faces: [5, 4, 3, 2, 1, 0],
      facesbounds: {
        '0': {
          '0': [0, 0, 0, 0],
        },
        '1': {
          '0': [0, 0, 0, 0],
        },
        '2': {
          '0': [0, 0, 0, 0],
        },
        '3': {
          '0': [0, 0, 0, 0],
        },
        '4': {
          '0': [0, 0, 0, 0],
        },
        '5': {
          '0': [0, 0, 0, 0],
        },
      },
      layers: {
        elev: {
          drawTypes: [DrawType.Grid],
          maxzoom: 0,
          minzoom: 0,
          shape: {},
        },
      },
      maxzoom: 0,
      minzoom: 0,
      name: 'Elevation Data',
      s2tilejson: '1.0.0',
      scheme: 'fzxy',
      tilestats: { '0': 1, '1': 1, '2': 1, '3': 1, '4': 1, '5': 1, total: 6 },
      type: 'vector',
      vector_layers: [
        {
          fields: {},
          id: 'elev',
          maxzoom: 0,
          minzoom: 0,
        },
      ],
      version: '1.0.0',
    });

    const zeroTileGrids = tileWriter.tiles.get('0/0/0/0');
    if (zeroTileGrids === undefined) throw Error('zeroTileGrids is undefined');
    const decompressedZerTileGrid = await decompressStream(zeroTileGrids, 'gzip');
    const tile = new VectorTile(decompressedZerTileGrid);
    const { elev } = tile.grids;
    expect(elev.name).toEqual('elev');
    expect(elev.extent).toEqual(8192);
    expect(elev.size).toEqual(512);
    expect(elev.min).toEqual(-92.80000305175781);
    expect(elev.max).toEqual(3030.39990234375);
  },
  40_000,
);
