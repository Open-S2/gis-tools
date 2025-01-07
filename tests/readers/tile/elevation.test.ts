import { buildServer } from '../../server';
import {
  RasterTilesReader,
  convertMapboxElevationData,
  convertTerrariumElevationData,
} from '../../../src';
import { expect, test } from 'bun:test';

test('convertMapboxElevationData', () => {
  expect(convertMapboxElevationData(0, 0, 0)).toBe(-10000);
  expect(convertMapboxElevationData(255, 255, 255)).toBe(1667721.5);
  expect(convertMapboxElevationData(0, 0, 255)).toBe(-9974.5);
  expect(convertMapboxElevationData(255, 0, 0)).toBe(1661168);
  expect(convertMapboxElevationData(0, 255, 0)).toBe(-3472);
});

test('convertTerrariumElevationData', () => {
  expect(convertTerrariumElevationData(0, 0, 0)).toBe(-32768);
  expect(convertTerrariumElevationData(255, 255, 255)).toBe(32767.99609375);
  expect(convertTerrariumElevationData(0, 0, 255)).toBe(-32767.00390625);
  expect(convertTerrariumElevationData(255, 0, 0)).toBe(32512);
  expect(convertTerrariumElevationData(0, 255, 0)).toBe(-32513);
});

test('read in wm terrain', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/terrain`,
    1,
    convertMapboxElevationData,
  );

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    attributions: { '© Mapbox': 'https://www.mapbox.com/about/maps/' },
    // @ts-expect-error - ignore for testing
    bounds: [-180, -85, 180, 85],
    description: 'height = -10000 + ((R * 256 * 256 + G * 256 + B) * 0.1)',
    encoding: 'none',
    extension: 'webp',
    maxzoom: 3,
    minzoom: 0,
    name: 'Terrain-DEM',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'raster',
  });

  // has tile
  expect(await reader.hasTile(0, 0, 0)).toEqual(true);
  const tile = await reader.getTile(3, 6, 2);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, zoom, x, y, tmsStyle } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(zoom).toEqual(3);
  expect(x).toEqual(6);
  expect(y).toEqual(2);
  expect(tmsStyle).toEqual(false);
  const tileData = await Array.fromAsync(tile);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(0, 5)).toEqual([
    { m: { elev: 344 }, x: 90.0439453125, y: 66.48832551891898 },
    { m: { elev: 276 }, x: 90.1318359375, y: 66.48832551891898 },
    { m: { elev: 348 }, x: 90.2197265625, y: 66.48832551891898 },
    { m: { elev: 374 }, x: 90.3076171875, y: 66.48832551891898 },
    { m: { elev: 526 }, x: 90.3955078125, y: 66.48832551891898 },
  ]);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  await server.stop();
});

test('read in wm terrain-v2', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/terrain-v2`,
    1,
    convertMapboxElevationData,
  );

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    attributions: { '© Mapbox': 'https://www.mapbox.com/about/maps/' },
    // @ts-expect-error - ignore for testing
    bounds: [-180, -85, 180, 85],
    description: 'height = -10000 + ((R * 256 * 256 + G * 256 + B) * 0.1)',
    encoding: 'none',
    extension: 'webp',
    maxzoom: 3,
    minzoom: 0,
    format: 'webp',
    name: 'Terrain-DEM',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'raster',
  });

  // has tile
  expect(await reader.hasTile(0, 0, 0)).toEqual(true);
  const tile = await reader.getTile(3, 6, 2);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, zoom, x, y, tmsStyle } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(zoom).toEqual(3);
  expect(x).toEqual(6);
  expect(y).toEqual(2);
  expect(tmsStyle).toEqual(false);
  const tileData = await Array.fromAsync(tile);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(0, 5)).toEqual([
    { m: { elev: 291.2000000000007 }, x: 90.0439453125, y: 66.48832551891898 },
    { m: { elev: 265.60000000000036 }, x: 90.1318359375, y: 66.48832551891898 },
    { m: { elev: 316.8000000000011 }, x: 90.2197265625, y: 66.48832551891898 },
    { m: { elev: 419.2000000000007 }, x: 90.3076171875, y: 66.48832551891898 },
    { m: { elev: 521.6000000000004 }, x: 90.3955078125, y: 66.48832551891898 },
  ]);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  await server.stop();
});

test('read in wm terrarium2x', async () => {
  const server = buildServer();
  const reader = new RasterTilesReader(
    `http://localhost:${server.port}/readers/tile/fixtures/wm/terrarium2x`,
    1,
    convertTerrariumElevationData,
  );

  const metadata = await reader.getMetadata();
  expect(metadata).toEqual({
    attributions: { Terrarium: 'https://registry.opendata.aws/terrain-tiles/' },
    // @ts-expect-error - ignore for testing
    bounds: [-180, -85, 180, 85],
    description: 'height = (R * 256. + G + B / 256.) - 32768.',
    encoding: 'none',
    extension: 'webp',
    maxzoom: 3,
    minzoom: 0,
    format: 'webp',
    name: 'Terrain-DEM',
    s2tilejson: '1.0.0',
    scheme: 'xyz',
    type: 'raster',
  });

  // has tile
  expect(await reader.hasTile(0, 0, 0)).toEqual(true);
  const tile = await reader.getTile(3, 6, 2);
  expect(tile).toBeDefined();
  if (tile === undefined) throw Error('tile is undefined');
  const { image, zoom, x, y, tmsStyle } = tile;
  expect(image.width).toEqual(512);
  expect(image.height).toEqual(512);
  expect(zoom).toEqual(3);
  expect(x).toEqual(6);
  expect(y).toEqual(2);
  expect(tmsStyle).toEqual(false);
  const tileData = await Array.fromAsync(tile);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(0, 5)).toEqual([
    { m: { elev: 346 }, x: 90.0439453125, y: 66.48832551891898 },
    { m: { elev: 280 }, x: 90.1318359375, y: 66.48832551891898 },
    { m: { elev: 392 }, x: 90.2197265625, y: 66.48832551891898 },
    { m: { elev: 520 }, x: 90.3076171875, y: 66.48832551891898 },
    { m: { elev: 549 }, x: 90.3955078125, y: 66.48832551891898 },
  ]);
  // @ts-expect-error - for testing
  expect(tileData[0].geometry.coordinates.slice(-5)).toEqual([
    { m: { elev: -3568 }, x: 134.6044921875, y: 41.004832993813 },
    { m: { elev: -3576 }, x: 134.6923828125, y: 41.004832993813 },
    { m: { elev: -3568 }, x: 134.7802734375, y: 41.004832993813 },
    { m: { elev: -3550 }, x: 134.8681640625, y: 41.004832993813 },
    { m: { elev: -3539 }, x: 134.9560546875, y: 41.004832993813 },
  ]);

  const tiles = await Array.fromAsync(reader);
  expect(tiles.length).toEqual(4);

  await server.stop();
});
