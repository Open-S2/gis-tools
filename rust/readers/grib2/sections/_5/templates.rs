use crate::{
    parsers::Reader,
    readers::{
        Grib2Table5_1, Grib2Table5_4, Grib2Table5_5, Grib2Table5_6, Grib2Table5_7, Grib2Table5_40,
    },
};

/// Returns a template generator for the given template number
///
/// @param template Template number
/// @returns Template generator
#[derive(Debug, Clone, PartialEq)]
pub enum Grib2Template5 {
    /// Data Representation Template 5.0 - Grid point data - simple packing
    Grib2Template50(Grib2Template50),
    /// Data Representation Template 5.2 – Complex packing (no spatial differencing).
    Grib2Template52(Grib2Template52),
    /// Data Representation Template 5.3 – Complex packing and spatial differencing.
    Grib2Template53(Grib2Template53),
    /// Data Representation Template 5.40
    Grib2Template540(Grib2Template540),
    /// Data Representation Template 5.50 - Spectral data - simple packing
    Grib2Template550(Grib2Template550),
    /// Data Representation Template 5.51 - Spectral data - complex packing
    Grib2Template551(Grib2Template551),
}
impl Grib2Template5 {
    /// Creates a new instance of Grib2Template50
    pub fn new<T: Reader>(section: &T, template: u8) -> Self {
        match template {
            0 => Grib2Template5::Grib2Template50(Grib2Template50::new(section)),
            2 => Grib2Template5::Grib2Template52(Grib2Template52::new(section)),
            3 => Grib2Template5::Grib2Template53(Grib2Template53::new(section)),
            40 => Grib2Template5::Grib2Template540(Grib2Template540::new(section)),
            50 => Grib2Template5::Grib2Template550(Grib2Template550::new(section)),
            51 => Grib2Template5::Grib2Template551(Grib2Template551::new(section)),
            _ => panic!("Template 5.{template} not defined"),
        }
    }
    /// Get the simple packing template
    pub fn get_simple_packing_template(&self) -> Option<&Grib2Template50> {
        match self {
            Grib2Template5::Grib2Template50(template) => Some(template),
            _ => None,
        }
    }
}

/// # Data Representation Template 5.0 - Grid point data - simple packing
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-0.shtml)
///
/// ## Notes
/// - Negative values of E or D shall be represented according to Regulation [92.1.5](https://codes.ecmwf.int/grib/format/grib2/regulations/).
///
/// @returns - description of how to decode simple unpacked data
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template50 {
    /// Reference value (R) (IEEE 32-bit floating-point value)
    pub reference_value: f32,
    /// Binary scale factor (E)
    pub binary_scale_factor: i32,
    /// Decimal scale factor (D)
    pub decimal_scale_factor: i32,
    /// Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing
    pub number_of_bits: u8,
    /// Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml))
    pub original_type: Grib2Table5_1,
}
impl Grib2Template50 {
    /// Creates a new instance of Grib2Template50
    pub fn new<T: Reader>(section: &T) -> Self {
        let original_type = section.uint8(Some(20));
        let mut binary_scale_factor = (section.uint16_be(Some(15)) & 0x7fff) as i32;
        if section.uint16_be(Some(15)) >> 15 > 0 {
            binary_scale_factor *= -1;
        }
        let mut decimal_scale_factor = (section.uint16_be(Some(17)) & 0x7fff) as i32;
        if section.uint16_be(Some(17)) >> 15 > 0 {
            decimal_scale_factor *= -1;
        }

        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits: section.uint8(Some(19)),
            original_type: original_type.into(),
        }
    }
}

/// # Data Representation Template 5.2 – Complex packing (no spatial differencing).
///
/// Reads and parses the metadata fields defined by GRIB2 Template 5.2.
/// For most templates, details of the packing process are described in Regulation 92.9.4.
///
/// @see {@link https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-2.shtml Template 5.2 documentation}
/// @see {@link https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-2.shtml Data template 7.2 for complementary info}
/// @param section - Binary reader providing access to the section data.
/// @returns Object containing the fields of Template 5.2.
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template52 {
    /// Reference value (R) (IEEE 32-bit floating-point). Octets 12–15 in the GRIB2 documentation.
    pub reference_value: f32,
    /// Binary scale factor (E). Octets 16–17.
    pub binary_scale_factor: i32,
    /// Decimal scale factor (D). Octets 18–19.
    pub decimal_scale_factor: i32,
    /// Number of bits per packed value for simple packing, or per group reference for
    /// complex packing. Octet 20 in the documentation.
    pub number_of_bits: u8,
    /// Type of original field values. See Code Table 5.1. Octet 21.
    pub original_type: Grib2Table5_1,

    // Fields specific to complex packing (no spatial differencing):
    /// Group splitting method used. See Code Table 5.4. Octet 22.
    pub group_splitting_method: Grib2Table5_4,
    /// Missing value management. See Code Table 5.5. Octet 23.
    pub missing_value_management: Grib2Table5_5,
    /// Primary missing value substitute. Octets 24–27.
    pub primary_missing_value_substitute: u32,
    /// Secondary missing value substitute. Octets 28–31.
    pub secondary_missing_value_substitute: u32,
    /// Number of groups of data values (NG). Octets 32–35.
    pub number_of_groups: u32,
    /// Reference for group widths. Octet 36.
    /// The group width is the number of bits used for every value in a group.
    pub reference_for_group_widths: u8,
    /// Number of bits used for the group widths (after subtracting the reference value). Octet 37.
    pub group_widths_bits: u8,
    /// Reference for group lengths. Octets 38–41.
    /// The group length (L) is the number of values in a group.
    pub reference_for_group_lengths: u32,
    /// Length increment for group lengths. Octet 42.
    /// Used in the formula: Lₙ = ref + Kₙ × len_inc.
    pub group_length_factor: u8,
    /// True length of the last group. Octets 43–46.
    /// A special-case group length if the sequence doesn’t fit the formula.
    pub true_length_of_last_group: u32,
    /// Number of bits used for scaled group lengths (after subtracting ref
    /// and dividing by the length increment). Octet 47.
    pub n_bits_group_length: u8,
}
impl Grib2Template52 {
    /// Creates a new instance of Grib2Template52
    pub fn new<T: Reader>(section: &T) -> Self {
        // Binary and decimal scale factors can be negative.
        // They are stored with the sign bit in the high-order bit (bit 15).
        let original_type_code = section.uint8(Some(20));

        let mut binary_scale_factor = (section.uint16_be(Some(15)) & 0x7fff) as i32;
        if section.uint16_be(Some(15)) >> 15 > 0 {
            binary_scale_factor *= -1;
        }
        let mut decimal_scale_factor = (section.uint16_be(Some(17)) & 0x7fff) as i32;
        if section.uint16_be(Some(17)) >> 15 > 0 {
            decimal_scale_factor *= -1;
        }
        // Fields unique to 5.2 (similar to 5.3, but no spatial differencing):
        let group_splitting_method_code = section.uint8(Some(21)); // Octet 22
        let missing_value_management_code = section.uint8(Some(22)); // Octet 23
        let primary_missing_value_substitute = section.uint32_be(Some(23)); // Octets 24–27
        let secondary_missing_value_substitute = section.uint32_be(Some(27)); // Octets 28–31
        let number_of_groups = section.uint32_be(Some(31)); // Octets 32–35
        let reference_for_group_widths = section.uint8(Some(35)); // Octet 36
        let group_widths_bits = section.uint8(Some(36)); // Octet 37
        let reference_for_group_lengths = section.uint32_be(Some(37)); // Octets 38–41
        let group_length_factor = section.uint8(Some(41)); // Octet 42
        let true_length_of_last_group = section.uint32_be(Some(42)); // Octets 43–46
        let n_bits_group_length = section.uint8(Some(46)); // Octet 47

        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits: section.uint8(Some(19)),
            original_type: original_type_code.into(),
            group_splitting_method: group_splitting_method_code.into(),
            missing_value_management: missing_value_management_code.into(),
            primary_missing_value_substitute,
            secondary_missing_value_substitute,
            number_of_groups,
            reference_for_group_widths,
            group_widths_bits,
            reference_for_group_lengths,
            group_length_factor,
            true_length_of_last_group,
            n_bits_group_length,
        }
    }
}

/// # Data Representation Template 5.3 – Complex packing and spatial differencing.
///
/// Reads and parses the metadata fields defined by GRIB2 Template 5.3.
/// For most templates, details of the packing process are described in Regulation 92.9.4.
///
/// See also:
/// - [GRIB2 Template 5.3 documentation](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-3.shtml)
/// - [Data template 7.3 and associated notes](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-3.shtml)
/// - Spatial differencing (Regulation 92.9.4).
///
/// @param section - Binary reader providing access to the section data
/// @returns Object containing the fields of Template 5.3
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template53 {
    /// Reference value (R) (IEEE 32-bit floating-point).
    /// Octets 12–15 in the GRIB2 documentation.
    pub reference_value: f32,
    /// Binary scale factor (E). Octets 16–17.
    pub binary_scale_factor: i32,
    /// Decimal scale factor (D). Octets 18–19.
    pub decimal_scale_factor: i32,
    /// Number of bits per packed value for simple packing,
    /// or per group reference for complex packing.
    /// Octet 20 in the documentation.
    pub number_of_bits: u8,
    /// Type of original field values. See Code Table 5.1. Octet 21.
    pub original_type: Grib2Table5_1,

    // Fields specific to complex packing and spatial differencing:
    /// Group splitting method used. See Code Table 5.4. Octet 22.
    pub group_splitting_method: Grib2Table5_4,
    /// Missing value management. See Code Table 5.5. Octet 23.
    pub missing_value_management: Grib2Table5_5,
    /// Primary missing value substitute. Octets 24–27.
    pub primary_missing_value_substitute: u32,
    /// Secondary missing value substitute. Octets 28–31.
    pub secondary_missing_value_substitute: u32,
    /// Number of groups of data values (NG). Octets 32–35.
    pub number_of_groups: u32,
    /// Reference for group widths. Octet 36.
    /// The group width is the number of bits used for every value in a group.
    pub reference_for_group_widths: u8,
    /// Number of bits used for the group widths (after subtracting the reference value).
    /// Octet 37.
    pub group_widths_bits: u8,
    /// Reference for group lengths. Octets 38–41.
    /// The group length (L) is the number of values in a group.
    pub reference_for_group_lengths: u32,
    /// Length increment for group lengths. Octet 42.
    /// Used in the formula: Lₙ = ref + Kₙ × len_inc.
    pub group_length_factor: u8,
    /// True length of the last group. Octets 43–46.
    /// A special-case group length if the sequence doesn’t fit the formula.
    pub true_length_of_last_group: u32,
    /// Number of bits used for scaled group lengths (after subtracting ref
    /// and dividing by the length increment). Octet 47.
    pub n_bits_group_length: u8,
    /// Order of spatial difference. See Code Table 5.6. Octet 48.
    pub order_of_spatial_difference: Grib2Table5_6,
    /// Number of extra descriptor octets needed for spatial differencing
    /// (octets 6–ww in data template 7.3). Octet 49.
    pub extra_descriptor_octets: u8,
}
impl Grib2Template53 {
    /// Create a new instance of Grib2Template53
    pub fn new<T: Reader>(section: &T) -> Self {
        // Binary and decimal scale factors can be negative.
        // They are stored with the sign bit in the high-order bit (bit 15).
        let original_type_code = section.uint8(Some(20));
        let mut binary_scale_factor = (section.uint16_be(Some(15)) & 0x7fff) as i32;
        if section.uint16_be(Some(15)) >> 15 > 0 {
            binary_scale_factor *= -1;
        }
        let mut decimal_scale_factor = (section.uint16_be(Some(17)) & 0x7fff) as i32;
        if section.uint16_be(Some(17)) >> 15 > 0 {
            decimal_scale_factor *= -1;
        }
        // New fields introduced by Template 5.3
        let group_splitting_method_code = section.uint8(Some(21));
        let missing_value_management_code = section.uint8(Some(22));
        let primary_missing_value_substitute = section.uint32_be(Some(23));
        let secondary_missing_value_substitute = section.uint32_be(Some(27));
        let number_of_groups = section.uint32_be(Some(31));
        let reference_for_group_widths = section.uint8(Some(35));
        let group_widths_bits = section.uint8(Some(36));
        let reference_for_group_lengths = section.uint32_be(Some(37));
        let group_length_factor = section.uint8(Some(41));
        let true_length_of_last_group = section.uint32_be(Some(42));
        let n_bits_group_length = section.uint8(Some(46));
        let order_of_spatial_difference_code = section.uint8(Some(47));
        let extra_descriptor_octets = section.uint8(Some(48));

        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits: section.uint8(Some(19)),
            original_type: original_type_code.into(),
            group_splitting_method: group_splitting_method_code.into(),
            missing_value_management: missing_value_management_code.into(),
            primary_missing_value_substitute,
            secondary_missing_value_substitute,
            number_of_groups,
            reference_for_group_widths,
            group_widths_bits,
            reference_for_group_lengths,
            group_length_factor,
            true_length_of_last_group,
            n_bits_group_length,
            order_of_spatial_difference: order_of_spatial_difference_code.into(),
            extra_descriptor_octets,
        }
    }
}

/// Data Representation Template 5.40
///  
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-40.shtml)
///
/// @param section - The raw section data to parse
/// @returns - Parsed Data Representation Information
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template540 {
    /// Reference value (R) (IEEE 32-bit floating-point value) */
    pub reference_value: f32,
    /// Binary scale factor (E) */
    pub binary_scale_factor: i16,
    /// Decimal scale factor (D) */
    pub decimal_scale_factor: i16,
    /// Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing */
    pub number_of_bits: u8,
    /// Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml)) */
    pub original_type: Grib2Table5_1,
    /// Type of Compression used. (see [Code Table 5.40](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-40.shtml)) */
    pub compression_type: Grib2Table5_40,
    /// Target compression ratio, M:1 (with respect to the bit-depth specified in octet 20),
    /// when octet 22 indicates Lossy Compression. Otherwise, set to missing.
    pub compression_ratio: u8,
}
impl Grib2Template540 {
    /// Create a new instance of Grib2Template540
    pub fn new<T: Reader>(section: &T) -> Self {
        let original_type_code = section.uint8(Some(20));
        let compression_type = section.uint8(Some(21));
        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor: section.int16_be(Some(15)),
            decimal_scale_factor: section.int16_be(Some(17)),
            number_of_bits: section.uint8(Some(19)),
            original_type: original_type_code.into(),
            compression_type: compression_type.into(),
            compression_ratio: section.uint8(Some(22)),
        }
    }
}
// export function grib2Template540(section: Reader) {
// }

/// # Data Representation Template 5.50 - Spectral data - simple packing
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-50.shtml)
///
/// ## Notes
/// - Removal of the real part of (0.0) coefficient from packed data is intended to reduce the
///   variability of the coefficients, in order to improve packing accuracy.
/// - For some spectral representations, the (0.0) coefficient represents the mean value of the
///   parameter represented.
/// - Negative values of E or D shall be represented according to Regulation [92.1.5](https://codes.ecmwf.int/grib/format/grib2/regulations/).
///
/// @returns - description of how to decode simple unpacked data
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template550 {
    /// Reference value (R) (IEEE 32-bit floating-point value)
    pub reference_value: f32,
    /// Binary scale factor (E)
    pub binary_scale_factor: i32,
    /// Decimal scale factor (D)
    pub decimal_scale_factor: i32,
    /// Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing
    pub number_of_bits: u8,
    /// Type of original field values (see Code [Table 5.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table5-1.shtml))
    pub real_part_coefficient_type: f32,
}
impl Grib2Template550 {
    /// Create a new instance of Grib2Template540
    pub fn new<T: Reader>(section: &T) -> Self {
        let mut binary_scale_factor = (section.uint16_be(Some(15)) & 0x7fff) as i32;
        if section.uint16_be(Some(15)) >> 15 > 0 {
            binary_scale_factor *= -1;
        }
        let mut decimal_scale_factor = (section.uint16_be(Some(17)) & 0x7fff) as i32;
        if section.uint16_be(Some(17)) >> 15 > 0 {
            decimal_scale_factor *= -1;
        }

        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits: section.uint8(Some(19)),
            real_part_coefficient_type: section.f32_be(Some(20)),
        }
    }
}

/// # Data Representation Template 5.51 - Spectral data - complex packing
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp5-51.shtml)
///
/// ## Notes
/// - The unpacked subset is a set of values defined in the same way as the full set of values
///   (on a spectrum limited to j_s, k_s and m_s ), but on which scaling and packing are not applied.
///   Associated values are stored in octets 6 onwards of section 7.
/// - The remaining coefficients are multiplied by `(n x (n+1))p` , scaled and packed. The operator
///   associated with this multiplication is derived from the Laplacian operator on the sphere.
/// - The retrieval formula for a coefficient of wave number n is then: `Y = (R+X x 2e ) x 10-d x (n x(n+1))-p`
///   where X is the packed scaled value associated with the coefficient.
///
/// @returns - description of how to decode simple unpacked data
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template551 {
    /// Reference value (R) (IEEE 32-bit floating-point value)
    pub reference_value: f32,
    /// Binary scale factor (E)
    pub binary_scale_factor: i32,
    /// Decimal scale factor (D)
    pub decimal_scale_factor: i32,
    /// Number of bits used for each packed value for simple packing, or for each group reference value for complex packing or spatial differencing
    pub number_of_bits: u8,
    /// P ― Laplacian scaling factor (expressed in 10^-6 units)
    pub p: f32,
    /// j_s ― pentagonal resolution parameter of the unpacked subset (see Note1)
    pub j_s: i16,
    /// k_s ― pentagonal resolution parameter of the unpacked subset (see Note1)
    pub k_s: i16,
    /// m_s ― pentagonal resolution parameter of the unpacked subset (see Note1)
    pub m_s: i16,
    /// t_s ― total number of values in the unpacked subset (see Note1)
    pub t_s: i32,
    /// Precision of the unpacked subset (see Code Table 5.7)
    pub precision: Grib2Table5_7,
}
impl Grib2Template551 {
    /// Create a new instance of Grib2Template540
    pub fn new<T: Reader>(section: &T) -> Self {
        let mut binary_scale_factor = (section.uint16_be(Some(15)) & 0x7fff) as i32;
        if section.uint16_be(Some(15)) >> 15 > 0 {
            binary_scale_factor *= -1;
        }
        let mut decimal_scale_factor = (section.uint16_be(Some(17)) & 0x7fff) as i32;
        if section.uint16_be(Some(17)) >> 15 > 0 {
            decimal_scale_factor *= -1;
        }
        let precision_code = section.uint8(Some(34));

        Self {
            reference_value: section.f32_be(Some(11)),
            binary_scale_factor,
            decimal_scale_factor,
            number_of_bits: section.uint8(Some(19)),
            p: section.f32_be(Some(20)),
            j_s: section.int16_be(Some(24)),
            k_s: section.int16_be(Some(26)),
            m_s: section.int16_be(Some(28)),
            t_s: section.int32_be(Some(30)),
            precision: precision_code.into(),
        }
    }
}
