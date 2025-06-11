use alloc::string::String;
use alloc::vec::Vec;

/// TIFF Field Types
pub enum FieldTypes {
    /// Byte
    BYTE = 0x0001,
    /// ASCII
    ASCII = 0x0002,
    /// Short
    SHORT = 0x0003,
    /// Long
    LONG = 0x0004,
    /// Rational
    RATIONAL = 0x0005,
    /// Signed byte
    SBYTE = 0x0006,
    /// Undefined
    UNDEFINED = 0x0007,
    /// Signed short
    SSHORT = 0x0008,
    /// Signed long
    SLONG = 0x0009,
    /// Signed rational
    SRATIONAL = 0x000a,
    /// Float
    FLOAT = 0x000b,
    /// Double
    DOUBLE = 0x000c,
    /// IFD offset, suggested by https://owl.phy.queensu.ca/~phil/exiftool/standards.html
    IFD = 0x000d,
    // introduced by BigTIFF
    /// Long
    LONG8 = 0x0010,
    /// Signed long
    SLONG8 = 0x0011,
    /// IFD offset
    IFD8 = 0x0012,
}
impl From<&str> for FieldTypes {
    fn from(value: &str) -> Self {
        match value {
            "BYTE" => FieldTypes::BYTE,
            "ASCII" => FieldTypes::ASCII,
            "SHORT" => FieldTypes::SHORT,
            "LONG" => FieldTypes::LONG,
            "RATIONAL" => FieldTypes::RATIONAL,
            "SBYTE" => FieldTypes::SBYTE,
            "SSHORT" => FieldTypes::SSHORT,
            "SLONG" => FieldTypes::SLONG,
            "SRATIONAL" => FieldTypes::SRATIONAL,
            "FLOAT" => FieldTypes::FLOAT,
            "DOUBLE" => FieldTypes::DOUBLE,
            "IFD" => FieldTypes::IFD,
            "LONG8" => FieldTypes::LONG8,
            "SLONG8" => FieldTypes::SLONG8,
            "IFD8" => FieldTypes::IFD8,
            _ => FieldTypes::UNDEFINED,
        }
    }
}

/// TIFF Photometric Interpretations
pub enum PhotometricInterpretations {
    /// White is zero
    WhiteIsZero = 0,
    /// Black is zero
    BlackIsZero = 1,
    /// RGB
    RGB = 2,
    /// Palette
    Palette = 3,
    /// Transparency mask
    TransparencyMask = 4,
    /// CMYK
    CMYK = 5,
    /// YCbCr
    YCbCr = 6,

    /// CIELab
    CIELab = 8,
    /// ICCLab
    ICCLab = 9,
    /// ITULab
    ITULab = 10,
}

/// TIFF Extra Samples
pub enum ExtraSamplesValues {
    /// Unspecified
    Unspecified = 0,
    /// Associated alpha
    Assocalpha = 1,
    /// Unassociated alpha
    Unassalpha = 2,
}

/// LERC Parameters
pub enum LercParameters {
    /// LERC version
    Version = 0,
    /// Add compression
    AddCompression = 1,
}

/// LERC Add Compression
pub enum LercAddCompression {
    /// No compression
    None = 0,
    /// Deflate
    Deflate = 1,
    /// Zstandard
    Zstandard = 2,
}

/// TIFF Array Fields
pub const ARRAY_FIELDS: [u16; 9] = [
    0x0102, // BitsPerSample
    0x0152, // ExtraSamples
    0x0153, // SampleFormat
    0x0117, // StripByteCounts
    0x0111, // StripOffsets
    0x022f, // StripRowCounts
    0x0145, // TileByteCounts
    0x0144, // TileOffsets
    0x014a, // SubIFDs
];

/// Find the name of a TIFF Field Tag Name
pub fn field_tag_name(key: u16) -> Option<&'static str> {
    match key {
        // TIFF Baseline
        0x013b => Some("Artist"),
        0x0102 => Some("BitsPerSample"),
        0x0109 => Some("CellLength"),
        0x0108 => Some("CellWidth"),
        0x0140 => Some("ColorMap"),
        0x0103 => Some("Compression"),
        0x8298 => Some("Copyright"),
        0x0132 => Some("DateTime"),
        0x0152 => Some("ExtraSamples"),
        0x010a => Some("FillOrder"),
        0x0121 => Some("FreeByteCounts"),
        0x0120 => Some("FreeOffsets"),
        0x0123 => Some("GrayResponseCurve"),
        0x0122 => Some("GrayResponseUnit"),
        0x013c => Some("HostComputer"),
        0x010e => Some("ImageDescription"),
        0x0101 => Some("ImageLength"),
        0x0100 => Some("ImageWidth"),
        0x010f => Some("Make"),
        0x0119 => Some("MaxSampleValue"),
        0x0118 => Some("MinSampleValue"),
        0x0110 => Some("Model"),
        0x00fe => Some("NewSubfileType"),
        0x0112 => Some("Orientation"),
        0x0106 => Some("PhotometricInterpretation"),
        0x011c => Some("PlanarConfiguration"),
        0x0128 => Some("ResolutionUnit"),
        0x0116 => Some("RowsPerStrip"),
        0x0115 => Some("SamplesPerPixel"),
        0x0131 => Some("Software"),
        0x0117 => Some("StripByteCounts"),
        0x0111 => Some("StripOffsets"),
        0x00ff => Some("SubfileType"),
        0x0107 => Some("Threshholding"),
        0x011a => Some("XResolution"),
        0x011b => Some("YResolution"),

        // TIFF Extended
        0x0146 => Some("BadFaxLines"),
        0x0147 => Some("CleanFaxData"),
        0x0157 => Some("ClipPath"),
        0x0148 => Some("ConsecutiveBadFaxLines"),
        0x01b1 => Some("Decode"),
        0x01b2 => Some("DefaultImageColor"),
        0x010d => Some("DocumentName"),
        0x0150 => Some("DotRange"),
        0x0141 => Some("HalftoneHints"),
        0x015a => Some("Indexed"),
        0x015b => Some("JPEGTables"),
        0x011d => Some("PageName"),
        0x0129 => Some("PageNumber"),
        0x013d => Some("Predictor"),
        0x013f => Some("PrimaryChromaticities"),
        0x0214 => Some("ReferenceBlackWhite"),
        0x0153 => Some("SampleFormat"),
        0x0154 => Some("SMinSampleValue"),
        0x0155 => Some("SMaxSampleValue"),
        0x022f => Some("StripRowCounts"),
        0x014a => Some("SubIFDs"),
        0x0124 => Some("T4Options"),
        0x0125 => Some("T6Options"),
        0x0145 => Some("TileByteCounts"),
        0x0143 => Some("TileLength"),
        0x0144 => Some("TileOffsets"),
        0x0142 => Some("TileWidth"),
        0x012d => Some("TransferFunction"),
        0x013e => Some("WhitePoint"),
        0x0158 => Some("XClipPathUnits"),
        0x011e => Some("XPosition"),
        0x0211 => Some("YCbCrCoefficients"),
        0x0213 => Some("YCbCrPositioning"),
        0x0212 => Some("YCbCrSubSampling"),
        0x0159 => Some("YClipPathUnits"),
        0x011f => Some("YPosition"),

        // EXIF
        0x9202 => Some("ApertureValue"),
        0xa001 => Some("ColorSpace"),
        0x9004 => Some("DateTimeDigitized"),
        0x9003 => Some("DateTimeOriginal"),
        0x8769 => Some("Exif IFD"),
        0x9000 => Some("ExifVersion"),
        0x829a => Some("ExposureTime"),
        0xa300 => Some("FileSource"),
        0x9209 => Some("Flash"),
        0xa000 => Some("FlashpixVersion"),
        0x829d => Some("FNumber"),
        0xa420 => Some("ImageUniqueID"),
        0x9208 => Some("LightSource"),
        0x927c => Some("MakerNote"),
        0x9201 => Some("ShutterSpeedValue"),
        0x9286 => Some("UserComment"),

        // IPTC
        0x83bb => Some("IPTC"),

        // ICC
        0x8773 => Some("ICC Profile"),

        // XMP
        0x02bc => Some("XMP"),

        // GDAL
        0xa480 => Some("GDAL_METADATA"),
        0xa481 => Some("GDAL_NODATA"),

        // Photoshop
        0x8649 => Some("Photoshop"),

        // GeoTiff
        0x830e => Some("ModelPixelScale"),
        0x8482 => Some("ModelTiepoint"),
        0x85d8 => Some("ModelTransformation"),
        0x87af => Some("GeoKeyDirectory"),
        0x87b0 => Some("GeoDoubleParams"),
        0x87b1 => Some("GeoAsciiParams"), // 34737

        // LERC
        0xc5f2 => Some("LercParameters"),
        _ => None,
    }
}

/// Returns the name of a GeoTiff key
pub fn geo_key_name(key: u16) -> Option<&'static str> {
    match key {
        1024 => Some("GTModelTypeGeoKey"),
        1025 => Some("GTRasterTypeGeoKey"),
        1026 => Some("GTCitationGeoKey"),
        2048 => Some("GeographicTypeGeoKey"),
        2049 => Some("GeogCitationGeoKey"),
        2050 => Some("GeogGeodeticDatumGeoKey"),
        2051 => Some("GeogPrimeMeridianGeoKey"),
        2052 => Some("GeogLinearUnitsGeoKey"),
        2053 => Some("GeogLinearUnitSizeGeoKey"),
        2054 => Some("GeogAngularUnitsGeoKey"),
        2055 => Some("GeogAngularUnitSizeGeoKey"),
        2056 => Some("GeogEllipsoidGeoKey"),
        2057 => Some("GeogSemiMajorAxisGeoKey"),
        2058 => Some("GeogSemiMinorAxisGeoKey"),
        2059 => Some("GeogInvFlatteningGeoKey"),
        2060 => Some("GeogAzimuthUnitsGeoKey"),
        2061 => Some("GeogPrimeMeridianLongGeoKey"),
        2062 => Some("GeogTOWGS84GeoKey"),
        3072 => Some("ProjectedCSTypeGeoKey"),
        3073 => Some("PCSCitationGeoKey"),
        3074 => Some("ProjectionGeoKey"),
        3075 => Some("ProjCoordTransGeoKey"),
        3076 => Some("ProjLinearUnitsGeoKey"),
        3077 => Some("ProjLinearUnitSizeGeoKey"),
        3078 => Some("ProjStdParallel1GeoKey"),
        3079 => Some("ProjStdParallel2GeoKey"),
        3080 => Some("ProjNatOriginLongGeoKey"),
        3081 => Some("ProjNatOriginLatGeoKey"),
        3082 => Some("ProjFalseEastingGeoKey"),
        3083 => Some("ProjFalseNorthingGeoKey"),
        3084 => Some("ProjFalseOriginLongGeoKey"),
        3085 => Some("ProjFalseOriginLatGeoKey"),
        3086 => Some("ProjFalseOriginEastingGeoKey"),
        3087 => Some("ProjFalseOriginNorthingGeoKey"),
        3088 => Some("ProjCenterLongGeoKey"),
        3089 => Some("ProjCenterLatGeoKey"),
        3090 => Some("ProjCenterEastingGeoKey"),
        3091 => Some("ProjCenterNorthingGeoKey"),
        3092 => Some("ProjScaleAtNatOriginGeoKey"),
        3093 => Some("ProjScaleAtCenterGeoKey"),
        3094 => Some("ProjAzimuthAngleGeoKey"),
        3095 => Some("ProjStraightVertPoleLongGeoKey"),
        3096 => Some("ProjRectifiedGridAngleGeoKey"),
        4096 => Some("VerticalCSTypeGeoKey"),
        4097 => Some("VerticalCitationGeoKey"),
        4098 => Some("VerticalDatumGeoKey"),
        4099 => Some("VerticalUnitsGeoKey"),
        _ => None,
    }
}

/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_map_projection_methods
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_map_projection_parameters
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#section-D-3
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_summary_of_geokey_ids_and_names
/// NOTE: Let's just use a BTreeMap instead
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GeoKeyDirectory {
    /// GeoTIFF model type
    pub GTModelTypeGeoKey: Option<i16>, // SHORT (1024)
    /// GeoTIFF raster type
    pub GTRasterTypeGeoKey: Option<i16>, // SHORT (1025)
    /// GeoTIFF citation
    pub GTCitationGeoKey: Option<String>, // ASCII (1026)
    /// Geographical type
    pub GeographicTypeGeoKey: Option<i16>, // SHORT (2048)
    /// Geographical citation
    pub GeogCitationGeoKey: Option<String>, // ASCII (2049)
    /// Geographical datum
    pub GeogGeodeticDatumGeoKey: Option<i16>, // SHORT (2050)
    /// Geographical prime meridian
    pub GeogPrimeMeridianGeoKey: Option<i16>, // SHORT (2051)
    /// Geographical linear units
    pub GeogLinearUnitsGeoKey: Option<i16>, // SHORT (2052)
    /// Geographical linear unit size
    pub GeogLinearUnitSizeGeoKey: Option<f64>, // DOUBLE (2053)
    /// Geographical angular units
    pub GeogAngularUnitsGeoKey: Option<i16>, // SHORT (2054)
    /// Geographical angular unit size
    pub GeogAngularUnitSizeGeoKey: Option<f64>, // DOUBLE (2055)
    /// Geographical ellipsoid
    pub GeogEllipsoidGeoKey: Option<i16>, // SHORT (2056)
    /// Geographical semi-major axis
    pub GeogSemiMajorAxisGeoKey: Option<f64>, // DOUBLE (2057)
    /// Geographical semi-minor axis
    pub GeogSemiMinorAxisGeoKey: Option<f64>, // DOUBLE (2058)
    /// Geographical inverse flattening
    pub GeogInvFlatteningGeoKey: Option<f64>, // DOUBLE (2059)
    /// Geographical azimuth
    pub GeogAzimuthUnitsGeoKey: Option<i16>, // SHORT (2060)
    /// Geographical prime meridian
    pub GeogPrimeMeridianLongGeoKey: Option<f64>, // DOUBLE (2061)
    /// Geographical TOWGS84
    pub GeogTOWGS84GeoKey: Option<Vec<f64>>, // DOUBLE (2062)
    /// Projected type
    pub ProjectedCSTypeGeoKey: Option<i16>, // SHORT (3072)
    /// Projected citation
    pub PCSCitationGeoKey: Option<String>, // ASCII (3073)
    /// Projection
    pub ProjectionGeoKey: Option<i16>, // SHORT (3074)
    /// Projection coordinate transformation
    pub ProjCoordTransGeoKey: Option<i16>, // SHORT (3075)
    /// Projection linear units
    pub ProjLinearUnitsGeoKey: Option<i16>, // SHORT (3076)
    /// Projection linear unit size
    pub ProjLinearUnitSizeGeoKey: Option<f64>, // DOUBLE (3077)
    /// Projection standard parallel
    pub ProjStdParallel1GeoKey: Option<f64>, // DOUBLE (3078)
    /// Projection standard parallel
    pub ProjStdParallel2GeoKey: Option<f64>, // DOUBLE (3079)
    /// Projection natural origin
    pub ProjNatOriginLongGeoKey: Option<f64>, // DOUBLE (3080)
    /// Projection natural origin
    pub ProjNatOriginLatGeoKey: Option<f64>, // DOUBLE (3081)
    /// Projection false easting
    pub ProjFalseEastingGeoKey: Option<f64>, // DOUBLE (3082)
    /// Projection false northing
    pub ProjFalseNorthingGeoKey: Option<f64>, // DOUBLE (3083)
    /// Projection false origin
    pub ProjFalseOriginLongGeoKey: Option<f64>, // DOUBLE (3084)
    /// Projection false origin
    pub ProjFalseOriginLatGeoKey: Option<f64>, // DOUBLE (3085)
    /// Projection false origin
    pub ProjFalseOriginEastingGeoKey: Option<f64>, // DOUBLE (3086)
    /// Projection false origin
    pub ProjFalseOriginNorthingGeoKey: Option<f64>, // DOUBLE (3087)
    /// Projection center lon
    pub ProjCenterLongGeoKey: Option<f64>, // DOUBLE (3088)
    /// Projection center lat
    pub ProjCenterLatGeoKey: Option<f64>, // DOUBLE (3089)
    /// Projection center easting
    pub ProjCenterEastingGeoKey: Option<f64>, // DOUBLE (3090)
    /// Projection center northing
    pub ProjCenterNorthingGeoKey: Option<f64>, // DOUBLE (3091)
    /// Projection scale at natural origin
    pub ProjScaleAtNatOriginGeoKey: Option<f64>, // DOUBLE (3092)
    /// Projection scale at center
    pub ProjScaleAtCenterGeoKey: Option<f64>, // DOUBLE (3093)
    /// Projection azimuth angle
    pub ProjAzimuthAngleGeoKey: Option<f64>, // DOUBLE (3094)
    /// Projection straight vertical pole
    pub ProjStraightVertPoleLongGeoKey: Option<f64>, // DOUBLE (3095)
    /// Rectified grid angle
    pub ProjRectifiedGridAngleGeoKey: Option<f64>, // DOUBLE (3096)
    /// Projection CS type
    pub VerticalCSTypeGeoKey: Option<i16>, // SHORT (4096)
    /// Vertical citation
    pub VerticalCitationGeoKey: Option<String>, // ASCII (4097)
    /// Vertical datum
    pub VerticalDatumGeoKey: Option<i16>, // SHORT (4098)
    /// Vertical units
    pub VerticalUnitsGeoKey: Option<i16>, // SHORT (4099)
}
