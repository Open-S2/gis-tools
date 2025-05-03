use super::StdReader;
use crate::parsers::Reader;
use core::cell::RefCell;
use half::f16;
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// A file reader for reading data from a file
#[derive(Debug)]
pub struct FileReader {
    file: RefCell<File>,
    size: u64,
    cursor: RefCell<u64>,
}

impl FileReader {
    /// Creates a new file reader from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let size = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        Ok(Self { file: file.into(), size, cursor: 0.into() })
    }

    fn seek_to(&self, offset: u64) {
        if *self.cursor.borrow() != offset {
            self.file.borrow_mut().seek(SeekFrom::Start(offset)).expect("Failed to seek");
            *self.cursor.borrow_mut() = offset;
        }
    }

    fn get_bytes(&self, byte_offset: Option<u64>, byte_length: u64) -> Vec<u8> {
        let offset = byte_offset.unwrap_or(*self.cursor.borrow());
        assert!(offset + byte_length <= self.size);
        self.seek_to(offset);

        let mut buffer = vec![0u8; byte_length as usize];
        self.file.borrow_mut().read_exact(&mut buffer).expect("Failed to read bytes");
        *self.cursor.borrow_mut() = offset + byte_length;

        buffer
    }
}
impl StdReader for FileReader {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let size = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        Ok(Self { file: file.into(), size, cursor: 0.into() })
    }
}
impl Reader for FileReader {
    fn len(&self) -> u64 {
        self.size
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
        self.seek_to(pos);
    }

    fn slice(&self, begin: Option<u64>, end: Option<u64>) -> Vec<u8> {
        let cursor = *self.cursor.borrow();
        let start = begin.unwrap_or(cursor);
        let end = end.unwrap_or(cursor);
        self.get_bytes(Some(start), end - start)
    }

    fn seek_slice(&self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; size];
        self.file.borrow_mut().read_exact(&mut buffer).expect("Failed to read slice");
        *self.cursor.borrow_mut() += size as u64;
        buffer
    }

    fn parse_string(&self, byte_offset: Option<u64>, byte_length: Option<u64>) -> String {
        let offset = byte_offset.unwrap_or(*self.cursor.borrow());
        let length = byte_length.unwrap_or(self.size - offset);
        let bytes = self.get_bytes(Some(offset), length);
        // Remove null bytes from the byte slice before decoding it
        let cleaned_str_buf: Vec<u8> = bytes.iter().cloned().filter(|&b| b != 0).collect();
        let string = String::from_utf8_lossy(&cleaned_str_buf).to_string();
        *self.cursor.borrow_mut() = offset + length;
        string
    }
}
impl From<PathBuf> for FileReader {
    fn from(path: PathBuf) -> Self {
        FileReader::new(path).unwrap()
    }
}
impl From<String> for FileReader {
    fn from(path: String) -> Self {
        FileReader::new(path).unwrap()
    }
}
impl From<&str> for FileReader {
    fn from(path: &str) -> Self {
        FileReader::new(path).unwrap()
    }
}
