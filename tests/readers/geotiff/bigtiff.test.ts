import FileReader from '../../../src/readers/file';
import { GeoTIFFReader } from '../../../src/readers/geotiff';
import { expect, test } from 'bun:test';

// import { fromArrayBuffer } from '../../../geotiff/src/geotiff';
import { fromArrayBuffer } from 'geotiff';

import type { ArrayTypes } from '../../../src/readers/geotiff';

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

testFunc('bigtiff.tiff test', async (): Promise<void> => {
  const fileReader = new FileReader(`${__dirname}/fixtures/bigtiff.tiff`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  const image = geotiffReader.getImage();
  const raster = await image.rasterData();
  const rgb = await image.getRGBA();

  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/bigtiff.tiff`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const cmpRaster = await cmpImage.readRasters({ interleave: true });
  expect(raster.data).toEqual(cmpRaster as unknown as ArrayTypes);
  expect(raster.width).toEqual(cmpRaster.width);
  expect(raster.height).toEqual(cmpRaster.height);

  const cmpRGB = await cmpImage.readRGB({ interleave: true, enableAlpha: true });
  expect(rgb.data).toEqual(cmpRGB as unknown as ArrayTypes);
  expect(rgb.width).toEqual(cmpRGB.width);
  expect(rgb.height).toEqual(cmpRGB.height);
  expect(rgb.alpha).toBeFalse();
});

testFunc('cog.tiff test', async (): Promise<void> => {
  const fileReader = new FileReader(`${__dirname}/fixtures/cog.tiff`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  const image = geotiffReader.getImage();
  const raster = await image.rasterData();
  const rgb = await image.getRGBA();

  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/cog.tiff`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const cmpRaster = await cmpImage.readRasters({ interleave: true });
  expect(raster.data).toEqual(cmpRaster as unknown as ArrayTypes);
  expect(raster.width).toEqual(cmpRaster.width);
  expect(raster.height).toEqual(cmpRaster.height);

  const cmpRGB = await cmpImage.readRGB({ interleave: true, enableAlpha: true });
  expect(rgb.data).toEqual(cmpRGB as unknown as ArrayTypes);
  expect(rgb.width).toEqual(cmpRGB.width);
  expect(rgb.height).toEqual(cmpRGB.height);
  expect(rgb.alpha).toBeFalse();
});
