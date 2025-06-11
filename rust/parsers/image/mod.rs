/// Image decoding
pub mod decoder;
/// RGBA data
pub mod rgba;
/// Image utility functions
pub mod util;

use super::Buffer;
pub use decoder::*;
pub use rgba::*;
pub use util::*;

/// Image Data structure
pub struct ImageData {
    /// Width of the image
    pub width: usize,
    /// Height of the image
    pub height: usize,
    /// RGBA or raw data
    pub data: Buffer,
}
