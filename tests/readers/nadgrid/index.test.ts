import { NadGridReader } from '../../../src';
import { expect, test } from 'bun:test';

import type { VectorPoint } from '../../../src';

test('NadGridReader', async () => {
  const grid = new NadGridReader(
    'BETA2007.gsb',
    await Bun.file(`${__dirname}/../../proj4/fixtures/BETA2007.gsb`).arrayBuffer(),
  );
  expect(grid).toBeInstanceOf(NadGridReader);
  expect(grid.header).toEqual({
    fromSemiMajorAxis: 6377397.155,
    fromSemiMinorAxis: 6356078.963,
    nFields: 11,
    nSubgridFields: 11,
    nSubgrids: 1,
    shiftType: 'SECONDS ',
    toSemiMajorAxis: 6378137,
    toSemiMinorAxis: 6356752.314,
  });

  const fc = grid.getFeatureCollection();
  expect(fc.type).toBe('FeatureCollection');
  expect(fc.features.length).toBe(1);

  const feature = fc.features[0];
  expect(feature.type).toBe('VectorFeature');

  const geometry = feature.geometry;
  expect(geometry.type).toBe('MultiPoint');

  const coordinates = geometry.coordinates as VectorPoint[];
  expect(coordinates.length).toBe(5208);
});
