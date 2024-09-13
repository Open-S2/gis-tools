import { expect, test } from 'bun:test';
import { parseWKTGeometry, parseWKTObject, parseWKTProjection } from '../../../src/readers/wkt';

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
    datumCode: 'nzgd49',
    datum_params: [59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993],
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
  });
});
