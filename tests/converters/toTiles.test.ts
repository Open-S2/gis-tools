import { RasterTilesFileReader } from '../../src/file';
import { VectorTile } from 'open-vector-tile';
import { BufferTileWriter, convertMapboxElevationData, decompressStream, toTiles } from '../../src';
import { expect, test } from 'bun:test';

import { DrawType } from 's2-tilejson';

// import sharp from 'sharp';

import type { ElevationPoint, RGBA, VectorPoint } from '../../src';

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

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
