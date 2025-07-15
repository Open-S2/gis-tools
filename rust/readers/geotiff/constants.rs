use alloc::{collections::BTreeMap, string::String, vec::Vec};

/// TIFF Photometric Interpretations
#[derive(Debug, Clone, Copy, PartialEq)]
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
impl From<i16> for PhotometricInterpretations {
    fn from(value: i16) -> Self {
        match value {
            0 => PhotometricInterpretations::WhiteIsZero,
            1 => PhotometricInterpretations::BlackIsZero,
            2 => PhotometricInterpretations::RGB,
            3 => PhotometricInterpretations::Palette,
            4 => PhotometricInterpretations::TransparencyMask,
            5 => PhotometricInterpretations::CMYK,
            6 => PhotometricInterpretations::YCbCr,
            8 => PhotometricInterpretations::CIELab,
            9 => PhotometricInterpretations::ICCLab,
            10 => PhotometricInterpretations::ITULab,
            _ => unreachable!(),
        }
    }
}

/// TIFF Extra Samples
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtraSamplesValues {
    /// Unspecified
    Unspecified = 0,
    /// Associated alpha
    Assocalpha = 1,
    /// Unassociated alpha
    Unassalpha = 2,
}
impl From<u16> for ExtraSamplesValues {
    fn from(value: u16) -> Self {
        match value {
            1 => ExtraSamplesValues::Assocalpha,
            2 => ExtraSamplesValues::Unassalpha,
            _ => ExtraSamplesValues::Unspecified,
        }
    }
}

/// LERC Parameters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LercParameters {
    /// LERC version
    Version = 0,
    /// Add compression
    AddCompression = 1,
}

/// LERC Add Compression
#[derive(Debug, Clone, Copy, PartialEq)]
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
    FieldTagNames::BitsPerSample as u16,
    FieldTagNames::ExtraSamples as u16,
    FieldTagNames::SampleFormat as u16,
    FieldTagNames::StripByteCounts as u16,
    FieldTagNames::StripOffsets as u16,
    FieldTagNames::StripRowCounts as u16,
    FieldTagNames::TileByteCounts as u16,
    FieldTagNames::TileOffsets as u16,
    FieldTagNames::SubIFDs as u16,
];

/// All GeoTIFF keys and their u16 representations
///
/// Geo Key Directory:
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_map_projection_methods
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_map_projection_parameters
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#section-D-3
/// https://docs.ogc.org/is/19-008r4/19-008r4.html#_summary_of_geokey_ids_and_names
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum GeoKeyDirectoryKeys {
    /// GeoTIFF model type
    GTModelTypeGeoKey = 1024, // SHORT (1024)
    /// GeoTIFF raster type
    GTRasterTypeGeoKey = 1025, // SHORT (1025)
    /// GeoTIFF citation
    GTCitationGeoKey = 1026, // ASCII (1026)
    /// Geographical type
    GeographicTypeGeoKey = 2048, // SHORT (2048)
    /// Geographical citation
    GeogCitationGeoKey = 2049, // ASCII (2049)
    /// Geographical datum
    GeogGeodeticDatumGeoKey = 2050, // SHORT (2050)
    /// Geographical prime meridian
    GeogPrimeMeridianGeoKey = 2051, // SHORT (2051)
    /// Geographical linear units
    GeogLinearUnitsGeoKey = 2052, // SHORT (2052)
    /// Geographical linear unit size
    GeogLinearUnitSizeGeoKey = 2053, // DOUBLE (2053)
    /// Geographical angular units
    GeogAngularUnitsGeoKey = 2054, // SHORT (2054)
    /// Geographical angular unit size
    GeogAngularUnitSizeGeoKey = 2055, // DOUBLE (2055)
    /// Geographical ellipsoid
    GeogEllipsoidGeoKey = 2056, // SHORT (2056)
    /// Geographical semi-major axis
    GeogSemiMajorAxisGeoKey = 2057, // DOUBLE (2057)
    /// Geographical semi-minor axis
    GeogSemiMinorAxisGeoKey = 2058, // DOUBLE (2058)
    /// Geographical inverse flattening
    GeogInvFlatteningGeoKey = 2059, // DOUBLE (2059)
    /// Geographical azimuth
    GeogAzimuthUnitsGeoKey = 2060, // SHORT (2060)
    /// Geographical prime meridian
    GeogPrimeMeridianLongGeoKey = 2061, // DOUBLE (2061)
    /// Geographical TOWGS84
    GeogTOWGS84GeoKey = 2062, // DOUBLE (2062)
    /// Projected type
    ProjectedCSTypeGeoKey = 3072, // SHORT (3072)
    /// Projected citation
    PCSCitationGeoKey = 3073, // ASCII (3073)
    /// Projection
    ProjectionGeoKey = 3074, // SHORT (3074)
    /// Projection coordinate transformation
    ProjCoordTransGeoKey = 3075, // SHORT (3075)
    /// Projection linear units
    ProjLinearUnitsGeoKey = 3076, // SHORT (3076)
    /// Projection linear unit size
    ProjLinearUnitSizeGeoKey = 3077, // DOUBLE (3077)
    /// Projection standard parallel
    ProjStdParallel1GeoKey = 3078, // DOUBLE (3078)
    /// Projection standard parallel
    ProjStdParallel2GeoKey = 3079, // DOUBLE (3079)
    /// Projection natural origin
    ProjNatOriginLongGeoKey = 3080, // DOUBLE (3080)
    /// Projection natural origin
    ProjNatOriginLatGeoKey = 3081, // DOUBLE (3081)
    /// Projection false easting
    ProjFalseEastingGeoKey = 3082, // DOUBLE (3082)
    /// Projection false northing
    ProjFalseNorthingGeoKey = 3083, // DOUBLE (3083)
    /// Projection false origin
    ProjFalseOriginLongGeoKey = 3084, // DOUBLE (3084)
    /// Projection false origin
    ProjFalseOriginLatGeoKey = 3085, // DOUBLE (3085)
    /// Projection false origin
    ProjFalseOriginEastingGeoKey = 3086, // DOUBLE (3086)
    /// Projection false origin
    ProjFalseOriginNorthingGeoKey = 3087, // DOUBLE (3087)
    /// Projection center lon
    ProjCenterLongGeoKey = 3088, // DOUBLE (3088)
    /// Projection center lat
    ProjCenterLatGeoKey = 3089, // DOUBLE (3089)
    /// Projection center easting
    ProjCenterEastingGeoKey = 3090, // DOUBLE (3090)
    /// Projection center northing
    ProjCenterNorthingGeoKey = 3091, // DOUBLE (3091)
    /// Projection scale at natural origin
    ProjScaleAtNatOriginGeoKey = 3092, // DOUBLE (3092)
    /// Projection scale at center
    ProjScaleAtCenterGeoKey = 3093, // DOUBLE (3093)
    /// Projection azimuth angle
    ProjAzimuthAngleGeoKey = 3094, // DOUBLE (3094)
    /// Projection straight vertical pole
    ProjStraightVertPoleLongGeoKey = 3095, // DOUBLE (3095)
    /// Rectified grid angle
    ProjRectifiedGridAngleGeoKey = 3096, // DOUBLE (3096)
    /// Projection CS type
    VerticalCSTypeGeoKey = 4096, // SHORT (4096)
    /// Vertical citation
    VerticalCitationGeoKey = 4097, // ASCII (4097)
    /// Vertical datum
    VerticalDatumGeoKey = 4098, // SHORT (4098)
    /// Vertical units
    VerticalUnitsGeoKey = 4099, // SHORT (4099)
}
impl GeoKeyDirectoryKeys {
    /// Convert a key to a TIFF type
    pub fn to_type(key: GeoKeyDirectoryKeys) -> GeoTIFFTypes {
        match key {
            GeoKeyDirectoryKeys::GTModelTypeGeoKey
            | GeoKeyDirectoryKeys::GTRasterTypeGeoKey
            | GeoKeyDirectoryKeys::GeographicTypeGeoKey
            | GeoKeyDirectoryKeys::GeogGeodeticDatumGeoKey
            | GeoKeyDirectoryKeys::GeogPrimeMeridianGeoKey
            | GeoKeyDirectoryKeys::GeogLinearUnitsGeoKey
            | GeoKeyDirectoryKeys::GeogAngularUnitsGeoKey
            | GeoKeyDirectoryKeys::GeogEllipsoidGeoKey
            | GeoKeyDirectoryKeys::GeogAzimuthUnitsGeoKey
            | GeoKeyDirectoryKeys::ProjectedCSTypeGeoKey
            | GeoKeyDirectoryKeys::ProjectionGeoKey
            | GeoKeyDirectoryKeys::ProjCoordTransGeoKey
            | GeoKeyDirectoryKeys::ProjLinearUnitsGeoKey
            | GeoKeyDirectoryKeys::VerticalCSTypeGeoKey
            | GeoKeyDirectoryKeys::VerticalDatumGeoKey
            | GeoKeyDirectoryKeys::VerticalUnitsGeoKey => GeoTIFFTypes::SHORT,
            GeoKeyDirectoryKeys::GTCitationGeoKey
            | GeoKeyDirectoryKeys::GeogCitationGeoKey
            | GeoKeyDirectoryKeys::PCSCitationGeoKey
            | GeoKeyDirectoryKeys::VerticalCitationGeoKey => GeoTIFFTypes::ASCII,
            _ => GeoTIFFTypes::DOUBLE,
        }
    }
}

/// TIFF Field Types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeoTIFFTypes {
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
impl GeoTIFFTypes {
    /// Return the size of the type
    pub fn to_size(&self) -> usize {
        match self {
            GeoTIFFTypes::BYTE
            | GeoTIFFTypes::ASCII
            | GeoTIFFTypes::SBYTE
            | GeoTIFFTypes::UNDEFINED => 1,
            GeoTIFFTypes::SHORT | GeoTIFFTypes::SSHORT => 2,
            GeoTIFFTypes::LONG | GeoTIFFTypes::SLONG | GeoTIFFTypes::FLOAT | GeoTIFFTypes::IFD => 4,
            GeoTIFFTypes::RATIONAL
            | GeoTIFFTypes::SRATIONAL
            | GeoTIFFTypes::DOUBLE
            | GeoTIFFTypes::LONG8
            | GeoTIFFTypes::SLONG8
            | GeoTIFFTypes::IFD8 => 8,
        }
    }
}
impl From<u16> for GeoTIFFTypes {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => GeoTIFFTypes::BYTE,
            0x0002 => GeoTIFFTypes::ASCII,
            0x0003 => GeoTIFFTypes::SHORT,
            0x0004 => GeoTIFFTypes::LONG,
            0x0005 => GeoTIFFTypes::RATIONAL,
            0x0006 => GeoTIFFTypes::SBYTE,
            0x0007 => GeoTIFFTypes::UNDEFINED,
            0x0008 => GeoTIFFTypes::SSHORT,
            0x0009 => GeoTIFFTypes::SLONG,
            0x000a => GeoTIFFTypes::SRATIONAL,
            0x000b => GeoTIFFTypes::FLOAT,
            0x000c => GeoTIFFTypes::DOUBLE,
            0x000d => GeoTIFFTypes::IFD,
            0x0010 => GeoTIFFTypes::LONG8,
            0x0011 => GeoTIFFTypes::SLONG8,
            0x0012 => GeoTIFFTypes::IFD8,
            _ => GeoTIFFTypes::UNDEFINED,
        }
    }
}

/// A GeoTIFF store system
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeoStore {
    /// Internal data
    pub data: BTreeMap<u16, Vec<u8>>,
}
impl GeoStore {
    /// len
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// has key
    pub fn has(&self, key: u16) -> bool {
        self.data.contains_key(&key)
    }
    /// Common function name
    pub fn insert(&mut self, key: u16, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    /// Set a value
    pub fn set(&mut self, key: u16, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    /// Get a value
    pub fn get(&self, key: u16) -> Option<Vec<u8>> {
        self.data.get(&key).cloned()
    }
    /// Set a short
    pub fn set_short(&mut self, key: u16, value: i16) {
        self.set(key, value.to_le_bytes().to_vec());
    }
    /// Get a short
    pub fn get_short(&self, key: u16) -> Option<i16> {
        self.get(key).map(|v| i16::from_le_bytes(v.try_into().unwrap()))
    }
    /// Set a string
    pub fn set_string(&mut self, key: u16, value: String) {
        self.set(key, value.as_bytes().to_vec());
    }
    /// Get a string
    pub fn get_string(&self, key: u16) -> Option<String> {
        self.get(key).map(|v| String::from_utf8_lossy(&v[..v.len().saturating_sub(1)]).into())
    }
    /// Set a double
    pub fn set_double(&mut self, key: u16, value: f64) {
        self.set(key, value.to_le_bytes().to_vec());
    }
    /// Get a double
    pub fn get_double(&self, key: u16) -> Option<f64> {
        self.get(key).map(|v| f64::from_le_bytes(v.try_into().unwrap()))
    }

    /// get u16 array
    pub fn get_u16s(&self, key: u16) -> Option<Vec<u16>> {
        self.get(key).map(|v| {
            v.chunks(2).map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get u32 array
    pub fn get_u32s(&self, key: u16) -> Option<Vec<u32>> {
        self.get(key).map(|v| {
            v.chunks(4).map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get u64 array
    pub fn get_u64s(&self, key: u16) -> Option<Vec<u64>> {
        self.get(key).map(|v| {
            v.chunks(8).map(|chunk| u64::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get i16 array
    pub fn geti16s(&self, key: u16) -> Option<Vec<i16>> {
        self.get(key).map(|v| {
            v.chunks(2).map(|chunk| i16::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get i32 array
    pub fn geti32s(&self, key: u16) -> Option<Vec<i32>> {
        self.get(key).map(|v| {
            v.chunks(4).map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get i64 array
    pub fn geti64s(&self, key: u16) -> Option<Vec<i64>> {
        self.get(key).map(|v| {
            v.chunks(8).map(|chunk| i64::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get f32 array
    pub fn getf32s(&self, key: u16) -> Option<Vec<f32>> {
        self.get(key).map(|v| {
            v.chunks(4).map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
    /// get f64 array
    pub fn getf64s(&self, key: u16) -> Option<Vec<f64>> {
        self.get(key).map(|v| {
            v.chunks(8).map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap())).collect()
        })
    }
}

/// List of Tag Names
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum FieldTagNames {
    // TIFF Baseline
    /// Artist
    Artist = 0x013b,
    /// BitsPerSample (u16 array)
    BitsPerSample = 0x0102,
    /// CellLength
    CellLength = 0x0109,
    /// CellWidth
    CellWidth = 0x0108,
    /// ColorMap
    ColorMap = 0x0140,
    /// Compression (short)
    Compression = 0x0103,
    /// Copyright
    Copyright = 0x8298,
    /// DateTime
    DateTime = 0x0132,
    /// ExtraSamples
    ExtraSamples = 0x0152,
    /// FillOrder
    FillOrder = 0x010a,
    /// FreeByteCounts
    FreeByteCounts = 0x0121,
    /// FreeOffsets
    FreeOffsets = 0x0120,
    /// GrayResponseCurve
    GrayResponseCurve = 0x0123,
    /// GrayResponseUnit
    GrayResponseUnit = 0x0122,
    /// HostComputer
    HostComputer = 0x013c,
    /// ImageDescription (ascii)
    ImageDescription = 0x010e,
    /// ImageLength (short)
    ImageLength = 0x0101,
    /// ImageWidth (short)
    ImageWidth = 0x0100,
    /// Make
    Make = 0x010f,
    /// MaxSampleValue
    MaxSampleValue = 0x0119,
    /// MinSampleValue
    MinSampleValue = 0x0118,
    /// Model
    Model = 0x0110,
    /// NewSubfileType
    NewSubfileType = 0x00fe,
    /// Orientation
    Orientation = 0x0112,
    /// PhotometricInterpretation (short)
    PhotometricInterpretation = 0x0106,
    /// PlanarConfiguration
    PlanarConfiguration = 0x011c,
    /// ResolutionUnit (short)
    ResolutionUnit = 0x0128,
    /// RowsPerStrip (short)
    RowsPerStrip = 0x0116,
    /// SamplesPerPixel (short)
    SamplesPerPixel = 0x0115,
    /// Software
    Software = 0x0131,
    /// StripByteCounts (U32 array)
    StripByteCounts = 0x0117,
    /// StripOffsets (U32 array)
    StripOffsets = 0x0111,
    /// SubfileType
    SubfileType = 0x00ff,
    /// Threshholding
    Threshholding = 0x0107,
    /// XResolution ([u32, u32]) - u32s array
    XResolution = 0x011a,
    /// YResolution ([u32, u32]) - u32s array
    YResolution = 0x011b,

    // TIFF Extended
    /// BadFaxLines
    BadFaxLines = 0x0146,
    /// CleanFaxData
    CleanFaxData = 0x0147,
    /// ClipPath
    ClipPath = 0x0157,
    /// ConsecutiveBadFaxLines
    ConsecutiveBadFaxLines = 0x0148,
    /// Decode
    Decode = 0x01b1,
    /// DefaultImageColor
    DefaultImageColor = 0x01b2,
    /// DocumentName
    DocumentName = 0x010d,
    /// DotRange
    DotRange = 0x0150,
    /// HalftoneHints
    HalftoneHints = 0x0141,
    /// Indexed
    Indexed = 0x015a,
    /// JPEGTables
    JPEGTables = 0x015b,
    /// PageName
    PageName = 0x011d,
    /// PageNumber
    PageNumber = 0x0129,
    /// Predictor (short)
    Predictor = 0x013d,
    /// PrimaryChromaticities
    PrimaryChromaticities = 0x013f,
    /// ReferenceBlackWhite
    ReferenceBlackWhite = 0x0214,
    /// SampleFormat
    SampleFormat = 0x0153,
    /// SMinSampleValue
    SMinSampleValue = 0x0154,
    /// SMaxSampleValue
    SMaxSampleValue = 0x0155,
    /// StripRowCounts
    StripRowCounts = 0x022f,
    /// SubIFDs
    SubIFDs = 0x014a,
    /// T4Options
    T4Options = 0x0124,
    /// T6Options
    T6Options = 0x0125,
    /// TileByteCounts
    TileByteCounts = 0x0145,
    /// TileLength
    TileLength = 0x0143,
    /// TileOffsets
    TileOffsets = 0x0144,
    /// TileWidth
    TileWidth = 0x0142,
    /// TransferFunction
    TransferFunction = 0x012d,
    /// WhitePoint
    WhitePoint = 0x013e,
    /// XClipPathUnits
    XClipPathUnits = 0x0158,
    /// XPosition
    XPosition = 0x011e,
    /// YCbCrCoefficients
    YCbCrCoefficients = 0x0211,
    /// YCbCrPositioning
    YCbCrPositioning = 0x0213,
    /// YCbCrSubSampling
    YCbCrSubSampling = 0x0212,
    /// YClipPathUnits
    YClipPathUnits = 0x0159,
    /// YPosition
    YPosition = 0x011f,

    // EXIF
    /// ApertureValue
    ApertureValue = 0x9202,
    /// ColorSpace
    ColorSpace = 0xa001,
    /// DateTimeDigitized
    DateTimeDigitized = 0x9004,
    /// DateTimeOriginal
    DateTimeOriginal = 0x9003,
    /// Exif IFD
    ExifIFD = 0x8769,
    /// ExifVersion
    ExifVersion = 0x9000,
    /// ExposureTime
    ExposureTime = 0x829a,
    /// FileSource
    FileSource = 0xa300,
    /// Flash
    Flash = 0x9209,
    /// FlashpixVersion
    FlashpixVersion = 0xa000,
    /// FNumber
    FNumber = 0x829d,
    /// ImageUniqueID
    ImageUniqueID = 0xa420,
    /// LightSource
    LightSource = 0x9208,
    /// MakerNote
    MakerNote = 0x927c,
    /// ShutterSpeedValue
    ShutterSpeedValue = 0x9201,
    /// UserComment
    UserComment = 0x9286,

    // IPTC
    /// IPTC
    IPTC = 0x83bb,

    // ICC
    /// ICC Profile
    ICC = 0x8773,

    // XMP
    /// XMP
    XMP = 0x02bc,

    // GDAL
    /// GDAL_METADATA
    GdalMetadata = 0xa480,
    /// GDAL_NODATA
    GdalNodata = 0xa481,

    // Photoshop
    /// Photoshop
    Photoshop = 0x8649,

    // GeoTiff
    /// ModelPixelScale
    ModelPixelScale = 0x830e,
    /// ModelTiepoint
    ModelTiepoint = 0x8482,
    /// ModelTransformation
    ModelTransformation = 0x85d8,
    /// GeoKeyDirectory
    GeoKeyDirectory = 0x87af,
    /// GeoDoubleParams
    GeoDoubleParams = 0x87b0,
    /// GeoAsciiParams
    GeoAsciiParams = 0x87b1,

    // LERC
    /// LercParameters
    LercParameters = 0xc5f2,
}
