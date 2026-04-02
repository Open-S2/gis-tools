import { buildServer } from '../../server.js';
import sharp from 'sharp';
import { RasterTilesReader, S2PMTilesReader, buildTileGridWM } from '../../../src/index.js';
import { describe, expect, test } from 'bun:test';

import { RasterTilesFileReader } from '../../../src/file.js';

import type { SharpOptions } from 'sharp';

test('read in wm satellite', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/satellite`,
    1,
  );

  const metadata = await reader.getMetadata();
  // @ts-expect-error - ignore for now
  expect(metadata).toEqual({
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
      y: 85.02070774312593,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -178.94531249999994,
      y: 85.02070774312593,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -178.24218749999997,
      y: 85.02070774312593,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -177.53906249999997,
      y: 85.02070774312593,
    },
    {
      m: { a: 255, b: 28, g: 14, r: 8 },
      x: -176.83593749999994,
      y: 85.02070774312593,
    },
  ]);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(-5)).toEqual([
    {
      m: { a: 255, b: 244, g: 239, r: 240 },
      x: 176.8359375,
      y: -85.02070774312594,
    },
    {
      m: { a: 255, b: 253, g: 248, r: 249 },
      x: 177.53906249999997,
      y: -85.02070774312594,
    },
    {
      m: { a: 255, b: 251, g: 246, r: 247 },
      x: 178.24218749999997,
      y: -85.02070774312594,
    },
    {
      m: { a: 255, b: 255, g: 255, r: 255 },
      x: 178.94531250000003,
      y: -85.02070774312594,
    },
    {
      m: { a: 255, b: 245, g: 240, r: 241 },
      x: 179.6484375,
      y: -85.02070774312594,
    },
  ]);

  // lon lat lookup
  const lonLatValue = await reader.getLonLatValuesWM(2, 20, -20);
  expect(lonLatValue).toEqual({ r: 146, g: 123, b: 56, a: 255 });

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  const iterTiles = await Array.fromAsync(reader.iterate());
  expect(iterTiles.length).toEqual(5);
  expect(iterTiles).toEqual([
    { x: 0, y: 0, zoom: 0 },
    { x: 1, y: 1, zoom: 1 },
    { x: 0, y: 1, zoom: 1 },
    { x: 1, y: 0, zoom: 1 },
    { x: 0, y: 0, zoom: 1 },
  ]);

  await server.stop();
});

test('read in wm satellite file', async () => {
  const reader = new RasterTilesFileReader(`${__dirname}/fixtures/wm/satellite`, 1);

  const metadata = await reader.getMetadata();
  // @ts-expect-error - ignore for now
  expect(metadata).toEqual({
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

  const iterTiles = await Array.fromAsync(reader.iterate());
  expect(iterTiles.length).toEqual(4);
  expect(iterTiles).toEqual([
    { x: 0, y: 1, zoom: 1 },
    { x: 0, y: 0, zoom: 1 },
    { x: 1, y: 1, zoom: 1 },
    { x: 1, y: 0, zoom: 1 },
  ]);
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
    // @ts-expect-error - ignore for now
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
    version: '1.0.0',
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

  // lon lat lookup
  const lonLatValue = await reader.getLonLatValuesS2(2, 20, -20);
  expect(lonLatValue).toEqual({ r: 93, g: 80, b: 46, a: 255 });

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(24);

  const iterTiles = await Array.fromAsync(reader.iterate());
  expect(iterTiles.length).toEqual(30);
  expect(iterTiles).toEqual([
    { face: 0, x: 0, y: 0, zoom: 0 },
    { face: 0, x: 1, y: 1, zoom: 1 },
    { face: 0, x: 0, y: 1, zoom: 1 },
    { face: 0, x: 1, y: 0, zoom: 1 },
    { face: 0, x: 0, y: 0, zoom: 1 },
    { face: 1, x: 0, y: 0, zoom: 0 },
    { face: 1, x: 1, y: 1, zoom: 1 },
    { face: 1, x: 0, y: 1, zoom: 1 },
    { face: 1, x: 1, y: 0, zoom: 1 },
    { face: 1, x: 0, y: 0, zoom: 1 },
    { face: 2, x: 0, y: 0, zoom: 0 },
    { face: 2, x: 1, y: 1, zoom: 1 },
    { face: 2, x: 0, y: 1, zoom: 1 },
    { face: 2, x: 1, y: 0, zoom: 1 },
    { face: 2, x: 0, y: 0, zoom: 1 },
    { face: 3, x: 0, y: 0, zoom: 0 },
    { face: 3, x: 1, y: 1, zoom: 1 },
    { face: 3, x: 0, y: 1, zoom: 1 },
    { face: 3, x: 1, y: 0, zoom: 1 },
    { face: 3, x: 0, y: 0, zoom: 1 },
    { face: 4, x: 0, y: 0, zoom: 0 },
    { face: 4, x: 1, y: 1, zoom: 1 },
    { face: 4, x: 0, y: 1, zoom: 1 },
    { face: 4, x: 1, y: 0, zoom: 1 },
    { face: 4, x: 0, y: 0, zoom: 1 },
    { face: 5, x: 0, y: 0, zoom: 0 },
    { face: 5, x: 1, y: 1, zoom: 1 },
    { face: 5, x: 0, y: 1, zoom: 1 },
    { face: 5, x: 1, y: 0, zoom: 1 },
    { face: 5, x: 0, y: 0, zoom: 1 },
  ]);

  await server.stop();
});

test('read in s2 modis-mini - file', async () => {
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
    // @ts-expect-error - ignore for now
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
    version: '1.0.0',
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

  const iterTiles = await Array.fromAsync(reader.iterate());
  expect(iterTiles.length).toEqual(6);
  expect(iterTiles).toEqual([
    { face: 0, x: 0, y: 0, zoom: 0 },
    { face: 1, x: 0, y: 0, zoom: 0 },
    { face: 2, x: 0, y: 0, zoom: 0 },
    { face: 3, x: 0, y: 0, zoom: 0 },
    { face: 4, x: 0, y: 0, zoom: 0 },
    { face: 5, x: 0, y: 0, zoom: 0 },
  ]);
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

describe('buildTileGridWM', () => {
  test('base case', () => {
    const gridGuide = buildTileGridWM({ zoom: 0, x: 0, y: 0 }, 0, 512, 512, false);
    expect(gridGuide).toEqual([
      {
        destOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        srcOffsets: [0, 0],
        writeSize: [512, 512],
      },
    ]);
  });
  test('base case double size', () => {
    const gridGuide = buildTileGridWM({ zoom: 0, x: 0, y: 0 }, 0, 256, 512, false);
    expect(gridGuide).toEqual([
      {
        destOffsets: [0, 0],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 1 },
        writeSize: [256, 256],
      },
      {
        destOffsets: [0, 256],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 1, zoom: 1 },
        writeSize: [256, 256],
      },
      {
        destOffsets: [256, 0],
        srcOffsets: [0, 0],
        tile: { x: 1, y: 0, zoom: 1 },
        writeSize: [256, 256],
      },
      {
        destOffsets: [256, 256],
        srcOffsets: [0, 0],
        tile: { x: 1, y: 1, zoom: 1 },
        writeSize: [256, 256],
      },
    ]);
  });
  test('small padding zoom 0', () => {
    const gridGuide = buildTileGridWM({ zoom: 0, x: 0, y: 0 }, 2, 512, 512, false);
    expect(gridGuide).toEqual([
      {
        destOffsets: [2, 2],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [512, 512],
      },
      {
        clamp: true,
        destOffsets: [2, 0],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [512, 2],
      },
      {
        clamp: true,
        destOffsets: [2, 514],
        srcOffsets: [0, 511],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [512, 2],
      },
      {
        destOffsets: [0, 2],
        srcOffsets: [510, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 512],
      },
      {
        clamp: true,
        destOffsets: [0, 0],
        srcOffsets: [510, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 2],
      },
      {
        clamp: true,
        destOffsets: [0, 514],
        srcOffsets: [510, 511],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 2],
      },
      {
        destOffsets: [514, 2],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 512],
      },
      {
        clamp: true,
        destOffsets: [514, 0],
        srcOffsets: [0, 0],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 2],
      },
      {
        clamp: true,
        destOffsets: [514, 514],
        srcOffsets: [0, 511],
        tile: { x: 0, y: 0, zoom: 0 },
        writeSize: [2, 2],
      },
    ]);
  });

  test('higher zoom small padding', () => {
    const gridGuide = buildTileGridWM({ zoom: 3, x: 2, y: 2 }, 4, 512, 512, false);
    expect(gridGuide).toEqual([
      {
        destOffsets: [4, 4],
        srcOffsets: [0, 0],
        tile: { x: 2, y: 2, zoom: 3 },
        writeSize: [512, 512],
      },
      {
        clamp: false,
        destOffsets: [4, 0],
        srcOffsets: [0, 508],
        tile: { x: 2, y: 1, zoom: 3 },
        writeSize: [512, 4],
      },
      {
        clamp: false,
        destOffsets: [4, 516],
        srcOffsets: [0, 0],
        tile: { x: 2, y: 3, zoom: 3 },
        writeSize: [512, 4],
      },
      {
        destOffsets: [0, 4],
        srcOffsets: [508, 0],
        tile: { x: 1, y: 2, zoom: 3 },
        writeSize: [4, 512],
      },
      {
        clamp: false,
        destOffsets: [0, 0],
        srcOffsets: [508, 508],
        tile: { x: 1, y: 1, zoom: 3 },
        writeSize: [4, 4],
      },
      {
        clamp: false,
        destOffsets: [0, 516],
        srcOffsets: [508, 0],
        tile: { x: 1, y: 3, zoom: 3 },
        writeSize: [4, 4],
      },
      {
        destOffsets: [516, 4],
        srcOffsets: [0, 0],
        tile: { x: 3, y: 2, zoom: 3 },
        writeSize: [4, 512],
      },
      {
        clamp: false,
        destOffsets: [516, 0],
        srcOffsets: [0, 508],
        tile: { x: 3, y: 1, zoom: 3 },
        writeSize: [4, 4],
      },
      {
        clamp: false,
        destOffsets: [516, 516],
        srcOffsets: [0, 0],
        tile: { x: 3, y: 3, zoom: 3 },
        writeSize: [4, 4],
      },
    ]);
  });
});

test('getTileWithPaddingWM - fetch', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/satellite`,
  );

  // BASE CASE
  const tile = await reader.getTileWithPaddingWM(0, 0, 0, 1, 512, 512);
  expect(tile).toBeDefined();
  const sharpOptions: SharpOptions = { raw: { width: 514, height: 514, channels: 4 } };
  const pngData = await sharp(tile!.image.data, sharpOptions).png().toBuffer();
  // uncomment to store the image
  // await Bun.write(`${__dirname}/fixtures/wm/satellite/baseCase.png`, pngData);
  const expectedPngData = Buffer.from(
    await Bun.file(`${__dirname}/fixtures/wm/satellite/baseCase.png`).arrayBuffer(),
  );
  expect(pngData).toEqual(expectedPngData);

  await server.stop();
});

test('getTileWithPaddingWM', async () => {
  // const reader = new RasterTilesFileReader(`${__dirname}/fixtures/wm/satellite`);

  // // BASE CASE
  // const tile = await reader.getTileWithPaddingWM(0, 0, 0, 1, 512, 512);
  // expect(tile).toBeDefined();
  // const sharpOptions: SharpOptions = { raw: { width: 514, height: 514, channels: 4 } };
  // const pngData = await sharp(tile!.image.data, sharpOptions).png().toBuffer();
  // // uncomment to store the image
  // // await Bun.write(`${__dirname}/fixtures/wm/satellite/baseCase.png`, pngData);
  // const expectedPngData = Buffer.from(
  //   await Bun.file(`${__dirname}/fixtures/wm/satellite/baseCase.png`).arrayBuffer(),
  // );
  // expect(pngData).toEqual(expectedPngData);

  // // Large padding
  // const tile2 = await reader.getTileWithPaddingWM(0, 0, 0, 16, 512, 512);
  // expect(tile2).toBeDefined();
  // const sharpOptions2: SharpOptions = { raw: { width: 544, height: 544, channels: 4 } };
  // const pngData2 = await sharp(tile2!.image.data, sharpOptions2).png().toBuffer();
  // // uncomment to store the image
  // // await Bun.write(`${__dirname}/fixtures/wm/satellite/largerPadding.png`, pngData2);
  // const expectedPngData2 = Buffer.from(
  //   await Bun.file(`${__dirname}/fixtures/wm/satellite/largerPadding.png`).arrayBuffer(),
  // );
  // expect(pngData2).toEqual(expectedPngData2);

  // // Wrapping lower zoom
  // const tile3 = await reader.getTileWithPaddingWM(2, 0, 0, 16, 512, 512);
  // expect(tile3).toBeDefined();
  // const sharpOptions3: SharpOptions = { raw: { width: 544, height: 544, channels: 4 } };
  // const pngData3 = await sharp(tile3!.image.data, sharpOptions3).png().toBuffer();
  // // uncomment to store the image
  // // await Bun.write(`${__dirname}/fixtures/wm/satellite/wrappingZoom.png`, pngData3);
  // const expectedPngData3 = Buffer.from(
  //   await Bun.file(`${__dirname}/fixtures/wm/satellite/wrappingZoom.png`).arrayBuffer(),
  // );
  // expect(pngData3).toEqual(expectedPngData3);

  // // Wrapping lower zoom other end
  // const tile4 = await reader.getTileWithPaddingWM(2, 3, 3, 16, 512, 512);
  // expect(tile4).toBeDefined();
  // const sharpOptions4: SharpOptions = { raw: { width: 544, height: 544, channels: 4 } };
  // const pngData4 = await sharp(tile4!.image.data, sharpOptions4).png().toBuffer();
  // // uncomment to store the image
  // // await Bun.write(`${__dirname}/fixtures/wm/satellite/wrappingZoom2.png`, pngData4);
  // const expectedPngData4 = Buffer.from(
  //   await Bun.file(`${__dirname}/fixtures/wm/satellite/wrappingZoom2.png`).arrayBuffer(),
  // );
  // expect(pngData4).toEqual(expectedPngData4);

  const reader2 = new RasterTilesFileReader(`${__dirname}/fixtures/wm/terrarium`);

  // BASE CASE with resizing
  const tile5 = await reader2.getTileWithPaddingWM(0, 0, 0, 1, 256, 512);
  expect(tile5).toBeDefined();
  const sharpOptions5: SharpOptions = { raw: { width: 514, height: 514, channels: 4 } };
  const pngData5 = await sharp(tile5!.image.data, sharpOptions5).png().toBuffer();
  // uncomment to store the image
  // await Bun.write(`${__dirname}/fixtures/wm/terrarium/resize.png`, pngData5);
  const expectedPngData5 = Buffer.from(
    await Bun.file(`${__dirname}/fixtures/wm/terrarium/resize.png`).arrayBuffer(),
  );
  expect(pngData5).toEqual(expectedPngData5);
});
