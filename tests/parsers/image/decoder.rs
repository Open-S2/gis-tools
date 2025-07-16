#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{Buffer, DecodeOptions, image_decoder, image_decoder_buffer};
    use std::path::PathBuf;

    #[test]
    fn test_image_decoder() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        let buf: Buffer = jpeg_data.into();

        let mut image_decoder_buffer = image_decoder_buffer(&buf, None);
        let image_decoder_bytes = image_decoder_buffer.take();

        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.rgba");
        let expected = std::fs::read(path).unwrap();

        assert!(image_decoder_bytes.len() == expected.len());
    }

    #[test]
    fn test_image_decoder_direct() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        let buf: Buffer = jpeg_data.into();

        let image_decoder_buffer = image_decoder(&buf, None).unwrap();

        assert_eq!(image_decoder_buffer.height, 180);
        assert_eq!(image_decoder_buffer.width, 320);
        assert_eq!(image_decoder_buffer.data.len(), 320 * 180 * 4);
    }

    #[test]
    fn test_image_decoder_direct_with_options() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        let buf: Buffer = jpeg_data.into();

        let image_decoder_buffer = image_decoder(
            &buf,
            Some(DecodeOptions {
                x: Some(20),
                y: Some(20),
                width: Some(100),
                height: Some(100),
                ..Default::default()
            }),
        )
        .unwrap();

        assert_eq!(image_decoder_buffer.height, 100);
        assert_eq!(image_decoder_buffer.width, 100);
        assert_eq!(image_decoder_buffer.data.len(), 100 * 100 * 4);
    }
}
