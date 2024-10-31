import { shapefileFromPath } from '../../../src/mmap';
import { expect, test } from 'bun:test';

test('shapefileFromPath', async () => {
  const shp = await shapefileFromPath(`${__dirname}/fixtures/utf.shp`);
  expect(shp.getHeader()).toEqual({
    bbox: [-108.97956848144531, 41.244772343082076, -108.6328125, 41.253032440653186, 0, 0],
    length: 156,
    shpCode: 1,
    version: 1000,
  });

  const featureCollection = await shp.getFeatureCollection();
  expect(featureCollection).toEqual({
    bbox: [-108.97956848144531, 41.244772343082076, -108.6328125, 41.253032440653186, 0, 0],
    features: [
      {
        geometry: {
          coordinates: { x: -108.6328125, y: 41.244772343082076 },
          is3D: false,
          type: 'Point',
        },
        id: 1,
        properties: { field: '💩' },
        type: 'VectorFeature',
      },
      {
        geometry: {
          coordinates: { x: -108.97956848144531, y: 41.253032440653186 },
          is3D: false,
          type: 'Point',
        },
        id: 2,
        properties: { field: 'Hněvošický háj' },
        type: 'VectorFeature',
      },
    ],
    type: 'FeatureCollection',
  });
});

test('shapefileFromPath', async () => {
  const shp = await shapefileFromPath(`${__dirname}/fixtures/utf.zip`);
  expect(shp.getHeader()).toEqual({
    bbox: [-108.97956848144531, 41.244772343082076, -108.6328125, 41.253032440653186, 0, 0],
    length: 156,
    shpCode: 1,
    version: 1000,
  });

  const featureCollection = await shp.getFeatureCollection();
  expect(featureCollection).toEqual({
    bbox: [-108.97956848144531, 41.244772343082076, -108.6328125, 41.253032440653186, 0, 0],
    features: [
      {
        geometry: {
          coordinates: { x: -108.6328125, y: 41.244772343082076 },
          is3D: false,
          type: 'Point',
        },
        id: 1,
        properties: { field: '💩' },
        type: 'VectorFeature',
      },
      {
        geometry: {
          coordinates: { x: -108.97956848144531, y: 41.253032440653186 },
          is3D: false,
          type: 'Point',
        },
        id: 2,
        properties: { field: 'Hněvošický háj' },
        type: 'VectorFeature',
      },
    ],
    type: 'FeatureCollection',
  });
});
