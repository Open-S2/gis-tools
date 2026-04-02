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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImageData {
    /// Width of the image
    pub width: usize,
    /// Height of the image
    pub height: usize,
    /// RGBA or raw data
    pub data: Buffer,
}
#[cfg(feature = "std")]
impl From<&image::RgbaImage> for ImageData {
    fn from(img: &image::RgbaImage) -> Self {
        let raw_data = img.as_raw();
        ImageData {
            width: img.width() as usize,
            height: img.height() as usize,
            data: raw_data.clone().into(),
        }
    }
}
#[cfg(feature = "std")]
impl From<&ImageData> for image::RgbaImage {
    fn from(img: &ImageData) -> Self {
        // We always store ImageData as an RGBA
        image::RgbaImage::from_raw(img.width as u32, img.height as u32, img.data.buf().into())
            .unwrap()
    }
}
