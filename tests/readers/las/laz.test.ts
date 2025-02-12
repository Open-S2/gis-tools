import { FileReader } from '../../../src/file';
import {
  EPSG_26915,
  LASZipReader,
  LambertConformalConic,
  NewLineDelimitedJSONReader,
  TransverseMercator,
} from '../../../src';
import { expect, test } from 'bun:test';

test('LASzipReader - 1.2_0 ZIP', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_0.laz`);
  const lazReader = new LASZipReader(fileReader, [TransverseMercator], { EPSG_26915 });
  expect(lazReader.header).toEqual({
    encoding: 16,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 36,
    fileCreationYear: 2025,
    generatingSoftware: 'PDAL 2.8.3 (Releas)',
    headerSize: 375,
    majorVersion: 1,
    maxX: 470692.44,
    maxY: 4602888.9,
    maxZ: 16,
    minX: 470692.44,
    minY: 4602888.9,
    minZ: 16,
    minorVersion: 4,
    numPoints: 1,
    numPointsByReturn: [0, 4294967296, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    numVariableLengthRecords: 3,
    offsetToPoints: 1815,
    pointDataFormatID: 135,
    pointDataRecordLength: 36,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'PDAL',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lazReader.variableLengthRecords).toEqual({
    '2112': {
      data: new DataView(
        new Uint8Array([
          80, 82, 79, 74, 67, 83, 91, 34, 78, 65, 68, 56, 51, 32, 47, 32, 85, 84, 77, 32, 122, 111,
          110, 101, 32, 49, 53, 78, 34, 44, 71, 69, 79, 71, 67, 83, 91, 34, 78, 65, 68, 56, 51, 34,
          44, 68, 65, 84, 85, 77, 91, 34, 78, 111, 114, 116, 104, 95, 65, 109, 101, 114, 105, 99,
          97, 110, 95, 68, 97, 116, 117, 109, 95, 49, 57, 56, 51, 34, 44, 83, 80, 72, 69, 82, 79,
          73, 68, 91, 34, 71, 82, 83, 32, 49, 57, 56, 48, 34, 44, 54, 51, 55, 56, 49, 51, 55, 44,
          50, 57, 56, 46, 50, 53, 55, 50, 50, 50, 49, 48, 49, 44, 65, 85, 84, 72, 79, 82, 73, 84,
          89, 91, 34, 69, 80, 83, 71, 34, 44, 34, 55, 48, 49, 57, 34, 93, 93, 44, 65, 85, 84, 72,
          79, 82, 73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44, 34, 54, 50, 54, 57, 34, 93, 93, 44,
          80, 82, 73, 77, 69, 77, 91, 34, 71, 114, 101, 101, 110, 119, 105, 99, 104, 34, 44, 48, 44,
          65, 85, 84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44, 34, 56, 57, 48, 49,
          34, 93, 93, 44, 85, 78, 73, 84, 91, 34, 100, 101, 103, 114, 101, 101, 34, 44, 48, 46, 48,
          49, 55, 52, 53, 51, 50, 57, 50, 53, 49, 57, 57, 52, 51, 51, 44, 65, 85, 84, 72, 79, 82,
          73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44, 34, 57, 49, 50, 50, 34, 93, 93, 44, 65, 85,
          84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44, 34, 52, 50, 54, 57, 34, 93,
          93, 44, 80, 82, 79, 74, 69, 67, 84, 73, 79, 78, 91, 34, 84, 114, 97, 110, 115, 118, 101,
          114, 115, 101, 95, 77, 101, 114, 99, 97, 116, 111, 114, 34, 93, 44, 80, 65, 82, 65, 77,
          69, 84, 69, 82, 91, 34, 108, 97, 116, 105, 116, 117, 100, 101, 95, 111, 102, 95, 111, 114,
          105, 103, 105, 110, 34, 44, 48, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 99,
          101, 110, 116, 114, 97, 108, 95, 109, 101, 114, 105, 100, 105, 97, 110, 34, 44, 45, 57,
          51, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 115, 99, 97, 108, 101, 95, 102,
          97, 99, 116, 111, 114, 34, 44, 48, 46, 57, 57, 57, 54, 93, 44, 80, 65, 82, 65, 77, 69, 84,
          69, 82, 91, 34, 102, 97, 108, 115, 101, 95, 101, 97, 115, 116, 105, 110, 103, 34, 44, 53,
          48, 48, 48, 48, 48, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 102, 97, 108, 115,
          101, 95, 110, 111, 114, 116, 104, 105, 110, 103, 34, 44, 48, 93, 44, 85, 78, 73, 84, 91,
          34, 109, 101, 116,
        ]).buffer,
      ),
      description: 'OGR variant of OpenGIS WKT SRS',
      recordID: 2112,
      recordLength: 616,
      reserved: 0,
      userID: 'liblas',
    },
    '22204': {
      data: new DataView(
        new Uint8Array([
          3, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
          255, 255, 255, 255, 255, 255, 255, 255, 2, 0, 10, 0, 30, 0, 3, 0, 11, 0, 6, 0, 3, 0,
        ]).buffer,
      ),
      description: 'http://laszip.org',
      recordID: 22204,
      recordLength: 46,
      reserved: 0,
      userID: 'laszip encoded',
    },
  });
  expect(lazReader.lazHeader).toEqual({
    chunkSize: 50000,
    coder: 0,
    compressor: 3,
    items: [
      { size: 30, type: 10, version: 3 },
      { size: 6, type: 11, version: 3 },
    ],
    numItems: 2,
    numSpecialEvlrs: -1,
    offsetSpecialEvlrs: -1,
    options: 0,
    versionMajor: 3,
    versionMinor: 4,
    versionRevision: 3,
  });
  expect(lazReader.wkt).toEqual(
    'PROJCS["NAD83 / UTM zone 15N",GEOGCS["NAD83",DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],PROJECTION["Transverse_Mercator"],PARAMETER["latitude_of_origin",0],PARAMETER["central_meridian",-93],PARAMETER["scale_factor",0.9996],PARAMETER["false_easting",500000],PARAMETER["false_northing",0],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["Easting",EAST],AXIS["Northing",NORTH],AUTHORITY["EPSG","26915"]]\u0000',
  );
  const data = await Array.fromAsync(lazReader);
  expect(data.length).toEqual(1);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          scannerChannel: 0,
          edgeOfFlightLine: 0,
          intensity: 0,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          gpsTime: 0,
          // scanAngleRank: -13,
          userData: 0,
          rgba: { r: 0, g: 0, b: 0, a: 255 },
        },
        x: -93.35156259019986,
        y: 41.577148395415115,
        z: 16,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASzipReader - simple ZIP', async () => {
  const jsonldCompare = new NewLineDelimitedJSONReader(
    new FileReader(`${__dirname}/fixtures/simple.jsonld`),
  );
  const comparePoints = await Array.fromAsync(jsonldCompare);
  expect(comparePoints.length).toEqual(1_065);
  const lazReader = new LASZipReader(new FileReader(`${__dirname}/fixtures/simple.laz`), [], {});
  expect(lazReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 0,
    fileCreationYear: 0,
    generatingSoftware: 'las2las (version 221128)',
    headerSize: 227,
    majorVersion: 1,
    maxX: 638982.55,
    maxY: 853535.43,
    maxZ: 586.38,
    minX: 635619.85,
    minY: 848899.7000000001,
    minZ: 406.59000000000003,
    minorVersion: 2,
    numPoints: 1065,
    numPointsByReturn: [925, 114, 21, 5, 0],
    numVariableLengthRecords: 1,
    offsetToPoints: 333,
    pointDataFormatID: 131,
    pointDataRecordLength: 34,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'LAStools (c) by rapidlasso GmbH',
    waveformDataPacketOffset: 0,
    xOffset: -0,
    xScaleFactor: 0.01,
    yOffset: -0,
    yScaleFactor: 0.01,
    zOffset: -0,
    zScaleFactor: 0.01,
  });
  expect(lazReader.variableLengthRecords).toEqual({
    '22204': {
      data: new DataView(
        new Uint8Array([
          2, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
          255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 6, 0, 20, 0, 2, 0, 7, 0, 8, 0, 2, 0, 8, 0,
          6, 0, 2, 0,
        ]).buffer,
      ),
      description: 'by laszip of LAStools (221128)',
      recordID: 22204,
      recordLength: 52,
      reserved: 43707,
      userID: 'laszip encoded',
    },
  });
  expect(lazReader.lazHeader).toEqual({
    chunkSize: 50000,
    coder: 0,
    compressor: 2,
    items: [
      { size: 20, type: 6, version: 2 },
      { size: 8, type: 7, version: 2 },
      { size: 6, type: 8, version: 2 },
    ],
    numItems: 3,
    numSpecialEvlrs: -1,
    offsetSpecialEvlrs: -1,
    options: 0,
    versionMajor: 3,
    versionMinor: 4,
    versionRevision: 3,
  });
  expect(lazReader.wkt).toBeUndefined();
  expect(lazReader.GeoKeyDirectory).toBeUndefined();
  const data = await Array.fromAsync(lazReader);
  expect(data.length).toEqual(1_065);
  for (let i = 0; i < data.length; i++) {
    const { type: compareType, coordinates: compareCoordinates } = comparePoints[i].geometry;
    if (compareType !== 'Point') throw new Error('not a point');
    const { type, coordinates } = data[i].geometry;
    if (type !== 'Point') throw new Error('not a point');
    expect(coordinates.x).toBeCloseTo(compareCoordinates.x);
    expect(coordinates.y).toBeCloseTo(compareCoordinates.y);
    expect(coordinates.z).toBeCloseTo(compareCoordinates.z!);
    // @ts-expect-error - this is ok
    expect(coordinates.m.intensity).toEqual(compareCoordinates.m.intensity);
    // @ts-expect-error - this is ok
    expect(coordinates.m?.gpsTime).toBeCloseTo(compareCoordinates.m.gps_time);
    // @ts-expect-error - this is ok
    expect(coordinates.m.rgba).toEqual(compareCoordinates.m.rgba);
  }
});

test('LASzipReader - simple V3 ZIP', async () => {
  const jsonldCompare = new NewLineDelimitedJSONReader(
    new FileReader(`${__dirname}/fixtures/simple.jsonld`),
  );
  const comparePoints = await Array.fromAsync(jsonldCompare);
  expect(comparePoints.length).toEqual(1_065);
  const lazReader = new LASZipReader(new FileReader(`${__dirname}/fixtures/simpleV3.laz`), [], {});
  expect(lazReader.header).toEqual({
    encoding: 16,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 41,
    fileCreationYear: 2025,
    generatingSoftware: 'PDAL 2.8.3 (Releas)',
    headerSize: 375,
    majorVersion: 1,
    maxX: 638982.55,
    maxY: 853535.43,
    maxZ: 586.38,
    minX: 635619.85,
    minY: 848899.7000000001,
    minZ: 406.59000000000003,
    minorVersion: 4,
    numPoints: 1065,
    numPointsByReturn: [
      3972844748800, 489626271744, 90194313216, 21474836480, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    numVariableLengthRecords: 1,
    offsetToPoints: 475,
    pointDataFormatID: 135,
    pointDataRecordLength: 36,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'PDAL',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lazReader.variableLengthRecords).toEqual({
    '22204': {
      data: new DataView(
        new Uint8Array([
          3, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
          255, 255, 255, 255, 255, 255, 255, 255, 2, 0, 10, 0, 30, 0, 3, 0, 11, 0, 6, 0, 3, 0,
        ]).buffer,
      ),
      description: 'http://laszip.org',
      recordID: 22204,
      recordLength: 46,
      reserved: 0,
      userID: 'laszip encoded',
    },
  });
  expect(lazReader.lazHeader).toEqual({
    chunkSize: 50000,
    coder: 0,
    compressor: 3,
    items: [
      { size: 30, type: 10, version: 3 },
      { size: 6, type: 11, version: 3 },
    ],
    numItems: 2,
    numSpecialEvlrs: -1,
    offsetSpecialEvlrs: -1,
    options: 0,
    versionMajor: 3,
    versionMinor: 4,
    versionRevision: 3,
  });
  expect(lazReader.wkt).toBeUndefined();
  expect(lazReader.GeoKeyDirectory).toBeUndefined();
  const data = await Array.fromAsync(lazReader);
  expect(data.length).toEqual(1_065);
  for (let i = 0; i < data.length; i++) {
    const { type: compareType, coordinates: compareCoordinates } = comparePoints[i].geometry;
    if (compareType !== 'Point') throw new Error('not a point');
    const { type, coordinates } = data[i].geometry;
    if (type !== 'Point') throw new Error('not a point');
    expect(coordinates.x).toBeCloseTo(compareCoordinates.x);
    expect(coordinates.y).toBeCloseTo(compareCoordinates.y);
    expect(coordinates.z).toBeCloseTo(compareCoordinates.z!);
    // @ts-expect-error - this is ok
    expect(coordinates.m.intensity).toEqual(compareCoordinates.m.intensity);
    // @ts-expect-error - this is ok
    expect(coordinates.m?.gpsTime).toBeCloseTo(compareCoordinates.m.gps_time);
    // @ts-expect-error - this is ok
    expect(coordinates.m.rgba).toEqual(compareCoordinates.m.rgba);
  }
});

test('LASzipReader - autzen ZIP', async () => {
  const jsonldCompare = new NewLineDelimitedJSONReader(
    new FileReader(`${__dirname}/fixtures/autzen_trim.jsonld`),
  );
  const comparePoints = await Array.fromAsync(jsonldCompare);
  expect(comparePoints.length).toEqual(110_000);
  const lazReader = new LASZipReader(
    new FileReader(`${__dirname}/fixtures/autzen_trim.laz`),
    [LambertConformalConic],
    {},
    [],
    true,
  );
  expect(lazReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 253,
    fileCreationYear: 2015,
    generatingSoftware: 'PDAL 1.0.0 (9e8465)',
    headerSize: 227,
    majorVersion: 1,
    maxX: 637179.2200000001,
    maxY: 849497.9,
    maxZ: 520.51,
    minX: 636001.76,
    minY: 848935.2000000001,
    minZ: 406.26,
    minorVersion: 2,
    numPoints: 110000,
    numPointsByReturn: [99257, 9021, 1623, 99, 0],
    numVariableLengthRecords: 6,
    offsetToPoints: 2144,
    pointDataFormatID: 131,
    pointDataRecordLength: 34,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'PDAL',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lazReader.variableLengthRecords).toEqual({
    '2112': {
      data: new DataView(
        new Uint8Array([
          80, 82, 79, 74, 67, 83, 91, 34, 78, 65, 68, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 95,
          76, 97, 109, 98, 101, 114, 116, 95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67,
          111, 110, 105, 99, 34, 44, 71, 69, 79, 71, 67, 83, 91, 34, 71, 67, 83, 95, 78, 111, 114,
          116, 104, 95, 65, 109, 101, 114, 105, 99, 97, 110, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78,
          34, 44, 68, 65, 84, 85, 77, 91, 34, 78, 65, 68, 56, 51, 95, 72, 105, 103, 104, 95, 65, 99,
          99, 117, 114, 97, 99, 121, 95, 82, 101, 103, 105, 111, 110, 97, 108, 95, 78, 101, 116,
          119, 111, 114, 107, 34, 44, 83, 80, 72, 69, 82, 79, 73, 68, 91, 34, 71, 82, 83, 95, 49,
          57, 56, 48, 34, 44, 54, 51, 55, 56, 49, 51, 55, 44, 50, 57, 56, 46, 50, 53, 55, 50, 50,
          50, 49, 48, 49, 44, 65, 85, 84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44,
          34, 55, 48, 49, 57, 34, 93, 93, 44, 65, 85, 84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80,
          83, 71, 34, 44, 34, 54, 49, 53, 50, 34, 93, 93, 44, 80, 82, 73, 77, 69, 77, 91, 34, 71,
          114, 101, 101, 110, 119, 105, 99, 104, 34, 44, 48, 93, 44, 85, 78, 73, 84, 91, 34, 100,
          101, 103, 114, 101, 101, 34, 44, 48, 46, 48, 49, 55, 52, 53, 51, 50, 57, 50, 53, 49, 57,
          57, 52, 51, 51, 93, 93, 44, 80, 82, 79, 74, 69, 67, 84, 73, 79, 78, 91, 34, 76, 97, 109,
          98, 101, 114, 116, 95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67, 111, 110, 105,
          99, 95, 50, 83, 80, 34, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 115, 116, 97,
          110, 100, 97, 114, 100, 95, 112, 97, 114, 97, 108, 108, 101, 108, 95, 49, 34, 44, 52, 51,
          93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 115, 116, 97, 110, 100, 97, 114, 100,
          95, 112, 97, 114, 97, 108, 108, 101, 108, 95, 50, 34, 44, 52, 53, 46, 53, 93, 44, 80, 65,
          82, 65, 77, 69, 84, 69, 82, 91, 34, 108, 97, 116, 105, 116, 117, 100, 101, 95, 111, 102,
          95, 111, 114, 105, 103, 105, 110, 34, 44, 52, 49, 46, 55, 53, 93, 44, 80, 65, 82, 65, 77,
          69, 84, 69, 82, 91, 34, 99, 101, 110, 116, 114, 97, 108, 95, 109, 101, 114, 105, 100, 105,
          97, 110, 34, 44, 45, 49, 50, 48, 46, 53, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91,
          34, 102, 97, 108, 115, 101, 95, 101, 97, 115, 116, 105, 110, 103, 34, 44, 49, 51, 49, 50,
          51, 51, 53, 46, 57, 53, 56, 48, 48, 53, 50,
        ]).buffer,
      ),
      description: 'OGR variant of OpenGIS WKT SRS',
      recordID: 2112,
      recordLength: 593,
      reserved: 0,
      userID: 'liblas',
    },
    '22204': {
      data: new DataView(
        new Uint8Array([
          2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
          255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 6, 0, 20, 0, 2, 0, 7, 0, 8, 0, 2, 0, 8, 0,
          6, 0, 2, 0,
        ]).buffer,
      ),
      description: 'by laszip of LAStools (140915)',
      recordID: 22204,
      recordLength: 52,
      reserved: 43707,
      userID: 'laszip encoded',
    },
    '34735': {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 21, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          38, 0, 0, 0, 0, 8, 0, 0, 1, 0, 255, 127, 1, 8, 177, 135, 60, 0, 38, 0, 2, 8, 0, 0, 1, 0,
          8, 24, 6, 8, 0, 0, 1, 0, 142, 35, 9, 8, 176, 135, 1, 0, 7, 0, 11, 8, 176, 135, 1, 0, 6, 0,
          13, 8, 176, 135, 1, 0, 8, 0, 243, 11, 0, 0, 1, 0, 1, 0, 0, 12, 0, 0, 1, 0, 255, 127, 2,
          12, 0, 0, 1, 0, 255, 127, 3, 12, 0, 0, 1, 0, 8, 0, 4, 12, 0, 0, 1, 0, 42, 35, 6, 12, 176,
          135, 1, 0, 2, 0, 7, 12, 176, 135, 1, 0, 3, 0, 12, 12, 176, 135, 1, 0, 1, 0, 13, 12, 176,
          135, 1, 0, 0, 0, 14, 12, 176, 135, 1, 0, 4, 0, 15, 12, 176, 135, 1, 0, 5, 0, 0, 0, 0, 0,
          0, 0, 0, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoKeyDirectoryTag',
      recordID: 34735,
      recordLength: 184,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34736': {
      data: new DataView(
        new Uint8Array([
          0, 0, 0, 0, 0, 224, 68, 64, 0, 0, 0, 0, 0, 32, 94, 192, 0, 0, 0, 0, 0, 128, 69, 64, 0, 0,
          0, 0, 0, 192, 70, 64, 254, 212, 63, 245, 79, 6, 52, 65, 0, 0, 0, 0, 0, 0, 0, 0, 168, 249,
          235, 148, 29, 164, 114, 64, 0, 0, 0, 64, 166, 84, 88, 65, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoDoubleParamsTag',
      recordID: 34736,
      recordLength: 72,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34737': {
      data: new DataView(
        new Uint8Array([
          78, 65, 68, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 95, 76, 97, 109, 98, 101, 114, 116,
          95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67, 111, 110, 105, 99, 124, 71, 67, 83,
          32, 78, 97, 109, 101, 32, 61, 32, 71, 67, 83, 95, 78, 111, 114, 116, 104, 95, 65, 109,
          101, 114, 105, 99, 97, 110, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 124, 80, 114, 105,
          109, 101, 109, 32, 61, 32, 71, 114, 101, 101, 110, 119, 105, 99, 104, 124, 124, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoAsciiParamsTag',
      recordID: 34737,
      recordLength: 99,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lazReader.lazHeader).toEqual({
    chunkSize: 50000,
    coder: 0,
    compressor: 2,
    items: [
      { size: 20, type: 6, version: 2 },
      { size: 8, type: 7, version: 2 },
      { size: 6, type: 8, version: 2 },
    ],
    numItems: 3,
    numSpecialEvlrs: -1,
    offsetSpecialEvlrs: -1,
    options: 0,
    versionMajor: 2,
    versionMinor: 2,
    versionRevision: 0,
  });
  expect(lazReader.wkt).toEqual(
    'PROJCS["NAD_1983_HARN_Lambert_Conformal_Conic",GEOGCS["GCS_North_American_1983_HARN",DATUM["NAD83_High_Accuracy_Regional_Network",SPHEROID["GRS_1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6152"]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",43],PARAMETER["standard_parallel_2",45.5],PARAMETER["latitude_of_origin",41.75],PARAMETER["central_meridian",-120.5],PARAMETER["false_easting",1312335.958005249],PARAMETER["false_northing",0],UNIT["foot",0.3048,AUTHORITY["EPSG","9002"]]]\u0000',
  );
  expect(lazReader.GeoKeyDirectory).toEqual({
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GTCitationGeoKey: 'NAD_1983_HARN_Lambert_Conformal_Conic',
    GeographicTypeGeoKey: 32767,
    GeogCitationGeoKey: 'GCS Name = GCS_North_American_1983_HARN|Primem = Greenwich|',
    GeogGeodeticDatumGeoKey: 6152,
    GeogAngularUnitsGeoKey: 9102,
    GeogSemiMajorAxisGeoKey: 6378137,
    GeogInvFlatteningGeoKey: 298.257222101,
    GeogPrimeMeridianLongGeoKey: 0,
    // @ts-expect-error - this is ok
    undefined: 1,
    ProjectedCSTypeGeoKey: 32767,
    ProjectionGeoKey: 32767,
    ProjCoordTransGeoKey: 8,
    ProjLinearUnitsGeoKey: 9002,
    ProjStdParallel1GeoKey: 43,
    ProjStdParallel2GeoKey: 45.5,
    ProjFalseOriginLongGeoKey: -120.5,
    ProjFalseOriginLatGeoKey: 41.75,
    ProjFalseOriginEastingGeoKey: 1312335.958005249,
    ProjFalseOriginNorthingGeoKey: 0,
  });
  const data = await Array.fromAsync(lazReader);
  expect(data.length).toEqual(110_000);
  for (let i = 0; i < data.length; i++) {
    const { type: compareType, coordinates: compareCoordinates } = comparePoints[i].geometry;
    if (compareType !== 'Point') throw new Error('not a point');
    const { type, coordinates } = data[i].geometry;
    if (type !== 'Point') throw new Error('not a point');
    expect(coordinates.x).toBeCloseTo(compareCoordinates.x);
    expect(coordinates.y).toBeCloseTo(compareCoordinates.y);
    expect(coordinates.z).toBeCloseTo(compareCoordinates.z!);
    // @ts-expect-error - this is ok
    expect(coordinates.m.intensity).toEqual(compareCoordinates.m.intensity);
    // @ts-expect-error - this is ok
    expect(coordinates.m?.gpsTime).toBeCloseTo(compareCoordinates.m.gps_time);
    // @ts-expect-error - this is ok
    expect(coordinates.m.rgba).toEqual(compareCoordinates.m.rgba);
  }
});

test('LASzipReader - autzen v3 ZIP', async () => {
  const jsonldCompare = new NewLineDelimitedJSONReader(
    new FileReader(`${__dirname}/fixtures/autzen_trim.jsonld`),
  );
  const comparePoints = await Array.fromAsync(jsonldCompare);
  expect(comparePoints.length).toEqual(110_000);
  const lazReader = new LASZipReader(
    new FileReader(`${__dirname}/fixtures/autzen_trim_v3.laz`),
    [LambertConformalConic],
    {},
    [],
    true,
  );
  expect(lazReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 253,
    fileCreationYear: 2015,
    generatingSoftware: 'las2las64 (version 250207)',
    headerSize: 375,
    majorVersion: 1,
    maxX: 637179.22,
    maxY: 849497.9,
    maxZ: 520.51,
    minX: 636001.76,
    minY: 848935.2000000001,
    minZ: 406.26,
    minorVersion: 4,
    numPoints: 110000,
    numPointsByReturn: [
      426305568899072, 38744899977216, 6970731921408, 425201762304, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    numVariableLengthRecords: 6,
    offsetToPoints: 2286,
    pointDataFormatID: 136,
    pointDataRecordLength: 38,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'LAStools (c) by rapidlasso GmbH',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lazReader.variableLengthRecords).toEqual({
    '2112': {
      data: new DataView(
        new Uint8Array([
          80, 82, 79, 74, 67, 83, 91, 34, 78, 65, 68, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 95,
          76, 97, 109, 98, 101, 114, 116, 95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67,
          111, 110, 105, 99, 34, 44, 71, 69, 79, 71, 67, 83, 91, 34, 71, 67, 83, 95, 78, 111, 114,
          116, 104, 95, 65, 109, 101, 114, 105, 99, 97, 110, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78,
          34, 44, 68, 65, 84, 85, 77, 91, 34, 78, 65, 68, 56, 51, 95, 72, 105, 103, 104, 95, 65, 99,
          99, 117, 114, 97, 99, 121, 95, 82, 101, 103, 105, 111, 110, 97, 108, 95, 78, 101, 116,
          119, 111, 114, 107, 34, 44, 83, 80, 72, 69, 82, 79, 73, 68, 91, 34, 71, 82, 83, 95, 49,
          57, 56, 48, 34, 44, 54, 51, 55, 56, 49, 51, 55, 44, 50, 57, 56, 46, 50, 53, 55, 50, 50,
          50, 49, 48, 49, 44, 65, 85, 84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80, 83, 71, 34, 44,
          34, 55, 48, 49, 57, 34, 93, 93, 44, 65, 85, 84, 72, 79, 82, 73, 84, 89, 91, 34, 69, 80,
          83, 71, 34, 44, 34, 54, 49, 53, 50, 34, 93, 93, 44, 80, 82, 73, 77, 69, 77, 91, 34, 71,
          114, 101, 101, 110, 119, 105, 99, 104, 34, 44, 48, 93, 44, 85, 78, 73, 84, 91, 34, 100,
          101, 103, 114, 101, 101, 34, 44, 48, 46, 48, 49, 55, 52, 53, 51, 50, 57, 50, 53, 49, 57,
          57, 52, 51, 51, 93, 93, 44, 80, 82, 79, 74, 69, 67, 84, 73, 79, 78, 91, 34, 76, 97, 109,
          98, 101, 114, 116, 95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67, 111, 110, 105,
          99, 95, 50, 83, 80, 34, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 115, 116, 97,
          110, 100, 97, 114, 100, 95, 112, 97, 114, 97, 108, 108, 101, 108, 95, 49, 34, 44, 52, 51,
          93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91, 34, 115, 116, 97, 110, 100, 97, 114, 100,
          95, 112, 97, 114, 97, 108, 108, 101, 108, 95, 50, 34, 44, 52, 53, 46, 53, 93, 44, 80, 65,
          82, 65, 77, 69, 84, 69, 82, 91, 34, 108, 97, 116, 105, 116, 117, 100, 101, 95, 111, 102,
          95, 111, 114, 105, 103, 105, 110, 34, 44, 52, 49, 46, 55, 53, 93, 44, 80, 65, 82, 65, 77,
          69, 84, 69, 82, 91, 34, 99, 101, 110, 116, 114, 97, 108, 95, 109, 101, 114, 105, 100, 105,
          97, 110, 34, 44, 45, 49, 50, 48, 46, 53, 93, 44, 80, 65, 82, 65, 77, 69, 84, 69, 82, 91,
          34, 102, 97, 108, 115, 101, 95, 101, 97, 115, 116, 105, 110, 103, 34, 44, 49, 51, 49, 50,
          51, 51, 53, 46, 57, 53, 56, 48, 48, 53, 50,
        ]).buffer,
      ),
      description: 'OGR variant of OpenGIS WKT SRS',
      recordID: 2112,
      recordLength: 593,
      reserved: 0,
      userID: 'liblas',
    },
    '22204': {
      data: new DataView(
        new Uint8Array([
          2, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
          255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 6, 0, 20, 0, 2, 0, 7, 0, 8, 0, 2, 0, 8, 0,
          6, 0, 2, 0,
        ]).buffer,
      ),
      description: 'by laszip of LAStools (250207)',
      recordID: 22204,
      recordLength: 46,
      reserved: 0,
      userID: 'laszip encoded',
    },
    '34735': {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 21, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          38, 0, 0, 0, 0, 8, 0, 0, 1, 0, 255, 127, 1, 8, 177, 135, 60, 0, 38, 0, 2, 8, 0, 0, 1, 0,
          8, 24, 6, 8, 0, 0, 1, 0, 142, 35, 9, 8, 176, 135, 1, 0, 7, 0, 11, 8, 176, 135, 1, 0, 6, 0,
          13, 8, 176, 135, 1, 0, 8, 0, 243, 11, 0, 0, 1, 0, 1, 0, 0, 12, 0, 0, 1, 0, 255, 127, 2,
          12, 0, 0, 1, 0, 255, 127, 3, 12, 0, 0, 1, 0, 8, 0, 4, 12, 0, 0, 1, 0, 42, 35, 6, 12, 176,
          135, 1, 0, 2, 0, 7, 12, 176, 135, 1, 0, 3, 0, 12, 12, 176, 135, 1, 0, 1, 0, 13, 12, 176,
          135, 1, 0, 0, 0, 14, 12, 176, 135, 1, 0, 4, 0, 15, 12, 176, 135, 1, 0, 5, 0, 0, 0, 0, 0,
          0, 0, 0, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoKeyDirectoryTag',
      recordID: 34735,
      recordLength: 184,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34736': {
      data: new DataView(
        new Uint8Array([
          0, 0, 0, 0, 0, 224, 68, 64, 0, 0, 0, 0, 0, 32, 94, 192, 0, 0, 0, 0, 0, 128, 69, 64, 0, 0,
          0, 0, 0, 192, 70, 64, 254, 212, 63, 245, 79, 6, 52, 65, 0, 0, 0, 0, 0, 0, 0, 0, 168, 249,
          235, 148, 29, 164, 114, 64, 0, 0, 0, 64, 166, 84, 88, 65, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoDoubleParamsTag',
      recordID: 34736,
      recordLength: 72,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34737': {
      data: new DataView(
        new Uint8Array([
          78, 65, 68, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 95, 76, 97, 109, 98, 101, 114, 116,
          95, 67, 111, 110, 102, 111, 114, 109, 97, 108, 95, 67, 111, 110, 105, 99, 124, 71, 67, 83,
          32, 78, 97, 109, 101, 32, 61, 32, 71, 67, 83, 95, 78, 111, 114, 116, 104, 95, 65, 109,
          101, 114, 105, 99, 97, 110, 95, 49, 57, 56, 51, 95, 72, 65, 82, 78, 124, 80, 114, 105,
          109, 101, 109, 32, 61, 32, 71, 114, 101, 101, 110, 119, 105, 99, 104, 124, 124, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoAsciiParamsTag',
      recordID: 34737,
      recordLength: 99,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lazReader.lazHeader).toEqual({
    chunkSize: 50000,
    coder: 0,
    compressor: 3,
    items: [
      { size: 30, type: 10, version: 3 },
      { size: 8, type: 12, version: 3 },
    ],
    numItems: 2,
    numSpecialEvlrs: -1,
    offsetSpecialEvlrs: -1,
    options: 0,
    versionMajor: 3,
    versionMinor: 4,
    versionRevision: 4,
  });
  expect(lazReader.wkt).toEqual(
    'PROJCS["NAD_1983_HARN_Lambert_Conformal_Conic",GEOGCS["GCS_North_American_1983_HARN",DATUM["NAD83_High_Accuracy_Regional_Network",SPHEROID["GRS_1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6152"]],PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]],PROJECTION["Lambert_Conformal_Conic_2SP"],PARAMETER["standard_parallel_1",43],PARAMETER["standard_parallel_2",45.5],PARAMETER["latitude_of_origin",41.75],PARAMETER["central_meridian",-120.5],PARAMETER["false_easting",1312335.958005249],PARAMETER["false_northing",0],UNIT["foot",0.3048,AUTHORITY["EPSG","9002"]]]\u0000',
  );
  expect(lazReader.GeoKeyDirectory).toEqual({
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GTCitationGeoKey: 'NAD_1983_HARN_Lambert_Conformal_Conic',
    GeographicTypeGeoKey: 32767,
    GeogCitationGeoKey: 'GCS Name = GCS_North_American_1983_HARN|Primem = Greenwich|',
    GeogGeodeticDatumGeoKey: 6152,
    GeogAngularUnitsGeoKey: 9102,
    GeogSemiMajorAxisGeoKey: 6378137,
    GeogInvFlatteningGeoKey: 298.257222101,
    GeogPrimeMeridianLongGeoKey: 0,
    // @ts-expect-error - this is ok
    undefined: 1,
    ProjectedCSTypeGeoKey: 32767,
    ProjectionGeoKey: 32767,
    ProjCoordTransGeoKey: 8,
    ProjLinearUnitsGeoKey: 9002,
    ProjStdParallel1GeoKey: 43,
    ProjStdParallel2GeoKey: 45.5,
    ProjFalseOriginLongGeoKey: -120.5,
    ProjFalseOriginLatGeoKey: 41.75,
    ProjFalseOriginEastingGeoKey: 1312335.958005249,
    ProjFalseOriginNorthingGeoKey: 0,
  });
  const data = await Array.fromAsync(lazReader);
  expect(data.length).toEqual(110_000);
  for (let i = 0; i < data.length; i++) {
    const { type: compareType, coordinates: compareCoordinates } = comparePoints[i].geometry;
    if (compareType !== 'Point') throw new Error('not a point');
    const { type, coordinates } = data[i].geometry;
    if (type !== 'Point') throw new Error('not a point');
    expect(coordinates.x).toBeCloseTo(compareCoordinates.x);
    expect(coordinates.y).toBeCloseTo(compareCoordinates.y);
    expect(coordinates.z).toBeCloseTo(compareCoordinates.z!);
    // @ts-expect-error - this is ok
    expect(coordinates.m.intensity).toEqual(compareCoordinates.m.intensity);
    // @ts-expect-error - this is ok
    expect(coordinates.m?.gpsTime).toBeCloseTo(compareCoordinates.m.gps_time);
    // @ts-expect-error - this is ok
    expect(coordinates.m.rgba).toEqual(compareCoordinates.m.rgba);
  }
});
