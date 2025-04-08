#[cfg(feature = "std")]
use std::{io::Result, path::Path};

use crate::geometry::VectorFeature;

/// Buffer Reader for reading data from a buffer
pub mod buffer;
/// CSV Reader
pub mod csv;
/// File Reader for reading data from a file
#[cfg(feature = "std")]
pub mod file;
/// GPX Reader
pub mod gpx;
/// Image based Readers
pub mod image;
/// JSON Reader
pub mod json;
/// Memory Mapped Reader for reading data from a file
#[cfg(feature = "std")]
pub mod mmap;
/// OSM (Open Street Map) PBF Reader
pub mod osm;
/// (S2)PMTiles Reader
pub mod pmtiles;
/// Shapefile Reader
pub mod shapefile;
/// Tile based readers
pub mod tile;
/// WKT Parsing of various formats
pub mod wkt;
/// XML Parser
pub mod xml;

use alloc::{string::String, vec::Vec};
pub use buffer::*;
pub use csv::*;
#[cfg(feature = "std")]
pub use file::*;
pub use gpx::*;
pub use image::*;
#[cfg(feature = "std")]
pub use mmap::*;
pub use osm::*;
pub use pmtiles::*;
use serde::{Deserialize, Serialize};
pub use shapefile::*;
pub use tile::*;
pub use wkt::*;
pub use xml::*;

/// The type of readers to choose from
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReaderType {
    /// CSV data
    CSV,
    /// GPX data
    GPX,
    /// JSON data
    JSON,
    /// OSM data
    OSM,
    /// (S2)PMTiles data
    PMTiles,
    /// Shapefile
    Shapefile,
    /// Tile data
    Tile,
    /// WKT
    WKT,
    /// Protobuf
    Protobuf,
}

/// Expands on the Reader trait to include creating a file
#[cfg(feature = "std")]
pub trait StdReader: Reader {
    /// Creates a new file reader from a file path
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: core::marker::Sized;
}

/// Reader interface. Implemented to read data from either a buffer or a filesystem
pub trait Reader {
    // Properties
    /// Get the number of bytes in the reader
    fn len(&self) -> u64;
    /// See if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // Getters
    /// Get the big-endian unsigned 64 bit integer at the given byte offset
    fn uint64_be(&self, byte_offset: Option<u64>) -> u64;
    /// Get the little-endian unsigned 64 bit integer at the given byte offset
    fn uint64_le(&self, byte_offset: Option<u64>) -> u64;
    /// Get the big-endian signed 64 bit integer at the given byte offset
    fn int64_be(&self, byte_offset: Option<u64>) -> i64;
    /// Get the little-endian signed 64 bit integer at the given byte offset
    fn int64_le(&self, byte_offset: Option<u64>) -> i64;
    /// Get the big-endian floating point 64 bit integer at the given byte offset
    fn f64_be(&self, byte_offset: Option<u64>) -> f64;
    /// Get the little-endian floating point 64 bit integer at the given byte offset
    fn f64_le(&self, byte_offset: Option<u64>) -> f64;
    /// Get the big-endian unsigned 32 bit integer at the given byte offset
    fn uint32_be(&self, byte_offset: Option<u64>) -> u32;
    /// Get the little-endian unsigned 32 bit integer at the given byte offset
    fn uint32_le(&self, byte_offset: Option<u64>) -> u32;
    /// Get the big-endian signed 32 bit integer at the given byte offset
    fn int32_be(&self, byte_offset: Option<u64>) -> i32;
    /// Get the little-endian signed 32 bit integer at the given byte offset
    fn int32_le(&self, byte_offset: Option<u64>) -> i32;
    /// Get the big-endian floating point 32 bit integer at the given byte offset
    fn f32_be(&self, byte_offset: Option<u64>) -> f32;
    /// Get the little-endian floating point 32 bit integer at the given byte offset
    fn f32_le(&self, byte_offset: Option<u64>) -> f32;
    /// Get the big-endian unsigned 16 bit integer at the given byte offset
    fn uint16_be(&self, byte_offset: Option<u64>) -> u16;
    /// Get the little-endian unsigned 16 bit integer at the given byte offset
    fn uint16_le(&self, byte_offset: Option<u64>) -> u16;
    /// Get the big-endian signed 16 bit integer at the given byte offset
    fn int16_be(&self, byte_offset: Option<u64>) -> i16;
    /// Get the little-endian signed 16 bit integer at the given byte offset
    fn int16_le(&self, byte_offset: Option<u64>) -> i16;
    /// Get the big-endian floating point 16 bit integer at the given byte offset
    fn f16_be(&self, byte_offset: Option<u64>) -> f32;
    /// Get the little-endian floating point 16 bit integer at the given byte offset
    fn f16_le(&self, byte_offset: Option<u64>) -> f32;
    /// Get the unsigned 8 bit integer at the given byte offset
    fn uint8(&self, byte_offset: Option<u64>) -> u8;
    /// Get the signed 8 bit integer at the given byte offset
    fn int8(&self, byte_offset: Option<u64>) -> i8;
    // Methods
    /// Seek to the given byte offset
    fn tell(&self) -> u64;
    /// Seek to the given byte offset
    fn seek(&self, pos: u64);
    /// Get a slice of the reader
    fn slice(&self, begin: Option<u64>, end: Option<u64>) -> Vec<u8>;
    /// Get a slice of the reader at the current position
    fn seek_slice(&self, size: usize) -> Vec<u8>;
    /// Parse a string from the reader
    fn parse_string(&self, byte_offset: Option<u64>, byte_length: Option<u64>) -> String;
}

/// A feature reader that all readers should implement
pub trait FeatureReader<M: Clone, P: Clone + Default, D: Clone + Default> {
    /// The Feature Reader should implement an iterator of some kind
    type FeatureIterator<'a>: Iterator<Item = VectorFeature<M, P, D>>
    where
        Self: 'a;
    /// All readers have an iter function that returns a Iterator struct
    fn iter(&self) -> Self::FeatureIterator<'_>;
}
