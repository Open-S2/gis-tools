/// CSV Reader
pub mod csv;
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
/// Shapefile Reader
pub mod shapefile;
/// Tile-based Readers
pub mod tile;
/// WKT Geometry Reader
pub mod wkt;

use crate::parsers::{BufferReader, Reader};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};
use core::fmt::Debug;
pub use csv::*;
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
use s2json::{MValue, MValueCompatible, Properties};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub use shapefile::*;
pub use tile::*;
pub use wkt::*;

/// The type of readers to choose from
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Protobuf
    Protobuf,
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
            "jsonsq" | "geojsonsq" | "s2jsonsq" | "json-sq" | "geojson-sq" | "s2json-sq" => {
                ReaderType::JSONSQ
            }
            "las" => ReaderType::LAS,
            "laz" => ReaderType::LAZ,
            "nadgrid" => ReaderType::NADGrid,
            "netcdf" | "nc4" | "cdf" | "nc" => ReaderType::NetCDF,
            "osm" => ReaderType::OSM,
            "shapefile" => ReaderType::Shapefile,
            "tile" => ReaderType::Tile,
            "wkt" => ReaderType::WKT,
            "protobuf" => ReaderType::Protobuf,
            _ => ReaderType::CSV,
        }
    }
}

/// The type of readers to choose from
#[derive(Debug)]
pub enum GISReader<
    T: Reader + Debug,
    M: Clone + DeserializeOwned = (),
    P: DeserializeOwned + MValueCompatible = Properties,
    D: DeserializeOwned + MValueCompatible = MValue,
> {
    /// CSV data
    CSV(Box<CSVReader<T, P>>),
    /// GeoTIFF data
    GeoTIFF(Box<GeoTIFFReader<T>>),
    /// GPX data
    GPX(Box<GPXReader>),
    /// GRIB 2 data
    GRIB2(Box<GRIB2Reader>),
    /// GTFS data
    GTFS(Box<GTFSScheduleReader>),
    /// JSON data
    JSON(Box<JSONReader<T, M, P, D>>),
    /// JSON-LD data
    JSONLD(Box<NewLineDelimitedJSONReader<T, M, P, D>>),
    /// JSON-SQ data
    JSONSQ(Box<SequenceJSONReader<T, M, P, D>>),
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
    Shapefile(Box<ShapeFileReader<T, P>>),
    // /// Tile data
    // Tile(Box<TileReader<P, D>>),
    /// WKT
    WKT(Box<WKTGeometryReader>),
}

/// Given a file and a file type, return a reader
///
/// @param urlPath - The URL path to the file
/// @param type - The file type if specified, otherwise it will be inferred
/// @returns - The reader with {@link FeatureIterator} implemented
pub fn file_type_to_reader<
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned + MValueCompatible,
    D: Clone + Default + DeserializeOwned + MValueCompatible,
>(
    data: Vec<u8>,
    file_type: ReaderType,
    epsg_codes: Option<BTreeMap<String, String>>,
) -> GISReader<BufferReader, M, P, D> {
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
        ReaderType::JSON => GISReader::JSON(JSONReader::new(buffer, None).into()),
        ReaderType::JSONLD => {
            GISReader::JSONLD(NewLineDelimitedJSONReader::new(buffer, None).into())
        }
        ReaderType::JSONSQ => GISReader::JSONSQ(SequenceJSONReader::new(buffer).into()),
        ReaderType::LAS => GISReader::LAS(
            LASReader::new(buffer, Some(LASReaderOptions { epsg_codes, dont_transform: false }))
                .into(),
        ),
        ReaderType::LAZ => GISReader::LAZ(
            LAZReader::new(buffer, Some(LASReaderOptions { epsg_codes, dont_transform: false }))
                .into(),
        ),
        ReaderType::NetCDF => GISReader::NetCDF(NetCDFReader::new(buffer, None).into()),
        ReaderType::OSM => GISReader::OSM(OSMLocalReader::new(buffer, None).into()),
        ReaderType::Shapefile => {
            GISReader::Shapefile(shapefile_from_gzip(&buffer.slice(None, None), epsg_codes).into())
        }
        ReaderType::WKT => {
            let input_str = buffer.parse_string(None, None);
            GISReader::WKT(WKTGeometryReader::new(input_str).into())
        }
        _ => panic!("Unsupported file type: {file_type:?}"),
    }
}
