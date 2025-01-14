import { buildServer } from '../../server';
import { RasterTilesReader, S2PMTilesReader } from '../../../src';
import { expect, test } from 'bun:test';

import { RasterTilesFileReader } from '../../../src/file';

test('read in wm satellite', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/satellite`,
    1,
  );

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    // @ts-expect-error - ignore for now
    bounds: [-180, -85, 180, 85],
    encoding: 'none',
    extension: 'webp',
    format: 'zxy',
    maxzoom: 3,
    minzoom: 0,
    name: 'Mapbox Satellite',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'raster',
  });

  // has tile
  expect(await reader.hasTileWM(0, 0, 0)).toEqual(true);
  const tile = await reader.getTileWM(0, 0, 0);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, zoom, x, y, tmsStyle } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(zoom).toEqual(0);
  expect(x).toEqual(0);
  expect(y).toEqual(0);
  expect(tmsStyle).toEqual(false);

  const tileData = await Array.fromAsync(tile);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(0, 5)).toEqual([
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -179.6484375,
      y: 84.88501329390853,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -178.9453125,
      y: 84.88501329390853,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -178.2421875,
      y: 84.88501329390853,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -177.5390625,
      y: 84.88501329390853,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -176.8359375,
      y: 84.88501329390853,
    },
  ]);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(-5)).toEqual([
    {
      m: { a: 255, b: 244, g: 239, r: 240 },
      x: 176.8359375,
      y: -84.88501329390854,
    },
    {
      m: { a: 255, b: 253, g: 248, r: 249 },
      x: 177.5390625,
      y: -84.88501329390854,
    },
    {
      m: { a: 255, b: 251, g: 246, r: 247 },
      x: 178.2421875,
      y: -84.88501329390854,
    },
    {
      m: { a: 255, b: 255, g: 255, r: 255 },
      x: 178.9453125,
      y: -84.88501329390854,
    },
    {
      m: { a: 255, b: 245, g: 240, r: 241 },
      x: 179.6484375,
      y: -84.88501329390854,
    },
  ]);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  await server.stop();
});

test('read in wm satellite file', async () => {
  const server = buildServer();
  const reader = new RasterTilesFileReader(`${__dirname}/fixtures/wm/satellite`, 1);

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    // @ts-expect-error - ignore for now
    bounds: [-180, -85, 180, 85],
    encoding: 'none',
    extension: 'webp',
    format: 'zxy',
    maxzoom: 3,
    minzoom: 0,
    name: 'Mapbox Satellite',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'raster',
  });

  // has tile
  expect(await reader.hasTileWM(0, 0, 0)).toEqual(true);
  const tile = await reader.getTileWM(0, 0, 0);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, zoom, x, y, tmsStyle } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(zoom).toEqual(0);
  expect(x).toEqual(0);
  expect(y).toEqual(0);
  expect(tmsStyle).toEqual(false);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  await server.stop();
});

test('read in s2 modis-mini', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/s2/modis-mini`,
  );

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    attributions: {
      MODIS: 'https://modis.gsfc.nasa.gov',
    },
    description: 'NASA Modis Dataset Reprojected by S2 MAPS INC.',
    encoding: 'none',
    extension: 'webp',
    faces: [0, 1, 2, 3, 4, 5],
    facesbounds: {
      '0': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '1': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '2': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '3': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '4': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '5': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
    },
    format: 's2',
    layers: {},
    maxzoom: 1,
    minzoom: 0,
    name: 'Modis Raster Dataset',
    s2tilejson: '1.0.0',
    tilestats: {
      '0': 1365,
      '1': 1365,
      '2': 1365,
      '3': 1365,
      '4': 1365,
      '5': 1365,
      total: 8190,
    },
    scheme: 'fzxy',
    type: 'raster',
    // @ts-expect-error - just old spec
    version: 1,
  });

  // has tile
  expect(await reader.hasTileS2(0, 0, 0, 0)).toEqual(true);
  const tile = await reader.getTileS2(0, 0, 0, 0);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, face, zoom, x, y } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(face).toEqual(0);
  expect(zoom).toEqual(0);
  expect(x).toEqual(0);
  expect(y).toEqual(0);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(24);

  await server.stop();
});

test('read in s2 modis-mini - file', async () => {
  const server = buildServer();
  const reader = new RasterTilesFileReader(`${__dirname}/fixtures/s2/modis-mini`, 0);

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    attributions: {
      MODIS: 'https://modis.gsfc.nasa.gov',
    },
    description: 'NASA Modis Dataset Reprojected by S2 MAPS INC.',
    encoding: 'none',
    extension: 'webp',
    faces: [0, 1, 2, 3, 4, 5],
    facesbounds: {
      '0': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '1': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '2': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '3': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '4': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
      '5': {
        '0': [0, 0, 0, 0],
        '1': [0, 0, 1, 1],
        '2': [0, 0, 3, 3],
        '3': [0, 0, 7, 7],
        '4': [0, 0, 15, 15],
        '5': [0, 0, 31, 31],
      },
    },
    format: 's2',
    layers: {},
    maxzoom: 1,
    minzoom: 0,
    name: 'Modis Raster Dataset',
    s2tilejson: '1.0.0',
    tilestats: {
      '0': 1365,
      '1': 1365,
      '2': 1365,
      '3': 1365,
      '4': 1365,
      '5': 1365,
      total: 8190,
    },
    scheme: 'fzxy',
    type: 'raster',
    // @ts-expect-error - just old spec
    version: 1,
  });

  // has tile
  expect(await reader.hasTileS2(0, 0, 0, 0)).toEqual(true);
  const tile = await reader.getTileS2(0, 0, 0, 0);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, face, zoom, x, y } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(face).toEqual(0);
  expect(zoom).toEqual(0);
  expect(x).toEqual(0);
  expect(y).toEqual(0);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(6);

  await server.stop();
});

test('read in pmtiles s2', async () => {
  const pmtiles = new S2PMTilesReader(
    await Bun.file(`${__dirname}/../pmtiles/fixtures/s2.s2pmtiles`).arrayBuffer(),
  );
  const reader = new RasterTilesReader(pmtiles);

  const metadata = await reader.getMetadata();
  // @ts-expect-error - just for testing
  expect(metadata).toEqual({ metadata: true });
  expect(await reader.hasTileS2(0, 0, 0, 0)).toEqual(true);

  // TODO: We need a pmtiles using S2 that has raster data in it.
  // const tile = await reader.getTileS2(0, 0, 0, 0);
  // console.log(tile);
});

test('read in pmtiles wm', async () => {
  const pmtiles = new S2PMTilesReader(
    await Bun.file(`${__dirname}/../pmtiles/fixtures/test_fixture_1.pmtiles`).arrayBuffer(),
  );
  const reader = new RasterTilesReader(pmtiles);

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    description: 'test_fixture_1.pmtiles',
    generator: 'tippecanoe v2.5.0',
    generator_options: './tippecanoe -zg -o test_fixture_1.pmtiles --force',
    name: 'test_fixture_1.pmtiles',
    tilestats: {
      // @ts-expect-error - just for testing
      layerCount: 1,
      layers: [
        {
          attributeCount: 0,
          attributes: [],
          count: 1,
          geometry: 'Polygon',
          layer: 'test_fixture_1pmtiles',
        },
      ],
    },
    type: 'overlay',
    vector_layers: [
      {
        description: '',
        fields: {},
        id: 'test_fixture_1pmtiles',
        maxzoom: 0,
        minzoom: 0,
      },
    ],
    version: '2',
  });
  expect(await reader.hasTileWM(0, 0, 0)).toEqual(true);

  // TODO: We need a pmtiles using WM that has raster data in it.
  // const tile = await reader.getTileS2(0, 0, 0, 0);
  // console.log(tile);
});
