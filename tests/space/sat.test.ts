import { Classification, Satellite, convertCelestrak } from '../../src';
import { expect, test } from 'bun:test';
import { propagate, sgp4, twoline2satrec } from 'satellite.js';

test('Parse 3 line TLE', () => {
  const tle = `0 VANGUARD 1
1 00005U 58002B   23048.45156751  .00000181  00000-0  25486-3 0  9999
2 00005  34.2454 104.3484 1845489  60.4612 316.7290 10.85063109311153`;

  const sat = new Satellite(tle);
  expect(sat.init).toBeTrue();
  expect(sat.name).toEqual('VANGUARD 1');
  expect(sat.number).toEqual(5);
  expect(sat.class).toEqual('U');
  expect(sat.id).toEqual('58002B');
  expect(sat.date).toEqual(new Date('2023-02-17T10:50:15.432Z'));
  expect(sat.epochyr).toEqual(23);
  expect(sat.epochdays).toEqual(48.45156751);
  expect(sat.jdsatepoch).toEqual(2459992.95156751);
  expect(sat.fdmm).toEqual(0.00000181);
  expect(sat.sdmm).toEqual(0);
  expect(sat.drag).toEqual(0.00025486);
  expect(sat.ephemeris).toEqual(0);
  expect(sat.esn).toEqual(999);
  expect(sat.inclination).toEqual(0.5976949836624661);
  expect(sat.ascension).toEqual(1.8212231491880508);
  expect(sat.eccentricity).toEqual(0.1845489);
  expect(sat.perigee).toEqual(1.0552470097067956);
  expect(sat.anomaly).toEqual(5.52796388654912);
  expect(sat.motion).toEqual(0.04732152797251528);
  expect(sat.revolution).toEqual(31115);
  expect(sat.opsmode).toEqual('i');
  expect(sat.rms).toBeUndefined();
  expect(sat.isimp).toEqual(0);
  expect(sat.method).toEqual('n');

  const res = sat.sgp4(0);
  if ('error' in res) throw new Error(res.error);
  expect(res.position).toEqual({
    x: -1890.492429739611,
    y: 7390.621411745235,
    z: -0.002347052928088251,
  });
  expect(res.velocity).toEqual({
    x: -5.7713025020338495,
    y: -2.6265789334048164,
    z: 4.251446670073898,
  });

  const propRes = sat.propagate(new Date('2023-03-01T00:00:00.000Z'));
  if ('error' in propRes) throw new Error(propRes.error);
  expect(propRes.position).toEqual({
    x: -502.2371910168779,
    y: -8100.02828196518,
    z: -1678.3876585442658,
  });
  expect(propRes.velocity).toEqual({
    x: 5.825765595419856,
    y: -0.8599211730107191,
    z: -3.9101899789514967,
  });

  const gpuValues = sat.gpu();
  const expected = [
    5.52796388654912, 0.04732152797251528, 0.1845489, 0.5976949836624661, 1, 1, 0.00025486,
    0.0473448075185996, 1.0552470097067956, 0.000054585673733659655, 1.8212231491880508,
    -0.000037353169000612346, -1.0876046284095624e-14, 8.633304471526831e-11, 5.25831237896029e-7,
    0.000016412617096688515, 1.2949956707290246e-10, 0, 3.546526141537639e-14, 0.7348325322103315,
    -1.773853994711644e-10, 3.6171313056732544, 1.1871101555605976e-19, 2.414315925315311e-28,
    5.709777886753732e-37, -0.6854499062015231, 1.336178047756768e-19, 2.1342843026655983e-28,
    4.149973577337839e-37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    5.405741198438328, 0, 0, 0, 0, 0, 0.000659830582190508, 0.0012883490581074237,
    1.0499759291281268, 0.31667469029062434, 3.78327716796563, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ];

  for (let i = 0; i < expected.length; i++) {
    expect(gpuValues[i]).toBeCloseTo(expected[i], 5);
  }
});

test('Parse as JSON', () => {
  const json = {
    name: 'VANGUARD 1',
    number: 90005,
    class: 'U' as Classification,
    id: '58002B',
    date: '2023-02-17T10:50:15.432Z',
    fdmm: 0.00000181,
    sdmm: 0,
    drag: 0.00025486,
    ephemeris: 0,
    esn: 999,
    inclination: 34.2454,
    ascension: 104.3484,
    eccentricity: 0.1845489,
    perigee: 60.4612,
    anomaly: 316.729,
    motion: 10.850631,
    revolution: 31115,
  };

  const sat = new Satellite(json);
  expect(sat.init).toBeTrue();
  expect(sat.name).toEqual('VANGUARD 1');
  expect(sat.number).toEqual(90005);
  expect(sat.class).toEqual('U');
  expect(sat.id).toEqual('58002B');
  expect(sat.date).toEqual(new Date('2023-02-17T10:50:15.432Z'));
  expect(sat.epochyr).toEqual(23);
  expect(sat.epochdays).toEqual(48.451567499898374);
  expect(sat.jdsatepoch).toEqual(2459992.9515675);
  expect(sat.fdmm).toEqual(0.00000181);
  expect(sat.sdmm).toEqual(0);
  expect(sat.drag).toEqual(0.00025486);
  expect(sat.ephemeris).toEqual(0);
  expect(sat.esn).toEqual(999);
  expect(sat.inclination).toEqual(0.5976949836624661);
  expect(sat.ascension).toEqual(1.8212231491880508);
  expect(sat.eccentricity).toEqual(0.1845489);
  expect(sat.perigee).toEqual(1.0552470097067956);
  expect(sat.anomaly).toEqual(5.52796388654912);
  expect(sat.motion).toEqual(0.04732152758026673);
  expect(sat.revolution).toEqual(31115);
  expect(sat.opsmode).toEqual('i');
  expect(sat.rms).toBeUndefined();
  expect(sat.isimp).toEqual(0);
  expect(sat.method).toEqual('n');
});

test('Parse 3 line TLE without the 0', () => {
  const tle = `STARLINK-1007
1 44713C 19074A   23052.85534722  .00022625  00000+0  15174-2 0   524
2 44713  53.0506 137.8409 0001488  98.7814 102.3561 15.06339386    13`;

  const sat = new Satellite(tle);
  expect(sat.init).toBeTrue();
  expect(sat.name).toEqual('STARLINK-1007');
  expect(sat.number).toEqual(44713);
  expect(sat.class).toEqual('C');
  expect(sat.id).toEqual('19074A');
  expect(sat.date).toEqual(new Date('2023-02-21T20:31:41.999Z'));
  expect(sat.epochyr).toEqual(23);
  expect(sat.epochdays).toEqual(52.85534722);
  expect(sat.jdsatepoch).toEqual(2459997.35534722);
  expect(sat.fdmm).toEqual(0.00022625);
  expect(sat.sdmm).toEqual(0);
  expect(sat.drag).toEqual(0.0015174);
  expect(sat.ephemeris).toEqual(0);
  expect(sat.esn).toEqual(52);
  expect(sat.inclination).toEqual(0.9259076401585038);
  expect(sat.ascension).toEqual(2.4057775489122517);
  expect(sat.eccentricity).toEqual(0.0001488);
  expect(sat.perigee).toEqual(1.7240606697295267);
  expect(sat.anomaly).toEqual(1.786450954500568);
  expect(sat.motion).toEqual(0.06572265280932744);
  expect(sat.revolution).toEqual(1);
  expect(sat.opsmode).toEqual('i');
  expect(sat.rms).toBeUndefined();
  expect(sat.isimp).toEqual(0);
  expect(sat.method).toEqual('n');
});

test('Parse celestrak json', () => {
  const celestrak = {
    OBJECT_NAME: 'STARLINK-1007',
    OBJECT_ID: '2019-074A',
    EPOCH: '2023-02-21T20:31:41.999808',
    MEAN_MOTION: 15.06339386,
    ECCENTRICITY: 0.0001488,
    INCLINATION: 53.0506,
    RA_OF_ASC_NODE: 137.8409,
    ARG_OF_PERICENTER: 98.7814,
    MEAN_ANOMALY: 102.3561,
    EPHEMERIS_TYPE: 0,
    CLASSIFICATION_TYPE: 'C',
    NORAD_CAT_ID: 44713,
    ELEMENT_SET_NO: 52,
    REV_AT_EPOCH: 1,
    BSTAR: 0.0015174,
    MEAN_MOTION_DOT: 0.00022625,
    MEAN_MOTION_DDOT: 0,
    RMS: '0.363',
    DATA_SOURCE: 'SpaceX-E',
  };
  const json = convertCelestrak(celestrak);

  const sat = new Satellite(json);
  expect(sat.init).toBeTrue();
  expect(sat.name).toEqual('STARLINK-1007');
  expect(sat.number).toEqual(44713);
  expect(sat.class).toEqual('C');
  expect(sat.id).toEqual('2019-074A');
  expect(sat.date).toEqual(new Date('2023-02-21T20:31:41.999Z'));
  expect(sat.epochyr).toEqual(23);
  expect(sat.epochdays).toEqual(52.85534721054137);
  expect(sat.jdsatepoch).toEqual(2459997.3553472105);
  expect(sat.fdmm).toEqual(0.00022625);
  expect(sat.sdmm).toEqual(0);
  expect(sat.drag).toEqual(0.0015174);
  expect(sat.ephemeris).toEqual(0);
  expect(sat.esn).toEqual(52);
  expect(sat.inclination).toEqual(0.9259076401585038);
  expect(sat.ascension).toEqual(2.4057775489122517);
  expect(sat.eccentricity).toEqual(0.0001488);
  expect(sat.perigee).toEqual(1.7240606697295267);
  expect(sat.anomaly).toEqual(1.786450954500568);
  expect(sat.motion).toEqual(0.06572265280932744);
  expect(sat.revolution).toEqual(1);
  expect(sat.opsmode).toEqual('i');
  expect(sat.rms).toEqual(0.363);
  expect(sat.isimp).toEqual(0);
  expect(sat.method).toEqual('n');
});

test('Check TLE matches satellite.js', () => {
  const tle = `0 VANGUARD 1
1 00005U 58002B   23048.45156751  .00000181  00000-0  25486-3 0  9999
2 00005  34.2454 104.3484 1845489  60.4612 316.7290 10.85063109311153`;

  const sat = new Satellite(tle);
  expect(sat.init).toBeTrue();
  const satZero = sat.sgp4(0);
  if ('error' in satZero) throw new Error(satZero.error);
  const sat1600 = sat.sgp4(1600);
  if ('error' in sat1600) throw new Error(sat1600.error);
  const sat3200 = sat.sgp4(3200);
  if ('error' in sat3200) throw new Error(sat3200.error);

  const [, firstLine, secondLine] = tle.split('\n');
  const satrec = twoline2satrec(firstLine, secondLine);
  const satrecZero = sgp4(satrec, 0);
  const satrec1600 = sgp4(satrec, 1600);
  const satrec3200 = sgp4(satrec, 3200);

  // @ts-expect-error - no need to type check against a package interface
  expect(satZero.position).toEqual(satrecZero.position);
  // @ts-expect-error - no need to type check against a package interface
  expect(sat1600.position).toEqual(satrec1600.position);
  // @ts-expect-error - no need to type check against a package interface
  expect(sat3200.position).toEqual(satrec3200.position);
});

test('Check data/tle.txt matches satellite.js', async () => {
  const TLEs = await Bun.file(`${__dirname}/fixtures/tle.txt`).text();
  const tles = TLEs.split('\n');
  // group every 2 lines into a group
  const groups: string[][] = [];
  for (let i = 0; i < tles.length; i += 2) {
    groups.push([tles[i], tles[i + 1]]);
  }

  for (const [firstLine, secondLine] of groups) {
    const joined = `${firstLine}\n${secondLine}`;

    const sat = new Satellite(joined);
    const satZero = sat.sgp4(0);
    if ('error' in satZero) continue;
    const sat1600 = sat.sgp4(1600);
    if ('error' in sat1600) continue;
    const sat3200 = sat.sgp4(3200);
    if ('error' in sat3200) continue;

    const satrec = twoline2satrec(firstLine, secondLine);
    const satrecZero = sgp4(satrec, 0);
    const satrec1600 = sgp4(satrec, 1600);
    const satrec3200 = sgp4(satrec, 3200);

    // @ts-expect-error - no need to type check against a package interface
    expect(satZero.position).toEqual(satrecZero.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(satZero.velocity).toEqual(satrecZero.velocity);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat1600.position).toEqual(satrec1600.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat1600.velocity).toEqual(satrec1600.velocity);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat3200.position).toEqual(satrec3200.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat3200.velocity).toEqual(satrec3200.velocity);

    // PROPAGATION
    const date = new Date('2023-05-11T00:00:00.000Z');
    const satProp = sat.propagate(date);
    if ('error' in satProp) continue;
    const satrecProp = propagate(satrec, date);
    // @ts-expect-error - no need to type check against a package interface
    expect(satProp.position, satrecProp.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(satProp.velocity, satrecProp.velocity);
  }
});

test('Check data/feb17.tle.txt matches satellite.js', async () => {
  const Feb17TLEs = await Bun.file(`${__dirname}/fixtures/feb17.tle.txt`).text();
  const tles = Feb17TLEs.split('\n');
  // group every 2 lines into a group
  const groups: string[][] = [];
  for (let i = 0; i < tles.length; i += 3) {
    // groups.push(tles.slice(i, i + 3));
    groups.push([tles[i], tles[i + 1], tles[i + 2]]);
  }

  for (const [firstLine, secondLine, thirdline] of groups) {
    const joined = `${firstLine}\n${secondLine}\n${thirdline}`;

    const sat = new Satellite(joined);
    const satZero = sat.sgp4(0);
    if ('error' in satZero) continue;
    const sat1600 = sat.sgp4(1600);
    if ('error' in sat1600) continue;
    const sat3200 = sat.sgp4(3200);
    if ('error' in sat3200) continue;

    const satrec = twoline2satrec(secondLine, thirdline);
    const satrecZero = sgp4(satrec, 0);
    if ('error' in satrecZero) continue;
    const satrec1600 = sgp4(satrec, 1600);
    if ('error' in satrec1600) continue;
    const satrec3200 = sgp4(satrec, 3200);
    if ('error' in satrec3200) continue;

    // @ts-expect-error - no need to type check against a package interface
    expect(satZero.position).toEqual(satrecZero.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(satZero.velocity).toEqual(satrecZero.velocity);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat1600.position).toEqual(satrec1600.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat1600.velocity).toEqual(satrec1600.velocity);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat3200.position).toEqual(satrec3200.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(sat3200.velocity).toEqual(satrec3200.velocity);

    // PROPAGATION
    const date = new Date('2023-05-11T00:00:00.000Z');
    const satProp = sat.propagate(date);
    if ('error' in satProp) continue;
    const satrecProp = propagate(satrec, date);
    // @ts-expect-error - no need to type check against a package interface
    expect(satProp.position).toEqual(satrecProp.position);
    // @ts-expect-error - no need to type check against a package interface
    expect(satProp.velocity).toEqual(satrecProp.velocity);
  }
});
