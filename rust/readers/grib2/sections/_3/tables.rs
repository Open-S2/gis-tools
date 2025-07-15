/// # Table 3.0 - Source of Grid Definition
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 6
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: A grid definition does not apply to this product.
///
/// ## Description
/// This table specifies the source of grid definitions used in GRIB2 files,
/// providing context for how the grid is defined, whether through predefined templates or originating centers.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-0.shtml)
///
/// ## Notes
/// - Created 05/11/2005
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_0 {
    SpecifiedInCodeTable3_1 = 0,
    PredeterminedGridDefinitionDefinedByOriginatingCenter = 1,
    AGridDefinitionDoesNotApplyToThisProduct = 255,
}
impl From<u8> for Grib2Table3_0 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::SpecifiedInCodeTable3_1,
            1 => Self::PredeterminedGridDefinitionDefinedByOriginatingCenter,
            _ => Self::AGridDefinitionDoesNotApplyToThisProduct,
        }
    }
}
impl core::fmt::Display for Grib2Table3_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::SpecifiedInCodeTable3_1 => "Specified in Code Table 3.1",
            Self::PredeterminedGridDefinitionDefinedByOriginatingCenter => {
                "Predetermined Grid Definition - Defined by Originating Center"
            }
            Self::AGridDefinitionDoesNotApplyToThisProduct => {
                "A grid definition does not apply to this product."
            }
        };
        f.write_str(desc)
    }
}

/// # Table 3.1 - Grid Definition Template Number
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 13-14
///
/// **Reserved Ranges**:
/// - `3-32767`: Reserved
/// - `32768-65534`: Reserved for Local Use
///
/// **Special Value**:
/// - `65535`: Missing
///
/// ## Description
/// This table enumerates the grid definition templates used in GRIB2 files,
/// providing detailed classifications for various grid types, projections, and modeling subdomains.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
///
/// ## Notes
/// - Revised 12/07/2023
/// - (1). WGS84 is a geodetic system that uses IAG-GRS80 as a basis.
/// - (2). With respect to code figures 0, 1, 3, 6, and 7, coordinates can only be unambiguously interpreted if the coordinate reference system in which they are embedded is known. Therefore, defining the shape of the Earth alone without coordinate system axis origins is ambiguous. Generally, the prime meridian defined in the geodetic system WGS-84 can be safely assumed to be the longitudinal origin. However, because these code figures do not specify the longitudinal origin explicitly, it is suggested to contact the originating center if high precision coordinates are needed to obtain the precise details of the coordinate system used (effective as from 16 November 2016).
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_1 {
    LatitudeLongitude = 0,
    RotatedLatitudeLongitude = 1,
    StretchedLatitudeLongitude = 2,
    RotatedAndStretchedLatitudeLongitude = 3,
    VariableResolutionLatitudeLongitude = 4,
    VariableResolutionRotatedLatitudeLongitude = 5,
    Mercator = 10,
    Reserved11 = 11, // Explicitly reserved value
    TransverseMercator = 12,
    MercatorWithModellingSubdomainsDefinition = 13,
    PolarStereographicProjection = 20,
    PolarStereographicWithModellingSubdomainsDefinition = 23,
    LambertConformal = 30,
    AlbersEqualArea = 31,
    Reserved32 = 32, // Explicitly reserved value
    LambertConformalWithModellingSubdomainsDefinition = 33,
    GaussianLatitudeLongitude = 40,
    RotatedGaussianLatitudeLongitude = 41,
    StretchedGaussianLatitudeLongitude = 42,
    RotatedAndStretchedGaussianLatitudeLongitude = 43,
    SphericalHarmonicCoefficients = 50,
    RotatedSphericalHarmonicCoefficients = 51,
    StretchedSphericalHarmonicCoefficients = 52,
    RotatedAndStretchedSphericalHarmonicCoefficients = 53,
    CubedSphereGnomonic = 60,
    SpectralMercatorWithModellingSubdomainsDefinition = 61,
    SpectralPolarStereographicWithModellingSubdomainsDefinition = 62,
    SpectralLambertConformalWithModellingSubdomainsDefinition = 63,
    SpaceViewPerspectiveOrOrthographic = 90,
    TriangularGridBasedOnAnIcosahedron = 100,
    GeneralUnstructuredGrid = 101,
    EquatorialAzimuthalEquidistantProjection = 110,
    AzimuthRangeProjection = 120,
    LambertAzimuthalEqualAreaProjection = 140,
    HierarchicalEqualAreaIsoLatitudePixelizationGridHealpix = 150,
    CurvilinearOrthogonalGrids = 204,
    CrossSectionGridWithPointsEquallySpacedOnTheHorizontal = 1000,
    HovmollerDiagramWithPointsEquallySpacedOnTheHorizontal = 1100,
    TimeSectionGrid = 1200,
    RotatedLatitudeLongitudeArakawaStaggeredEGrid = 32768,
    RotatedLatitudeLongitudeArakawaNonEStaggeredGrid = 32769,
    Missing = 65535,
}
impl From<u16> for Grib2Table3_1 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::LatitudeLongitude,
            1 => Self::RotatedLatitudeLongitude,
            2 => Self::StretchedLatitudeLongitude,
            3 => Self::RotatedAndStretchedLatitudeLongitude,
            4 => Self::VariableResolutionLatitudeLongitude,
            5 => Self::VariableResolutionRotatedLatitudeLongitude,
            10 => Self::Mercator,
            11 => Self::Reserved11,
            12 => Self::TransverseMercator,
            13 => Self::MercatorWithModellingSubdomainsDefinition,
            20 => Self::PolarStereographicProjection,
            23 => Self::PolarStereographicWithModellingSubdomainsDefinition,
            30 => Self::LambertConformal,
            31 => Self::AlbersEqualArea,
            32 => Self::Reserved32,
            33 => Self::LambertConformalWithModellingSubdomainsDefinition,
            40 => Self::GaussianLatitudeLongitude,
            41 => Self::RotatedGaussianLatitudeLongitude,
            42 => Self::StretchedGaussianLatitudeLongitude,
            43 => Self::RotatedAndStretchedGaussianLatitudeLongitude,
            50 => Self::SphericalHarmonicCoefficients,
            51 => Self::RotatedSphericalHarmonicCoefficients,
            52 => Self::StretchedSphericalHarmonicCoefficients,
            53 => Self::RotatedAndStretchedSphericalHarmonicCoefficients,
            60 => Self::CubedSphereGnomonic,
            61 => Self::SpectralMercatorWithModellingSubdomainsDefinition,
            62 => Self::SpectralPolarStereographicWithModellingSubdomainsDefinition,
            63 => Self::SpectralLambertConformalWithModellingSubdomainsDefinition,
            90 => Self::SpaceViewPerspectiveOrOrthographic,
            100 => Self::TriangularGridBasedOnAnIcosahedron,
            101 => Self::GeneralUnstructuredGrid,
            110 => Self::EquatorialAzimuthalEquidistantProjection,
            120 => Self::AzimuthRangeProjection,
            140 => Self::LambertAzimuthalEqualAreaProjection,
            150 => Self::HierarchicalEqualAreaIsoLatitudePixelizationGridHealpix,
            204 => Self::CurvilinearOrthogonalGrids,
            1000 => Self::CrossSectionGridWithPointsEquallySpacedOnTheHorizontal,
            1100 => Self::HovmollerDiagramWithPointsEquallySpacedOnTheHorizontal,
            1200 => Self::TimeSectionGrid,
            32768 => Self::RotatedLatitudeLongitudeArakawaStaggeredEGrid,
            32769 => Self::RotatedLatitudeLongitudeArakawaNonEStaggeredGrid,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::LatitudeLongitude => {
                "Latitude/Longitude (See Template 3.0) Also called Equidistant Cylindrical or \
                 Plate Caree"
            }
            Self::RotatedLatitudeLongitude => "Rotated Latitude/Longitude (See Template 3.1)",
            Self::StretchedLatitudeLongitude => "Stretched Latitude/Longitude (See Template 3.2)",
            Self::RotatedAndStretchedLatitudeLongitude => {
                "Rotated and Stretched Latitude/Longitude (See Template 3.3)"
            }
            Self::VariableResolutionLatitudeLongitude => {
                "Variable Resolution Latitude/longitude (See Template 3.4)"
            }
            Self::VariableResolutionRotatedLatitudeLongitude => {
                "Variable Resolution Rotated Latitude/longitude (See Template 3.5)"
            }
            Self::Mercator => "Mercator (See Template 3.10)",
            Self::Reserved11 => "Reserved",
            Self::TransverseMercator => "Transverse Mercator (See Template 3.12)",
            Self::MercatorWithModellingSubdomainsDefinition => {
                "Mercator with modelling subdomains definition (See Template 3.13)"
            }
            Self::PolarStereographicProjection => {
                "Polar Stereographic Projection (Can be North or South) (See Template 3.20)"
            }
            Self::PolarStereographicWithModellingSubdomainsDefinition => {
                "Polar Stereographic with modelling subdomains definition (See Template 3.23)"
            }
            Self::LambertConformal => {
                "Lambert Conformal (Can be Secant, Tangent, Conical, or Bipolar) (See Template \
                 3.30)"
            }
            Self::AlbersEqualArea => "Albers Equal Area (See Template 3.31)",
            Self::Reserved32 => "Reserved",
            Self::LambertConformalWithModellingSubdomainsDefinition => {
                "Lambert conformal with modelling subdomains definition (See Template 3.33)"
            }
            Self::GaussianLatitudeLongitude => "Gaussian Latitude/Longitude (See Template 3.40)",
            Self::RotatedGaussianLatitudeLongitude => {
                "Rotated Gaussian Latitude/Longitude (See Template 3.41)"
            }
            Self::StretchedGaussianLatitudeLongitude => {
                "Stretched Gaussian Latitude/Longitude (See Template 3.42)"
            }
            Self::RotatedAndStretchedGaussianLatitudeLongitude => {
                "Rotated and Stretched Gaussian Latitude/Longitude (See Template 3.43)"
            }
            Self::SphericalHarmonicCoefficients => {
                "Spherical Harmonic Coefficients (See Template 3.50)"
            }
            Self::RotatedSphericalHarmonicCoefficients => {
                "Rotated Spherical Harmonic Coefficients (See Template 3.51)"
            }
            Self::StretchedSphericalHarmonicCoefficients => {
                "Stretched Spherical Harmonic Coefficients (See Template 3.52)"
            }
            Self::RotatedAndStretchedSphericalHarmonicCoefficients => {
                "Rotated and Stretched Spherical Harmonic Coefficients (See Template 3.53)"
            }
            Self::CubedSphereGnomonic => "Cubed-Sphere Gnomonic (See Template 3.60) Validation",
            Self::SpectralMercatorWithModellingSubdomainsDefinition => {
                "Spectral Mercator with modelling subdomains definition (See Template 3.61)"
            }
            Self::SpectralPolarStereographicWithModellingSubdomainsDefinition => {
                "Spectral Polar Stereographic with modelling subdomains definition (See Template \
                 3.62)"
            }
            Self::SpectralLambertConformalWithModellingSubdomainsDefinition => {
                "Spectral Lambert conformal with modelling subdomains definition (See Template \
                 3.63)"
            }
            Self::SpaceViewPerspectiveOrOrthographic => {
                "Space View Perspective or Orthographic (See Template 3.90)"
            }
            Self::TriangularGridBasedOnAnIcosahedron => {
                "Triangular Grid Based on an Icosahedron (See Template 3.100)"
            }
            Self::GeneralUnstructuredGrid => "General Unstructured Grid (see Template 3.101)",
            Self::EquatorialAzimuthalEquidistantProjection => {
                "Equatorial Azimuthal Equidistant Projection (See Template 3.110)"
            }
            Self::AzimuthRangeProjection => "Azimuth-Range Projection (See Template 3.120)",
            Self::LambertAzimuthalEqualAreaProjection => {
                "Lambert Azimuthal Equal Area Projection (See Template 3.140)"
            }
            Self::HierarchicalEqualAreaIsoLatitudePixelizationGridHealpix => {
                "Hierarchical Equal Area isoLatitude Pixelization grid (HEALPix) (See Template \
                 3.150)"
            }
            Self::CurvilinearOrthogonalGrids => "Curvilinear Orthogonal Grids (See Template 3.204)",
            Self::CrossSectionGridWithPointsEquallySpacedOnTheHorizontal => {
                "Cross Section Grid with Points Equally Spaced on the Horizontal (See Template \
                 3.1000)"
            }
            Self::HovmollerDiagramWithPointsEquallySpacedOnTheHorizontal => {
                "Hovmoller Diagram with Points Equally Spaced on the Horizontal (See Template \
                 3.1100)"
            }
            Self::TimeSectionGrid => "Time Section Grid (See Template 3.1200)",
            Self::RotatedLatitudeLongitudeArakawaStaggeredEGrid => {
                "Rotated Latitude/Longitude (Arakawa Staggered E-Grid) (See Template 3.32768)"
            }
            Self::RotatedLatitudeLongitudeArakawaNonEStaggeredGrid => {
                "Rotated Latitude/Longitude (Arakawa Non-E Staggered Grid) (See Template 3.32769)"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.2 - Shape of the Reference System
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 15
///
/// **Reserved Ranges**:
/// - `12-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the shape of the reference system used in GRIB2 files,
/// providing context for interpreting the Earth's shape and the coordinate reference system.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
///
/// ## Notes
/// - (1) WGS84 is a geodetic system that uses IAG-GRS80 as a basis.
/// - (2) With respect to code figures 0, 1, 3, 6, and 7, coordinates can only be unambiguously interpreted if the coordinate reference system in which they are embedded is known. Therefore, defining the shape of the Earth alone without coordinate system axis origins is ambiguous. Generally, the prime meridian defined in the geodetic system WGS-84 can be safely assumed to be the longitudinal origin. However, because these code figures do not specify the longitudinal origin explicitly, it is suggested to contact the originating center if high precision coordinates are needed to obtain the precise details of the coordinate system used (effective as from 16 November 2016).
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_2 {
    EarthSphericalRadius6367470 = 0,
    EarthSphericalRadiusSpecifiedByProducer = 1,
    EarthOblateSpheroidIau1965 = 2,
    EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerKm = 3,
    EarthOblateSpheroidIagGrs80 = 4,
    EarthRepresentedByWgs84 = 5,
    EarthSphericalRadius6371229 = 6,
    EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerM = 7,
    EarthSphericalRadius6371200Wgs84Datum = 8,
    EarthOsgb1936Datum = 9,
    EarthWgs84CorrectedGeomagnetic = 10,
    SunSphericalRadius695990000 = 11,
    Missing = 255,
}
impl From<u8> for Grib2Table3_2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::EarthSphericalRadius6367470,
            1 => Self::EarthSphericalRadiusSpecifiedByProducer,
            2 => Self::EarthOblateSpheroidIau1965,
            3 => Self::EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerKm,
            4 => Self::EarthOblateSpheroidIagGrs80,
            5 => Self::EarthRepresentedByWgs84,
            6 => Self::EarthSphericalRadius6371229,
            7 => Self::EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerM,
            8 => Self::EarthSphericalRadius6371200Wgs84Datum,
            9 => Self::EarthOsgb1936Datum,
            10 => Self::EarthWgs84CorrectedGeomagnetic,
            11 => Self::SunSphericalRadius695990000,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::EarthSphericalRadius6367470 => {
                "Earth assumed spherical with radius = 6,367,470.0 m"
            }
            Self::EarthSphericalRadiusSpecifiedByProducer => {
                "Earth assumed spherical with radius specified (in m) by data producer"
            }
            Self::EarthOblateSpheroidIau1965 => {
                "Earth assumed oblate spheroid with size as determined by IAU in 1965 (major axis \
                 = 6,378,160.0 m, minor axis = 6,356,775.0 m, f = 1/297.0)"
            }
            Self::EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerKm => {
                "Earth assumed oblate spheroid with major and minor axes specified (in km) by data \
                 producer"
            }
            Self::EarthOblateSpheroidIagGrs80 => {
                "Earth assumed oblate spheroid as defined in IAG-GRS80 model (major axis = \
                 6,378,137.0 m, minor axis = 6,356,752.314 m, f = 1/298.257222101)"
            }
            Self::EarthRepresentedByWgs84 => {
                "Earth assumed represented by WGS84 (as used by ICAO since 1998) (Uses IAG-GRS80 \
                 as a basis)"
            }
            Self::EarthSphericalRadius6371229 => {
                "Earth assumed spherical with radius = 6,371,229.0 m"
            }
            Self::EarthOblateSpheroidMajorMinorAxesSpecifiedByProducerM => {
                "Earth assumed oblate spheroid with major and minor axes specified (in m) by data \
                 producer"
            }
            Self::EarthSphericalRadius6371200Wgs84Datum => {
                "Earth model assumed spherical with radius 6,371,200 m, but the horizontal datum \
                 of the resulting Latitude/Longitude field is the WGS84 reference frame"
            }
            Self::EarthOsgb1936Datum => {
                "Earth represented by the OSGB 1936 Datum, using the Airy_1830 Spheroid, the \
                 Greenwich meridian as 0 Longitude, the Newlyn datum as mean sea level, 0 height."
            }
            Self::EarthWgs84CorrectedGeomagnetic => {
                "Earth model assumed WGS84 with corrected geomagnetic coordinates (latitude and \
                 longitude) defined by Gustafsson et al., 1992\". (see Note 1)"
            }
            Self::SunSphericalRadius695990000 => {
                "Sun assumed spherical with radius = 695 990 000 m (Allen, C.W., Astrophysical \
                 Quantities, 3rd ed.; Athlone: London, 1976) and Stonyhurst latitude and longitude \
                 system with origin at the intersection of the solar central meridian (as seen \
                 from Earth) and the solar equator (Thompson, W., Coordinate systems for solar \
                 image data, Astron. Astrophys. 2006, 449, 791-803)"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.3 - RESOLUTION AND COMPONENT FLAGS
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 55
/// - **Applicable Grid Templates**: 0-3, 40-43
///
/// **Reserved Bits**:
/// - `1-2`: Reserved
/// - `6-8`: Reserved - set to zero
///
/// **Special Values**:
/// - None
///
/// ## Description
/// This table defines the resolution and component flags used in GRIB2 files,
/// specifying various increments and component resolutions for vector quantities.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
///
/// ## Notes
/// - Created 05/11/2005
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grib2Table3_3 {
    /// Bit 3: i Direction Increments
    pub bit3: Grib2Table3_3Bit3,
    /// Bit 4: j Direction Increments
    pub bit4: Grib2Table3_3Bit4,
    /// Bit 5: Resolved Components of Vector Quantities
    pub bit5: Grib2Table3_3Bit5,
    /// If any reserved bits (1-2, 6-8) are set, this field will indicate the original byte value.
    /// Otherwise, it will be 0.
    pub reserved_bits_set: u8,
}
impl From<u8> for Grib2Table3_3 {
    fn from(val: u8) -> Self {
        // Reserved Bits:
        // 1-2 (0-indexed: 0-1)
        // 6-8 (0-indexed: 5-7)
        let reserved_bits_mask: u8 = 0b1110_0011; // Bits 1, 2, 6, 7, 8

        Self {
            bit3: Grib2Table3_3Bit3::from((val >> 2) & 1),
            bit4: Grib2Table3_3Bit4::from((val >> 3) & 1),
            bit5: Grib2Table3_3Bit5::from((val >> 4) & 1),
            reserved_bits_set: val & reserved_bits_mask,
        }
    }
}
impl core::fmt::Display for Grib2Table3_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.reserved_bits_set != 0 {
            write!(
                f,
                "Unknown Resolution and Component Flags (Reserved bits set: {:#010b})",
                self.reserved_bits_set
            )
        } else {
            write!(f, "Bit 3: {}; Bit 4: {}; Bit 5: {}", self.bit3, self.bit4, self.bit5)
        }
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_3Bit3 {
    IDirectionIncrementsNotGiven = 0,
    IDirectionIncrementsGiven = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_3Bit3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::IDirectionIncrementsNotGiven,
            1 => Self::IDirectionIncrementsGiven,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_3Bit3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::IDirectionIncrementsNotGiven => "i direction increments not given",
            Self::IDirectionIncrementsGiven => "i direction increments given",
            Self::Unknown(v) => return write!(f, "Unknown Bit 3 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_3Bit4 {
    JDirectionIncrementsNotGiven = 0,
    JDirectionIncrementsGiven = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_3Bit4 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::JDirectionIncrementsNotGiven,
            1 => Self::JDirectionIncrementsGiven,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_3Bit4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::JDirectionIncrementsNotGiven => "j direction increments not given",
            Self::JDirectionIncrementsGiven => "j direction increments given",
            Self::Unknown(v) => return write!(f, "Unknown Bit 4 value ({v})"),
        };
        f.write_str(desc)
    }
}
/// Enum for Bit 5 of Table 3.3 (Resolved Components of Vector Quantities)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_3Bit5 {
    ResolvedUvComponentsEasterlyNortherly = 0,
    ResolvedUvComponentsGridIncreasingXy = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_3Bit5 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ResolvedUvComponentsEasterlyNortherly,
            1 => Self::ResolvedUvComponentsGridIncreasingXy,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_3Bit5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ResolvedUvComponentsEasterlyNortherly => {
                "Resolved u and v components of vector quantities relative to easterly and \
                 northerly directions"
            }
            Self::ResolvedUvComponentsGridIncreasingXy => {
                "Resolved u and v components of vector quantities relative to the defined grid in \
                 the direction of increasing x and y (or i and j) coordinates, respectively."
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 5 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.4 - SCANNING MODE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 72
/// - **Applicable Grid Templates**: 0-3, 40-43, 204
///
/// **Reserved Bits**:
/// - None
///
/// **Special Values**:
/// - None
///
/// ## Description
/// This table defines the scanning mode flags used in GRIB2 files,
/// specifying the scanning direction and row/column offsets.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
///
/// ## Notes
/// - (1).  i direction - West to east along a parallel or left to right along an x-axis.
/// - (2).  j direction - South to north along a meridian, or bottom to top along a y-axis.
/// - (3).  If bit number 4 is set, the first row scan is defined by previous flags.
/// - (4).  La1 and Lo1 define the first row, which is an odd row.
/// - (5).  Di and Dj are assumed to be positive, with the direction of i and j being given by bits 1 and 2.
/// - (6).  Bits 5 through 8 may be used to generate staggered grids, such as Arakawa grids (see Attachment, Volume 1.2, Part A, Att. GRIB).
/// - (7).  If any of bits 5, 6, 7 or 8 are set, Di and Dj are not optional.
///
/// This table defines individual bit flags. To use them, you will need to extract the
/// relevant bit from the byte at Octet 72 (index 71) and convert it using the
/// corresponding `From<u8>` implementation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grib2Table3_4 {
    pub bit1: Grib2Table3_4Bit1,
    pub bit2: Grib2Table3_4Bit2,
    pub bit3: Grib2Table3_4Bit3,
    pub bit4: Grib2Table3_4Bit4,
    pub bit5: Grib2Table3_4Bit5,
    pub bit6: Grib2Table3_4Bit6,
    pub bit7: Grib2Table3_4Bit7,
    pub bit8: Grib2Table3_4Bit8,
}
impl From<u8> for Grib2Table3_4 {
    fn from(val: u8) -> Self {
        Grib2Table3_4 {
            bit1: Grib2Table3_4Bit1::from(val & 1),
            bit2: Grib2Table3_4Bit2::from((val >> 1) & 1),
            bit3: Grib2Table3_4Bit3::from((val >> 2) & 1),
            bit4: Grib2Table3_4Bit4::from((val >> 3) & 1),
            bit5: Grib2Table3_4Bit5::from((val >> 4) & 1),
            bit6: Grib2Table3_4Bit6::from((val >> 5) & 1),
            bit7: Grib2Table3_4Bit7::from((val >> 6) & 1),
            bit8: Grib2Table3_4Bit8::from((val >> 7) & 1),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Bit 1: {}; Bit 2: {}; Bit 3: {}; Bit 4: {}; Bit 5: {}; Bit 6: {}; Bit 7: {}; Bit 8: \
             {}",
            self.bit1, self.bit2, self.bit3, self.bit4, self.bit5, self.bit6, self.bit7, self.bit8
        )
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit1 {
    PointsFirstRowColumnScanPlusIDirection = 0,
    PointsFirstRowColumnScanMinusIDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsFirstRowColumnScanPlusIDirection,
            1 => Self::PointsFirstRowColumnScanMinusIDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsFirstRowColumnScanPlusIDirection => {
                "Points in the first row or column scan in the +i (+x) direction"
            }
            Self::PointsFirstRowColumnScanMinusIDirection => {
                "Points in the first row or column scan in the -i (-x) direction"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 1 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit2 {
    PointsFirstRowColumnScanMinusJDirection = 0,
    PointsFirstRowColumnScanPlusJDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsFirstRowColumnScanMinusJDirection,
            1 => Self::PointsFirstRowColumnScanPlusJDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsFirstRowColumnScanMinusJDirection => {
                "Points in the first row or column scan in the -j (-y) direction"
            }
            Self::PointsFirstRowColumnScanPlusJDirection => {
                "Points in the first row or column scan in the +j (+y) direction"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 2 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit3 {
    AdjacentPointsIDirectionConsecutive = 0,
    AdjacentPointsJDirectionConsecutive = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AdjacentPointsIDirectionConsecutive,
            1 => Self::AdjacentPointsJDirectionConsecutive,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AdjacentPointsIDirectionConsecutive => {
                "Adjacent points in the i (x) direction are consecutive"
            }
            Self::AdjacentPointsJDirectionConsecutive => {
                "Adjacent points in the j (y) direction are consecutive"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 3 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit4 {
    AllRowsScanSameDirection = 0,
    AdjacentRowsScanOppositeDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit4 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AllRowsScanSameDirection,
            1 => Self::AdjacentRowsScanOppositeDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AllRowsScanSameDirection => "All rows scan in the same direction",
            Self::AdjacentRowsScanOppositeDirection => {
                "Adjacent rows scan in the opposite direction"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 4 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit5 {
    PointsOddRowsNotOffsetIDirection = 0,
    PointsOddRowsOffsetDi2IDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit5 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsOddRowsNotOffsetIDirection,
            1 => Self::PointsOddRowsOffsetDi2IDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsOddRowsNotOffsetIDirection => {
                "Points within odd rows are not offset in i(x) direction"
            }
            Self::PointsOddRowsOffsetDi2IDirection => {
                "Points within odd rows are offset by Di/2 in i(x) direction"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 5 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit6 {
    PointsEvenRowsNotOffsetIDirection = 0,
    PointsEvenRowsOffsetDi2IDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit6 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsEvenRowsNotOffsetIDirection,
            1 => Self::PointsEvenRowsOffsetDi2IDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsEvenRowsNotOffsetIDirection => {
                "Points within even rows are not offset in i(x) direction"
            }
            Self::PointsEvenRowsOffsetDi2IDirection => {
                "Points within even rows are offset by Di/2 in i(x) direction"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 6 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit7 {
    PointsNotOffsetJDirection = 0,
    PointsOffsetDj2JDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit7 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsNotOffsetJDirection,
            1 => Self::PointsOffsetDj2JDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsNotOffsetJDirection => "Points are not offset in j(y) direction",
            Self::PointsOffsetDj2JDirection => "Points are offset by Dj/2 in j(y) direction",
            Self::Unknown(v) => return write!(f, "Unknown Bit 7 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_4Bit8 {
    RowsNiColumnsNjGridPoints = 0,
    RowsNiOrNiMinus1ColumnsNjOrNjMinus1GridPoints = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_4Bit8 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::RowsNiColumnsNjGridPoints,
            1 => Self::RowsNiOrNiMinus1ColumnsNjOrNjMinus1GridPoints,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_4Bit8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::RowsNiColumnsNjGridPoints => {
                "Rows have Ni grid points and columns have Nj grid points"
            }
            Self::RowsNiOrNiMinus1ColumnsNjOrNjMinus1GridPoints => {
                "Rows have Ni grid points if points are not offset in i direction; Rows have Ni-1 \
                 grid points if points are offset by Di/2 in i direction. Columns have Nj grid \
                 points if points are not offset in j direction; Columns have Nj-1 grid points if \
                 points are offset by Dj/2 in j(y) direction."
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 8 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.5 - PROJECTION CENTER
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 55
/// - **Applicable Grid Templates**: 20, 30, 31
///
/// **Reserved Bits**:
/// - `3-8`: Reserved
///
/// **Special Values**:
/// - None
///
/// ## Description
/// This table defines the projection center flags used in GRIB2 files,
/// specifying the pole location and projection type.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml)
///
/// ## Notes
/// - Created 05/11/2005
///
/// This table defines individual bit flags. To use them, you will need to extract the
/// relevant bit from the byte at Octet 55 (index 54) and convert it using the
/// corresponding `From<u8>` implementation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grib2Table3_5 {
    /// Bit 1: North Pole on projection plane (0) or South Pole on projection plane (1).
    pub bit1: Grib2Table3_5Bit1,
    /// Bit 2: Only one projection center used (0) or projection is bi-polar and symmetric (1).
    pub bit2: Grib2Table3_5Bit2,
    /// If any reserved bits (3-8) are set, this field will indicate the original byte value.
    /// Otherwise, it will be 0.
    pub reserved_bits_set: u8,
}
impl From<u8> for Grib2Table3_5 {
    fn from(val: u8) -> Self {
        // Reserved Bits: 3-8 (0-indexed: 2-7) must be zero.
        let reserved_bits_mask: u8 = 0b1111_1100; // Bits 3, 4, 5, 6, 7, 8

        Self {
            bit1: Grib2Table3_5Bit1::from(val & 1),
            bit2: Grib2Table3_5Bit2::from((val >> 1) & 1),
            reserved_bits_set: val & reserved_bits_mask,
        }
    }
}
impl core::fmt::Display for Grib2Table3_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.reserved_bits_set != 0 {
            write!(
                f,
                "Unknown Projection Center (Reserved bits set: {:#010b})",
                self.reserved_bits_set
            )
        } else {
            write!(f, "Bit 1: {}; Bit 2: {}", self.bit1, self.bit2)
        }
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_5Bit1 {
    NorthPoleOnProjectionPlane = 0,
    SouthPoleOnProjectionPlane = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_5Bit1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NorthPoleOnProjectionPlane,
            1 => Self::SouthPoleOnProjectionPlane,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_5Bit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NorthPoleOnProjectionPlane => "North Pole is on the projection plane",
            Self::SouthPoleOnProjectionPlane => "South Pole is on the projection plane",
            Self::Unknown(v) => return write!(f, "Unknown Bit 1 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_5Bit2 {
    OnlyOneProjectionCenterUsed = 0,
    ProjectionBiPolarAndSymmetric = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_5Bit2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::OnlyOneProjectionCenterUsed,
            1 => Self::ProjectionBiPolarAndSymmetric,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_5Bit2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::OnlyOneProjectionCenterUsed => "Only one projection center is used",
            Self::ProjectionBiPolarAndSymmetric => "Projection is bi-polar and symmetric",
            Self::Unknown(v) => return write!(f, "Unknown Bit 2 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.6 - SPECTRAL DATA REPRESENTATION TYPE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: [Not Specified]
///
/// **Reserved Ranges**:
/// - `3-254`: Reserved
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the spectral data representation types used in GRIB2 files,
/// specifying the mathematical representations employed for spectral data.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-6.shtml)
///
/// ## Notes
/// - Revised 08/23/2023
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_6 {
    AssociatedLegendreFunctionsFirstKind = 1,
    BiFourierRepresentation = 2,
    Missing = 255,
}

impl From<u8> for Grib2Table3_6 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::AssociatedLegendreFunctionsFirstKind,
            2 => Self::BiFourierRepresentation,
            _ => Self::Missing,
        }
    }
}

impl core::fmt::Display for Grib2Table3_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AssociatedLegendreFunctionsFirstKind => {
                "The Associated Legendre Functions of the first kind are defined by:"
            }
            Self::BiFourierRepresentation => "Bi-Fourier representation",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.7 - SPECTRAL DATA REPRESENTATION MODE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 55
///
/// **Reserved Ranges**:
/// - `2-254`: Reserved
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the spectral data representation modes used in GRIB2 files,
/// specifying how spectral data is represented, including the mathematical representations employed.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-6.shtml)
///
/// ## Notes
/// - (1) Values of N(m) for common truncation cases are as follows:
///   - Triangular:     M = J = K,        N(m) = J
///   - Rhomboidal:     K = J + M,        N(m) = J + m
///   - Trapezoidal:    K = J, K > M,     N(m) = J
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_7 {
    ComplexNumbersFnmStoredAsPairsOfRealNumbers = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table3_7 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::ComplexNumbersFnmStoredAsPairsOfRealNumbers,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ComplexNumbersFnmStoredAsPairsOfRealNumbers => {
                "The complex numbers Fnm (See Code Table 3.6) are stored for M>=0 as pairs of real \
                 numbers Re(Fnm), lm(Fnm) ordered with n increasing from m to N(m), first for m=0 \
                 and then for m=1, 2, ... M (see note below)."
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.8 - GRID POINT POSITION
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 32
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the grid point positions used in GRIB2 files,
/// specifying where grid points are located relative to grid shapes.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-8.shtml)
///
/// ## Notes
/// - Revised 12/07/2023
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_8 {
    GridPointsAtTriangleVertices = 0,
    GridPointsAtCentersOfTriangles = 1,
    GridPointsAtMidpointsOfTriangleSides = 2,
    GridPointsAtShapeVertices = 3,
    GridPointsAtCentreOfShapes = 4,
    GridPointsAtMidpointsOfShapeSides = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table3_8 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::GridPointsAtTriangleVertices,
            1 => Self::GridPointsAtCentersOfTriangles,
            2 => Self::GridPointsAtMidpointsOfTriangleSides,
            3 => Self::GridPointsAtShapeVertices,
            4 => Self::GridPointsAtCentreOfShapes,
            5 => Self::GridPointsAtMidpointsOfShapeSides,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::GridPointsAtTriangleVertices => "Grid points at triangle vertices",
            Self::GridPointsAtCentersOfTriangles => "Grid points at centers of triangles",
            Self::GridPointsAtMidpointsOfTriangleSides => {
                "Grid points at midpoints of triangle sides"
            }
            Self::GridPointsAtShapeVertices => "Grid points at shape vertices",
            Self::GridPointsAtCentreOfShapes => "Grid points at centre of shapes",
            Self::GridPointsAtMidpointsOfShapeSides => "Grid points at midpoints of shape sides",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.9 - GRID POINT POSITION AS SEEN FROM THE CORRESPONDING POLE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 33
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Bits**:
/// - `2-8`: Reserved
///
/// **Special Values**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the grid point positions as seen from the corresponding pole in GRIB2 files,
/// specifying where grid points are located relative to grid shapes.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-9.shtml)
///
/// ## Notes
/// - Revised 12/07/2023
///
/// This table defines individual bit flags. To use them, you will need to extract the
/// relevant bit from the byte at Octet 33 (index 32) and convert it using the
/// corresponding `From<u8>` implementation. For example, to get the meaning of Bit 1,
/// you would do `Grib2Table3_9Bit1::from((octet_33 >> 0) & 1)`.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_9Bit1 {
    ClockwiseOrientation = 0,
    CounterClockwiseOrientation = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_9Bit1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ClockwiseOrientation,
            1 => Self::CounterClockwiseOrientation,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_9Bit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ClockwiseOrientation => "Clockwise orientation",
            Self::CounterClockwiseOrientation => "Counter-clockwise orientation",
            Self::Unknown(v) => return write!(f, "Unknown Bit 1 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.10 - SCANNING MODE FOR ONE DIAMOND AS SEEN FROM THE CORRESPONDING POLE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 34
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Bits**:
/// - `4-8`: Reserved
///
/// **Special Values**:
/// - None
///
/// ## Description
/// This table defines the scanning mode flags for one diamond as seen from the corresponding pole in GRIB2 files,
/// specifying the scanning directions and grid points alignment.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-10.shtml)
///
/// ## Notes
/// - Created 05/11/2005
///
/// This table defines individual bit flags. To use them, you will need to extract the
/// relevant bit from the byte at Octet 34 (index 33) and convert it using the
/// corresponding `From<u8>` implementation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grib2Table3_10 {
    /// Bit 1: Points scan in +i (pole to Equator) (0) or -i (Equator to pole) (1).
    pub bit1: Grib2Table3_10Bit1,
    /// Bit 2: Points scan in +j (west to east) (0) or -j (east to west) (1).
    pub bit2: Grib2Table3_10Bit2,
    /// Bit 3: Adjacent points in i direction are consecutive (0) or j direction are consecutive (1).
    pub bit3: Grib2Table3_10Bit3,
    /// If any reserved bits (4-8) are set, this field will indicate the original byte value.
    /// Otherwise, it will be 0.
    pub reserved_bits_set: u8,
}
impl From<u8> for Grib2Table3_10 {
    fn from(val: u8) -> Self {
        // Reserved Bits: 4-8 (0-indexed: 3-7) must be zero.
        let reserved_bits_mask: u8 = 0b1111_1000; // Bits 4, 5, 6, 7, 8

        Self {
            bit1: Grib2Table3_10Bit1::from(val & 1),
            bit2: Grib2Table3_10Bit2::from((val >> 1) & 1),
            bit3: Grib2Table3_10Bit3::from((val >> 2) & 1),
            reserved_bits_set: val & reserved_bits_mask,
        }
    }
}
impl core::fmt::Display for Grib2Table3_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.reserved_bits_set != 0 {
            write!(
                f,
                "Unknown Scanning Mode for One Diamond (Reserved bits set: {:#010b})",
                self.reserved_bits_set
            )
        } else {
            write!(f, "Bit 1: {}; Bit 2: {}; Bit 3: {}", self.bit1, self.bit2, self.bit3)
        }
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_10Bit1 {
    PointsScanPlusIDirectionPoleToEquator = 0,
    PointsScanMinusIDirectionEquatorToPole = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_10Bit1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsScanPlusIDirectionPoleToEquator,
            1 => Self::PointsScanMinusIDirectionEquatorToPole,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_10Bit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsScanPlusIDirectionPoleToEquator => {
                "Points scan in the +i direction, i.e. from pole to Equator"
            }
            Self::PointsScanMinusIDirectionEquatorToPole => {
                "Points scan in the -i direction, i.e. from Equator to pole"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 1 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_10Bit2 {
    PointsScanPlusJDirectionWestToEast = 0,
    PointsScanMinusJDirectionEastToWest = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_10Bit2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsScanPlusJDirectionWestToEast,
            1 => Self::PointsScanMinusJDirectionEastToWest,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_10Bit2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsScanPlusJDirectionWestToEast => {
                "Points scan in the +j direction, i.e. from west to east"
            }
            Self::PointsScanMinusJDirectionEastToWest => {
                "Points scan in the -j direction, i.e. from east to west"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 2 value ({v})"),
        };
        f.write_str(desc)
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_10Bit3 {
    AdjacentPointsIDirectionConsecutive = 0,
    AdjacentPointsJDirectionConsecutive = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_10Bit3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AdjacentPointsIDirectionConsecutive,
            1 => Self::AdjacentPointsJDirectionConsecutive,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_10Bit3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AdjacentPointsIDirectionConsecutive => {
                "Adjacent points in the i direction are consecutive"
            }
            Self::AdjacentPointsJDirectionConsecutive => {
                "Adjacent points in the j direction are consecutive"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 3 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.11 - Interpretation of List of Numbers at End of Section 3
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 12
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Ranges**:
/// - `4-254`: Reserved
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the interpretation of the list of numbers appended at the end of Section 3 in GRIB2 files,
/// specifying how the numbers correspond to points in the grid based on various definitions.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-11.shtml)
///
/// ## Notes
/// - (1) For entry 1, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, effective number of points per row may be less than the number of points on the current circle.
/// - (2) For value for the constant direction increment Di (or Dx) in the accompanying Grid Definition Template should be set to all ones (missing).
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_11 {
    NoAppendedList = 0,
    FullCoordinateCircles = 1,
    ExtremeCoordinateValues = 2,
    ActualLatitudesForEachRow = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table3_11 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoAppendedList,
            1 => Self::FullCoordinateCircles,
            2 => Self::ExtremeCoordinateValues,
            3 => Self::ActualLatitudesForEachRow,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoAppendedList => "There is no appended list",
            Self::FullCoordinateCircles => {
                "Numbers define number of points corresponding to full coordinate circles (i.e. \
                 parallels). Coordinate values on each circle are multiple of the circle mesh, and \
                 extreme coordinate values given in grid definition may not be reached in all rows."
            }
            Self::ExtremeCoordinateValues => {
                "Numbers define number of points corresponding to coordinate lines delimited by \
                 extreme coordinate values given in grid definition which are present in each row."
            }
            Self::ActualLatitudesForEachRow => {
                "Numbers define the actual latitudes for each row in the grid. The list of numbers \
                 are integer values of the valid latitudes in microdegrees (scale by 106) or in \
                 unit equal to the ratio of the basic angle and the subdivisions number for each \
                 row, in the same order as specified in the \"scanning mode flag\" (bit no. 2) \
                 (see note 2)"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.12 - HEALPix Rhomboids or Points Ordering
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 34
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the ordering of HEALPix rhomboids or points in GRIB2 files,
/// specifying how points are ordered within the HEALPix grid structure.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-12.shtml)
///
/// ## Notes
/// - Created 12/07/2023
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_12 {
    Reserved0 = 0, // Explicitly reserved value
    RingOrdering = 1,
    NestedOrdering = 2,
    Missing = 255,
}

impl From<u8> for Grib2Table3_12 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved0,
            1 => Self::RingOrdering,
            2 => Self::NestedOrdering,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved0 => "Reserved",
            Self::RingOrdering => "Ring ordering",
            Self::NestedOrdering => "Nested ordering",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.13 - HEALPix Scanning Mode
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 34
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Bits**:
/// - `4-8`: Reserved
///
/// **Special Value**:
/// - None
///
/// ## Description
/// This table defines the HEALPix scanning mode flags used in GRIB2 files,
/// specifying the scanning directions and grid points alignment.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-13.shtml)
///
/// ## Notes
/// - Created 12/07/2023
///
/// This table defines individual bit flags. To use them, you will need to extract the
/// relevant bit from the byte at Octet 34 (index 33) and convert it using the
/// corresponding `From<u8>` implementation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grib2Table3_13 {
    /// Bit 1: Points scan in the +i (+x) direction (0) or -i (-x) direction (1).
    pub bit1: Grib2Table3_13Bit1,
    /// Bit 2: Points scan in -j (-y) direction (0) or +j (+y) direction (1).
    pub bit2: Grib2Table3_13Bit2,
    /// Bit 3: Adjacent points in the i (x) direction are consecutive (0) or j (y) direction are consecutive (1).
    pub bit3: Grib2Table3_13Bit3,
    /// If any reserved bits (4-8) are set, this field will indicate the original byte value.
    /// Otherwise, it will be 0.
    pub reserved_bits_set: u8,
}
impl From<u8> for Grib2Table3_13 {
    fn from(val: u8) -> Self {
        // Reserved Bits: 4-8 (0-indexed: 3-7) must be zero.
        let reserved_bits_mask: u8 = 0b1111_1000; // Bits 4, 5, 6, 7, 8

        Self {
            bit1: Grib2Table3_13Bit1::from(val & 1),
            bit2: Grib2Table3_13Bit2::from((val >> 1) & 1),
            bit3: Grib2Table3_13Bit3::from((val >> 2) & 1),
            reserved_bits_set: val & reserved_bits_mask,
        }
    }
}
impl core::fmt::Display for Grib2Table3_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.reserved_bits_set != 0 {
            write!(
                f,
                "Unknown HEALPix Scanning Mode (Reserved bits set: {:#010b})",
                self.reserved_bits_set
            )
        } else {
            write!(f, "Bit 1: {}; Bit 2: {}; Bit 3: {}", self.bit1, self.bit2, self.bit3)
        }
    }
}
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_13Bit1 {
    PointsScanPlusIDirection = 0,
    PointsScanMinusIDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_13Bit1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsScanPlusIDirection,
            1 => Self::PointsScanMinusIDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_13Bit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsScanPlusIDirection => "Points scan in the +i (+x) direction",
            Self::PointsScanMinusIDirection => "Points scan in the -i (-x) direction",
            Self::Unknown(v) => return write!(f, "Unknown Bit 1 value ({v})"),
        };
        f.write_str(desc)
    }
}

#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_13Bit2 {
    PointsScanMinusJDirection = 0,
    PointsScanPlusJDirection = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_13Bit2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::PointsScanMinusJDirection,
            1 => Self::PointsScanPlusJDirection,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_13Bit2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::PointsScanMinusJDirection => "Points scan in -j (-y) direction",
            Self::PointsScanPlusJDirection => "Points scan in +j (+y) direction",
            Self::Unknown(v) => return write!(f, "Unknown Bit 2 value ({v})"),
        };
        f.write_str(desc)
    }
}

#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_13Bit3 {
    AdjacentPointsIDirectionConsecutive = 0,
    AdjacentPointsJDirectionConsecutive = 1,
    Unknown(u8),
}
impl From<u8> for Grib2Table3_13Bit3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AdjacentPointsIDirectionConsecutive,
            1 => Self::AdjacentPointsJDirectionConsecutive,
            other => Self::Unknown(other),
        }
    }
}
impl core::fmt::Display for Grib2Table3_13Bit3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AdjacentPointsIDirectionConsecutive => {
                "Adjacent points in the i (x) direction are consecutive"
            }
            Self::AdjacentPointsJDirectionConsecutive => {
                "Adjacent points in the j (y) direction are consecutive"
            }
            Self::Unknown(v) => return write!(f, "Unknown Bit 3 value ({v})"),
        };
        f.write_str(desc)
    }
}

/// # Table 3.15 - PHYSICAL MEANING OF VERTICAL COORDINATE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 63
/// - **Applicable Grid Templates**: 100
///
/// **Reserved Ranges**:
/// - `0-19`: Reserved
/// - `21-99`: Reserved
/// - `114-159`: Reserved
/// - `161-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the physical meanings of vertical coordinates used in GRIB2 files,
/// specifying various vertical coordinate systems and their corresponding units.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-15.shtml)
///
/// ## Notes
/// - (1) For entry 103, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, the effective number of points per row may be less than the number of points on the current circle.
/// - (2) For the value of the constant direction increment Di (or Dx) in the accompanying Grid Definition Template, it should be set to all ones (missing).
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_15 {
    Reserved0 = 0, // Explicitly reserved value
    TemperatureK = 20,
    PressurePa = 100,
    PressureDeviationFromMeanSeaLevelPa = 101,
    AltitudeAboveMeanSeaLevelM = 102,
    HeightAboveGroundM = 103,
    SigmaCoordinate = 104,
    HybridCoordinate = 105,
    DepthBelowLandSurfaceM = 106,
    PotentialTemperatureK = 107,
    PressureDeviationFromGroundToLevelPa = 108,
    PotentialVorticityKgm2s1 = 109,
    GeometricHeightM = 110,
    EtaCoordinate = 111,
    GeopotentialHeightGpm = 112,
    LogarithmicHybridCoordinate = 113,
    DepthBelowSeaLevelM = 160,
    Missing = 255,
}
impl From<u8> for Grib2Table3_15 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved0,
            20 => Self::TemperatureK,
            100 => Self::PressurePa,
            101 => Self::PressureDeviationFromMeanSeaLevelPa,
            102 => Self::AltitudeAboveMeanSeaLevelM,
            103 => Self::HeightAboveGroundM,
            104 => Self::SigmaCoordinate,
            105 => Self::HybridCoordinate,
            106 => Self::DepthBelowLandSurfaceM,
            107 => Self::PotentialTemperatureK,
            108 => Self::PressureDeviationFromGroundToLevelPa,
            109 => Self::PotentialVorticityKgm2s1,
            110 => Self::GeometricHeightM,
            111 => Self::EtaCoordinate,
            112 => Self::GeopotentialHeightGpm,
            113 => Self::LogarithmicHybridCoordinate,
            160 => Self::DepthBelowSeaLevelM,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved0 => "Reserved",
            Self::TemperatureK => "Temperature (K)",
            Self::PressurePa => "Pressure (Pa)",
            Self::PressureDeviationFromMeanSeaLevelPa => {
                "Pressure deviation from mean sea level (Pa)"
            }
            Self::AltitudeAboveMeanSeaLevelM => "Altitude above mean sea level (m)",
            Self::HeightAboveGroundM => "Height above ground (see note 1) (m)",
            Self::SigmaCoordinate => "Sigma coordinate",
            Self::HybridCoordinate => "Hybrid coordinate",
            Self::DepthBelowLandSurfaceM => "Depth below land surface (m)",
            Self::PotentialTemperatureK => "Potential temperature (theta) (K)",
            Self::PressureDeviationFromGroundToLevelPa => {
                "Pressure deviation from ground to level (Pa)"
            }
            Self::PotentialVorticityKgm2s1 => "Potential vorticity (K m-2 kg-1 s-1)",
            Self::GeometricHeightM => "Geometric height (m)",
            Self::EtaCoordinate => "Eta coordinate (see note 2)",
            Self::GeopotentialHeightGpm => "Geopotential height (gpm)",
            Self::LogarithmicHybridCoordinate => "Logarithmic hybrid coordinate",
            Self::DepthBelowSeaLevelM => "Depth below sea level (m)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.20 - TYPE OF HORIZONTAL LINE AS SEEN FROM THE CORRESPONDING POLE
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 60
/// - **Applicable Grid Templates**: 1000, 1100
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the types of horizontal lines used in GRIB2 files,
/// specifying whether lines are Rhumb or Great Circle, among other definitions.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-20.shtml)
///
/// ## Notes
/// - Created 05/11/2005
/// - Red text in the original table depicts changes made since 05/11/2005.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_20 {
    Rhumb = 0,
    GreatCircle = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table3_20 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Rhumb,
            1 => Self::GreatCircle,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Rhumb => "Rhumb",
            Self::GreatCircle => "Great Circle",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.21 - PHYSICAL MEANING OF VERTICAL COORDINATE VALUES DEFINITION
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: 64
/// - **Applicable Grid Templates**: 1000
///
/// **Reserved Ranges**:
/// - `0-19`: Reserved
/// - `21-99`: Reserved
/// - `114-159`: Reserved
/// - `161-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the physical meanings of vertical coordinates used in GRIB2 files,
/// specifying various vertical coordinate systems and their corresponding units.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-21.shtml)
///
/// ## Notes
/// - (1) For entry 103, it should be noted that depending on values of extreme (first/last) coordinates, and regardless of bit-map, the effective number of points per row may be less than the number of points on the current circle.
/// - (2) For the value for the constant direction increment Di (or Dx) in the accompanying Grid Definition Template should be set to all ones (missing).
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_21 {
    TemperatureK = 20,
    PressurePa = 100,
    PressureDeviationFromMeanSeaLevelPa = 101,
    AltitudeAboveMeanSeaLevelM = 102,
    HeightAboveGroundM = 103,
    SigmaCoordinate = 104,
    HybridCoordinate = 105,
    DepthBelowLandSurfaceM = 106,
    PotentialTemperatureK = 107,
    PressureDeviationFromGroundToLevelPa = 108,
    PotentialVorticityKgm2s1 = 109,
    GeometricHeightM = 110,
    EtaCoordinate = 111,
    GeopotentialHeightGpm = 112,
    LogarithmicHybridCoordinate = 113,
    DepthBelowSeaLevelM = 160,
    Missing = 255,
}
impl From<u8> for Grib2Table3_21 {
    fn from(val: u8) -> Self {
        match val {
            20 => Self::TemperatureK,
            100 => Self::PressurePa,
            101 => Self::PressureDeviationFromMeanSeaLevelPa,
            102 => Self::AltitudeAboveMeanSeaLevelM,
            103 => Self::HeightAboveGroundM,
            104 => Self::SigmaCoordinate,
            105 => Self::HybridCoordinate,
            106 => Self::DepthBelowLandSurfaceM,
            107 => Self::PotentialTemperatureK,
            108 => Self::PressureDeviationFromGroundToLevelPa,
            109 => Self::PotentialVorticityKgm2s1,
            110 => Self::GeometricHeightM,
            111 => Self::EtaCoordinate,
            112 => Self::GeopotentialHeightGpm,
            113 => Self::LogarithmicHybridCoordinate,
            160 => Self::DepthBelowSeaLevelM,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::TemperatureK => "Temperature (K)",
            Self::PressurePa => "Pressure (Pa)",
            Self::PressureDeviationFromMeanSeaLevelPa => {
                "Pressure deviation from mean sea level (Pa)"
            }
            Self::AltitudeAboveMeanSeaLevelM => "Altitude above mean sea level (m)",
            Self::HeightAboveGroundM => "Height above ground (see note 1) (m)",
            Self::SigmaCoordinate => "Sigma coordinate",
            Self::HybridCoordinate => "Hybrid coordinate",
            Self::DepthBelowLandSurfaceM => "Depth below land surface (m)",
            Self::PotentialTemperatureK => "Potential temperature (theta) (K)",
            Self::PressureDeviationFromGroundToLevelPa => {
                "Pressure deviation from ground to level (Pa)"
            }
            Self::PotentialVorticityKgm2s1 => "Potential vorticity (K m-2 kg-1 s-1)",
            Self::GeometricHeightM => "Geometric height (m)",
            Self::EtaCoordinate => "Eta coordinate (see note 2)",
            Self::GeopotentialHeightGpm => "Geopotential height (gpm)",
            Self::LogarithmicHybridCoordinate => "Logarithmic hybrid coordinate",
            Self::DepthBelowSeaLevelM => "Depth below sea level (m)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 3.25 - TYPE OF BI-FOURIER TRUNCATION
///
/// **Details**:
/// - **Section**: 3
/// - **Octet**: [Not Specified]
/// - **Applicable Grid Templates**: [Not Specified]
///
/// **Reserved Ranges**:
/// - `0-76`: Reserved
/// - `78-87`: Reserved
/// - `89-98`: Reserved
/// - `100-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the types of Bi-Fourier truncation used in GRIB2 files,
/// specifying how spectral data is truncated in the horizontal direction.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-25.shtml)
///
/// ## Notes
/// - Created 06/22/2022
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table3_25 {
    Rectangular = 77,
    Elliptic = 88,
    Diamond = 99,
    Missing = 255,
}
impl From<u8> for Grib2Table3_25 {
    fn from(val: u8) -> Self {
        match val {
            77 => Self::Rectangular,
            88 => Self::Elliptic,
            99 => Self::Diamond,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table3_25 {
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
