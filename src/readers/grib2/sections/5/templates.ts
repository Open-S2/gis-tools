import {
  lookupTable51,
  lookupTable54,
  lookupTable540,
  lookupTable55,
  lookupTable56,
} from './tables';

import type { Reader } from '../../..';

/**
 * @description Returns a template generator for the given template number
 * @param template Template number
 * @returns Template generator
 */
export function getTemplate5(template: number) {
  switch (template) {
    case 0:
      return template50;
    case 2:
      return template52;
    case 3:
      return template53;
    case 40:
      return template540;
    default:
      throw new Error(`Template 5.${template} not defined`);
  }
}

/**
 * # Data Representation Template 5.0 - Grid point data - simple packing
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-0.shtml)
 * @param section
 *
 * ## Notes
 * - Negative values of E or D shall be represented according to Regulation [92.1.5](https://codes.ecmwf.int/grib/format/grib2/regulations/).
 * @returns - description of how to decode simple unpacked data
 */
function template50(section: Reader) {
  const originalType = section.getUint8(20);
  let binaryScaleFactor = section.getUint16(15) & 0x7fff;
  if (section.getUint16(15) >> 15 > 0) binaryScaleFactor *= -1;
  let decimalScaleFactor = section.getUint16(17) & 0x7fff;
  if (section.getUint16(17) >> 15 > 0) decimalScaleFactor *= -1;

  return {
    /** Reference value (R) (IEEE 32-bit floating-point value) */
    referenceValue: section.getFloat32(11),
    /** Binary scale factor (E) */
    binaryScaleFactor,
    /** Decimal scale factor (D) */
    decimalScaleFactor,
    /** Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    numberOfBits: section.getUint8(19),
    /** Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml)) */
    originalType: {
      code: originalType,
      description: lookupTable51[originalType],
    },
  };
}

/** Complex packing return type */
export type ComplexPackingTemplate = ReturnType<typeof template52>;

/**
 * Data Representation Template 5.2 – Complex packing (no spatial differencing).
 *
 * Reads and parses the metadata fields defined by GRIB2 Template 5.2.
 * For most templates, details of the packing process are described in Regulation 92.9.4.
 * @see {@link https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-2.shtml Template 5.2 documentation}
 * @see {@link https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-2.shtml Data template 7.2 for complementary info}
 * @param section - Binary reader providing access to the section data.
 * @returns Object containing the fields of Template 5.2.
 */
function template52(section: Reader) {
  /**
   * Binary and decimal scale factors can be negative.
   * They are stored with the sign bit in the high-order bit (bit 15).
   */
  const originalTypeCode = section.getUint8(20);

  let binaryScaleFactor = section.getUint16(15) & 0x7fff;
  if (section.getUint16(15) >> 15 > 0) binaryScaleFactor *= -1;

  let decimalScaleFactor = section.getUint16(17) & 0x7fff;
  if (section.getUint16(17) >> 15 > 0) decimalScaleFactor *= -1;
  // Fields unique to 5.2 (similar to 5.3, but no spatial differencing):
  const groupSplittingMethodCode = section.getUint8(21); // Octet 22
  const missingValueManagementCode = section.getUint8(22); // Octet 23
  const primaryMissingValueSubstitute = section.getUint32(23); // Octets 24–27
  const secondaryMissingValueSubstitute = section.getUint32(27); // Octets 28–31
  const numberOfGroups = section.getUint32(31); // Octets 32–35
  const referenceForGroupWidths = section.getUint8(35); // Octet 36
  const groupWidthsBits = section.getUint8(36); // Octet 37
  const referenceForGroupLengths = section.getUint32(37); // Octets 38–41
  const groupLengthFactor = section.getUint8(41); // Octet 42
  const trueLengthOfLastGroup = section.getUint32(42); // Octets 43–46
  const nBitsGroupLength = section.getUint8(46); // Octet 47

  return {
    /** Reference value (R) (IEEE 32-bit floating-point). Octets 12–15 in the GRIB2 documentation. */
    referenceValue: section.getFloat32(11),
    /** Binary scale factor (E). Octets 16–17. */
    binaryScaleFactor,
    /** Decimal scale factor (D). Octets 18–19. */
    decimalScaleFactor,
    /**
     * Number of bits per packed value for simple packing, or per group reference for
     * complex packing. Octet 20 in the documentation.
     */
    numberOfBits: section.getUint8(19),

    /** Type of original field values. See Code Table 5.1. Octet 21. */
    originalType: {
      code: originalTypeCode,
      description: lookupTable51[originalTypeCode],
    },

    // Fields specific to complex packing (no spatial differencing):

    /** Group splitting method used. See Code Table 5.4. Octet 22. */
    groupSplittingMethod: {
      code: groupSplittingMethodCode,
      description: lookupTable54[groupSplittingMethodCode],
    },
    /** Missing value management. See Code Table 5.5. Octet 23. */
    missingValueManagement: {
      code: missingValueManagementCode,
      description: lookupTable55[missingValueManagementCode],
    },
    /** Primary missing value substitute. Octets 24–27. */
    primaryMissingValueSubstitute,
    /** Secondary missing value substitute. Octets 28–31. */
    secondaryMissingValueSubstitute,

    /** Number of groups of data values (NG). Octets 32–35. */
    numberOfGroups,
    /**
     * Reference for group widths. Octet 36.
     * The group width is the number of bits used for every value in a group.
     */
    referenceForGroupWidths,
    /** Number of bits used for the group widths (after subtracting the reference value). Octet 37. */
    groupWidthsBits,
    /**
     * Reference for group lengths. Octets 38–41.
     * The group length (L) is the number of values in a group.
     */
    referenceForGroupLengths,
    /**
     * Length increment for group lengths. Octet 42.
     * Used in the formula: Lₙ = ref + Kₙ × len_inc.
     */
    groupLengthFactor,
    /**
     * True length of the last group. Octets 43–46.
     * A special-case group length if the sequence doesn’t fit the formula.
     */
    trueLengthOfLastGroup,
    /**
     * Number of bits used for scaled group lengths (after subtracting ref
     * and dividing by the length increment). Octet 47.
     */
    nBitsGroupLength,
  };
}

/** Complex packing and spatial differencing return type */
export type ComplexSpatialPackingTemplate = ReturnType<typeof template53>;

/**
 * Data Representation Template 5.3 – Complex packing and spatial differencing.
 *
 * Reads and parses the metadata fields defined by GRIB2 Template 5.3.
 * For most templates, details of the packing process are described in Regulation 92.9.4.
 * See also:
 * - [GRIB2 Template 5.3 documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml)
 * - [Data template 7.3 and associated notes](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-3.shtml)
 * - Spatial differencing (Regulation 92.9.4).
 * @param section - Binary reader providing access to the section data
 * @returns Object containing the fields of Template 5.3
 */
function template53(section: Reader) {
  /**
   * Binary and decimal scale factors can be negative.
   * They are stored with the sign bit in the high-order bit (bit 15).
   */
  const originalTypeCode = section.getUint8(20);
  let binaryScaleFactor = section.getUint16(15) & 0x7fff;
  if (section.getUint16(15) >> 15 > 0) binaryScaleFactor *= -1;
  let decimalScaleFactor = section.getUint16(17) & 0x7fff;
  if (section.getUint16(17) >> 15 > 0) decimalScaleFactor *= -1;
  // New fields introduced by Template 5.3
  const groupSplittingMethodCode = section.getUint8(21);
  const missingValueManagementCode = section.getUint8(22);
  const primaryMissingValueSubstitute = section.getUint32(23);
  const secondaryMissingValueSubstitute = section.getUint32(27);
  const numberOfGroups = section.getUint32(31);
  const referenceForGroupWidths = section.getUint8(35);
  const groupWidthsBits = section.getUint8(36);
  const referenceForGroupLengths = section.getUint32(37);
  const groupLengthFactor = section.getUint8(41);
  const trueLengthOfLastGroup = section.getUint32(42);
  const nBitsGroupLength = section.getUint8(46);
  const orderOfSpatialDifferenceCode = section.getUint8(47);
  const extraDescriptorOctets = section.getUint8(48);

  return {
    /**
     * Reference value (R) (IEEE 32-bit floating-point).
     * Octets 12–15 in the GRIB2 documentation.
     */
    referenceValue: section.getFloat32(11),
    /** Binary scale factor (E). Octets 16–17. */
    binaryScaleFactor,
    /** Decimal scale factor (D). Octets 18–19. */
    decimalScaleFactor,
    /**
     * Number of bits per packed value for simple packing,
     * or per group reference for complex packing.
     * Octet 20 in the documentation.
     */
    numberOfBits: section.getUint8(19),
    /** Type of original field values. See Code Table 5.1. Octet 21. */
    originalType: {
      code: originalTypeCode,
      description: lookupTable51[originalTypeCode],
    },

    // Fields specific to complex packing and spatial differencing:

    /** Group splitting method used. See Code Table 5.4. Octet 22. */
    groupSplittingMethod: {
      code: groupSplittingMethodCode,
      description: lookupTable54[groupSplittingMethodCode],
    },
    /** Missing value management. See Code Table 5.5. Octet 23. */
    missingValueManagement: {
      code: missingValueManagementCode,
      description: lookupTable55[missingValueManagementCode],
    },
    /** Primary missing value substitute. Octets 24–27. */
    primaryMissingValueSubstitute,
    /** Secondary missing value substitute. Octets 28–31. */
    secondaryMissingValueSubstitute,
    /** Number of groups of data values (NG). Octets 32–35. */
    numberOfGroups,
    /**
     * Reference for group widths. Octet 36.
     * The group width is the number of bits used for every value in a group.
     */
    referenceForGroupWidths,
    /**
     * Number of bits used for the group widths (after subtracting the reference value).
     * Octet 37.
     */
    groupWidthsBits,
    /**
     * Reference for group lengths. Octets 38–41.
     * The group length (L) is the number of values in a group.
     */
    referenceForGroupLengths,
    /**
     * Length increment for group lengths. Octet 42.
     * Used in the formula: Lₙ = ref + Kₙ × len_inc.
     */
    groupLengthFactor,
    /**
     * True length of the last group. Octets 43–46.
     * A special-case group length if the sequence doesn’t fit the formula.
     */
    trueLengthOfLastGroup,
    /**
     * Number of bits used for scaled group lengths (after subtracting ref
     * and dividing by the length increment). Octet 47.
     */
    nBitsGroupLength,
    /** Order of spatial difference. See Code Table 5.6. Octet 48. */
    orderOfSpatialDifference: {
      code: orderOfSpatialDifferenceCode,
      description: lookupTable56[orderOfSpatialDifferenceCode],
    },
    /**
     * Number of extra descriptor octets needed for spatial differencing
     * (octets 6–ww in data template 7.3). Octet 49.
     */
    extraDescriptorOctets,
  };
}

/**
 * Data Representation Template 5.40
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-40.shtml)
 * @param section - The raw section data to parse
 * @returns - Parsed Data Representation Information
 */
function template540(section: Reader) {
  const originalTypeCode = section.getUint8(20);
  const compressionType = section.getUint8(21);
  return {
    /** Reference value (R) (IEEE 32-bit floating-point value) */
    referenceValue: section.getFloat32(11),
    /** Binary scale factor (E) */
    binaryScaleFactor: section.getInt16(15),
    /** Decimal scale factor (D) */
    decimalScaleFactor: section.getInt16(17),
    /** Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    numberOfBits: section.getUint8(19),
    /** Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml)) */
    originalType: {
      code: originalTypeCode,
      description: lookupTable51[originalTypeCode],
    },
    /** Type of Compression used. (see [Code Table 5.40](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-40.shtml)) */
    compressionType: {
      code: compressionType,
      description: lookupTable540[compressionType],
    },
    /**
     * Target compression ratio, M:1 (with respect to the bit-depth specified in octet 20),
     * when octet 22 indicates Lossy Compression. Otherwise, set to missing.
     */
    compressionRatio: section.getUint8(22),
  };
}
