use crate::readers::Reader;

use alloc::str::from_utf8;
use alloc::vec::Vec;

/// A basic buffer reader for reading data from a buffer
#[derive(Default, Debug)]
pub struct BufferReader {
    /// The buffer
    pub buffer: Vec<u8>, // This struct contains some data
    cursor: usize,
}
impl BufferReader {
    /// Creates a new buffer reader
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, cursor: 0 }
    }
}
impl Reader for BufferReader {
    fn byte_length(&self) -> usize {
        self.buffer.len()
    }

    // GETTERS

    fn uint64_be(&mut self, byte_offset: Option<usize>) -> u64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        u64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn uint64_le(&mut self, byte_offset: Option<usize>) -> u64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        u64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn int64_be(&mut self, byte_offset: Option<usize>) -> i64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        i64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn int64_le(&mut self, byte_offset: Option<usize>) -> i64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        i64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_be(&mut self, byte_offset: Option<usize>) -> f64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        f64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_le(&mut self, byte_offset: Option<usize>) -> f64 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 8 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 8];
        self.cursor += 8;
        f64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn uint32_be(&mut self, byte_offset: Option<usize>) -> u32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        u32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint32_le(&mut self, byte_offset: Option<usize>) -> u32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        u32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_be(&mut self, byte_offset: Option<usize>) -> i32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        i32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_le(&mut self, byte_offset: Option<usize>) -> i32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        i32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_be(&mut self, byte_offset: Option<usize>) -> f32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        f32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_le(&mut self, byte_offset: Option<usize>) -> f32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 4 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 4];
        self.cursor += 4;
        f32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint16_be(&mut self, byte_offset: Option<usize>) -> u16 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        u16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn uint16_le(&mut self, byte_offset: Option<usize>) -> u16 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        u16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_be(&mut self, byte_offset: Option<usize>) -> i16 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        i16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_le(&mut self, byte_offset: Option<usize>) -> i16 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        i16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn f16_be(&mut self, byte_offset: Option<usize>) -> f32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        let f = f16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn f16_le(&mut self, byte_offset: Option<usize>) -> f32 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 2 > self.buffer.len());

        let bytes = &self.buffer[offset..offset + 2];
        self.cursor += 2;
        let f = f16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn uint8(&mut self, byte_offset: Option<usize>) -> u8 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 1 > self.buffer.len());

        let byte = self.buffer[offset];
        self.cursor += 1;
        byte
    }
    fn int8(&mut self, byte_offset: Option<usize>) -> i8 {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + 1 > self.buffer.len());

        let byte = self.buffer[offset];
        self.cursor += 1;
        byte.try_into().unwrap()
    }

    // Methods

    fn tell(&mut self) -> usize {
        self.cursor
    }
    fn seek(&mut self, pos: usize) {
        self.cursor = pos;
    }
    fn slice(&mut self, begin: Option<usize>, end: Option<usize>) -> &[u8] {
        let begin = begin.unwrap_or(self.cursor);
        let end = end.unwrap_or(self.buffer.len());
        &self.buffer[begin..end]
    }
    fn slice_mut(&mut self, begin: Option<usize>, end: Option<usize>) -> &mut [u8] {
        let begin = begin.unwrap_or(self.cursor);
        let end = end.unwrap_or(self.buffer.len());
        &mut self.buffer[begin..end]
    }
    fn seek_slice(&mut self, size: usize) -> &[u8] {
        self.slice(Some(self.cursor), Some(self.cursor + size))
    }
    fn seek_slice_mut(&mut self, size: usize) -> &mut [u8] {
        self.slice_mut(Some(self.cursor), Some(self.cursor + size))
    }
    fn parse_string(&mut self, byte_offset: Option<usize>, byte_length: Option<usize>) -> &str {
        let offset = byte_offset.unwrap_or(self.cursor);
        let length = byte_length.unwrap_or(self.buffer.len() - offset);
        let string = from_utf8(&self.buffer[offset..offset + length]).unwrap();
        self.cursor += length;
        string
    }
}
impl From<Vec<u8>> for BufferReader {
    fn from(buffer: Vec<u8>) -> Self {
        BufferReader::new(buffer) // Uses the `new` constructor to initialize the `BufferReader`
    }
}
impl From<&[u8]> for BufferReader {
    fn from(buffer: &[u8]) -> Self {
        BufferReader::new(buffer.to_vec()) // Converts the slice into a `Vec<u8>` and creates a `BufferReader`
    }
}
