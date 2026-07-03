use crate::{
    parsers::{BufferReader, Reader},
    readers::{Grib2Sections, Grib2Template5},
};
use libm::pow;

/// # Data Template 7.2 - Grid Point Data - Complex Packing
///
/// ## Contents
/// - **6-xx**: NG group reference values (X1 in the decoding formula), each of which is encoded using
///   the number of bits specified in octet 20 of data representation template 5.0. Bits set to zero shall
///   be appended as necessary to ensure this sequence of numbers ends on an octet boundary
/// - **[xx+1]-yy**: NG group widths, each of which is encoded using the number of bits specified in
///   octet 37 of data representation template 5.2. Bits set to zero shall be appended as necessary to
///   ensure this sequence of numbers ends on an octet boundary
/// - **[yy+1]-zz**: NG scaled group lengths, each of which is encoded using the number of bits
///   specified in octet 47 of data representation template 5.2. Bits set to zero shall be appended as
///   necessary to ensure this sequence of numbers ends on an octet boundary (see Note 5)
/// - **[zz+1]-nn**: Packed vaules (X2 in the decoding formula), where each value is a deviation from
///   its respective group reference value
///
/// ## Notes
/// - (1) Group descriptors mentioned above may not be physically present; if associated field width is 0.
/// - (2) Group lengths have no meaning for row by row packing; for consistency, associated field width
///   should then be encoded as 0. So no specific test for row case is mandatory at decoding software level
///   to handle endcoding/decoding of group descriptors.
/// - (3) Scaled group lengths, if present, are encoded for each group. But the true last group length
///   (unscaled) should be taken from data representation template.
/// - (4) For groups with a constant value, associated field width is 0, and no incremental data are
///   physically present.
/// - (5) The essence of the complex packing method is to subdivide a field of values into NG groups,
///   where the values in each group have similar sizes. In this procedure, it is necessary to retain
///   enough information to recover the group lengths upon decoding. The NG group lengths for any given
///   field can be described by Ln = ref + Kn x len_inc, n = 1, NG, where ref is given by octets 38 - 41
///   and len_inc by octet 42. The NG values of K (the scaled group lengths) are stored in the data section,
///   each with the number of bits specified by octet 47. Since the last group is a special case which
///   may not be able to be specified by this relationship, the length of the last group is stored in
///   octets 43-46.
///
/// # Data Template 7.3 - Grid Point Data - Complex Packing and Spatial Differencing
///
/// ## Contents
/// - **6-ww**: First value(s) of original (undifferenced) scale values, followed by the overall
///   minimum of the differences. The number of values stored is 1 greater than the oerder of
///   differentiation, and the field width is described at octet 49 of data representation template
///   5.3 (see Note 1)
/// - **[ww+1]-xx**: NG group difference values, (X1 in the decoding formula), each of which is
///   encoded using the number of bits specified in octet 20 of data representation template 5.0. Bits
///   set to zero shall be appended where necessary to ensure this sequence of numbers ends on an octet
///   boundary
/// - **[xx+1]-nn**: Packed vaules (X2 in the decoding formula), where each value is a deviation from
///   its respective group reference value
///
/// ## Notes
/// - (1) Referring to the notation in Note 1 of data representation template 5.3, at order 1, the
///   values stored in octet 6-ww are g1 and gmin. At order 2, the values stored are h1, h2 and hmin.
/// - (2) Extra descriptors related to spatial differencing are added before the splitting descriptors,
///   to refect the separation between the two approaches. It enables to share software parts between cases
///   with and without spatial differencing.
/// - (3) The position of overall minimum after initial data values is a choice that enables less
///   software management.
/// - (4) Overall minimum will be negative in most cases. First bit should indicate the sign:0 if
///   positive, 1 if negative.
///
/// ## Parameters
/// - `reader`: Binary data reader positioned at the start of the data section.
/// - `sections`: A collection of all sections in the GRIB file.
///
/// ## Returns
/// An array of decoded values.
pub fn complex_unpacking(reader: &BufferReader, sections: &Grib2Sections) -> Vec<f64> {
    // Implementation: https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/unpk_complex.c#L24
    // cleaner impl: https://github.com/NOAA-EMC/NCEPLIBS-g2c/blob/develop/src/comunpack.c

    // Data representation section (Template 5.3) with fields like order_of_spatial_difference, etc.
    let drs = sections.data_representation.as_ref().unwrap_or_else(|| {
        panic!("Data Representation Section is not defined");
    });
    let bms = sections.bit_map.as_ref().unwrap_or_else(|| {
        panic!("Bit Map Section is not defined");
    });
    let bit_map_indicator = bms.bit_map_indicator;
    let bit_map_code: u8 = bit_map_indicator.into();
    let bit_map = &bms.bit_map;
    // 0) Distinguish between 5.2 (no spatial differencing) and 5.3 (with differencing).
    let metadata = &drs.data_representation;
    let is_spatial = matches!(metadata, Grib2Template5::Grib2Template53(_));

    // 1) Extract common fields from Template 5.2 or 5.3
    let mut extra_values: (i32, i32) = (0, 0);
    let num_points = drs.number_of_data_points as usize;
    let mut res: Vec<f64> = vec![0.0; num_points];
    let reference_value = metadata.reference_value(); // R
    let binary_scale_factor = metadata.binary_scale_factor(); // E
    let decimal_scale_factor = metadata.decimal_scale_factor(); // D
    let number_of_bits = metadata.number_of_bits() as usize; // number of bits for group references
    // NG - ngroups (31)
    let number_of_groups = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.number_of_groups as usize
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.number_of_groups as usize
    } else {
        0
    };
    // ref_group_width
    let reference_for_group_widths = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.reference_for_group_widths
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.reference_for_group_widths
    } else {
        0
    };
    // nbit_group_width
    let group_widths_bits = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.group_widths_bits
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.group_widths_bits
    } else {
        0
    };
    // ref_group_length
    let reference_for_group_lengths = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.reference_for_group_lengths
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.reference_for_group_lengths
    } else {
        0
    };
    // group_length_factor
    let group_length_factor = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.group_length_factor
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.group_length_factor
    } else {
        0
    };
    // len_last
    let true_length_of_last_group = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.true_length_of_last_group
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.true_length_of_last_group
    } else {
        0
    };
    // n_bits_group_len
    let n_bits_group_len = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.n_bits_group_length
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.n_bits_group_length
    } else {
        0
    };
    // table_5_4
    let group_splitting_method = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.group_splitting_method as u8
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.group_splitting_method as u8
    } else {
        0
    };
    // table_5_5
    let missing_value_management = if let Grib2Template5::Grib2Template52(m) = metadata {
        m.missing_value_management as u8
    } else if let Grib2Template5::Grib2Template53(m) = metadata {
        m.missing_value_management as u8
    } else {
        0
    };

    // 2) Complex Spatial specific fields
    // table_5_6
    let order_of_spatial_difference = if let Grib2Template5::Grib2Template53(m) = metadata {
        m.order_of_spatial_difference as u8
    } else {
        0
    };
    if is_spatial && order_of_spatial_difference != 1 && order_of_spatial_difference != 2 {
        panic!("Only order 1 and 2 supported for spatial differencing");
    }
    // grab extra octets
    let extra_octets = if let Grib2Template5::Grib2Template53(m) = metadata {
        m.extra_descriptor_octets
    } else {
        0
    };
    // Compute scaling factors
    let factor2 = pow(2., binary_scale_factor);
    let factor10 = pow(10., -decimal_scale_factor);

    // compute corrected reference value for no groups case
    let ref_val = reference_value * factor10;
    if number_of_groups == 0 {
        if bit_map_code == 255 {
            for res_item in res.iter_mut().take(num_points) {
                *res_item = ref_val;
            }
            return res;
        } else if bit_map_code == 0 || bit_map_code == 254 {
            let mut mask_index = 0;
            let mut mask = 0;
            let bit_map_ref = bit_map.as_ref().expect("Expected a bitmap");
            for (i, res_item) in res.iter_mut().enumerate().take(num_points) {
                if (i & 7) == 0 {
                    mask = bit_map_ref.uint8(Some(mask_index as u64));
                    mask_index += 1;
                }
                *res_item = if (mask & 128) != 0 { ref_val } else { 0. };
                mask <<= 1;
            }
            return res;
        }
    }

    let n_sub_missing = missing_value_management;
    let mut group_refs = vec![0_i32; number_of_groups];
    let mut group_widths = vec![0_i32; number_of_groups];
    let mut group_lengths = vec![0_i32; number_of_groups];
    let mut group_location = vec![0_u32; number_of_groups];
    let mut group_c_location = vec![0_u32; number_of_groups];
    let mut group_offset = vec![0_i32; number_of_groups];
    let mut udata = vec![0_i32; num_points];

    // read any extra values
    let mut reader_cursor = 0;
    let mut min_val: i32 = 0;
    if extra_octets != 0 {
        extra_values.0 = read_uint_n(reader, extra_octets as usize, reader_cursor) as i32;
        reader_cursor += extra_octets as u64;
        if order_of_spatial_difference == 2 {
            extra_values.1 = read_uint_n(reader, extra_octets as usize, reader_cursor) as i32;
            reader_cursor += extra_octets as u64;
        }
        min_val = read_int_n(reader, extra_octets as usize, reader_cursor) as i32;
        reader_cursor += extra_octets as u64;
    }

    if group_splitting_method != 1 {
        panic!("internal decode does not support code table 5.4={group_splitting_method}");
    }

    // do a check for number of grid points and size
    let mut i;
    let mut j = 0;
    let mut n_bits = 0;
    let mut n_bytes = 0;
    let mut offset = 0;
    let mut c_location = 0;

    // read the group reference values in a single-threaded loop
    rd_bitstream(
        reader,
        reader_cursor,
        0,
        &mut group_refs,
        number_of_bits as i32,
        number_of_groups,
        None,
    );
    reader_cursor += (number_of_groups * number_of_bits).div_ceil(8) as u64;
    // read the group widths
    rd_bitstream(
        reader,
        reader_cursor,
        0,
        &mut group_widths,
        group_widths_bits as i32,
        number_of_groups,
        None,
    );
    reader_cursor += (number_of_groups * (group_widths_bits as usize)).div_ceil(8) as u64;
    for group_width in group_widths.iter_mut().take(number_of_groups) {
        *group_width += reference_for_group_widths as i32;
    }
    // read the group lengths if ctable_5_4 == 1
    if group_splitting_method == 1 {
        rd_bitstream(
            reader,
            reader_cursor,
            0,
            &mut group_lengths,
            n_bits_group_len as i32,
            number_of_groups - 1,
            None,
        );
        for group_length in group_lengths.iter_mut().take(number_of_groups - 1) {
            *group_length =
                *group_length * (group_length_factor as i32) + (reference_for_group_lengths as i32);
        }
        group_lengths[number_of_groups - 1] = true_length_of_last_group as i32;
    }
    reader_cursor += (number_of_groups * (n_bits_group_len as usize)).div_ceil(8) as u64;

    // compute group_location, group_c_location, group_offset, n_bytes, n_bits
    for i in 0..number_of_groups {
        group_location[i] = j;
        j += group_lengths[i] as u32;
    }
    for i in 0..number_of_groups {
        n_bytes += (group_lengths[i] * group_widths[i]) / 8;
        n_bits += (group_lengths[i] * group_widths[i]) % 8;
    }
    for i in 0..number_of_groups {
        group_c_location[i] = c_location;
        let term1 = group_lengths[i] * (group_widths[i] / 8);
        let term2 = (group_lengths[i] / 8) * (group_widths[i] % 8);
        c_location += (term1 + term2) as u32;
    }
    for i in 0..number_of_groups {
        group_offset[i] = offset;
        offset += (group_lengths[i] % 8) * (group_widths[i] % 8);
    }

    // check everything added up correctly
    if j != num_points as u32 {
        panic!("bad complex packing: n points `{j}`");
    }
    n_bytes += (n_bits + 7) / 8;
    if reader_cursor + (n_bytes as u64) != reader.len() {
        panic!("complex unpacking size mismatch old test");
    }
    if reader_cursor + ((c_location as i32) + (offset + 7) / 8) as u64 != reader.len() {
        panic!("complex unpacking size mismatch");
    }

    // read group data
    for i in 0..number_of_groups {
        group_c_location[i] += (group_offset[i] / 8) as u32;
        group_offset[i] %= 8;
        // We want to access udata at group_location[i] offset
        rd_bitstream(
            reader,
            reader_cursor + group_c_location[i] as u64,
            group_offset[i],
            &mut udata,
            group_widths[i],
            group_lengths[i] as usize,
            Some(group_location[i] as usize),
        );
    }

    // handle substitute, missing values, reference value
    if n_sub_missing == 0 {
        for i in 0..number_of_groups {
            j = group_location[i];
            for k in 0..group_lengths[i] {
                udata[(j + k as u32) as usize] += group_refs[i];
            }
        }
    } else if n_sub_missing == 1 {
        for i in 0..number_of_groups {
            j = group_location[i];
            if group_widths[i] == 0 {
                let m1 = (1 << number_of_bits) - 1;
                if m1 == group_refs[i] {
                    for k in 0..(group_lengths[i] as u32) {
                        udata[(j + k) as usize] = i32::MAX;
                    }
                } else {
                    for k in 0..group_lengths[i] as u32 {
                        udata[(j + k) as usize] += group_refs[i];
                    }
                }
            } else {
                let m1 = (1 << group_widths[i]) - 1;
                for k in 0..group_lengths[i] as u32 {
                    if udata[(j + k) as usize] == m1 {
                        udata[(j + k) as usize] = i32::MAX;
                    } else {
                        udata[(j + k) as usize] += group_refs[i];
                    }
                }
            }
        }
    } else if n_sub_missing == 2 {
        for i in 0..number_of_groups {
            j = group_location[i];
            if group_widths[i] == 0 {
                let m1 = (1 << number_of_bits) - 1;
                let m2 = m1 - 1;
                if m1 == group_refs[i] || m2 == group_refs[i] {
                    for k in 0..group_lengths[i] as u32 {
                        udata[(j + k) as usize] = i32::MAX;
                    }
                } else {
                    for k in 0..group_lengths[i] as u32 {
                        udata[(j + k) as usize] += group_refs[i];
                    }
                }
            } else {
                let m1 = (1 << group_widths[i]) - 1;
                let m2 = m1 - 1;
                for k in 0..group_lengths[i] as u32 {
                    if udata[(j + k) as usize] == m1 || udata[(j + k) as usize] == m2 {
                        udata[(j + k) as usize] = i32::MAX;
                    } else {
                        udata[(j + k) as usize] += group_refs[i];
                    }
                }
            }
        }
    }

    // post processing for spatial differencing (pack == 3)
    if is_spatial {
        if order_of_spatial_difference == 1 {
            let mut last = extra_values.0;
            i = 0;
            while i < num_points {
                if udata[i] == i32::MAX {
                    i += 1;
                } else {
                    udata[i] = extra_values.0;
                    i += 1;
                    break;
                }
            }
            while i < num_points {
                if udata[i] != i32::MAX {
                    udata[i] += last + min_val;
                    last = udata[i];
                }
                i += 1;
            }
        } else if order_of_spatial_difference == 2 {
            let mut penultimate = extra_values.0;
            let mut last = extra_values.1;

            i = 0;
            while i < num_points {
                if udata[i] == i32::MAX {
                    i += 1;
                } else {
                    udata[i] = extra_values.1;
                    i += 1;
                    break;
                }
            }
            while i < num_points {
                if udata[i] == i32::MAX {
                    i += 1;
                } else {
                    udata[i] = extra_values.1;
                    i += 1;
                    break;
                }
            }
            while i < num_points {
                if udata[i] != i32::MAX {
                    udata[i] = udata[i] + min_val + last + last - penultimate;
                    penultimate = last;
                    last = udata[i];
                }
                i += 1;
            }
        } else {
            panic!("Unsupported: code table 5.6={order_of_spatial_difference}");
        }
    }

    // convert to float
    if bit_map_code == 255 {
        // no bitmap
        for i in 0..num_points {
            res[i] = if udata[i] == i32::MAX {
                f64::NAN
            } else {
                (reference_value + (udata[i] as f64) * factor2) * factor10
            };
        }
    } else if bit_map_code == 0 || bit_map_code == 254 {
        // handle bitmap
        j = 0;
        let mut mask = 0;
        let mut mask_index = 0;
        i = 0;
        let bit_map_ref = bit_map.as_ref().expect("a bitmap must exist");
        while i < num_points {
            if (i & 7) == 0 {
                mask = bit_map_ref.uint8(Some(mask_index as u64));
                mask_index += 1;
            }
            res[i] = if (mask & 128) != 0 {
                (reference_value + (udata[j as usize] as f64) * factor2) * factor10
            } else {
                f64::NAN
            };
            i += 1;
            j += 1;
            mask <<= 1;
        }
    } else {
        panic!("unknown bitmap: {bit_map_indicator:?}");
    }

    res
}

/// Converts n bytes to unsigned int
///
/// ## Parameters
/// - `reader`: reader to parse data from
/// - `n`: number of bytes
/// - `offset`: offset to start from
///
/// ## Returns
/// unsigned int of n bytes size
fn read_uint_n(reader: &BufferReader, n: usize, mut offset: u64) -> u64 {
    let mut val: u64 = 0;

    for _ in 0..n {
        // Use | for bitwise combination and u64 for the accumulator
        val = (val << 8) | (reader.uint8(Some(offset)) as u64);
        offset += 1;
    }

    val
}

/// Converts n bytes to int
///
/// ## Parameters
/// - `reader`: reader to parse data from
/// - `n`: number of bytes
/// - `offset`: offset to start from
///
/// ## Returns
/// int of n bytes size
fn read_int_n(reader: &BufferReader, n: usize, mut offset: u64) -> i64 {
    if n == 0 {
        return 0;
    }

    let first_byte = reader.uint8(Some(offset));
    // Start with the lower 7 bits of the first byte
    let mut val = (first_byte & 0x7F) as i64;
    offset += 1;

    // We already read 1 byte, so we need to read n-1 more bytes
    for _ in 1..n {
        val = (val << 8) | (reader.uint8(Some(offset)) as i64);
        offset += 1;
    }
    if (first_byte & 0x80) != 0 {
        val = -val;
    }

    val
}

/// Conversion of the C function `rd_bitstream`.
/// [Implementation](https://github.com/NOAA-EMC/wgrib2/blob/a9a04f0e81ff1630b41ebf55ae77ec79474c1845/wgrib2/bitstream.c#L21)
///
/// Reads `n` unsigned integers of width `n_bits` from a bitstream that starts
/// on a byte boundary. The bitstream is provided by a `Reader` object, which
/// should offer `getUint8(offset: number): number`. The resulting integers
/// are written to the `out` array.
///
/// ## Parameters
/// - `reader`: reader to parse data from
/// - `cursor`: position in the reader to start
/// - `offset`: 0..7 bits of offset within the first byte
/// - `out`: array to store the unpacked integers
/// - `n_bits`: number of bits per integer
/// - `n`: how many integers to unpack
/// - `out_offset`: offset in the out array
fn rd_bitstream(
    reader: &BufferReader,
    cursor: u64,
    offset: i32,        // 0..7 bits of offset within the first byte
    out: &mut Vec<i32>, // array to store the unpacked integers
    n_bits: i32,        // number of bits per integer
    n: usize,           // how many integers to unpack
    out_offset: Option<usize>,
) {
    let out_offset = out_offset.unwrap_or(0);
    let ones = [0, 1, 3, 7, 15, 31, 63, 127, 255];

    if n_bits > 31 {
        panic!("rd_bitstream: n_bits too large ({n_bits}).");
    }
    if !(0..=7).contains(&offset) {
        panic!("rd_bitstream: illegal offset {offset}.");
    }

    // Ensure the output vector is large enough before starting
    if out.len() < n + out_offset {
        out.resize(n + out_offset, 0);
    }

    if n_bits == 0 {
        for i in 0..n {
            out[i + out_offset] = 0;
        }
        return;
    }

    let mut byte_pos = cursor;
    let mut t_bits = 8 - offset;

    // Explicitly cast tbits to i32 to prevent shift overflow
    let mut tbits = (reader.uint8(Some(byte_pos)) & ones[t_bits as usize]) as i32;
    byte_pos += 1;

    for i in 0..n {
        // Accumulate full bytes
        while n_bits - t_bits >= 8 {
            tbits = (tbits << 8) | (reader.uint8(Some(byte_pos)) as i32);
            t_bits += 8;
            byte_pos += 1;
        }

        if n_bits > t_bits {
            let n_bits_needed = n_bits - t_bits;
            let new_t_bits = 8 - n_bits_needed;
            let next_byte = reader.uint8(Some(byte_pos)) as i32;

            out[i + out_offset] = (tbits << n_bits_needed) | (next_byte >> new_t_bits);

            t_bits = new_t_bits;
            tbits = next_byte & (ones[t_bits as usize] as i32);
            byte_pos += 1;
        } else if n_bits == t_bits {
            out[i + out_offset] = tbits;
            t_bits = 0;
            tbits = 0;
        } else {
            // We have more bits than needed
            t_bits -= n_bits;
            out[i + out_offset] = tbits >> t_bits;
            tbits &= ones[t_bits as usize] as i32;
        }
    }
}
