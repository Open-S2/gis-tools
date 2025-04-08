/// Buffer Writer for writing data from a vector
pub mod buffer;
/// File Writer for writing data from a file
#[cfg(feature = "std")]
pub mod file;
/// The (S2)PMTiles Writer
pub mod pmtiles;
/// Tile based writers
pub mod tile;

use alloc::vec::Vec;
pub use buffer::*;
#[cfg(feature = "std")]
pub use file::*;
pub use tile::*;

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
    /// Flush the writer (if applicable)
    fn flush(&mut self);
}
