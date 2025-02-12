import { FileReader } from '../../../src/file';
import {
  EPSG_26915,
  EPSG_26916,
  EPSG_32617,
  LASReader,
  UniversalTransverseMercator,
  toLASClassification,
  toLASClassification14,
  toLASClassificationFlag,
} from '../../../src';
import { expect, test } from 'bun:test';

test('toLASClassificationFlag', () => {
  expect(toLASClassificationFlag(0)).toEqual('Synthetic');
  expect(toLASClassificationFlag(1)).toEqual('Key-point');
  expect(toLASClassificationFlag(2)).toEqual('Withheld');
  expect(toLASClassificationFlag(3)).toEqual('Overlap');
  expect(toLASClassificationFlag(4)).toEqual('Unknown');
});

test('toLASClassification14', () => {
  expect(toLASClassification14(0)).toEqual('Created, Never Classified');
  expect(toLASClassification14(1)).toEqual('Unclassified');
  expect(toLASClassification14(2)).toEqual('Ground');
  expect(toLASClassification14(3)).toEqual('Low Vegetation');
  expect(toLASClassification14(4)).toEqual('Medium Vegetation');
  expect(toLASClassification14(5)).toEqual('High Vegetation');
  expect(toLASClassification14(6)).toEqual('Building');
  expect(toLASClassification14(7)).toEqual('Low Point (Noise)');
  expect(toLASClassification14(8)).toEqual('Reserved');
  expect(toLASClassification14(9)).toEqual('Water');
  expect(toLASClassification14(10)).toEqual('Rail');
  expect(toLASClassification14(11)).toEqual('Road Surface');
  expect(toLASClassification14(12)).toEqual('Reserved');
  expect(toLASClassification14(13)).toEqual('Wire – Guard (Shield)');
  expect(toLASClassification14(14)).toEqual('Wire – Conductor (Phase)');
  expect(toLASClassification14(15)).toEqual('Transmission Tower');
  expect(toLASClassification14(16)).toEqual('Wire-structure Connector (e.g. Insulator)');
  expect(toLASClassification14(17)).toEqual('Bridge Deck');
  expect(toLASClassification14(18)).toEqual('High Noise');
  expect(toLASClassification14(20)).toEqual('Reserved');
  expect(toLASClassification14(66)).toEqual('User Definable');
  expect(toLASClassification14(5000)).toEqual('Missing');
});

test('toLASClassification', () => {
  expect(toLASClassification(0)).toEqual('Created, Never Classified');
  expect(toLASClassification(1)).toEqual('Unclassified');
  expect(toLASClassification(2)).toEqual('Ground');
  expect(toLASClassification(3)).toEqual('Low Vegetation');
  expect(toLASClassification(4)).toEqual('Medium Vegetation');
  expect(toLASClassification(5)).toEqual('High Vegetation');
  expect(toLASClassification(6)).toEqual('Building');
  expect(toLASClassification(7)).toEqual('Low Point (Noise)');
  expect(toLASClassification(8)).toEqual('Model Key-point (mass point)');
  expect(toLASClassification(9)).toEqual('Water');
  expect(toLASClassification(12)).toEqual('Overlap Points');
  expect(toLASClassification(13)).toEqual('Reserved');
});

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

test('LASReader - 1.2_4', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_4.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
    headerSize: 235,
    majorVersion: 1,
    maxX: 470692.44,
    maxY: 4602888.9,
    maxZ: 16,
    minX: 470692.44,
    minY: 4602888.9,
    minZ: 16,
    minorVersion: 3,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 446,
    pointDataFormatID: 4,
    pointDataRecordLength: 57,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          wavePacketDescriptorIndex: 0,
          wavePacketLength: 0,
          wavePacketOffset: 0,
          waveformLocationReturnPoint: 0,
          xT: 0,
          yT: 0,
          zT: 0,
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

test('LASReader - 1.2_5', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_5.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
    headerSize: 235,
    majorVersion: 1,
    maxX: 470692.44,
    maxY: 4602888.9,
    maxZ: 16,
    minX: 470692.44,
    minY: 4602888.9,
    minZ: 16,
    minorVersion: 3,
    numPoints: 1,
    numPointsByReturn: [0, 1, 0, 0, 0],
    numVariableLengthRecords: 2,
    offsetToPoints: 446,
    pointDataFormatID: 5,
    pointDataRecordLength: 63,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
          wavePacketDescriptorIndex: 0,
          wavePacketLength: 0,
          wavePacketOffset: 0,
          waveformLocationReturnPoint: 0,
          xT: 0,
          yT: 0,
          zT: 0,
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

test('LASReader - 1.2_6', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_6.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
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
    numVariableLengthRecords: 2,
    offsetToPoints: 586,
    pointDataFormatID: 6,
    pointDataRecordLength: 30,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          scanDirectionFlag: 0,
          scannerChannel: 0,
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

test('LASReader - 1.2_7', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_7.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
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
    numVariableLengthRecords: 2,
    offsetToPoints: 586,
    pointDataFormatID: 7,
    pointDataRecordLength: 36,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          scanDirectionFlag: 0,
          scannerChannel: 0,
          userData: 0,
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
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

test('LASReader - 1.2_8', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_8.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
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
    numVariableLengthRecords: 2,
    offsetToPoints: 586,
    pointDataFormatID: 8,
    pointDataRecordLength: 38,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          scanDirectionFlag: 0,
          scannerChannel: 0,
          userData: 0,
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
          nir: 0,
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

test('LASReader - 1.2_9', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_9.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
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
    numVariableLengthRecords: 2,
    offsetToPoints: 586,
    pointDataFormatID: 9,
    pointDataRecordLength: 59,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          scanDirectionFlag: 0,
          scannerChannel: 0,
          userData: 0,
          wavePacketDescriptorIndex: 0,
          wavePacketLength: 0,
          wavePacketOffset: 0,
          waveformLocationReturnPoint: 0,
          xT: 0,
          yT: 0,
          zT: 0,
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

test('LASReader - 1.2_10', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/1.2_10.las`);
  const lasReader = new LASReader(fileReader, [UniversalTransverseMercator], { EPSG_26915 });
  expect(lasReader.header).toEqual({
    encoding: 0,
    extendedVariableLengthRecordOffset: 0,
    extendedVariableLengthSize: 0,
    fileCreationDay: 78,
    fileCreationYear: 2008,
    generatingSoftware: 'las2las64 (version 250207)',
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
    numVariableLengthRecords: 2,
    offsetToPoints: 586,
    pointDataFormatID: 10,
    pointDataRecordLength: 67,
    projectID1: 2206790072,
    projectID2: 43547,
    projectID3: 16648,
    projectID4: '��kƎ{\u0006.',
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
          classification: 'Ground',
          classificationFlag: 'Synthetic',
          edgeOfFlightLine: 0,
          gpsTime: 1205902800,
          intensity: 0,
          numberOfReturns: 0,
          pointSourceID: 0,
          returnNumber: 2,
          scanAngle: -2167,
          scanDirectionFlag: 0,
          scannerChannel: 0,
          userData: 0,
          wavePacketDescriptorIndex: 0,
          wavePacketLength: 0,
          wavePacketOffset: 0,
          waveformLocationReturnPoint: 0,
          xT: 0,
          yT: 0,
          zT: 0,
          rgba: {
            a: 255,
            b: 234,
            g: 12,
            r: 255,
          },
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
