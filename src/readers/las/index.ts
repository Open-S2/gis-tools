import {
  ArithmeticDecoder,
  IntegerCompressor,
  LASWavePacket13,
  LASpoint10,
  LASpoint14,
  LASrgba,
  LASrgbaNir,
  LAZPoint10v1Reader,
  LAZPoint10v2Reader,
  LAZPoint14v3Reader,
  LAZbyte10v1Reader,
  LAZbyte10v2Reader,
  LAZbyte14v3Reader,
  LAZgpstime11v1Reader,
  LAZgpstime11v2Reader,
  LAZrgb12v1Reader,
  LAZrgb12v2Reader,
  LAZrgb14v3Reader,
  LAZrgbNir14v3Reader,
  LAZwavepacket13v1Reader,
  LAZwavepacket14v3Reader,
  modifyPoint14RawInput,
} from './laz';
import { LAZCompressor, LAZHeaderItemType } from './types';
import { Transformer, buildParamsFromGeoKeys, parseGeotiffRawGeoKeys, toReader } from '../..';
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
  VectorFeature,
  VectorPointM,
} from '../..';
import type { ItemReader, LAZContext } from './laz';
import type {
  LASExtendedVariableLengthRecord,
  LASFormat,
  LASHeader,
  LASVariableLengthRecord,
  LAZHeader,
  LAZHeaderItem,
} from './types';

export * from './types';

const U32_MAX = 4294967295;

/** Cleaned up type for getPoint */
type GetPointParams = [Reader, LASHeader, number];

/**
 * # LAS Reader
 *
 * ## Description
 * Reads LAS data. Supports LAS 1.4 specification although missing some support.
 * [See specification](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf)
 * Implements the {@link FeatureIterator} interface
 *
 * Data is stored like so: (EOF is a LAZ optional field and not used by LAS)
 *```
 * |            PUBLIC HEADER BLOCK           |
 * |          VARIABLE LENGTH RECORDS         |
 * |             POINT DATA RECORDS           |
 * ```
 *
 * ## Links
 * - https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
 * - https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
 * - https://liblas.org/development/index.html
 * - https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
 * - https://github.com/PDAL/PDAL
 * - https://github.com/libLAS/libLAS (deprecated for PDAL)
 * - https://github.com/LASzip
 */
export class LASReader implements FeatureIterator<undefined, LASFormat, Properties> {
  readonly reader: Reader;
  readonly header: LASHeader;
  readonly variableLengthRecords: Record<
    number,
    LASVariableLengthRecord | LASExtendedVariableLengthRecord
  > = {};
  readonly wkt?: string;
  readonly GeoKeyDirectory?: GeoKeyDirectory;
  readonly transformer = new Transformer();
  #decoder = new TextDecoder();
  /**
   * @param input - The LAS input data from a reader/buffer
   * @param definitions - an array of projection definitions for the transformer if needed
   * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
   * @param gridStores - an array of grid readers if needed
   */
  constructor(
    input: ReaderInputs,
    definitions: ProjectionTransformDefinition[] = [],
    epsgCodes: Record<string, string> = {},
    gridStores: GridReader[] = [],
  ) {
    this.reader = toReader(input);
    this.header = this.#parseHeader();
    this.#parseVariableLengthRecords();
    // set definitions, espgCodes, and gridStores
    for (const proj of definitions) this.transformer.insertDefinition(proj);
    for (const [key, value] of Object.entries(epsgCodes))
      this.transformer.insertEPSGCode(key, value);
    for (const { key, reader } of gridStores) this.transformer.addGridFromReader(key, reader);
    // try WTK
    this.wkt = this.#buildWKT();
    // they try GeoTiff
    this.GeoKeyDirectory = this.#buildGeoKeyDirectory();
  }

  /**
   * Get the number of points stored
   * @returns - the number of points
   */
  get length(): number {
    return this.header.numPoints;
  }

  /**
   * Reads a point in at index
   * @param index - The index of the point to read
   * @returns - The parsed point
   */
  getPoint(index: number): VectorPointM<LASFormat> {
    const { reader, header } = this;
    const { offsetToPoints, pointDataFormatID: format, pointDataRecordLength } = header;
    const offset = offsetToPoints + index * pointDataRecordLength;
    // const format = pointDataFormatID > 127 ? pointDataFormatID - 128 : pointDataFormatID;
    const params: GetPointParams = [reader, header, offset];
    let point: VectorPointM<LASFormat>;
    if (format === 0) point = getPointFormat0(...params);
    else if (format === 1) point = getPointFormat1(...params);
    else if (format === 2) point = getPointFormat2(...params);
    else if (format === 3) point = getPointFormat3(...params);
    else if (format === 6) point = getPointFormat6(...params);
    else if (format === 7) point = getPointFormat7(...params);
    else if (format === 8) point = getPointFormat8(...params);
    else if (format === 4 || format === 5 || format === 9 || format === 10)
      throw new Error(
        `Format ${format} is not implemented because waveform data was dropped by 1.4+ specs`,
      );
    else throw new Error(`Unknown Point Data Format ID: ${format}`);

    point = this.transformer.forward(point) as VectorPointM<LASFormat>;
    return point;
  }

  /**
   * Generator to iterate over each WGS84 lon-lat point
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<undefined, LASFormat, Properties>> {
    const numPoints = this.length;
    if (numPoints === 0) return;
    for (let i = 0; i < numPoints; i++) {
      const coordinates = this.getPoint(i);
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
   * - User ID char[16] 16 bytes
   * - Record ID unsigned short 2 bytes
   * - Record Length After Header unsigned short 2 bytes
   * - Description char[32] 32 bytes
   * - optional data: variable size
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
   * WKT Parsing
   *
   * For definition of WKT, we refer to Open Geospatial Consortium (OGC) specification “OpenGIS
   * coordinate transformation service implementation specification” revision 1.00 released 12
   * January 2001, section 7 (coordinate transformation services spec). This specification may be
   * found at www.opengeospatial.org/standards/ct. As there are a few dialects of WKT, please note
   * that LAS is not using the “ESRI WKT” dialect, which does not include TOWGS84 and authority
   * nodes.
   * - OGC MATH TRANSFORM WKT RECORD (2111)
   * - OGC COORDINATE SYSTEM WKT (2112)
   *
   * NOTE: It is required to use WKT if the point type is 6-10
   * @returns - the WKT string if it exists
   */
  #buildWKT(): string | undefined {
    const { header, variableLengthRecords } = this;
    // 4th bit of global encoding must be set
    if ((header.encoding & (1 << 3)) !== 0) return;
    // OGC MATH TRANSFORM WKT RECORD:
    const wktMathOGC = variableLengthRecords[2111]?.data;
    // OGC COORDINATE SYSTEM WKT:
    const wktCoordSystemData = variableLengthRecords[2112]?.data;
    if (wktMathOGC === undefined && wktCoordSystemData === undefined) return;
    const wktCoordSystem = this.#decoder.decode(wktMathOGC ?? wktCoordSystemData);
    this.transformer.setSource(wktCoordSystem);

    return wktCoordSystem;
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
  #buildGeoKeyDirectory(): GeoKeyDirectory | undefined {
    const { variableLengthRecords } = this;
    // GeoKeyDirectoryTag
    const geokeyRecord = variableLengthRecords[34735]?.data;
    if (geokeyRecord === undefined) return;
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
    const gkd = parseGeotiffRawGeoKeys(rawGeoKeys, {
      GeoKeyDirectory: this.GeoKeyDirectory,
      GeoDoubleParams,
      GeoAsciiParams,
    });
    const gkdParams = buildParamsFromGeoKeys(gkd);
    if (gkdParams !== undefined) this.transformer.setSource(gkdParams);

    return gkd;
  }
}

/** A step of decompression from the reader */
export interface LAZPointData {
  type: LAZHeaderItemType;
  rawData: DataView;
}

/**
 * # LASzip Reader
 *
 * ## Description
 * Reads LAS zipped data. Supports LAS 1.4 specification although missing some support.
 * [See specification](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf)
 * Implements the {@link FeatureIterator} interface
 *
 * Data is stored like so:
 *```
 * |            PUBLIC HEADER BLOCK           |
 * |          VARIABLE LENGTH RECORDS         |
 * |             POINT DATA RECORDS           |
 * | Extended Variable Length Records (EVLRs) |
 * |  Field Chunk table start position (EOF)  |
 * ```
 *
 * ## Links
 * - https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
 * - https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
 * - https://liblas.org/development/index.html
 * - https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
 * - https://github.com/PDAL/PDAL
 * - https://github.com/libLAS/libLAS (deprecated for PDAL)
 * - https://github.com/LASzip
 */
export class LASZipReader extends LASReader {
  readonly lazHeader: LAZHeader;
  #dec?: ArithmeticDecoder;
  decompressSelective = 0xffffffff; // all
  // Is true if this file uses layered compression for LAS 1.4.
  layeredLas14Compression = false;
  // number of points per chunk
  // #numReaders = 0;
  #chunkSize = U32_MAX; // U32
  // #chunkCount = 0; // U32
  // #currChunk = 0; // U32
  #numberChunks = 0; // U32
  // TODO: Remove this
  // eslint-disable-next-line no-unused-private-class-members
  #tabledChunks = 0; // U32
  #chunkTotals: number[] = []; // U32
  #chunkStarts: number[] = []; // I64
  // #pointStart = 0; // I64
  // #pointSize = 0; // U32
  // #seekPoint = 0;
  // #readersRaw: any[] = [];
  #readers: ItemReader[] = [];
  // #readersCompressed: any[] = [];

  /**
   * @param input - The LAZ input data from a reader/buffer
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
    // setup header variables and VLRs
    super(input, definitions, epsgCodes, gridStores);
    this.lazHeader = this.#buildLaz();
    this.#parseExtendedVariableLengthRecords();
    // prep decoder
    if (this.lazHeader.coder !== 0) throw Error(`Unsupported decoder: ${this.lazHeader.coder}`);
    // setup other decoding variables
    this.layeredLas14Compression = this.lazHeader.compressor === LAZCompressor.LAYERED_AND_CHUNKED;
    // this.#chunkCount = this.lazHeader.chunkSize;
    if (this.lazHeader.compressor !== LAZCompressor.POINTWISE) {
      if (this.lazHeader.chunkSize !== 0) this.#chunkSize = this.lazHeader.chunkSize;
      this.#numberChunks = U32_MAX;
      this.#readChunkTable();
    }
  }

  /**
   * Generator to iterate over each WGS84 lon-lat point
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<undefined, LASFormat, Properties>> {
    const startPos =
      this.header.offsetToPoints + (this.lazHeader.compressor !== LAZCompressor.POINTWISE ? 8 : 0);
    const numPoints = this.length;
    if (numPoints === 0) return;
    // set pos
    this.reader.seek(startPos);
    // init all readers
    for (let i = 0; i < numPoints; i++) {
      // console.log('i', i, this.reader.tell(), this.#chunkStarts);
      // if (i > 2) break;
      const coordinates = this.#readPoint(i === 0);
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
   * If the LAZ variable length record is present, build a LAZ parser
   * @returns - the LAZ header guide for decompressing the point data
   */
  #buildLaz() {
    const lazData = this.variableLengthRecords['22204']?.data;
    if (lazData === undefined || this.header.pointDataFormatID < 127)
      throw Error('LAZ data, but LAZ record not found.');
    return this.#parseLAZHeader(lazData);
  }

  /**
   * Parses the LAZ header
   * @param rawHeader - The raw header data
   * @returns - The parsed header
   */
  #parseLAZHeader(rawHeader: DataView) {
    const header: LAZHeader = {
      compressor: rawHeader.getUint16(0, true) as LAZCompressor,
      coder: rawHeader.getUint16(2, true),
      versionMajor: rawHeader.getUint8(4),
      versionMinor: rawHeader.getUint8(5),
      versionRevision: rawHeader.getUint16(6, true),
      options: rawHeader.getUint32(8, true),
      chunkSize: rawHeader.getUint32(12, true),
      numSpecialEvlrs: Number(rawHeader.getBigInt64(16, true)),
      offsetSpecialEvlrs: Number(rawHeader.getBigInt64(24, true)),
      numItems: rawHeader.getUint16(32, true),
      items: [],
    };
    // add decoder if compressor is not NONE
    const isCompressed = header.compressor !== LAZCompressor.NONE;
    if (isCompressed) this.#dec = new ArithmeticDecoder(this.reader);
    // Parse items
    for (let i = 0; i < header.numItems; i++) {
      const item: LAZHeaderItem = {
        type: rawHeader.getUint16(34 + i * 6, true) as LAZHeaderItemType,
        size: rawHeader.getUint16(36 + i * 6, true),
        version: rawHeader.getUint16(38 + i * 6, true),
      };
      header.items.push(item);
      if (isCompressed) this.#readers.push(this.#getPointCompressedReader(item));
    }

    return header;
  }

  /**
   * @param item - the item to build readers for
   * @returns - the compression reader for the item
   */
  #getPointCompressedReader(item: LAZHeaderItem): ItemReader {
    const { type, size, version } = item;
    if (type === LAZHeaderItemType.POINT10) {
      if (version === 1) return new LAZPoint10v1Reader(this.#dec!);
      else if (version === 2) return new LAZPoint10v2Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.GPSTIME11) {
      if (version === 1) return new LAZgpstime11v1Reader(this.#dec!);
      else if (version === 2) return new LAZgpstime11v2Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.RGB12) {
      if (version === 1) return new LAZrgb12v1Reader(this.#dec!);
      else if (version === 2) return new LAZrgb12v2Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.WAVEPACKET13) {
      if (version === 1) return new LAZwavepacket13v1Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.BYTE) {
      if (version === 1) return new LAZbyte10v1Reader(this.#dec!, size);
      else if (version === 2) return new LAZbyte10v2Reader(this.#dec!, size);
    } else if (type === LAZHeaderItemType.POINT14) {
      // TODO: V4
      if (version === 3) return new LAZPoint14v3Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.RGB14) {
      // TODO: V4
      if (version === 3) return new LAZrgb14v3Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.RGBNIR14) {
      // TODO: V4
      if (version === 3) return new LAZrgbNir14v3Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.WAVEPACKET14) {
      // TODO: V4
      if (version === 3) return new LAZwavepacket14v3Reader(this.#dec!);
    } else if (type === LAZHeaderItemType.BYTE14) {
      // TODO: V4
      if (version === 3) return new LAZbyte14v3Reader(this.#dec!, size);
    }
    throw Error(`Unsupported compressed point type: ${type} & version: ${version}`);
  }

  /**
   * The Compressed Data Block can be followed by any number of EVLRs, which are identical to the
   * LAS 1.4 specification. The EVLR is similar to a VLR, but can carry a larger payload, as the Record
   * Length After Header field is 8 bytes instead of 2 bytes. The number of EVLRs is specified in the
   * Number of Extended Variable Length Records field in the Public Header Block. The start of the
   * first EVLR is at the file offset indicated by the Start of first Extended Variable Length Record in
   * the Public Header Block.
   *
   * The Extended Variable Length Records must be accessed sequentially, since the size of each
   * variable length record is contained in the Extended Variable Length Record Header. Each Extended
   * Variable Length Record Header (i.e. without the optional payload data) is 60 bytes in length.
   *
   * Each record is as follows:
   * - Reserved unsigned short 2 bytes
   * - User ID char[16] 16 bytes
   * - Record ID unsigned short 2 bytes
   * - Record Length After Header unsigned long long 8 bytes
   * - Description char[32] 32 bytes
   * - optional data: variable size
   */
  #parseExtendedVariableLengthRecords(): void {
    const { reader, lazHeader } = this;
    const { offsetSpecialEvlrs, numSpecialEvlrs } = lazHeader;
    let position = offsetSpecialEvlrs;
    for (let i = 0; i < numSpecialEvlrs; ++i) {
      const recordLength = Number(reader.getBigUint64(position + 20, true));
      const record: LASExtendedVariableLengthRecord = {
        reserved: reader.getUint16(position, true),
        userID: reader.parseString(position + 2, 16),
        recordID: reader.getUint16(position + 18, true),
        recordLength,
        description: reader.parseString(position + 28, 32),
        data:
          recordLength > 0 ? reader.slice(position + 60, position + 60 + recordLength) : undefined,
      };
      position += 60 + recordLength;
      this.variableLengthRecords[record.recordID] = record;
    }
  }

  /**
   * @param uncompressed - true if the point data is raw and not compressed
   * @returns - the next point
   */
  #readPoint(uncompressed: boolean): VectorPointM<LASFormat> {
    // TODO: POINTWISE_AND_CHUNKED needs to use uncompressed section for each new chunk
    const { compressor } = this.lazHeader;
    const pointData: LAZPointData[] = [];
    const context: LAZContext = { value: 0 };
    // no decoder means it wasn't compressed, just pull the point
    if (uncompressed || compressor === LAZCompressor.NONE) {
      this.#firstChunkRead(context, pointData);
    } else this.#pointwiseCompressRead(pointData, context);

    let point = this.#toVectorPoint(pointData, this.header);
    point = this.transformer.forward(point) as VectorPointM<LASFormat>;
    // console.log('point', point);
    return point;
  }

  /**
   * @param context - current context
   * @param pointData - where to store the decompressed data
   */
  #firstChunkRead(context: LAZContext, pointData: LAZPointData[]): void {
    const { items } = this.lazHeader;
    let i: number;
    // first read in the raw data
    for (i = 0; i < items.length; i++) {
      const { type, size } = items[i];
      let rawData = this.reader.seekSlice(size);
      if (type === LAZHeaderItemType.POINT14) rawData = modifyPoint14RawInput(rawData);
      pointData.push({ type, rawData });
    }
    // now choose how we initialize
    if (this.layeredLas14Compression) {
      // set decoder
      this.#dec?.init(false);
      const _count = this.reader.getUint32(undefined);
      // chunk sizes
      for (i = 0; i < items.length; i++) this.#readers[i]?.chunkSizes(this.reader);
      // init
      for (i = 0; i < items.length; i++) this.#readers[i]?.init(pointData[i].rawData, context);
    } else {
      // initialize readers
      for (i = 0; i < items.length; i++) this.#readers[i]?.init(pointData[i].rawData, context);
      // NOW set decoder
      this.#dec?.init();
    }
  }

  /**
   * @param pointData - where to store the decompressed data
   * @param context - the context
   */
  #pointwiseCompressRead(pointData: LAZPointData[], context: LAZContext): void {
    const { items } = this.lazHeader;
    for (let i = 0; i < items.length; i++) {
      const { type, size } = items[i];
      let rawData: DataView = new DataView(new ArrayBuffer(size));
      if (type === LAZHeaderItemType.POINT14) rawData = modifyPoint14RawInput(rawData);
      this.#readers[i].read(rawData, context);
      pointData.push({ type, rawData });
    }
  }

  /**
   * @param pointData - a collection of raw data with it's type
   * @param header - the las header data
   * @returns a vector point
   */
  #toVectorPoint(pointData: LAZPointData[], header: LASHeader): VectorPointM<LASFormat> {
    const res = { x: 0, y: 0, z: 0, m: {} } as VectorPointM<LASFormat>;

    for (const { type, rawData } of pointData) {
      if (type === LAZHeaderItemType.POINT10) LASpoint10.rawToVectorPoint(rawData, header, res);
      else if (type === LAZHeaderItemType.GPSTIME11) res.m.gpsTime = rawData.getFloat64(0, true);
      else if (type === LAZHeaderItemType.RGB12 || type === LAZHeaderItemType.RGB14)
        LASrgba.rawToVectorPoint(rawData, res);
      else if (type === LAZHeaderItemType.WAVEPACKET13 || type === LAZHeaderItemType.WAVEPACKET14)
        LASWavePacket13.rawToVectorPoint(rawData, res);
      else if (type === LAZHeaderItemType.POINT14)
        LASpoint14.rawToVectorPoint(rawData, header, res);
      else if (type === LAZHeaderItemType.RGBNIR14) LASrgbaNir.rawToVectorPoint(rawData, res);
    }

    return res;
  }

  // /**
  //  *
  //  */
  // #initDec(): boolean {
  //   // maybe read chunk table (only if chunking enabled)
  //   if (this.#numberChunks === U32_MAX) {
  //     if (!this.#readChunkTable()) return false;
  //     this.#currChunk = 0;
  //     if (this.#chunkTotals.length > 0) this.#chunkSize = this.#chunkTotals[1];
  //   }

  //   this.#pointStart = this.reader.tell();
  //   // TODO: re-add here?
  //   // this.#readers = [];

  //   return true;
  // }

  /** If chunking is enabled, read the chunk table */
  #readChunkTable(): void {
    const { offsetToPoints } = this.header;
    let chunkTableStartPosition = Number(this.reader.getBigInt64(offsetToPoints, true));
    // this is where the chunks start
    const chunksStart = this.reader.tell(); // I64

    if (chunkTableStartPosition === -1) {
      // the compressor was writing to a non-seekable stream and wrote the chunk table start at the end
      // read the last 8 bytes
      chunkTableStartPosition = Number(this.reader.getBigInt64(this.reader.byteLength - 8, true));
    }

    // read the chunk table
    // move to where the chunk table starts
    this.reader.seek(chunkTableStartPosition);
    // fail if the version is wrong
    const version = this.reader.getUint32(undefined, true);
    if (version !== 0) throw new Error('Bad version number. Aborting.');
    // build the chunk table
    this.#numberChunks = this.reader.getUint32(undefined, true);
    this.#chunkTotals = [];
    // set chunk start and totals
    if (this.#chunkSize === U32_MAX) {
      this.#chunkTotals = new Array(this.#numberChunks + 1);
      this.#chunkTotals[0] = 0;
    } else this.#chunkStarts[0] = chunksStart;
    this.#tabledChunks = 1;
    if (this.#numberChunks > 0) {
      let i: number;
      this.#dec?.init();
      const ic = new IntegerCompressor(this.#dec!, 32, 2);
      ic.initDecompressor();
      for (i = 1; i <= this.#numberChunks; i++) {
        if (this.#chunkSize === U32_MAX)
          this.#chunkTotals[i] = ic.decompress(i > 1 ? this.#chunkTotals[i - 1] : 0, { value: 0 });
        this.#chunkStarts[i] = ic.decompress(i > 1 ? this.#chunkStarts[i - 1] : 0, { value: 1 });
        this.#tabledChunks++;
      }
      for (i = 1; i <= this.#numberChunks; i++) {
        if (this.#chunkSize === U32_MAX) this.#chunkTotals[i] += this.#chunkTotals[i - 1];
        this.#chunkStarts[i] += this.#chunkStarts[i - 1];
      }
    }
  }
}
