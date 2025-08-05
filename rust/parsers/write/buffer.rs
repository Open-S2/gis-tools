use super::Writer;
use alloc::{borrow::ToOwned, vec::Vec};

/// Buffer writer is used on smaller datasets that are easy to write in memory. Faster then the Filesystem
#[derive(Debug, Default)]
pub struct BufferWriter {
    buffer: Vec<u8>,
}
impl BufferWriter {
    /// Create a new BufferWriter
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    /// Resize the buffer if needed
    pub fn resize(&mut self, size: usize) {
        if size <= self.buffer.len() {
            return;
        }
        self.buffer.resize(size, 0);
    }
}
impl Writer for BufferWriter {
    fn offset(&mut self) -> u64 {
        self.buffer.len() as u64
    }
    fn write(&mut self, data: &[u8], offset: u64) {
        self.resize(offset as usize + data.len());
        let offset = offset as usize;
        self.buffer[offset..offset + data.len()].copy_from_slice(data);
    }
    fn append(&mut self, data: &[u8]) {
        self.buffer.extend(data);
    }
    fn append_string(&mut self, string: &str) {
        self.buffer.extend(string.as_bytes());
    }
    fn take(&mut self) -> Vec<u8> {
        self.buffer.to_owned()
    }
    fn slice(&mut self, start: u64, end: u64) -> Vec<u8> {
        let start = start as usize;
        let end = end as usize;
        self.buffer[start..end].to_vec()
    }
    fn flush(&mut self) {}
}
