import { FileReader } from '../../../src/file';
import { GeoTIFFReader } from '../../../src/readers/geotiff';
import { expect, test } from 'bun:test';

// import { fromArrayBuffer } from '../../../geotiff/src/geotiff';
import { fromArrayBuffer } from 'geotiff';

import type { ArrayTypes } from '../../../src/readers/geotiff';

test('utm.tif test', async (): Promise<void> => {
  const fileReader = new FileReader(`${__dirname}/fixtures/utm.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  const image = geotiffReader.getImage();
  const raster = await image.rasterData();
  const rgb = await image.getRGBA();

  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/utm.tif`).arrayBuffer(),
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

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [688258.223819, 4539289.697337001, 700411.209419, 4555765.966137]);

  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-78.76218940982291, 40.98307212979634, -78.61248498502884, 41.12847021213023],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [688258.223819, 4539289.697337001, 700411.209419, 4555765.966137]);
});

// NOTE: This is our base for all other proj4 tests

test('pixel_is_point_wgs84.tif test', async (): Promise<void> => {
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/pixel_is_point_wgs84.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);

  const image = geotiffReader.getImage();
  const raster = await image.rasterData();
  const rgb = await image.getRGBA();

  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/pixel_is_point_wgs84.tif`).arrayBuffer(),
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

  const bounds = image.getBoundingBox(true);
  closeToArray(bounds, [-1, 49, 3, 53]);
  const boundsCmp = cmpImage.getBoundingBox(true);
  closeToArray(boundsCmp, [-1, 49, 3, 53]);

  const vectorFeatures = await Array.fromAsync(geotiffReader);
  expect(vectorFeatures).toEqual([
    {
      geometry: {
        coordinates: [
          { m: { a: 255, b: 0, g: 0, r: 0 }, x: -1, y: 49 },
          { m: { a: 255, b: 0, g: 0, r: 0 }, x: 3, y: 49 },
          { m: { a: 255, b: 0, g: 0, r: 0 }, x: -1, y: 53 },
          { m: { a: 255, b: 0, g: 0, r: 0 }, x: 3, y: 53 },
        ],
        is3D: false,
        type: 'MultiPoint',
      },
      metadata: {
        alpha: false,
        height: 2,
        width: 2,
      },
      properties: {},
      type: 'VectorFeature',
    },
  ]);
});

test('albers_equal_area.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/albers_equal_area.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/albers_equal_area.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-108.87275117545303, 6.173382885196224, -108.87229095452848, 6.17401378974672],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);

  const vectorFeatures = await Array.fromAsync(geotiffReader);
  expect(vectorFeatures).toEqual([
    {
      geometry: {
        coordinates: [
          { m: { a: 255, b: 107, g: 107, r: 107 }, x: -108.87252106555276, y: 6.173698338044412 },
        ],
        is3D: false,
        type: 'MultiPoint',
      },
      metadata: {
        alpha: false,
        height: 1,
        width: 1,
      },
      properties: {},
      type: 'VectorFeature',
    },
  ]);
});

test('mercator1sp.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/mercator1sp.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/mercator1sp.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-27.980082548328227, 25.02030724471789, -27.9795418746082, 25.020799891095436],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);

  const vectorFeatures = await Array.fromAsync(geotiffReader);
  expect(vectorFeatures).toEqual([
    {
      geometry: {
        coordinates: [
          { m: { a: 255, b: 107, g: 107, r: 107 }, x: -27.979812211468207, y: 25.02055356815652 },
        ],
        is3D: false,
        type: 'MultiPoint',
      },
      metadata: {
        alpha: false,
        height: 1,
        width: 1,
      },
      properties: {},
      type: 'VectorFeature',
    },
  ]);
});

test('byte.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/byte.tif`);
  // const ntv2 = new MMapReader(`${__dirname}/fixtures/ntv2_0.gsb`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  // geotiffReader.addGridReader('NTv2_0.gsb', ntv2);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/byte.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3750120, 441920, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-117.64108762997199, 33.89153016859067, -117.62819018953391, 33.90241956192106],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3750120, 441920, 3751320]);
});

test('byte_v11.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/byte_v11.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/byte_v11.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3750120, 441920, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-117.64108762997199, 33.89153016859067, -117.62819018953391, 33.90241956192106],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3750120, 441920, 3751320]);
});

test('cassini_soldner.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/cassini_soldner.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/cassini_soldner.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [40.790879898859956, 64.22188358453757, 40.792219231938205, 64.2223720893481],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('epsg_27563_allgeokeys.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/epsg_27563_allgeokeys.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/epsg_27563_allgeokeys.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(
    bounds,
    [827294.1414437726, 523980.63806166063, 827299.1478137725, 523985.64443166065],
  );
  const boundsCorrected = image.getBoundingBox(true);
  // https://github.com/OSGeo/libgeotiff/blob/e00dcd652cd99e1ee81bd24a14b998cadab52c60/libgeotiff/test/testlistgeo_out.dist#L2110C47-L2110C74
  // TODO: The link claims the longitude is wrong by roughly 2 degrees. This is 100% an lcc projection inverse problem if it's wrong
  closeToArray(
    boundsCorrected,
    [5.321526154745895, 46.97709562609565, 5.321594210778345, 46.97713894791709],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(
    boundsCmp,
    [827294.1414437726, 523980.63806166063, 827299.1478137725, 523985.64443166065],
  );
});

test('epsg_27563_only_pcs_code.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/epsg_27563_only_pcs_code.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/epsg_27563_only_pcs_code.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(
    bounds,
    [827294.1414437726, 523980.63806166063, 827299.1478137725, 523985.64443166065],
  );
  const boundsCorrected = image.getBoundingBox(true);
  // https://github.com/OSGeo/libgeotiff/blob/e00dcd652cd99e1ee81bd24a14b998cadab52c60/libgeotiff/test/testlistgeo_out.dist#L2110C47-L2110C74
  // TODO: The link claims the longitude is wrong by roughly 2 degrees.
  closeToArray(
    boundsCorrected,
    [5.321526154745895, 46.97709562609565, 5.321594210778345, 46.97713894791709],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(
    boundsCmp,
    [827294.1414437726, 523980.63806166063, 827299.1478137725, 523985.64443166065],
  );
});

test('equidistant_cylindrical.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/equidistant_cylindrical.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/equidistant_cylindrical.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [3.959055120171555, 33.69814192706196, 3.9595941093420266, 33.69868091623243],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('GeogAngularUnitsGeoKey_9114.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/GeogAngularUnitsGeoKey_9114.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/GeogAngularUnitsGeoKey_9114.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [130, 31, 131, 32]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(boundsCorrected, [130, 31, 131, 32]);
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [130, 31, 131, 32]);
});

test('GeogGeodeticDatumGeoKey.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/GeogGeodeticDatumGeoKey.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/GeogGeodeticDatumGeoKey.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [130, 31, 131, 32]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(boundsCorrected, [130, 31, 131, 32]);
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [130, 31, 131, 32]);
});

test('GeogPrimeMeridianGeoKey.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/GeogPrimeMeridianGeoKey.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/GeogPrimeMeridianGeoKey.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [130, 31, 131, 32]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(boundsCorrected, [130, 31, 131, 32]);
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [130, 31, 131, 32]);
});

test('hotine_oblique_mercator_variant_a.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/hotine_oblique_mercator_variant_a.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/hotine_oblique_mercator_variant_a.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-51.85864116709959, 59.421978643657354, -51.85860959263306, 59.42131092467509],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('polyconic.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/polyconic.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/polyconic.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-113.31933294734617, -41.66713783099524, -113.31832403831677, -41.66708033666237],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_2046_transverse_mercator_south_oriented.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_2046_transverse_mercator_south_oriented.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_2046_transverse_mercator_south_oriented.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [19.75705540981311, 33.796319597379416, 19.75773097852268, 33.796833708967974],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3032_polar_stereographic_variant_b.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3032_polar_stereographic_variant_b.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3032_polar_stereographic_variant_b.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-42.02343167633273, -38.450318233839496, -42.02311520862427, -38.45090518430949],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3035_lambert_azimuthal_equal_area.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3035_lambert_azimuthal_equal_area.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3035_lambert_azimuthal_equal_area.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-42.96495690856996, 43.598542922816904, -42.96482656961041, 43.59929165328441],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3083_albers_equal_area.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3083_albers_equal_area.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3083_albers_equal_area.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-108.22504462114269, -4.331739226552316, -108.22461500598334, -4.331064089186024],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3410_lambert_cylindrical_equal_area.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3410_lambert_cylindrical_equal_area.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3410_lambert_cylindrical_equal_area.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [4.576480467318869, 30.657140786480117, 4.577103513307341, 30.657683994304794],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3812_lcc2sp.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3812_lcc2sp.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3812_lcc2sp.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-2.8272462789302906, 77.1238801981508, -2.8253915591441183, 77.12438254221291],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_3814_transverse_mercator.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3814_transverse_mercator.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_3814_transverse_mercator.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-90.66656670544788, 54.56296538271008, -90.66565121650869, 54.563511434831334],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_4087_equidistant_cylindrical.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_4087_equidistant_cylindrical.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_4087_equidistant_cylindrical.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [3.959055120171555, 33.69814192706196, 3.9595941093420266, 33.69868091623243],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_5329_mercator1sp.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5329_mercator1sp.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5329_mercator1sp.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [78.82763689611578, 25.019848065873887, 78.82817756983582, 25.02034070564569],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_5456_lcc1sp.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5456_lcc1sp.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5456_lcc1sp.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-84.93299846146739, 40.375665986398914, -84.93239011516971, 40.37613299657465],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_5482_polar_stereographic_variant_a.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5482_polar_stereographic_variant_a.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5482_polar_stereographic_variant_a.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [121.1085356377889, -44.49337154833407, 121.10942192177373, -44.49352843639299],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_5588_oblique_stereographic.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5588_oblique_stereographic.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5588_oblique_stereographic.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-69.08909055988515, 54.002196820883135, -69.08882272890078, 54.0023659994581],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_5641_mercator2sp.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5641_mercator2sp.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_5641_mercator2sp.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-83.98150694017086, -49.058641836811034, -83.98096762466132, -49.05828740676869],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_6808_hotine_oblique_mercator_variant_a.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_6808_hotine_oblique_mercator_variant_a.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_6808_hotine_oblique_mercator_variant_a.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  // TODO: https://github.com/OSGeo/libgeotiff/blob/e00dcd652cd99e1ee81bd24a14b998cadab52c60/libgeotiff/test/testlistgeo_out.dist#L555
  // claims a different result. hotine_oblique_mercator inverse function needs fixing?
  closeToArray(
    boundsCorrected,
    [-111.49806198380007, 76.25766320050388, -111.49567408556169, 76.25801291448202],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_8065_hotine_oblique_mercator_variant_b.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_8065_hotine_oblique_mercator_variant_b.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_8065_hotine_oblique_mercator_variant_b.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-110.41511644912529, 40.3421828360064, -110.41490098360144, 40.34234597944844],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

// TODO: support 'Laborde Grid' projection
test.skip('ProjectedCSTypeGeoKey_8441_oblique_mercator_laborde.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_8441_oblique_mercator_laborde.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_8441_oblique_mercator_laborde.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-110.41511644912529, 40.3421828360064, -110.41490098360144, 40.34234597944844],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_27200_new_zealand_mapping_grid.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_27200_new_zealand_mapping_grid.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_27200_new_zealand_mapping_grid.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [142.5869840558114, -55.838863750513596, 142.58790609359477, -55.83872449613639],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_28191_cassini_soldner.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_28191_cassini_soldner.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_28191_cassini_soldner.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [40.797896375174304, 64.22387868660537, 40.799235956583296, 64.22436706728936],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjectedCSTypeGeoKey_29101_polyconic.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_29101_polyconic.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjectedCSTypeGeoKey_29101_polyconic.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-113.3199662151629, -41.66728402682234, -113.31895731014677, -41.66722653844196],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('ProjLinearUnitsGeoKey_9036.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/ProjLinearUnitsGeoKey_9036.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/ProjLinearUnitsGeoKey_9036.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-102.33809671112067, 10.326816035405699, -102.33793170372446, 10.32698289376],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('transverse_mercator_south_oriented.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(
    `${__dirname}/fixtures/projections/transverse_mercator_south_oriented.tif`,
  );
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(
      `${__dirname}/fixtures/projections/transverse_mercator_south_oriented.tif`,
    ).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [19.757055409886483, 33.79631959785644, 19.75773097859621, 33.79683370944542],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

test('transverse_mercator.tif test', async (): Promise<void> => {
  // actual data
  const fileReader = new FileReader(`${__dirname}/fixtures/projections/transverse_mercator.tif`);
  const geotiffReader = new GeoTIFFReader(fileReader);
  const image = geotiffReader.getImage();
  // compare data
  const cmpTiff = await fromArrayBuffer(
    await Bun.file(`${__dirname}/fixtures/projections/transverse_mercator.tif`).arrayBuffer(),
  );
  const cmpImage = await cmpTiff.getImage();

  const bounds = image.getBoundingBox(false);
  closeToArray(bounds, [440720, 3751260, 440780, 3751320]);
  const boundsCorrected = image.getBoundingBox(true);
  closeToArray(
    boundsCorrected,
    [-90.66656670544788, 54.56296538271008, -90.66565121650869, 54.563511434831334],
  );
  const boundsCmp = cmpImage.getBoundingBox();
  closeToArray(boundsCmp, [440720, 3751260, 440780, 3751320]);
});

/**
 * closeToArray - check if contents of two arrays are close enough to each other
 * @param actual - array of actual values
 * @param expected - array of expected values
 * @param numDigits - number of digits of precision
 */
function closeToArray(actual: number[], expected: number[], numDigits?: number) {
  for (let i = 0; i < actual.length; i++) {
    expect(actual[i]).toBeCloseTo(expected[i], numDigits ?? 5);
  }
}
