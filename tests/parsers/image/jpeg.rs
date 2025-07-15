#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::decode_jpeg_data;
    use std::path::PathBuf;

    #[test]
    fn decode_a_jpeg() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 320);
        assert_eq!(raw_image_data.height, 180);
        assert_eq!(raw_image_data.data, expected);
    }
}
