import { LASzip } from './laz';
import {
  buildTransformFromGeoKeys as buildTransform,
  parseGeotiffRawGeoKeys,
  toReader,
} from '../..';
import {
  getPointFormat0,
  getPointFormat1,
  getPointFormat2,
  getPointFormat3,
  getPointFormat6,
  getPointFormat7,
  getPointFormat8,
} from './getPoint';

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
import type { LASFormat, LASHeader, LASVariableLengthRecord } from './types';

/** Cleaned up type for getPoint */
type GetPointParams = [Reader, LASHeader, Transformer, number];

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
    const { reader, header } = this;
    const { offsetToPoints, pointDataFormatID, pointDataRecordLength } = header;
    const offset = offsetToPoints + index * pointDataRecordLength;
    const format = pointDataFormatID > 127 ? pointDataFormatID - 128 : pointDataFormatID;
    const params: GetPointParams = [reader, header, this.#transformer, offset];
    if (format === 0) return getPointFormat0(...params);
    else if (format === 1) return getPointFormat1(...params);
    else if (format === 2) return getPointFormat2(...params);
    else if (format === 3) return getPointFormat3(...params);
    else if (format === 6) return getPointFormat6(...params);
    else if (format === 7) return getPointFormat7(...params);
    else if (format === 8) return getPointFormat8(...params);
    else if (format === 4 || format === 5 || format === 9 || format === 10)
      throw new Error(
        `Format ${format} is not implemented because waveform data was dropped by 1.4+ specs`,
      );
    else throw new Error(`Unknown Point Data Format ID: ${format}`);
  }
}
