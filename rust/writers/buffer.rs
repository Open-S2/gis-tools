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
}
impl Writer for BufferWriter {
    fn offset(&mut self) -> u64 {
        self.buffer.len() as u64
    }
    fn write(&mut self, data: &[u8], offset: u64) {
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

    fn flush(&mut self) {}
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    use alloc::vec;

    #[test]
    fn test_new() {
        let mut writer = BufferWriter::new(vec![]);
        writer.append(&[0, 1, 2, 3, 4]);
        writer.append_string("TEST!");
        writer.write(&[10, 9], 1);

        let data = writer.take();

        assert_eq!(data, vec![0, 10, 9, 3, 4, 84, 69, 83, 84, 33]);
    }
}
