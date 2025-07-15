use super::{LASExtendedVariableLengthRecord, LASHeader, LASPoint};
use crate::{
    parsers::{FeatureReader, Reader},
    proj::Transformer,
    readers::{FieldTagNames, GeoStore, build_transform_from_geo_keys, parse_geotiff_raw_geokeys},
};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use s2json::{Properties, VectorFeature, VectorGeometry, VectorPoint};

/// Options for the LAS Reader
#[derive(Debug, Default, Clone)]
pub struct LASReaderOptions {
    /// Whether to transform the data to WGS84 if it's not already in WGS84
    pub dont_transform: bool,
    /// List of EPSG codes to utilize e.g. `{ "4326": "WKT_STRING" }``
    pub epsg_codes: BTreeMap<String, String>,
}

/// An LAS Shaped Vector Feature
pub type LASVectorFeature = VectorFeature<(), Properties, LASPoint>;

/// # LAS Reader
///
/// ## Description
/// Reads LAS data. Supports up to the LAS 1.4 specification.
/// [See specification](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf)
/// Implements the {@link FeatureIterator} interface
///
/// Data is stored like so:
/// ```txt
/// |           PUBLIC HEADER BLOCK           |
/// |         VARIABLE LENGTH RECORDS         |
/// |            POINT DATA RECORDS           |
/// ```
///
/// ## Usage
///
/// ```ts
/// // TODO
/// ```
///
/// ## Links
/// - https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
/// - https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
/// - https://liblas.org/development/index.html
/// - https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
/// - https://github.com/PDAL/PDAL
/// - https://github.com/libLAS/libLAS (deprecated for PDAL)
/// - https://github.com/LASzip
#[derive(Debug)]
pub struct LASReader<T: Reader> {
    reader: T,
    /// Public Header Block
    pub header: LASHeader,
    /// Extended VARIABLE LENGTH RECORDS
    pub variable_length_records: BTreeMap<u32, LASExtendedVariableLengthRecord>,
    /// WKT projection string
    pub wkt: Option<String>,
    /// GeoKeyDirectory
    pub geo_key_directory: GeoStore,
    transformer: Transformer,
    dont_transform: bool,
}
impl<T: Reader> LASReader<T> {
    /// Create a new LASReader
    pub fn new(reader: T, options: Option<LASReaderOptions>) -> Self {
        let options = options.unwrap_or_default();
        let header = LASHeader::from_reader(&reader);
        let variable_length_records = las_parse_variable_length_records(&header, &reader);
        let mut transformer = Transformer::new();
        for (epsg_code, wkt) in options.epsg_codes.iter() {
            transformer.insert_epsg_code(epsg_code.clone(), wkt.clone());
        }
        let wkt = build_wkt(&header, &variable_length_records, &mut transformer);
        let geo_key_directory = build_geo_key_directory(&variable_length_records, &mut transformer);
        Self {
            reader,
            header,
            variable_length_records,
            wkt,
            geo_key_directory,
            transformer,
            dont_transform: options.dont_transform,
        }
    }

    /// Get the number of points stored
    pub fn len(&self) -> u64 {
        self.header.num_points as u64
    }

    /// Check if the reader is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reads a point in at index as a feature
    pub fn get_feature(&self, index: u64) -> Option<LASVectorFeature> {
        self.get_point(index).map(|point| {
            VectorFeature::new_wm(
                None,
                Properties::default(),
                VectorGeometry::new_point(point, None),
                None,
            )
        })
    }

    /// Reads a point in at index
    pub fn get_point(&self, index: u64) -> Option<VectorPoint<LASPoint>> {
        let Self { reader, header, dont_transform, .. } = self;
        let LASHeader {
            num_points,
            offset_to_points,
            point_data_format_id: format,
            point_data_record_length,
            ..
        } = header;
        if index + 1 > *num_points as u64 {
            return None;
        }
        let offset_to_points = *offset_to_points as u64;
        let point_data_record_length = *point_data_record_length as u64;
        let offset = offset_to_points + index * point_data_record_length;
        let point = if *format == 0 {
            LASPoint::format0(reader, offset)
        } else if *format == 1 {
            LASPoint::format1(reader, offset)
        } else if *format == 2 {
            LASPoint::format2(reader, offset)
        } else if *format == 3 {
            LASPoint::format3(reader, offset)
        } else if *format == 4 {
            LASPoint::format4(reader, offset)
        } else if *format == 5 {
            LASPoint::format5(reader, offset)
        } else if *format == 6 {
            LASPoint::format6(reader, offset)
        } else if *format == 7 {
            LASPoint::format7(reader, offset)
        } else if *format == 8 {
            LASPoint::format8(reader, offset)
        } else if *format == 9 {
            LASPoint::format9(reader, offset)
        } else if *format == 10 {
            LASPoint::format10(reader, offset)
        } else {
            panic!("Unknown Point Data Format ID: {}", format);
        };
        let mut vp = point.to_vector_point(header);

        if !*dont_transform {
            self.transformer.forward_mut(&mut vp);
        }

        Some(vp)
    }
}

/// The LAS Iterator tool
#[derive(Debug)]
pub struct LASIterator<'a, T: Reader> {
    reader: &'a LASReader<T>,
    index: u64,
}
impl<T: Reader> Iterator for LASIterator<'_, T> {
    type Item = LASVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let las = &self.reader;
        if let Some(point) = las.get_feature(self.index) {
            self.index += 1;
            Some(point)
        } else {
            None
        }
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader> FeatureReader<(), Properties, LASPoint> for LASReader<T> {
    type FeatureIterator<'a>
        = LASIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        LASIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}

/// The Public Header Block is followed by one or more Variable Length Records (There is one
/// mandatory Variable Length Record, GeoKeyDirectoryTag). The number of Variable Length
/// Records is specified in the "Number of Variable Length Records" field in the Public Header Block.
/// The Variable Length Records must be accessed sequentially since the size of each variable length
/// record is contained in the Variable Length Record Header. Each Variable Length Record Header
/// is 54 bytes in length.
///
/// Each record is as follows:
/// - Reserved unsigned short 2 bytes
/// - User ID char[16] 16 bytes
/// - Record ID unsigned short 2 bytes
/// - Record Length After Header unsigned short 2 bytes
/// - Description char[32] 32 bytes
/// - optional data: variable size
pub fn las_parse_variable_length_records<T: Reader>(
    header: &LASHeader,
    reader: &T,
) -> BTreeMap<u32, LASExtendedVariableLengthRecord> {
    let LASHeader { header_size, num_variable_length_records, .. } = header;
    let mut res = BTreeMap::new();
    let mut offset = *header_size as u64;
    let mut i = 0;
    while i < *num_variable_length_records {
        let record = LASExtendedVariableLengthRecord::from_reader(reader, offset);
        offset += 54 + record.record_length;
        res.insert(record.record_id as u32, record);
        i += 1;
    }

    res
}

/// WKT Parsing
///
/// For definition of WKT, we refer to Open Geospatial Consortium (OGC) specification “OpenGIS
/// coordinate transformation service implementation specification” revision 1.00 released 12
/// January 2001, section 7 (coordinate transformation services spec). This specification may be
/// found at www.opengeospatial.org/standards/ct. As there are a few dialects of WKT, please note
/// that LAS is not using the “ESRI WKT” dialect, which does not include TOWGS84 and authority
/// nodes.
/// - OGC MATH TRANSFORM WKT RECORD (2111)
/// - OGC COORDINATE SYSTEM WKT (2112)
///
/// NOTE: It is required to use WKT if the point type is 6-10
///
/// @returns - the WKT string if it exists
pub fn build_wkt(
    header: &LASHeader,
    variable_length_records: &BTreeMap<u32, LASExtendedVariableLengthRecord>,
    transformer: &mut Transformer,
) -> Option<String> {
    // 4th bit of global encoding must be set
    if (header.encoding & (1 << 3)) != 0 {
        return None;
    }
    // OGC MATH TRANSFORM WKT RECORD:
    let wkt_math_ogc = variable_length_records.get(&2111).and_then(|v| v.data.clone());
    // OGC COORDINATE SYSTEM WKT:
    let wkt_coord_system_data = variable_length_records.get(&2112).and_then(|v| v.data.clone());
    if wkt_math_ogc.is_none() && wkt_coord_system_data.is_none() {
        return None;
    }
    let wkt_coord_system = wkt_math_ogc.or(wkt_coord_system_data).unwrap();
    let wkt_str: String = String::from_utf8_lossy(&wkt_coord_system).into();
    transformer.set_source(wkt_str.clone());

    Some(wkt_str)
}

/// userID of "LASF_Projection" will contain at least 3 records:
/// - GeoKeyDirectoryTag (34735)
/// - GeoDoubleParamsTag (34736)
/// - GeoASCIIParamsTag (34737)
///
/// Only the `GeoKeyDirectoryTag` record is required. This parses the `GeoKeyDirectoryTag`.
/// This record contains the key values that define the coordinate system. A complete description
/// can be found in the GeoTIFF format specification. Here is a summary from a programmatic point
/// of view for someone interested in implementation.
///
/// The `GeoKeyDirectoryTag` is defined as just an array of unsigned short values. But,
/// programmatically, the data can be seen as something like this:
///
/// @returns - The parsed GeoKeyDirectory
pub fn build_geo_key_directory(
    variable_length_records: &BTreeMap<u32, LASExtendedVariableLengthRecord>,
    transformer: &mut Transformer,
) -> GeoStore {
    let mut file_dir = GeoStore::default();
    // GeoKeyDirectoryTag (34735)
    let geokey_record = variable_length_records
        .get(&(FieldTagNames::GeoKeyDirectory as u32))
        .and_then(|v| v.data.clone());
    if geokey_record.is_none() {
        return GeoStore::default();
    }
    let raw_geo_keys: Vec<u16> = geokey_record
        .unwrap()
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    // GeoDoubleParamsTag (34736)
    let double_record = variable_length_records
        .get(&(FieldTagNames::GeoDoubleParams as u32))
        .and_then(|v| v.data.clone());
    if let Some(double_record) = double_record {
        file_dir.set(FieldTagNames::GeoDoubleParams as u16, double_record.to_vec());
    }
    // GeoAsciiParamsTag (34737)
    let ascii_record = variable_length_records
        .get(&(FieldTagNames::GeoAsciiParams as u32))
        .and_then(|v| v.data.clone());
    if let Some(ascii_record) = ascii_record {
        file_dir.set(FieldTagNames::GeoAsciiParams as u16, ascii_record.to_vec());
    }
    let gkd = parse_geotiff_raw_geokeys(&raw_geo_keys, &file_dir);
    build_transform_from_geo_keys(transformer, &gkd);

    gkd
}
