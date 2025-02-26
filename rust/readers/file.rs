use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::readers::Reader;

use alloc::{
    str::from_utf8,
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// A file reader for reading data from a file
pub struct FileReader {
    file: File,
    size: usize,
    cursor: usize,
}

impl FileReader {
    /// Creates a new file reader from a file path
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let file = File::open(path)?;
        let size = file.metadata().map(|metadata| metadata.len() as usize).unwrap_or(0);
        Ok(Self { file, size, cursor: 0 })
    }
}
impl FileReader {
    fn seek_to(&mut self, offset: usize) {
        if self.cursor != offset {
            self.file.seek(SeekFrom::Start(offset as u64)).expect("Failed to seek");
            self.cursor = offset;
        }
    }

    fn get_bytes(&mut self, byte_offset: Option<usize>, byte_length: usize) -> Vec<u8> {
        let offset = byte_offset.unwrap_or(self.cursor);
        assert!(offset + byte_length <= self.size);
        self.seek_to(offset);

        let mut buffer = vec![0u8; byte_length];
        self.file.read_exact(&mut buffer).expect("Failed to read bytes");
        self.cursor = offset + byte_length;

        buffer
    }
}

impl Reader for FileReader {
    fn len(&self) -> usize {
        self.size
    }

    // GETTERS

    fn uint64_be(&mut self, byte_offset: Option<usize>) -> u64 {
        let bytes = self.get_bytes(byte_offset, 8);
        u64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn uint64_le(&mut self, byte_offset: Option<usize>) -> u64 {
        let bytes = self.get_bytes(byte_offset, 8);
        u64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }

    fn int64_be(&mut self, byte_offset: Option<usize>) -> i64 {
        let bytes = self.get_bytes(byte_offset, 8);
        i64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn int64_le(&mut self, byte_offset: Option<usize>) -> i64 {
        let bytes = self.get_bytes(byte_offset, 8);
        i64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_be(&mut self, byte_offset: Option<usize>) -> f64 {
        let bytes = self.get_bytes(byte_offset, 8);
        f64::from_be_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn f64_le(&mut self, byte_offset: Option<usize>) -> f64 {
        let bytes = self.get_bytes(byte_offset, 8);
        f64::from_le_bytes(bytes.try_into().expect("Failed to read 8 bytes"))
    }
    fn uint32_be(&mut self, byte_offset: Option<usize>) -> u32 {
        let bytes = self.get_bytes(byte_offset, 4);
        u32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint32_le(&mut self, byte_offset: Option<usize>) -> u32 {
        let bytes = self.get_bytes(byte_offset, 4);
        u32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_be(&mut self, byte_offset: Option<usize>) -> i32 {
        let bytes = self.get_bytes(byte_offset, 4);
        i32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn int32_le(&mut self, byte_offset: Option<usize>) -> i32 {
        let bytes = self.get_bytes(byte_offset, 4);
        i32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_be(&mut self, byte_offset: Option<usize>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 4);
        f32::from_be_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn f32_le(&mut self, byte_offset: Option<usize>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 4);
        f32::from_le_bytes(bytes.try_into().expect("Failed to read 4 bytes"))
    }
    fn uint16_be(&mut self, byte_offset: Option<usize>) -> u16 {
        let bytes = self.get_bytes(byte_offset, 2);
        u16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn uint16_le(&mut self, byte_offset: Option<usize>) -> u16 {
        let bytes = self.get_bytes(byte_offset, 2);
        u16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_be(&mut self, byte_offset: Option<usize>) -> i16 {
        let bytes = self.get_bytes(byte_offset, 2);
        i16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn int16_le(&mut self, byte_offset: Option<usize>) -> i16 {
        let bytes = self.get_bytes(byte_offset, 2);
        i16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"))
    }
    fn f16_be(&mut self, byte_offset: Option<usize>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 2);
        let f = f16::from_be_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn f16_le(&mut self, byte_offset: Option<usize>) -> f32 {
        let bytes = self.get_bytes(byte_offset, 2);
        let f = f16::from_le_bytes(bytes.try_into().expect("Failed to read 2 bytes"));
        f32::from_bits(f.to_bits().into())
    }
    fn uint8(&mut self, byte_offset: Option<usize>) -> u8 {
        let bytes = self.get_bytes(byte_offset, 1);
        bytes[0]
    }
    fn int8(&mut self, byte_offset: Option<usize>) -> i8 {
        let bytes = self.get_bytes(byte_offset, 1);
        bytes[0] as i8
    }

    // Methods

    fn tell(&mut self) -> usize {
        self.cursor
    }

    fn seek(&mut self, pos: usize) {
        self.seek_to(pos);
    }

    fn slice(&mut self, begin: Option<usize>, end: Option<usize>) -> Vec<u8> {
        let start = begin.unwrap_or(self.cursor);
        let end = end.unwrap_or(self.cursor);
        self.get_bytes(Some(start), end - start)
    }

    fn seek_slice(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; size];
        self.file.read_exact(&mut buffer).expect("Failed to read slice");
        self.cursor += size;
        buffer
    }

    fn parse_string(&mut self, byte_offset: Option<usize>, byte_length: Option<usize>) -> String {
        let offset = byte_offset.unwrap_or(self.cursor);
        let length = byte_length.unwrap_or(self.size - offset);
        let bytes = self.get_bytes(Some(offset), length);
        let string: &str = from_utf8(&bytes).unwrap();
        string.to_string()
    }
}
impl From<&str> for FileReader {
    fn from(path: &str) -> Self {
        FileReader::new(PathBuf::from(path)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/codepage.cpg");

        let mut reader = FileReader::new(path).unwrap();
        let string = reader.parse_string(None, None);
        assert_eq!(string, "ANSI 1250\n");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_default_functions() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/fixtures/dv.bin");
        let path_str = path.to_str().unwrap();

        let mut reader = FileReader::from(path_str);

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
