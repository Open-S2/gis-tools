import { LASzip } from './laz';
import {
  buildTransformFromGeoKeys as buildTransform,
  parseGeotiffRawGeoKeys,
  toReader,
} from '../..';

import type {
  FeatureIterator,
  GeoKeyDirectory,
  GridReader,
  ProjectionTransformDefinition,
  Properties,
  Reader,
  ReaderInputs,
  Transformer,
  VectorFeature,
  VectorPoint,
} from '../..';
import type {
  LASFormat,
  LASFormat0,
  LASFormat1,
  LASFormat10,
  LASFormat2,
  LASFormat3,
  LASFormat4,
  LASFormat5,
  LASFormat6,
  LASFormat7,
  LASFormat8,
  LASFormat9,
  LASHeader,
} from './types';

/**
 * VARIABLE LENGTH RECORDS:
 * The Variable Length Records are used to add custom data to the Public Header Block.
 * The `GeoKeyDirectoryTag` is required
 */
export interface LASVariableLengthRecord {
  /** Reserved unsigned short 2 bytes */
  reserved: number;
  /** User ID char[16] 16 bytes */
  userID: string;
  /** Record ID unsigned short 2 bytes */
  recordID: number;
  /** Record Length After Header unsigned short 2 bytes */
  recordLength: number;
  /** Description char[32] 32 bytes */
  description: string;
  /** The data of the record */
  data?: DataView;
}

// TODO: It's vastly more performant to completely build the M-Values individually.
// TODO: Don't pull from parent object. Ex: `#getPointFormat1` pulls from `#getPointFormat0`

/**
 * # LAS/LAZ Reader
 *
 * ## Description
 * Reads LAS data including LASzip data. Supports LAS 1.4 specification although missing some support.
 * [See specification](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf)
 * Implements the {@link FeatureIterator} interface
 *
 * Data is stored like so: (EOF is a LAZ optional field and not used by LAS)
 *```
 * |            PUBLIC HEADER BLOCK           |
 * |          VARIABLE LENGTH RECORDS         |
 * |             POINT DATA RECORDS           |
 * | Extended Variable Length Records (EVLRs) |
 * |  Field Chunk table start position (EOF)  |
 * ```
 *
 * ## Links
 * https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
 * https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
 * https://liblas.org/development/index.html
 * https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
 * https://github.com/PDAL/PDAL
 * https://github.com/libLAS/libLAS (deprecated for PDAL)
 * https://github.com/LASzip
 */
export class LASReader implements FeatureIterator<undefined, LASFormat, Properties> {
  private reader: Reader;
  readonly header: LASHeader;
  readonly variableLengthRecords: Record<number, LASVariableLengthRecord> = {};
  readonly GeoKeyDirectory?: GeoKeyDirectory;
  #transformer: Transformer;
  #decoder = new TextDecoder();
  readonly laz?: LASzip;
  /**
   * @param input - The LAS or LAZ input data from a reader/buffer
   * @param definitions - an array of projection definitions for the transformer if needed
   * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
   * @param gridStores - an array of grid readers if needed
   */
  constructor(
    input: ReaderInputs,
    definitions: ProjectionTransformDefinition[] = [],
    epsgCodes: Record<string, string> = {},
    gridStores?: GridReader[],
  ) {
    this.reader = toReader(input);
    this.header = this.#parseHeader();
    this.#parseVariableLengthRecords();
    this.laz = this.#buildLaz();
    this.GeoKeyDirectory = this.#buildGeokeyDirectory();
    this.#transformer = buildTransform(this.GeoKeyDirectory, gridStores, definitions, epsgCodes);
  }

  /**
   * Generator to iterate over each WGS84 lon-lat point
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<undefined, LASFormat, Properties>> {
    const { numPoints } = this.header;
    if (numPoints === 0) return;
    for (let i = 0; i < numPoints; i++) {
      const coordinates = this.#getPoint(i);
      yield {
        type: 'VectorFeature',
        geometry: {
          type: 'Point',
          coordinates,
          is3D: true,
        },
        properties: {},
      };
    }
  }

  /**
   * Reads the Public Header Block
   * @returns - The public header
   */
  #parseHeader(): LASHeader {
    const { reader } = this;
    const header: LASHeader = {
      signature: reader.parseString(0, 4),
      sourceID: reader.getUint16(4, true),
      encoding: reader.getUint16(6, true),
      projectID1: reader.getUint32(8, true),
      projectID2: reader.getUint16(12, true),
      projectID3: reader.getUint16(14, true),
      projectID4: reader.parseString(16, 8),
      majorVersion: reader.getUint8(24),
      minorVersion: reader.getUint8(25),
      systemIdentifier: reader.parseString(26, 32),
      generatingSoftware: reader.parseString(58, 32),
      fileCreationDay: reader.getUint16(90, true),
      fileCreationYear: reader.getUint16(92, true),
      headerSize: reader.getUint16(94, true),
      offsetToPoints: reader.getUint32(96, true),
      numVariableLengthRecords: reader.getUint32(100, true),
      pointDataFormatID: reader.getUint8(104),
      pointDataRecordLength: reader.getUint16(105, true),
      numPoints: reader.getUint32(107, true),
      numPointsByReturn: [
        reader.getUint32(111, true),
        reader.getUint32(115, true),
        reader.getUint32(119, true),
        reader.getUint32(123, true),
        reader.getUint32(127, true),
      ],
      xScaleFactor: reader.getFloat64(131, true),
      yScaleFactor: reader.getFloat64(139, true),
      zScaleFactor: reader.getFloat64(147, true),
      xOffset: reader.getFloat64(155, true),
      yOffset: reader.getFloat64(163, true),
      zOffset: reader.getFloat64(171, true),
      maxX: reader.getFloat64(179, true),
      minX: reader.getFloat64(187, true),
      maxY: reader.getFloat64(195, true),
      minY: reader.getFloat64(203, true),
      maxZ: reader.getFloat64(211, true),
      minZ: reader.getFloat64(219, true),
      waveformDataPacketOffset: 0,
      extendedVariableLengthRecordOffset: 0,
      extendedVariableLengthSize: 0,
    };
    // 1.4 or later supports wave packets
    if (header.headerSize > 227) {
      header.waveformDataPacketOffset = Number(reader.getBigUint64(227, true));
      header.extendedVariableLengthRecordOffset = reader.getUint32(235, true);
      header.extendedVariableLengthSize = Number(reader.getBigUint64(239, true));
      // re-adjust numPoints and numPointsByReturn
      header.numPoints = reader.getUint32(247, true);
      // 15 times
      let curOffset = 251;
      header.numPointsByReturn = [];
      for (let i = 0; i < 15; i++) {
        header.numPointsByReturn.push(Number(reader.getBigUint64(curOffset, true)));
        curOffset += 8;
      }
    }

    return header;
  }

  /**
   * The Public Header Block is followed by one or more Variable Length Records (There is one
   * mandatory Variable Length Record, GeoKeyDirectoryTag). The number of Variable Length
   * Records is specified in the "Number of Variable Length Records" field in the Public Header Block.
   * The Variable Length Records must be accessed sequentially since the size of each variable length
   * record is contained in the Variable Length Record Header. Each Variable Length Record Header
   * is 54 bytes in length.
   *
   * Each record is as follows:
   * - Reserved unsigned short 2 bytes
   * - User ID char[16] 16 bytes *
   * - Record ID unsigned short 2 bytes *
   * - Record Length After Header unsigned short 2 bytes *
   * - Description char[32] 32 bytes
   */
  #parseVariableLengthRecords(): void {
    const { reader, header } = this;
    const { headerSize, numVariableLengthRecords } = header;
    let position = headerSize;
    for (let i = 0; i < numVariableLengthRecords; ++i) {
      const recordLength = reader.getUint16(position + 20, true);
      const record: LASVariableLengthRecord = {
        reserved: reader.getUint16(position, true),
        userID: reader.parseString(position + 2, 16),
        recordID: reader.getUint16(position + 18, true),
        recordLength,
        description: reader.parseString(position + 22, 32),
        data:
          recordLength > 0 ? reader.slice(position + 54, position + 54 + recordLength) : undefined,
      };
      position += 54 + recordLength;
      this.variableLengthRecords[record.recordID] = record;
    }
  }

  /**
   * If the LAZ variable length record is present, build a LAZ parser
   * @returns - The LAZ parser
   */
  #buildLaz(): LASzip | undefined {
    const lazData = this.variableLengthRecords['22204']?.data;
    if (lazData === undefined && this.header.pointDataFormatID > 127)
      throw Error('LAZ data, but LAZ record not found.');
    if (lazData === undefined) return;
    return new LASzip(lazData);
  }

  /**
   * userID of "LASF_Projection" will contain at least 3 records:
   * - GeoKeyDirectoryTag (34735)
   * - GeoDoubleParamsTag (34736)
   * - GeoASCIIParamsTag (34737)
   *
   * Only the `GeoKeyDirectoryTag` record is required. This parses the `GeoKeyDirectoryTag`.
   * This record contains the key values that define the coordinate system. A complete description
   * can be found in the GeoTIFF format specification. Here is a summary from a programmatic point
   * of view for someone interested in implementation.
   *
   * The `GeoKeyDirectoryTag` is defined as just an array of unsigned short values. But,
   * programmatically, the data can be seen as something like this:
   * @returns - The parsed GeoKeyDirectory
   */
  #buildGeokeyDirectory(): GeoKeyDirectory | undefined {
    const { variableLengthRecords } = this;
    // GeoKeyDirectoryTag
    const geokeyRecord = variableLengthRecords[34735]?.data;
    if (geokeyRecord === undefined) {
      console.warn('GeoKeyDirectoryTag not found. May fail to parse correctly.');
      return;
    }
    const rawGeoKeys = new Uint16Array(geokeyRecord.buffer, geokeyRecord.byteOffset);
    // GeoDoubleParamsTag
    const doubleRecord = variableLengthRecords[34736]?.data;
    const GeoDoubleParams =
      doubleRecord !== undefined
        ? [...new Float64Array(doubleRecord.buffer, doubleRecord.byteOffset)]
        : undefined;
    // GeoAsciiParamsTag
    const asciiRecord = variableLengthRecords[34737]?.data;
    const GeoAsciiParams =
      asciiRecord !== undefined ? this.#decoder.decode(asciiRecord) : undefined;
    return parseGeotiffRawGeoKeys(rawGeoKeys, {
      GeoKeyDirectory: this.GeoKeyDirectory,
      GeoDoubleParams,
      GeoAsciiParams,
    });
  }

  /**
   * Reads a point in
   * @param index - The index of the point to read
   * @returns - The parsed point
   */
  #getPoint(index: number): VectorPoint<LASFormat> {
    const { offsetToPoints, pointDataFormatID } = this.header;
    const offset = offsetToPoints + index * this.header.pointDataRecordLength;
    const format = pointDataFormatID > 127 ? pointDataFormatID - 128 : pointDataFormatID;
    if (format === 0) return this.#getPointFormat0(offset);
    else if (format === 1) return this.#getPointFormat1(offset);
    else if (format === 2) return this.#getPointFormat2(offset);
    else if (format === 3) return this.#getPointFormat3(offset);
    else if (format === 4) return this.#getPointFormat4(offset);
    else if (format === 5) return this.#getPointFormat5(offset);
    else if (format === 6) return this.#getPointFormat6(offset);
    else if (format === 7) return this.#getPointFormat7(offset);
    else if (format === 8) return this.#getPointFormat8(offset);
    else if (format === 9) return this.#getPointFormat9(offset);
    else if (format === 10) return this.#getPointFormat10(offset);
    else throw new Error(`Unknown Point Data Format ID: ${format}`);
  }

  /**
   * Reads a point using the Point Data Record Format 0
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 0 metadata
   */
  #getPointFormat0(offset: number): VectorPoint<LASFormat0> {
    const { reader } = this;
    const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = this.header;
    const bits = reader.getUint32(offset + 14, true);
    const classBits = reader.getUint8(offset + 15);
    const point: VectorPoint<LASFormat0> = {
      x: reader.getUint32(offset, true) * xScaleFactor + xOffset,
      y: reader.getUint32(offset + 4, true) * yScaleFactor + yOffset,
      z: reader.getUint32(offset + 8, true) * zScaleFactor + zOffset,
      m: {
        intensity: reader.getUint16(offset + 12, true),
        returnNumber: bits & 0b00000111, // 3 bits (bits 0 – 2)
        numberOfReturns: (bits & 0b00011000) >> 3, // 3 bits (bits 3 – 5)
        ScanDirectionFlag: (bits & 0b00100000) >> 5, // 1 bit (bit 6)
        edgeOfFlightLine: (bits & 0b01000000) >> 6, // 1 bit (bit 7)
        classification: toLASClassification(classBits),
        isSynthetic: (classBits & (1 << 5)) !== 0,
        isKeyPoint: (classBits & (1 << 6)) !== 0,
        isWithheld: (classBits & (1 << 7)) !== 0,
        scanAngleRank: reader.getUint8(offset + 16),
        userData: reader.getUint8(offset + 17),
        pointSourceID: reader.getUint16(offset + 18, true),
      },
    };
    return this.#transformer.forward(point);
  }

  /**
   * Reads a point using the Point Data Record Format 1
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 1 metadata
   */
  #getPointFormat1(offset: number): VectorPoint<LASFormat1> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat0(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat0),
        gpsTime: reader.getFloat64(offset + 20, true),
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 2
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 2 metadata
   */
  #getPointFormat2(offset: number): VectorPoint<LASFormat2> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat0(offset);
    const r = reader.getUint16(offset + 20, true);
    const g = reader.getUint16(offset + 22, true);
    const b = reader.getUint16(offset + 24, true);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat0),
        rgba: { r, g, b, a: 255 },
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 3
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 3 metadata
   */
  #getPointFormat3(offset: number): VectorPoint<LASFormat3> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat0(offset);
    const gpsTime = reader.getFloat64(offset + 20, true);
    const r = reader.getUint16(offset + 28, true);
    const g = reader.getUint16(offset + 30, true);
    const b = reader.getUint16(offset + 32, true);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat0),
        gpsTime,
        rgba: { r, g, b, a: 255 },
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 4
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 4 metadata
   */
  #getPointFormat4(offset: number): VectorPoint<LASFormat4> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat1(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat1),
        wavePacketDescriptorIndex: reader.getUint8(offset + 28),
        wavePacketOffset: reader.getFloat64(offset + 29, true),
        wavePacketLength: reader.getUint32(offset + 37, true),
        waveformLocationReturnPoint: reader.getFloat32(offset + 41),
        xT: reader.getFloat32(offset + 45),
        yT: reader.getFloat32(offset + 49),
        zT: reader.getFloat32(offset + 53),
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 5
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 4 metadata
   */
  #getPointFormat5(offset: number): VectorPoint<LASFormat5> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat3(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat3),
        wavePacketDescriptorIndex: reader.getUint8(offset + 28),
        wavePacketOffset: reader.getFloat64(offset + 29, true),
        wavePacketLength: reader.getUint32(offset + 37, true),
        waveformLocationReturnPoint: reader.getFloat32(offset + 41),
        xT: reader.getFloat32(offset + 45),
        yT: reader.getFloat32(offset + 49),
        zT: reader.getFloat32(offset + 53),
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 0
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 0 metadata
   */
  #getPointFormat6(offset: number): VectorPoint<LASFormat6> {
    const { reader } = this;
    const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = this.header;
    const bits1 = reader.getUint8(offset + 14);
    const bits2 = reader.getUint8(offset + 15);
    const point: VectorPoint<LASFormat6> = {
      x: reader.getUint32(offset, true) * xScaleFactor + xOffset,
      y: reader.getUint32(offset + 4, true) * yScaleFactor + yOffset,
      z: reader.getUint32(offset + 8, true) * zScaleFactor + zOffset,
      m: {
        intensity: reader.getUint16(offset + 12, true),
        returnNumber: bits1 & 0b00001111, // 4 bits (bits 0 – 3)
        numberOfReturns: (bits1 & 0b11110000) >> 4, // 4 bits (bits 4 – 7)
        classificationFlag: toLASClassificationFlag(bits2), // 4 bis (bit 0 - 3)
        scannerChannel: (bits2 & 0b00110000) >> 4, // 2 bits (bit 4 - 5)
        scanDirectionFlag: (bits2 & 0b01000000) >> 6, // 1 bit (bit 6)
        edgeOfFlightLine: (bits2 & 0b10000000) >> 7, // 1 bit (bit 7)
        classification: toLASClassification2(reader.getUint8(offset + 16)),
        userData: reader.getUint8(offset + 17),
        scanAngle: reader.getUint16(offset + 18, true),
        pointSourceID: reader.getUint16(offset + 20, true),
        gpsTime: reader.getFloat64(offset + 22, true),
      },
    };
    return this.#transformer.forward(point);
  }

  /**
   * Reads a point using the Point Data Record Format 7
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 7 metadata
   */
  #getPointFormat7(offset: number): VectorPoint<LASFormat7> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat6(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat6),
        rgba: {
          r: reader.getUint16(offset + 30, true),
          g: reader.getUint16(offset + 32, true),
          b: reader.getUint16(offset + 34, true),
          a: reader.getUint16(offset + 36, true),
        },
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 8
   * @param offset  - where to start reading in the point data
   * @returns - The parsed point with Format 8 metadata
   */
  #getPointFormat8(offset: number): VectorPoint<LASFormat8> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat7(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat7),
        nir: reader.getUint16(offset + 38, true),
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 9
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 9 metadata
   */
  #getPointFormat9(offset: number): VectorPoint<LASFormat9> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat6(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat8),
        wavePacketDescriptorIndex: reader.getUint8(offset + 30),
        wavePacketOffset: reader.getFloat64(offset + 31, true),
        wavePacketLength: reader.getUint32(offset + 39, true),
        waveformLocationReturnPoint: reader.getFloat32(offset + 43),
        xT: reader.getFloat32(offset + 47),
        yT: reader.getFloat32(offset + 51),
        zT: reader.getFloat32(offset + 55),
      },
    };
  }

  /**
   * Reads a point using the Point Data Record Format 10
   * @param offset - where to start reading in the point data
   * @returns - The parsed point with Format 10 metadata
   */
  #getPointFormat10(offset: number): VectorPoint<LASFormat10> {
    const { reader } = this;
    const { x, y, z, m } = this.#getPointFormat7(offset);
    return {
      x,
      y,
      z,
      m: {
        ...(m as LASFormat7),
        wavePacketDescriptorIndex: reader.getUint8(offset + 38),
        wavePacketOffset: reader.getFloat64(offset + 39, true),
        wavePacketLength: reader.getUint32(offset + 47, true),
        waveformLocationReturnPoint: reader.getFloat32(offset + 51),
        xT: reader.getFloat32(offset + 55),
        yT: reader.getFloat32(offset + 59),
        zT: reader.getFloat32(offset + 63),
      },
    };
  }
}

/**
 * Converts a number into a LASClassification
 * @param classification - the number
 * @returns - the LASClassification
 */
function toLASClassification(classification: number): string {
  // we only wany the first 5 bits
  classification &= 0b11111;
  if (classification === 0) return 'Created, Never Classified';
  if (classification === 1) return 'Unclassified';
  if (classification === 2) return 'Ground';
  if (classification === 3) return 'Low Vegetation';
  if (classification === 4) return 'Medium Vegetation';
  if (classification === 5) return 'High Vegetation';
  if (classification === 6) return 'Building';
  if (classification === 7) return 'Low Point (Noise)';
  if (classification === 8) return 'Model Key-point (mass point)';
  if (classification === 9) return 'Water';
  if (classification === 12) return 'Overlap Points';
  return 'Reserved';
}

/**
 * Converts a number into a classification flag
 * @param classFlag - the input
 * @returns - the classification flag
 */
function toLASClassificationFlag(classFlag: number): string {
  const firstThreeBits = classFlag & 0b1111;
  if (firstThreeBits === 0) return 'Synthetic';
  if (firstThreeBits === 1) return 'Key-point';
  if (firstThreeBits === 2) return 'Withheld';
  if (firstThreeBits === 3) return 'Overlap';
  return 'Unknown';
}

/**
 * Converts a number into a LASClassification
 * @param classification - the number
 * @returns - the LASClassification
 */
export function toLASClassification2(classification: number): string {
  if (classification === 0) return 'Created, Never Classified';
  if (classification === 1) return 'Unclassified';
  if (classification === 2) return 'Ground';
  if (classification === 3) return 'Low Vegetation';
  if (classification === 4) return 'Medium Vegetation';
  if (classification === 5) return 'High Vegetation';
  if (classification === 6) return 'Building';
  if (classification === 7) return 'Low Point (Noise)';
  if (classification === 8) return 'Reserved';
  if (classification === 9) return 'Water';
  if (classification === 10) return 'Rail';
  if (classification === 11) return 'Road Surface';
  if (classification === 12) return 'Reserved';
  if (classification === 13) return 'Wire – Guard (Shield)';
  if (classification === 14) return 'Wire – Conductor (Phase)';
  if (classification === 15) return 'Transmission Tower';
  if (classification === 16) return 'Wire-structure Connector (e.g. Insulator)';
  if (classification === 17) return 'Bridge Deck';
  if (classification === 18) return 'High Noise';
  if (classification >= 19 && classification <= 63) return 'Reserved';
  if (classification >= 64 && classification <= 255) return 'User Definable';
  return 'Missing';
}
