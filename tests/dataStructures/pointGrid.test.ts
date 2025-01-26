import { RasterTilesFileReader } from '../../src/file';
import { PointGrid, fromFace } from '../../src';
import { expect, test } from 'bun:test';

// import sharp from 'sharp';

import type { RGBA } from '../../src';

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

// NOTE: We skip tests in this file because they are just too slow. But we know they work,
// and we can re-add if we start messing with pointGrid again.

testFunc(
  'pointGrid - WM from WM tiles',
  async () => {
    const grid = new PointGrid<RGBA>({
      projection: 'WM',
      minzoom: 0,
      maxzoom: 1,
      bufferSize: 0,
      maxzoomInterpolation: 'idw',
      getInterpolationValue: 'rgba',
    });
    const reader = new RasterTilesFileReader<RGBA>(
      `${__dirname}/../readers/tile/fixtures/wm/satellite`,
      1,
    );
    await grid.insertReader(reader);
    grid.insertLonLat({ x: 0, y: 0, m: { r: 255, g: 0, b: 0, a: 255 } });
    grid.insertFaceST(0, 0, 0, { r: 0, g: 255, b: 0, a: 255 });
    await grid.buildClusters();

    const tile0 = await grid.getTile(fromFace(0));
    if (tile0 === undefined) throw new Error('Tile is undefined');
    const data = tile0.data as RGBA[];
    const image = data.flatMap(({ r, g, b, a }) => [r, g, b, a]);
    expect(new Uint8ClampedArray(image)).toMatchSnapshot(
      `${__dirname}/fixtures/wm/wmTileFromWM.png`,
    );
    // await sharp(new Uint8ClampedArray(image), {
    //   raw: {
    //     width: 512,
    //     height: 512,
    //     channels: 4,
    //   },
    // })
    //   .png()
    //   .toFile(`${__dirname}/fixtures/wm/wmTileFromWM.png`);
  },
  40_000,
);

test.skip('pointGrid - WM from S2 tiles', async () => {
  const grid = new PointGrid<RGBA>({
    projection: 'WM',
    minzoom: 0,
    maxzoom: 1,
    bufferSize: 0,
    maxzoomInterpolation: 'idw',
    getInterpolationValue: 'rgba',
  });
  const reader = new RasterTilesFileReader<RGBA>(
    `${__dirname}/../readers/tile/fixtures/s2/modis-mini`,
    0,
  );
  await grid.insertReader(reader);
  await grid.buildClusters();

  const tile0 = await grid.getTile(fromFace(0));
  if (tile0 === undefined) throw new Error('Tile is undefined');
  const data = tile0.data as RGBA[];
  const image = data.flatMap(({ r, g, b, a }) => [r, g, b, a]);
  expect(new Uint8ClampedArray(image)).toMatchSnapshot(`${__dirname}/fixtures/wm/wmTileFromS2.png`);
  // await sharp(new Uint8ClampedArray(image), {
  //   raw: {
  //     width: 512,
  //     height: 512,
  //     channels: 4,
  //   },
  // })
  //   .png()
  //   .toFile(`${__dirname}/fixtures/wm/wmTileFromS2.png`);
}, 40_000);

test.skip('pointGrid - S2 from S2 tiles', async () => {
  const grid = new PointGrid<RGBA>({
    projection: 'S2',
    minzoom: 0,
    maxzoom: 0,
    getInterpolationValue: 'rgba',
    bufferSize: 0,
  });
  const reader = new RasterTilesFileReader<RGBA>(
    `${__dirname}/../readers/tile/fixtures/s2/modis-mini`,
    0,
  );
  await grid.insertReader(reader);
  await grid.buildClusters();

  const tile0 = await grid.getTile(fromFace(1));
  if (tile0 === undefined) throw new Error('Tile is undefined');
  const data = tile0.data as RGBA[];
  const image = data.flatMap(({ r, g, b, a }) => [r, g, b, a]);
  expect(new Uint8ClampedArray(image)).toMatchSnapshot(`${__dirname}/fixtures/s2/S2TileFromS2.png`);
  // await sharp(new Uint8ClampedArray(image), {
  //   raw: {
  //     width: 512,
  //     height: 512,
  //     channels: 4,
  //   },
  // })
  //   .png()
  //   .toFile(`${__dirname}/fixtures/s2/S2TileFromS2.png`);
}, 40_000);

test.skip('pointGrid - S2 from WM tiles', async () => {
  const grid = new PointGrid<RGBA>({
    projection: 'S2',
    minzoom: 0,
    maxzoom: 0,
    getInterpolationValue: 'rgba',
    bufferSize: 0,
  });
  const reader = new RasterTilesFileReader<RGBA>(
    `${__dirname}/../readers/tile/fixtures/wm/satellite`,
    1,
  );
  await grid.insertReader(reader);
  await grid.buildClusters();

  const tile0 = await grid.getTile(fromFace(1));
  if (tile0 === undefined) throw new Error('Tile is undefined');
  const data = tile0.data as RGBA[];
  const image = data.flatMap(({ r, g, b, a }) => [r, g, b, a]);
  expect(new Uint8ClampedArray(image)).toMatchSnapshot(`${__dirname}/fixtures/s2/S2TileFromWM.png`);
  // await sharp(new Uint8ClampedArray(image), {
  //   raw: {
  //     width: 512,
  //     height: 512,
  //     channels: 4,
  //   },
  // })
  //   .png()
  //   .toFile(`${__dirname}/fixtures/s2/S2TileFromWM.png`);
}, 40_000);
