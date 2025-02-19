import {
  grib2LookupTable51,
  grib2LookupTable54,
  grib2LookupTable540,
  grib2LookupTable55,
  grib2LookupTable56,
  grib2LookupTable57,
} from './tables';

import type { Reader } from '../../..';

/**
 * Returns a template generator for the given template number
 * @param template Template number
 * @returns Template generator
 */
export function getGrib2Template5(template: number) {
  switch (template) {
    case 0:
      return grib2Template50;
    case 2:
      return grib2Template52;
    case 3:
      return grib2Template53;
    case 40:
      return grib2Template540;
    case 50:
      return grib2Template550;
    case 51:
      return grib2Template551;
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
export function grib2Template50(section: Reader) {
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
      description: grib2LookupTable51[originalType],
    },
  };
}

/** Complex packing return type */
export type ComplexPackingTemplate = ReturnType<typeof grib2Template52>;

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
export function grib2Template52(section: Reader) {
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
      description: grib2LookupTable51[originalTypeCode],
    },

    // Fields specific to complex packing (no spatial differencing):

    /** Group splitting method used. See Code Table 5.4. Octet 22. */
    groupSplittingMethod: {
      code: groupSplittingMethodCode,
      description: grib2LookupTable54[groupSplittingMethodCode],
    },
    /** Missing value management. See Code Table 5.5. Octet 23. */
    missingValueManagement: {
      code: missingValueManagementCode,
      description: grib2LookupTable55[missingValueManagementCode],
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
export type ComplexSpatialPackingTemplate = ReturnType<typeof grib2Template53>;

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
export function grib2Template53(section: Reader) {
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
      description: grib2LookupTable51[originalTypeCode],
    },

    // Fields specific to complex packing and spatial differencing:

    /** Group splitting method used. See Code Table 5.4. Octet 22. */
    groupSplittingMethod: {
      code: groupSplittingMethodCode,
      description: grib2LookupTable54[groupSplittingMethodCode],
    },
    /** Missing value management. See Code Table 5.5. Octet 23. */
    missingValueManagement: {
      code: missingValueManagementCode,
      description: grib2LookupTable55[missingValueManagementCode],
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
      description: grib2LookupTable56[orderOfSpatialDifferenceCode],
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
export function grib2Template540(section: Reader) {
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
      description: grib2LookupTable51[originalTypeCode],
    },
    /** Type of Compression used. (see [Code Table 5.40](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-40.shtml)) */
    compressionType: {
      code: compressionType,
      description: grib2LookupTable540[compressionType],
    },
    /**
     * Target compression ratio, M:1 (with respect to the bit-depth specified in octet 20),
     * when octet 22 indicates Lossy Compression. Otherwise, set to missing.
     */
    compressionRatio: section.getUint8(22),
  };
}

/**
 * # Data Representation Template 5.50 - Spectral data - simple packing
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-50.shtml)
 * @param section
 *
 * ## Notes
 * - Removal of the real part of (0.0) coefficient from packed data is intended to reduce the
 * variability of the coefficients, in order to improve packing accuracy.
 * - For some spectral representations, the (0.0) coefficient represents the mean value of the
 * parameter represented.
 * - Negative values of E or D shall be represented according to Regulation [92.1.5](https://codes.ecmwf.int/grib/format/grib2/regulations/).
 * @returns - description of how to decode simple unpacked data
 */
export function grib2Template550(section: Reader) {
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
    realPartCoefficientType: section.getFloat32(20),
  };
}

/**
 * # Data Representation Template 5.51 - Spectral data - complex packing
 *
 * [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-51.shtml)
 * @param section
 *
 * ## Notes
 * - The unpacked subset is a set of values defined in the same way as the full set of values
 * (on a spectrum limited to Js, Ks and Ms ), but on which scaling and packing are not applied.
 * Associated values are stored in octets 6 onwards of section 7.
 * - The remaining coefficients are multiplied by `(n x (n+1))p` , scaled and packed. The operator
 * associated with this multiplication is derived from the Laplacian operator on the sphere.
 * - The retrieval formula for a coefficient of wave number n is then: `Y = (R+X x 2e ) x 10-d x (n x(n+1))-p`
 * where X is the packed scaled value associated with the coefficient.
 * @returns - description of how to decode simple unpacked data
 */
export function grib2Template551(section: Reader) {
  let binaryScaleFactor = section.getUint16(15) & 0x7fff;
  if (section.getUint16(15) >> 15 > 0) binaryScaleFactor *= -1;
  let decimalScaleFactor = section.getUint16(17) & 0x7fff;
  if (section.getUint16(17) >> 15 > 0) decimalScaleFactor *= -1;
  const precisionCode = section.getUint8(34);

  return {
    /** Reference value (R) (IEEE 32-bit floating-point value) */
    referenceValue: section.getFloat32(11),
    /** Binary scale factor (E) */
    binaryScaleFactor,
    /** Decimal scale factor (D) */
    decimalScaleFactor,
    /** Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    numberOfBits: section.getUint8(19),
    /** P ― Laplacian scaling factor (expressed in 10^-6 units) */
    P: section.getFloat32(20),
    /** Js ― pentagonal resolution parameter of the unpacked subset (see Note1) */
    Js: section.getInt16(24),
    /** Ks ― pentagonal resolution parameter of the unpacked subset (see Note1) */
    Ks: section.getInt16(26),
    /** Ms ― pentagonal resolution parameter of the unpacked subset (see Note1) */
    Ms: section.getInt16(28),
    /** Ts ― total number of values in the unpacked subset (see Note1) */
    Ts: section.getInt32(30),
    /** Precision of the unpacked subset (see Code Table 5.7) */
    precision: {
      code: precisionCode,
      description: grib2LookupTable57[precisionCode],
    },
  };
}

/** Complex packing and spatial differencing return type */
export type ComplexSpectralPackingTemplate = ReturnType<typeof grib2Template551>;
