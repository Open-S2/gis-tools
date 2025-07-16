#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{JPEGOptions, JpegStreamReader, decode_jpeg_data, jpeg_decoder};
    use std::path::PathBuf;

    #[test]
    fn jpeg_clear() {
        let mut test = JpegStreamReader::new(None);
        test.reset_frames();
        // assert!(test.frames.is_empty());
    }

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

    #[test]
    fn decode_a_jpg_reads_image_with_a_bad_e1_marker() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/fillbytes.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        assert_eq!(raw_image_data.width, 704);
        assert_eq!(raw_image_data.height, 576);
    }

    #[test]
    fn decode_a_jpeg_rst_intervals() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/redbox-with-rst.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/redbox.jpg");
        let expected = std::fs::read(path).unwrap();
        let expected_data = decode_jpeg_data(&expected, None, None);
        // compares
        assert_eq!(raw_image_data.data, expected_data.data);
    }

    #[test]
    fn decode_a_jpeg_with_trailing_bytes() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/redbox-with-trailing-bytes.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/redbox.jpg");
        let expected = std::fs::read(path).unwrap();
        let expected_data = decode_jpeg_data(&expected, None, None);
        // compares
        assert_eq!(raw_image_data.data, expected_data.data);
    }

    #[test]
    fn decode_a_jpeg_bad_e1_marker_not_preceeded_by_ff() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/table-with-bad-e1.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/table-with-good-e1.jpg");
        let expected = std::fs::read(path).unwrap();
        let expected_data = decode_jpeg_data(&expected, None, None);
        // compares
        assert_eq!(raw_image_data.data, expected_data.data);
    }

    #[test]
    fn decode_a_jpeg_bad_e1_marker_not_preceeded_by_ff_2() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/table-with-bad-e1.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = jpeg_decoder(&jpeg_data, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/table-with-good-e1.jpg");
        let expected = std::fs::read(path).unwrap();
        let expected_data = jpeg_decoder(&expected, None);
        // compares
        assert_eq!(raw_image_data, expected_data);
    }

    #[test]
    fn decode_a_jpeg_grayscale() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/apsara.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/apsara.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 580);
        assert_eq!(raw_image_data.height, 599);
        assert_eq!(raw_image_data.data, expected);
        assert_eq!(
            raw_image_data.comments,
            vec!["File source: http://commons.wikimedia.org/wiki/File:Apsara-mit-Sitar.jpg"]
        );
    }

    #[test]
    fn decode_a_jpeg_rgb_image_32_bit_truecolor() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/truecolor.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(
            &jpeg_data,
            Some(JPEGOptions { color_transform: Some(false), ..Default::default() }),
            None,
        );
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/truecolor.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 1280);
        assert_eq!(raw_image_data.height, 2000);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_cmyk_correct_colors() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-cmyk.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-cmyk.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 400);
        assert_eq!(raw_image_data.height, 250);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_cmyk_correct_colors_without_transform() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-cmyk-notransform.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-cmyk-notransform.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 400);
        assert_eq!(raw_image_data.height, 250);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_rgb_with_correct_colors() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-rgb.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/tree-rgb.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 400);
        assert_eq!(raw_image_data.height, 250);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_rgb_progressive_with_correct_colors() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/rgb.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/rgb.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 350);
        assert_eq!(raw_image_data.height, 262);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_rgb_cmyk_grey_with_correct_colors() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/cmyk-grey.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/cmyk-grey.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 300);
        assert_eq!(raw_image_data.height, 389);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_rgb_cmyk_with_correct_colors_adobe() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/cmyktest.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/cmyktest.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 300);
        assert_eq!(raw_image_data.height, 111);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_rgb_cmyk_with_correct_colors_adobe_2() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/plusshelf-drawing.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/plusshelf-drawing.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 350);
        assert_eq!(raw_image_data.height, 233);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_unconventional_table() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/unconventional-table.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // compares
        assert_eq!(raw_image_data.width, 1920);
        assert_eq!(raw_image_data.height, 1200);
    }

    #[test]
    fn decode_a_jpeg_progressive() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/skater-progressive.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/skater-progressive.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 256);
        assert_eq!(raw_image_data.height, 256);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_non_progressive() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/skater.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        assert_eq!(raw_image_data.width, 256);
        assert_eq!(raw_image_data.height, 256);
    }

    #[test]
    fn decode_a_jpeg_basic_with_options() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(
            &jpeg_data,
            Some(JPEGOptions { color_transform: Some(false), ..Default::default() }),
            None,
        );
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat-nocolortrans.rgba");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 320);
        assert_eq!(raw_image_data.height, 180);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_basic_with_options_2() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(
            &jpeg_data,
            Some(JPEGOptions { format_as_rgba: false, ..Default::default() }),
            None,
        );
        // grab the raw result data
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/grumpycat.rgb");
        let expected = std::fs::read(path).unwrap();
        // compares
        assert_eq!(raw_image_data.width, 320);
        assert_eq!(raw_image_data.height, 180);
        assert_eq!(raw_image_data.data, expected);
    }

    #[test]
    fn decode_a_jpeg_ffdc_marker() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/marker-ffdc.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        assert_eq!(raw_image_data.width, 200);
        assert_eq!(raw_image_data.height, 200);
    }

    #[test]
    fn decode_a_jpeg_within_memory_limits() {
        // grab the jpeg
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/image/jpeg/fixtures/black-6000x6000.jpg");
        let jpeg_data = std::fs::read(path).unwrap();
        // decode
        let raw_image_data = decode_jpeg_data(&jpeg_data, None, None);
        assert_eq!(raw_image_data.width, 6000);
        assert_eq!(raw_image_data.height, 6000);
    }
}

// // See https://github.com/eugeneware/jpeg-js/issues/53
// test('limits resolution exposure', () => {
//   expect(() => decodeJpegData(SUPER_LARGE_RESOLUTION_JPEG_BUFFER.buffer)).toThrow(
//     'maxResolutionInMP limit exceeded by 3405MP',
//   );
// });

// test('limits memory exposure', async () => {
//   expect(() => decodeJpegData(SUPER_LARGE_JPEG_BUFFER.buffer, { maxResolutionInMP: 500 })).toThrow(
//     /maxMemoryUsageInMB limit exceeded by at least \d+MB/,
//   );

//   // Make sure the limit resets each decode.
//   const jpegData = await fixture('grumpycat.jpg');
//   expect(() => decodeJpegData(jpegData)).not.toThrow();
// }, 30000);

// // See https://github.com/jpeg-js/jpeg-js/issues/105
// test('errors out invalid sampling factors', () => {
//   expect(() => decodeJpegData(Buffer.from('/9j/wfFR2AD/UdgA/9r/3g==', 'base64').buffer)).toThrow(
//     'marker was not found',
//   );
// });
