use crate::geometry::VectorFeature;

/// Buffer Reader for reading data from a buffer
pub mod buffer;

// use buffer::*;

/// Reader interface. Implemented to read data from either a buffer or a filesystem
pub trait Reader {
    // Properties
    /// Get the number of bytes in the reader
    fn byte_length(&self) -> usize;
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
    fn slice(&mut self, begin: Option<usize>, end: Option<usize>) -> &[u8];
    /// Get a mutable slice of the reader
    fn slice_mut(&mut self, begin: Option<usize>, end: Option<usize>) -> &mut [u8];
    /// Get a slice of the reader at the current position
    fn seek_slice(&mut self, size: usize) -> &[u8];
    /// Get a mutable slice of the reader at the current position
    fn seek_slice_mut(&mut self, size: usize) -> &mut [u8];
    /// Parse a string from the reader
    fn parse_string(&mut self, byte_offset: Option<usize>, byte_length: Option<usize>) -> &str;
}

/// A feature iterator that all readers should implement
pub trait FeatureIterator<M = ()>: Iterator<Item = VectorFeature<M>> {
    /// Get the next feature
    fn next_feature(&mut self) -> Option<VectorFeature<M>>;
}
