#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{FetchReader, Reader};

    #[test]
    fn test_no_ops() {
        let fetch_reader = FetchReader::new("".into(), false);

        assert_eq!(fetch_reader.len(), 0);
        assert_eq!(fetch_reader.uint64(None, None), 0);
        assert_eq!(fetch_reader.uint64_be(None), 0);
        assert_eq!(fetch_reader.uint64_le(None), 0);
        assert_eq!(fetch_reader.int64(None, None), 0);
        assert_eq!(fetch_reader.int64_be(None), 0);
        assert_eq!(fetch_reader.int64_le(None), 0);
        assert_eq!(fetch_reader.f64(None, None), 0.);
        assert_eq!(fetch_reader.f64_be(None), 0.);
        assert_eq!(fetch_reader.f64_le(None), 0.);
        assert_eq!(fetch_reader.uint32(None, None), 0);
        assert_eq!(fetch_reader.uint32_be(None), 0);
        assert_eq!(fetch_reader.uint32_le(None), 0);
        assert_eq!(fetch_reader.int32(None, None), 0);
        assert_eq!(fetch_reader.int32_be(None), 0);
        assert_eq!(fetch_reader.int32_le(None), 0);
        assert_eq!(fetch_reader.f32(None, None), 0.);
        assert_eq!(fetch_reader.f32_be(None), 0.);
        assert_eq!(fetch_reader.f32_le(None), 0.);
        assert_eq!(fetch_reader.uint16(None, None), 0);
        assert_eq!(fetch_reader.uint16_be(None), 0);
        assert_eq!(fetch_reader.uint16_le(None), 0);
        assert_eq!(fetch_reader.int16(None, None), 0);
        assert_eq!(fetch_reader.int16_be(None), 0);
        assert_eq!(fetch_reader.int16_le(None), 0);
        assert_eq!(fetch_reader.f16(None, None), 0.);
        assert_eq!(fetch_reader.f16_be(None), 0.);
        assert_eq!(fetch_reader.f16_le(None), 0.);
        assert_eq!(fetch_reader.uint8(None), 0);
        assert_eq!(fetch_reader.int8(None), 0);
        assert_eq!(fetch_reader.tell(), 0);
        fetch_reader.seek(10);
        assert_eq!(fetch_reader.tell(), 10);
        assert_eq!(fetch_reader.slice(None, None), Vec::<u8>::new());
        assert_eq!(fetch_reader.seek_slice(100), Vec::<u8>::new());
        assert_eq!(fetch_reader.parse_string(None, None), String::new());
    }
}
