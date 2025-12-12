/// Buffer Reader for reading data from a buffer
mod buffer;
/// Fetch Reader
#[cfg(any(feature = "std", target_arch = "wasm32", feature = "wasm"))]
mod fetch;
/// File Reader for reading data from a file
#[cfg(feature = "std")]
mod file;
/// Memory Mapped Reader for reading data from a file
#[cfg(feature = "std")]
mod mmap;

use alloc::{string::String, vec::Vec};
pub use buffer::*;
#[cfg(any(feature = "std", target_arch = "wasm32", feature = "wasm"))]
pub use fetch::*;
#[cfg(feature = "std")]
pub use file::*;
#[cfg(feature = "std")]
pub use mmap::*;
use s2json::VectorFeature;
#[cfg(feature = "std")]
use std::{io::Result, path::Path};

/// Expands on the Reader trait to include creating a file
#[cfg(feature = "std")]
pub trait StdReader: Reader {
    /// Creates a new file reader from a file path
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: core::marker::Sized;
}

/// Reader interface. Implemented to read data from either a buffer or a filesystem
pub trait Reader: Clone {
    // Properties
    /// Get the number of bytes in the reader
    fn len(&self) -> u64;
    /// See if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // Getters
    /// Get the unsigned 64 bit integer at the given byte offset and endian. Default to big-endian
    fn uint64(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> u64;
    /// Get the big-endian unsigned 64 bit integer at the given byte offset
    fn uint64_be(&self, byte_offset: Option<u64>) -> u64;
    /// Get the little-endian unsigned 64 bit integer at the given byte offset
    fn uint64_le(&self, byte_offset: Option<u64>) -> u64;
    /// Get the signed 64 bit integer at the given byte offset and endian. Default to big-endian
    fn int64(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> i64;
    /// Get the big-endian signed 64 bit integer at the given byte offset
    fn int64_be(&self, byte_offset: Option<u64>) -> i64;
    /// Get the little-endian signed 64 bit integer at the given byte offset
    fn int64_le(&self, byte_offset: Option<u64>) -> i64;
    /// Get the 64 bit floating point at the given byte offset and endian. Default to big-endian
    fn f64(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> f64;
    /// Get the big-endian floating point 64 bit integer at the given byte offset
    fn f64_be(&self, byte_offset: Option<u64>) -> f64;
    /// Get the little-endian floating point 64 bit integer at the given byte offset
    fn f64_le(&self, byte_offset: Option<u64>) -> f64;
    /// Get the unsigned 32 bit integer at the given byte offset and endian. Default to big-endian
    fn uint32(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> u32;
    /// Get the big-endian unsigned 32 bit integer at the given byte offset
    fn uint32_be(&self, byte_offset: Option<u64>) -> u32;
    /// Get the little-endian unsigned 32 bit integer at the given byte offset
    fn uint32_le(&self, byte_offset: Option<u64>) -> u32;
    /// Get the signed 32 bit integer at the given byte offset and endian. Default to big-endian
    fn int32(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> i32;
    /// Get the big-endian signed 32 bit integer at the given byte offset
    fn int32_be(&self, byte_offset: Option<u64>) -> i32;
    /// Get the little-endian signed 32 bit integer at the given byte offset
    fn int32_le(&self, byte_offset: Option<u64>) -> i32;
    /// Get the 32 bit floating point at the given byte offset and endian. Default to big-endian
    fn f32(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> f32;
    /// Get the big-endian floating point 32 bit integer at the given byte offset
    fn f32_be(&self, byte_offset: Option<u64>) -> f32;
    /// Get the little-endian floating point 32 bit integer at the given byte offset
    fn f32_le(&self, byte_offset: Option<u64>) -> f32;
    /// Get the unsigned 16 bit integer at the given byte offset and endian. Default to big-endian
    fn uint16(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> u16;
    /// Get the big-endian unsigned 16 bit integer at the given byte offset
    fn uint16_be(&self, byte_offset: Option<u64>) -> u16;
    /// Get the little-endian unsigned 16 bit integer at the given byte offset
    fn uint16_le(&self, byte_offset: Option<u64>) -> u16;
    /// Get the signed 16 bit integer at the given byte offset and endian. Default to big-endian
    fn int16(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> i16;
    /// Get the big-endian signed 16 bit integer at the given byte offset
    fn int16_be(&self, byte_offset: Option<u64>) -> i16;
    /// Get the little-endian signed 16 bit integer at the given byte offset
    fn int16_le(&self, byte_offset: Option<u64>) -> i16;
    /// Get the 16 bit floating point at the given byte offset and endian. Default to big-endian
    fn f16(&self, byte_offset: Option<u64>, little_endian: Option<bool>) -> f32;
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
    /// Fetch based mechanic. Defaults to slice mechanic
    fn get_slice<'a>(
        &'a self,
        byte_offset: u64,
        byte_length: Option<u64>,
    ) -> impl Future<Output = Vec<u8>> + 'a {
        async move { self.slice(Some(byte_offset), byte_length.map(|l| l + byte_offset)) }
    }
}

/// A feature reader that all readers should implement
pub trait FeatureReader<M: Clone, P: Clone + Default, D: Clone + Default> {
    /// The Feature Reader should implement an iterator of some kind
    type FeatureIterator<'a>: Iterator<Item = VectorFeature<M, P, D>>
    where
        Self: 'a;
    /// All readers have an iter function that returns a Iterator struct
    fn iter(&self) -> Self::FeatureIterator<'_>;
    /// All readers have a par_iter function that returns a ParallelIterator struct
    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_>;
}
