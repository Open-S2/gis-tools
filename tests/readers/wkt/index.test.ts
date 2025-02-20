import {
  WKTGeometryReader,
  parseWKTGeometry,
  parseWKTObject,
  parseWKTProjection,
  splitWKTGeometry,
} from '../../../src/readers/wkt';
import { expect, test } from 'bun:test';

test('GEOMETERYCOLLECTION', async () => {
  const collectionWKT =
    'GEOMETRYCOLLECTION (LINESTRING (6308869.40378 356821.22669, 6308867.893 356822.41744, 6308852.75314 356830.22159, 6308869.92754 356844.26638), LINESTRING (6308755.07971 356674.51686, 6308784.81355 356719.16757, 6308815.20022 356765.46178, 6308829.63774 356763.22832, 6308852.87023 356759.82402, 6308867.19982 356771.06823, 6308875.40631 356796.20162, 6308872.51907 356815.17242), LINESTRING (6308874.12086 356813.73392, 6308876.83028 356795.77697, 6308868.23871 356770.06254, 6308853.09618 356758.29456, 6308815.86529 356763.89689, 6308799.76731 356739.37835, 6308747.77971 356662.11613, 6308746.55411 356661.61702, 6308744.06545 356657.72563, 6308731.77184 356668.45076, 6308699.45221 356683.15463, 6308682.44689 356684.63193, 6308654.96629 356683.66846, 6308636.13879 356680.0482, 6308618.19888 356671.76352, 6308608.41685 356661.79428, 6308578.7973 356592.35062, 6308545.33908 356542.14886, 6308517.52088 356509.38474, 6308505.40266 356506.84141, 6308493.59689 356506.98067, 6308375.07918 356520.46209), LINESTRING (6308877.92941 356819.50984, 6309072.26249 356514.14689, 6309073.44938 356513.3739, 6309076.25423 356511.31751, 6309096.05004 356528.52014, 6309103.33938 356535.32615, 6309107.49584 356539.20699, 6309107.78601 356539.47793, 6309119.09139 356550.03322, 6309137.04465 356567.13752, 6309137.6323 356567.69515, 6309138.92096 356568.91355, 6309138.46355 356569.69798, 6309150.68532 356566.34027, 6309151.94333 356567.03108, 6309157.81557 356565.41779, 6309161.54152 356564.33408, 6309174.6464 356579.77423, 6309175.71622 356581.0361, 6309177.25892 356582.84545, 6309225.37695 356611.76515, 6309226.90588 356612.65173, 6309229.72021 356614.34101, 6309232.64678 356598.75445, 6309244.10246 356528.49893, 6309251.20809 356487.90256, 6309252.35489 356481.34967, 6309258.41778 356442.34047, 6309258.56036 356441.19511, 6309258.76115 356440.13123, 6309260.99127 356426.22389, 6309258.49745 356425.57244, 6309240.94882 356422.48836, 6309240.53276 356422.37171, 6309240.10958 356422.29068), LINESTRING (6308870.96141 356823.05522, 6308881.43519 356846.04558, 6308859.94336 356857.75024, 6308859.6305 356857.95378, 6308893.96675 356932.14467, 6308921.19517 356993.60222, 6308942.68768 357040.82051, 6308961.42173 357079.52481, 6308976.48471 357108.08898, 6308992.14194 357136.52543, 6309018.60922 357184.68892, 6309024.87557 357193.57884, 6309025.31785 357194.20629, 6309028.73486 357199.05392, 6309045.86114 357220.97586, 6309078.85225 357261.01696, 6309131.17986 357323.22098, 6309184.03434 357388.33409, 6309212.61182 357423.54026, 6309252.80543 357467.20429, 6309288.51836 357504.59499, 6309318.98068 357536.37443, 6309366.01084 357588.07961, 6309383.32941 357609.89089, 6309383.33718 357609.92579, 6309383.36584 357611.49516), POLYGON ((6309096.87876754 357058.96992573235, 6309100.9240038069 357067.89795246266, 6309103.1497403858 357077.44361610821, 6309103.4704434676 357087.24008216924, 6309101.8737886148 357096.91087794991, 6309098.421134375 357106.08436019259, 6309093.2451643161 357114.40799712023, 6309086.5447880644 357121.56191603432, 6309078.5774973193 357127.27119584358, 6309013.594489282 357164.78915304772, 6309004.6664625369 357168.8343893392, 6308995.1207988719 357171.06012593472, 6308985.3243327877 357171.38082902494, 6308975.6535369828 357169.78417417, 6308966.4800547194 357166.33151992067, 6308958.156417775 357161.1555498438, 6308951.0024988521 357154.45517356717, 6308945.293219042 357146.48788279406, 6308795.0043396624 356886.1799069175, 6308790.959103398 356877.251880196, 6308788.7333668182 356867.70621655986, 6308788.4126637317 356857.90975050797, 6308790.0093185771 356848.2389547351, 6308793.4619728047 356839.0654724979, 6308798.6379428506 356830.74183557258, 6308805.3383190883 356823.587916657, 6308813.3056098176 356817.87863684172, 6308878.2886178084 356780.36067953659, 6308887.2166445563 356776.315443229, 6308896.7623082288 356774.08970661991, 6308906.5587743223 356773.76900351944, 6308916.2295701364 356775.36565836804, 6308925.403052411 356778.81831261492, 6308933.7266893657 356783.99428269314, 6308940.8806082979 356790.69465897442, 6308946.5898881136 356798.66194975481, 6309096.87876754 357058.96992573235)))';
  const splitted = splitWKTGeometry(collectionWKT);
  expect(splitted).toEqual([
    'LINESTRING (6308869.40378 356821.22669, 6308867.893 356822.41744, 6308852.75314 356830.22159, 6308869.92754 356844.26638)',
    'LINESTRING (6308755.07971 356674.51686, 6308784.81355 356719.16757, 6308815.20022 356765.46178, 6308829.63774 356763.22832, 6308852.87023 356759.82402, 6308867.19982 356771.06823, 6308875.40631 356796.20162, 6308872.51907 356815.17242)',
    'LINESTRING (6308874.12086 356813.73392, 6308876.83028 356795.77697, 6308868.23871 356770.06254, 6308853.09618 356758.29456, 6308815.86529 356763.89689, 6308799.76731 356739.37835, 6308747.77971 356662.11613, 6308746.55411 356661.61702, 6308744.06545 356657.72563, 6308731.77184 356668.45076, 6308699.45221 356683.15463, 6308682.44689 356684.63193, 6308654.96629 356683.66846, 6308636.13879 356680.0482, 6308618.19888 356671.76352, 6308608.41685 356661.79428, 6308578.7973 356592.35062, 6308545.33908 356542.14886, 6308517.52088 356509.38474, 6308505.40266 356506.84141, 6308493.59689 356506.98067, 6308375.07918 356520.46209)',
    'LINESTRING (6308877.92941 356819.50984, 6309072.26249 356514.14689, 6309073.44938 356513.3739, 6309076.25423 356511.31751, 6309096.05004 356528.52014, 6309103.33938 356535.32615, 6309107.49584 356539.20699, 6309107.78601 356539.47793, 6309119.09139 356550.03322, 6309137.04465 356567.13752, 6309137.6323 356567.69515, 6309138.92096 356568.91355, 6309138.46355 356569.69798, 6309150.68532 356566.34027, 6309151.94333 356567.03108, 6309157.81557 356565.41779, 6309161.54152 356564.33408, 6309174.6464 356579.77423, 6309175.71622 356581.0361, 6309177.25892 356582.84545, 6309225.37695 356611.76515, 6309226.90588 356612.65173, 6309229.72021 356614.34101, 6309232.64678 356598.75445, 6309244.10246 356528.49893, 6309251.20809 356487.90256, 6309252.35489 356481.34967, 6309258.41778 356442.34047, 6309258.56036 356441.19511, 6309258.76115 356440.13123, 6309260.99127 356426.22389, 6309258.49745 356425.57244, 6309240.94882 356422.48836, 6309240.53276 356422.37171, 6309240.10958 356422.29068)',
    'LINESTRING (6308870.96141 356823.05522, 6308881.43519 356846.04558, 6308859.94336 356857.75024, 6308859.6305 356857.95378, 6308893.96675 356932.14467, 6308921.19517 356993.60222, 6308942.68768 357040.82051, 6308961.42173 357079.52481, 6308976.48471 357108.08898, 6308992.14194 357136.52543, 6309018.60922 357184.68892, 6309024.87557 357193.57884, 6309025.31785 357194.20629, 6309028.73486 357199.05392, 6309045.86114 357220.97586, 6309078.85225 357261.01696, 6309131.17986 357323.22098, 6309184.03434 357388.33409, 6309212.61182 357423.54026, 6309252.80543 357467.20429, 6309288.51836 357504.59499, 6309318.98068 357536.37443, 6309366.01084 357588.07961, 6309383.32941 357609.89089, 6309383.33718 357609.92579, 6309383.36584 357611.49516)',
    'POLYGON ((6309096.87876754 357058.96992573235, 6309100.9240038069 357067.89795246266, 6309103.1497403858 357077.44361610821, 6309103.4704434676 357087.24008216924, 6309101.8737886148 357096.91087794991, 6309098.421134375 357106.08436019259, 6309093.2451643161 357114.40799712023, 6309086.5447880644 357121.56191603432, 6309078.5774973193 357127.27119584358, 6309013.594489282 357164.78915304772, 6309004.6664625369 357168.8343893392, 6308995.1207988719 357171.06012593472, 6308985.3243327877 357171.38082902494, 6308975.6535369828 357169.78417417, 6308966.4800547194 357166.33151992067, 6308958.156417775 357161.1555498438, 6308951.0024988521 357154.45517356717, 6308945.293219042 357146.48788279406, 6308795.0043396624 356886.1799069175, 6308790.959103398 356877.251880196, 6308788.7333668182 356867.70621655986, 6308788.4126637317 356857.90975050797, 6308790.0093185771 356848.2389547351, 6308793.4619728047 356839.0654724979, 6308798.6379428506 356830.74183557258, 6308805.3383190883 356823.587916657, 6308813.3056098176 356817.87863684172, 6308878.2886178084 356780.36067953659, 6308887.2166445563 356776.315443229, 6308896.7623082288 356774.08970661991, 6308906.5587743223 356773.76900351944, 6308916.2295701364 356775.36565836804, 6308925.403052411 356778.81831261492, 6308933.7266893657 356783.99428269314, 6308940.8806082979 356790.69465897442, 6308946.5898881136 356798.66194975481, 6309096.87876754 357058.96992573235))',
  ]);

  const reader = new WKTGeometryReader(collectionWKT);
  const features = await Array.fromAsync(reader);
  expect(features.length).toEqual(6);
});

test('Collection', async () => {
  const collectionWKT = `POINT(4 6)
    GEOMETRYCOLLECTION(POINT(1 2), LINESTRING(3 4,5 6))
    MULTIPOLYGON EMPTY
    TRIANGLE((0 0 0,0 1 0,1 1 0,0 0 0))`;
  const splitted = splitWKTGeometry(collectionWKT);
  expect(splitted).toEqual([
    'POINT(4 6)',
    'TRIANGLE((0 0 0,0 1 0,1 1 0,0 0 0))',
    'POINT(1 2)',
    'LINESTRING(3 4,5 6)',
  ]);

  const reader = new WKTGeometryReader(collectionWKT);
  const features = await Array.fromAsync(reader);
  expect(features).toEqual([
    {
      geometry: {
        coordinates: { x: 4, y: 6 },
        is3D: false,
        type: 'Point',
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: 1, y: 2 },
        is3D: false,
        type: 'Point',
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: [
          { x: 3, y: 4 },
          { x: 5, y: 6 },
        ],
        is3D: false,
        type: 'LineString',
      },
      properties: {},
      type: 'VectorFeature',
    },
  ]);
});

test('parse geometry point', () => {
  const wktStr = 'POINT(0 0)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'Point',
    is3D: false,
    coordinates: { x: 0, y: 0 },
  });

  const wktStr2 = 'POINT(5.4321 1.2345)';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'Point',
    is3D: false,
    coordinates: { x: 5.4321, y: 1.2345 },
  });
});

test('parse geometry point 3D', () => {
  const wktStr = 'POINT Z (0 0 0)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'Point',
    is3D: true,
    coordinates: { x: 0, y: 0, z: 0 },
  });

  const wktStr2 = 'POINT Z (5.4321 1.2345 2.3456)';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'Point',
    is3D: true,
    coordinates: { x: 5.4321, y: 1.2345, z: 2.3456 },
  });
});

test('parse geometry multipoint', () => {
  const wktStr = 'MULTIPOINT (30 10, 10 30, 40 40)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiPoint',
    is3D: false,
    coordinates: [
      { x: 30, y: 10 },
      { x: 10, y: 30 },
      { x: 40, y: 40 },
    ],
  });

  const wktStr2 = 'MULTIPOINT ((30 10), (10 30), (40 40))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiPoint',
    is3D: false,
    coordinates: [
      { x: 30, y: 10 },
      { x: 10, y: 30 },
      { x: 40, y: 40 },
    ],
  });
});

test('parse geometry multipoint 3D', () => {
  const wktStr = 'MULTIPOINT Z (30 10 0, 10 30 1, 40 40 2)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 30, y: 10, z: 0 },
      { x: 10, y: 30, z: 1 },
      { x: 40, y: 40, z: 2 },
    ],
  });

  const wktStr2 = 'MULTIPOINT Z ((30 10 0), (10 30 1), (40 40 2))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiPoint',
    is3D: true,
    coordinates: [
      { x: 30, y: 10, z: 0 },
      { x: 10, y: 30, z: 1 },
      { x: 40, y: 40, z: 2 },
    ],
  });
});

test('parse geometry linestring', () => {
  const wktStr = 'LINESTRING (30 10, 10 30, 40 40)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'LineString',
    is3D: false,
    coordinates: [
      { x: 30, y: 10 },
      { x: 10, y: 30 },
      { x: 40, y: 40 },
    ],
  });

  const wktStr2 = 'LINESTRING ((30 10), (10 30), (40 40))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'LineString',
    is3D: false,
    coordinates: [
      { x: 30, y: 10 },
      { x: 10, y: 30 },
      { x: 40, y: 40 },
    ],
  });
});

test('parse geometry linestring 3D', () => {
  const wktStr = 'LINESTRING Z (30 10 0, 10 30 1, 40 40 2)';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'LineString',
    is3D: true,
    coordinates: [
      { x: 30, y: 10, z: 0 },
      { x: 10, y: 30, z: 1 },
      { x: 40, y: 40, z: 2 },
    ],
  });

  const wktStr2 = 'LINESTRING Z ((30 10 0), (10 30 1), (40 40 2))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'LineString',
    is3D: true,
    coordinates: [
      { x: 30, y: 10, z: 0 },
      { x: 10, y: 30, z: 1 },
      { x: 40, y: 40, z: 2 },
    ],
  });
});

test('parse geometry multilinestring', () => {
  const wktStr = 'MULTILINESTRING ((10 10, 20 20, 10 40),(40 40, 30 30, 40 20, 30 10))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiLineString',
    is3D: false,
    coordinates: [
      [
        { x: 10, y: 10 },
        { x: 20, y: 20 },
        { x: 10, y: 40 },
      ],
      [
        { x: 40, y: 40 },
        { x: 30, y: 30 },
        { x: 40, y: 20 },
        { x: 30, y: 10 },
      ],
    ],
  });

  const wktStr2 =
    'MULTILINESTRING (((10 10), (20 20), (10 40)),((40 40), (30 30), (40 20), (30 10)))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiLineString',
    is3D: false,
    coordinates: [
      [
        { x: 10, y: 10 },
        { x: 20, y: 20 },
        { x: 10, y: 40 },
      ],
      [
        { x: 40, y: 40 },
        { x: 30, y: 30 },
        { x: 40, y: 20 },
        { x: 30, y: 10 },
      ],
    ],
  });
});

test('parse geometry multilinestring 3D', () => {
  const wktStr =
    'MULTILINESTRING Z ((10 10 1, 20 20 2, 10 40 3),(40 40 4, 30 30 5, 40 20 6, 30 10 7))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 10, y: 10, z: 1 },
        { x: 20, y: 20, z: 2 },
        { x: 10, y: 40, z: 3 },
      ],
      [
        { x: 40, y: 40, z: 4 },
        { x: 30, y: 30, z: 5 },
        { x: 40, y: 20, z: 6 },
        { x: 30, y: 10, z: 7 },
      ],
    ],
  });

  const wktStr2 =
    'MULTILINESTRING Z (((10 10 1), (20 20 2), (10 40 3)),((40 40 4), (30 30 5), (40 20 6), (30 10 7)))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiLineString',
    is3D: true,
    coordinates: [
      [
        { x: 10, y: 10, z: 1 },
        { x: 20, y: 20, z: 2 },
        { x: 10, y: 40, z: 3 },
      ],
      [
        { x: 40, y: 40, z: 4 },
        { x: 30, y: 30, z: 5 },
        { x: 40, y: 20, z: 6 },
        { x: 30, y: 10, z: 7 },
      ],
    ],
  });
});

test('parse geometry polygon', () => {
  const wktStr = 'POLYGON ((10 10, 20 20, 10 40),(40 40, 30 30, 40 20, 30 10))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'Polygon',
    is3D: false,
    coordinates: [
      [
        { x: 10, y: 10 },
        { x: 20, y: 20 },
        { x: 10, y: 40 },
      ],
      [
        { x: 40, y: 40 },
        { x: 30, y: 30 },
        { x: 40, y: 20 },
        { x: 30, y: 10 },
      ],
    ],
  });

  const wktStr2 = 'POLYGON (((10 10), (20 20), (10 40)),((40 40), (30 30), (40 20), (30 10)))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'Polygon',
    is3D: false,
    coordinates: [
      [
        { x: 10, y: 10 },
        { x: 20, y: 20 },
        { x: 10, y: 40 },
      ],
      [
        { x: 40, y: 40 },
        { x: 30, y: 30 },
        { x: 40, y: 20 },
        { x: 30, y: 10 },
      ],
    ],
  });
});

test('parse geometry polygon 3D', () => {
  const wktStr = 'POLYGON Z ((10 10 1, 20 20 2, 10 40 3),(40 40 4, 30 30 5, 40 20 6, 30 10 7))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'Polygon',
    is3D: true,
    coordinates: [
      [
        { x: 10, y: 10, z: 1 },
        { x: 20, y: 20, z: 2 },
        { x: 10, y: 40, z: 3 },
      ],
      [
        { x: 40, y: 40, z: 4 },
        { x: 30, y: 30, z: 5 },
        { x: 40, y: 20, z: 6 },
        { x: 30, y: 10, z: 7 },
      ],
    ],
  });

  const wktStr2 =
    'POLYGON Z (((10 10 1), (20 20 2), (10 40 3)),((40 40 4), (30 30 5), (40 20 6), (30 10 7)))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'Polygon',
    is3D: true,
    coordinates: [
      [
        { x: 10, y: 10, z: 1 },
        { x: 20, y: 20, z: 2 },
        { x: 10, y: 40, z: 3 },
      ],
      [
        { x: 40, y: 40, z: 4 },
        { x: 30, y: 30, z: 5 },
        { x: 40, y: 20, z: 6 },
        { x: 30, y: 10, z: 7 },
      ],
    ],
  });
});

test('parse geometry multipolygon', () => {
  const wktStr =
    'MULTIPOLYGON (((40 40, 20 45, 45 30, 40 40)),((20 35, 10 30, 10 10, 30 5, 45 20, 20 35),(30 20, 20 15, 20 25, 30 20)))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiPolygon',
    is3D: false,
    coordinates: [
      [
        [
          { x: 40, y: 40 },
          { x: 20, y: 45 },
          { x: 45, y: 30 },
          { x: 40, y: 40 },
        ],
      ],
      [
        [
          { x: 20, y: 35 },
          { x: 10, y: 30 },
          { x: 10, y: 10 },
          { x: 30, y: 5 },
          { x: 45, y: 20 },
          { x: 20, y: 35 },
        ],
        [
          { x: 30, y: 20 },
          { x: 20, y: 15 },
          { x: 20, y: 25 },
          { x: 30, y: 20 },
        ],
      ],
    ],
  });

  const wktStr2 =
    'MULTIPOLYGON ((((40 40), (20 45), (45 30), (40 40))),(((20 35), (10 30), (10 10), (30 5), (45 20), (20 35)),((30 20), (20 15), (20 25), (30 20))))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiPolygon',
    is3D: false,
    coordinates: [
      [
        [
          { x: 40, y: 40 },
          { x: 20, y: 45 },
          { x: 45, y: 30 },
          { x: 40, y: 40 },
        ],
      ],
      [
        [
          { x: 20, y: 35 },
          { x: 10, y: 30 },
          { x: 10, y: 10 },
          { x: 30, y: 5 },
          { x: 45, y: 20 },
          { x: 20, y: 35 },
        ],
        [
          { x: 30, y: 20 },
          { x: 20, y: 15 },
          { x: 20, y: 25 },
          { x: 30, y: 20 },
        ],
      ],
    ],
  });
});

test('parse geometry multipolygon 3D', () => {
  const wktStr =
    'MULTIPOLYGON Z (((40 40 0, 20 45 1, 45 30 2, 40 40 3)),((20 35 4, 10 30 5, 10 10 6, 30 5 7, 45 20 8, 20 35 9),(30 20 10, 20 15 11, 20 25 12, 30 20 13)))';
  const geo = parseWKTGeometry(wktStr);
  expect(geo).toEqual({
    type: 'MultiPolygon',
    is3D: true,
    coordinates: [
      [
        [
          { x: 40, y: 40, z: 0 },
          { x: 20, y: 45, z: 1 },
          { x: 45, y: 30, z: 2 },
          { x: 40, y: 40, z: 3 },
        ],
      ],
      [
        [
          { x: 20, y: 35, z: 4 },
          { x: 10, y: 30, z: 5 },
          { x: 10, y: 10, z: 6 },
          { x: 30, y: 5, z: 7 },
          { x: 45, y: 20, z: 8 },
          { x: 20, y: 35, z: 9 },
        ],
        [
          { x: 30, y: 20, z: 10 },
          { x: 20, y: 15, z: 11 },
          { x: 20, y: 25, z: 12 },
          { x: 30, y: 20, z: 13 },
        ],
      ],
    ],
  });

  const wktStr2 =
    'MULTIPOLYGON Z ((((40 40 0), (20 45 1), (45 30 2), (40 40 3))),(((20 35 4), (10 30 5), (10 10 6), (30 5 7), (45 20 8), (20 35 9)),((30 20 10), (20 15 11), (20 25 12), (30 20 13))))';
  const geo2 = parseWKTGeometry(wktStr2);
  expect(geo2).toEqual({
    type: 'MultiPolygon',
    is3D: true,
    coordinates: [
      [
        [
          { x: 40, y: 40, z: 0 },
          { x: 20, y: 45, z: 1 },
          { x: 45, y: 30, z: 2 },
          { x: 40, y: 40, z: 3 },
        ],
      ],
      [
        [
          { x: 20, y: 35, z: 4 },
          { x: 10, y: 30, z: 5 },
          { x: 10, y: 10, z: 6 },
          { x: 30, y: 5, z: 7 },
          { x: 45, y: 20, z: 8 },
          { x: 20, y: 35, z: 9 },
        ],
        [
          { x: 30, y: 20, z: 10 },
          { x: 20, y: 15, z: 11 },
          { x: 20, y: 25, z: 12 },
          { x: 30, y: 20, z: 13 },
        ],
      ],
    ],
  });
});

test('parseWKTObject base case', () => {
  const wktStr =
    'PROJCS["NZGD49 / New Zealand Map Grid",GEOGCS["NZGD49",DATUM["New_Zealand_Geodetic_Datum_1949",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],AUTHORITY["EPSG","6272"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4272"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["New_Zealand_Map_Grid"],PARAMETER["latitude_of_origin",-41],PARAMETER["central_meridian",173],PARAMETER["false_easting",2510000],PARAMETER["false_northing",6023150],AUTHORITY["EPSG","27200"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]';
  const obj = parseWKTObject(wktStr);
  expect(obj).toEqual([
    'PROJCS',
    [
      'NZGD49 / New Zealand Map Grid',
      'GEOGCS',
      [
        'NZGD49',
        'DATUM',
        [
          'New_Zealand_Geodetic_Datum_1949',
          'SPHEROID',
          ['International 1924', '6378388', '297', 'AUTHORITY', ['EPSG', '7022']],
          'TOWGS84',
          ['59.47', '-5.04', '187.44', '0.47', '-0.1', '1.024', '-4.5993'],
          'AUTHORITY',
          ['EPSG', '6272'],
        ],
        'PRIMEM',
        ['Greenwich', '0', 'AUTHORITY', ['EPSG', '8901']],
        'UNIT',
        ['degree', '0.01745329251994328', 'AUTHORITY', ['EPSG', '9122']],
        'AUTHORITY',
        ['EPSG', '4272'],
      ],
      'UNIT',
      ['metre', '1', 'AUTHORITY', ['EPSG', '9001']],
      'PROJECTION',
      ['New_Zealand_Map_Grid'],
      'PARAMETER',
      ['latitude_of_origin', '-41'],
      'PARAMETER',
      ['central_meridian', '173'],
      'PARAMETER',
      ['false_easting', '2510000'],
      'PARAMETER',
      ['false_northing', '6023150'],
      'AUTHORITY',
      ['EPSG', '27200'],
      'AXIS',
      ['Easting', 'EAST'],
      'AXIS',
      ['Northing', 'NORTH'],
    ],
  ]);
});

test('parse fixture 0', () => {
  expect(
    parseWKTProjection(
      'PROJCS["NZGD49 / New Zealand Map Grid",GEOGCS["NZGD49",DATUM["New_Zealand_Geodetic_Datum_1949",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],AUTHORITY["EPSG","6272"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4272"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["New_Zealand_Map_Grid"],PARAMETER["latitude_of_origin",-41],PARAMETER["central_meridian",173],PARAMETER["false_easting",2510000],PARAMETER["false_northing",6023150],AUTHORITY["EPSG","27200"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]',
    ),
  ).toEqual({
    type: 'PROJCS',
    name: 'NZGD49 / New Zealand Map Grid',
    GEOGCS: {
      name: 'NZGD49',
      DATUM: {
        name: 'New_Zealand_Geodetic_Datum_1949',
        SPHEROID: {
          name: 'International 1924',
          a: 6378388,
          rf: 297,
          AUTHORITY: {
            EPSG: '7022',
          },
        },
        TOWGS84: [59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993],
        AUTHORITY: {
          EPSG: '6272',
        },
      },
      PRIMEM: {
        name: 'Greenwich',
        convert: 0,
        AUTHORITY: {
          EPSG: '8901',
        },
      },
      UNIT: {
        name: 'degree',
        convert: 0.01745329251994328,
        AUTHORITY: {
          EPSG: '9122',
        },
      },
      AUTHORITY: {
        EPSG: '4272',
      },
    },
    UNIT: {
      name: 'metre',
      convert: 1,
      AUTHORITY: {
        EPSG: '9001',
      },
    },
    PROJECTION: 'New_Zealand_Map_Grid',
    latitude_of_origin: -41,
    central_meridian: 173,
    false_easting: 2510000,
    false_northing: 6023150,
    AUTHORITY: {
      EPSG: '27200',
    },
    AXIS: [
      ['Easting', 'EAST'],
      ['Northing', 'NORTH'],
    ],
    projName: 'New_Zealand_Map_Grid',
    units: 'meter',
    to_meter: 1,
    toMeter: 1,
    datumCode: 'nzgd49',
    datum_params: [59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993],
    datumParams: [59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993],
    ellps: 'intl',
    a: 6378388,
    rf: 297,
    x0: 2510000,
    y0: 6023150,
    long0: 3.01941960595019,
    lat0: -0.715584993317675,
    axis: 'enu',
    srsCode: 'NZGD49 / New Zealand Map Grid',
  });
});

test('parse fixture 1', () => {
  expect(
    parseWKTProjection(
      'PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83",DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",42.68333333333333],PARAMETER["standard_parallel_2",41.71666666666667],PARAMETER["latitude_of_origin",41],PARAMETER["central_meridian",-71.5],PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]',
    ),
  ).toEqual({
    type: 'PROJCS',
    name: 'NAD83 / Massachusetts Mainland',
    GEOGCS: {
      name: 'NAD83',
      DATUM: {
        name: 'North_American_Datum_1983',
        SPHEROID: {
          name: 'GRS 1980',
          a: 6378137,
          rf: 298.257222101,
          AUTHORITY: {
            EPSG: '7019',
          },
        },
        AUTHORITY: {
          EPSG: '6269',
        },
      },
      PRIMEM: {
        name: 'Greenwich',
        convert: 0,
        AUTHORITY: {
          EPSG: '8901',
        },
      },
      UNIT: {
        name: 'degree',
        convert: 0.01745329251994328,
        AUTHORITY: {
          EPSG: '9122',
        },
      },
      AUTHORITY: {
        EPSG: '4269',
      },
    },
    UNIT: {
      name: 'metre',
      convert: 1,
      AUTHORITY: {
        EPSG: '9001',
      },
    },
    PROJECTION: 'Lambert_Conformal_Conic_2SP',
    standard_parallel_1: 42.68333333333333,
    standard_parallel_2: 41.71666666666667,
    latitude_of_origin: 41,
    central_meridian: -71.5,
    false_easting: 200000,
    false_northing: 750000,
    AUTHORITY: {
      EPSG: '26986',
    },
    AXIS: [
      ['X', 'EAST'],
      ['Y', 'NORTH'],
    ],
    projName: 'Lambert_Conformal_Conic_2SP',
    units: 'meter',
    to_meter: 1,
    toMeter: 1,
    datumCode: 'north_american_datum_1983',
    ellps: 'GRS 1980',
    a: 6378137,
    rf: 298.257222101,
    x0: 200000,
    y0: 750000,
    long0: -1.2479104151759457,
    lat0: 0.715584993317675,
    lat1: 0.744964702392913,
    lat2: 0.7280931862903011,
    axis: 'enu',
    srsCode: 'NAD83 / Massachusetts Mainland',
  });
});

test('parse fixture 2', () => {
  expect(
    parseWKTProjection(
      'PROJCS["ETRS89 / ETRS-LAEA",GEOGCS["ETRS89",DATUM["European_Terrestrial_Reference_System_1989",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6258"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4258"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Azimuthal_Equal_Area"],PARAMETER["latitude_of_center",52],PARAMETER["longitude_of_center",10],PARAMETER["false_easting",4321000],PARAMETER["false_northing",3210000],AUTHORITY["EPSG","3035"],AXIS["X",EAST],AXIS["Y",NORTH]]',
    ),
  ).toEqual({
    type: 'PROJCS',
    name: 'ETRS89 / ETRS-LAEA',
    GEOGCS: {
      name: 'ETRS89',
      DATUM: {
        name: 'European_Terrestrial_Reference_System_1989',
        SPHEROID: {
          name: 'GRS 1980',
          a: 6378137,
          rf: 298.257222101,
          AUTHORITY: {
            EPSG: '7019',
          },
        },
        AUTHORITY: {
          EPSG: '6258',
        },
      },
      PRIMEM: {
        name: 'Greenwich',
        convert: 0,
        AUTHORITY: {
          EPSG: '8901',
        },
      },
      UNIT: {
        name: 'degree',
        convert: 0.01745329251994328,
        AUTHORITY: {
          EPSG: '9122',
        },
      },
      AUTHORITY: {
        EPSG: '4258',
      },
    },
    UNIT: {
      name: 'metre',
      convert: 1,
      AUTHORITY: {
        EPSG: '9001',
      },
    },
    PROJECTION: 'Lambert_Azimuthal_Equal_Area',
    latitude_of_center: 52,
    longitude_of_center: 10,
    false_easting: 4321000,
    false_northing: 3210000,
    AUTHORITY: {
      EPSG: '3035',
    },
    AXIS: [
      ['X', 'EAST'],
      ['Y', 'NORTH'],
    ],
    projName: 'Lambert_Azimuthal_Equal_Area',
    axis: 'enu',
    units: 'meter',
    to_meter: 1,
    toMeter: 1,
    datumCode: 'european_terrestrial_reference_system_1989',
    ellps: 'GRS 1980',
    a: 6378137,
    rf: 298.257222101,
    lat0: 0.9075712110370514,
    longc: 0.17453292519943295,
    x0: 4321000,
    y0: 3210000,
    srsCode: 'ETRS89 / ETRS-LAEA',
    long0: 0.17453292519943295,
  });
});

test('parse fixture 3', () => {
  expect(
    parseWKTProjection(
      'GEOGCS["ETRS89",DATUM["European Terrestrial Reference System 1989 ensemble",SPHEROID["GRS 1980",6378137,298.2572221,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6258"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9102"]],AXIS["Lat",north],AXIS["Lon",east],AUTHORITY["EPSG","4258"]]',
    ),
  ).toEqual({
    type: 'GEOGCS',
    name: 'ETRS89',
    DATUM: {
      name: 'European Terrestrial Reference System 1989 ensemble',
      SPHEROID: {
        name: 'GRS 1980',
        a: 6378137,
        rf: 298.2572221,
        AUTHORITY: {
          EPSG: '7019',
        },
      },
      AUTHORITY: {
        EPSG: '6258',
      },
    },
    PRIMEM: {
      name: 'Greenwich',
      convert: 0,
      AUTHORITY: {
        EPSG: '8901',
      },
    },
    UNIT: {
      name: 'degree',
      convert: 0.0174532925199433,
      AUTHORITY: {
        EPSG: '9102',
      },
    },
    AXIS: [
      ['Lat', 'north'],
      ['Lon', 'east'],
    ],
    AUTHORITY: {
      EPSG: '4258',
    },
    projName: 'longlat',
    units: 'degree',
    to_meter: 111319.4907932736,
    toMeter: 111319.4907932736,
    datumCode: 'european terrestrial reference system 1989 ensemble',
    ellps: 'GRS 1980',
    a: 6378137,
    rf: 298.2572221,
    axis: 'neu',
    srsCode: 'ETRS89',
  });
});

test('parse fixture 4', () => {
  expect(
    parseWKTProjection(
      'COMPD_CS["unknown",PROJCS["NAD83 / Texas North Central",GEOGCS["NAD83",DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101004,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["latitude_of_origin",31.6666666666667],PARAMETER["central_meridian",-98.5],PARAMETER["standard_parallel_1",32.1333333333333],PARAMETER["standard_parallel_2",33.9666666666667],PARAMETER["false_easting",1968500],PARAMETER["false_northing",6561666.66666667],UNIT["US survey foot",0.304800609601219,AUTHORITY["EPSG","9003"]],AXIS["Easting",EAST],AXIS["Northing",NORTH],AUTHORITY["EPSG","32138"]],VERT_CS["NAVD88 height (ftUS)",VERT_DATUM["North American Vertical Datum 1988",2005,AUTHORITY["EPSG","5103"]],UNIT["US survey foot",0.304800609601219,AUTHORITY["EPSG","9003"]],AXIS["Up",UP],AUTHORITY["EPSG","6360"]]]',
    ),
  ).toEqual({
    type: 'COMPD_CS',
    name: 'unknown',
    PROJCS: {
      name: 'NAD83 / Texas North Central',
      projName: 'Lambert_Conformal_Conic_2SP',
      GEOGCS: {
        name: 'NAD83',
        DATUM: {
          name: 'North_American_Datum_1983',
          SPHEROID: {
            name: 'GRS 1980',
            a: 6378137,
            rf: 298.257222101004,
            AUTHORITY: {
              EPSG: '7019',
            },
          },
          AUTHORITY: {
            EPSG: '6269',
          },
        },
        PRIMEM: {
          name: 'Greenwich',
          convert: 0,
          AUTHORITY: {
            EPSG: '',
          },
        },
        UNIT: {
          name: 'degree',
          convert: 0.0174532925199433,
          AUTHORITY: {
            EPSG: '9122',
          },
        },
        AUTHORITY: {
          EPSG: '4269',
        },
      },
      PROJECTION: 'Lambert_Conformal_Conic_2SP',
      latitude_of_origin: 31.6666666666667,
      central_meridian: -98.5,
      standard_parallel_1: 32.1333333333333,
      standard_parallel_2: 33.9666666666667,
      false_easting: 1968500,
      false_northing: 6561666.66666667,
      UNIT: {
        name: 'US survey foot',
        convert: 0.304800609601219,
        AUTHORITY: {
          EPSG: '9003',
        },
      },
      AXIS: [
        ['Easting', 'EAST'],
        ['Northing', 'NORTH'],
      ],
      AUTHORITY: {
        EPSG: '32138',
      },
    },
    VERT_CS: {
      name: 'NAVD88 height (ftUS)',
      VERT_DATUM: {
        name: 'North American Vertical Datum 1988',
        convert: 2005,
        AUTHORITY: {
          EPSG: '5103',
        },
      },
      UNIT: {
        name: 'US survey foot',
        convert: 0.304800609601219,
        AUTHORITY: {
          EPSG: '9003',
        },
      },
      AXIS: [['Up', 'UP']],
      AUTHORITY: {
        EPSG: '6360',
      },
    },
    srsCode: 'unknown',
  });
});

test('parse fixture 5', () => {
  expect(
    parseWKTProjection(
      'PROJCS["WGS 84 / Antarctic Polar Stereographic",GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["Polar_Stereographic"],PARAMETER["latitude_of_origin",-71],PARAMETER["central_meridian",0],PARAMETER["scale_factor",1],PARAMETER["false_easting",0],PARAMETER["false_northing",0],AUTHORITY["EPSG","3031"],AXIS["Easting",UNKNOWN],AXIS["Northing",UNKNOWN]]',
    ),
  ).toEqual({
    type: 'PROJCS',
    name: 'WGS 84 / Antarctic Polar Stereographic',
    GEOGCS: {
      name: 'WGS 84',
      DATUM: {
        name: 'WGS_1984',
        SPHEROID: {
          name: 'WGS 84',
          a: 6378137,
          rf: 298.257223563,
          AUTHORITY: {
            EPSG: '7030',
          },
        },
        AUTHORITY: {
          EPSG: '6326',
        },
      },
      PRIMEM: {
        name: 'Greenwich',
        convert: 0,
        AUTHORITY: {
          EPSG: '8901',
        },
      },
      UNIT: {
        name: 'degree',
        convert: 0.01745329251994328,
        AUTHORITY: {
          EPSG: '9122',
        },
      },
      AUTHORITY: {
        EPSG: '4326',
      },
    },
    UNIT: {
      name: 'metre',
      convert: 1,
      AUTHORITY: {
        EPSG: '9001',
      },
    },
    PROJECTION: 'Polar_Stereographic',
    latitude_of_origin: -71,
    central_meridian: 0,
    scale_factor: 1,
    scaleFactor: 1,
    false_easting: 0,
    false_northing: 0,
    AUTHORITY: {
      EPSG: '3031',
    },
    AXIS: [
      ['Easting', 'UNKNOWN'],
      ['Northing', 'UNKNOWN'],
    ],
    projName: 'Polar_Stereographic',
    axis: 'enu',
    units: 'meter',
    to_meter: 1,
    toMeter: 1,
    datumCode: 'wgs84',
    ellps: 'WGS 84',
    a: 6378137,
    rf: 298.257223563,
    k0: 1,
    x0: 0,
    y0: 0,
    long0: 0,
    lat0: -1.5707963267948966,
    srsCode: 'WGS 84 / Antarctic Polar Stereographic',
    lat_ts: -1.239183768915974,
    latTs: -1.239183768915974,
  });
});
