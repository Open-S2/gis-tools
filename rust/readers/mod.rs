use crate::geometry::VectorFeature;

/// Buffer Reader for reading data from a buffer
pub mod buffer;
/// File Reader for reading data from a file
#[cfg(feature = "std")]
pub mod file;
/// Image based Readers
pub mod image;
/// JSON Reader
pub mod json;
/// Memory Mapped Reader for reading data from a file
#[cfg(feature = "std")]
pub mod mmap;
/// (S2)PMTiles Reader
pub mod pmtiles;
/// Shapefile Reader
pub mod shapefile;

pub use buffer::*;
#[cfg(feature = "std")]
pub use file::*;
pub use image::*;
#[cfg(feature = "std")]
pub use mmap::*;
pub use pmtiles::*;
pub use shapefile::*;

use alloc::{string::String, vec::Vec};
use s2json::MValueCompatible;

/// Reader interface. Implemented to read data from either a buffer or a filesystem
pub trait Reader {
    // Properties
    /// Get the number of bytes in the reader
    fn len(&self) -> usize;
    /// See if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // Getters
    /// Get the big-endian unsigned 64 bit integer at the given byte offset
    fn uint64_be(&mut self, byte_offset: Option<usize>) -> u64;
    /// Get the little-endian unsigned 64 bit integer at the given byte offset
    fn uint64_le(&mut self, byte_offset: Option<usize>) -> u64;
    /// Get the big-endian signed 64 bit integer at the given byte offset
    fn int64_be(&mut self, byte_offset: Option<usize>) -> i64;
    /// Get the little-endian signed 64 bit integer at the given byte offset
    fn int64_le(&mut self, byte_offset: Option<usize>) -> i64;
    /// Get the big-endian floating point 64 bit integer at the given byte offset
    fn f64_be(&mut self, byte_offset: Option<usize>) -> f64;
    /// Get the little-endian floating point 64 bit integer at the given byte offset
    fn f64_le(&mut self, byte_offset: Option<usize>) -> f64;
    /// Get the big-endian unsigned 32 bit integer at the given byte offset
    fn uint32_be(&mut self, byte_offset: Option<usize>) -> u32;
    /// Get the little-endian unsigned 32 bit integer at the given byte offset
    fn uint32_le(&mut self, byte_offset: Option<usize>) -> u32;
    /// Get the big-endian signed 32 bit integer at the given byte offset
    fn int32_be(&mut self, byte_offset: Option<usize>) -> i32;
    /// Get the little-endian signed 32 bit integer at the given byte offset
    fn int32_le(&mut self, byte_offset: Option<usize>) -> i32;
    /// Get the big-endian floating point 32 bit integer at the given byte offset
    fn f32_be(&mut self, byte_offset: Option<usize>) -> f32;
    /// Get the little-endian floating point 32 bit integer at the given byte offset
    fn f32_le(&mut self, byte_offset: Option<usize>) -> f32;
    /// Get the big-endian unsigned 16 bit integer at the given byte offset
    fn uint16_be(&mut self, byte_offset: Option<usize>) -> u16;
    /// Get the little-endian unsigned 16 bit integer at the given byte offset
    fn uint16_le(&mut self, byte_offset: Option<usize>) -> u16;
    /// Get the big-endian signed 16 bit integer at the given byte offset
    fn int16_be(&mut self, byte_offset: Option<usize>) -> i16;
    /// Get the little-endian signed 16 bit integer at the given byte offset
    fn int16_le(&mut self, byte_offset: Option<usize>) -> i16;
    /// Get the big-endian floating point 16 bit integer at the given byte offset
    fn f16_be(&mut self, byte_offset: Option<usize>) -> f32;
    /// Get the little-endian floating point 16 bit integer at the given byte offset
    fn f16_le(&mut self, byte_offset: Option<usize>) -> f32;
    /// Get the unsigned 8 bit integer at the given byte offset
    fn uint8(&mut self, byte_offset: Option<usize>) -> u8;
    /// Get the signed 8 bit integer at the given byte offset
    fn int8(&mut self, byte_offset: Option<usize>) -> i8;
    // Methods
    /// Seek to the given byte offset
    fn tell(&mut self) -> usize;
    /// Seek to the given byte offset
    fn seek(&mut self, pos: usize);
    /// Get a slice of the reader
    fn slice(&mut self, begin: Option<usize>, end: Option<usize>) -> Vec<u8>;
    /// Get a slice of the reader at the current position
    fn seek_slice(&mut self, size: usize) -> Vec<u8>;
    /// Parse a string from the reader
    fn parse_string(&mut self, byte_offset: Option<usize>, byte_length: Option<usize>) -> String;
}

/// A feature iterator that all readers should implement
pub trait FeatureIterator<M: Clone, P: MValueCompatible, D: MValueCompatible>:
    IntoIterator<Item = VectorFeature<M, P, D>>
{
}
