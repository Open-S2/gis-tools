use super::Writer;
use alloc::{vec, vec::Vec};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

/// A writer that operates on the filesystem
#[derive(Debug)]
pub struct FileWriter {
    path: PathBuf,
    file: File,
}
impl FileWriter {
    /// Create a new FileWriter, truncating the file if it exists
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file =
            OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path)?;
        Ok(Self { path: path_buf, file })
    }

    /// Get the length of the file
    pub fn len(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    /// Check if the file is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl Clone for FileWriter {
    fn clone(&self) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Failed to reopen file");
        Self { path: self.path.clone(), file }
    }
}
impl Writer for FileWriter {
    fn tell(&mut self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    fn offset(&mut self) -> u64 {
        self.file.stream_position().expect("Seek failed")
    }

    fn write(&mut self, data: &[u8], offset: u64) {
        self.file.seek(SeekFrom::Start(offset)).expect("Seek failed");
        self.file.write_all(data).expect("Write failed");
    }

    fn append(&mut self, data: &[u8]) {
        self.file.seek(SeekFrom::End(0)).expect("Seek failed");
        self.file.write_all(data).expect("Write failed");
    }

    fn append_string(&mut self, string: &str) {
        self.append(string.as_bytes());
    }

    fn take(&mut self) -> Vec<u8> {
        self.file.seek(SeekFrom::Start(0)).expect("Seek failed");
        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer).expect("Read failed");
        buffer
    }

    fn slice(&mut self, start: u64, end: u64) -> Vec<u8> {
        let len = end - start;
        let mut buffer = vec![0u8; len as usize];
        self.file.seek(SeekFrom::Start(start)).expect("Seek failed");
        self.file.read_exact(&mut buffer).expect("Read failed");
        buffer
    }

    fn flush(&mut self) {
        self.file.flush().expect("Flush failed");
    }
}
