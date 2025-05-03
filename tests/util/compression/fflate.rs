#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::util::{FFlateError, decompress_fflate};
    use std::{fs, path::PathBuf};

    #[test]
    fn deflate_sync_dictionary() {
        // get dictionary
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/spdyDict.txt");
        let dictionary: Vec<u8> = fs::read(&path).expect("Failed to read file dict");
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateSync_dictionary_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, Some(&dictionary)).unwrap();

        assert_eq!(decompressed.len(), expected.len());

        assert_eq!(
            decompressed[decompressed.len() - 20..decompressed.len()],
            expected[expected.len() - 20..expected.len()]
        );
    }

    #[test]
    fn deflate_raw_sync_level_0() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateRawSync_level_0_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn deflate_sync_level_9() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateSync_level_9_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn deflate_sync_mem_level_9() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateSync_memLevel_9_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn deflate_sync_strategy_0() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateSync_strategy_0_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn deflate_raw_sync_window_bits_15() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
        // get compressed
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/deflateRawSync_windowBits_15_compressed.bin");
        let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

        let decompressed = decompress_fflate(&compressed, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn dictionary() {
        let dict: Vec<u8> = [97, 98, 99, 100].to_vec();
        let compressed: Vec<u8> = [
            120, 187, 3, 216, 1, 139, 203, 72, 205, 201, 201, 7, 19, 10, 229, 249, 69, 57, 41, 0,
            55, 19, 6, 113,
        ]
        .to_vec();
        let expected: Vec<u8> =
            [104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
                .to_vec();

        let decompressed = decompress_fflate(&compressed, Some(&dict)).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_gzip() {
        let compressed_gzip: Vec<u8> = [
            31, 139, 8, 8, 230, 176, 184, 103, 0, 3, 101, 120, 112, 101, 99, 116, 101, 100, 46,
            116, 120, 116, 0, 11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252,
            52, 133, 204, 188, 130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110,
            126, 81, 42, 88, 72, 143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 162,
            255, 102, 10, 59, 0, 0, 0,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_fflate(&compressed_gzip, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_deflate() {
        let compressed_gzip: Vec<u8> = [
            120, 156, 11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252, 52, 133,
            204, 188, 130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110, 126, 81,
            42, 88, 72, 143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 80, 157, 18,
            21,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_fflate(&compressed_gzip, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_deflate_raw() {
        let compressed_gzip: Vec<u8> = [
            11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252, 52, 133, 204, 188,
            130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110, 126, 81, 42, 88, 72,
            143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 80, 157, 18,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_fflate(&compressed_gzip, None).unwrap();

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_deflate_raw_intentionally_fail() {
        let compressed_gzip: Vec<u8> = [
            133, 146, 212, 226, 18, 133, 252, 52, 133, 204, 188, 130, 210, 18, 133, 148, 196, 146,
            68, 46, 174, 16, 168, 108, 110, 126, 81, 42, 88, 72, 143, 139, 203, 208, 200, 216, 196,
            216, 204, 212, 156, 11, 0, 80, 157, 18,
        ]
        .to_vec();

        let decompress_error = decompress_fflate(&compressed_gzip, None);
        assert_eq!(decompress_error.unwrap_err(), FFlateError::InvalidLengthLiteral);
    }
}
