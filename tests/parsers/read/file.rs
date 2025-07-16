#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{FileReader, Reader};
    use std::path::PathBuf;

    #[test]
    fn test_read_string() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/codepage.cpg");

        let reader = FileReader::new(path).unwrap();
        let string = reader.parse_string(None, None);
        assert_eq!(string, "ANSI 1250\n");
        let reader_clone = reader.clone();
        let string = reader_clone.parse_string(None, None);
        assert_eq!(string, "ANSI 1250\n");

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/codepage.cpg");

        let reader = FileReader::from(path);
        let string = reader.parse_string(None, None);
        assert_eq!(string, "ANSI 1250\n");

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/codepage.cpg");
        let path_str: String = path.to_str().unwrap().to_string();

        let reader = FileReader::from(path_str);
        let string = reader.parse_string(None, None);
        assert_eq!(string, "ANSI 1250\n");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_default_functions() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/fixtures/dv.bin");
        let path_str = path.to_str().unwrap();

        let reader = FileReader::from(path_str);

        assert_eq!(reader.tell(), 0);
        assert_eq!(reader.len(), 42);
        assert!(!reader.is_empty());

        let mut offset = 0;

        assert_eq!(reader.uint8(Some(offset)), 255);
        offset += 1;
        assert_eq!(reader.uint16_le(Some(offset)), 65535);
        assert_eq!(reader.uint16_be(Some(offset)), 65535);
        assert_eq!(reader.uint16(Some(offset), Some(true)), 65535);
        assert_eq!(reader.uint16(Some(offset), Some(false)), 65535);
        assert_eq!(reader.f16_le(Some(offset)), 9.1834e-41);
        assert_eq!(reader.f16_be(Some(offset)), 9.1834e-41);
        assert_eq!(reader.f16(Some(offset), Some(true)), 9.1834e-41);
        assert_eq!(reader.f16(Some(offset), Some(false)), 9.1834e-41);
        offset += 2;
        assert_eq!(reader.uint32_le(Some(offset)), 4294967295);
        assert_eq!(reader.uint32_be(Some(offset)), 4294967295);
        assert_eq!(reader.uint32(Some(offset), Some(true)), 4294967295);
        assert_eq!(reader.uint32(Some(offset), Some(false)), 4294967295);
        offset += 4;
        assert_eq!(reader.int8(Some(offset)), -128);
        offset += 1;
        assert_eq!(reader.int16_le(Some(offset)), -32768);
        assert_eq!(reader.int16_be(Some(offset)), 128);
        assert_eq!(reader.int16(Some(offset), Some(true)), -32768);
        assert_eq!(reader.int16(Some(offset), Some(false)), 128);
        offset += 2;
        assert_eq!(reader.int32_le(Some(offset)), -2147483648);
        assert_eq!(reader.int32_be(Some(offset)), 128);
        assert_eq!(reader.int32(Some(offset), Some(true)), -2147483648);
        assert_eq!(reader.int32(Some(offset), Some(false)), 128);
        offset += 4;
        assert_eq!(reader.f32_le(Some(offset)), 3.14);
        assert_eq!(reader.f32_be(Some(offset)), -490.56445);
        assert_eq!(reader.f32(Some(offset), Some(true)), 3.14);
        assert_eq!(reader.f32(Some(offset), Some(false)), -490.56445);
        offset += 4;
        assert_eq!(reader.f64_le(Some(offset)), 3.14159265359);
        assert_eq!(reader.f64_be(Some(offset)), -2.965482352282314e203);
        assert_eq!(reader.f64(Some(offset), Some(true)), 3.14159265359);
        assert_eq!(reader.f64(Some(offset), Some(false)), -2.965482352282314e203);
        offset += 8;
        assert_eq!(reader.uint64_le(Some(offset)), 12345678901234567890);
        assert_eq!(reader.uint64_be(Some(offset)), 15134944594269656235);
        assert_eq!(reader.uint64(Some(offset), Some(true)), 12345678901234567890);
        assert_eq!(reader.uint64(Some(offset), Some(false)), 15134944594269656235);
        offset += 8;
        assert_eq!(reader.int64_le(Some(offset)), -1234567890123456789);
        assert_eq!(reader.int64_be(Some(offset)), -1477718879929115154);
        assert_eq!(reader.int64(Some(offset), Some(true)), -1234567890123456789);
        assert_eq!(reader.int64(Some(offset), Some(false)), -1477718879929115154);

        let slice = reader.slice(Some(4), Some(8));
        assert_eq!(slice, &[255, 255, 255, 128]);
        reader.seek(4);
        assert_eq!(reader.seek_slice(4), &[255, 255, 255, 128]);
        assert_eq!(reader.tell(), 8);
    }
}
