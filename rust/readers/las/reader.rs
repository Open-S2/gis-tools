use super::{LASExtendedVariableLengthRecord, LASHeader, LASPoint};
use crate::parsers::{FeatureReader, Reader};
use alloc::collections::BTreeMap;
use s2json::{Properties, VectorFeature, VectorGeometry, VectorPoint};

/// Options for the LAS Reader
#[derive(Debug, Default, Clone)]
pub struct LASReaderOptions {
    /// Whether to transform the data to WGS84 if it's not already in WGS84
    pub dont_transform: bool,
}

/// An LAS Shaped Vector Feature
pub type LASVectorFeature = VectorFeature<(), Properties, LASPoint>;

/**
 * # LAS Reader
 *
 * ## Description
 * Reads LAS data. Supports up to the LAS 1.4 specification.
 * [See specification](https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf)
 * Implements the {@link FeatureIterator} interface
 *
 * Data is stored like so:
 * ```txt
 * |           PUBLIC HEADER BLOCK           |
 * |         VARIABLE LENGTH RECORDS         |
 * |            POINT DATA RECORDS           |
 * ```
 *
 * ## Usage
 *
 * ```ts
 * // TODO
 * ```
 *
 * ## Links
 * - https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
 * - https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
 * - https://liblas.org/development/index.html
 * - https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
 * - https://github.com/PDAL/PDAL
 * - https://github.com/libLAS/libLAS (deprecated for PDAL)
 * - https://github.com/LASzip
 */
#[derive(Debug)]
pub struct LASReader<T: Reader> {
    reader: T,
    /// Public Header Block
    pub header: LASHeader,
    /// Extended VARIABLE LENGTH RECORDS
    pub variable_length_records: BTreeMap<u32, LASExtendedVariableLengthRecord>,
    //   pub wkt?: string;
    //   pub GeoKeyDirectory?: GeoKeyDirectory;
    //   pub transformer = new Transformer();
    dont_transform: bool,
}
impl<T: Reader> LASReader<T> {
    /// Create a new LASReader
    pub fn new(reader: T, options: Option<LASReaderOptions>) -> Self {
        let options = options.unwrap_or_default();
        let header = LASHeader::from_reader(&reader);
        let variable_length_records = las_parse_variable_length_records(&header, &reader);
        Self { reader, header, variable_length_records, dont_transform: options.dont_transform }
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
        let num_points = *num_points as u64;
        if index + 1 > num_points {
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
        let vp = point.to_vector_point(header);

        if *dont_transform {
            // TODO
            // point = this.transformer.forward(point) as VectorPointM<LASFormat>;
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

//   /**
//    * @param input - The LAS input data from a reader/buffer
//    * @param definitions - an array of projection definitions for the transformer if needed
//    * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
//    * @param gridStores - an array of grid readers if needed
//    * @param dontTransform - if you set to true, the source projection is kept
//    */
//   constructor(
//     input: ReaderInputs,
//     definitions: ProjectionTransform[] = [],
//     epsgCodes: Record<string, string> = {},
//     gridStores: GridReader[] = [],
//     readonly dontTransform = false,
//   ) {
//     this.reader = toReader(input);
//     this.header = this.#parseHeader();
//     this.#las_parse_variable_length_records();
//     // set definitions, espgCodes, and gridStores
//     for (const proj of definitions) this.transformer.insertDefinition(proj);
//     for (const [key, value] of Object.entries(epsgCodes))
//       this.transformer.insertEPSGCode(key, value);
//     for (const { key, reader } of gridStores) this.transformer.addGridFromReader(key, reader);
//     // try WTK
//     this.wkt = this.#buildWKT();
//     // they try GeoTiff
//     this.GeoKeyDirectory = this.#buildGeoKeyDirectory();
//   }

//   /**
//    * WKT Parsing
//    *
//    * For definition of WKT, we refer to Open Geospatial Consortium (OGC) specification “OpenGIS
//    * coordinate transformation service implementation specification” revision 1.00 released 12
//    * January 2001, section 7 (coordinate transformation services spec). This specification may be
//    * found at www.opengeospatial.org/standards/ct. As there are a few dialects of WKT, please note
//    * that LAS is not using the “ESRI WKT” dialect, which does not include TOWGS84 and authority
//    * nodes.
//    * - OGC MATH TRANSFORM WKT RECORD (2111)
//    * - OGC COORDINATE SYSTEM WKT (2112)
//    *
//    * NOTE: It is required to use WKT if the point type is 6-10
//    * @returns - the WKT string if it exists
//    */
//   #buildWKT(): string | undefined {
//     const { header, variableLengthRecords } = this;
//     // 4th bit of global encoding must be set
//     if ((header.encoding & (1 << 3)) !== 0) return;
//     // OGC MATH TRANSFORM WKT RECORD:
//     const wktMathOGC = variableLengthRecords[2111]?.data;
//     // OGC COORDINATE SYSTEM WKT:
//     const wktCoordSystemData = variableLengthRecords[2112]?.data;
//     if (wktMathOGC === undefined && wktCoordSystemData === undefined) return;
//     const wktCoordSystem = this.#decoder.decode(wktMathOGC ?? wktCoordSystemData);
//     this.transformer.setSource(wktCoordSystem);

//     return wktCoordSystem;
//   }

//   /**
//    * userID of "LASF_Projection" will contain at least 3 records:
//    * - GeoKeyDirectoryTag (34735)
//    * - GeoDoubleParamsTag (34736)
//    * - GeoASCIIParamsTag (34737)
//    *
//    * Only the `GeoKeyDirectoryTag` record is required. This parses the `GeoKeyDirectoryTag`.
//    * This record contains the key values that define the coordinate system. A complete description
//    * can be found in the GeoTIFF format specification. Here is a summary from a programmatic point
//    * of view for someone interested in implementation.
//    *
//    * The `GeoKeyDirectoryTag` is defined as just an array of unsigned short values. But,
//    * programmatically, the data can be seen as something like this:
//    * @returns - The parsed GeoKeyDirectory
//    */
//   #buildGeoKeyDirectory(): GeoKeyDirectory | undefined {
//     const { variableLengthRecords } = this;
//     // GeoKeyDirectoryTag
//     const geokeyRecord = variableLengthRecords[34735]?.data;
//     if (geokeyRecord === undefined) return;
//     const rawGeoKeys = new Uint16Array(geokeyRecord.buffer, geokeyRecord.byteOffset);
//     // GeoDoubleParamsTag
//     const doubleRecord = variableLengthRecords[34736]?.data;
//     const GeoDoubleParams =
//       doubleRecord !== undefined
//         ? [...new Float64Array(doubleRecord.buffer, doubleRecord.byteOffset)]
//         : undefined;
//     // GeoAsciiParamsTag
//     const asciiRecord = variableLengthRecords[34737]?.data;
//     const GeoAsciiParams =
//       asciiRecord !== undefined ? this.#decoder.decode(asciiRecord) : undefined;
//     const gkd = parseGeotiffRawGeoKeys(rawGeoKeys, {
//       GeoKeyDirectory: this.GeoKeyDirectory,
//       GeoDoubleParams,
//       GeoAsciiParams,
//     });
//     const gkdParams = buildParamsFromGeoKeys(gkd);
//     if (gkdParams !== undefined) this.transformer.setSource(gkdParams);

//     return gkd;
//   }
// }

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
