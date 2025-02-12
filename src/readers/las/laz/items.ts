import { toLASClassification, toLASClassification14, toLASClassificationFlag } from '../getPoint';

import type { LASFormat, LASHeader, VectorPointM } from '../../..';
import { i16Quantize, i8Clamp } from '../util';

/**
 * @param input - 30bits input
 * @returns - parsed to a LASpoint14 output
 */
export function modifyPoint14RawInput(input: DataView): DataView {
  const tmp = new LAStempReadPoint14(input);
  const res = new LASpoint14();
  res.x = tmp.x;
  res.y = tmp.y;
  res.z = tmp.z;
  res.intensity = tmp.intensity;
  if (tmp.numberOfReturns > 7) {
    if (tmp.returnNumber > 6) {
      if (tmp.returnNumber >= tmp.numberOfReturns) {
        res.legacyReturnNumber = 7;
      } else {
        res.legacyReturnNumber = 6;
      }
    } else {
      res.legacyReturnNumber = tmp.returnNumber;
    }
    res.legacyNumberOfReturns = 7;
  } else {
    res.legacyReturnNumber = tmp.returnNumber;
    res.legacyNumberOfReturns = tmp.numberOfReturns;
  }
  res.scanDirectionFlag = tmp.scanDirectionFlag;
  res.edgeOfFlightLine = tmp.edgeOfFlightLine;
  res.legacyFlags = (tmp.classificationFlags << 5) & 0xe0;
  if (tmp.classification < 32) res.legacyClassification |= tmp.classification;
  res.legacyScanAngleRank = i8Clamp(i16Quantize(0.006 * tmp.scanAngle));
  res.userData = tmp.userData;
  res.pointSourceID = tmp.pointSourceID;
  res.scannerChannel = tmp.scannerChannel;
  res.classificationFlags = tmp.classificationFlags;
  res.classification = tmp.classification;
  res.returnNumber = tmp.returnNumber;
  res.numberOfReturns = tmp.numberOfReturns;
  res.scanAngle = tmp.scanAngle;
  res.gpsTime = input.getFloat64(22, true);
  return res.data;
}

/** Base class for common data manipulation */
export class CopyDataView {
  /** @param data - the data to manipulate */
  constructor(readonly data: DataView) {}
  /**
   * @param data - the data to copy from
   * @param length - the length to copy
   */
  copyFrom(data: DataView, length: number): void {
    new Uint8Array(this.data.buffer).set(new Uint8Array(data.buffer, data.byteOffset, length));
  }
  /**
   * @param item - the data to copy to
   * @param length - the length to copy
   */
  copyTo(item: DataView, length: number): void {
    const { data } = this;
    new Uint8Array(item.buffer, item.byteOffset, length).set(
      new Uint8Array(data.buffer, data.byteOffset, length),
    );
  }
}

/**
 * Point 1.0 Internal manipulation tool
 * [Reference](https://github.com/LASzip/LASzip/blob/658d03825b221fab91ddf4571439724fda930191/src/lasreaditemcompressed_v1.cpp#L44)
 * The struct form:
 * ```cpp
 * struct LASpoint10
 * {
 *   I32 x;
 *   I32 y;
 *   I32 z;
 *   U16 intensity;
 *   U8 return_number : 3;
 *   U8 number_of_returns_of_given_pulse : 3;
 *   U8 scan_direction_flag : 1;
 *   U8 edge_of_flight_line : 1;
 *   U8 classification;
 *   I8 scan_angle_rank;
 *   U8 user_data;
 *   U16 point_source_ID;
 * };
 * ```
 */
export class LASpoint10 extends CopyDataView {
  /** @param data - 20 bytes */
  constructor(data: DataView = new DataView(new ArrayBuffer(20))) {
    super(data);
  }

  /**
   * Modify the vector point given the input data and header
   * @param data - the raw data
   * @param header - the las header
   * @param point - the vector point to modify
   */
  static rawToVectorPoint(data: DataView, header: LASHeader, point: VectorPointM<LASFormat>): void {
    const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = header;
    const lasPoint10 = new LASpoint10(data);
    point.x = lasPoint10.x * xScaleFactor + xOffset;
    point.y = lasPoint10.y * yScaleFactor + yOffset;
    point.z = lasPoint10.z * zScaleFactor + zOffset;
    point.m.intensity = lasPoint10.intensity;
    point.m.returnNumber = lasPoint10.returnNumber;
    point.m.numberOfReturns = lasPoint10.numberOfReturns;
    point.m.scanDirectionFlag = lasPoint10.scanDirectionFlag;
    point.m.edgeOfFlightLine = lasPoint10.edgeOfFlightLine;
    point.m.classification = toLASClassification(lasPoint10.class);
    point.m.isSynthetic = lasPoint10.isSynthetic;
    point.m.isKeyPoint = lasPoint10.isKeyPoint;
    point.m.isWithheld = lasPoint10.isWithheld;
    point.m.scanAngleRank = lasPoint10.scanAngleRank;
    point.m.userData = lasPoint10.userData;
    point.m.pointSourceID = lasPoint10.pointSourceID;
  }

  /** @returns the I32 at x position */
  get x(): number {
    return this.data.getInt32(0, true);
  }
  /** set an I32 at x position */
  set x(value: number) {
    this.data.setInt32(0, value, true);
  }

  /** @returns the I32 at y position */
  get y(): number {
    return this.data.getInt32(4, true);
  }
  /** set an I32 at y position */
  set y(value: number) {
    this.data.setInt32(4, value, true);
  }

  /** @returns the I32 at z position */
  get z(): number {
    return this.data.getInt32(8, true);
  }
  /** set an I32 at z position */
  set z(value: number) {
    this.data.setInt32(8, value, true);
  }

  /** @returns the U16 at intensity position */
  get intensity(): number {
    return this.data.getUint16(12, true);
  }
  /** set an U16 at intensity position */
  set intensity(value: number) {
    this.data.setUint16(12, value, true);
  }

  /** @returns the U8 at flags position */
  get flags(): number {
    return this.data.getUint8(14);
  }
  /** @returns the return number flag */
  get returnNumber(): number {
    return this.flags & 0b00000111;
  }
  /** @returns the number of returns flag */
  get numberOfReturns(): number {
    return (this.flags & 0b00111000) >> 3;
  }
  /** @returns the scan direction flag */
  get scanDirectionFlag(): number {
    return (this.flags & 0b01000000) >> 6;
  }
  /** @returns the edge of flight line flag */
  get edgeOfFlightLine(): number {
    return this.flags >> 7;
  }
  /** set an U8 at flags position */
  set flags(value: number) {
    this.data.setUint8(14, value);
  }

  /** @returns the U8 at class position */
  get class(): number {
    return this.data.getUint8(15);
  }
  /** @returns the isSynthetic flag */
  get isSynthetic(): boolean {
    return (this.class & (1 << 5)) !== 0;
  }
  /** @returns the isKeyPoint flag */
  get isKeyPoint(): boolean {
    return (this.class & (1 << 6)) !== 0;
  }
  /** @returns the isWithheld flag */
  get isWithheld(): boolean {
    return (this.class & (1 << 7)) !== 0;
  }
  /** set an U8 at class position */
  set class(value: number) {
    this.data.setUint8(15, value);
  }

  /** @returns the I8 at scan angle rank */
  get scanAngleRank(): number {
    return this.data.getInt8(16);
  }
  /** set an I8 at scan angle rank */
  set scanAngleRank(value: number) {
    this.data.setInt8(16, value);
  }

  /** @returns the U8 at user data position */
  get userData(): number {
    return this.data.getUint8(17);
  }
  /** set an U8 at user data position */
  set userData(value: number) {
    this.data.setUint8(17, value);
  }

  /** @returns the U16 at point source ID position */
  get pointSourceID(): number {
    return this.data.getUint16(18, true);
  }
  /** set an U16 at point source ID position */
  set pointSourceID(value: number) {
    this.data.setUint16(18, value, true);
  }
}

/** LAS 1.0 RGBA Point data. Each color channel is 2 bytes. */
export class LASrgba extends CopyDataView {
  /**  @param data - 6 bytes (2 bytes each for R, G, B) */
  constructor(data: DataView = new DataView(new ArrayBuffer(6))) {
    super(data);
  }

  /**
   * Modify the vector point given the input data
   * @param data - the raw data
   * @param point - the vector point to modify
   */
  static rawToVectorPoint(data: DataView, point: VectorPointM<LASFormat>): void {
    const lasRgba = new LASrgba(data);
    point.m.rgba = {
      r: lasRgba.r,
      g: lasRgba.g,
      b: lasRgba.b,
      a: 255,
    };
  }

  /** @returns the U16 red position */
  get r(): number {
    return this.data.getInt16(0, true);
  }
  /** set an U16 red position */
  set r(value: number) {
    this.data.setInt16(0, value, true);
  }
  /** @returns the U16 green position */
  get g(): number {
    return this.data.getInt16(2, true);
  }
  /** set an U16 green position */
  set g(value: number) {
    this.data.setInt16(2, value, true);
  }
  /** @returns the U16 blue position */
  get b(): number {
    return this.data.getInt16(4, true);
  }
  /** set an U16 blue position */
  set b(value: number) {
    this.data.setInt16(4, value, true);
  }
}

/** LAS 1.0 RGBA Point data. Each color channel is 2 bytes. */
export class LASrgbaNir extends LASrgba {
  /**  @param data - 8 bytes (2 bytes each for R, G, B, and NIR) */
  constructor(data: DataView = new DataView(new ArrayBuffer(8))) {
    super(data);
  }

  /**
   * Modify the vector point given the input data
   * @param data - the raw data
   * @param point - the vector point to modify
   */
  static rawToVectorPoint(data: DataView, point: VectorPointM<LASFormat>): void {
    super.rawToVectorPoint(data, point);
    point.m.nir = data.getInt16(6, true);
  }

  /** @returns the U16 blue position */
  get nir(): number {
    return this.data.getInt16(6, true);
  }
  /** set an U16 blue position */
  set nir(value: number) {
    this.data.setInt16(6, value, true);
  }
}

/** LAS 1.0 Wave Packet. Each wave packet is 29 bytes. */
export class LASWavePacket13 extends CopyDataView {
  /** @param data - 29 */
  constructor(data: DataView = new DataView(new ArrayBuffer(29))) {
    const actualData = new DataView(data.buffer, data.byteOffset, data.byteLength);
    super(actualData);
  }

  /**
   * Modify the vector point given the input data
   * @param data - the raw data
   * @param point - the vector point to modify
   */
  static rawToVectorPoint(data: DataView, point: VectorPointM<LASFormat>): void {
    const lasWavePacket = new LASWavePacket13(data);
    point.m.wavePacketDescriptorIndex = lasWavePacket.index;
    point.m.wavePacketOffset = lasWavePacket.offset;
    point.m.wavePacketLength = lasWavePacket.packetSize;
    point.m.waveformLocationReturnPoint = lasWavePacket.returnPoint;
    point.m.xT = lasWavePacket.x;
    point.m.yT = lasWavePacket.y;
    point.m.zT = lasWavePacket.z;
  }

  /** @returns - the index */
  get index(): number {
    return this.data.getUint8(0);
  }
  /** set the index */
  set index(value: number) {
    this.data.setUint8(0, value);
  }

  /** @returns the offset */
  get offset(): number {
    return Number(this.data.getBigUint64(1, true));
  }
  /** set an offset */
  set offset(value: number | bigint) {
    this.data.setBigUint64(1, BigInt(value), true);
  }
  /** @returns the packet size */
  get packetSize(): number {
    return this.data.getUint32(9, true);
  }
  /** set an packet size */
  set packetSize(value: number) {
    this.data.setUint32(9, value, true);
  }
  /** @returns the return point */
  get returnPoint(): number {
    return this.data.getInt32(13, true);
  }
  /** set an return point */
  set returnPoint(value: number) {
    this.data.setInt32(13, value, true);
  }
  /** @returns the x */
  get x(): number {
    return this.data.getInt32(17, true);
  }
  /** set an x */
  set x(value: number) {
    this.data.setInt32(17, value, true);
  }
  /** @returns the y */
  get y(): number {
    return this.data.getInt32(21, true);
  }
  /** set an y */
  set y(value: number) {
    this.data.setInt32(21, value, true);
  }
  /** @returns the z */
  get z(): number {
    return this.data.getInt32(25, true);
  }
  /** set an z */
  set z(value: number) {
    this.data.setInt32(25, value, true);
  }
}

/**
 * LAS Internal Temp Point Format 1.4
 * The struct form:
 * ```cpp
 * typedef struct LAStempReadPoint14 {
 *   I32 X;
 *   I32 Y;
 *   I32 Z;
 *   U16 intensity;
 *   U8 return_number : 4;
 *   U8 number_of_returns : 4;
 *   U8 classification_flags : 4;
 *   U8 scanner_channel : 2;
 *   U8 scan_direction_flag : 1;
 *   U8 edge_of_flight_line : 1;
 *   U8 classification;
 *   U8 user_data;
 *   I16 scan_angle;
 *   U16 point_source_ID;
 * };
 * ```
 */
export class LAStempReadPoint14 extends CopyDataView {
  /** @param data - 22 bytes */
  constructor(data: DataView = new DataView(new ArrayBuffer(22))) {
    super(data);
  }

  /** @returns the I32 at x position */
  get x(): number {
    return this.data.getInt32(0, true);
  }
  /** @returns the I32 at y position */
  get y(): number {
    return this.data.getInt32(4, true);
  }
  /** @returns the I32 at z position */
  get z(): number {
    return this.data.getInt32(8, true);
  }
  /** @returns the U16 at intensity position */
  get intensity(): number {
    return this.data.getUint16(12, true);
  }
  /** @returns the U8 at returnNumber position */
  get returnNumber(): number {
    return this.data.getUint8(14) & 0b1111;
  }
  /** @returns the U8 at numberOfReturns position */
  get numberOfReturns(): number {
    return (this.data.getUint8(14) & 0b11110000) >> 4;
  }
  /** @returns the U8 at classificationFlags position */
  get classificationFlags(): number {
    return this.data.getUint8(15) & 0b1111;
  }
  /** @returns the U8 at scannerChannel position */
  get scannerChannel(): number {
    return (this.data.getUint8(15) & 0b00110000) >> 4;
  }
  /** @returns the U8 at scanDirectionFlag position */
  get scanDirectionFlag(): number {
    return (this.data.getUint8(15) & 0b01000000) >> 6;
  }
  /** @returns the U8 at edgeOfFlightLine position */
  get edgeOfFlightLine(): number {
    return (this.data.getUint8(15) & 0b10000000) >> 7;
  }
  /** @returns the U8 at classification position */
  get classification(): number {
    return this.data.getUint8(16);
  }
  /** @returns the U8 at userData position */
  get userData(): number {
    return this.data.getUint8(17);
  }
  /** @returns the I16 at scanAngle position */
  get scanAngle(): number {
    return this.data.getInt16(18, true);
  }
  /** @returns the U16 at pointSourceID position */
  get pointSourceID(): number {
    return this.data.getUint16(20, true);
  }
}

/**
 * LAS Internal Point Format 1.4
 * The struct form:
 * ```cpp
 * typedef struct LASpoint14 {
 *   I32 X;
 *   I32 Y;
 *   I32 Z;
 *   U16 intensity;
 *   U8 legacy_return_number : 3;
 *   U8 legacy_number_of_returns : 3;
 *   U8 scan_direction_flag : 1;
 *   U8 edge_of_flight_line : 1;
 *
 *   U8 legacy_classification : 5;
 *   U8 legacy_flags : 3;
 *   I8 legacy_scan_angle_rank;
 *   U8 user_data;
 *   U16 point_source_ID;
 *   // LAS 1.4 only
 *   I16 scan_angle;
 *   U8 legacy_point_type : 2;
 *   U8 scanner_channel : 2;
 *   U8 classification_flags : 4;
 *   U8 classification;
 *   U8 return_number : 4;
 *   U8 number_of_returns : 4;
 *   // LASlib internal use only
 *   U8 deleted_flag;
 *   // for 8 byte alignment of the GPS time
 *   U8 dummy[2];
 *   // compressed LASzip 1.4 points only
 *   BOOL gps_time_change;
 *   F64 gps_time;
 *   U16 rgb[4];
 *   LASwavepacket wavepacket;
 * } LASpoint14;
 * ```
 */
export class LASpoint14 extends CopyDataView {
  /** @param data - 45 bytes */
  constructor(data: DataView = new DataView(new ArrayBuffer(45))) {
    super(data);
  }

  /**
   * Modify the vector point given the input data and header
   * @param data - the raw data
   * @param header - the las header
   * @param point - the vector point to modify
   */
  static rawToVectorPoint(data: DataView, header: LASHeader, point: VectorPointM<LASFormat>): void {
    const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = header;
    const lasPoint14 = new LASpoint14(data);
    point.x = lasPoint14.x * xScaleFactor + xOffset;
    point.y = lasPoint14.y * yScaleFactor + yOffset;
    point.z = lasPoint14.z * zScaleFactor + zOffset;
    point.m.intensity = lasPoint14.intensity;
    point.m.returnNumber = lasPoint14.legacyReturnNumber;
    point.m.numberOfReturns = lasPoint14.legacyNumberOfReturns;
    point.m.scanDirectionFlag = lasPoint14.scanDirectionFlag;
    point.m.edgeOfFlightLine = lasPoint14.edgeOfFlightLine;
    point.m.classification = toLASClassification14(lasPoint14.classification);
    point.m.classificationFlag = toLASClassificationFlag(lasPoint14.classificationFlags);
    point.m.isSynthetic = lasPoint14.isSynthetic;
    point.m.isKeyPoint = lasPoint14.isKeyPoint;
    point.m.isWithheld = lasPoint14.isWithheld;
    point.m.scanAngle = lasPoint14.scanAngle;
    point.m.userData = lasPoint14.userData;
    point.m.scannerChannel = lasPoint14.scannerChannel;
    point.m.pointSourceID = lasPoint14.pointSourceID;
    point.m.gpsTime = lasPoint14.gpsTime;
  }

  /** @returns the I32 at x position */
  get x(): number {
    return this.data.getInt32(0, true);
  }
  /** set an I32 at x position */
  set x(value: number) {
    this.data.setInt32(0, value, true);
  }

  /** @returns the I32 at y position */
  get y(): number {
    return this.data.getInt32(4, true);
  }
  /** set an I32 at y position */
  set y(value: number) {
    this.data.setInt32(4, value, true);
  }

  /** @returns the I32 at z position */
  get z(): number {
    return this.data.getInt32(8, true);
  }
  /** set an I32 at z position */
  set z(value: number) {
    this.data.setInt32(8, value, true);
  }

  /** @returns the U16 at intensity position */
  get intensity(): number {
    return this.data.getUint16(12, true);
  }
  /** set an U16 at intensity position */
  set intensity(value: number) {
    this.data.setUint16(12, value, true);
  }

  /** @returns the U8 at first flags position */
  get flags1(): number {
    return this.data.getUint8(14);
  }
  /** @returns the U8 at legacy return number position */
  get legacyReturnNumber(): number {
    return this.flags1 & 0b00000111;
  }
  /** set an U8 at legacy return number position */
  set legacyReturnNumber(value: number) {
    this.data.setUint8(14, (this.flags1 & 0b11111000) | (value & 0b111));
  }
  /** @returns the U8 at legacy number of returns position */
  get legacyNumberOfReturns(): number {
    return (this.flags1 & 0b00111000) >> 3;
  }
  /** set an U8 at legacy number of returns position */
  set legacyNumberOfReturns(value: number) {
    this.data.setUint8(14, (this.flags1 & 0b11000111) | ((value & 0b111) << 3));
  }
  /** @returns the U8 at scan direction flag position */
  get scanDirectionFlag(): number {
    return (this.flags1 & 0b01000000) >> 6;
  }
  /** set an U8 at scan direction flag position */
  set scanDirectionFlag(value: number) {
    this.data.setUint8(14, (this.flags1 & 0b10111111) | ((value & 0b1) << 6));
  }
  /** @returns the U8 at edge of flight line position */
  get edgeOfFlightLine(): number {
    return (this.flags1 & 0b10000000) >> 7;
  }
  /** set an U8 at edge of flight line position */
  set edgeOfFlightLine(value: number) {
    this.data.setUint8(14, (this.flags1 & 0b01111111) | ((value & 0b1) << 7));
  }
  /** set an U8 at first flags position */
  set flags1(value: number) {
    this.data.setUint8(14, value);
  }

  /** @returns the U8 at second flags position */
  get flags2(): number {
    return this.data.getUint8(15);
  }
  /** @returns the U8 at legacy classification position */
  get legacyClassification(): number {
    return this.flags2 & 0b11111;
  }
  /** @returns the isSynthetic flag */
  get isSynthetic(): boolean {
    return (this.legacyClassification & (1 << 5)) !== 0;
  }
  /** @returns the isKeyPoint flag */
  get isKeyPoint(): boolean {
    return (this.legacyClassification & (1 << 6)) !== 0;
  }
  /** @returns the isWithheld flag */
  get isWithheld(): boolean {
    return (this.legacyClassification & (1 << 7)) !== 0;
  }
  /** set an U8 at legacy classification position */
  set legacyClassification(value: number) {
    this.data.setUint8(15, (this.flags2 & 0b11100000) | (value & 0b11111));
  }
  /** @returns the U8 at legacy flags position */
  get legacyFlags(): number {
    return (this.flags2 & 0b11100000) >> 5;
  }
  /** set an U8 at legacy flags position */
  set legacyFlags(value: number) {
    this.data.setUint8(15, (this.flags2 & 0b00011111) | ((value & 0b111) << 5));
  }
  /** set an U8 at second flags position */
  set flags2(value: number) {
    this.data.setUint8(15, value);
  }

  /** @returns the I8 at legacy scan angle rank position */
  get legacyScanAngleRank(): number {
    return this.data.getInt8(16);
  }
  /** set an I8 at legacy scan angle rank position */
  set legacyScanAngleRank(value: number) {
    this.data.setInt8(16, value);
  }

  /** @returns the U8 at user data position */
  get userData(): number {
    return this.data.getUint8(17);
  }
  /** set an U8 at user data position */
  set userData(value: number) {
    this.data.setUint8(17, value);
  }

  /** @returns the U16 at point source ID position */
  get pointSourceID(): number {
    return this.data.getUint16(18, true);
  }
  /** set an U16 at point source ID position */
  set pointSourceID(value: number) {
    this.data.setUint16(18, value, true);
  }

  /// LAS 1.4 only

  /** @returns the I16 at scan angle position */
  get scanAngle(): number {
    return this.data.getInt16(20, true);
  }
  /** set an I16 at scan angle position */
  set scanAngle(value: number) {
    this.data.setInt16(20, value, true);
  }

  /** @returns the U8 at third flags position */
  get flags3(): number {
    return this.data.getUint8(22);
  }
  /** @returns the U8 at legacy point type position */
  get legacyPointType(): number {
    return this.flags3 & 0b11;
  }
  /** @returns the U8 at scanner channel position */
  get scannerChannel(): number {
    return (this.flags3 & 0b1100) >> 2;
  }
  /** set an U8 at scanner channel position */
  set scannerChannel(value: number) {
    this.data.setUint8(22, (this.flags3 & 0b11110011) | ((value & 0b11) << 2));
  }
  /** @returns the U8 at classification flags position */
  get classificationFlags(): number {
    return (this.flags3 & 0b11110000) >> 4;
  }
  /** set an U8 at classification flags position */
  set classificationFlags(value: number) {
    this.data.setUint8(22, (this.flags3 & 0b00001111) | ((value & 0b1111) << 4));
  }
  /** set an U8 at third flags position */
  set flags3(value: number) {
    this.data.setUint8(22, value);
  }

  /** @returns the U8 at classification position */
  get classification(): number {
    return this.data.getUint8(23);
  }
  /** set an U8 at classification position */
  set classification(value: number) {
    this.data.setUint8(23, value);
  }

  /** @returns the U8 at fourth flags position */
  get flags4(): number {
    return this.data.getUint8(24);
  }
  /** @returns the U8 at return number position */
  get returnNumber(): number {
    return this.flags4 & 0b00001111;
  }
  /** set an U8 at return number position */
  set returnNumber(value: number) {
    this.data.setUint8(24, (this.flags4 & 0b11110000) | (value & 0b1111));
  }
  /** @returns the U8 at number of returns position */
  get numberOfReturns(): number {
    return (this.flags4 & 0b11110000) >> 4;
  }
  /** set an U8 at return number position */
  set numberOfReturns(value: number) {
    this.data.setUint8(24, (this.flags4 & 0b00001111) | ((value & 0b1111) << 4));
  }
  /** set an U8 at fourth flags position */
  set flags4(value: number) {
    this.data.setUint8(24, value);
  }

  /// LASlib internal use only

  /** @returns the U8 at deleted flag position */
  get deletedFlag(): number {
    return this.data.getUint8(25);
  }
  /** set an U8 at deleted flag position */
  set deletedFlag(value: number) {
    this.data.setUint8(25, value);
  }

  /// For 8 byte alignment of the GPS time

  /** @returns the U8 at dummy position */
  get dummy(): number {
    return this.data.getUint16(26, true);
  }
  /** set an U8 at dummy position */
  set dummy(value: number) {
    this.data.setUint16(26, value, true);
  }

  /// Compressed LASzip 1.4 points only

  /** @returns the BOOL at gps time change position */
  get gpsTimeChange(): number {
    return this.data.getUint8(28);
  }
  /** set an BOOL at gps time change position */
  set gpsTimeChange(value: number) {
    this.data.setUint8(28, value);
  }

  /** @returns the F64 at gps time position */
  get gpsTime(): number {
    return this.data.getFloat64(29, true);
  }
  /** set an F64 at gps time position */
  set gpsTime(value: number) {
    this.data.setFloat64(29, value, true);
  }

  /** @returns the U16 red position */
  get r(): number {
    return this.data.getUint16(37, true);
  }
  /** set an U16 red position */
  set r(value: number) {
    this.data.setUint16(37, value, true);
  }
  /** @returns the U16 green position */
  get g(): number {
    return this.data.getUint16(39, true);
  }
  /** set an U16 green position */
  set g(value: number) {
    this.data.setUint16(39, value, true);
  }
  /** @returns the U16 blue position */
  get b(): number {
    return this.data.getUint16(41, true);
  }
  /** set an U16 blue position */
  set b(value: number) {
    this.data.setUint16(41, value, true);
  }
}
