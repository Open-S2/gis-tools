import { Mercator, Transformer, injectAllDefinitions } from '../../src/proj4';
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

//     describe('Nadgrids BETA2007', function () {
//       const tests = [
//         ['EPSG:31466', 'EPSG:4326', 2559552, 5670982, 6.850861772, 51.170707759, 0.0000001, 0.01],
//         [
//           'EPSG:31466',
//           'EPSG:3857',
//           2559552,
//           5670982,
//           762634.443931574,
//           6651545.68026527,
//           0.01,
//           0.01,
//         ],
//         [
//           'EPSG:31466',
//           'EPSG:25832',
//           2559552,
//           5670982,
//           349757.381712518,
//           5671004.06504954,
//           0.01,
//           0.01,
//         ],
//       ];

//       /**
//        * @param buffer
//        */
//       function initializeNadgrid(buffer) {
//         proj4.nadgrid('BETA2007.gsb', buffer);
//         proj4.defs(
//           'EPSG:31466',
//           '+proj=tmerc +lat_0=0 +lon_0=6 +k=1 +x_0=2500000 +y_0=0 +ellps=bessel +nadgrids=BETA2007.gsb +units=m +no_defs +type=crs',
//         );
//         proj4.defs(
//           'EPSG:25832',
//           '+proj=utm +zone=32 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs +type=crs',
//         );
//       }

//       before(function (done) {
//         if (typeof XMLHttpRequest !== 'undefined') {
//           const xhr = new XMLHttpRequest();
//           xhr.open('GET', 'BETA2007.gsb', true);
//           xhr.responseType = 'arraybuffer';
//           xhr.addEventListener('load', function () {
//             initializeNadgrid(xhr.response);
//             done();
//           });
//           xhr.addEventListener('error', done);
//           xhr.send();
//         } else if (typeof require === 'function') {
//           const fs = require('fs');
//           const path = require('path');
//           fs.readFile(path.join(__dirname, 'BETA2007.gsb'), function (err, data) {
//             if (err) {
//               done(err);
//             } else {
//               initializeNadgrid(data.buffer);
//               done();
//             }
//           });
//         }
//       });

//       tests.forEach(function (test) {
//         const fromProj = test[0];
//         const toProj = test[1];
//         const fromX = test[2];
//         const fromY = test[3];
//         const toX = test[4];
//         const toY = test[5];
//         const fromPrecision = test[6];
//         const toPrecision = test[7];
//         it('should transform ' + fromProj + ' to ' + toProj, function () {
//           const transformed = proj4(fromProj, toProj, [fromX, fromY]);
//           assert.approximately(transformed[0], toX, fromPrecision);
//           assert.approximately(transformed[1], toY, fromPrecision);
//         });
//         it('should transform ' + toProj + ' to ' + fromProj, function () {
//           const transformed = proj4(toProj, fromProj, [toX, toY]);
//           assert.approximately(transformed[0], fromX, toPrecision);
//           assert.approximately(transformed[1], fromY, toPrecision);
//         });
//       });
//     });

//     describe('Nadgrids ntv2', function () {
//       const tests = [
//         [-44.382211538462, 40.3768, -44.380749, 40.377457], // just inside the lower limit
//         [-87.617788, 59.623262, -87.617659, 59.623441], // just inside the upper limit
//         [-44.5, 40.5, -44.498553, 40.500632], // inside the first square
//         [-60, 50, -59.999192, 50.000058], // a general point towards the middle of the grid
//         [0, 0, 0, 0], // fall back to null
//       ];

//       let converter;

//       /**
//        * @param buffer
//        */
//       function initializeNadgrid(buffer) {
//         proj4.nadgrid('ntv2', buffer);
//         proj4.defs('ntv2_from', '+proj=longlat +ellps=clrk66 +nadgrids=@ignorable,ntv2,null');
//         proj4.defs('ntv2_to', '+proj=longlat +datum=WGS84 +no_defs');
//         converter = proj4('ntv2_from', 'ntv2_to');
//       }

//       before(function (done) {
//         if (typeof XMLHttpRequest !== 'undefined') {
//           const xhr = new XMLHttpRequest();
//           xhr.open('GET', 'ntv2_0_downsampled.gsb', true);
//           xhr.responseType = 'arraybuffer';
//           xhr.addEventListener('load', function () {
//             initializeNadgrid(xhr.response);
//             done();
//           });
//           xhr.addEventListener('error', done);
//           xhr.send();
//         } else if (typeof require === 'function') {
//           const fs = require('fs');
//           const path = require('path');
//           fs.readFile(path.join(__dirname, 'ntv2_0_downsampled.gsb'), function (err, data) {
//             if (err) {
//               done(err);
//             } else {
//               initializeNadgrid(data.buffer);
//               done();
//             }
//           });
//         }
//       });

//       tests.forEach(function (test) {
//         const fromLng = test[0];
//         const fromLat = test[1];
//         const toLng = test[2];
//         const toLat = test[3];
//         it('should interpolate ' + [fromLng, fromLat] + ' to ' + [toLng, toLat], function () {
//           const actual = converter.forward([fromLng, fromLat]);
//           assert.approximately(actual[0], toLng, 0.000001);
//           assert.approximately(actual[1], toLat, 0.000001);
//         });
//       });

//       const inverseTests = [
//         [-44.5, 40.5, -44.498553, 40.500632],
//         [-60, 50, -59.999192, 50.000058],
//       ];

//       inverseTests.forEach(function (test) {
//         const fromLng = test[0];
//         const fromLat = test[1];
//         const toLng = test[2];
//         const toLat = test[3];
//         it(
//           'should inverse interpolate ' + [toLng, toLat] + ' to ' + [fromLng, fromLat],
//           function () {
//             const actual = converter.inverse([toLng, toLat]);
//             assert.approximately(actual[0], fromLng, 0.000001);
//             assert.approximately(actual[1], fromLat, 0.000001);
//           },
//         );
//       });
//     });
//   });
// }
