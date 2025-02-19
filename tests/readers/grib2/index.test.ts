import { FileReader } from '../../../src/file';
import { buildServer } from '../../server';
import { GRIB2Reader, fetchGFSAtmos, fetchGFSWave, parseIDX, parsedIDXFromURL } from '../../../src';
import { expect, test } from 'bun:test';

import type { VectorPoint } from '../../../src';

test('Basic GRIB2 case', async () => {
  // build expected
  const expectedRaw = await Bun.file(
    `${__dirname}/fixtures/ref_simple_packing.grib2.spread.txt`,
  ).text();
  const expected: VectorPoint<{ TMP: number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lon, lat, tmp] = line.split(',');
    expected.push({ x: parseFloat(lon), y: parseFloat(lat), m: { TMP: parseFloat(tmp) } });
  }

  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/ref_simple_packing.grib2`);
  const grib2Reader = new GRIB2Reader(reader);
  const features = await Array.fromAsync(grib2Reader);
  const { coordinates } = features[0].geometry;

  // sort coordinates with the same sorting done by the GRIB2 expected data
  coordinates.sort((a, b) => {
    if (a.y > b.y) return 1;
    else if (a.y < b.y) return -1;
    else if (a.x > b.x) return 1;
    else if (a.x < b.x) return -1;
    return 0;
  });

  // compare coordinates against expected
  for (let i = 0; i < coordinates.length; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
    expect(coordinates[i].m?.['0']).toBeCloseTo(expected[i].m?.TMP ?? 0);
  }
});

test('parseIDX', async () => {
  const data = await Bun.file(
    `${__dirname}/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2.txt`,
  ).text();
  const sections = parseIDX(data, [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:']);

  expect(sections.length).toEqual(3);

  expect(sections).toEqual([
    {
      start: 1231864,
      line: '12:1231864:d=2024042612:DZDT:0.01 mb:anl:',
      end: 1337928,
      name: ':DZDT:0.01 mb:',
    },
    {
      start: 7024838,
      line: '68:7024838:d=2024042612:TMP:0.4 mb:anl:',
      end: 7122757,
      name: ':TMP:0.4 mb:',
    },
    {
      start: 7710271,
      line: '75:7710271:d=2024042612:ABSV:0.4 mb:anl:',
      name: ':ABSV:0.4 mb:anl:',
    },
  ]);
});

test('GRIB2Reader from filtered IDX and FileReader', async () => {
  const data = await Bun.file(
    `${__dirname}/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2.txt`,
  ).text();
  const sections = parseIDX(data, [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:']);

  const reader = new FileReader(
    `${__dirname}/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2`,
  );
  const grib2Reader = await GRIB2Reader.fromIDX(reader, sections);

  expect(grib2Reader.packets.length).toEqual(3);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'DZDT',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'Vertical Velocity (Geometric)',
      units: 'm s-1',
    },
    {
      abbrev: 'TMP',
      category: 'Temperature (see Table 4.2-0-0)',
      parameter: 'Temperature',
      units: 'K',
    },
    {
      abbrev: 'ABSV',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'Absolute Vorticity',
      units: 's-1',
    },
  ]);
});

test('GRIB2Reader from filtered IDX and fetch calls', async () => {
  const server = buildServer();
  const sections = await parsedIDXFromURL(
    `http://localhost:${server.port}/readers/grib2/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2.txt`,
    [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:'],
  );
  const grib2Reader = await GRIB2Reader.fromIDX(
    `http://localhost:${server.port}/readers/grib2/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2`,
    sections,
  );
  await server.stop();

  expect(grib2Reader.packets.length).toEqual(3);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'DZDT',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'Vertical Velocity (Geometric)',
      units: 'm s-1',
    },
    {
      abbrev: 'TMP',
      category: 'Temperature (see Table 4.2-0-0)',
      parameter: 'Temperature',
      units: 'K',
    },
    {
      abbrev: 'ABSV',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'Absolute Vorticity',
      units: 's-1',
    },
  ]);
});

test('GRIB2Reader using GFS Atmosphere data tooling', async () => {
  const server = buildServer();

  const grib2Reader = await fetchGFSAtmos(
    `http://localhost:${server.port}/readers/grib2/fixtures/`,
    'pgrb2b.1p00',
    '2024',
    '12',
    '14',
    '12',
    '003',
    ['TMP:2 m'],
  );

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'TMP',
      category: 'Temperature (see Table 4.2-0-0)',
      parameter: 'Temperature',
      units: 'K',
    },
  ]);

  await server.stop();

  // build expected
  const expectedRaw = await Bun.file(
    `${__dirname}/fixtures/gfs.20241214/12/atmos/expected_tmp2m.csv`,
  ).text();
  const expected: VectorPoint<{ 'TMP:2 m': number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lat, lon, tmp] = line.split(',');
    expected.push({ x: parseFloat(lon), y: parseFloat(lat), m: { 'TMP:2 m': parseFloat(tmp) } });
  }

  const features = await Array.fromAsync(grib2Reader);
  const { coordinates } = features[0].geometry;
  // compare coordinates against expected
  for (let i = 0; i < coordinates.length; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
    expect(coordinates[i].m?.['TMP:2 m']).toBeCloseTo(expected[i].m?.['TMP:2 m'] ?? 0);
  }
});

test('GRIB2Reader using GFS Wave arctic data tooling', async () => {
  const server = buildServer();

  const grib2Reader = await fetchGFSWave(
    `http://localhost:${server.port}/readers/grib2/fixtures/`,
    'arctic.9km',
    '2025',
    '02',
    '09',
    '12',
    '000',
    ['UGRD:surface', 'VGRD:surface'],
  );

  expect(grib2Reader.packets.length).toEqual(2);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'UGRD',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'U-Component of Wind',
      units: 'm s-1',
    },
    {
      abbrev: 'VGRD',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'V-Component of Wind',
      units: 'm s-1',
    },
  ]);

  await server.stop();

  // // build expected
  // const expectedRaw = await Bun.file(
  //   `${__dirname}/fixtures/gfs.20241214/12/atmos/expected_tmp2m.csv`,
  // ).text();
  // const expected: VectorPoint<{ 'TMP:2 m': number }>[] = [];
  // const expectedLines = expectedRaw.split('\n');
  // for (const line of expectedLines.slice(1)) {
  //   const [lat, lon, tmp] = line.split(',');
  //   expected.push({ x: parseFloat(lon), y: parseFloat(lat), m: { 'TMP:2 m': parseFloat(tmp) } });
  // }

  // const features = await Array.fromAsync(grib2Reader);
  // const { coordinates } = features[0].geometry;
  // // compare coordinates against expected
  // for (let i = 0; i < coordinates.length; i++) {
  //   expect(coordinates[i].x).toBeCloseTo(expected[i].x);
  //   expect(coordinates[i].y).toBeCloseTo(expected[i].y);
  //   expect(coordinates[i].m?.['TMP:2 m']).toBeCloseTo(expected[i].m?.['TMP:2 m'] ?? 0);
  // }
});

test('GRIB2Reader using GFS Wave global data tooling', async () => {
  const server = buildServer();

  const grib2Reader = await fetchGFSWave(
    `http://localhost:${server.port}/readers/grib2/fixtures/`,
    'global.0p16',
    '2025',
    '02',
    '19',
    '00',
    '000',
    ['UGRD:surface', 'VGRD:surface'],
  );

  expect(grib2Reader.packets.length).toEqual(2);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'UGRD',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'U-Component of Wind',
      units: 'm s-1',
    },
    {
      abbrev: 'VGRD',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'V-Component of Wind',
      units: 'm s-1',
    },
  ]);

  await server.stop();

  // build expected
  const expectedRaw = await Bun.file(
    `${__dirname}/fixtures/gfs.20250219/00/wave/gridded/gfs_wave_global.csv`,
  ).text();
  const expected: VectorPoint<{ UGRD?: number; VGRD?: number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lat, lon, ugrd, vgrd] = line.split(',');
    expected.push({
      x: parseFloat(lon),
      y: parseFloat(lat),
      m: {
        UGRD: ugrd !== '' ? parseFloat(ugrd) : undefined,
        VGRD: vgrd !== '' ? parseFloat(vgrd) : undefined,
      },
    });
  }

  const features = await Array.fromAsync(grib2Reader);
  const { coordinates } = features[0].geometry;
  // compare coordinates against expected
  for (let i = 0; i < coordinates.length; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
    // if (expected[i].m?.UGRD !== undefined) {
    //   expect(coordinates[i].m?.['UGRD:surface']).toBeCloseTo(
    //     expected[i].m?.UGRD ?? Number.NEGATIVE_INFINITY,
    //   );
    // }
    // if (expected[i].m?.VGRD !== undefined) {
    //   expect(coordinates[i].m?.VGRD ?? 0).toBeCloseTo(expected[i].m?.VGRD ?? Number.NEGATIVE_INFINITY);
    // }
    // expect(coordinates[i].m?.['UGRD:surface']).toBeCloseTo(expected[i].m?.UGRD ?? Number.NEGATIVE_INFINITY);
    // expect(coordinates[i].m?.['VGRD:surface']).toBeCloseTo(expected[i].m?.VGRD ?? Number.NEGATIVE_INFINITY);
  }
});
