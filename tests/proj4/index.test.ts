import { Mercator, Transformer, injectAllDefinitions } from '../../src';
import { describe, expect, it } from 'bun:test';

import { TEST_DATA } from './testData';

// was untested:
// - src/proj4/projections/cea.ts
// - src/proj4/projections/gstmerc.ts
// - src/proj4/projections/ortho.ts
// - src/proj4/projections/somerc.ts

describe('basic tests', () => {
  it('should parse mercator and units', () => {
    const transform = new Transformer();
    transform.setSource(
      '+proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +units=m +k=1.0 +nadgrids=@null +no_defs',
    );
    const testMerc = transform.source;
    expect(testMerc).toBeInstanceOf(Mercator);
    // @ts-expect-error - we just want to check internal properties
    expect(testMerc.units).toBe('m');
    expect(transform.forward({ x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
    expect(transform.forward({ x: 4156404, y: 7480076.5 })).toEqual({
      x: 37.33761240175515,
      y: 55.604470490269755,
    });
  });
});

describe('proj2proj', () => {
  it('should work transforming from one projection to another', () => {
    const transform = new Transformer();
    injectAllDefinitions(transform);
    const sweref99tm = '+proj=utm +zone=33 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs';
    const rt90 =
      '+lon_0=15.808277777799999 +lat_0=0.0 +k=1.0 +x_0=1500000.0 +y_0=0.0 +proj=tmerc +ellps=bessel +units=m +towgs84=414.1,41.3,603.1,-0.855,2.141,-7.023,0 +no_defs';
    transform.setSource(sweref99tm);
    transform.setDestination(rt90);
    const rslt = transform.forward({ x: 319180, y: 6399862 });
    expect(rslt).toEqual({ x: 1271137.9275601413, y: 6404230.291459565 });
  });
});

describe('test data', () => {
  TEST_DATA.forEach((testPoint, i) => {
    it(`${testPoint.code} (${i})`, () => {
      const transform = new Transformer();
      injectAllDefinitions(transform);
      transform.setSource(testPoint.code);
      const { ll, xy, acc } = testPoint;
      const to = transform.forward({ x: xy[0], y: xy[1], z: xy[2] });
      expect(to.x).toBeCloseTo(ll[0], acc?.ll ?? -1);
      expect(to.y).toBeCloseTo(ll[1], acc?.ll ?? -1);
      if ('z' in to) {
        expect(to.z).toBeCloseTo(ll[2], acc?.ll ?? -1);
      }
      const from = transform.inverse({ x: ll[0], y: ll[1], z: ll[2] });
      expect(from.x).toBeCloseTo(xy[0], acc?.xy ?? -1);
      expect(from.y).toBeCloseTo(xy[1], acc?.xy ?? -1);
      if ('z' in from) {
        expect(from.z).toBeCloseTo(xy[2], acc?.xy ?? -1);
      }
    });
  });
});

describe('axes should be invertable with proj4.transform()', function () {
  const enu = '+proj=longlat +axis=enu';
  const esu = '+proj=longlat +axis=esu';
  const wnu = '+proj=longlat +axis=wnu';
  const transform = new Transformer(enu, esu);
  injectAllDefinitions(transform);
  const result = transform.forward({ x: 40, y: 50 }, true);
  expect(result.x).toBeCloseTo(40, 5);
  expect(result.y).toBeCloseTo(-50, 5);
  transform.setDestination(wnu);
  const result2 = transform.forward({ x: 40, y: 50 }, true);
  expect(result2.x).toBeCloseTo(-40, 5);
  expect(result2.y).toBeCloseTo(50, 5);
});
