import { RasterTilesFileReader } from '../../../src/file.js';
import sharp from 'sharp';
import {
  buildContours,
  buildTerrainMesh,
  generateHillshade,
  vectorizeHillshade,
} from '../../../src/index.js';
import { expect, test } from 'bun:test';

import type { SharpOptions } from 'sharp';

// import { toFlatGeometry } from '../../../src/index.js';

test('contours - base case', async () => {
  const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1556_3084.webp`).arrayBuffer();
  const isolines = await buildContours(
    elevationImage,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
  );

  // write as flattened geojson features instead of vector features
  // const flatFeatures = isolines.features.map((vf) => toFlatGeometry(vf));
  // await Bun.write(
  //   `${__dirname}/fixtures/tmp.json`,
  //   JSON.stringify({ type: 'FeatureCollection', features: flatFeatures }, null, 2),
  // );

  // uncomment to update
  // await Bun.write(
  //   `${__dirname}/fixtures/13_1556_3084_lines.json`,
  //   JSON.stringify(isolines, null, 2),
  // );

  const expectData = await Bun.file(`${__dirname}/fixtures/13_1556_3084_lines.json`).json();

  expect(isolines).toEqual(expectData);
});

test('contours - with padding', async () => {
  // Setup a tile reader
  const reader = new RasterTilesFileReader(
    `${__dirname}/../../readers/tile/fixtures/wm/terrain-v2`,
  );
  const metadata = await reader.getMetadata();
  const isTMS = metadata.scheme === 'tms';
  // read in the elevation data with a big bigger padding
  const tile = await reader.getTileWithPaddingWM(3, 3, 1, 16, 512, 512);
  expect(tile).toBeDefined();
  // build the isolines
  const isolines = await buildContours(tile!.image, undefined, 1_000, isTMS, 16, undefined);

  // write as flattened geojson features instead of vector features
  // const flatFeatures = isolines.features.map((vf) => toFlatGeometry(vf));
  // await Bun.write(
  //   `${__dirname}/fixtures/tmp.json`,
  //   JSON.stringify({ type: 'FeatureCollection', features: flatFeatures }, null, 2),
  // );

  // uncomment to update
  // await Bun.write(
  //   `${__dirname}/fixtures/terrain-v2_isolines.json`,
  //   JSON.stringify(isolines, null, 2),
  // );

  const expectData = await Bun.file(`${__dirname}/fixtures/terrain-v2_isolines.json`).json();
  expect(isolines).toEqual(expectData);
});

test('contours - lines from query', async () => {
  const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1544_3085.webp`).arrayBuffer();

  let isolines = await buildContours(elevationImage, undefined, 50, undefined, undefined);
  isolines = JSON.parse(JSON.stringify(isolines));

  // write as flattened geojson features instead of vector features
  // const flatFeatures = isolines.features.map((vf) => toFlatGeometry(vf));
  // await Bun.write(
  //   `${__dirname}/fixtures/tmp.json`,
  //   JSON.stringify({ type: 'FeatureCollection', features: flatFeatures }, null, 2),
  // );

  // uncomment to update
  // await Bun.write(
  //   `${__dirname}/fixtures/13_1544_3085_lines.json`,
  //   JSON.stringify(isolines, null, 2),
  // );

  const expectData = await Bun.file(`${__dirname}/fixtures/13_1544_3085_lines.json`).json();
  expect(isolines).toEqual(expectData);
});

test('terrain mesh', async () => {
  const elevationImage = await Bun.file(`${__dirname}/fixtures/fuji.png`).arrayBuffer();
  const { vertices, triangles } = await buildTerrainMesh(elevationImage, 500, undefined, undefined);

  expect([
    320, 64, 256, 128, 320, 128, 384, 128, 256, 0, 288, 160, 256, 192, 288, 192, 320, 192, 304, 176,
    256, 256, 288, 224, 352, 160, 320, 160, 512, 0, 384, 0, 128, 128, 128, 0, 64, 64, 64, 0, 0, 0,
    32, 32, 192, 192, 384, 384, 512, 256, 384, 256, 320, 320, 320, 256, 512, 512, 512, 128, 448,
    192, 384, 192, 128, 384, 256, 512, 256, 384, 0, 512, 128, 256, 64, 192, 0, 256, 64, 128, 32, 96,
    0, 128, 32, 64, 16, 48, 0, 64, 0, 32,
  ]).toEqual(vertices);

  expect([
    0, 1, 2, 3, 0, 2, 4, 1, 0, 5, 6, 7, 7, 8, 9, 5, 7, 9, 1, 6, 5, 6, 10, 11, 11, 8, 7, 6, 11, 7,
    12, 2, 13, 8, 12, 13, 3, 2, 12, 2, 1, 5, 13, 5, 9, 8, 13, 9, 2, 5, 13, 3, 14, 15, 15, 4, 0, 3,
    15, 0, 16, 4, 17, 18, 17, 19, 19, 20, 21, 18, 19, 21, 16, 17, 18, 1, 16, 22, 22, 10, 6, 1, 22,
    6, 4, 16, 1, 23, 24, 25, 26, 25, 27, 10, 26, 27, 23, 25, 26, 28, 24, 23, 29, 3, 30, 24, 29, 30,
    14, 3, 29, 8, 25, 31, 31, 3, 12, 8, 31, 12, 27, 8, 11, 10, 27, 11, 25, 8, 27, 25, 24, 30, 30, 3,
    31, 25, 30, 31, 32, 33, 34, 10, 32, 34, 35, 33, 32, 33, 28, 23, 34, 23, 26, 10, 34, 26, 33, 23,
    34, 36, 16, 37, 38, 36, 37, 36, 10, 22, 16, 36, 22, 39, 18, 40, 41, 39, 40, 16, 18, 39, 42, 21,
    43, 44, 42, 43, 18, 21, 42, 21, 20, 45, 45, 44, 43, 21, 45, 43, 44, 41, 40, 40, 18, 42, 44, 40,
    42, 41, 38, 37, 37, 16, 39, 41, 37, 39, 38, 35, 32, 32, 10, 36, 38, 32, 36,
  ]).toEqual(triangles);
});

test('generateHillshade', async () => {
  const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1544_3085.webp`).arrayBuffer();
  const { width, height, hillshade } = await generateHillshade(
    elevationImage,
    { zoom: 13, x: 1544, y: 3085 },
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
  );

  const sharpOptions: SharpOptions = { raw: { width, height, channels: 1 } };
  const pngData = await sharp(new Uint8ClampedArray(hillshade), sharpOptions).png().toBuffer();
  // uncomment to store the image
  // await Bun.write(`${__dirname}/fixtures/13_1544_3085_hillshade.png`, pngData);
  const expectedPngData = Buffer.from(
    await Bun.file(`${__dirname}/fixtures/13_1544_3085_hillshade.png`).arrayBuffer(),
  );
  expect(pngData).toEqual(expectedPngData);
});

test('vectorizeHillshade', async () => {
  const elevationImage = await Bun.file(`${__dirname}/fixtures/13_1544_3085.webp`).arrayBuffer();
  const featureCollection = await vectorizeHillshade(
    elevationImage,
    { zoom: 13, x: 1544, y: 3085 },
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
  );

  // write as flattened geojson features instead of vector features
  // const flatFeatures = featureCollection.features.map((vf) => toFlatGeometry(vf));
  // await Bun.write(
  //   `${__dirname}/fixtures/tmp.json`,
  //   JSON.stringify({ type: 'FeatureCollection', features: flatFeatures }, null, 2),
  // );

  // uncomment to store the image
  // await Bun.write(
  //   `${__dirname}/fixtures/13_1544_3085_vector_hillshade.json`,
  //   JSON.stringify(featureCollection, null, 2),
  // );

  const expectedFeatureCollection = await Bun.file(
    `${__dirname}/fixtures/13_1544_3085_vector_hillshade.json`,
  ).json();
  expect(featureCollection).toEqual(expectedFeatureCollection);
});
