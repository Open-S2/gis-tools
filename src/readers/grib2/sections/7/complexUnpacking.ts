import type { Grib2Sections } from '..';
import type { Reader } from '../../..';
import type { ComplexPackingTemplate, ComplexSpatialPackingTemplate } from '../5';

/**
 * # Data Template 7.2 - Grid Point Data - Complex Packing
 *
 * ## Contents
 * - **6-xx**: NG group reference values (X1 in the decoding formula), each of which is encoded using
 * the number of bits specified in octet 20 of data representation template 5.0. Bits set to zero shall
 * be appended as necessary to ensure this sequence of numbers ends on an octet boundary
 * - **[xx+1]-yy**: NG group widths, each of which is encoded using the number of bits specified in
 * octet 37 of data representation template 5.2. Bits set to zero shall be appended as necessary to
 * ensure this sequence of numbers ends on an octet boundary
 * - **[yy+1]-zz**: NG scaled group lengths, each of which is encoded using the number of bits
 * specified in octet 47 of data representation template 5.2. Bits set to zero shall be appended as
 * necessary to ensure this sequence of numbers ends on an octet boundary (see Note 5)
 * - **[zz+1]-nn**: Packed vaules (X2 in the decoding formula), where each value is a deviation from
 * its respective group reference value
 *
 * ## Notes
 * - (1) Group descriptors mentioned above may not be physically present; if associated field width is 0.
 * - (2) Group lengths have no meaning for row by row packing; for consistency, associated field width
 *  should then be encoded as 0. So no specific test for row case is mandatory at decoding software level
 *  to handle endcoding/decoding of group descriptors.
 * - (3) Scaled group lengths, if present, are encoded for each group. But the true last group length
 *  (unscaled) should be taken from data representation template.
 * - (4) For groups with a constant value, associated field width is 0, and no incremental data are
 *  physically present.
 * - (5) The essence of the complex packing method is to subdivide a field of values into NG groups,
 * where the values in each group have similar sizes. In this procedure, it is necessary to retain
 * enough information to recover the group lengths upon decoding. The NG group lengths for any given
 * field can be described by Ln = ref + Kn x len_inc, n = 1, NG, where ref is given by octets 38 - 41
 * and len_inc by octet 42. The NG values of K (the scaled group lengths) are stored in the data section,
 * each with the number of bits specified by octet 47. Since the last group is a special case which
 * may not be able to be specified by this relationship, the length of the last group is stored in
 * octets 43-46.
 *
 * # Data Template 7.3 - Grid Point Data - Complex Packing and Spatial Differencing
 *
 * ## Contents
 * - **6-ww**: First value(s) of original (undifferenced) scale values, followed by the overall
 * minimum of the differences. The number of values stored is 1 greater than the oerder of
 * differentiation, and the field width is described at octet 49 of data representation template
 * 5.3 (see Note 1)
 * - **[ww+1]-xx**: NG group difference values, (X1 in the decoding formula), each of which is
 * encoded using the number of bits specified in octet 20 of data representation template 5.0. Bits
 * set to zero shall be appended where necessary to ensure this sequence of numbers ends on an octet
 * boundary
 * - **[xx+1]-nn**: Packed vaules (X2 in the decoding formula), where each value is a deviation from
 * its respective group reference value
 *
 * ## Notes
 * - (1) Referring to the notation in Note 1 of data representation template 5.3, at order 1, the
 * values stored in octet 6-ww are g1 and gmin. At order 2, the values stored are h1, h2 and hmin.
 * - (2) Extra descriptors related to spatial differencing are added before the splitting descriptors,
 * to refect the separation between the two approaches. It enables to share software parts between cases
 * with and without spatial differencing.
 * - (3) The position of overall minimum after initial data values is a choice that enables less
 * software management.
 * - (4) Overall minimum will be negative in most cases. First bit should indicate the sign:0 if
 * positive, 1 if negative.
 * @param reader - Binary data reader positioned at the start of the data section.
 * @param sections - A collection of all sections in the GRIB file.
 * @returns An array of decoded values.
 */
export function complexUnpacking(reader: Reader, sections: Grib2Sections): number[] {
  // Implementation: https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/unpk_complex.c#L24
  // cleaner impl: https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/comunpack.c
  const res: number[] = [];

  // Data representation section (Template 5.3) with fields like orderOfSpatialDifference, etc.
  const { dataRepresentation: drs, bitMap: bms } = sections;
  // dataRepresentationTemplate
  if (drs === undefined || bms === undefined) return res;
  const {
    bitMapIndicator: { code: bitMapCode },
    bitMap,
  } = bms;
  // 0) Distinguish between 5.2 (no spatial differencing) and 5.3 (with differencing).
  const metadata = drs.dataRepresentation as ComplexPackingTemplate | ComplexSpatialPackingTemplate;
  const isSpatial = 'orderOfSpatialDifference' in metadata;

  // 1) Extract common fields from Template 5.2 or 5.3
  const extraValues = [0, 0];
  const numPoints = drs.numberOfDataPoints;
  const {
    referenceValue, // R
    binaryScaleFactor, // E
    decimalScaleFactor, // D
    numberOfBits, // number of bits for group references
    numberOfGroups, // NG - ngroups (31)
    referenceForGroupWidths, // ref_group_width
    groupWidthsBits, // nbit_group_width
    referenceForGroupLengths, // ref_group_length
    groupLengthFactor, // group_length_factor
    trueLengthOfLastGroup, // len_last
    nBitsGroupLength, // nbits_group_len
    groupSplittingMethod, // table_5_4
    missingValueManagement, // table_5_5
  } = metadata;
  // 2) Complex Spatial specific fields
  // table_5_6
  const orderOfSpatialDifference = isSpatial ? metadata.orderOfSpatialDifference.code : 0;
  if (isSpatial && orderOfSpatialDifference !== 1 && orderOfSpatialDifference !== 2)
    throw new Error('Only order 1 and 2 supported for spatial differencing');
  // grab extra octets
  const extraOctets = isSpatial ? metadata.extraDescriptorOctets : 0;
  // Compute scaling factors
  const factor2 = Math.pow(2, binaryScaleFactor);
  const factor10 = Math.pow(10, -decimalScaleFactor);

  // compute corrected reference value for no groups case
  const refVal = referenceValue * factor10;
  if (numberOfGroups === 0) {
    if (bitMapCode === 255) {
      for (let i = 0; i < numPoints; i++) res[i] = refVal;
      return res;
    } else if (bitMapCode === 0 || bitMapCode === 254) {
      let maskIndex = 0;
      let mask = 0;
      for (let i = 0; i < numPoints; i++) {
        if ((i & 7) === 0) mask = bitMap?.getUint8(maskIndex++) ?? 0;
        res[i] = (mask & 128) !== 0 ? refVal : 0;
        mask <<= 1;
      }
      return res;
    }
  }

  const nSubMissing = missingValueManagement.code;
  const groupRefs: number[] = new Array(numberOfGroups).fill(0);
  const groupWidths: number[] = new Array(numberOfGroups).fill(0);
  const groupLengths: number[] = new Array(numberOfGroups).fill(0);
  const groupLocation: number[] = new Array(numberOfGroups).fill(0);
  const groupClocation: number[] = new Array(numberOfGroups).fill(0);
  const groupOffset: number[] = new Array(numberOfGroups).fill(0);
  const udata: number[] = new Array(numPoints).fill(0);

  // read any extra values
  let readerCursor = 0;
  let minVal = 0;
  if (extraOctets !== 0) {
    extraValues[0] = readUintN(reader, extraOctets, readerCursor);
    readerCursor += extraOctets;
    if (orderOfSpatialDifference === 2) {
      extraValues[1] = readUintN(reader, extraOctets, readerCursor);
      readerCursor += extraOctets;
    }
    minVal = readIntN(reader, extraOctets, readerCursor);
    readerCursor += extraOctets;
  }

  if (groupSplittingMethod.code !== 1)
    throw new Error('internal decode does not support code table 5.4=' + groupSplittingMethod);

  // do a check for number of grid points and size
  let i = 0;
  let j = 0;
  let nBits = 0;
  let nBytes = 0;
  let offset = 0;
  let cLocation = 0;

  // read the group reference values in a single-threaded loop
  rdBitstream(reader, readerCursor, 0, groupRefs, numberOfBits, numberOfGroups);
  readerCursor += Math.floor((numberOfBits * numberOfGroups + 7) / 8);
  // read the group widths
  rdBitstream(reader, readerCursor, 0, groupWidths, groupWidthsBits, numberOfGroups);
  readerCursor += Math.floor((numberOfGroups * groupWidthsBits + 7) / 8);
  for (i = 0; i < numberOfGroups; i++) groupWidths[i] += referenceForGroupWidths;
  // read the group lengths if ctable_5_4 == 1
  if (groupSplittingMethod.code === 1) {
    rdBitstream(reader, readerCursor, 0, groupLengths, nBitsGroupLength, numberOfGroups - 1);
    for (i = 0; i < numberOfGroups - 1; i++) {
      groupLengths[i] = groupLengths[i] * groupLengthFactor + referenceForGroupLengths;
    }
    groupLengths[numberOfGroups - 1] = trueLengthOfLastGroup;
  }
  readerCursor += Math.floor((numberOfGroups * nBitsGroupLength + 7) / 8);

  // compute group_location, groupClocation, groupOffset, nBytes, nBits
  for (i = 0; i < numberOfGroups; i++) {
    groupLocation[i] = j;
    j += groupLengths[i];
  }
  for (i = 0; i < numberOfGroups; i++) {
    nBytes += Math.floor((groupLengths[i] * groupWidths[i]) / 8);
    nBits += Math.floor((groupLengths[i] * groupWidths[i]) % 8);
  }
  for (i = 0; i < numberOfGroups; i++) {
    groupClocation[i] = cLocation;
    cLocation +=
      Math.floor(groupLengths[i] * Math.floor(groupWidths[i] / 8)) +
      Math.floor(groupLengths[i] / 8) * (groupWidths[i] % 8);
  }
  for (i = 0; i < numberOfGroups; i++) {
    groupOffset[i] = offset;
    offset += Math.floor((groupLengths[i] % 8) * (groupWidths[i] % 8));
  }

  // check everything added up correctly
  if (j !== numPoints) throw new Error('bad complex packing: n points `${j}`');
  nBytes += Math.floor((nBits + 7) / 8);
  if (readerCursor + nBytes !== reader.byteLength)
    throw new Error('complex unpacking size mismatch old test');
  if (readerCursor + Math.floor(cLocation + (offset + 7) / 8) !== reader.byteLength)
    throw new Error('complex unpacking size mismatch');

  // read group data
  for (i = 0; i < numberOfGroups; i++) {
    groupClocation[i] += Math.floor(groupOffset[i] / 8);
    groupOffset[i] = Math.floor(groupOffset[i] % 8);
    // We want to access udata at groupLocation[i] offset
    rdBitstream(
      reader,
      readerCursor + groupClocation[i],
      groupOffset[i],
      udata,
      groupWidths[i],
      groupLengths[i],
      groupLocation[i],
    );
  }

  // handle substitute, missing values, reference value
  if (nSubMissing === 0) {
    for (i = 0; i < numberOfGroups; i++) {
      j = groupLocation[i];
      for (let k = 0; k < groupLengths[i]; k++) udata[j + k] += groupRefs[i];
    }
  } else if (nSubMissing === 1) {
    for (i = 0; i < numberOfGroups; i++) {
      j = groupLocation[i];
      if (groupWidths[i] === 0) {
        const m1 = (1 << numberOfBits) - 1;
        if (m1 === groupRefs[i]) {
          for (let k = 0; k < groupLengths[i]; k++) udata[j + k] = Number.MAX_SAFE_INTEGER;
        } else {
          for (let k = 0; k < groupLengths[i]; k++) udata[j + k] += groupRefs[i];
        }
      } else {
        const m1 = (1 << groupWidths[i]) - 1;
        for (let k = 0; k < groupLengths[i]; k++) {
          if (udata[j + k] === m1) udata[j + k] = Number.MAX_SAFE_INTEGER;
          else udata[j + k] += groupRefs[i];
        }
      }
    }
  } else if (nSubMissing === 2) {
    for (i = 0; i < numberOfGroups; i++) {
      j = groupLocation[i];
      if (groupWidths[i] === 0) {
        const m1 = (1 << numberOfBits) - 1;
        const m2 = m1 - 1;
        if (m1 === groupRefs[i] || m2 === groupRefs[i]) {
          for (let k = 0; k < groupLengths[i]; k++) udata[j + k] = Number.MAX_SAFE_INTEGER;
        } else {
          for (let k = 0; k < groupLengths[i]; k++) udata[j + k] += groupRefs[i];
        }
      } else {
        const m1 = (1 << groupWidths[i]) - 1;
        const m2 = m1 - 1;
        for (let k = 0; k < groupLengths[i]; k++) {
          if (udata[j + k] === m1 || udata[j + k] === m2) {
            udata[j + k] = Number.MAX_SAFE_INTEGER;
          } else {
            udata[j + k] += groupRefs[i];
          }
        }
      }
    }
  }

  // post processing for spatial differencing (pack == 3)
  if (isSpatial) {
    if (orderOfSpatialDifference === 1) {
      let last = extraValues[0];
      i = 0;
      while (i < numPoints) {
        if (udata[i] === Number.MAX_SAFE_INTEGER) i++;
        else {
          udata[i++] = extraValues[0];
          break;
        }
      }
      for (; i < numPoints; i++) {
        if (udata[i] !== Number.MAX_SAFE_INTEGER) {
          udata[i] += last + minVal;
          last = udata[i];
        }
      }
    } else if (orderOfSpatialDifference === 2) {
      let penultimate = extraValues[0];
      let last = extraValues[1];

      i = 0;
      while (i < numPoints) {
        if (udata[i] === Number.MAX_SAFE_INTEGER) {
          i++;
        } else {
          udata[i++] = extraValues[0];
          break;
        }
      }
      while (i < numPoints) {
        if (udata[i] === Number.MAX_SAFE_INTEGER) {
          i++;
        } else {
          udata[i++] = extraValues[1];
          break;
        }
      }
      for (; i < numPoints; i++) {
        if (udata[i] !== Number.MAX_SAFE_INTEGER) {
          udata[i] = udata[i] + minVal + last + last - penultimate;
          penultimate = last;
          last = udata[i];
        }
      }
    } else {
      throw new Error('Unsupported: code table 5.6=${metadata.orderOfSpatialDifference}');
    }
  }

  // convert to float
  if (bitMapCode === 255) {
    // no bitmap
    for (i = 0; i < numPoints; i++) {
      res[i] =
        udata[i] === Number.MAX_SAFE_INTEGER
          ? Number.MAX_SAFE_INTEGER
          : (referenceValue + udata[i] * factor2) * factor10;
    }
  } else if (bitMapCode === 0 || bitMapCode === 254) {
    // handle bitmap
    j = 0;
    let mask = 0;
    let maskIndex = 0;
    i = 0;
    while (i < numPoints) {
      if ((i & 7) === 0) mask = bitMap?.getUint8(maskIndex++) ?? 0;
      res[i++] =
        (mask & 128) !== 0
          ? (referenceValue + udata[j++] * factor2) * factor10
          : Number.MAX_SAFE_INTEGER;
      mask <<= 1;
    }
  } else {
    throw new Error('unknown bitmap: {bms.bitMapIndicator}');
  }

  return res;
}

/**
 * Converts n bytes to unsigned int
 * @param reader - reader to parse data from
 * @param n - number of bytes
 * @param offset - offset to start from
 * @returns - unsigned int of n bytes size
 */
function readUintN(reader: Reader, n: number, offset: number): number {
  let i = 0;
  while (n-- > 0) i = (i << 8) + reader.getUint8(offset++);
  return i;
}

/**
 * Converts n bytes to int
 * @param reader - reader to parse data from
 * @param n - number of bytes
 * @param offset - offset to start from
 * @returns - int of n bytes size
 */
function readIntN(reader: Reader, n: number, offset: number): number {
  if (n === 0) return 0;
  const sign = reader.getUint8(offset);
  let i = reader.getUint8(offset++) & 127;
  while (n-- > 1) i = i * 256 + reader.getUint8(offset++);
  if ((sign & 0x80) !== 0) i = -i;
  return i;
}

/**
 * Conversion of the C function `rd_bitstream`.
 * [Implementation](https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/bitstream.c#L21)
 *
 * Reads `n` unsigned integers of width `nBits` from a bitstream that starts
 * on a byte boundary. The bitstream is provided by a `Reader` object, which
 * should offer `getUint8(offset: number): number`. The resulting integers
 * are written to the `out` array.
 * @param reader - reader to parse data from
 * @param cursor - position in the reader to start
 * @param offset - 0..7 bits of offset within the first byte
 * @param out - array to store the unpacked integers
 * @param nBits - number of bits per integer
 * @param n - how many integers to unpack
 * @param outOffset - offset in the out array
 */
function rdBitstream(
  reader: Reader,
  cursor: number,
  offset: number, // 0..7 bits of offset within the first byte
  out: number[], // array to store the unpacked integers
  nBits: number, // number of bits per integer
  n: number, // how many integers to unpack
  outOffset = 0,
): void {
  // https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/bitstream.c#L21
  // Equivalent to the C "ones" array for masking lower bits:
  const ONES = [0, 1, 3, 7, 15, 31, 63, 127, 255];

  // If we assume 32-bit integers, we can’t handle nBits > 31 safely.
  if (nBits > 31) {
    throw new Error(`rd_bitstream: nBits too large (${nBits}).`);
  }
  if (offset < 0 || offset > 7) {
    throw new Error(`rd_bitstream: illegal offset ${offset}.`);
  }

  // Special case: if nBits == 0, all output is 0.
  if (nBits === 0) {
    for (let i = 0; i < n; i++) out[i + outOffset] = 0;
    return;
  }

  // We'll emulate the pointer arithmetic with a local bytePos
  // that starts at the first byte. Then (p++) becomes getUint8(bytePos++).
  let bytePos = cursor; // local "pointer" index

  // The variable `tBits` = how many bits we currently have stored in `tbits`.
  // The variable `tbits` accumulates partial data from the stream.
  let tBits = 8 - offset; // how many bits we've just read
  let tbits = reader.getUint8(bytePos++) & ONES[tBits];

  for (let i = 0; i < n; i++) {
    // Keep reading full bytes while the integer we want is still bigger
    // than the bits we currently have in `tbits`.
    while (nBits - tBits >= 8) {
      tBits += 8;
      tbits = (tbits << 8) | reader.getUint8(bytePos++);
    }

    if (nBits > tBits) {
      // We need more bits from the next byte, but fewer than 8.
      const newTBits = 8 - (nBits - tBits);
      const nextByte = reader.getUint8(bytePos);
      // The integer is the combination of the left-shifted `tbits`
      // plus the top bits of `nextByte`.
      out[i + outOffset] = (tbits << (nBits - tBits)) | (nextByte >> newTBits);

      // Now update `tBits` and `tbits`
      tBits = newTBits;
      // We consume the current byte from the stream
      tbits = reader.getUint8(bytePos++) & ONES[tBits];
    } else if (nBits === tBits) {
      // Exactly enough bits in `tbits`
      out[i + outOffset] = tbits;
      tBits = 0;
      tbits = 0;
    } else {
      // We have more bits in `tbits` than we need.
      tBits -= nBits;
      out[i + outOffset] = tbits >> tBits;
      tbits = tbits & ONES[tBits];
    }
  }
}
