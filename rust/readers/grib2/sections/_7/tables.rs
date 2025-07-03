/// # Table 7.0 - DATA TEMPLATE DEFINITIONS USED IN SECTION 7
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 64
/// - **Applicable Grid Templates**: 1000
///
/// **Reserved Ranges**:
/// - `5-39`: Reserved
/// - `43-49`: Reserved
/// - `52`: Reserved
/// - `54-49151`: Reserved
/// - `49152-65534`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the data template definitions used in Section 7 of GRIB2 files,
/// specifying various data representation types and their corresponding templates.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table7-0.shtml)
///
/// ## Notes
/// - Created 05/11/2005
/// - Red text depicts changes made since 05/11/2005.
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table7_0 {
    GridPointDataSimplePacking = 0,
    MatrixValueAtGridPointSimplePacking = 1,
    GridPointDataComplexPacking = 2,
    GridPointDataComplexPackingAndSpatialDifferencing = 3,
    GridPointDataIeeeFloatingPointData = 4,
    GridPointDataJpeg2000Compression = 40,
    GridPointDataPortableNetworkGraphicsPngFormat = 41,
    GridPointAndSpectralDataCcsdsRecommendedLosslessCompression = 42,
    SpectralDataSimplePacking = 50,
    SpectralDataComplexPacking = 51,
    SpectralDataForLimitedAreaModelsComplexPacking = 53,
    Missing = 255, // Note: The original JS has 255 as Missing, but the reserved range goes higher.
    Unknown(u16),
}

impl From<u16> for Grib2Table7_0 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::GridPointDataSimplePacking,
            1 => Self::MatrixValueAtGridPointSimplePacking,
            2 => Self::GridPointDataComplexPacking,
            3 => Self::GridPointDataComplexPackingAndSpatialDifferencing,
            4 => Self::GridPointDataIeeeFloatingPointData,
            40 => Self::GridPointDataJpeg2000Compression,
            41 => Self::GridPointDataPortableNetworkGraphicsPngFormat,
            42 => Self::GridPointAndSpectralDataCcsdsRecommendedLosslessCompression,
            50 => Self::SpectralDataSimplePacking,
            51 => Self::SpectralDataComplexPacking,
            53 => Self::SpectralDataForLimitedAreaModelsComplexPacking,
            255 => Self::Missing,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table7_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::GridPointDataSimplePacking => {
                "Grid Point Data - Simple Packing (see Template 7.0)"
            }
            Self::MatrixValueAtGridPointSimplePacking => {
                "Matrix Value at Grid Point - Simple Packing (see Template 7.1)"
            }
            Self::GridPointDataComplexPacking => {
                "Grid Point Data - Complex Packing (see Template 7.2)"
            }
            Self::GridPointDataComplexPackingAndSpatialDifferencing => {
                "Grid Point Data - Complex Packing and Spatial Differencing (see Template 7.3)"
            }
            Self::GridPointDataIeeeFloatingPointData => {
                "Grid Point Data - IEEE Floating Point Data (see Template 7.4)"
            }
            Self::GridPointDataJpeg2000Compression => {
                "Grid Point Data - JPEG2000 Compression (see Template 7.40)"
            }
            Self::GridPointDataPortableNetworkGraphicsPngFormat => {
                "Grid Point Data - Portable Network Graphics (PNG) format (see Template 7.41)"
            }
            Self::GridPointAndSpectralDataCcsdsRecommendedLosslessCompression => {
                "Grid Point and Spectral data - CCSDS recommended lossless compression (see Template 7.42)"
            }
            Self::SpectralDataSimplePacking => "Spectral Data - Simple Packing (see Template 7.50)",
            Self::SpectralDataComplexPacking => {
                "Spectral Data - Complex Packing (see Template 7.51)"
            }
            Self::SpectralDataForLimitedAreaModelsComplexPacking => {
                "Spectral Data for limited area models - Complex Packing (see Template 7.53)"
            }
            Self::Missing => "Missing",
            Self::Unknown(v) => return write!(f, "Unknown Data Template Definition ({})", v),
        };
        f.write_str(desc)
    }
}
