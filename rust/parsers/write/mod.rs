/// Buffer Writer for writing data from a vector
mod buffer;
/// File Writer for writing data from a file
#[cfg(feature = "std")]
mod file;

use alloc::vec::Vec;
pub use buffer::*;
#[cfg(feature = "std")]
pub use file::*;

/// The defacto interface for all writers.
pub trait Writer {
    /// Get the current offset of the writer
    fn offset(&mut self) -> u64;
    /// Write data at the specified offset to the writer
    fn write(&mut self, data: &[u8], offset: u64);
    /// Append data to the writer
    fn append(&mut self, data: &[u8]);
    /// Append string to the writer
    fn append_string(&mut self, string: &str);
    /// Take the data for oneself
    fn take(&mut self) -> Vec<u8>;
    /// Get a slice of written data
    fn slice(&mut self, start: u64, end: u64) -> Vec<u8>;
    /// Flush the writer (if applicable)
    fn flush(&mut self);
}
