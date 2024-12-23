/**
 * # GRIB2 - TABLE 6.0 - BIT MAP INDICATOR
 *
 * **Details**:
 * - **Section**: 6
 * - **Octet**: 6
 * - **Revised**: 05/17/2005
 *
 * **Value Ranges**:
 * - `1-253`: A bit map pre-determined by the originating/generating center applies to this product and is not specified in this section.
 *
 * **Special Value**:
 * - `255`: A bit map does not apply to this product.
 *
 * ## Description
 * This table defines the bit map indicators used in GRIB2 files,
 * specifying how bit maps apply to products based on various definitions.
 *
 * ## Notes
 * - Revised 05/17/2005
 */
export const lookupTable60: Record<number, string> = {
  0: 'A bit map applies to this product and is specified in this section.',
  // 1-253: A bit map pre-determined by the originating/generating center applies to this product and is not specified in this section.
  254: 'A bit map previously defined in the same GRIB2 message applies to this product.',
  255: 'A bit map does not apply to this product.',
};
