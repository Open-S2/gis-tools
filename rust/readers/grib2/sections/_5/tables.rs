/// # GRIB2 - TABLE 5.0 - DATA REPRESENTATION TEMPLATE NUMBER
///
/// **Details**:
/// - **Section**: 5
/// - **Octets**: 10-11
/// - **Revised**: 07/01/2022
///
/// **Reserved Ranges**:
/// - `5-39`: Reserved
/// - `43-49`: Reserved
/// - `52`: Reserved
/// - `54-60`: Reserved
/// - `62-199`: Reserved
/// - `201-49151`: Reserved
/// - `49152-65534`: Reserved for Local Use
///
/// **Special Value**:
/// - `65535`: Missing
///
/// ## Notes
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_0 {
    GridPointDataSimplePacking = 0,
    MatrixValueAtGridPointSimplePacking = 1,
    GridPointDataComplexPacking = 2,
    GridPointDataComplexPackingAndSpatialDifferencing = 3,
    GridPointDataIeeeFloatingPointData = 4,
    GridPointDataJpeg2000CodeStreamFormat = 40,
    GridPointDataPortableNetworkGraphicsPng = 41,
    GridPointDataCcsdsRecommendedLosslessCompression = 42,
    SpectralDataSimplePacking = 50,
    SpectralDataComplexPacking = 51,
    SpectralDataForLimitedAreaModelsComplexPacking = 53,
    GridPointDataSimplePackingWithLogarithmPreProcessing = 61,
    RunLengthPackingWithLevelValues = 200,
    GridPointDataJpeg2000CodeStreamFormatAndSpatialDifferencing = 40000,
    Missing = 65535,
}
impl From<u16> for Grib2Table5_0 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::GridPointDataSimplePacking,
            1 => Self::MatrixValueAtGridPointSimplePacking,
            2 => Self::GridPointDataComplexPacking,
            3 => Self::GridPointDataComplexPackingAndSpatialDifferencing,
            4 => Self::GridPointDataIeeeFloatingPointData,
            40 => Self::GridPointDataJpeg2000CodeStreamFormat,
            41 => Self::GridPointDataPortableNetworkGraphicsPng,
            42 => Self::GridPointDataCcsdsRecommendedLosslessCompression,
            50 => Self::SpectralDataSimplePacking,
            51 => Self::SpectralDataComplexPacking,
            53 => Self::SpectralDataForLimitedAreaModelsComplexPacking,
            61 => Self::GridPointDataSimplePackingWithLogarithmPreProcessing,
            200 => Self::RunLengthPackingWithLevelValues,
            40000 => Self::GridPointDataJpeg2000CodeStreamFormatAndSpatialDifferencing,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::GridPointDataSimplePacking => {
                "Grid Point Data - Simple Packing (see Template 5.0)"
            }
            Self::MatrixValueAtGridPointSimplePacking => {
                "Matrix Value at Grid Point - Simple Packing (see Template 5.1)"
            }
            Self::GridPointDataComplexPacking => {
                "Grid Point Data - Complex Packing (see Template 5.2)"
            }
            Self::GridPointDataComplexPackingAndSpatialDifferencing => {
                "Grid Point Data - Complex Packing and Spatial Differencing (see Template 5.3)"
            }
            Self::GridPointDataIeeeFloatingPointData => {
                "Grid Point Data - IEEE Floating Point Data (see Template 5.4)"
            }
            Self::GridPointDataJpeg2000CodeStreamFormat => {
                "Grid point data - JPEG 2000 code stream format (see Template 5.40)"
            }
            Self::GridPointDataPortableNetworkGraphicsPng => {
                "Grid point data - Portable Network Graphics (PNG) (see Template 5.41)"
            }
            Self::GridPointDataCcsdsRecommendedLosslessCompression => {
                "Grid point data - CCSDS recommended lossless compression (see Template 5.42)"
            }
            Self::SpectralDataSimplePacking => "Spectral Data - Simple Packing (see Template 5.50)",
            Self::SpectralDataComplexPacking => {
                "Spectral Data - Complex Packing (see Template 5.51)"
            }
            Self::SpectralDataForLimitedAreaModelsComplexPacking => {
                "Spectral data for limited area models - complex packing (see Template 5.53)"
            }
            Self::GridPointDataSimplePackingWithLogarithmPreProcessing => {
                "Grid Point Data - Simple Packing With Logarithm Pre-processing (see Template 5.61)"
            }
            Self::RunLengthPackingWithLevelValues => {
                "Run Length Packing With Level Values (see Template 5.200)"
            }
            Self::GridPointDataJpeg2000CodeStreamFormatAndSpatialDifferencing => {
                "Grid point data - JPEG 2000 code stream format and spatial differencing (see Template 5.40000)"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.1 - TYPE OF ORIGINAL FIELD VALUES
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_1 {
    FloatingPoint = 0,
    Integer = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table5_1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::FloatingPoint,
            1 => Self::Integer,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::FloatingPoint => "Floating Point",
            Self::Integer => "Integer",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.2 - MATRIX COORDINATE VALUE FUNCTION DEFINITION
///
/// **Details**:
/// - **Revised**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-10`: Reserved
/// - `12-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_2 {
    ExplicitCoordinateValuesSet = 0,
    LinearCoordinates = 1,
    GeometricCoordinates = 11,
    Missing = 255,
}
impl From<u8> for Grib2Table5_2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ExplicitCoordinateValuesSet,
            1 => Self::LinearCoordinates,
            11 => Self::GeometricCoordinates,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ExplicitCoordinateValuesSet => "Explicit Coordinate Values Set",
            Self::LinearCoordinates => "Linear Coordinates: f(1) = C1, f(n) = f(n-1) + C2",
            Self::GeometricCoordinates => "Geometric Coordinates: f(1) = C1, f(n) = C2 x f(n-1)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.3 - MATRIX COORDINATE PARAMETER
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_3 {
    DirectionDegreesTrue = 1,
    FrequencyS1 = 2,
    RadialNumber2piLambdaM1 = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table5_3 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::DirectionDegreesTrue,
            2 => Self::FrequencyS1,
            3 => Self::RadialNumber2piLambdaM1,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::DirectionDegreesTrue => "Direction Degrees True",
            Self::FrequencyS1 => "Frequency (s-1)",
            Self::RadialNumber2piLambdaM1 => "Radial Number (2pi/lambda) (m-1)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.4 - GROUP SPLITTING METHOD
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_4 {
    RowByRowSplitting = 0,
    GeneralGroupSplitting = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table5_4 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::RowByRowSplitting,
            1 => Self::GeneralGroupSplitting,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::RowByRowSplitting => "Row by Row Splitting",
            Self::GeneralGroupSplitting => "General Group Splitting",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.5 - MISSING VALUE MANAGEMENT FOR COMPLEX PACKING
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_5 {
    NoExplicitMissingValues = 0,
    PrimaryMissingValuesIncluded = 1,
    PrimaryAndSecondaryMissingValuesIncluded = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table5_5 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoExplicitMissingValues,
            1 => Self::PrimaryMissingValuesIncluded,
            2 => Self::PrimaryAndSecondaryMissingValuesIncluded,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoExplicitMissingValues => {
                "No explicit missing values included within the data values"
            }
            Self::PrimaryMissingValuesIncluded => {
                "Primary missing values included within the data values"
            }
            Self::PrimaryAndSecondaryMissingValuesIncluded => {
                "Primary and secondary missing values included within the data values"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.6 - ORDER OF SPATIAL DIFFERENCING
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_6 {
    FirstOrderSpatialDifferencing = 1,
    SecondOrderSpatialDifferencing = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table5_6 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::FirstOrderSpatialDifferencing,
            2 => Self::SecondOrderSpatialDifferencing,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::FirstOrderSpatialDifferencing => "First-Order Spatial Differencing",
            Self::SecondOrderSpatialDifferencing => "Second-Order Spatial Differencing",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.7 - PRECISION OF FLOATING POINT NUMBERS
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `4-254`: Reserved
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_7 {
    Ieee32Bit = 1,
    Ieee64Bit = 2,
    Ieee128Bit = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table5_7 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Ieee32Bit,
            2 => Self::Ieee64Bit,
            3 => Self::Ieee128Bit,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Ieee32Bit => "IEEE 32-bit (I=4 in Section 7)",
            Self::Ieee64Bit => "IEEE 64-bit (I=8 in Section 7)",
            Self::Ieee128Bit => "IEEE 128-bit (I=16 in Section 7)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.25 - TYPE OF BI-FOURIER SUBTRUNCATION
///
/// **Details**:
/// - **Created**: 05/29/2019
///
/// **Reserved Ranges**:
/// - `0-76`: Reserved
/// - `78-87`: Reserved
/// - `89-98`: Reserved
/// - `100-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_25 {
    Rectangular = 77,
    Elliptic = 88,
    Diamond = 99,
    Missing = 255,
}
impl From<u8> for Grib2Table5_25 {
    fn from(val: u8) -> Self {
        match val {
            77 => Self::Rectangular,
            88 => Self::Elliptic,
            99 => Self::Diamond,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_25 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Rectangular => "Rectangular",
            Self::Elliptic => "Elliptic",
            Self::Diamond => "Diamond",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.26 - PACKING MODE FOR AXES
///
/// **Details**:
/// - **Created**: 05/29/2019
///
/// **Reserved Ranges**:
/// - `2-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_26 {
    SpectralCoefficientsForAxesArePacked = 0,
    SpectralCoefficientsForAxesIncludedInUnpackedSubset = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table5_26 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::SpectralCoefficientsForAxesArePacked,
            1 => Self::SpectralCoefficientsForAxesIncludedInUnpackedSubset,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::SpectralCoefficientsForAxesArePacked => {
                "Spectral coefficients for axes are packed"
            }
            Self::SpectralCoefficientsForAxesIncludedInUnpackedSubset => {
                "Spectral coefficients for axes included in the unpacked subset"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - TABLE 5.40 - TYPE OF COMPRESSION
///
/// **Details**:
/// - **Created**: 02/14/2006
///
/// **Reserved Ranges**:
/// - `2-254`: Reserved
///
/// **Special Value**:
/// - `255`: Missing
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table5_40 {
    Lossless = 0,
    Lossy = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table5_40 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Lossless,
            1 => Self::Lossy,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table5_40 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Lossless => "Lossless",
            Self::Lossy => "Lossy",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}
