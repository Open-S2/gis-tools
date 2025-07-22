/// Image decoding
pub mod decoder;
/// JPEG data
pub mod jpeg;
/// RGBA data
pub mod rgba;
// /// Image utility functions
// pub mod util;
// /// JPEG 2000 data
// pub mod jpeg2000;

use super::Buffer;
pub use decoder::*;
pub use jpeg::*;
pub use rgba::*;
// pub use util::*;

/// Image Data structure
pub struct ImageData {
    /// Width of the image
    pub width: usize,
    /// Height of the image
    pub height: usize,
    /// RGBA or raw data
    pub data: Buffer,
}
