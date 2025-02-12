import type { LASFormat0_5, LASFormat6_10, LASHeader } from './types';
import type { Reader, VectorPointM } from '../..';

/**
 * Reads a point using the Point Data Record Format 0
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 0 metadata
 */
export function getPointFormat0(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = header;
  const bits = reader.getUint32(offset + 14, littleEndian);
  const classBits = reader.getUint8(offset + 15);
  const point: VectorPointM<LASFormat0_5> = {
    x: reader.getInt32(offset, littleEndian) * xScaleFactor + xOffset,
    y: reader.getInt32(offset + 4, littleEndian) * yScaleFactor + yOffset,
    z: reader.getInt32(offset + 8, littleEndian) * zScaleFactor + zOffset,
    m: {
      intensity: reader.getUint16(offset + 12, littleEndian),
      returnNumber: bits & 0b00000111, // 3 bits (bits 0 – 2)
      numberOfReturns: (bits & 0b00111000) >> 3, // 3 bits (bits 3 – 5)
      scanDirectionFlag: (bits & 0b01000000) >> 6, // 1 bit (bit 6)
      edgeOfFlightLine: (bits & 0b10000000) >> 7, // 1 bit (bit 7)
      classification: toLASClassification(classBits),
      isSynthetic: (classBits & (1 << 5)) !== 0,
      isKeyPoint: (classBits & (1 << 6)) !== 0,
      isWithheld: (classBits & (1 << 7)) !== 0,
      scanAngleRank: reader.getInt8(offset + 16),
      userData: reader.getUint8(offset + 17),
      pointSourceID: reader.getUint16(offset + 18, littleEndian),
    },
  };
  return point;
}

/**
 * Reads a point using the Point Data Record Format 1
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 1 metadata
 */
export function getPointFormat1(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const point = getPointFormat0(reader, header, offset, littleEndian);
  point.m.gpsTime = reader.getFloat64(offset + 20, littleEndian);
  return point;
}

/**
 * Reads a point using the Point Data Record Format 2
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 2 metadata
 */
export function getPointFormat2(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const point = getPointFormat0(reader, header, offset, littleEndian);
  point.m.rgba = {
    r: reader.getUint16(offset + 20, littleEndian),
    g: reader.getUint16(offset + 22, littleEndian),
    b: reader.getUint16(offset + 24, littleEndian),
    a: 255,
  };
  return point;
}

/**
 * Reads a point using the Point Data Record Format 3
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 3 metadata
 */
export function getPointFormat3(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const point = getPointFormat1(reader, header, offset, littleEndian);
  point.m.rgba = {
    r: reader.getUint16(offset + 28, littleEndian),
    g: reader.getUint16(offset + 30, littleEndian),
    b: reader.getUint16(offset + 32, littleEndian),
    a: 255,
  };
  return point;
}

/**
 * Reads a point using the Point Data Record Format 4
 * https://github.com/ASPRSorg/LAS/wiki/Waveform-Data-Packet-Descriptors-Explained
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 4 metadata
 */
export function getPointFormat4(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const point = getPointFormat1(reader, header, offset, littleEndian);
  point.m.wavePacketDescriptorIndex = reader.getUint8(offset + 28);
  point.m.wavePacketOffset = Number(reader.getBigUint64(offset + 29, littleEndian));
  point.m.wavePacketLength = reader.getUint32(offset + 37, littleEndian);
  point.m.waveformLocationReturnPoint = reader.getFloat32(offset + 41, littleEndian);
  point.m.xT = reader.getFloat32(offset + 45, littleEndian);
  point.m.yT = reader.getFloat32(offset + 49, littleEndian);
  point.m.zT = reader.getFloat32(offset + 53, littleEndian);
  return point;
}

/**
 * Reads a point using the Point Data Record Format 5
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 4 metadata
 */
export function getPointFormat5(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat0_5> {
  const point = getPointFormat3(reader, header, offset, littleEndian);
  point.m.wavePacketDescriptorIndex = reader.getUint8(offset + 34);
  point.m.wavePacketOffset = Number(reader.getBigUint64(offset + 35, littleEndian));
  point.m.wavePacketLength = reader.getUint32(offset + 43, littleEndian);
  point.m.waveformLocationReturnPoint = reader.getFloat32(offset + 47, littleEndian);
  point.m.xT = reader.getFloat32(offset + 51, littleEndian);
  point.m.yT = reader.getFloat32(offset + 55, littleEndian);
  point.m.zT = reader.getFloat32(offset + 59, littleEndian);
  return point;
}

/**
 * Reads a point using the Point Data Record Format 0
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 0 metadata
 */
export function getPointFormat6(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat6_10> {
  const { xOffset, yOffset, zOffset, xScaleFactor, yScaleFactor, zScaleFactor } = header;
  const bits1 = reader.getUint8(offset + 14);
  const bits2 = reader.getUint8(offset + 15);
  const point: VectorPointM<LASFormat6_10> = {
    x: reader.getInt32(offset, littleEndian) * xScaleFactor + xOffset,
    y: reader.getInt32(offset + 4, littleEndian) * yScaleFactor + yOffset,
    z: reader.getInt32(offset + 8, littleEndian) * zScaleFactor + zOffset,
    m: {
      intensity: reader.getUint16(offset + 12, littleEndian),
      returnNumber: bits1 & 0b00001111, // 4 bits (bits 0 – 3)
      numberOfReturns: (bits1 & 0b11110000) >> 4, // 4 bits (bits 4 – 7)
      classificationFlag: toLASClassificationFlag(bits2), // 4 bis (bit 0 - 3)
      scannerChannel: (bits2 & 0b00110000) >> 4, // 2 bits (bit 4 - 5)
      scanDirectionFlag: (bits2 & 0b01000000) >> 6, // 1 bit (bit 6)
      edgeOfFlightLine: (bits2 & 0b10000000) >> 7, // 1 bit (bit 7)
      classification: toLASClassification14(reader.getUint8(offset + 16)),
      userData: reader.getUint8(offset + 17),
      scanAngle: reader.getInt16(offset + 18, littleEndian),
      pointSourceID: reader.getUint16(offset + 20, littleEndian),
      gpsTime: reader.getFloat64(offset + 22, littleEndian),
    },
  };
  return point;
}

/**
 * Reads a point using the Point Data Record Format 7
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 7 metadata
 */
export function getPointFormat7(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat6_10> {
  const point = getPointFormat6(reader, header, offset, littleEndian);
  point.m.rgba = {
    r: reader.getUint16(offset + 30, littleEndian),
    g: reader.getUint16(offset + 32, littleEndian),
    b: reader.getUint16(offset + 34, littleEndian),
    a: 255,
  };
  return point;
}

/**
 * Reads a point using the Point Data Record Format 8
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 8 metadata
 */
export function getPointFormat8(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat6_10> {
  const point = getPointFormat7(reader, header, offset, littleEndian);
  point.m.nir = reader.getUint16(offset + 36, littleEndian);
  return point;
}

/**
 * Reads a point using the Point Data Record Format 9
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 9 metadata
 */
export function getPointFormat9(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat6_10> {
  const point = getPointFormat6(reader, header, offset, littleEndian);
  point.m.wavePacketDescriptorIndex = reader.getInt16(offset + 30, littleEndian);
  point.m.wavePacketOffset = Number(reader.getBigUint64(offset + 31, littleEndian));
  point.m.wavePacketLength = reader.getUint32(offset + 39, littleEndian);
  point.m.waveformLocationReturnPoint = reader.getFloat32(offset + 43, littleEndian);
  point.m.xT = reader.getFloat32(offset + 47, littleEndian);
  point.m.yT = reader.getFloat32(offset + 51, littleEndian);
  point.m.zT = reader.getFloat32(offset + 55, littleEndian);
  return point;
}

/**
 * Reads a point using the Point Data Record Format 10
 * @param reader - data reader, works like a DataView
 * @param header - las header
 * @param offset - where to start reading in the point data
 * @param littleEndian - endianess. Defaults to true (LAS always LE but LAZ may have BE)
 * @returns - The parsed point with Format 10 metadata
 */
export function getPointFormat10(
  reader: Reader | DataView,
  header: LASHeader,
  offset = 0,
  littleEndian = true,
): VectorPointM<LASFormat6_10> {
  const point = getPointFormat7(reader, header, offset, littleEndian);
  point.m.wavePacketDescriptorIndex = reader.getUint8(offset + 38);
  point.m.wavePacketOffset = Number(reader.getBigUint64(offset + 39, littleEndian));
  point.m.wavePacketLength = reader.getUint32(offset + 47, littleEndian);
  point.m.waveformLocationReturnPoint = reader.getFloat32(offset + 51, littleEndian);
  point.m.xT = reader.getFloat32(offset + 55, littleEndian);
  point.m.yT = reader.getFloat32(offset + 59, littleEndian);
  point.m.zT = reader.getFloat32(offset + 63, littleEndian);
  return point;
}

/**
 * Converts a number into a LASClassification
 * @param classification - the number
 * @returns - the LASClassification
 */
export function toLASClassification(classification: number): string {
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
export function toLASClassificationFlag(classFlag: number): string {
  const firstThreeBits = classFlag & 0b1111;
  if (firstThreeBits === 0) return 'Synthetic';
  if (firstThreeBits === 1) return 'Key-point';
  if (firstThreeBits === 2) return 'Withheld';
  if (firstThreeBits === 3) return 'Overlap';
  return 'Unknown';
}

/**
 * Converts a number into a LASClassification v1.4
 * @param classification - the number
 * @returns - the LASClassification
 */
export function toLASClassification14(classification: number): string {
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
