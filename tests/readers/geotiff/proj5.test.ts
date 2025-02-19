import { FileReader } from '../../../src/file';
import { GeoTIFFImage, GeoTIFFReader } from '../../../src';
import { expect, test } from 'bun:test';

// We are setting up for improved GRID SHIFTING in the PROJ package

test('proj5 tif grids: MX', async (): Promise<void> => {
  const fileReader = new FileReader(`${__dirname}/fixtures/mx_inegi_ggm10.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  expect(geotiffReader.length).toEqual(1);

  const image = geotiffReader.getImage();
  expect(image).toBeInstanceOf(GeoTIFFImage);
  expect(image.width).toEqual(792);
  expect(image.height).toEqual(456);
  expect(image.tileHeight).toEqual(256);
  expect(image.tileWidth).toEqual(256);

  const raster = await image.rasterData();
  const value_2_0 = await image.getValue(2, 0);
  expect(value_2_0[0]).toEqual(raster.data[0 * image.width + 2]);
  const value_15_12 = await image.getValue(15, 12);
  expect(value_15_12[0]).toEqual(raster.data[12 * image.width + 15]);
});

test('proj5 tif grids: US NOAA', (): void => {
  const fileReader = new FileReader(`${__dirname}/fixtures/us_noaa_geoid09_conus.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  expect(geotiffReader.length).toEqual(1);

  const image = geotiffReader.getImage();
  expect(image).toBeInstanceOf(GeoTIFFImage);
  expect(image.width).toEqual(4201);
  expect(image.height).toEqual(2041);
  expect(image.tileHeight).toEqual(256);
  expect(image.tileWidth).toEqual(256);
  expect(image.blockWidth).toEqual(256);
  expect(image.getBlockHeight(0)).toEqual(256);
  expect(image.bytesPerPixel).toEqual(4);
  expect(image.getSampleFormat(0)).toEqual(3);
  expect(image.tiePoints).toEqual([{ i: 0, j: 0, k: 0, x: 230, y: 58.00000000000681, z: 0 }]);
  expect(image.origin).toEqual({ x: 230, y: 58.00000000000681, z: 0 });
  expect(image.originLL).toEqual({ x: 230, y: 58.00000000000681, z: 0 });
  expect(image.pixelIsArea).toBeFalse();
  expect(image.resolution).toEqual({ x: 0.01666666666667, y: -0.01666666666667, z: 0 });
  expect(image.resolutionLL).toEqual({ x: 0.01666666666667, y: -0.01666666666667, z: 0 });
  expect(image.getBoundingBox(true)).toEqual([
    230, 23.983333333333334, 300.0166666666807, 58.00000000000681,
  ]);
});
