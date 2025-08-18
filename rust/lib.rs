#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::collapsible_if)]
// NOTE: Once coverage stabilizes, we can simplify this https://github.com/rust-lang/rust/issues/84605
#![cfg_attr(feature = "nightly", feature(coverage_attribute))]
//! # GIS Tools 🌎 🗺️
//!
//! ```text
//!                 ___                       ___                          
//!                /\  \          ___        /\  \
//!               /::\  \        /\  \      /::\  \
//!              /:/\:\  \       \:\  \    /:/\ \  \
//!             /:/  \:\  \      /::\__\  _\:\~\ \  \
//!            /:/__/_\:\__\  __/:/\/__/ /\ \:\ \ \__\
//!            \:\  /\ \/__/ /\/:/  /    \:\ \:\ \/__/
//!             \:\ \:\__\   \::/__/      \:\ \:\__\
//!              \:\/:/  /    \:\__\       \:\/:/  /
//!               \::/  /      \/__/        \::/  /
//!                \/__/                     \/__/
//!    ___           ___           ___           ___       ___
//!   /\  \         /\  \         /\  \         /\__\     /\  \
//!   \:\  \       /::\  \       /::\  \       /:/  /    /::\  \
//!    \:\  \     /:/\:\  \     /:/\:\  \     /:/  /    /:/\ \  \
//!    /::\  \   /:/  \:\  \   /:/  \:\  \   /:/  /    _\:\~\ \  \
//!   /:/\:\__\ /:/__/ \:\__\ /:/__/ \:\__\ /:/__/    /\ \:\ \ \__\
//!  /:/  \/__/ \:\  \ /:/  / \:\  \ /:/  / \:\  \    \:\ \:\ \/__/
//! /:/  /       \:\  /:/  /   \:\  /:/  /   \:\  \    \:\ \:\__\
//! \/__/         \:\/:/  /     \:\/:/  /     \:\  \    \:\/:/  /
//!                \::/  /       \::/  /       \:\__\    \::/  /
//!                 \/__/         \/__/         \/__/     \/__/
//! ```
//!
//! ## Install
//!
//! ```bash
//! cargo add gis-tools
//! ```
//!
//! ## About
//!
//! A collection of geospatial tools primarily designed for WGS84, Web Mercator, and S2.
//!
//! This library is designed to be **lightweight** with minimal dependencies. All code that doesn't
//! require STD will avoid it, so all readers and writers are accessible to the browser or embedded
//! systems if needed.
//!
//! ## Features
//!
//! Notable features of GIS Tools are:
//!
//! * 🔗 Lightweight, fast, and memory efficient. `no_std` builds for Rust.
//! * 🗺️ Full toolkit support for WGS84, Web Mercator, and S2 projections. Other projections are
//!   coerced under the hood for your benefit.
//! * 🌱 A large list of projections can be converted to/from one of the above 3 via Transformers.
//! * 📦 Build Vector Tiles, Raster Tiles, and Gridded Data Tiles. Vector supports 3 output formats
//!   (Mapbox Vector Tile, Open S2 Tiles, and Flat Open S2 Tiles).
//! * ✅ Most data structures support all projections, but primarily focus the big 3 states above.
//!   Also handle large data sets through working with the filesystem and mmap buffers.
//! * 📖 Contains **23** native GIS readers. The list of readers are: CSV, GBFS, GeoTIFF, GPX, GRIB2,
//!   GTFS, JPEG and JPEG2000, (Geo|S2)JSON, LineDelimted GeoJSON, GeoJSON Text Sequences, LAS, LAZ,
//!   NadGrids, NetCDF, OSM, (S2)PMTiles, Shapefiles, raster and vector tiles, WKT, and XML.
//! * 🦺 Secure code where the only external dependencies are [libm](https://github.com/rust-lang/libm), [half](https://docs.rs/half/latest/half/), [regex](https://github.com/rust-lang/regex), [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), and some optional std [memmap2](https://github.com/RazrFalcon/memmap2-rs), [image](https://github.com/image-rs/image), and [flate2](https://github.com/rust-lang/flate2-rs).
//! * 🧲 Full suite of tools for points, lines, polygons, greater-circle-arcs, predicates, and more.
//! * 🌌 Space specific tools for planets and satellite orbits.
//!
//! ## Goals
//!
//! Making GIS data easy to parse and work with. One of the biggest issues in GIS right now is how
//! segmented various niche tools are. The other issue is how most solutions to read GIS data are
//! half baked, deprecated, or partially parse results that may also need to be transformed one more
//! time to use them.
//!
//! This library exists to make GIS tools simple to use, various data types are fast to parse,
//! and a gaurentee that the parsed data is compatible with said tools. Stored geometry projections
//! will be transformed to be either WGS84 or S2 Projections so you don't have to worry about those
//! complexities.
//!
//! Lastly the goal is for all code to be accessible to both the browser and locally. An example is
//! the Shapefile reader where it can pull from online data or handle extremely large file locally
//! in parallel locally as well.
//!
//! ## Usage
//!
//! The Documentation is very thorough in this library. Therefore the best thing to do is follow
//! the links provided as needed.
//!
//! ### Readers
//!
//! All the following readers implement the [`crate::parsers::FeatureReader`] trait. This is
//! useful because each tool and writer use this trait to work with any reader to ensure consistency
//! of the projection and structure without needing you to worry about the setup. It also allows you
//! to directly read in [`s2json::VectorFeature`] data using the `iter` or `par_iter` functions.
//!
//! - [`crate::readers::GISReader`]: Read in any GIS file type supported by this library.
//! - [`crate::readers::CSVReader`]: Parse (Geo|S2)JSON from a file that is in the CSV format
//! - [`crate::readers::GeoTIFFReader`]: This class reads a GeoTIFF file and returns a list of GeoTIFF images.
//! - [`crate::readers::GPXReader`]: The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.
//! - [`crate::readers::GRIB2Reader`]: This class reads a GRIB2 file and returns a list of GRIB2 products.
//! - [`crate::readers::GTFSScheduleReader`]: Schedule class that pulls in all of the GTFS schedule files and parses them into a single object
//! - [`crate::readers::JSONReader`]: Parse (Geo|S2)JSON. Can handle millions of features.
//! - [`crate::readers::NewLineDelimitedJSONReader`]: Parse (Geo|S2)JSON from a file that is in a newline-delimited format
//! - [`crate::readers::JSONCollectionReader`]: Data parsed using the [`crate::readers::ToGisJSON`] trait can be coerced into this struct
//! - [`crate::readers::LASReader`]: Reads LAS data. Supports up to the LAS 1.4 specification.
//! - [`crate::readers::LAZReader`]: Reads LAS zipped data. Supports LAS 1.4 specification.
//! - [`crate::readers::NadGridReader`]: Loads/reads a binary NTv2 file (.gsb)
//! - [`crate::readers::NetCDFReader`]: Read the NetCDF v3.x file format
//! - [`crate::readers::OSMReader`]: Parses OSM PBF files
//! - [`crate::readers::PMTilesReader`]: A V3.0 PMTiles reader for reading standard WebMercator Tile data and V1.0 S2 Tile data.
//! - [`crate::readers::S2TilesReader`]: An S2 Tile Reader to store tile and metadata in a cloud optimized format.
//! - [`crate::readers::ShapeFileReader`]: Reads data from a Shapefile
//! - [`crate::readers::RasterTileFetcher`]: Read an entire archive of raster tiles, where the max zoom data is iterated upon
//! - [`crate::readers::WKTGeometryReader`]: Parse a collection of WKT geometries from a string
//!
//! ### Tools
//!
//! #### Data Tools
//!
//! - [`crate::tools::Delaunator`]: An incredibly fast and robust Typescript library for Delaunay triangulation of 2D points.
//! - [`crate::tools::Orthodrome`]: Represents an Orthodrome, which is the shortest path between two points on a sphere.
//! - [`crate::tools::polylabels()`]: Find the labels for a collection of vector polygons
//! - [`crate::tools::polylabel()`]: Find the label for a vector polygon
//!
//! #### Geometry Tools
//!
//! Points
//!
//! - [`crate::geometry::AverageOfPoints`]: Get the average of a collection of [`s2json::VectorPoint`].
//! - [`crate::geometry::bearing()`]: Get the bearing in degrees between two points
//! - [`crate::geometry::CenterOfPoints`]: Get the center of a bounding box from a collection of [`s2json::VectorPoint`]
//! - [`crate::geometry::destination()`]: Get the destination given a start point, bearing, and distance
//! - [`crate::geometry::NearestPoint`]: Get the nearest of a collection of [`s2json::VectorPoint`].
//! - [`crate::geometry::ToPoints`]: Convert any geometry shape to a [`s2json::VectorMultiPoint`]
//!
//! Lines
//!
//! - [`crate::geometry::along_line`]: Given a linestring in degrees and a distance, create a [`s2json::VectorPoint`] along the line
//! - [`crate::geometry::LengthOfLines`]: Get the total distance of a line or lines
//! - [`crate::geometry::ToLines`]: Given a Geometry, attempt to Return a VectorLineString.
//!
//! Polygons
//!
//! - [`crate::geometry::Area`]: Get the area of the polygon. Lines return 0 if not closed. Other geometries return 0.
//! - [`crate::geometry::Inside`]: Check if a point is inside a geometry.
//!
//! Clip
//!
//! - [`crate::geometry::clip_point`]: Clip a point to an axis and range
//! - [`crate::geometry::clip_multi_point`]: Clip points to an axis and range
//! - [`crate::geometry::clip_line_string`]: Clip a linestring to an axis and range
//! - [`crate::geometry::clip_multi_line_string`]: Clip a multilinestring to an axis and range
//! - [`crate::geometry::clip_polygon`]: Clip a polygon to an axis and range
//! - [`crate::geometry::clip_multi_polygon`]: Clip a multipolygon to an axis and range
//!
//! #### Interpolation Tools
//!
//! - [`crate::util::average_interpolation`]: Finds the average point in the reference data to the given point and returns its value.
//! - [`crate::util::idw_interpolation`]: Given a reference of data, interpolate a point using inverse distance weighting
//! - [`crate::util::lanczos_interpolation`]: Perform interpolation using the Lanczos filter. This method uses a kernel-based approach to weigh contributions from nearby points, providing a balance between smoothing and sharpness.
//! - [`crate::util::nearest_interpolation`]: Finds the nearest point in the reference data to the given point and returns its value.
//!
//! ### Writers
//!
//! Writers are tools for writing to certain formats, build tiles, etc.
//!
//! All writer tools use the [`crate::parsers::Writer`] trait to write data to. Some of the writers
//! have another level of indirection for writing tiles and use both/either the [`crate::writers::TileWriter`]
//! and [`crate::writers::TemporalTileWriter`] traits.
//!
//! - [`crate::writers::to_json`]: Given a writer and an array of readers, write the input features to the writer as a JSON object
//! - [`crate::writers::to_jsonld`]: Given a writer and an array of readers, write the input features to the writer as JSON-LD
//! - [`crate::writers::PMTilesWriter`]: The File reader is to be used by the local filesystem.
//! - [`crate::writers::S2TilesWriter`]: An S2 Tile Writer to store tile and metadata in a cloud optimized format.
//! - [`crate::writers::FileTileWriter`]: A Folder-File tile writer
//! - [`crate::writers::LocalTileWriter`]: A Local Memory Tile Write Store
//! - [`crate::writers::TileBuilder`]: Create vector tiles, raster tiles, or gridded tiles.
//!
//! ### Geometry
//!
//! #### Longitude Latitude
//!
//! - [`crate::geometry::LonLat`]: A container for a longitude latitude point in degrees.
//!
//! #### Predicates
//!
//! - [`crate::geometry::orient2d()`]: Highly accurate returns a negative value if the points a, b, and c occur in counterclockwise order (c lies to the left of the directed line defined by points a and b).
//! - [`crate::geometry::orient2dfast()`]: Returns a positive value if the points a, b, and c occur in counterclockwise order (c lies to the left of the directed line defined by points a and b).
//! - [`crate::geometry::incirclefast()`]: An in-circle test fast test, lacks the same accuracy as incircle
//!
//! #### S1
//!
//! - [`crate::geometry::S1Angle`]: This struct represents a one-dimensional angle
//! - [`crate::geometry::S1ChordAngle`]: S1ChordAngle represents the angle subtended by a chord
//!
//! #### S2
//!
//! - [`crate::geometry::S2CellId`]: An S2CellId is a 64-bit unsigned integer that uniquely identifies a cell in the S2 cell decomposition.
//! - [`crate::geometry::S2Cap`]: S2Cap represents a disc-shaped region defined by a center and radius.
//! - [`crate::geometry::ConvertVectorFeatureS2`]: Underlying conversion mechanic to move S2 Geometry to GeoJSON Geometry
//! - [`crate::geometry::S2Point`]: An S2Point represents a point on the unit sphere as a 3D vector.
//!
//! #### WGS84 & Web Mercator
//!
//! - [`crate::geometry::ConvertFeature`]: Underlying conversion mechanic to move GeoJSON Feature to GeoJSON Vector Feature
//! - [`crate::geometry::ConvertVectorFeatureWM`]: Underlying conversion mechanic to move GeoJSON Geometry to S2 Geometry
//!
//! #### External
//!
//! - [`s2json::BBox`]: Bounding Box for 2D data
//! - [`s2json::BBox3D`]: Bounding Box for 3D data
//! - [`s2json::VectorPoint`]: Wrapper of a 3D point with an m-value
//! - [`s2json::VectorGeometry`]: WGS84, Web Mercator, and S2 GeoJSON Vector Geometry Wrapper
//! - [`s2json::VectorFeature`]: WGS84, Web Mercator, and S2 GeoJSON Vector Feature Wrapper
//! - [`s2json::Point`]: Wrapper of a 2D point
//! - [`s2json::Point3D`]: Wrapper of a 3D point
//! - [`s2json::Geometry`]: WGS84 GeoJSON Geometry Wrapper
//! - [`s2json::Feature`]: WGS84 GeoJSON Feature Wrapper
//!
//! ### Parsers
//!
//! #### Image
//!
//! - [`crate::parsers::image_decoder`]: Decode any image type into an RGBA buffer
//! - [`crate::parsers::decode_jpeg_data`]: Decode a JPEG image into an RGBA buffer. Used by GeoTIFF
//! - [`crate::parsers::RGBA`]: RGBA data in 0->1 range floats These values remove gamma-corrected values so that you can apply maths on them This means the RGBA values are in linear space
//!
//! #### Readers
//!
//! - [`crate::parsers::Buffer`]: This works as a wrapper around a byte buffer.
//! - [`crate::parsers::Reader`]: Reader interface. Implemented to read data from either a buffer or a filesystem
//! - [`crate::parsers::BufferReader`]: A basic buffer reader for reading data from a buffer
//! - [`crate::parsers::FileReader`]: A file reader for reading data from a file
//! - [`crate::parsers::MMapReader`]: A file reader for reading data from a file using memory mapping
//!
//! #### WKT
//!
//! - [`crate::parsers::parse_wkt_object`]: Parses a WKT object
//!
//! #### Writers
//!
//! - [`crate::parsers::Writer`]: The defacto interface for all writers.
//! - [`crate::parsers::BufferWriter`]: Buffer writer is used on smaller datasets that are easy to write in memory. Faster then the Filesystem
//! - [`crate::parsers::FileWriter`]: A writer that operates on the filesystem storing data in a file
//!
//! #### XML
//!
//! - [`crate::parsers::xml_count_substring`]: Count the number of times a substring appears in a string
//! - [`crate::parsers::xml_find_tag_by_name`]: Find the first tag with the given name
//! - [`crate::parsers::xml_find_tag_by_path`]: Find the first tag with the given path
//! - [`crate::parsers::xml_find_tags_by_name`]: All tags with the given name
//! - [`crate::parsers::xml_find_tags_by_path`]: Find all tags with the given path
//! - [`crate::parsers::xml_get_attribute`]: Get the value of an attribute
//! - [`crate::parsers::xml_index_of_match_end`]: Find the index of the last match
//! - [`crate::parsers::xml_index_of_match`]: Find the index of the first match
//! - [`crate::parsers::xml_remove_comments`]: Remove XML comments from a string
//! - [`crate::parsers::xml_remove_tags_by_name`]: Remove tags given a name
//!
//! ### Proj
//!
//! - [`crate::proj::Transformer`]: A Transformer class contains all projections necessary for converting coordinates from one projection to another.
//! - [`crate::proj::TransformCoordinates`]: Projection trait to modify a Point's values. Used by the Transformer
//! - [`crate::proj::Coords`]: A generic 4-dimensional point/vector
//!
//! ### Space
//!
//! #### Planet Constants
//!
//! Earth:
//! [`crate::space::EARTH_RADIUS`], [`crate::space::EARTH_RADIUS_EQUATORIAL`], [`crate::space::EARTH_RADIUS_POLAR`], [`crate::space::EARTH_CIRCUMFERENCE`], [`crate::space::EARTH_LOWEST_ALTITUDE`], [`crate::space::EARTH_HIGHEST_ALTITUDE`]
//!
//! Jupiter:
//! [`crate::space::JUPITER_RADIUS`], [`crate::space::JUPITER_RADIUS_EQUATORIAL`], [`crate::space::JUPITER_RADIUS_POLAR`], [`crate::space::JUPITER_CIRCUMFERENCE`]
//!
//! Mars:
//! [`crate::space::MARS_RADIUS`], [`crate::space::MARS_RADIUS_EQUATORIAL`], [`crate::space::MARS_RADIUS_POLAR`], [`crate::space::MARS_CIRCUMFERENCE`], [`crate::space::MARS_LOWEST_ALTITUDE`], [`crate::space::MARS_HIGHEST_ALTITUDE`]
//!
//! Mercury:
//! [`crate::space::MERCURY_RADIUS`], [`crate::space::MERCURY_RADIUS_EQUATORIAL`], [`crate::space::MERCURY_RADIUS_POLAR`], [`crate::space::MERCURY_CIRCUMFERENCE`], [`crate::space::MERCURY_LOWEST_ALTITUDE`], [`crate::space::MERCURY_HIGHEST_ALTITUDE`]
//!
//! Moon:
//! [`crate::space::MOON_RADIUS`], [`crate::space::MOON_RADIUS_EQUATORIAL`], [`crate::space::MOON_RADIUS_POLAR`], [`crate::space::MOON_CIRCUMFERENCE`], [`crate::space::MOON_LOWEST_ALTITUDE`], [`crate::space::MOON_HIGHEST_ALTITUDE`]
//!
//! Neptune:
//! [`crate::space::NEPTUNE_RADIUS`], [`crate::space::NEPTUNE_RADIUS_EQUATORIAL`], [`crate::space::NEPTUNE_RADIUS_POLAR`], [`crate::space::NEPTUNE_CIRCUMFERENCE`]
//!
//! Pluto:
//! [`crate::space::PLUTO_RADIUS`], [`crate::space::PLUTO_RADIUS_EQUATORIAL`], [`crate::space::PLUTO_RADIUS_POLAR`], [`crate::space::PLUTO_CIRCUMFERENCE`], [`crate::space::PLUTO_LOWEST_ALTITUDE`], [`crate::space::PLUTO_HIGHEST_ALTITUDE`]
//!
//! Saturn:
//! [`crate::space::SATURN_RADIUS`], [`crate::space::SATURN_RADIUS_EQUATORIAL`], [`crate::space::SATURN_RADIUS_POLAR`], [`crate::space::SATURN_CIRCUMFERENCE`]
//!
//! Venus:
//! [`crate::space::VENUS_RADIUS`], [`crate::space::VENUS_RADIUS_EQUATORIAL`], [`crate::space::VENUS_RADIUS_POLAR`], [`crate::space::VENUS_CIRCUMFERENCE`], [`crate::space::VENUS_LOWEST_ALTITUDE`], [`crate::space::VENUS_HIGHEST_ALTITUDE`]
//!
//! ### Data Structures
//!
//! - [`crate::data_structures::Cache`]: A cache of values with a max size to ensure that too much old data is not stored.
//! - [`crate::data_structures::PointCluster`]: A cluster store to index points at each zoom level
//! - [`crate::data_structures::PointGrid`]: A cluster store to build grid data of grid_size x grid_size. The resultant tiles are filled. Useful for building raster tiles or other grid like data (temperature, precipitation, wind, etc).
//! - [`crate::data_structures::PointIndex`]: An index of cells with radius queries Assumes the data is compatible with S2JSON MValues with serde_json serialization
//! - [`crate::data_structures::PriorityQueue`]: A priority queue is a data structure that stores elements in a specific order.
//! - [`crate::data_structures::Tile`]: Tile Class to contain the tile information for splitting or simplifying
//! - [`crate::data_structures::TileStore`]: TileStore Class is a tile-lookup system that splits and simplifies as needed for each tile request
//! - [`crate::data_structures::TransformVectorGeometry`]: A trait for transforming a geometry from the 0->1 coordinate system to a tile coordinate system
//!
//! ### Utilities
//!
//! - [`crate::util::decompress_fflate`]: Expands compressed GZIP, Zlib/DEFLATE, or DEFLATE_RAW data, automatically detecting the format
//! - [`crate::util::decompress_lzw`]: The decompressed data
//! - [`crate::util::compress_data`]: Compresses data using the specified format
//! - [`crate::util::decompress_data`]: Decompress data using the specified format
//! - [`crate::util::iter_zip_folder`]: Iterates through the items in a zip file
//! - [`crate::util::Date`]: Convenience Date structure to model like a Javascript Date object.
//! - [`crate::util::fetch_url`]: fetch mechanic for raw data

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Data Storage Tools
pub mod data_store;
/// Data structures
pub mod data_structures;
/// Geometry Tools
pub mod geometry;
/// GIS Core Tools
pub mod parsers;
/// Projection Tools
pub mod proj;
/// GIS Readers
pub mod readers;
/// Space Tools
pub mod space;
/// Generic Geospatial Tools
pub mod tools;
/// Utility Tools
pub mod util;
/// GIS Writers
pub mod writers;
