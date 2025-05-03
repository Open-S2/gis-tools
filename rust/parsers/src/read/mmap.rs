use super::StdReader;
use crate::Reader;
use core::cell::RefCell;
use half::f16;
use memmap2::Mmap;
use std::{
    fs::File,
    io::{self},
    path::{Path, PathBuf},
    string::{String, ToString},
    vec::Vec,
};

/// A file reader for reading data from a file
#[derive(Debug)]
pub struct MMapReader {
    _file: File,
    mmap: Mmap,
    size: u64,
    cursor: RefCell<u64>,
}
impl MMapReader {
    fn get_bytes(&self, byte_offset: Option<u64>, byte_length: u64) -> &[u8] {
        let mut cursor = *self.cursor.borrow();
        let offset = byte_offset.unwrap_or(cursor);
        assert!(offset + byte_length <= self.size);
        cursor = offset;

        let buffer = &self.mmap[cursor as usize..(cursor + byte_length) as usize];
        *self.cursor.borrow_mut() = offset + byte_length;

        buffer
    }
}
impl StdReader for MMapReader {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let _file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&_file)? };
        let size = _file.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        Ok(Self { _file, mmap, size, cursor: 0.into() })
    }
}
impl Reader for MMapReader {
    fn len(&self) -> u64 {
        self.mmap.len() as u64
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
        let end = end.unwrap_or(self.mmap.len() as u64) as usize;
        assert!(end <= self.mmap.len());
        self.mmap[begin..end].to_vec()
    }

    fn seek_slice(&self, size: usize) -> Vec<u8> {
        assert!(*self.cursor.borrow() + size as u64 <= self.mmap.len() as u64);
        *self.cursor.borrow_mut() += size as u64;
        let cursor = *self.cursor.borrow();
        self.slice(Some(cursor - size as u64), Some(cursor))
    }
    fn parse_string(&self, byte_offset: Option<u64>, byte_length: Option<u64>) -> String {
        let offset = byte_offset.unwrap_or(*self.cursor.borrow()) as usize;
        let length = byte_length.unwrap_or(self.mmap.len() as u64 - offset as u64) as usize;
        let str_buf = &self.mmap[offset..offset + length];
        let cleaned_str_buf: Vec<u8> = str_buf.iter().cloned().filter(|&b| b != 0).collect();
        let string = String::from_utf8_lossy(&cleaned_str_buf).to_string();
        *self.cursor.borrow_mut() = (offset + length) as u64;
        string
    }
}
impl From<PathBuf> for MMapReader {
    fn from(path: PathBuf) -> Self {
        MMapReader::new(path).unwrap()
    }
}
impl From<String> for MMapReader {
    fn from(path: String) -> Self {
        MMapReader::new(path).unwrap()
    }
}
impl From<&str> for MMapReader {
    fn from(path: &str) -> Self {
        MMapReader::new(path).unwrap()
    }
}
