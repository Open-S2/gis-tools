import { describe, expect, it } from 'bun:test';
import { mgrsForward, mgrsGetLetterDesignator, mgrsInverse, mgrsToPoint } from '../../src';

describe('First MGRS set', () => {
  const mgrsStr = '33UXP04';
  const point = mgrsToPoint(mgrsStr);
  if (point === undefined) throw new Error('Invalid MGRS string');
  it('Longitude of point from MGRS correct.', () => {
    expect(point.x).toBeCloseTo(16.4145, 6);
  });
  it('Latitude of point from MGRS correct.', () => {
    expect(point.y).toBeCloseTo(48.24949, 6);
  });
  it('MGRS reference with highest accuracy correct.', () => {
    expect(mgrsForward(point)).toEqual('33UXP0500444997');
  });
  it('MGRS reference with 1-digit accuracy correct.', () => {
    expect(mgrsForward(point, 1)).toEqual(mgrsStr);
  });
  it('MGRS reference with 0-digit accuracy correct.', () => {
    expect(mgrsForward(point, 0)).toEqual('33UXP');
    expect(mgrsInverse('33UXP')).toEqual([
      16.33659097483748, 47.84556125140087, 16.33659097483748, 47.84556125140087,
    ]);
  });
});

describe('Second MGRS set', () => {
  const mgrsStr = '24XWT783908'; // near UTM zone border, so there are two ways to reference this
  const point = mgrsToPoint(mgrsStr);
  if (point === undefined) throw new Error('Invalid MGRS string');
  it('Longitude of point from MGRS correct.', () => {
    expect(point.x).toBeCloseTo(-32.66433, 5);
  });
  it('Latitude of point from MGRS correct.', () => {
    expect(point.y).toBeCloseTo(83.62778, 5);
  });
  it('MGRS reference with 3-digit accuracy correct.', () => {
    expect(mgrsForward(point, 3)).toEqual('25XEN041865');
  });
  it('MGRS reference with 5-digit accuracy, northing all zeros', () => {
    expect(mgrsForward({ x: 0, y: 0 }, 5)).toEqual('31NAA6602100000');
  });
  it('MGRS reference with 5-digit accuracy, northing one digit', () => {
    expect(mgrsForward({ x: 0, y: 0.00001 }, 5)).toEqual('31NAA6602100001');
  });
  it('MGRS reference with 0-digit accuracy correct.', () => {
    expect(mgrsForward(point, 0)).toEqual('25XEN');
  });
});

describe('third mgrs set', () => {
  const mgrsStr = '11SPA7234911844';
  const point = { x: -115.0820944, y: 36.2361322 };
  it('MGRS reference with highest accuracy correct.', () => {
    expect(mgrsForward(point)).toEqual(mgrsStr);
    expect(mgrsInverse(mgrsStr)).toEqual([
      -115.08209766323476, 36.236123461597515, -115.08208632067898, 36.236132293763625,
    ]);
  });
  it('MGRS reference with 0-digit accuracy correct.', () => {
    expect(mgrsForward(point, 0)).toEqual('11SPA');
  });
});

describe('lots of various forwards and backwards', () => {
  let p = mgrsForward({ x: 0, y: 0 });
  expect(p).toEqual('31NAA6602100000');
  expect(mgrsToPoint(p)).toEqual({ x: 5.100560029891454e-7, y: 0.0000045174155428847555 });
  p = mgrsForward({ x: -180, y: 0 });
  expect(p).toEqual('1NAA6602100000');
  expect(mgrsToPoint(p)).toEqual({ x: -179.999999489944, y: 0.0000045174155428847555 });
  p = mgrsForward({ x: 180, y: 0 });
  expect(p).toEqual('60NZF3397800000');
  expect(mgrsToPoint(p)).toEqual({ x: 179.999999489944, y: 0.0000045174155055519 });
  p = mgrsForward({ x: 0, y: 84 });
  expect(p).toEqual('31XDP6500529005');
  expect(mgrsToPoint(p)).toEqual({ x: 0.000011844240836555997, y: 84.00000291206752 });
  p = mgrsForward({ x: 0, y: -80 });
  expect(p).toEqual('31CDM4186716915');
  expect(mgrsToPoint(p)).toEqual({ x: -0.00001347072431578944, y: -79.99999578854278 });
  p = mgrsForward({ x: 24, y: 73 });
  expect(p).toEqual('35XMB0213502930');
  expect(mgrsToPoint(p)).toEqual({ x: 23.99998502461724, y: 72.999999080815 });
  p = mgrsForward({ x: 34, y: 73 });
  expect(p).toEqual('37XCB3700307286');
  expect(mgrsToPoint(p)).toEqual({ x: 34.000006934267915, y: 73.00000314488452 });
});

describe('build forward and toPoint at increments', () => {
  for (let x = -180; x <= 180; x += 5) {
    for (let y = -80; y <= 84; y += 4) {
      const p = mgrsForward({ x, y });
      const q = mgrsToPoint(p);
      if (q === undefined) throw new Error(`failed to convert ${p} to point`);
      expect(q.x).toBeCloseTo(x, 3);
      expect(q.y).toBeCloseTo(y, 3);
    }
  }
});

describe('data validation', () => {
  describe('toPoint function', () => {
    it('toPoint should return the same result whether or not spaces are included in the MGRS String', () => {
      const { x: lon1, y: lat1 } = mgrsToPoint('4QFJ 12345 67890') ?? { x: -1, y: -1 };
      const { x: lon2, y: lat2 } = mgrsToPoint('4QFJ1234567890') ?? { x: 1, y: 1 };
      expect(lat1).toEqual(lat2);
      expect(lon1).toEqual(lon2);
    });
  });
  describe('forward function', () => {
    it('forward throws an error when longitude is outside bounds', () => {
      expect(() => {
        mgrsForward({ x: 200, y: 0 });
      }).toThrowError('forward received an invalid longitude of 200');
    });
    it('forward throws an error when latitude is outside bounds', () => {
      expect(() => {
        mgrsForward({ x: 90, y: 180 });
      }).toThrowError('forward received an invalid latitude of 180');
    });
    it('forward throws an error when latitude is near the north pole', () => {
      expect(() => {
        mgrsForward({ x: 45, y: 88 });
      }).toThrowError(
        'forward received a latitude of 88, but this library does not support conversions of points in polar regions below 80°S and above 84°N',
      );
    });
    it('forward throws an error when latitude is near the south pole', () => {
      expect(() => {
        mgrsForward({ x: 45, y: -88 });
      }).toThrowError(
        'forward received a latitude of -88, but this library does not support conversions of points in polar regions below 80°S and above 84°N',
      );
    });
  });
  describe('getLetterDesignator', () => {
    it('should return Z when latitude band is outside latitude handled by library', () => {
      const latitude = -83.3026329741;
      const letter = mgrsGetLetterDesignator(latitude);
      expect(letter).toEqual('Z');
    });
  });
});

// eslint-disable-next-line @typescript-eslint/no-misused-promises
describe('test against all data from mgrsToGeo_WE.txt', async (): Promise<void> => {
  const text = await Bun.file(`${__dirname}/fixtures/mgrsToGeo_WE.txt`).text();

  const [_header, _description, _blank, ...rows] = text.split(/\r?\n/);

  const testCases = rows
    .filter((testCase) => {
      return testCase.includes('Successful-Equivalent');
    })
    .map((row) => {
      const cells = row.replace(/\t+/g, '\t').split('\t');
      return {
        latitude: parseFloat(cells[9].trim()),
        longitude: parseFloat(cells[10].trim()),
        mgrs: cells[5].trim(),
      };
    });

  for (const { latitude, longitude, mgrs } of testCases) {
    // some polar regions are not supported:
    if (['A', 'B', 'Y', 'Z'].includes(mgrs[0])) return;
    it(`should match ${mgrs}`, () => {
      expect(mgrsForward({ x: longitude, y: latitude })).toEqual(mgrs);
    });
  }
});
