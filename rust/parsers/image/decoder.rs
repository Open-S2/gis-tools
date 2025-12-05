#[cfg(feature = "std")]
use super::ImageData;
#[cfg(feature = "std")]
use crate::parsers::Buffer;
#[cfg(feature = "std")]
use image::ImageReader;
#[cfg(feature = "std")]
use std::io::Cursor;

/// Image Decoder Options
#[derive(Debug, Default, Clone)]
pub struct DecodeOptions {
    /// The x position to start at
    pub x: Option<u32>,
    /// The y position to start at
    pub y: Option<u32>,
    /// The width to read in
    pub width: Option<u32>,
    /// The height to read in
    pub height: Option<u32>,
    /// The modulo to use. For example you may have a 514x514 image, but you want to use 512x512 [Default=1]
    pub modulo: Option<u32>,
}

/// Decode any image
///
/// ## Parameters
/// - `buffer`: the input buffer
/// - `options`: the decode options
///
/// ## Returns
/// the raw decoded buffer
#[cfg(feature = "std")]
pub fn image_decoder(
    buffer: &Buffer,
    options: Option<DecodeOptions>,
) -> Result<ImageData, image::ImageError> {
    use image::GenericImageView;

    let img = ImageReader::new(Cursor::new(buffer.buf())).with_guessed_format()?.decode()?;

    let (img_width, img_height) = img.dimensions();
    let opts = options.unwrap_or_default();

    let modulo = opts.modulo.unwrap_or(1);
    let diff = img_width % modulo;

    let (x, y, width, height) = if diff != 0 {
        let shift = diff / 2;
        (shift, shift, img_width - diff, img_width - diff)
    } else {
        (
            opts.x.unwrap_or(0),
            opts.y.unwrap_or(0),
            opts.width.unwrap_or(img_width),
            opts.height.unwrap_or(img_height),
        )
    };

    let sub_image = img.view(x, y, width, height).to_image();

    Ok(ImageData {
        width: width as usize,
        height: height as usize,
        data: sub_image.into_raw().into(),
    })
}

/// Image decoder
///
/// ## Parameters
/// - `buffer`: the input buffer
/// - `options`: user defined options
///
/// ## Returns
/// the decoded buffer
#[cfg(feature = "std")]
pub fn image_decoder_buffer(buffer: &Buffer, options: Option<DecodeOptions>) -> Buffer {
    image_decoder(buffer, options).unwrap().data
}
