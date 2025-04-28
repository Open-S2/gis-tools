use super::Writer;
use alloc::vec::Vec;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

/// A writer that operates on the filesystem
#[derive(Debug)]
pub struct FileWriter {
    file: File,
}

impl FileWriter {
    /// Create a new FileWriter, truncating the file if it exists
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file =
            OpenOptions::new().read(true).write(true).create(true).truncate(true).open(path)?;
        Ok(Self { file })
    }
}

impl Writer for FileWriter {
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

    fn flush(&mut self) {
        self.file.flush().expect("Flush failed");
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    use tempfile::NamedTempFile;

    use alloc::vec;

    #[test]
    fn test_new() {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let mut writer = FileWriter::new(&file_path).unwrap();
        writer.append(&[0, 1, 2, 3, 4]);
        writer.append_string("TEST!");
        writer.write(&[10, 9], 1);

        let data = writer.take();

        assert_eq!(data, vec![0, 10, 9, 3, 4, 84, 69, 83, 84, 33]);
    }
}
