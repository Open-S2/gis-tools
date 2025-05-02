use crate::Reader;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;
use pbf::bit_cast::BitCast;

const MAX_VARINT_LENGTH: usize = u64::BITS as usize * 8 / 7 + 1;
const BIT_SHIFT: [u64; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];

/// The `Buffer` struct is used to read and write Buffer messages.
///
/// ## Description
/// This works as a wrapper around a byte buffer. The idea is to have an opinionated reading/writing
/// API for little-endian encodings for various signed and unsigned integer types. It also includes
/// basic varint encoding and decoding.
///
/// ## Usage
///
/// Create a new Buffer instance:
/// ```rs
/// use gistools::util::Buffer;
///
/// let mut buf = Buffer::new();
/// ```
/// Create a Buffer instance from a byte buffer:
/// ```rs
/// use gistools::util::Buffer;
///
/// let mut vec = vec![0x0A, 0x03, 0x74, 0x65, 0x73, 0x74];
/// let mut buf = Buffer::new(vec);
/// // DO STUFF
/// let res = buf.take();
/// ```
#[derive(Debug, Default)]
pub struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}
impl From<Vec<u8>> for Buffer {
    fn from(buf: Vec<u8>) -> Buffer {
        Buffer { buf, pos: 0 }
    }
}
impl Buffer {
    /// Create a new Buffer instance.
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer { buf, pos: 0 }
    }

    /// Set the position to read from the buffer next.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// get the length of the bufer
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// return the current u8 under the buffer
    pub fn get_u8(&mut self) -> u8 {
        let value = self.get_u8_at(self.pos);
        self.pos += 1;

        value
    }

    /// return the current u8 at position
    pub fn get_u8_at(&mut self, pos: usize) -> u8 {
        self.buf[pos]
    }

    /// set the current u8 under the buffer
    pub fn set_u8(&mut self, value: u8) {
        self.set_u8_at(self.pos, value);
        self.pos += 1;
    }

    /// set the current u8 at position
    pub fn set_u8_at(&mut self, pos: usize, value: u8) {
        if pos >= self.buf.len() {
            self.buf.resize(pos + 1, 0);
        }
        self.buf[pos] = value;
    }

    /// return the current i32 under the buffer
    pub fn get_i32(&mut self) -> i32 {
        let value = self.get_i32_at(self.pos);
        // Update the position
        self.pos += 4;

        value
    }

    /// return the current i32 at position
    pub fn get_i32_at(&mut self, pos: usize) -> i32 {
        // Borrow the buffer and slice the next 4 bytes
        let bytes = &self.buf[pos..pos + 4];

        i32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current i32 under the buffer
    pub fn set_i32(&mut self, value: i32) {
        self.set_i32_at(self.pos, value);
        self.pos += 4;
    }

    /// set the current i32 at position
    pub fn set_i32_at(&mut self, pos: usize, value: i32) {
        // Borrow the buffer and slice the next 4 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 4, 0);
        }
        let bytes = &mut self.buf[pos..pos + 4];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u16 under the buffer
    pub fn get_u16(&mut self) -> u16 {
        let value = self.get_u16_at(self.pos);
        // Update the position
        self.pos += 2;

        value
    }

    /// return the current u16 at position
    pub fn get_u16_at(&mut self, pos: usize) -> u16 {
        // Borrow the buffer and slice the next 2 bytes
        let bytes = &self.buf[pos..pos + 2];

        u16::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u16 under the buffer
    pub fn set_u16(&mut self, value: u16) {
        self.set_u16_at(self.pos, value);
        self.pos += 2;
    }

    /// set the current u16 at position
    pub fn set_u16_at(&mut self, pos: usize, value: u16) {
        // Borrow the buffer and slice the next 2 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 2, 0);
        }
        let bytes = &mut self.buf[pos..pos + 2];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u32 under the buffer
    pub fn get_u32(&mut self) -> u32 {
        let value = self.get_u32_at(self.pos);
        // Update the position
        self.pos += 4;

        value
    }

    /// return the current u32 at position
    pub fn get_u32_at(&mut self, pos: usize) -> u32 {
        // Borrow the buffer and slice the next 4 bytes
        let bytes = &self.buf[pos..pos + 4];

        u32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u32 under the buffer
    pub fn set_u32(&mut self, value: u32) {
        self.set_u32_at(self.pos, value);
        self.pos += 4;
    }

    /// set the current u32 at position
    pub fn set_u32_at(&mut self, pos: usize, value: u32) {
        // Borrow the buffer and slice the next 4 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 4, 0);
        }
        let bytes = &mut self.buf[pos..pos + 4];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current i32 under the buffer
    pub fn get_i64(&mut self) -> i64 {
        let value = self.get_i64_at(self.pos);
        // Update the position
        self.pos += 8;

        value
    }

    /// return the current i32 at position
    pub fn get_i64_at(&mut self, pos: usize) -> i64 {
        // Borrow the buffer and slice the next 8 bytes
        let bytes = &self.buf[pos..pos + 8];

        i64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current i32 under the buffer
    pub fn set_i64(&mut self, value: i64) {
        self.set_i64_at(self.pos, value);
        self.pos += 8;
    }

    /// set the current i32 at position
    pub fn set_i64_at(&mut self, pos: usize, value: i64) {
        // Borrow the buffer and slice the next 8 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 8, 0);
        }
        let bytes = &mut self.buf[pos..pos + 8];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u64 under the buffer
    pub fn get_u64(&mut self) -> u64 {
        let value = self.get_u64_at(self.pos);
        // Update the position
        self.pos += 8;

        value
    }

    /// return the current u64 at position
    pub fn get_u64_at(&mut self, pos: usize) -> u64 {
        // Borrow the buffer and slice the next 8 bytes
        let bytes = &self.buf[pos..pos + 8];

        u64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u64 under the buffer
    pub fn set_u64(&mut self, value: u64) {
        self.set_u64_at(self.pos, value);
        self.pos += 8;
    }

    /// set the current u64 at position
    pub fn set_u64_at(&mut self, pos: usize, value: u64) {
        // Borrow the buffer and slice the next 8 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 8, 0);
        }
        let bytes = &mut self.buf[pos..pos + 8];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// Decode a varint from the buffer at the current position.
    pub fn decode_varint(&mut self) -> u64 {
        if self.pos >= self.buf.len() {
            unreachable!();
        }
        let mut val: u64 = 0;

        for (n, shift) in BIT_SHIFT.iter().enumerate().take(MAX_VARINT_LENGTH) {
            let b = self.buf[self.pos] as u64;
            self.pos += 1;
            if n == 0 {
                if b & 0x80 == 0 {
                    return b;
                }
                val = b & 0x7f;
            } else {
                val |= (b & 0x7f) << shift;
            }
            if b < 0x80 {
                break;
            }
        }

        val
    }

    /// Read in a variable size value from the buffer.
    pub fn read_varint<T>(&mut self) -> T
    where
        T: BitCast,
    {
        let val = self.decode_varint();
        T::from_u64(val)
    }

    /// Write a u64 to the buffer.
    pub fn write_varint<T>(&mut self, val: T)
    where
        T: BitCast,
    {
        let mut val = val.to_u64();

        while val >= 0x80 {
            self.buf.push(((val & 0x7f) | 0x80) as u8);
            val >>= 7;
        }
        self.buf.push(val as u8);
    }

    /// When done writing to the buffer, call this function to take ownership
    pub fn take(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.buf)
    }
}

/// A basic buffer reader for reading data from a buffer
#[derive(Default, Debug)]
pub struct BufferReader {
    /// The buffer
    pub buffer: Vec<u8>, // This struct contains some data
    cursor: RefCell<u64>,
}
impl BufferReader {
    /// Creates a new buffer reader
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, cursor: 0.into() }
    }
}
impl BufferReader {
    fn get_bytes(&self, byte_offset: Option<u64>, byte_length: usize) -> &[u8] {
        let offset = byte_offset.unwrap_or(*self.cursor.borrow()) as usize;
        assert!(offset + byte_length <= self.buffer.len());

        let bytes = &self.buffer[offset..offset + byte_length];
        *self.cursor.borrow_mut() = (offset + byte_length) as u64;
        bytes
    }
}
impl Reader for BufferReader {
    fn len(&self) -> u64 {
        self.buffer.len() as u64
    }

    // GETTERS

    fn uint64_be(&self, byte_offset: Option<u64>) -> u64 {
        let bytes = self.get_bytes(byte_offset, 8);
        u64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn uint64_le(&self, byte_offset: Option<u64>) -> u64 {
        let bytes = self.get_bytes(byte_offset, 8);
        u64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn int64_be(&self, byte_offset: Option<u64>) -> i64 {
        let bytes = self.get_bytes(byte_offset, 8);
        i64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn int64_le(&self, byte_offset: Option<u64>) -> i64 {
        let bytes = self.get_bytes(byte_offset, 8);
        i64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_be(&self, byte_offset: Option<u64>) -> f64 {
        let bytes = self.get_bytes(byte_offset, 8);
        f64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_le(&self, byte_offset: Option<u64>) -> f64 {
        let bytes = self.get_bytes(byte_offset, 8);
        f64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn uint32_be(&self, byte_offset: Option<u64>) -> u32 {
        let bytes = self.get_bytes(byte_offset, 4);
        u32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint32_le(&self, byte_offset: Option<u64>) -> u32 {
        let bytes = self.get_bytes(byte_offset, 4);
        u32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_be(&self, byte_offset: Option<u64>) -> i32 {
        let bytes = self.get_bytes(byte_offset, 4);
        i32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_le(&self, byte_offset: Option<u64>) -> i32 {
        let bytes = self.get_bytes(byte_offset, 4);
        i32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_be(&self, byte_offset: Option<u64>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 4);
        f32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_le(&self, byte_offset: Option<u64>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 4);
        f32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint16_be(&self, byte_offset: Option<u64>) -> u16 {
        let bytes = self.get_bytes(byte_offset, 2);
        u16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn uint16_le(&self, byte_offset: Option<u64>) -> u16 {
        let bytes = self.get_bytes(byte_offset, 2);
        u16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_be(&self, byte_offset: Option<u64>) -> i16 {
        let bytes = self.get_bytes(byte_offset, 2);
        i16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_le(&self, byte_offset: Option<u64>) -> i16 {
        let bytes = self.get_bytes(byte_offset, 2);
        i16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn f16_be(&self, byte_offset: Option<u64>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 2);
        let f = f16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn f16_le(&self, byte_offset: Option<u64>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 2);
        let f = f16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn uint8(&self, byte_offset: Option<u64>) -> u8 {
        let bytes = self.get_bytes(byte_offset, 1);
        bytes[0]
    }
    fn int8(&self, byte_offset: Option<u64>) -> i8 {
        let bytes = self.get_bytes(byte_offset, 1);
        bytes[0] as i8
    }

    // Methods

    fn tell(&self) -> u64 {
        *self.cursor.borrow()
    }
    fn seek(&self, pos: u64) {
        *self.cursor.borrow_mut() = pos;
    }
    fn slice(&self, begin: Option<u64>, end: Option<u64>) -> Vec<u8> {
        let begin = begin.unwrap_or(*self.cursor.borrow()) as usize;
        let end = end.unwrap_or(self.buffer.len() as u64) as usize;
        assert!(end <= self.buffer.len());
        self.buffer[begin..end].to_vec()
    }
    fn seek_slice(&self, size: usize) -> Vec<u8> {
        let size = size as u64;
        assert!(*self.cursor.borrow() + size <= self.buffer.len() as u64);
        *self.cursor.borrow_mut() += size;
        let cursor = *self.cursor.borrow();
        self.slice(Some(cursor - size), Some(cursor))
    }
    fn parse_string(&self, byte_offset: Option<u64>, byte_length: Option<u64>) -> String {
        let offset = byte_offset.unwrap_or(*self.cursor.borrow()) as usize;
        let length = byte_length.unwrap_or((self.buffer.len() - offset) as u64) as usize;
        let str_buf = &self.buffer[offset..offset + length];
        // Remove null bytes from the byte slice before decoding it
        let cleaned_str_buf: Vec<u8> = str_buf.iter().cloned().filter(|&b| b != 0).collect();
        let string = String::from_utf8_lossy(&cleaned_str_buf).to_string();
        *self.cursor.borrow_mut() = (offset + length) as u64;
        string
    }
}
impl<const N: usize> From<&[u8; N]> for BufferReader {
    fn from(buffer: &[u8; N]) -> Self {
        BufferReader::new(buffer.into()) // `&[u8; N]` coerces to `&[u8]` automatically here
    }
}
impl From<&[u8]> for BufferReader {
    fn from(buffer: &[u8]) -> Self {
        BufferReader::new(buffer.into()) // Converts the slice into a `Vec<u8>` and creates a `BufferReader`
    }
}
impl From<Vec<u8>> for BufferReader {
    fn from(buffer: Vec<u8>) -> Self {
        BufferReader::new(buffer) // Converts the slice into a `Vec<u8>` and creates a `BufferReader`
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    extern crate std;

    use super::*;
    use alloc::{vec, vec::Vec};
    use std::{fs, path::PathBuf};

    #[test]
    fn test_buffer() {
        // new
        let buf = Buffer::default();
        let vec1: Vec<u8> = vec![];
        assert_eq!(vec1, buf.buf.to_vec());

        // from
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let buf2: Buffer = Buffer::new(vec.clone());
        assert_eq!(vec, buf2.buf.to_vec());
    }

    #[test]
    fn test_set_pos() {
        let mut buf = Buffer::default();
        assert_eq!(0, buf.pos);
        buf.set_pos(1);
        assert_eq!(1, buf.pos);
    }

    // len
    #[test]
    fn test_len() {
        let mut buf = Buffer::default();
        assert_eq!(0, buf.len());
        buf.set_u8(1);
        assert_eq!(1, buf.len());
    }

    // is_empty
    #[test]
    fn test_is_empty() {
        let mut buf = Buffer::default();
        assert!(buf.is_empty());
        buf.set_u8(1);
        assert!(!buf.is_empty());
    }

    // get_u8, get_u8_at & set_u8
    #[test]
    fn test_get_u8() {
        let mut buf = Buffer::default();
        buf.set_u8(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u8());
        assert_eq!(1, buf.get_u8_at(0));
    }

    // get_u16, get_u16_at & set_u16
    #[test]
    fn test_get_u16() {
        let mut buf = Buffer::default();
        buf.set_u16(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u16());
        assert_eq!(1, buf.get_u16_at(0));
    }

    // get_i32, get_i32_at & set_i32
    #[test]
    fn test_get_i32() {
        let mut buf = Buffer::default();
        buf.set_i32(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i32());
        assert_eq!(1, buf.get_i32_at(0));
    }

    // get_u32, get_u32_at & set_u32
    #[test]
    fn test_get_u32() {
        let mut buf = Buffer::default();
        buf.set_u32(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u32());
        assert_eq!(1, buf.get_u32_at(0));
    }

    // get_i64, get_i64_at & set_i64
    #[test]
    fn test_get_i64() {
        let mut buf = Buffer::default();
        buf.set_i64(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i64());
        assert_eq!(1, buf.get_i64_at(0));
    }

    // get_u64, get_u64_at & set_u64
    #[test]
    fn test_get_u64() {
        let mut buf = Buffer::default();
        buf.set_u64(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u64());
        assert_eq!(1, buf.get_u64_at(0));
    }

    // decode_varint, read_varint, & write_varint
    #[test]
    fn test_decode_varint() {
        let mut buf = Buffer::default();
        buf.write_varint(1_u16);
        buf.write_varint(19393930202_u64);
        buf.set_pos(0);
        assert_eq!(1, buf.read_varint::<u16>());
        assert_eq!(19393930202, buf.read_varint::<u64>());
        buf.set_pos(0);
        assert_eq!(1, buf.decode_varint());
        assert_eq!(19393930202, buf.decode_varint());
    }

    // take
    #[test]
    fn test_take() {
        let mut buf = Buffer::default();
        buf.set_u8(1);
        buf.set_u8(2);
        buf.set_u8(3);
        assert_eq!(vec![1, 2, 3], buf.take());
    }

    #[test]
    fn test_buffer_reader() {
        let buffer = b"Hello, world!";
        let reader = BufferReader::from(buffer);
        assert_eq!(reader.parse_string(None, None), "Hello, world!");

        let vec_buff = Vec::<u8>::from(buffer);
        let reader = BufferReader::from(vec_buff);
        assert_eq!(reader.parse_string(None, None), "Hello, world!");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_default_functions() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/readers/fixtures/dv.bin");
        let raw_data: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        let reader = BufferReader::from(&raw_data[..]);

        assert_eq!(reader.tell(), 0);
        assert_eq!(reader.len(), 42);
        assert!(!reader.is_empty());

        let mut offset = 0;

        assert_eq!(reader.uint8(Some(offset)), 255);
        offset += 1;
        assert_eq!(reader.uint16_le(Some(offset)), 65535);
        assert_eq!(reader.uint16_be(Some(offset)), 65535);
        assert_eq!(reader.f16_le(Some(offset)), 9.1834e-41);
        assert_eq!(reader.f16_be(Some(offset)), 9.1834e-41);
        offset += 2;
        assert_eq!(reader.uint32_le(Some(offset)), 4294967295);
        assert_eq!(reader.uint32_be(Some(offset)), 4294967295);
        offset += 4;
        assert_eq!(reader.int8(Some(offset)), -128);
        offset += 1;
        assert_eq!(reader.int16_le(Some(offset)), -32768);
        assert_eq!(reader.int16_be(Some(offset)), 128);
        offset += 2;
        assert_eq!(reader.int32_le(Some(offset)), -2147483648);
        assert_eq!(reader.int32_be(Some(offset)), 128);
        offset += 4;
        assert_eq!(reader.f32_le(Some(offset)), 3.14);
        assert_eq!(reader.f32_be(Some(offset)), -490.56445);
        offset += 4;
        assert_eq!(reader.f64_le(Some(offset)), 3.14159265359);
        assert_eq!(reader.f64_be(Some(offset)), -2.965482352282314e203);
        offset += 8;
        assert_eq!(reader.uint64_le(Some(offset)), 12345678901234567890);
        assert_eq!(reader.uint64_be(Some(offset)), 15134944594269656235);
        offset += 8;
        assert_eq!(reader.int64_le(Some(offset)), -1234567890123456789);
        assert_eq!(reader.int64_be(Some(offset)), -1477718879929115154);

        let slice = reader.slice(Some(4), Some(8));
        assert_eq!(slice, &[255, 255, 255, 128]);
        reader.seek(4);
        assert_eq!(reader.seek_slice(4), &[255, 255, 255, 128]);
        assert_eq!(reader.tell(), 8);
    }
}
