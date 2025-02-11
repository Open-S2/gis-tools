import { FileReader } from '../../../src/file';
import {
  EPSG_26915,
  EPSG_26916,
  EPSG_32617,
  LASReader,
  UniversalTransverseMercator,
} from '../../../src';
import { expect, test } from 'bun:test';

test('LASReader - spatialreference', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/srs.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_32617 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 0,
    fileCreationYear: 0,
    generatingSoftware: 'TerraScan',
    headerSize: 227,
    majorVersion: 1,
    maxX: 289818.5,
    maxY: 4320980.59,
    maxZ: 170.76,
    minX: 289814.15,
    minY: 4320978.61,
    minZ: 170.58,
    minorVersion: 0,
    numPoints: 10,
    numPointsByReturn: [10, 0, 0, 0, 0],
    numVariableLengthRecords: 3,
    offsetToPoints: 759,
    pointDataFormatID: 1,
    pointDataRecordLength: 28,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'MODIFIED',
    waveformDataPacketOffset: 0,
    xOffset: -0,
    xScaleFactor: 0.01,
    yOffset: -0,
    yScaleFactor: 0.01,
    zOffset: -0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    34735: {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 8, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 0, 12, 0, 0, 1, 0,
          105, 127, 4, 12, 0, 0, 1, 0, 41, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: '',
      recordID: 34735,
      recordLength: 72,
      reserved: 43707,
      userID: 'LASF_Projection',
    },
    34736: {
      data: new DataView(
        new Uint8Array([
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: '',
      recordID: 34736,
      recordLength: 40,
      reserved: 43707,
      userID: 'LASF_Projection',
    },
    34737: {
      data: new DataView(
        new Uint8Array([
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: '',
      recordID: 34737,
      recordLength: 256,
      reserved: 43707,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    ProjLinearUnitsGeoKey: 9001,
    ProjectedCSTypeGeoKey: 32617,
    // @ts-expect-error - Not sure why this is undefined
    undefined: 0,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(10);
  expect(data[0]).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          edgeOfFlightLine: 0,
          gpsTime: 499450.8059940542,
          intensity: 260,
          numberOfReturns: 6,
          pointSourceID: 0,
          returnNumber: 0,
          scanAngleRank: 0,
          userData: 0,
        },
        x: -83.42759776257799,
        y: 39.012599045239895,
        z: 170.76,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - 1.2_0', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_0.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'libLAS 1.2',
    headerSize: 227,
    majorVersion: 1,
    maxX: 470692.447538,
    maxY: 4602888.904642,
    maxZ: 16,
    minX: 470692.447538,
    minY: 4602888.904642,
    minZ: 16,
    minorVersion: 2,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 438,
    pointDataFormatID: 0,
    pointDataRecordLength: 20,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'libLAS',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    34735: {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 7, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          33, 0, 0, 0, 1, 8, 177, 135, 6, 0, 33, 0, 6, 8, 0, 0, 1, 0, 142, 35, 0, 12, 0, 0, 1, 0,
          35, 105, 4, 12, 0, 0, 1, 0, 41, 35,
        ]).buffer,
      ),
      description: '',
      recordID: 34735,
      recordLength: 64,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    34737: {
      data: new DataView(
        new Uint8Array([
          85, 84, 77, 32, 90, 111, 110, 101, 32, 49, 53, 44, 32, 78, 111, 114, 116, 104, 101, 114,
          110, 32, 72, 101, 109, 105, 115, 112, 104, 101, 114, 101, 124, 78, 65, 68, 56, 51, 124,
        ]).buffer,
      ),
      description: '',
      recordID: 34737,
      recordLength: 39,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTCitationGeoKey: 'UTM Zone 15, Northern Hemisphere',
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GeogAngularUnitsGeoKey: 9102,
    GeogCitationGeoKey: 'NAD83',
    ProjLinearUnitsGeoKey: 9001,
    ProjectedCSTypeGeoKey: 26915,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(1);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          edgeOfFlightLine: 0,
          intensity: 0,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngleRank: -13,
          userData: 0,
        },
        x: -93.35156259019989,
        y: 41.577148395419115,
        z: 16,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - 1.2_1', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_1.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'libLAS 1.2',
    headerSize: 227,
    majorVersion: 1,
    maxX: 470692.447538,
    maxY: 4602888.904642,
    maxZ: 16,
    minX: 470692.447538,
    minY: 4602888.904642,
    minZ: 16,
    minorVersion: 2,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 438,
    pointDataFormatID: 1,
    pointDataRecordLength: 28,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'libLAS',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    34735: {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 7, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          33, 0, 0, 0, 1, 8, 177, 135, 6, 0, 33, 0, 6, 8, 0, 0, 1, 0, 142, 35, 0, 12, 0, 0, 1, 0,
          35, 105, 4, 12, 0, 0, 1, 0, 41, 35,
        ]).buffer,
      ),
      description: '',
      recordID: 34735,
      recordLength: 64,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    34737: {
      data: new DataView(
        new Uint8Array([
          85, 84, 77, 32, 90, 111, 110, 101, 32, 49, 53, 44, 32, 78, 111, 114, 116, 104, 101, 114,
          110, 32, 72, 101, 109, 105, 115, 112, 104, 101, 114, 101, 124, 78, 65, 68, 56, 51, 124,
        ]).buffer,
      ),
      description: '',
      recordID: 34737,
      recordLength: 39,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTCitationGeoKey: 'UTM Zone 15, Northern Hemisphere',
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GeogAngularUnitsGeoKey: 9102,
    GeogCitationGeoKey: 'NAD83',
    ProjLinearUnitsGeoKey: 9001,
    ProjectedCSTypeGeoKey: 26915,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(1);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngleRank: -13,
          userData: 0,
        },
        x: -93.35156259019989,
        y: 41.577148395419115,
        z: 16,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - 1.2_2', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_2.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'libLAS 1.2',
    headerSize: 227,
    majorVersion: 1,
    maxX: 470692.447538,
    maxY: 4602888.904642,
    maxZ: 16,
    minX: 470692.447538,
    minY: 4602888.904642,
    minZ: 16,
    minorVersion: 2,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 438,
    pointDataFormatID: 2,
    pointDataRecordLength: 26,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'libLAS',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    34735: {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 7, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          33, 0, 0, 0, 1, 8, 177, 135, 6, 0, 33, 0, 6, 8, 0, 0, 1, 0, 142, 35, 0, 12, 0, 0, 1, 0,
          35, 105, 4, 12, 0, 0, 1, 0, 41, 35,
        ]).buffer,
      ),
      description: '',
      recordID: 34735,
      recordLength: 64,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    34737: {
      data: new DataView(
        new Uint8Array([
          85, 84, 77, 32, 90, 111, 110, 101, 32, 49, 53, 44, 32, 78, 111, 114, 116, 104, 101, 114,
          110, 32, 72, 101, 109, 105, 115, 112, 104, 101, 114, 101, 124, 78, 65, 68, 56, 51, 124,
        ]).buffer,
      ),
      description: '',
      recordID: 34737,
      recordLength: 39,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTCitationGeoKey: 'UTM Zone 15, Northern Hemisphere',
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GeogAngularUnitsGeoKey: 9102,
    GeogCitationGeoKey: 'NAD83',
    ProjLinearUnitsGeoKey: 9001,
    ProjectedCSTypeGeoKey: 26915,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(1);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          edgeOfFlightLine: 0,
          intensity: 0,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
          scanAngleRank: -13,
          userData: 0,
        },
        x: -93.35156259019989,
        y: 41.577148395419115,
        z: 16,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - 1.2_3', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_3.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'libLAS 1.2',
    headerSize: 227,
    majorVersion: 1,
    maxX: 470692.447538,
    maxY: 4602888.904642,
    maxZ: 16,
    minX: 470692.447538,
    minY: 4602888.904642,
    minZ: 16,
    minorVersion: 2,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 438,
    pointDataFormatID: 3,
    pointDataRecordLength: 34,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'libLAS',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    34735: {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 7, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          33, 0, 0, 0, 1, 8, 177, 135, 6, 0, 33, 0, 6, 8, 0, 0, 1, 0, 142, 35, 0, 12, 0, 0, 1, 0,
          35, 105, 4, 12, 0, 0, 1, 0, 41, 35,
        ]).buffer,
      ),
      description: '',
      recordID: 34735,
      recordLength: 64,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    34737: {
      data: new DataView(
        new Uint8Array([
          85, 84, 77, 32, 90, 111, 110, 101, 32, 49, 53, 44, 32, 78, 111, 114, 116, 104, 101, 114,
          110, 32, 72, 101, 109, 105, 115, 112, 104, 101, 114, 101, 124, 78, 65, 68, 56, 51, 124,
        ]).buffer,
      ),
      description: '',
      recordID: 34737,
      recordLength: 39,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTCitationGeoKey: 'UTM Zone 15, Northern Hemisphere',
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GeogAngularUnitsGeoKey: 9102,
    GeogCitationGeoKey: 'NAD83',
    ProjLinearUnitsGeoKey: 9001,
    ProjectedCSTypeGeoKey: 26915,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(1);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
          scanAngleRank: -13,
          userData: 0,
        },
        x: -93.35156259019989,
        y: 41.577148395419115,
        z: 16,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - 1.2 color', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2-with-color.las`);
  const lasReader = new LASReader(fileReader);
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 0,
    fileCreationYear: 0,
    generatingSoftware: 'TerraScan',
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
    numVariableLengthRecords: 0,
    offsetToPoints: 229,
    pointDataFormatID: 3,
    pointDataRecordLength: 34,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: '',
    waveformDataPacketOffset: 0,
    xOffset: -0,
    xScaleFactor: 0.01,
    yOffset: -0,
    yScaleFactor: 0.01,
    zOffset: -0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({});
  expect(lasReader.GeoKeyDirectory).toBeUndefined();
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(1065);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 1,
          classification: 'Unclassified',
          edgeOfFlightLine: 0,
          gpsTime: 245380.78254962614,
          intensity: 143,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 1,
          pointSourceID: 7326,
          returnNumber: 1,
          rgba: {
            a: 255,
            b: 88,
            g: 77,
            r: 68,
          },
          scanAngleRank: -9,
          userData: 132,
        },
        x: 637012.24,
        y: 849028.31,
        z: 431.66,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

test('LASReader - utm16', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/test_utm16.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26916 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 0,
    fileCreationYear: 0,
    generatingSoftware: 'TerraScan',
    headerSize: 227,
    majorVersion: 1,
    maxX: 809331.8965662403,
    maxY: 4324251.741352423,
    maxZ: 170.76,
    minX: 809327.6838636857,
    minY: 4324249.4775006445,
    minZ: 170.58,
    minorVersion: 2,
    numPoints: 10,
    numPointsByReturn: [0, 0, 0, 0, 0],
    numVariableLengthRecords: 3,
    offsetToPoints: 513,
    pointDataFormatID: 1,
    pointDataRecordLength: 28,
    projectID1: 0,
    projectID2: 0,
    projectID3: 0,
    projectID4: '',
    signature: 'LASF',
    sourceID: 0,
    systemIdentifier: 'MODIFIED',
    waveformDataPacketOffset: 0,
    xOffset: 0,
    xScaleFactor: 0.01,
    yOffset: 0,
    yScaleFactor: 0.01,
    zOffset: 0,
    zScaleFactor: 0.01,
  });
  expect(lasReader.variableLengthRecords).toEqual({
    '34735': {
      data: new DataView(
        new Uint8Array([
          1, 0, 1, 0, 0, 0, 8, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0, 2, 4, 177, 135,
          21, 0, 0, 0, 1, 8, 177, 135, 6, 0, 21, 0, 6, 8, 0, 0, 1, 0, 142, 35, 14, 8, 176, 135, 3,
          0, 0, 0, 0, 12, 0, 0, 1, 0, 36, 105, 4, 12, 0, 0, 1, 0, 41, 35,
        ]).buffer,
      ),
      description: 'GeoTiff GeoKeyDirectoryTag',
      recordID: 34735,
      recordLength: 72,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34736': {
      data: new DataView(
        new Uint8Array([
          0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoDoubleParamsTag',
      recordID: 34736,
      recordLength: 24,
      reserved: 0,
      userID: 'LASF_Projection',
    },
    '34737': {
      data: new DataView(
        new Uint8Array([
          78, 65, 68, 56, 51, 32, 47, 32, 85, 84, 77, 32, 122, 111, 110, 101, 32, 49, 54, 78, 124,
          78, 65, 68, 56, 51, 124, 0,
        ]).buffer,
      ),
      description: 'GeoTiff GeoAsciiParamsTag',
      recordID: 34737,
      recordLength: 28,
      reserved: 0,
      userID: 'LASF_Projection',
    },
  });
  expect(lasReader.GeoKeyDirectory).toEqual({
    GTModelTypeGeoKey: 1,
    GTRasterTypeGeoKey: 1,
    GTCitationGeoKey: 'NAD83 / UTM zone 16N',
    GeogCitationGeoKey: 'NAD83',
    GeogAngularUnitsGeoKey: 9102,
    GeogTOWGS84GeoKey: [0, 0, 0],
    ProjectedCSTypeGeoKey: 26916,
    ProjLinearUnitsGeoKey: 9001,
  });
  const data = await Array.fromAsync(lasReader);
  expect(data.length).toEqual(10);
  const firstPoint = data[0];
  expect(firstPoint).toEqual({
    geometry: {
      coordinates: {
        m: {
          scanDirectionFlag: 0,
          classification: 'Ground',
          edgeOfFlightLine: 0,
          gpsTime: 499450.8059940542,
          intensity: 260,
          isKeyPoint: false,
          isSynthetic: false,
          isWithheld: false,
          numberOfReturns: 6,
          pointSourceID: 0,
          returnNumber: 0,
          scanAngleRank: 0,
          userData: 0,
        },
        x: -83.4275978058665,
        y: 39.01259906906862,
        z: 170.76,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {},
    type: 'VectorFeature',
  });
});

// test('LASReader - 1.2 color ZIPPED', async () => {
//   const fileReader = new FileReader(`${__dirname}/fixtures/1.2-with-color.laz`);
//   const lasReader = new LASReader(fileReader);
//   expect(lasReader.header).toEqual({
//     encoding: 0,
//     extendedVariableLengthRecordOffset: 0,
//     extendedVariableLengthSize: 0,
//     fileCreationDay: 0,
//     fileCreationYear: 0,
//     generatingSoftware: 'TerraScan',
//     headerSize: 227,
//     majorVersion: 1,
//     maxX: 638982.55,
//     maxY: 853535.43,
//     maxZ: 586.38,
//     minX: 635619.85,
//     minY: 848899.7000000001,
//     minZ: 406.59000000000003,
//     minorVersion: 2,
//     numPoints: 1065,
//     numPointsByReturn: [925, 114, 21, 5, 0],
//     numVariableLengthRecords: 1,
//     offsetToPoints: 335,
//     pointDataFormatID: 131,
//     pointDataRecordLength: 34,
//     projectID1: 0,
//     projectID2: 0,
//     projectID3: 0,
//     projectID4: '',
//     signature: 'LASF',
//     sourceID: 0,
//     systemIdentifier: '',
//     waveformDataPacketOffset: 0,
//     xOffset: -0,
//     xScaleFactor: 0.01,
//     yOffset: -0,
//     yScaleFactor: 0.01,
//     zOffset: -0,
//     zScaleFactor: 0.01,
//   });
//   expect(lasReader.variableLengthRecords).toEqual({
//     '22204': {
//       data: new DataView(
//         new Uint8Array([
//           2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
//           255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 6, 0, 20, 0, 2, 0, 7, 0, 8, 0, 2, 0, 8, 0,
//           6, 0, 2, 0,
//         ]).buffer,
//       ),
//       description: '(c) LAStools by Martin Isenburg',
//       recordID: 22204,
//       recordLength: 52,
//       reserved: 43707,
//       userID: 'laszip encoded',
//     },
//   });
//   expect(lasReader.GeoKeyDirectory).toBeUndefined();

//   expect(lasReader.laz?.header).toEqual({
//     chunkSize: 50000,
//     coder: 0,
//     compressor: 2,
//     items: [
//       { size: 20, type: 6, version: 2 },
//       { size: 8, type: 7, version: 2 },
//       { size: 6, type: 8, version: 2 },
//     ],
//     numItems: 3,
//     numSpecialEvlrs: -1,
//     offsetSpecialEvlrs: -1,
//     options: 0,
//     versionMajor: 2,
//     versionMinor: 0,
//     versionRevision: 0,
//   });

//   const data = await Array.fromAsync(lasReader);
//   expect(data.length).toEqual(1065);
//   const firstPoint = data[0];
//   expect(firstPoint).toEqual({
//     geometry: {
//       coordinates: {
//         m: {
//           scanDirectionFlag: 0,
//           classification: 'Unclassified',
//           edgeOfFlightLine: 1,
//           gpsTime: 245380.78254962614,
//           intensity: 143,
//           isKeyPoint: false,
//           isSynthetic: false,
//           isWithheld: false,
//           numberOfReturns: 1,
//           pointSourceID: 7326,
//           returnNumber: 1,
//           rgba: {
//             a: 255,
//             b: 88,
//             g: 77,
//             r: 68,
//           },
//           scanAngleRank: 247,
//           userData: 132,
//         },
//         x: 637012.24,
//         y: 849028.31,
//         z: 431.66,
//       },
//       is3D: true,
//       type: 'Point',
//     },
//     properties: {},
//     type: 'VectorFeature',
//   });
// });

// test('LASReader - zipped', () => {
//   const fileReader = new FileReader(`${__dirname}/fixtures/5points_14.las`);
//   const lasReader = new LASReader(fileReader);
//   expect(lasReader.header).toEqual({
//     encoding: 17,
//     extendedVariableLengthRecordOffset: 0,
//     extendedVariableLengthSize: 0,
//     fileCreationDay: 30,
//     fileCreationYear: 2015,
//     generatingSoftware: 'LASzip DLL 2.4 r1 (150905)',
//     headerSize: 375,
//     majorVersion: 1,
//     maxX: 630499.95,
//     maxY: 4834749.66,
//     maxZ: 63.68,
//     minX: 630498.56,
//     minY: 4834748.73,
//     minZ: 61.33,
//     minorVersion: 4,
//     numPoints: 5,
//     numPointsByReturn: [
//       4294967296, 8589934592, 0, 0, 0, 0, 0, 4294967296, 4294967296, 0, 0, 0, 0, 0, 0,
//     ],
//     numVariableLengthRecords: 3,
//     offsetToPoints: 921,
//     pointDataFormatID: 6,
//     pointDataRecordLength: 33,
//     projectID1: 0,
//     projectID2: 0,
//     projectID3: 0,
//     projectID4: '',
//     signature: 'LASF',
//     sourceID: 4711,
//     systemIdentifier: 'LASzip DLL example 9',
//     waveformDataPacketOffset: 0,
//     xOffset: 600000,
//     xScaleFactor: 0.01,
//     yOffset: 4800000,
//     yScaleFactor: 0.01,
//     zOffset: 0,
//     zScaleFactor: 0.01,
//   });
//   expect(lasReader.variableLengthRecords).toEqual({
//     4: {
//       data: new DataView(
//         new Uint8Array([
//           0, 0, 4, 24, 104, 101, 105, 103, 104, 116, 32, 97, 98, 111, 118, 101, 32, 103, 114, 111,
//           117, 110, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 154, 153, 153, 153, 153, 153, 169, 63, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0,
//           0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           113, 117, 97, 110, 116, 105, 122, 101, 100, 32, 116, 111, 32, 53, 32, 99, 109, 32, 97, 98,
//           111, 118, 101, 32, 49, 39, 32, 83, 82, 84, 77, 0, 0, 0, 1, 24, 99, 111, 118, 101, 114, 97,
//           103, 101, 32, 99, 111, 117, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0,
//           0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//           0, 0, 0, 0, 0, 0, 0, 0, 98, 121, 32, 104, 105, 103, 104, 101, 114, 32, 114, 101, 116, 117,
//           114, 110, 115, 39, 32, 48, 46, 53, 32, 109, 32, 114, 97, 100, 105, 117, 115, 0,
//         ]).buffer,
//       ),
//       description: 'LASzip DLL 2.4 r1 (150905)',
//       recordID: 4,
//       recordLength: 384,
//       reserved: 43707,
//       userID: 'LASF_Spec',
//     },
//     2112: {
//       data: undefined,
//       description: 'intentionally empty OGC WKT',
//       recordID: 2112,
//       recordLength: 0,
//       reserved: 43707,
//       userID: 'LASF_Projection',
//     },
//     12345: {
//       data: undefined,
//       description: 'just a funny VLR',
//       recordID: 12345,
//       recordLength: 0,
//       reserved: 43707,
//       userID: 'funny',
//     },
//   });
//   expect(lasReader.GeoKeyDirectory).toBeUndefined();
//   // const data = await Array.fromAsync(lasReader);
//   // expect(data.length).toEqual(5);
// });
