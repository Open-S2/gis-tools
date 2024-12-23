import type { Reader } from '../../..';
import type {
  ComplexPackingTemplate,
  ComplexSpatialPackingTemplate,
  DataRepresentationSection,
} from '../5';

/**
 * # Data Template 7.2 - Grid Point Data - Complex Packing
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
 * @param _reader - Binary data reader positioned at the start of the data section.
 * @param drs - Data representation section (Template 5.3) with fields like orderOfSpatialDifference, etc.
 * @returns An array of decoded values.
 */
export function complexUnpacking(_reader: Reader, drs: DataRepresentationSection): number[] {
  // Implementation: https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/unpk_complex.c#L24
  // TODO:
  const _dataRepresentation = drs.dataRepresentation as
    | ComplexPackingTemplate
    | ComplexSpatialPackingTemplate;
  return [];
}
