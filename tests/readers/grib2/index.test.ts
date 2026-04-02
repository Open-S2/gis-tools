import { FileReader } from '../../../src/file.js';
import { buildServer } from '../../server.js';
import {
  GRIB2Reader,
  fetchGFSAtmos,
  fetchGFSWave,
  getGrib2Template3,
  llNormalize,
  parseIDX,
  parsedIDXFromURL,
  toReader,
} from '../../../src/index.js';
import { expect, test } from 'bun:test';

import type { VectorPoint } from '../../../src/index.js';

test('Basic GRIB2 case', async () => {
  // build expected
  const expectedRaw = await Bun.file(
    `${__dirname}/fixtures/ref_simple_packing.grib2.spread.txt`,
  ).text();
  const expected: VectorPoint<{ TMP: number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lon, lat, tmp] = line.split(',');
    expected.push(
      llNormalize({ x: parseFloat(lon), y: parseFloat(lat), m: { TMP: parseFloat(tmp) } }),
    );
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
  expected.sort((a, b) => {
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
    expected.push(
      llNormalize({ x: parseFloat(lon), y: parseFloat(lat), m: { 'TMP:2 m': parseFloat(tmp) } }),
    );
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
    expected.push(
      llNormalize({
        x: parseFloat(lon),
        y: parseFloat(lat),
        m: {
          UGRD: ugrd !== '' ? parseFloat(ugrd) : undefined,
          VGRD: vgrd !== '' ? parseFloat(vgrd) : undefined,
        },
      }),
    );
  }

  const features = await Array.fromAsync(grib2Reader);
  const { coordinates } = features[0].geometry;
  // compare coordinates against expected
  for (let i = 0; i < coordinates.length; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
  }
});

test('GRIB2 polar stero case - arctic_9km', async () => {
  const server = buildServer();

  // wgrib2 ./gfs.20250209/12/wave/gridded/gfswave.t12z.arctic.9km.f000.grib2 -match ":(UGRD|VGRD|WDIR):surface" -csv ./gfs.20250209/12/wave/gridded/gfswave.t12z.arctic.9km.f000.csv
  // Stored as:
  // Date, Time, Variable, Level, Longitude, Latitude, Value

  const grib2Reader = await fetchGFSWave(
    `http://localhost:${server.port}/readers/grib2/fixtures/`,
    'arctic.9km',
    '2025',
    '02',
    '09',
    '12',
    '000',
    ['UGRD:surface', 'VGRD:surface', 'WDIR:surface'],
  );

  expect(grib2Reader.packets.length).toEqual(3);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'WDIR',
      category: 'Momentum (see Table 4.2-0-2)',
      parameter: 'Wind Direction (from which blowing)',
      units: '°',
    },
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

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) =>
      coord.m?.['UGRD:surface'] !== undefined ||
      coord.m?.['VGRD:surface'] !== undefined ||
      coord.m?.['WDIR:surface'] !== undefined,
  );
  expect(coordinates.length).toEqual(127_876);
  // console.log('coordinates', coordinates[0]);

  // sort coordinates by greatest latitude to smallest. Then longitude by smallest to greatest
  coordinates.sort((a, b) => {
    if (a.y > b.y) return 1;
    else if (a.y < b.y) return -1;
    else if (a.x > b.x) return 1;
    else if (a.x < b.x) return -1;
    return 0;
  });

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }
  // "2025-02-09 12:00:00","2025-02-09 12:00:00","WDIR","surface",-3.03081,50.0016,52.4
  expect([minLat, maxLat]).toEqual([49.95951268876489, 77.98745445479484]);
  expect([minLon, maxLon]).toEqual([-68.05638466326674, 56.31356792974]);

  await server.stop();
});

test('GRIB2 rotated LL case', async () => {
  // setup reader
  const reader = new FileReader(
    `${__dirname}/fixtures/20260219T00Z_MSC_HRDPS_CAPE_Sfc_RLatLon0.0225_PT000H.grib2`,
  );
  const grib2Reader = new GRIB2Reader(reader);

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'CAPE',
      category: 'Thermodynamic Stability indices (see Table 4.2-0-7)',
      parameter: 'Convective Available Potential Energy',
      units: 'J kg-1',
    },
  ]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  expect(coordinates.length).toEqual(3_276_600);
  // console.log('coordinates', coordinates[0]);
  // console.log(features[0].metadata);

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([-179.99997478972665, 179.99998699273897]);
  expect([minLat, maxLat]).toEqual([-66.21395977931718, 66.56854088113894]);

  // const expectedRaw = await Bun.file(
  //   `${__dirname}/fixtures/20260219T00Z_MSC_HRDPS_CAPE_Sfc_RLatLon0.0225_PT000H.csv`,
  // ).text();
  // const expected: VectorPoint<{ Value?: number }>[] = [];
  // const expectedLines = expectedRaw.split('\n');
  // for (const line of expectedLines.slice(1)) {
  //   const [lat, lon, value] = line.split(',');
  //   expected.push({
  //     x: parseFloat(lon),
  //     y: parseFloat(lat),
  //     m: {
  //       Value: value !== '' ? parseFloat(value) : undefined,
  //     },
  //   });
  // }

  // XXX: The algorithm used by grib_get_data uses some interpolation so they won't match exactly
  // for (let i = 0; i < 5; i++) {
  //   expect(coordinates[i].x).toBeCloseTo(expected[i].x);
  //   expect(coordinates[i].y).toBeCloseTo(expected[i].y);
  //   expect(expected[i].m?.Value as number).toBeCloseTo(coordinates[i].m?.['0'] as number);
  // }
});

test('GRIB2 polar stereo case', async () => {
  // setup reader
  const reader = new FileReader(
    `${__dirname}/fixtures/CMC_RDPA_APCP-024-0100cutoff_SFC_0_ps10km_2023121806_000.grib2`,
  );
  const grib2Reader = new GRIB2Reader(reader);

  expect(grib2Reader.packets.length).toEqual(2);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'APCP',
      category: 'Moisture (see Table 4.2-0-1)',
      parameter: 'Total Precipitation',
      units: 'kg m-2',
    },
    {
      abbrev: 'CFRZR',
      category: 'Moisture (see Table 4.2-0-1)',
      parameter: 'Categorical Freezing Rain',
      units: 'Code table 4.222',
    },
  ]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  expect(coordinates.length).toEqual(733_298); // 770,440

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([-179.99976765517715, 179.9999203939235]);
  expect([minLat, maxLat]).toEqual([17.342726124319366, 90]);

  const expectedRaw = await Bun.file(
    `${__dirname}/fixtures/CMC_RDPA_APCP-024-0100cutoff_SFC_0_ps10km_2023121806_000.csv`,
  ).text();
  const expected: VectorPoint<{ Value?: number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lat, lon, value] = line.split(',');
    expected.push(
      llNormalize({
        x: parseFloat(lon),
        y: parseFloat(lat),
        m: {
          Value: value !== '' ? parseFloat(value) : undefined,
        },
      }),
    );
  }

  // sort both coordinates and expected by greatest latitude to smallest. Then longitude by smallest to greatest
  // coordinates.sort((a, b) => {
  //   if (a.y > b.y) return 1;
  //   else if (a.y < b.y) return -1;
  //   else if (a.x > b.x) return 1;
  //   else if (a.x < b.x) return -1;
  //   return 0;
  // });
  // console.log('coordinates', coordinates[0]);
  // console.log('expected', expected[0]);
  // expected = expected.sort((a, b) => {
  //   if (a.y > b.y) return 1;
  //   else if (a.y < b.y) return -1;
  //   else if (a.x > b.x) return 1;
  //   else if (a.x < b.x) return -1;
  //   return 0;
  // });

  for (let i = 0; i < 50; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
    expect(expected[i].m?.Value as number).toBeCloseTo(coordinates[i].m?.['0'] as number);
  }
});

test('GRIB2 polar stereo case 2', async () => {
  // setup reader
  const reader = new FileReader(
    `${__dirname}/fixtures/CMC_RDPA_APCP-006-0700cutoff_SFC_0_ps15km_2012100300_000.grib2`,
  );
  const grib2Reader = new GRIB2Reader(reader);

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    // @ts-expect-error - this is fine.
    {
      category: 'Missing',
    },
  ]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  expect(coordinates.length).toEqual(196_707); // 770,440

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([-179.99659872259667, 179.99985420781064]);
  expect([minLat, maxLat]).toEqual([24.541010809168096, 90]);

  // const expectedRaw = await Bun.file(
  //   `${__dirname}/fixtures/CMC_RDPA_APCP-024-0100cutoff_SFC_0_ps10km_2023121806_000.csv`,
  // ).text();
  // const expected: VectorPoint<{ Value?: number }>[] = [];
  // const expectedLines = expectedRaw.split('\n');
  // for (const line of expectedLines.slice(1)) {
  //   const [lat, lon, value] = line.split(',');
  //   expected.push(
  //     llNormalize({
  //       x: parseFloat(lon),
  //       y: parseFloat(lat),
  //       m: {
  //         Value: value !== '' ? parseFloat(value) : undefined,
  //       },
  //     }),
  //   );
  // }

  // sort both coordinates and expected by greatest latitude to smallest. Then longitude by smallest to greatest
  // coordinates.sort((a, b) => {
  //   if (a.y > b.y) return 1;
  //   else if (a.y < b.y) return -1;
  //   else if (a.x > b.x) return 1;
  //   else if (a.x < b.x) return -1;
  //   return 0;
  // });
  // console.log('coordinates', coordinates[0]);
  // console.log('expected', expected[0]);
  // expected = expected.sort((a, b) => {
  //   if (a.y > b.y) return 1;
  //   else if (a.y < b.y) return -1;
  //   else if (a.x > b.x) return 1;
  //   else if (a.x < b.x) return -1;
  //   return 0;
  // });

  // for (let i = 0; i < 50; i++) {
  //   expect(coordinates[i].x).toBeCloseTo(expected[i].x);
  //   expect(coordinates[i].y).toBeCloseTo(expected[i].y);
  //   expect(expected[i].m?.Value as number).toBeCloseTo(coordinates[i].m?.['0'] as number);
  // }
});

test('GRIB2 - gdas gaussian case', async () => {
  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/gdas.t00z.sfluxgrbf000.grib2`);
  const grib2Reader = new GRIB2Reader(reader);

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'HGT',
      category: 'Mass (see Table 4.2-0-3)',
      parameter: 'Geopotential Height',
      units: 'gpm',
    },
  ]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  expect(coordinates.length).toEqual(4_718_592);

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([-179.999232, 179.88358]);
  expect([minLat, maxLat]).toEqual([-89.91032453466268, 89.91032453466268]);

  const expectedRaw = await Bun.file(`${__dirname}/fixtures/gdas.t00z.sfluxgrbf000.csv`).text();
  const expected: VectorPoint<{ Value?: number }>[] = [];
  const expectedLines = expectedRaw.split('\n');
  for (const line of expectedLines.slice(1)) {
    const [lat, lon, value] = line.split(',');
    expected.push(
      llNormalize({
        x: parseFloat(lon),
        y: parseFloat(lat),
        m: {
          Value: value !== '' ? parseFloat(value) : undefined,
        },
      }),
    );
  }

  // will never be perfectly the same. the algorithms are the tiniest bit differnt and maths are slightly diff
  for (let i = 0; i < 150; i++) {
    expect(coordinates[i].x).toBeCloseTo(expected[i].x);
    expect(coordinates[i].y).toBeCloseTo(expected[i].y);
    expect(expected[i].m?.Value as number).toBeCloseTo(coordinates[i].m?.['0'] as number);
  }
});

test('GRIB2 - gdas mercator case', async () => {
  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/mercator.grib2`);
  const grib2Reader = new GRIB2Reader(reader);

  // -new_grid mercator:lad lon0:nx:dx:lonn lat0:ny:dy:latn
  // wgrib2 gdas.t00z.sfluxgrbf000.grib2 -new_grid_winds earth -new_grid mercator:0 0:300:10000:30 0:300:10000:30 mercator.grib2

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });

  expect(packetProducts).toEqual([
    {
      abbrev: 'HGT',
      category: 'Mass (see Table 4.2-0-3)',
      parameter: 'Geopotential Height',
      units: 'gpm',
    },
  ]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  expect(coordinates.length).toEqual(90_000);

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([0, 30]);
  expect([minLat, maxLat]).toEqual([0, 29.999999999999986]);
});

test('GRIB2 - NOAA lambert case', () => {
  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/ds.critfireo.bin`);
  const slice = reader.slice(0x83 - 14);
  const template3Reader = toReader(
    new Uint8Array(slice.buffer, slice.byteOffset, slice.byteLength),
  );
  const grib2Reader = getGrib2Template3(30, template3Reader);

  const grid = grib2Reader.buildGrid();
  expect(grid[0]).toEqual({ x: -121.550004, y: 20.190000000000015, m: {} });
  expect(grid[grid.length - 1]).toEqual({ x: -60.88202274199159, y: 50.10246110127133, m: {} });
});

test('GRIB2 - NOAA lambert case 2', () => {
  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/dspr.temp.bin`);
  const slice = reader.slice(0x83 - 14);
  const template3Reader = toReader(
    new Uint8Array(slice.buffer, slice.byteOffset, slice.byteLength),
  );
  const grib2Reader = getGrib2Template3(30, template3Reader);

  const grid = grib2Reader.buildGrid();
  expect(grid[0]).toEqual({ x: -68.02783299999999, y: 16.97748499999998, m: {} });
  expect(grid[grid.length - 1]).toEqual({ x: -6.505849659198702, y: 89.20962465025708, m: {} });

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of grid) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  expect([minLon, maxLon]).toEqual([-119.40503379902958, 159.2586455823198]);
  expect([minLat, maxLat]).toEqual([-89.63754750787379, 89.20962465025708]);
});

test('GRIB2 - png case', async () => {
  // setup reader
  const reader = new FileReader(`${__dirname}/fixtures/large_png.grib2`);
  const grib2Reader = new GRIB2Reader(reader);

  expect(grib2Reader.packets.length).toEqual(1);

  const packetProducts = grib2Reader.packets.map((packet) => {
    const { paramater } = packet.productDefinition?.values ?? {};
    return paramater;
  });
  // @ts-expect-error - just for testing
  expect(packetProducts).toEqual([{ category: undefined }]);

  const features = await Array.fromAsync(grib2Reader);
  const coordinates = features[0].geometry.coordinates.filter(
    (coord) => Object.values(coord.m ?? {}).length > 0,
  );
  // 24_500_000 if including undefined values
  expect(coordinates.length).toEqual(17_955_835);

  let minLat = Infinity;
  let maxLat = -Infinity;
  let minLon = Infinity;
  let maxLon = -Infinity;
  for (const coord of coordinates) {
    minLat = Math.min(minLat, coord.y);
    maxLat = Math.max(maxLat, coord.y);
    minLon = Math.min(minLon, coord.x);
    maxLon = Math.max(maxLon, coord.x);
  }

  // wgrib2 large_png.grib2 -domain
  // 1:0:N=54.995000 S=20.005000 W=-129.994996 E=-60.005002
  // All the data below 29.345 lat were dead inputs
  expect([minLon, maxLon]).toEqual([-129.99499600000001, -60.00500199999999]);
  expect([minLat, maxLat]).toEqual([29.345000000000002, 54.995]);

  let minValue = Infinity;
  let maxValue = -Infinity;
  for (const coord of coordinates) {
    minValue = Math.min(minValue, coord.m!['0'] ?? 0);
    maxValue = Math.max(maxValue, coord.m!['0'] ?? 0);
  }
  // wgrib2 large_png.grib2 -stats
  // 1:0:ndata=24500000:undef=12529443:mean=0.00319137:min=0:max=11.5:cos_wt_mean=0.00314073
  expect([minValue, maxValue]).toEqual([0, 10.28]);
}, 30_000);

// -new_grid mercator:lad lon0:nx:dx:lonn lat0:ny:dy:latn
// wgrib2 gdas.t00z.sfluxgrbf000.grib2 -new_grid_winds earth -new_grid mercator:0 0:300:10000:30 0:300:10000:30 mercator.grib2
