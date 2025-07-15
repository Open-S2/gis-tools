mod fflate;

#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::util::{
        CompressError, CompressionFormat, FFlateError, compress_data, decompress_data,
        iter_zip_folder,
    };
    use std::{fs, path::PathBuf};

    #[test]
    fn encode_decode_none() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(expected.clone(), CompressionFormat::None).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::None).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_gzip() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(expected.clone(), CompressionFormat::Gzip).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Gzip).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_deflate() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(expected.clone(), CompressionFormat::Deflate).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Deflate).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_deflate_raw() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(expected.clone(), CompressionFormat::DeflateRaw).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::DeflateRaw).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_brotli() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(expected.clone(), CompressionFormat::Brotli).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Brotli).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_zstd() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let _encoded = compress_data(expected.clone(), CompressionFormat::Zstd).unwrap_err();
        // assert_eq!(encoded, CompressError::UnimplementedZstd);
        // decode
        let _decoded = decompress_data(&expected, CompressionFormat::Zstd).unwrap_err();

        // assert_eq!(decoded, expected);
    }

    #[test]
    fn test_compression_format() {
        let compression_format = CompressionFormat::from("gzip");
        assert_eq!(compression_format, CompressionFormat::Gzip);

        let compression_format = CompressionFormat::from("deflate");
        assert_eq!(compression_format, CompressionFormat::Deflate);

        let compression_format = CompressionFormat::from("deflate-raw");
        assert_eq!(compression_format, CompressionFormat::DeflateRaw);

        let compression_format = CompressionFormat::from("brotli");
        assert_eq!(compression_format, CompressionFormat::Brotli);

        let compression_format = CompressionFormat::from("zstd");
        assert_eq!(compression_format, CompressionFormat::Zstd);

        let compression_format = CompressionFormat::from("none");
        assert_eq!(compression_format, CompressionFormat::None);

        let string_format = String::from(CompressionFormat::Gzip);
        assert_eq!(string_format, "gzip");

        let string_format = String::from(CompressionFormat::Deflate);
        assert_eq!(string_format, "deflate");

        let string_format = String::from(CompressionFormat::DeflateRaw);
        assert_eq!(string_format, "deflate-raw");

        let string_format = String::from(CompressionFormat::Brotli);
        assert_eq!(string_format, "brotli");

        let string_format = String::from(CompressionFormat::Zstd);
        assert_eq!(string_format, "zstd");

        let string_format = String::from(CompressionFormat::None);
        assert_eq!(string_format, "none");

        let number: u8 = CompressionFormat::Gzip.into();
        assert_eq!(number, 2);

        let number: u8 = CompressionFormat::Deflate.into();
        assert_eq!(number, 5);

        let number: u8 = CompressionFormat::DeflateRaw.into();
        assert_eq!(number, 6);

        let number: u8 = CompressionFormat::Brotli.into();
        assert_eq!(number, 3);

        let number: u8 = CompressionFormat::Zstd.into();
        assert_eq!(number, 4);

        let number: u8 = CompressionFormat::None.into();
        assert_eq!(number, 1);

        // from number
        let compression_format = CompressionFormat::from(2);
        assert_eq!(compression_format, CompressionFormat::Gzip);

        let compression_format = CompressionFormat::from(5);
        assert_eq!(compression_format, CompressionFormat::Deflate);

        let compression_format = CompressionFormat::from(6);
        assert_eq!(compression_format, CompressionFormat::DeflateRaw);

        let compression_format = CompressionFormat::from(3);
        assert_eq!(compression_format, CompressionFormat::Brotli);

        let compression_format = CompressionFormat::from(4);
        assert_eq!(compression_format, CompressionFormat::Zstd);

        let compression_format = CompressionFormat::from(1);
        assert_eq!(compression_format, CompressionFormat::None);

        let compression_format = CompressionFormat::from(20);
        assert_eq!(compression_format, CompressionFormat::None);
    }

    #[test]
    fn test_compressor_err_from_fflate() {
        let err: CompressError = FFlateError::InvalidBlockType.into();
        assert_eq!(err, CompressError::FFlate(FFlateError::InvalidBlockType));
    }

    #[test]
    fn decode_zip_folder() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/utf.zip");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // get files
        let items = iter_zip_folder(&expected).unwrap();
        assert_eq!(items.len(), 6);
        // check each string
        // pull out all filenames
        let filenames: Vec<String> = items.iter().map(|item| item.filename.to_string()).collect();
        assert_eq!(
            filenames,
            vec!["utf.cpg", "utf.dbf", "utf.prj", "utf.qpj", "utf.shp", "utf.shx"]
        );
        let first = items.first().unwrap();
        let first_data = (first.read)().unwrap();

        assert_eq!(first_data, vec![85, 84, 70, 45, 56]);
        // convert first_dat to string
        let first_string = String::from_utf8(first_data).unwrap();
        assert_eq!(first_string, "UTF-8");
    }

    #[test]
    fn decode_zip_folder_2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/caltrain_20160406.zip");
        let data = fs::read(&path).expect("Failed to read file expected");

        let items = iter_zip_folder(&data).unwrap();
        assert_eq!(items.len(), 10);

        let filenames: Vec<String> = items.iter().map(|item| item.filename.to_string()).collect();
        assert_eq!(
            filenames,
            vec![
                "agency.txt",
                "calendar.txt",
                "calendar_dates.txt",
                "fare_attributes.txt",
                "fare_rules.txt",
                "routes.txt",
                "shapes.txt",
                "stop_times.txt",
                "stops.txt",
                "trips.txt"
            ]
        );
    }
}
