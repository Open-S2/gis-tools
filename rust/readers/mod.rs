/// CSV Reader
pub mod csv;
/// GBFS Readers
pub mod gbfs;
/// GeoTIFF Reader
pub mod geotiff;
/// GPX Reader
pub mod gpx;
/// Grib2 Reader
pub mod grib2;
/// GTFS Readers
pub mod gtfs;
/// JSON Readers
pub mod json;
/// LAS/LAZ Readers
pub mod las;
/// NAD Grid Reader
pub mod nadgrid;
/// NetCDF Reader
pub mod netcdf;
/// OSM (Open Street Map) PBF Reader
pub mod osm;
/// (S2)PMTiles Reader
pub mod pmtiles;
/// S2 Tiles Reader
pub mod s2tiles;
/// Shapefile Reader
pub mod shapefile;
/// Tile-based Readers
pub mod tile;
/// WKT Geometry Reader
pub mod wkt;

#[cfg(feature = "std")]
use crate::parsers::FileReader;
use crate::parsers::{BufferReader, FeatureReader, Reader};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};
use core::fmt::Debug;
pub use csv::*;
pub use gbfs::*;
pub use geotiff::*;
pub use gpx::*;
pub use grib2::*;
pub use gtfs::*;
pub use image::*;
pub use json::*;
pub use las::*;
pub use nadgrid::*;
pub use netcdf::*;
pub use osm::*;
pub use pmtiles::*;
use s2json::{MValue, Properties, VectorFeature};
pub use s2tiles::*;
use serde::{Deserialize, Serialize};
pub use shapefile::*;
#[cfg(feature = "std")]
use std::path::Path;
pub use tile::*;
pub use wkt::*;

/// The type of readers to choose from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReaderType {
    /// CSV data
    CSV,
    /// GeoTIFF data
    GeoTIFF,
    /// GPX data
    GPX,
    /// GRIB 2 data
    GRIB2,
    /// GTFS data
    GTFS,
    /// JSON data
    JSON,
    /// JSON-LD data
    JSONLD,
    /// JSON-SQ data
    JSONSQ,
    /// LAS data
    LAS,
    /// LAZ data
    LAZ,
    /// NAD Grid data
    NADGrid,
    /// NetCDF data
    NetCDF,
    /// OSM data
    OSM,
    /// Shapefile
    Shapefile,
    /// Tile data
    Tile,
    /// WKT
    WKT,
}
impl From<&str> for ReaderType {
    fn from(value: &str) -> Self {
        match value {
            "csv" => ReaderType::CSV,
            "geotiff" | "tif" | "tiff" | "geotif" => ReaderType::GeoTIFF,
            "gpx" => ReaderType::GPX,
            "grib2" | "grib" => ReaderType::GRIB2,
            "gtfs" => ReaderType::GTFS,
            "json" | "geojson" | "s2json" => ReaderType::JSON,
            "jsonld" | "geojsonld" | "s2jsonld" | "json-ld" | "geojson-ld" | "s2json-ld" => {
                ReaderType::JSONLD
            }
            "jsonsq" | "geojsonseq" | "geojsonsq" | "s2jsonseq" | "s2jsonsq" | "json-seq"
            | "json-sq" | "geojson-seq" | "geojson-sq" | "s2json-seq" | "s2json-sq" => {
                ReaderType::JSONSQ
            }
            "las" => ReaderType::LAS,
            "laz" => ReaderType::LAZ,
            "nadgrid" | "nad" | "gsb" => ReaderType::NADGrid,
            "netcdf" | "nc4" | "cdf" | "nc" => ReaderType::NetCDF,
            "osm" | "pbf" => ReaderType::OSM,
            "shapefile" | "shp" | "zip" => ReaderType::Shapefile,
            "wkt" => ReaderType::WKT,
            _ => ReaderType::Tile,
        }
    }
}

/// # GIS Reader
///
/// ## Description
/// Parse all data types supported by this library
///
/// Implements the [`FeatureReader`] trait
///
/// It is recommended to use this reader for testing or ease of access, but the better
/// alternative is to use the file type readers directly. Here is the list of readers:
/// - [`CSVReader`]: Parse (Geo|S2)JSON from a file that is in the CSV format
/// - [`GeoTIFFReader`]: This class reads a GeoTIFF file and returns a list of GeoTIFF images.
/// - [`GPXReader`]: The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.
/// - [`GRIB2Reader`]: This class reads a GRIB2 file and returns a list of GRIB2 products.
/// - [`GTFSScheduleReader`]: Schedule class that pulls in all of the GTFS schedule files and parses them into a single object
/// - [`JSONReader`]: Parse (Geo|S2)JSON. Can handle millions of features.
/// - [`NewLineDelimitedJSONReader`]: Parse (Geo|S2)JSON from a file that is in a newline-delimited format
/// - [`SequenceJSONReader`]: Parse GeoJSON from a file that is in the `geojson-text-sequences` format.
/// - [`LASReader`]: Reads LAS data. Supports up to the LAS 1.4 specification.
/// - [`LAZReader`]: Reads LAS zipped data. Supports LAS 1.4 specification although missing some support.
/// - [`NadGridReader`]: Loads/reads a binary NTv2 file (.gsb) implementing the {@link FeatureIterator} interface.
/// - [`NetCDFReader`]: Read the NetCDF v3.x file format.
/// - [`OSMLocalReader`]: OSM PBF Data. Direct use allows for the use of the [`OSMFileReader`] as well
/// - [`ShapeFileReader`]: Reads data from a shapefile implementing the {@link FeatureIterator} interface
/// - [`WKTGeometryReader`]: Parse a collection of WKT geometries from a string
///
/// ## Usage
///
/// ### Read from a file
/// ```rust
/// use gistools::{
///     parsers::{FeatureReader},
///     readers::{GISReader, ReaderType},
/// };
/// use s2json::{MValue, Properties, VectorFeature};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/csv/fixtures/basic.csv");
///
/// let reader = GISReader::from_path(path, None, None);
/// assert_eq!(reader.get_type(), ReaderType::CSV);
///
/// let features: Vec<VectorFeature<(), Properties, MValue>> = reader.iter().collect();
/// ```
///
/// ### Read from a buffer
///
/// It is recommended to use a Buffer Reader when the file is small because it is more efficient
///
/// ```rust
/// use gistools::{
///     parsers::{FeatureReader},
///     readers::{GISReader, ReaderType},
/// };
/// use s2json::{MValue, Properties, VectorFeature};
///
/// // ignore the use of the filesystem, setup is just for the example
/// use std::path::PathBuf;
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/csv/fixtures/basic.csv");
/// let bytes = std::fs::read(path.clone()).unwrap();
///
/// let reader = GISReader::from_buffer(bytes, ReaderType::CSV, None);
/// let features: Vec<VectorFeature<(), Properties, MValue>> = reader.iter().collect();
/// ```
#[derive(Debug)]
pub enum GISReader<T: Reader + Debug> {
    /// CSV data
    CSV(Box<CSVReader<T, Properties>>),
    /// GeoTIFF data
    GeoTIFF(Box<GeoTIFFReader<T>>),
    /// GPX data
    GPX(Box<GPXReader>),
    /// GRIB 2 data
    GRIB2(Box<GRIB2Reader>),
    /// GTFS data
    GTFS(Box<GTFSScheduleReader>),
    /// JSON data
    JSON(Box<JSONReader<T, (), Properties, MValue>>),
    /// JSON-LD data
    JSONLD(Box<NewLineDelimitedJSONReader<T, (), Properties, MValue>>),
    /// JSON-SQ data
    JSONSQ(Box<SequenceJSONReader<T, (), Properties, MValue>>),
    /// LAS data
    LAS(Box<LASReader<T>>),
    /// LAZ data
    LAZ(Box<LAZReader<T>>),
    /// NAD Grid data
    NADGrid(Box<NadGridReader<T>>),
    /// NetCDF data
    NetCDF(Box<NetCDFReader<T>>),
    /// OSM data
    OSM(Box<OSMLocalReader<T>>),
    /// Shapefile
    Shapefile(Box<ShapeFileReader<T, Properties>>),
    // /// Tile data
    // Tile(Box<TileReader<P, D>>),
    /// WKT
    WKT(Box<WKTGeometryReader>),
}
impl<T: Reader + Debug> GISReader<T> {
    /// Get the type of the reader
    pub fn get_type(&self) -> ReaderType {
        match self {
            GISReader::CSV(_) => ReaderType::CSV,
            GISReader::GeoTIFF(_) => ReaderType::GeoTIFF,
            GISReader::GPX(_) => ReaderType::GPX,
            GISReader::GRIB2(_) => ReaderType::GRIB2,
            GISReader::GTFS(_) => ReaderType::GTFS,
            GISReader::JSON(_) => ReaderType::JSON,
            GISReader::JSONLD(_) => ReaderType::JSONLD,
            GISReader::JSONSQ(_) => ReaderType::JSONSQ,
            GISReader::LAS(_) => ReaderType::LAS,
            GISReader::LAZ(_) => ReaderType::LAZ,
            GISReader::NADGrid(_) => ReaderType::NADGrid,
            GISReader::NetCDF(_) => ReaderType::NetCDF,
            GISReader::OSM(_) => ReaderType::OSM,
            GISReader::Shapefile(_) => ReaderType::Shapefile,
            // GISReader::Tile(_) => ReaderType::Tile,
            GISReader::WKT(_) => ReaderType::WKT,
        }
    }
}
impl GISReader<BufferReader> {
    /// Given a raw data and a file type, return the appropriate reader
    ///
    /// ## Parameters
    ///
    /// - `data`: The data to parse
    /// - `file_type`: The file type to parse the data as
    /// - `epsg_codes`: The EPSG codes to use. E.g. `{"4326": "...WKT STRING..."}`
    ///
    /// ## Returns
    ///
    /// The [`GISReader`] using a [`BufferReader`] for fast parsing
    pub fn from_buffer(
        data: Vec<u8>,
        file_type: ReaderType,
        epsg_codes: Option<BTreeMap<String, String>>,
    ) -> GISReader<BufferReader> {
        let buffer = BufferReader::new(data);
        let epsg_codes = epsg_codes.unwrap_or_default();
        match file_type {
            ReaderType::CSV => GISReader::CSV(CSVReader::new(buffer, None).into()),
            ReaderType::GeoTIFF => GISReader::GeoTIFF(
                GeoTIFFReader::new(buffer, Some(GeoTIFFOptions { epsg_codes })).into(),
            ),
            ReaderType::GPX => {
                let input_str = buffer.parse_string(None, None);
                GISReader::GPX(GPXReader::new(&input_str).into())
            }
            ReaderType::GRIB2 => GISReader::GRIB2(GRIB2Reader::new(buffer.into(), vec![]).into()),
            ReaderType::GTFS => {
                GISReader::GTFS(GTFSScheduleReader::from_gzip(&buffer.slice(None, None)).into())
            }
            ReaderType::JSON => GISReader::JSON(JSONReader::new(buffer).into()),
            ReaderType::JSONLD => {
                GISReader::JSONLD(NewLineDelimitedJSONReader::new(buffer, None).into())
            }
            ReaderType::JSONSQ => GISReader::JSONSQ(SequenceJSONReader::new(buffer).into()),
            ReaderType::LAS => GISReader::LAS(
                LASReader::new(
                    buffer,
                    Some(LASReaderOptions { epsg_codes, dont_transform: false }),
                )
                .into(),
            ),
            ReaderType::LAZ => GISReader::LAZ(
                LAZReader::new(
                    buffer,
                    Some(LASReaderOptions { epsg_codes, dont_transform: false }),
                )
                .into(),
            ),
            ReaderType::NADGrid => {
                GISReader::NADGrid(NadGridReader::new("default".into(), buffer).into())
            }
            ReaderType::NetCDF => GISReader::NetCDF(NetCDFReader::new(buffer, None).into()),
            ReaderType::OSM => {
                let mut osm = OSMLocalReader::new(buffer, None);
                osm.parse_blocks();
                GISReader::OSM(osm.into())
            }
            ReaderType::Shapefile => GISReader::Shapefile(
                shapefile_from_gzip(&buffer.slice(None, None), epsg_codes).into(),
            ),
            ReaderType::WKT => {
                let input_str = buffer.parse_string(None, None);
                GISReader::WKT(WKTGeometryReader::new(input_str).into())
            }
            _ => panic!("Unsupported file type: {file_type:?}"),
        }
    }
}
impl GISReader<FileReader> {
    /// Given a file and a file type (or inferred if not provided), return a reader
    ///
    /// ## Parameters
    ///
    /// - `file`: The path to the file
    /// - `file_type`: The file type if specified, otherwise it will be inferred. Useful for `zip` files
    /// - `epsg_codes`: The EPSG codes to use. E.g. `{"4326": "...WKT STRING..."}`
    ///
    /// ## Returns
    ///
    /// The [`GISReader`] using a [`FileReader`]
    #[cfg(feature = "std")]
    pub fn from_path<P: AsRef<Path>>(
        file: P,
        file_type: Option<ReaderType>,
        epsg_codes: Option<BTreeMap<String, String>>,
    ) -> GISReader<FileReader> {
        use crate::readers::file::shapefile_from_path;
        use std::{ffi::OsStr, fs};

        let path = file.as_ref().to_path_buf();
        let path_ending = path.extension().and_then(OsStr::to_str).unwrap_or("");
        let file_type: ReaderType = file_type.unwrap_or(path_ending.into());
        let epsg_codes = epsg_codes.unwrap_or_default();
        match file_type {
            ReaderType::CSV => {
                GISReader::CSV(CSVReader::new(FileReader::new(file).unwrap(), None).into())
            }
            ReaderType::GeoTIFF => GISReader::GeoTIFF(
                GeoTIFFReader::new(
                    FileReader::new(file).unwrap(),
                    Some(GeoTIFFOptions { epsg_codes }),
                )
                .into(),
            ),
            ReaderType::GPX => {
                GISReader::GPX(GPXReader::new(&fs::read_to_string(file).unwrap()).into())
            }
            ReaderType::GRIB2 => GISReader::GRIB2(
                GRIB2Reader::new(FileReader::new(file).unwrap().into(), vec![]).into(),
            ),
            ReaderType::GTFS => {
                GISReader::GTFS(GTFSScheduleReader::from_gzip(&fs::read(file).unwrap()).into())
            }
            ReaderType::JSON => {
                GISReader::JSON(JSONReader::new(FileReader::new(file).unwrap()).into())
            }
            ReaderType::JSONLD => GISReader::JSONLD(
                NewLineDelimitedJSONReader::new(FileReader::new(file).unwrap(), None).into(),
            ),
            ReaderType::JSONSQ => {
                GISReader::JSONSQ(SequenceJSONReader::new(FileReader::new(file).unwrap()).into())
            }
            ReaderType::LAS => GISReader::LAS(
                LASReader::new(
                    FileReader::new(file).unwrap(),
                    Some(LASReaderOptions { epsg_codes, dont_transform: false }),
                )
                .into(),
            ),
            ReaderType::LAZ => GISReader::LAZ(
                LAZReader::new(
                    FileReader::new(file).unwrap(),
                    Some(LASReaderOptions { epsg_codes, dont_transform: false }),
                )
                .into(),
            ),
            ReaderType::NADGrid => GISReader::NADGrid(
                NadGridReader::new("default".into(), FileReader::new(file).unwrap()).into(),
            ),
            ReaderType::NetCDF => {
                GISReader::NetCDF(NetCDFReader::new(FileReader::new(file).unwrap(), None).into())
            }
            ReaderType::OSM => {
                let mut osm = OSMLocalReader::new(FileReader::new(file).unwrap(), None);
                osm.parse_blocks();
                GISReader::OSM(osm.into())
            }
            ReaderType::Shapefile => {
                // if file ends in zip, use shapefile_from_zip
                if path_ending == "zip" {
                    unimplemented!("Shapefile from zip not implemented yet")
                } else {
                    GISReader::Shapefile(shapefile_from_path(file, epsg_codes).into())
                }
            }
            ReaderType::WKT => {
                let input_str = fs::read_to_string(file).unwrap();
                GISReader::WKT(WKTGeometryReader::new(input_str).into())
            }
            _ => panic!("Unsupported file type: {file_type:?}"),
        }
    }
}

/// The Global GIS Iterator tool
#[derive(Debug)]
pub enum GISIterator<'a, T: Reader + Debug> {
    /// CSV Iterator
    CSV(CSVIterator<'a, T, Properties>),
    /// GeoTIFF Iterator
    GeoTIFF(GeoTIFFIterator<'a, T>),
    /// GPX Iterator
    GPX(GPXIterator<'a>),
    /// GRIB2 Iterator
    GRIB2(GRIB2Iterator<'a>),
    /// GTFS Iterator
    GTFS(GTFSScheduleIterator),
    /// JSON Iterator
    JSON(JSONIterator<'a, T, (), Properties, MValue>),
    /// JSON-LD Iterator
    JSONLD(NewLineDelimitedJSONIterator<'a, T, (), Properties, MValue>),
    /// JSON-SQ Iterator
    JSONSQ(SequenceJSONIterator<'a, T, (), Properties, MValue>),
    /// LAS Iterator
    LAS(LASIterator<'a, T>),
    /// LAZ Iterator
    LAZ(LAZIterator<'a, T>),
    /// NAD Grid Iterator
    NADGrid(NadGridIterator<'a, T>),
    /// NetCDF Iterator
    NetCDF(CDFIterator<'a, T>),
    /// OSM Iterator
    OSM(OSMLocalReaderIter<'a, T>),
    /// Shapefile Iterator
    Shapefile(ShapefileIterator<'a, T, Properties>),
    /// WKT Iterator
    WKT(WKTIterator<'a>),
}
impl<'a, T: Reader + Debug> Iterator for GISIterator<'a, T> {
    type Item = VectorFeature<(), Properties, MValue>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            GISIterator::CSV(iterator) => iterator.next().map(|f| f.to_m_vector_feature(|_| None)),
            GISIterator::GeoTIFF(iterator) => {
                iterator.next().map(|f| f.to_m_vector_feature(|_| None))
            }
            GISIterator::GPX(iterator) => iterator.next().map(|f| f.to_m_vector_feature(|_| None)),
            GISIterator::GRIB2(iterator) => {
                iterator.next().map(|f| f.to_m_vector_feature(|_| None))
            }
            GISIterator::GTFS(iterator) => iterator.next(),
            GISIterator::JSON(iterator) => iterator.next(),
            GISIterator::JSONLD(iterator) => iterator.next(),
            GISIterator::JSONSQ(iterator) => iterator.next(),
            GISIterator::LAS(iterator) => iterator.next().map(|f| f.to_m_vector_feature(|_| None)),
            GISIterator::LAZ(iterator) => iterator.next().map(|f| f.to_m_vector_feature(|_| None)),
            GISIterator::NADGrid(iterator) => iterator.next().map(|f| VectorFeature {
                _type: f._type,
                id: f.id,
                face: f.face,
                properties: f.properties,
                geometry: f.geometry,
                metadata: None,
            }),
            GISIterator::NetCDF(iterator) => iterator.next(),
            GISIterator::OSM(iterator) => iterator.next().map(|f| VectorFeature {
                _type: f._type,
                id: f.id,
                face: f.face,
                properties: f.properties,
                geometry: f.geometry,
                metadata: None,
            }),
            GISIterator::Shapefile(iterator) => iterator.next(),
            GISIterator::WKT(iterator) => iterator.next(),
        }
    }
}
impl<T: Reader + Debug> FeatureReader<(), Properties, MValue> for GISReader<T> {
    type FeatureIterator<'a>
        = GISIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        match self {
            GISReader::CSV(reader) => GISIterator::CSV(reader.iter()),
            GISReader::GeoTIFF(reader) => GISIterator::GeoTIFF(reader.iter()),
            GISReader::GPX(reader) => GISIterator::GPX(reader.iter()),
            GISReader::GRIB2(reader) => GISIterator::GRIB2(reader.iter()),
            GISReader::GTFS(reader) => GISIterator::GTFS(reader.iter()),
            GISReader::JSON(reader) => GISIterator::JSON(reader.iter()),
            GISReader::JSONLD(reader) => GISIterator::JSONLD(reader.iter()),
            GISReader::JSONSQ(reader) => GISIterator::JSONSQ(reader.iter()),
            GISReader::LAS(reader) => GISIterator::LAS(reader.iter()),
            GISReader::LAZ(reader) => GISIterator::LAZ(reader.iter()),
            GISReader::NADGrid(reader) => GISIterator::NADGrid(reader.iter()),
            GISReader::NetCDF(reader) => GISIterator::NetCDF(reader.iter()),
            GISReader::OSM(reader) => GISIterator::OSM(reader.iter()),
            GISReader::Shapefile(reader) => GISIterator::Shapefile(reader.iter()),
            GISReader::WKT(reader) => GISIterator::WKT(reader.iter()),
        }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        match self {
            GISReader::CSV(reader) => GISIterator::CSV(reader.par_iter(pool_size, thread_id)),
            GISReader::GeoTIFF(reader) => {
                GISIterator::GeoTIFF(reader.par_iter(pool_size, thread_id))
            }
            GISReader::GPX(reader) => GISIterator::GPX(reader.par_iter(pool_size, thread_id)),
            GISReader::GRIB2(reader) => GISIterator::GRIB2(reader.par_iter(pool_size, thread_id)),
            GISReader::GTFS(reader) => GISIterator::GTFS(reader.par_iter(pool_size, thread_id)),
            GISReader::JSON(reader) => GISIterator::JSON(reader.par_iter(pool_size, thread_id)),
            GISReader::JSONLD(reader) => GISIterator::JSONLD(reader.par_iter(pool_size, thread_id)),
            GISReader::JSONSQ(reader) => GISIterator::JSONSQ(reader.par_iter(pool_size, thread_id)),
            GISReader::LAS(reader) => GISIterator::LAS(reader.par_iter(pool_size, thread_id)),
            GISReader::LAZ(reader) => GISIterator::LAZ(reader.par_iter(pool_size, thread_id)),
            GISReader::NADGrid(reader) => {
                GISIterator::NADGrid(reader.par_iter(pool_size, thread_id))
            }
            GISReader::NetCDF(reader) => GISIterator::NetCDF(reader.par_iter(pool_size, thread_id)),
            GISReader::OSM(reader) => GISIterator::OSM(reader.par_iter(pool_size, thread_id)),
            GISReader::Shapefile(reader) => {
                GISIterator::Shapefile(reader.par_iter(pool_size, thread_id))
            }
            GISReader::WKT(reader) => GISIterator::WKT(reader.par_iter(pool_size, thread_id)),
        }
    }
}
